use std::collections::HashMap;
use std::fmt;

use anyhow::{anyhow, Result};
use aoc_2022::Args;
use structopt::StructOpt;

#[derive(Clone, Debug)]
struct File {
    name: String,
    size: u64,
}

#[derive(Clone, Debug)]
struct Directory {
    entry_names: Vec<String>,
}

impl Directory {
    fn empty() -> Self {
        Directory {
            entry_names: Vec::new(),
        }
    }
}

#[derive(Clone, Debug)]
enum DirectoryEntry {
    File(File),
    Directory(Directory),
}

fn calculate_directory_size(
    hierarchy: &HashMap<String, DirectoryEntry>,
    sizes: &mut HashMap<String, u64>,
    directory_name: &str,
) -> Result<()> {
    if sizes.contains_key(directory_name) {
        return Ok(());
    }

    let entry = hierarchy
        .get(directory_name)
        .ok_or_else(|| anyhow!("No entry found for \"{}\"", directory_name))?;

    match entry {
        DirectoryEntry::Directory(directory) => {
            let mut total_size = 0;

            for entry_name in directory.entry_names.iter() {
                match hierarchy
                    .get(entry_name)
                    .ok_or_else(|| anyhow!("No entry found for path \"{}\"", entry_name))?
                {
                    DirectoryEntry::Directory(_) => {
                        calculate_directory_size(&hierarchy, sizes, entry_name)?;
                        total_size += sizes
                            .get(entry_name)
                            .ok_or_else(|| anyhow!("No entry found for path \"{}\"", entry_name))?;
                    }
                    DirectoryEntry::File(file) => {
                        total_size += file.size;
                    }
                }
            }

            sizes.insert(directory_name.to_string(), total_size);
        }
        DirectoryEntry::File(file) => {
            sizes.insert(file.name.clone(), file.size);
        }
    }

    Ok(())
}

fn calculate_directory_sizes(
    hierarchy: &HashMap<String, DirectoryEntry>,
) -> Result<HashMap<String, u64>> {
    let mut sizes = HashMap::new();

    calculate_directory_size(hierarchy, &mut sizes, "/")?;

    Ok(sizes)
}

#[derive(Clone, Debug, Eq, PartialEq, Hash)]
struct WorkingDirectory {
    path: Vec<String>,
}

impl WorkingDirectory {
    fn new(initial_path: &str) -> Self {
        Self {
            path: vec![initial_path.to_string()],
        }
    }

    fn descend(&mut self, directory: &str) {
        self.path.push(directory.to_string());
    }

    fn ascend(&mut self) {
        self.path.pop();
    }

    fn path_for_target(&self, target: &str) -> String {
        format!(
            "{}{}{}",
            self,
            if self.path.len() == 1 { "" } else { "/" },
            target
        )
    }
}

impl fmt::Display for WorkingDirectory {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "/{}", self.path[1..].join("/"))
    }
}

