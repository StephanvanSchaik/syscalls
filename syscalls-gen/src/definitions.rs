use color_eyre::eyre::Result;
use itertools::Itertools;
use regex::Regex;
use std::collections::HashMap;
use std::fs::File;
use std::io::{BufRead, BufReader};
use std::path::Path;
use walkdir::WalkDir;

#[derive(Clone, Debug)]
pub struct Argument {
    pub ty: String,
    pub name: String,
}

#[derive(Clone, Debug)]
pub struct SystemCall {
    pub arguments: Vec<Argument>,
}

pub fn parse_definitions<P>(root: P) -> Result<HashMap<String, SystemCall>>
where
    P: AsRef<Path>,
{
    let mut system_calls = HashMap::new();
    let re = Regex::new(r"SYSCALL_DEFINE(\d)\(([a-zA-Z0-9_\* ]+(?:,\s*[a-zA-Z0-9_\* ]+)*)\)").unwrap();

    for entry in WalkDir::new(root) {
        let entry = entry?;
        let path = entry.path();

        // Extract the file extension.
        let extension = match path.extension() {
            Some(extension) => extension.to_str().unwrap(),
            None => continue,
        };

        // Skip files that are not .c files.
        if extension != "c" {
            continue;
        }

        // Open the file.
        let file = File::open(path)?;
        let reader = BufReader::new(file);
        let mut s = String::new();

        // Iterate over the lines.
        for line in reader.lines() {
            let line = line.unwrap();

            // Once we have the matching ')', the system call definition has been collected and we
            // can start parsing it.
            if s.contains(")") {
                // Match the definition against the regular expression.
                let caps = re.captures(&s).expect("system call definition does not match");

                // Split the arguments.
                let mut args = caps[2]
                    .split(",")
                    .map(|arg| arg.trim().to_owned());

                // The first argument is the name.
                let name = args.next().expect("system call definition does not have a name");

                // Group the arguments.
                let args = args.tuples()
                    .map(|(ty, name)| Argument { ty, name })
                    .collect::<Vec<Argument>>();

                system_calls.insert(name, SystemCall {
                    arguments: args,
                });

                s = String::new();
            }

            // The line is not a system call definition.
            if !line.starts_with("SYSCALL_DEFINE") {
                // We are currently not collecting a system call definition.
                if s.is_empty() {
                    continue;
                }

                // Collect it as part of the current system call definition that we are collecting.
                s.push_str(line.trim());

                continue;
            }

            // The line contains a system call definition, start collecting the string.
            s.push_str(&line);
        }
    }

    Ok(system_calls)
}
