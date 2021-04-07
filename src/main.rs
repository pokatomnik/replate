use std::env;

mod constants;
mod help;
mod utils;

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

    let (tpl_kv, other_args) = utils::parse(args.clone());

    let template_path_arg = &utils::get_template_path(other_args);

    if !utils::template_exists(template_path_arg) {
        help::display_template_missing(template_path_arg);
        return;
    }

    let mut contents = utils::read_file_contents(template_path_arg);
    contents = utils::replace_with_source(contents, tpl_kv);
    contents = utils::cleanup_ignored(contents);

    println!("{}", contents);
}
