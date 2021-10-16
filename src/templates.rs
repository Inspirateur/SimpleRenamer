use cached::proc_macro::cached;
use difflib::sequencematcher::SequenceMatcher;
use itertools::Itertools;
use lazy_static::lazy_static;
use regex::Regex;
use std::fs;
use std::{collections::HashMap, path::PathBuf};

lazy_static! {
    static ref RE_TOK: Regex = Regex::new(r"([^\d\W]+|[0-9]+|\W+)").unwrap();
    static ref RE_WRD: Regex = Regex::new(r"\w|\(\.\+\)|\(\\d\+\)").unwrap();
    pub static ref RE_UNESC: Regex =
        Regex::new(r"\\([\\\.\+\*\?\(\)\|\[\]\{\}\^\$\#\&\-\~])").unwrap();
}

fn extract_rule(seq1: &Vec<String>, seq2: &Vec<String>) -> Option<String> {
    let mut seq_matcher = SequenceMatcher::new(seq1, seq2);
    // if the 2 sequences are too dissimilar we don't return a rule
    if seq_matcher.ratio() < 0.5 {
        return None;
    }
    let (mut _i, mut _j, mut _n) = (0, 0, 0);
    let mut rule = String::new();
    let mut var1: String;
    let mut var2: String;
    let mut cst: String;
    for m in seq_matcher.get_matching_blocks() {
        var1 = seq1[(_i + _n)..m.first_start].join("");
        var2 = seq2[(_j + _n)..m.second_start].join("");
        cst = seq1[m.first_start..(m.first_start + m.size)].join("");
        if _n != 0 && m.size != 0 && (var1.len() == 0 || var2.len() == 0) {
            // there's no template
            return None;
        }
        let var_is_num = var1.parse::<u16>().is_ok() && var2.parse::<u16>().is_ok();
        if var_is_num || RE_WRD.is_match(&cst) {
            if var1.len() > 0 {
                if var_is_num {
                    rule += r"(\d+)";
                } else {
                    rule += "(.+)";
                }
            }
            rule += &regex::escape(&cst);
        }
        _i = m.first_start;
        _j = m.second_start;
        _n = m.size;
    }
    return Some(rule);
}

fn find_rule(filenames_tok: &Vec<Vec<String>>) -> Option<String> {
    let mut rule = String::new();
    // compare every pair of unmatched file until a rule is found
    for (seq1, seq2) in filenames_tok.iter().tuple_combinations() {
        if let Some(_rule) = extract_rule(seq1, seq2) {
            rule = _rule;
            break;
        }
    }
    // if no rule is found we can stop the process
    if rule.len() == 0 {
        None
    } else {
        Some(rule)
    }
}

fn templates(filenames: &Vec<String>) -> HashMap<String, Vec<String>> {
    let mut untemplated: Vec<Vec<String>> = filenames
        .iter()
        .map(|filename| {
            RE_TOK
                .find_iter(filename)
                .map(|m| m.as_str().to_string())
                .collect()
        })
        .collect();
    let mut res = HashMap::new();
    // template the files until we cannot find another rule
    while let Some(rule) = find_rule(&untemplated) {
        // try out the rule on every unmatched title
        let last_len = untemplated.len();
        let re_rule = Regex::new(&format!("^{}$", rule)).unwrap();
        let mut _temp = Vec::new();
        for seq in untemplated.into_iter() {
            let filename = seq.join("");
            if re_rule.is_match(&filename) {
                res.entry(rule.clone()).or_insert(Vec::new()).push(filename);
            } else {
                _temp.push(seq);
            }
        }
        untemplated = _temp;
        assert_ne!(last_len, untemplated.len());
    }
    // try to unify the rules if there's more than 1
    if res.len() > 1 {
        let meta_templates = templates(&res.keys().cloned().collect());
        for (meta_rule, rules) in meta_templates.iter() {
            let un_meta_rule = RE_UNESC.replace_all(&meta_rule, r"$1").into_owned();
            res.insert(un_meta_rule.clone(), Vec::new());
            for rule in rules {
                let mut v = res.remove(rule).unwrap();
                res.get_mut(&un_meta_rule).unwrap().append(&mut v);
            }
        }
    }
    res
}

#[cached(size = 1)]
pub fn templates_in(path: PathBuf) -> HashMap<String, Vec<String>> {
    let filenames: Vec<String> = fs::read_dir(path)
        .unwrap()
        .map(|path| path.unwrap().file_name().to_str().unwrap().to_string())
        .collect();
    templates(&filenames)
}

#[cfg(test)]
mod tests {
    use crate::templates::templates;
    use itertools::iproduct;
    use lipsum::lipsum_title;
    use std::collections::HashSet;

    #[test]
    fn simple() {
        let filenames: Vec<String> = (1..=20).map(|e| format!("Episode {}.mp4", e)).collect();
        let mut truth = HashSet::new();
        truth.insert(String::from(r"Episode (\d+)\.mp4"));
        let res: HashSet<String> = templates(&filenames).keys().cloned().collect();
        assert_eq!(&truth, &res);
    }

    #[test]
    fn multivar() {
        let filenames: Vec<String> = iproduct!(["EN", "FR"], 1..=6)
            .map(|(lang, e)| format!("Episode {} {}.mp4", lang, e))
            .collect();
        let mut truth = HashSet::new();
        truth.insert(String::from(r"Episode (.+) (\d+)\.mp4"));
        let res: HashSet<String> = templates(&filenames).keys().cloned().collect();
        assert_eq!(&truth, &res);
    }

    #[test]
    fn title() {
        let filenames: Vec<String> = (1..=10)
            .map(|e| format!("Episode {} - {}.mp4", e, lipsum_title()))
            .collect();
        let mut truth = HashSet::new();
        truth.insert(String::from(r"Episode (\d+) \- (.+)\.mp4"));
        let res: HashSet<String> = templates(&filenames).keys().cloned().collect();
        assert_eq!(&truth, &res);
    }

    #[test]
    fn multi_template() {
        let mut filenames: Vec<String> = (1..=9)
            .map(|e| format!("Harry Potter {}.mp4", e))
            .chain((1..=4).map(|e| format!("Fantastic Beasts and Where to Find Them {}.mp4", e)))
            .collect();
        filenames.push(String::from("README.txt"));
        let mut truth = HashSet::new();
        truth.insert(String::from(r"Harry Potter (\d+)\.mp4"));
        truth.insert(String::from(
            r"Fantastic Beasts and Where to Find Them (\d+)\.mp4",
        ));
        let res: HashSet<String> = templates(&filenames).keys().cloned().collect();
        assert_eq!(&truth, &res);
    }

    #[test]
    fn dates() {
        let filenames: Vec<String> = iproduct!(2014..=2015, 1..=3, 1..=3)
            .map(|(y, m, d)| format!("{}-{}-{}.jpg", y, m, d))
            .collect();
        let mut truth = HashSet::new();
        truth.insert(String::from(r"(\d+)\-(\d+)\-(\d+)\.jpg"));
        let res: HashSet<String> = templates(&filenames).keys().cloned().collect();
        assert_eq!(&truth, &res);
    }

    #[test]
    fn regex_in_title() {
        let filenames: Vec<String> = iproduct!(1..=20)
            .map(|e| format!("[1080p] Episode {}.mkv", e))
            .collect();
        let mut truth = HashSet::new();
        truth.insert(String::from(r"\[1080p\] Episode (\d+)\.mkv"));
        let res: HashSet<String> = templates(&filenames).keys().cloned().collect();
        assert_eq!(&truth, &res);
    }
}
