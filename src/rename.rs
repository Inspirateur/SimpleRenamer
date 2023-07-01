use crate::templates::templates_in;
use itertools::Itertools;
use regex::Regex;
use std::{collections::HashMap, fs, io, path::PathBuf};
use log::info;

fn ambiguities(rule: &Regex, filename: &String) -> u32 {
    // count the amount of ambiguities in the filename
    let mut copy = filename.clone();
    let mut count = 0;
    for m in rule
        .captures(filename)
        .unwrap()
        .iter()
        .map(|m_opt| m_opt.unwrap().as_str())
        .skip(1)
        .sorted_by_key(|m| -(m.len() as i32))
    {
        let occurences = copy.match_indices(m).count();
        // assert!(occurences > 0);
        count += (occurences as u32) - 1;
        copy = copy.replace(m, "");
    }
    count
}

pub fn get_rule_rep(cwd: &PathBuf, a: &String) -> String {
    // sometimes there's ambiguity in the title
    // (= the value of a variable is present multiple times in the string)
    // if this happens with the file the user chose we need to use another one
    let templates = templates_in(cwd.clone());
    if let Some(rule) = get_rule(&templates, a) {
        let re_rule = Regex::new(&format!("^{}$", rule)).unwrap();
        let mut min_ambiguities = ambiguities(&re_rule, a);
        let mut best_rep = a;
        for candidate in &templates[&rule] {
            if min_ambiguities == 0 {
                break;
            }
            let new_ambiguities = ambiguities(&re_rule, candidate);
            if new_ambiguities < min_ambiguities {
                min_ambiguities = new_ambiguities;
                best_rep = candidate;
            }
        }
        return best_rep.clone();
    }
    return a.clone();
}

fn get_rule(templates: &HashMap<String, Vec<String>>, filename: &String) -> Option<String> {
    // return the rule that parses the filename if it exists
    for rule in templates.keys() {
        if Regex::new(&format!("^{}$", rule))
            .unwrap()
            .is_match(filename)
        {
            return Some(rule.clone());
        }
    }
    None
}

fn get_rename_str(rule: &Regex, a: &String, b: &String) -> String {
    // return the regex replacement to turn a into b
    // extract the value of the variables
    let vars: Vec<String> = rule
        .captures(a)
        .unwrap()
        .iter()
        .skip(1)
        .map(|m_opt| m_opt.unwrap().as_str().to_string())
        .collect();
    // find those values again in the new string, and replace them with capture group stand-in eg: ${1}, ${2}, etc.
    let mut res = String::new();
    let mut b_ =  b.as_str();
    for (i, value) in vars.into_iter().enumerate() {
        if let Some((add, remain)) = b_.split_once(&value) {
            res += add;
            res += &format!("${{{}}}", i+1);
            b_ = remain;
        }
    }
    res += b_;
    res
}

pub fn rename_map(cwd: &PathBuf, a: &String, b: &PathBuf) -> Vec<(PathBuf, PathBuf)> {
    // Returns a table that maps old filename to new filename
    let mut path_b = cwd.clone();
    if let Some(parent) = b.parent() {
        path_b.push(parent);
    }
    let b_name = if let Some(name) = b.file_name() {
        name.to_string_lossy().to_string()
    } else {
        String::new()
    };
    let templates = templates_in(cwd.clone());
    let mut res: Vec<(PathBuf, PathBuf)> = Vec::new();
    if let Some(rule) = get_rule(&templates, a) {
        info!(target: "simple renamer", "rule: {}", rule);
        let re_rule = Regex::new(&format!("^{}$", rule)).unwrap();
        let rename_rule = get_rename_str(&re_rule, a, &b_name);
        info!(target: "simple renamer", "rename: {}", rename_rule);
        for file in templates.get(&rule).unwrap() {
            res.push((
                cwd.join(file.clone()),
                path_b.join(re_rule.replace(file, &rename_rule).to_string())
            ));
        }
    }
    res.sort_by_key(|(old, _new)| old.clone());
    res
}

pub fn apply_rename(rename: &Vec<(PathBuf, PathBuf)>) -> io::Result<()> {
    for (old, new) in rename.iter() {
        // create the path if it doesn't exist
        if let Some(new_parent) = new.parent() {
            fs::create_dir_all(new_parent)?;
        }        
        fs::rename(old, new)?;
    }
    Ok(())
}
