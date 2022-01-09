use std::error::Error;
use std::fs;

pub struct Config {
  pub filename: String,
}

impl Config {
  pub fn new(args: &[String]) -> Result<Config, &str> {
      if args.len() < 2 {
          return Err("not enough arguments, expecting a filename");
      }

      let filename = args[1].clone();

      Ok(Config { filename })
  }
}

pub struct Counts {
  pub total_lines: u32,
  pub empty_lines: u32,
}

pub fn run(config:Config) -> Result<(), Box<dyn Error>> {
  let contents = fs::read_to_string(config.filename)?;

  let counts = count(&contents);

  println!("There are {} lines of code", counts.total_lines);
  println!("There are {} empty lines", counts.empty_lines);
  println!("{:.2}% of the lines are empty", counts.empty_lines as f32 / counts.total_lines as f32 * 100.0);

  Ok(())
}

pub fn count(contents: &str) -> Counts {
  let mut total_lines: u32 = 0;
  let mut empty_lines: u32 = 0;
  for line in contents.lines() {
    total_lines += 1;
    if line.is_empty() {
      empty_lines += 1;
    }
  }
  Counts { total_lines, empty_lines }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn simple_multiline() {
        let contents = "\
Rust:
safe, fast, productive.
Pick three.";

        assert_eq!(3, count(contents).total_lines);
        assert_eq!(0, count(contents).empty_lines);
    }

    #[test]
    fn simple_multiline_with_empty() {
      let contents = "\
Rust:

Pick three.";

      assert_eq!(3, count(contents).total_lines);
      assert_eq!(1, count(contents).empty_lines);
  }
}
