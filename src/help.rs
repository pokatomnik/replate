use crate::constants;
use colored::*;

pub fn display_help() {
  println!("Usage: tpl [--help] [--template <templatefile>] [-key1 <value1> ...]\n");
  println!("Fills the template with provided keys and values");
  println!("  --help                  print this information");
  println!("  --template filename     use specific template file to fill, \"template.tpl\" is used by default");
  println!("  -key value              replace \"{{{{key}}}}\" (without quotes) with \"value\" (without quotes)");
}

pub fn display_template_missing(template_path: &str) {
  let message = format!("Template by path '{}' does not exists", template_path);
  let colored_message = (&message[..]).red();
  println!("{}", colored_message);
}

pub fn display_incorrect_args_number() {
  let message = (constants::INCORRECT_ARGS_NUMBER).red();
  println!("{}", message);
}
