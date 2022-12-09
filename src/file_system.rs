use std::{
    collections::{HashMap, VecDeque},
    fmt::Debug,
    str::FromStr,
};

pub enum Command {
    ChangeDirectory(String),
    List,
}

impl FromStr for Command {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<_> = s.split(' ').collect();

        if parts[0] != "$" {
            return Err(());
        }

        match parts[1] {
            "cd" => {
                let path = parts[2];
                Ok(Command::ChangeDirectory(path.to_owned()))
            }
            "ls" => Ok(Command::List),
            _e => Err(()),
        }
    }
}

pub struct FileSystem {
    contents: HashMap<String, Vec<FileOrDir>>,
}

impl FileSystem {
    fn new(contents: HashMap<String, Vec<FileOrDir>>) -> Self {
        Self { contents }
    }

    pub fn dir_sizes(&self) -> HashMap<String, u32> {
        let mut sizes = HashMap::new();
        for dir in self.contents.keys() {
            sizes.insert(dir.to_owned(), self.dir_size(dir));
        }
        sizes
    }

    fn dir_size(&self, dir: &str) -> u32 {
        // dbg!(&self.contents);
        let mut size_sum = 0;
        for file_or_dir in self
            .contents
            .get(dir)
            .unwrap_or_else(|| panic!("no entry for dir '{dir}'"))
        {
            size_sum += match file_or_dir {
                FileOrDir::Dir { name } => self.dir_size(name),
                FileOrDir::File { name, size } => *size,
            };
        }
        size_sum
    }
}

pub fn traverse_file_system(lines: &[String]) -> FileSystem {
    let mut idx = 0;
    let mut dirs = vec!["".to_owned()];
    let mut files: HashMap<String, Vec<FileOrDir>> = HashMap::new();

    loop {
        if idx >= lines.len() {
            break;
        }
        let line = &lines[idx];
        let current_dir = dirs.last().unwrap();

        idx += 1;
        if line.starts_with('$') {
            // command
            let cmd = Command::from_str(line).unwrap();
            match cmd {
                Command::ChangeDirectory(dir) => {
                    if dir == ".." {
                        dirs.pop();
                    } else if dir == "/" {
                        dirs.push(dir);
                    } else {
                        dirs.push(format!("{}/{}", current_dir, dir));
                    }
                }
                Command::List => loop {
                    if idx >= lines.len() {
                        break;
                    }
                    let ls_line = &lines[idx];
                    if ls_line.starts_with('$') {
                        break;
                    } else if ls_line.starts_with("dir") {
                        let parts: Vec<_> = ls_line.split(' ').collect();
                        let name = format!("{}/{}", current_dir, parts[1].to_owned());
                        files
                            .entry(current_dir.to_owned())
                            .or_default()
                            .push(FileOrDir::Dir { name });
                    } else {
                        let parts: Vec<_> = ls_line.split(' ').collect();
                        let size = parts[0].parse().unwrap();
                        let name = parts[1].to_owned();
                        files
                            .entry(current_dir.clone())
                            .or_default()
                            .push(FileOrDir::File { name, size });
                    }
                    idx += 1;
                },
            }
        }
    }
    FileSystem::new(files)
}

pub enum FileOrDir {
    Dir { name: String },
    File { name: String, size: u32 },
}

impl Debug for FileOrDir {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Dir { name } => write!(f, "file {name}"),
            Self::File { name, size } => write!(f, "dir {name}"),
        }
    }
}
