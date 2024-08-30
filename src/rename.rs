use crate::text_histo::{text_histo, TextHisto};
use auto_regex::infer_regex;
use itertools::Itertools;
use regex::Regex;
use std::{fs, io, path::PathBuf};
use log::info;

fn get_rename_str(rule: &Regex, a: &String, b: &String) -> String {
    // return the regex replacement to turn a into b
    // extract the value and index of the variables
    let vars: Vec<(String, usize)> = rule
        .captures(a)
        .unwrap()
        .iter()
        .skip(1)
        .filter(|m_opt| m_opt.is_some())
        .map(|m_opt| m_opt.unwrap())
        .map(|m| (m.as_str().to_string(), m.start()))
        .collect();

    // find those values again in the new string, and replace them with capture group stand-in eg: ${1}, ${2}, etc.
    // In the new string there can be ambiguities, with values appearing mutliple times, 
    // with use a "histogram" of every character to their left to distinguish them
    let mut res = b.clone(); 
    for (i, (value, start)) in vars.into_iter().enumerate() {
        let matches = res.match_indices(&value).collect_vec();
        let ref_histo = text_histo(&a[0..start]);
        let mut best_match_opt = None;
        let mut best_dist = usize::MAX;
        for (pos, _) in matches {
            let histo = text_histo(&b[0..pos]);
            let dist = ref_histo.dist(&histo);
            if dist < best_dist {
                best_dist = dist;
                best_match_opt = Some(pos);
            }
        }
        if let Some(best_match) = best_match_opt {
            let start = best_match as usize;
            let end = start + value.len();
            res.replace_range(start..end, &format!("${{{}}}", i+1));
        }
    }
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
    let filenames: Vec<String> = fs::read_dir(cwd)
        .unwrap()
        .map(|path| path.unwrap().file_name().to_str().unwrap().to_string())
        .collect();
    let mut res: Vec<(PathBuf, PathBuf)> = Vec::new();
    if let Some(regex) = infer_regex(a.to_string(), filenames.clone()) {
        info!(target: "srenamer", "rule: {}", regex);
        let rename_rule = get_rename_str(&regex, a, &b_name);
        info!(target: "srenamer", "rename: {}", rename_rule);
        for file in filenames {
            if !regex.is_match(&file) {
                continue;
            }
            res.push((
                cwd.join(file.clone()),
                path_b.join(regex.replace(&file, &rename_rule).to_string())
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
