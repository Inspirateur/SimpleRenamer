use crate::templates::templates_in;
use itertools::Itertools;
use regex::Regex;
use std::{collections::HashMap, fs, io, path::PathBuf};

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
    let vars: Vec<String> = rule
        .captures(a)
        .unwrap()
        .iter()
        .skip(1)
        .map(|m_opt| m_opt.unwrap().as_str().to_string())
        .collect();
    let var2idx: HashMap<&String, usize> =
        vars.iter().enumerate().map(|(i, m)| (m, i + 1)).collect();
    let re_match = Regex::new(
        &vars
            .iter()
            .sorted_by_key(|m| -(m.len() as i32))
            .map(|m| regex::escape(m))
            .join("|"),
    )
    .unwrap();
    re_match
        .replace_all(b, |captures: &regex::Captures| {
            format!("${{{}}}", var2idx[&captures[0].to_string()])
        })
        .to_string()
}

pub fn rename_map(cwd: &PathBuf, a: &String, b: &PathBuf) -> HashMap<PathBuf, PathBuf> {
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
    let mut res: HashMap<PathBuf, PathBuf> = HashMap::new();
    if let Some(rule) = get_rule(&templates, a) {
        println!("{}", rule);
        let re_rule = Regex::new(&format!("^{}$", rule)).unwrap();
        let rename_rule = get_rename_str(&re_rule, a, &b_name);
        println!("{}", rename_rule);
        for file in templates.get(&rule).unwrap() {
            res.insert(
                cwd.join(file.clone()),
                path_b.join(re_rule.replace(file, &rename_rule).to_string()),
            );
        }
    }
    res
}

pub fn apply_rename(rename: &HashMap<PathBuf, PathBuf>) -> io::Result<()> {
    if let Some(first_new_path) = rename.values().next() {
        // create the path if it doesn't exist
        if let Some(first_new_parent) = first_new_path.parent() {
            fs::create_dir_all(first_new_parent)?;
        }
        for (old, new) in rename.iter().sorted_by_key(|(old, _new)| *old) {
            fs::rename(old, new)?;
        }
    }
    Ok(())
}
