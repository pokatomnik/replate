use crate::constants;

pub fn display_help() {
  println!("Usage: tpl [--help] [--template <templatefile>] [-key1 <value1> ...]\n");
  println!("Fills the template with provided keys and values");
  println!("  --help                  print this information");
  println!("  --template filename     use specified template file to fill, \"template.tpl\" is used by default");
  println!("  -key value              replace \"{{{{key}}}}\" (without quotes) with \"value\" (without quotes)");
}

pub fn display_template_missing(template_path: &str) {
  println!("Template by path '{}' does not exists", template_path);
}

pub fn display_incorrect_args_number() {
  println!("{}", constants::INCORRECT_ARGS_NUMBER);
}
