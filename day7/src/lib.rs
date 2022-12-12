use std::rc::Rc;
use std::cell::RefCell;
use std::collections::HashMap;

use aoc_common::FileContents;

#[cfg(test)]
mod tests {
    use super::*;

    fn setup(input_file: &str) -> HashMap<String, u32>{
        // Initialize root "/"
        let root_tree = Tree::make_root_dir("/");
        let mut directory = Directory::build(&root_tree);

        let test_input_filepath = input_file;
        let split_delim = "\n";

        let file_contents = 
            FileContents::build(test_input_filepath, split_delim, -1, -1)
                .unwrap_or_else(|err| {
                    panic!("Unable to parse file: {err}");
                });
        
        let mut prev_cmd = ""; 

        for line in &file_contents.split_contents{
            // Split line further to distinguish between input and output
            let words: Vec<&str> = line.split(" ").collect();

            match words[0] {
                "$" => {
                    match words[1] {
                        "cd" => {
                            match words[2] {
                                "/" => {
                                    directory.move_dir_to_root()
                                },
                                ".." => {
                                    directory.move_dir_up();
                                },
                                dir_name => {
                                    let dir_name_resolved = dir_name.to_owned();
                                    directory.add_dir(&dir_name_resolved);
                                    directory.move_dir_to(&dir_name_resolved);
                                }
                            };
                        },
                        "ls" => {
                            // Do nothing here
                        },
                        _ => {
                            panic!("Invalid command")
                        }
                    };
                    prev_cmd = words[1];
                },
                _ => {
                    match prev_cmd {
                        "cd" => {
                            panic!("'cd' is not supposed to have any output")
                        },
                        "ls" => {
                            match words[0] {
                                "dir" => {
                                    let dir_name_resolved = words[1].to_owned();
                                    directory.add_dir(&dir_name_resolved);
                                },
                                _ => {
                                    directory.add_file(
                                        words[1].to_string(), 
                                        words[0].parse::<u32>().unwrap());
                                }
                            }
                        },
                        _ => {
                            panic!("Invalid previous command");
                        }
                    }
                },
            }
        };

        let mut dir_storage: HashMap<String, u32> = HashMap::new();
        
        directory.get_directory_storage_occupied(&directory.root_tree, &mut dir_storage);

        // add root directory storage
        let outer_abs_dir = &directory.root_tree.borrow().abs_dir;  
        dir_storage.entry(outer_abs_dir.to_string()).or_insert(0);
        // add files to root directory
        for file in &directory.directories[outer_abs_dir].borrow().files{
            dir_storage.entry(outer_abs_dir.to_string())
                        .and_modify(|f_size| *f_size += file.size);
        }
        //Add child directories currently within root directory
        for grandchild in  &directory.root_tree.borrow().children {
            
            let grandchild_abs_dir = &grandchild.borrow().abs_dir;
            let grandchild_space = dir_storage[grandchild_abs_dir];
            dir_storage.entry(outer_abs_dir.to_string())
                        .and_modify(|f_size| *f_size += grandchild_space);
        } 

        dir_storage
    }

    #[test]
    fn test_example_tree() { 
        // Initialize root "/"
        let root_dir = Tree::make_root_dir("/");

        // Initialize child directory "/a/"
        let a_dir = Tree::make_dir("a", &root_dir);
        a_dir.borrow_mut().add_file("photo".to_string(), 999);

        // Make toddler directory "/a/b"
        let b_dir = Tree::make_dir("b", &a_dir);
        b_dir.borrow_mut().add_file("ugly_code".to_string(), 666);

        // Make fetus directory "/a/b/c"
        let c_dir = Tree::make_dir("c", &b_dir);

        // Make nephew directory "/a/d"
        let d_dir = Tree::make_dir("d", &a_dir);

        // Make sibling directory "/e"
        let e_dir = Tree::make_dir("e", &root_dir);

        // Test directory names
        assert_eq!(
            File {
                name: "photo".to_string(),
                size: 999,
            }, 
            root_dir.borrow().children[0].borrow().files[0]
        );

        assert_eq!(
            File {
                name: "ugly_code".to_string(),
                size: 666,
            }, 
            root_dir.borrow().children[0].borrow().children[0].borrow().files[0]
        );
        
        // Test directory names
        assert_eq!("/a/b/c/", c_dir.borrow().abs_dir);
        assert_eq!("/a/d/", d_dir.borrow().abs_dir);
        assert_eq!("/e/", e_dir.borrow().abs_dir);

    }

