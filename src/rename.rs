use crate::templates::{templates_in, RE_UNESC};
use itertools::Itertools;
use regex::Regex;
use std::{collections::HashMap, fs, io, path::PathBuf};

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
    let mut res = b.clone();
    for (i, m_opt) in rule.captures(a).unwrap().iter().enumerate().skip(1) {
        let m = m_opt.unwrap().as_str();
        let re_repl = if m.parse::<u8>().is_ok() {
            Regex::new(&format!(r"(\D|^){}(\D|$)", m))
        } else {
            Regex::new(&format!("(.*){}(.*)", regex::escape(m)))
        }
        .unwrap();
        res = re_repl
            .replace(&res, format!("${{1}}\\$\\{{{}\\}}${{2}}", i))
            .to_string();
    }
    RE_UNESC.replace_all(&res, r"$1").into_owned()
}

pub fn rename_map(cwd: &PathBuf, a: &PathBuf, b: &PathBuf) -> HashMap<PathBuf, PathBuf> {
    // Returns a table that maps old filename to new filename
    let mut path_a = cwd.clone();
    if let Some(parent) = a.parent() {
        path_a.push(parent);
    }
    let mut path_b = cwd.clone();
    if let Some(parent) = b.parent() {
        path_b.push(parent);
    }
    let a_name = a.file_name().unwrap().to_string_lossy().to_string();
    let b_name = if let Some(name) = b.file_name() {
        name.to_string_lossy().to_string()
    } else {
        String::new()
    };
    let templates = templates_in(path_a.clone());
    let mut res: HashMap<PathBuf, PathBuf> = HashMap::new();
    if let Some(rule) = get_rule(&templates, &a_name) {
        let re_rule = Regex::new(&format!("^{}$", rule)).unwrap();
        let rename_rule = get_rename_str(&re_rule, &a_name, &b_name);
        for file in templates.get(&rule).unwrap() {
            res.insert(
                path_a.join(file.clone()),
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
