use std::{collections::HashSet, fs};

pub type FileId = usize;

#[derive(Clone, Debug)]
pub enum SOC {
    Size(usize),
    Children(HashSet<FileId>),
}

#[derive(Clone, Debug)]
pub struct File {
    name: String,
    id: FileId,
    parent: Option<FileId>,
    soc: SOC,
}

impl File {
    fn size(&self, finder: &Finder) -> usize {
        match &self.soc {
            SOC::Size(x) => *x,
            SOC::Children(x) => x.iter().map(|x| finder.get(*x).unwrap().size(finder)).sum(),
        }
    }
}

pub struct Finder {
    curr: FileId,
    files: Vec<File>,
}

impl Finder {
    pub fn from(root: &str) -> Finder {
        Finder {
            curr: 0,
            files: vec![File {
                name: root.to_string(),
                id: 0,
                parent: None,
                soc: SOC::Children(HashSet::new()),
            }],
        }
    }

    pub fn get(&self, id: FileId) -> Option<&File> {
        self.files.get(id)
    }

    pub fn get_mut(&mut self, id: FileId) -> Option<&mut File> {
        self.files.get_mut(id)
    }

    pub fn add(&mut self, name: String, soc: SOC) {
        let id = self.files.len();
        if let SOC::Children(children) = &mut self.curr_mut().unwrap().soc {
            children.insert(id);
        }
        self.files.push(File {
            name,
            soc,
            id,
            parent: Some(self.curr),
        });
    }

    pub fn cd(&mut self, name: &str) -> bool {
        if name == ".." {
            self.curr = self.curr().unwrap().parent.unwrap();
            return true;
        }
        let mut curr = None;
        if let SOC::Children(children) = &self.curr().unwrap().soc {
            for child in children {
                if let Some(child) = self.files.get(*child) {
                    if name == child.name {
                        curr = Some(child.id);
                        break;
                    }
                }
            }
        }
        if let Some(curr) = curr {
            self.curr = curr;
            return true;
        }
        false
    }

    pub fn curr(&self) -> Option<&File> {
        self.get(self.curr)
    }

    pub fn curr_mut(&mut self) -> Option<&mut File> {
        self.get_mut(self.curr)
    }

    pub fn parse(&mut self, line: &str) {
        let mut args = line.split_whitespace();
        match args.next() {
            Some("$") => {
                if args.next() == Some("cd") {
                    self.cd(args.next().unwrap());
                }
            }
            Some("dir") => self.add(
                args.next().unwrap().to_string(),
                SOC::Children(HashSet::new()),
            ),
            Some(x) => self.add(
                args.next().unwrap().to_string(),
                SOC::Size(x.parse().unwrap()),
            ),
            None => {}
        }
    }
}

pub fn puzzle1(path: &str) -> usize {
    let mut finder = Finder::from("/");
    for line in fs::read_to_string(path).unwrap().lines() {
        finder.parse(line);
    }
    finder
        .files
        .iter()
        .filter(|f| match f.soc {
            SOC::Children(..) => true,
            SOC::Size(..) => false,
        })
        .map(|f| f.size(&finder))
        .filter(|&x| x <= 100_000)
        .sum()
}

pub fn puzzle2(path: &str) -> Option<usize> {
    let mut finder = Finder::from("/");
    for line in fs::read_to_string(path).unwrap().lines() {
        finder.parse(line);
    }
    if finder.files[0].size(&finder) < 40_000_000 {
        return None;
    }
    let needed = finder.files[0].size(&finder) - 40_000_000;
    finder
        .files
        .iter()
        .filter(|f| match f.soc {
            SOC::Children(..) => true,
            SOC::Size(..) => false,
        })
        .map(|f| f.size(&finder))
        .filter(|&f| f >= needed)
        .min()
}