    #[test]
    fn test_dir_space() {
        let dir_storage = setup("../input/day7/test_input.txt");

        let mut sum_small_storage: u32 = 0;
        for (_, space) in &dir_storage{
            if *space < 100000 as u32 {
                sum_small_storage += space;
            }
        }

        assert_eq!(dir_storage["/a/e/"], 584);
        assert_eq!(dir_storage["/a/"], 94853);
        assert_eq!(dir_storage["/d/"], 24933642);
        assert_eq!(dir_storage["/"], 48381165);
        assert_eq!(sum_small_storage, 95437);
    }

    #[test]
    fn do_part1() {
        let dir_storage = setup("../input/day7/final_input.txt");

        let mut sum_storage: u32 = 0;
        for (_, space) in &dir_storage{
            if *space < 100000 as u32 {
                sum_storage += space;
            }
        }

        println!("Part 1 answer: {}", sum_storage);
    }

    #[test]
    fn test_part2() {
        let dir_storage = setup("../input/day7/test_input.txt");

        let max_space:u32 = 70000000;

        let total_storage_occupied: u32 = dir_storage["/"];
        assert_eq!(total_storage_occupied, 48381165);

        let to_free_up = 30000000 - (max_space - total_storage_occupied);

        let mut smallest_candidates_storage: u32 = max_space;
        let mut smallest_candidate = "".to_string();

        for (dir_name, space) in &dir_storage{
            if *space <= smallest_candidates_storage 
                && *space >= to_free_up
            {
                smallest_candidate = dir_name.clone();
                smallest_candidates_storage = *space;
            }

        }

        assert_eq!(smallest_candidates_storage, 24933642);
        assert_eq!(smallest_candidate, "/d/");
    }

    #[test]
    fn do_part2() {
        let dir_storage = setup("../input/day7/test_input.txt");

        let max_space:u32 = 70000000;

        let total_storage_occupied: u32 = dir_storage["/"];

        let to_free_up = 30000000 - (max_space - total_storage_occupied);

        let mut smallest_candidates_storage: u32 = max_space;
        let mut smallest_candidate = "".to_string();

        for (dir_name, space) in &dir_storage{
            if *space <= smallest_candidates_storage 
                && *space >= to_free_up
            {
                smallest_candidate = dir_name.clone();
                smallest_candidates_storage = *space;
            }

        }

        println!("Part 2 answer: dir '{}' which occupies {}", smallest_candidate, smallest_candidates_storage);
    }

}


#[derive(Debug)]
pub struct File {
    pub name: String,
    pub size: u32,
}

impl PartialEq for File{
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name && 
        self.size == other.size
    }
}

#[derive(Debug)]
pub struct Tree{
    pub name: String,
    pub abs_dir: String,
    pub parent: Option<Rc<RefCell<Tree>>>,
    pub children: Vec<Rc<RefCell<Tree>>>,
    pub files: Vec<File>,
}

impl Tree{
    pub fn add_child(&mut self, child: &Rc<RefCell<Tree>> ) -> () {
        // Add children to parent tree
        self.children.push(Rc::clone(&child));
    }

    pub fn add_file(&mut self, name: String, size: u32) -> () {
        self.files.push(
            File {
                name,
                size,
            }
        );
    }

    pub fn make_root_dir(
        dir_name: &str) 
        -> Rc<RefCell<Tree>> 
    {
        return Rc::new(RefCell::new(
            Tree {
                name: dir_name.to_string(),
                abs_dir: dir_name.to_string(),
                parent: None,
                children: Vec::new(),
                files: Vec::new(),
            }
        ));
    }
    