fn main() -> Result<()> {
    let args = Args::from_args();
    let data = std::fs::read_to_string(&args.filename)?;

    let mut pwd: Option<WorkingDirectory> = None;
    let mut hierarchy: HashMap<String, DirectoryEntry> = HashMap::new();

    for line in data.lines() {
        let parts: Vec<&str> = line.split(' ').collect();
        let first_part = parts
            .get(0)
            .ok_or_else(|| anyhow!("Missing initial section of line \"{}\"", line))?;

        match first_part {
            &"$" => {
                let command = parts
                    .get(1)
                    .ok_or_else(|| anyhow!("Missing command in line \"{}\"", line))?;
                match command {
                    &"cd" => {
                        let target = parts
                            .get(2)
                            .ok_or_else(|| anyhow!("Missing cd target in line \"{}\"", line))?;

                        if target == &".." {
                            match &mut pwd {
                                Some(working_directory) => {
                                    let pwd_entry = hierarchy
                                        .get(&working_directory.to_string())
                                        .ok_or_else(|| {
                                        anyhow!(
                                            "No directory found for pwd name \"{}\"",
                                            working_directory
                                        )
                                    })?;

                                    match pwd_entry {
                                        DirectoryEntry::Directory(_) => {
                                            working_directory.ascend();
                                        }
                                        DirectoryEntry::File(_) => {
                                            return Err(anyhow!(
                                                "pwd \"{}\" is somehow a file",
                                                working_directory
                                            ));
                                        }
                                    }
                                }
                                None => {
                                    return Err(anyhow!(
                                        "Attempted to cd out of a nonexistent directory"
                                    ));
                                }
                            }
                        } else {
                            match hierarchy.get(*target).cloned() {
                                Some(_) => match pwd.as_mut() {
                                    Some(working_directory) => {
                                        working_directory.descend(target);
                                    }
                                    None => {
                                        pwd = Some(WorkingDirectory::new(target));
                                    }
                                },
                                None => {
                                    let entry_name = match pwd.as_mut() {
                                        Some(working_directory) => {
                                            let entry_name =
                                                working_directory.path_for_target(target);

                                            working_directory.descend(target);

                                            entry_name
                                        }
                                        None => {
                                            pwd = Some(WorkingDirectory::new(target));
                                            target.to_string()
                                        }
                                    };

                                    hierarchy.insert(
                                        entry_name,
                                        DirectoryEntry::Directory(Directory::empty()),
                                    );
                                }
                            }
                        }
                    }
                    &"ls" => {}
                    _ => {
                        return Err(anyhow!(
                            "Unknown command \"{}\" in line \"{}\"",
                            command,
                            line
                        ));
                    }
                }
            }
            &"dir" => {
                let directory_name = parts.get(1).ok_or_else(|| {
                    anyhow!(
                        "Directory listing missing directory name in line \"{}\"",
                        line
                    )
                })?;

                let working_directory = pwd
                    .as_ref()
                    .ok_or_else(|| anyhow!("Listing files in nonexistent directory"))?;

                match hierarchy.get_mut(&working_directory.to_string()) {
                    Some(entry) => match entry {
                        DirectoryEntry::Directory(directory) => {
                            directory
                                .entry_names
                                .push(working_directory.path_for_target(directory_name));
                        }
                        DirectoryEntry::File(_) => {
                            return Err(anyhow!("pwd \"{}\" is somehow a file", working_directory));
                        }
                    },
                    None => {
                        return Err(anyhow!("No entry found for pwd \"{}\"", working_directory));
                    }
                }

                hierarchy.insert(
                    working_directory.path_for_target(directory_name),
                    DirectoryEntry::Directory(Directory::empty()),
                );
            }
            _ => {
                let size: u64 = parts
                    .get(0)
                    .ok_or_else(|| anyhow!("File missing size in line \"{}\"", line))?
                    .parse()?;
                let filename = parts
                    .get(1)
                    .ok_or_else(|| anyhow!("File missing name in line \"{}\"", line))?;

                let entry_name = match &pwd {
                    Some(working_directory) => {
                        match hierarchy.get_mut(&working_directory.to_string()) {
                            Some(entry) => match entry {
                                DirectoryEntry::Directory(directory) => {
                                    directory
                                        .entry_names
                                        .push(working_directory.path_for_target(filename));
                                }
                                DirectoryEntry::File(_) => {
                                    return Err(anyhow!(
                                        "pwd \"{}\" is somehow a file",
                                        working_directory
                                    ));
                                }
                            },
                            None => {
                                return Err(anyhow!(
                                    "No entry found for pwd \"{}\"",
                                    working_directory
                                ));
                            }
                        }

                        working_directory.path_for_target(filename)
                    }
                    None => {
                        return Err(anyhow!("File listed in nonexistent directory"));
                    }
                };

                hierarchy.insert(
                    entry_name,
                    DirectoryEntry::File(File {
                        name: filename.to_string(),
                        size,
                    }),
                );
            }
        }
    }

    let directory_sizes = calculate_directory_sizes(&hierarchy)?;

    let mut total_size = 0;
    for size in directory_sizes.values() {
        if size <= &100000 {
            total_size += size;
        }
    }

    println!("{}", total_size);

    let mut sizes: Vec<u64> = directory_sizes.values().map(|v| *v).collect();
    sizes.sort();

    let total_size = directory_sizes["/"];
    let free_space = 70000000 - total_size;
    let delta = 30000000 - free_space;

    for size in sizes {
        if size >= delta {
            println!("{}", size);
            break;
        }
    }

    Ok(())
}
