use regex::Regex;
use std::env;
use std::fs;
use std::path::Path;

mod args_parser;
mod constants;
mod help;

fn main() {
    let args: Vec<String> = env::args().collect();
    let len = args.len();

    if len < 2 {
        return;
    }

    if len == 2 && args[1] == constants::KEY_HELP {
        help::display_help();
        return;
    }

    if (len - 1) % 2 != 0 {
        help::display_incorrect_args_number();
        help::display_help();
        return;
    }

    let (tpl_kv_parsed, other_args_parsed) = args_parser::parse(args.clone());
    let mut tpl_kv = tpl_kv_parsed.clone();
    let mut other_args = other_args_parsed.clone();

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
    let mut template_path_arg = constants::DEFAULT_TEMPLATE_PATH;
    if other_args.contains_key(constants::KEY_TEMPLATE) {
        template_path_arg = other_args.get(constants::KEY_TEMPLATE).unwrap();
    }

    let path = Path::new(template_path_arg);
    if !path.exists() || !path.is_file() {
        help::display_template_missing(template_path_arg);
        return;
    }

    let mut contents =
        fs::read_to_string(template_path_arg).expect("Something went wrong reading the file");
    for (k, v) in tpl_kv.iter() {
        let key = format!("{{{{{}}}}}", k);
        contents = contents.replace(&key, v);
    }

    let re = Regex::new("[{]{2}.+[}]{2}").unwrap();
    let res = re.replace_all(&contents, "").to_string();

    println!("{}", res);
}
