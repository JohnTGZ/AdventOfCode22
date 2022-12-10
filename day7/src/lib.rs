// use aoc_common::FileContents;

use std::rc::Rc;
use std::cell::RefCell;
// use std::collections::HashMap;

#[cfg(test)]
mod tests {
    use super::*;

    // fn setup() -> Directory {
    //     let test_input_filepath = "../input/day7/test_input.txt";
    //     let split_delim = "\n";

    //     let file_contents = 
    //         FileContents::build(test_input_filepath, split_delim, -1, -1)
    //             .unwrap_or_else(|err| {
    //                 panic!("Unable to parse file: {err}");
    //             });
        
    //     Directory::build(&file_contents)
    // }

    // #[test]
    // fn test_parse_cd() {
    //     let directory = Directory::build(); 
    //     let assignments = setup();

    //     assert_eq!(2, 2);
    // }


    #[test]
    fn test_construct_tree() { 

        // Initialize root "/"
        let root_dir = Tree::make_root_dir("/");

        // Initialize child directory "/a/"
        let a_dir = Tree::make_dir("a", &root_dir);
        a_dir.borrow_mut().add_file("photo".to_string(), 999);

        // Make toddler directory "/a/b"
        let b_dir = Tree::make_dir("a", &a_dir);
        b_dir.borrow_mut().add_file("ugly_code".to_string(), 666);

        println!("Child tree filename: {:?}", root_dir.borrow_mut().children[0].borrow_mut().children[0].borrow_mut().files[0]);

        assert_eq!(
            File {
                name: "photo".to_string(),
                size: 999,
            }, 
            root_dir.borrow_mut().children[0].borrow_mut().files[0]
        );

        assert_eq!(
            File {
                name: "ugly_code".to_string(),
                size: 666,
            }, 
            root_dir.borrow_mut().children[0].borrow_mut().children[0].borrow_mut().files[0]
        );
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
        dir_name: &str) -> Rc<RefCell<Tree>> 
    {
        return Rc::new(RefCell::new(
            Tree {
                name: dir_name.to_string(),
                parent: None,
                children: Vec::new(),
                files: Vec::new(),
            }
        ));
    }
    
    pub fn make_dir(
        dir_name: &str, 
        parent_tree: &Rc<RefCell<Tree>>) -> Rc<RefCell<Tree>> {
    
        let child_tree = Rc::new(RefCell::new(
            Tree {
                name: dir_name.to_string(),
                parent: Some(Rc::clone(&parent_tree)),
                children: Vec::new(),
                files: Vec::new(),
            }
        ));

        //Add children to parent
        parent_tree.borrow_mut().add_child(&child_tree);
        
        return child_tree;
    }
}

#[derive(Debug)]
pub struct Directory{
    pub cwd: String,
    pub root_tree: Rc<RefCell<Tree>>,
}

impl Directory{

    pub fn ls(&self) {
        println!("{:?}:", self.cwd);
        // self.list_cwd_contents();
    }
    
    pub fn cd(&mut self, arg: &str) -> (){
        match arg {
            ".." => {
                println!("Move out one level");
                self.move_dir_up();
            },
            "/" => {
                println!("Move to root dir");
                self.move_dir_to_root();
            },
            dir_name => { 
                println!("Switch to {dir_name} dir");
                self.move_dir_to(dir_name);
            },
        }
    }

    pub fn move_dir_up(&self) -> () {
        // Remove "/X" from self.cwd 
    }

    pub fn move_dir_to_root(&self) -> () {
        while self.cwd != "/"{
            self.move_dir_up()
        }
    }

    pub fn move_dir_to(&mut self, dir_name: &str) -> () {
        let dir = "/".to_string() +  dir_name;
        self.cwd += &dir;
        // self.cwd += "/" + dir_name;
    }

}