    pub fn make_dir(
        dir_name: &str, 
        parent_tree: &Rc<RefCell<Tree>>) 
        -> Rc<RefCell<Tree>> {
        
        let new_abs_dir = parent_tree.borrow().abs_dir.clone() + dir_name + "/";

        let child_tree = Rc::new(RefCell::new(
            Tree {
                name: dir_name.to_string(),
                abs_dir: new_abs_dir,
                parent: Some(Rc::clone(&parent_tree)),
                children: Vec::new(),
                files: Vec::new(),
            }
        ));

        //Add children to parent
        parent_tree.borrow_mut().add_child(&child_tree);
        
        return child_tree;
    }

    pub fn print_tree(&self, hierarchy_spacing: String) -> (){
        let mut hierarchy_spacing_child = hierarchy_spacing.clone();
        
        hierarchy_spacing_child += "  ";

        for child in &self.children {
            child.borrow_mut().print_tree(hierarchy_spacing_child.clone());
        }
    }

}

#[derive(Debug)]
pub struct Directory{
    pub cwd: String,
    pub directories: HashMap<String, Rc<RefCell<Tree>>>,
    pub root_tree: Rc<RefCell<Tree>>,
}

impl Directory{

    pub fn build(root_tree: &Rc<RefCell<Tree>>) -> Directory {
        let mut directories = HashMap::new();
        directories.insert(root_tree.borrow().abs_dir.to_string(), Rc::clone(&root_tree));

        Directory{
            cwd: "/".to_string(),
            directories: directories,
            root_tree: Rc::clone(&root_tree),
        }
    }

    pub fn add_dir(&mut self, dir_name: &str) -> () {
        let abs_dir_name = self.get_abs_dir(&dir_name);

        if self.check_dir_exists(&abs_dir_name){
            return;
        }

        let parent_tree = match self.directories.get(&self.cwd) {
            Some(parent_tree) => {
                parent_tree
            },
            None => {
                panic!("Parent directory '{}' does not exist", &self.cwd);
            },
        };

        let new_tree = Tree::make_dir(&dir_name, &parent_tree);

        self.directories.insert(new_tree.borrow().abs_dir.to_string(), Rc::clone(&new_tree));
    }

    pub fn add_file(&mut self, name: String, size: u32) -> () {
        
        let tree = match self.directories.get(&self.cwd) {
            Some(tree) => {
                tree
            },
            None => {
                panic!("directory '{}' does not exist", &self.cwd);
            },
        };

        tree.borrow_mut().add_file(name, size);
    }

    pub fn move_dir_up(&mut self) -> () {
        let split_dir: Vec<&str> = self.cwd.rsplitn(3, "/").collect();

        self.cwd = split_dir[2].to_string() + "/";
    }

    pub fn move_dir_to_root(&mut self) -> () {
        self.cwd = String::from("/");
    }

    pub fn move_dir_to(&mut self, dir_name: &str) -> () {
        let abs_dir_name = self.get_abs_dir(&dir_name);
        if !self.check_dir_exists(&abs_dir_name){
            panic!("Directory {} does not exist", abs_dir_name);
        }

        self.cwd = abs_dir_name;
    }

    fn get_abs_dir(&self, dir_name: &str) -> String {
        return self.cwd.clone() + dir_name + "/";
    }

    fn check_dir_exists(&self, abs_dir_name: &str) -> bool {
        self.directories.contains_key(abs_dir_name)
    }

    pub fn get_directory_storage_occupied(
        &self, 
        tree: &Rc<RefCell<Tree>>,
        dir_storage: &mut HashMap<String, u32>) -> () {
        
        for child in &tree.borrow().children {
            self.get_directory_storage_occupied(child, dir_storage);
            
            let abs_dir = &child.borrow().abs_dir;

            //Add files curernt within directory
            dir_storage.entry(abs_dir.to_string()).or_insert(0);
            for file in &self.directories[abs_dir].borrow().files{
                dir_storage.entry(abs_dir.to_string())
                            .and_modify(|f_size| *f_size += file.size);
            }

            //Add child directories currently within directory
            for grandchild in &child.borrow().children {
                
                let grandchild_abs_dir = &grandchild.borrow().abs_dir;
                let grandchild_space = dir_storage[grandchild_abs_dir];
                
                dir_storage.entry(abs_dir.to_string())
                            .and_modify(|f_size| *f_size += grandchild_space);
            } 

        }

    }

}
