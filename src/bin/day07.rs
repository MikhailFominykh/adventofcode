use std::collections::HashMap;

#[derive(Debug)]
struct File {
    size: u32,
}

#[derive(Debug)]
struct Dir {
    nodes: HashMap<String, Node>,
}

#[derive(Debug)]
enum Node {
    Dir(Dir),
    File(File),
}

impl Node {
    fn get_dir(&mut self) -> &mut Dir {
        if let Self::Dir(dir) = self {
            return dir;
        }
        panic!();
    }
}

impl Dir {
    fn new() -> Self {
        Dir {
            nodes: HashMap::new(),
        }
    }

    fn make_directory(&mut self, name: String) {
        self.nodes.insert(name, Node::Dir(Dir::new()));
    }

    fn make_file(&mut self, name: String, size: u32) {
        self.nodes.insert(name, Node::File(File { size }));
    }

    fn get_directory<'a>(&'a mut self, path: &[&str]) -> &'a mut Dir {
        let mut cur = self;
        for name in path {
            cur = cur
                .nodes
                .entry(String::from(*name))
                .or_insert_with(|| {
                    Node::Dir(Dir {
                        nodes: HashMap::new(),
                    })
                })
                .get_dir();
        }
        cur
    }

    fn size(&self) -> u32 {
        self.nodes
            .values()
            .map(|node| match node {
                Node::Dir(dir) => dir.size(),
                Node::File(file) => file.size,
            })
            .sum()
    }

    fn filter_map_into<T, F>(&self, acc: &mut Vec<T>, f: &F)
    where
        F: Fn(&Dir) -> Option<T>,
    {
        if let Some(result) = f(self) {
            acc.push(result);
        }
        for node in self.nodes.values() {
            if let Node::Dir(dir) = node {
                dir.filter_map_into(acc, f);
            }
        }
    }
}

fn make_file_tree(lines: &[&str]) -> Dir {
    let mut root = Dir {
        nodes: HashMap::new(),
    };
    let mut path: Vec<&str> = Vec::new();
    for line in lines {
        let parts = line.split_whitespace().collect::<Vec<&str>>();
        match parts[0] {
            "$" => match parts[1] {
                "ls" => (),
                "cd" => match parts[2] {
                    "/" => path.clear(),
                    ".." => {
                        path.pop();
                    }
                    dir_name => {
                        path.push(dir_name);
                    }
                },
                _ => (),
            },
            "dir" => {
                let dir = root.get_directory(&path);
                dir.make_directory(parts[1].to_string());
            }
            file_size => {
                let dir = root.get_directory(&path);
                dir.make_file(parts[1].to_string(), file_size.parse().unwrap());
            }
        }
    }

    root
}

fn print_indent(size: usize, s: &str) {
    for _ in 0..size {
        print!("{}", s);
    }
}

fn print_tree(root: &Dir, indent_level: usize) {
    for (key, value) in &root.nodes {
        match value {
            Node::Dir(dir) => {
                print_indent(indent_level, "  ");
                println!("- {}", key);
                print_tree(&dir, indent_level + 1);
            }
            Node::File(file) => {
                print_indent(indent_level, "  ");
                println!("{}: {}", key, file.size);
            }
        }
    }
}

fn main() {
    let data = std::fs::read_to_string("data/day07.txt").unwrap();
    let lines = data.lines().collect::<Vec<&str>>();
    let root = make_file_tree(&lines);
    let mut mapped: Vec<u32> = Vec::new();
    root.filter_map_into(&mut mapped, &|d| {
        let size = d.size();
        if size <= 100000 {
            return Some(size);
        } else {
            return None;
        }
    });
    let sum = mapped.iter().sum::<u32>();
    println!("Part 1: {}", sum);

    const TOTAL_SPACE: i32 = 70000000;
    const NEEDED_SPACE: i32 = 30000000;
    let total_size = root.size() as i32;
    let free_space = TOTAL_SPACE - total_size;
    let min_dir_size = NEEDED_SPACE - free_space;
    let mut candidates_for_deletion = Vec::new();
    root.filter_map_into(&mut candidates_for_deletion, &|d| {
        let size = d.size();
        if size as i32 >= min_dir_size {
            return Some(size);
        } else {
            return None;
        }
    });
    candidates_for_deletion.sort();
    println!("Part 2: {}", candidates_for_deletion[0]);
}
