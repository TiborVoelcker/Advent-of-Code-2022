use std::cell::RefCell;
use std::fmt;
use std::rc::Rc;

#[derive(Clone, Eq)]
pub struct TreeNode {
    pub name: String,
    pub size: Option<u32>,
    pub children: Vec<Rc<RefCell<TreeNode>>>,
    pub parent: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    pub fn new(name: &str, size: Option<u32>) -> TreeNode {
        TreeNode {
            name: name.to_string(),
            size,
            children: vec![],
            parent: None,
        }
    }

    pub fn new_ref(name: &str, size: Option<u32>) -> Rc<RefCell<TreeNode>> {
        Rc::new(RefCell::new(TreeNode::new(name, size)))
    }

    pub fn next_sibling(&self) -> Option<Rc<RefCell<TreeNode>>> {
        match &self.parent {
            Some(parent) => {
                let pos = Rc::clone(parent)
                    .borrow()
                    .children
                    .iter()
                    .position(|r| r.borrow().name == self.name)
                    .unwrap();
                if pos != parent.borrow().children.len() - 1 {
                    Some(Rc::clone(&parent.borrow().children[pos + 1]))
                } else {
                    None
                }
            }
            None => None,
        }
    }

    pub fn print_tree(&self, prepend: String) -> String {
        let print_last_child = |child: &Rc<RefCell<TreeNode>>| {
            prepend.clone() + "┗━" + &child.borrow().print_tree(prepend.clone() + "  ")
        };

        let mut print_str = String::from(&self.name);
        if let Some(size) = self.size {
            print_str += &(String::from(" (") + &size.to_string() + ")");
        }
        print_str += "\n";

        if self.children.len() >= 2 {
            print_str
                + &self.children[0..self.children.len() - 1]
                    .iter()
                    .map(|tn| {
                        prepend.clone() + "┣━" + &tn.borrow().print_tree(prepend.clone() + "┃ ")
                    })
                    .collect::<Vec<String>>()
                    .join("")
                + &print_last_child(&self.children[self.children.len() - 1])
        } else if self.children.len() == 1 {
            print_str + &print_last_child(&self.children[0])
        } else {
            print_str
        }
    }

    pub fn calc_size(&mut self) -> u32 {
        if let None = self.size {
            self.size = Some(
                self.children
                    .iter()
                    .map(|tn| tn.borrow_mut().calc_size())
                    .sum(),
            );
        }
        self.size.unwrap()
    }
}

impl fmt::Debug for TreeNode {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "TreeNode {{ name: {:?}, size: {:?} }}",
            self.name, self.size
        )
    }
}

impl PartialEq for TreeNode {
    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

pub fn init_tree(lines: std::str::Lines) -> Rc<RefCell<TreeNode>> {
    let mut lines = lines.peekable();
    let root = TreeNode::new_ref("/", None);
    let mut current = Rc::clone(&root);

    while let Some(line) = lines.next() {
        let split: Vec<&str> = line.split(" ").collect();
        match split[1] {
            "cd" => match split[2] {
                ".." => {
                    println!("Going up");
                    let current_clone = Rc::clone(&current);
                    current = Rc::clone(current_clone.borrow().parent.as_ref().unwrap());
                }
                "/" => {
                    println!("Going to root");
                    current = Rc::clone(&root);
                }
                name => {
                    println!("Going to {name}");
                    let current_clone = Rc::clone(&current);
                    current = Rc::clone(
                        current_clone
                            .borrow()
                            .children
                            .iter()
                            .find(|&child| child.borrow().name == name)
                            .unwrap(),
                    );
                }
            },
            "ls" => {
                while let Some(line) = lines.peek() {
                    if line.split(" ").collect::<Vec<&str>>()[0] == "$" {
                        break;
                    }

                    let line = lines.next().unwrap();
                    println!("Processing ls for '{line}'");
                    let split: Vec<&str> = line.split(" ").collect();

                    let child;
                    match split[0] {
                        "dir" => child = TreeNode::new_ref(split[1], None),
                        size => {
                            child = TreeNode::new_ref(split[1], Some(size.parse().unwrap()));
                        }
                    }
                    current.borrow_mut().children.push(Rc::clone(&child));

                    let mut mut_child = child.borrow_mut();
                    mut_child.parent = Some(Rc::clone(&current));
                }
            }
            _ => panic!("Unknown command: '{line}'"),
        }
    }
    return root;
}

pub struct TreeNodeIterator {
    root: Rc<RefCell<TreeNode>>,
    stack: Vec<Rc<RefCell<TreeNode>>>,
}

impl Iterator for TreeNodeIterator {
    type Item = Rc<RefCell<TreeNode>>;

    fn next(&mut self) -> Option<Self::Item> {
        let mut current;

        current = match self.stack.last() {
            None => Some(Rc::clone(&self.root)),
            Some(last) => match last.borrow().children.first() {
                Some(child) => Some(Rc::clone(child)),
                None => None,
            },
        };

        if current == None {
            while let Some(parent) = self.stack.pop() {
                match parent.borrow().next_sibling() {
                    Some(sibling) => {
                        current = Some(sibling);
                        break;
                    }
                    None => continue,
                }
            }
        }

        match current {
            Some(curr) => {
                self.stack.push(Rc::clone(&curr));
                Some(curr)
            }
            None => None,
        }
    }
}

impl IntoIterator for TreeNode {
    type Item = Rc<RefCell<TreeNode>>;

    type IntoIter = TreeNodeIterator;

    fn into_iter(self) -> Self::IntoIter {
        TreeNodeIterator {
            root: Rc::new(RefCell::new(self)),
            stack: vec![],
        }
    }
}
