use std::{collections, error, fs, io::Write, path};

const NUM_DAYS: u32 = 25;

fn main() -> Result<(), Box<dyn error::Error>> {
    let days = path::Path::new("src/day");
    let mut dir_elements = collections::HashSet::new();
    fs::read_dir(days)?
        .map(|file| String::from(file.unwrap().file_name().to_str().unwrap()))
        .for_each(|file_name| {
            dir_elements.insert(file_name);
        });

    let mut mod_file = fs::File::create(path::Path::new("src/day/mod.rs"))?;
    writeln!(
        mod_file,
        "// This file is auto-generated. Please edit build.rs to make changes"
    )?;
    let mut mod_file = fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(path::Path::new("src/day/mod.rs"))?;
    writeln!(mod_file, "use std::error::Error;")?;
    writeln!(mod_file, "use super::stage::Stage;")?;

    let mut first_default_method = true;
    for day in 1..=NUM_DAYS {
        let day_name = format!("day{:02}.rs", day);
        if !dir_elements.contains(&day_name) {
            if first_default_method {
                writeln!(
                    mod_file,
                    "\
                    use std::fmt;\
                    #[derive(Debug)]\
                    struct DayError(u8);\
                    \
                    impl fmt::Display for DayError {{\
                        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {{\
                            write!(f, \"Invalid day: {{}}\", self.0)\
                        }}\
                    }}\
                    \
                    impl Error for DayError {{}}"
                )?;
                first_default_method = false;
            }
            writeln!(
                mod_file,
                "pub mod day{day} {{\
                    pub fn run(_s: &str, _stage: super::Stage) -> Result<String, Box<dyn super::Error>> {{\
                        Err(Box::new(super::DayError({day}.into())))
                    }}\
                }}"
            )?;
        } else {
            let day_module_name = format!("day{:02}", day);
            if day >= 10 {
                writeln!(mod_file, "pub mod {day_module_name};")?;
            } else {
                let day_module_fmt_name = format!("day{}", day);
                writeln!(mod_file, "mod {day_module_name}; pub mod {day_module_fmt_name} {{pub use super::{day_module_name}::run;}}")?;
            }
        }
    }

    Ok(())
}
