use std::collections::HashMap;

pub fn parse(args: Vec<String>) -> (HashMap<String, String>, HashMap<String, String>, usize) {
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

  return (tpl_kv, other_args, len);
}
