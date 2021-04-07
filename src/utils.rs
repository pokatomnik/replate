use crate::constants;
use regex::Regex;
use std::collections::HashMap;
use std::fs;
use std::path::Path;

pub fn parse(args: Vec<String>) -> (HashMap<String, String>, HashMap<String, String>) {
  let mut tpl_kv: HashMap<String, String> = HashMap::new();
  let mut other_args: HashMap<String, String> = HashMap::new();
  let len = args.len();

  for i in (1..len).step_by(2) {
    let key = args[i].clone();
    let value = args[i + 1].clone();
    if key.starts_with("--") {
      other_args.insert(key, value);
    } else if key.starts_with("-") {
      tpl_kv.insert(key.trim_start_matches("-").to_owned(), value);
    } else {
      other_args.insert(key, value);
    }
  }

  return (tpl_kv, other_args);
}

pub fn get_template_path(source: HashMap<String, String>) -> String {
  let template_path_arg = constants::DEFAULT_TEMPLATE_PATH;
  if source.contains_key(constants::KEY_TEMPLATE) {
    return source.get(constants::KEY_TEMPLATE).unwrap().to_string();
  }

  return String::from(template_path_arg);
}

pub fn template_exists(path: &str) -> bool {
  let path = Path::new(path);
  return path.exists() && path.is_file();
}

pub fn read_file_contents(path: &str) -> String {
  return fs::read_to_string(path).expect("Something went wrong reading the file");
}

pub fn replace_with_source(mut contents: String, source: HashMap<String, String>) -> String {
  for (k, v) in source.iter() {
    let key = format!("{{{{{}}}}}", k);
    contents = contents.replace(&key, v);
  }
  return contents;
}

pub fn cleanup_ignored(contents: String) -> String {
  let re = Regex::new("[{]{2}[^{{]{2}[^}}]{2}+[}]{2}").unwrap();
  return re.replace_all(&contents, "").to_string();
}
