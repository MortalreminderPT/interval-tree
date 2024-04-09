use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

struct Node {
    left: i32,
    right: i32,
    value: i32,
}

impl Node {
    fn new(left: i32, right: i32) -> Node {
        Node {
            left,
            right,
            value: 0,
        }
    }
}

struct Tree {
    node: Rc<RefCell<Node>>,
    left_child: Option<Rc<RefCell<Tree>>>,
    right_child: Option<Rc<RefCell<Tree>>>,
}

impl Tree {
    fn new() -> Tree {
        Tree {
            node: Rc::new(RefCell::new(Node::new(0, 1))),
            left_child: None,
            right_child: None,
        }
    }

    fn from_node(node: Node) -> Tree {
        let node = Rc::new(RefCell::new(node));
        Tree {
            node: node,
            left_child: None,
            right_child: None,
        }
    }

    fn add_node(&mut self, new_left: i32, new_right: i32) {
        if new_left >= new_right { return }
        let node = &self.node;
        let left = node.borrow().left;
        let right = node.borrow().right;
        if new_left == left && new_right == right {
            return;
        }
        // println!("Adding [{new_left}, {new_right}]");
        if new_right <= left {
            let node = Node::new(new_left, new_right);
            let tree = Tree::from_node(node);
            match &self.left_child {
                None => self.left_child = Some(Rc::new(RefCell::new(tree))),
                Some(left_child) => {
                    left_child.borrow_mut().add_node(new_left, new_right)
                },
            };
        } else if new_left >= right {
            let node = Node::new(new_left, new_right);
            let tree = Tree::from_node(node);
            match &self.right_child {
                None => self.right_child = Some(Rc::new(RefCell::new(tree))),
                Some(right_child) => right_child.borrow_mut().add_node(new_left, new_right),
            };
        }
        else {
            let mut set = HashSet::new();
            set.insert(left);
            set.insert(right);
            set.insert(new_left);
            set.insert(new_right);
            let mut inters = set.iter().map(|v|*v).collect::<Vec<i32>>();// vec![left, right, new_left, new_right];
            inters.sort();
            let mut lid = 0;
            for i in 0..inters.len() - 1 {
                if inters[i] == left {
                    self.node.borrow_mut().right = inters[i + 1];
                    lid = i;
                    break;
                }
            }
            for i in 0..inters.len() - 1 {
                if i != lid {
                    self.add_node(inters[i], inters[i + 1]);
                }
            }
        }
    }

    fn update_node(&mut self, new_left: i32, new_right: i32) {
        if new_left >= new_right { return }
        let node = &self.node;
        let left = node.borrow().left;
        let right = node.borrow().right;
        if new_left == left && new_right == right {
            return;
        }
        // println!("Adding [{new_left}, {new_right}]");
        if new_right <= left {
            let node = Node::new(new_left, new_right);
            let tree = Tree::from_node(node);
            match &self.left_child {
                None => self.left_child = Some(Rc::new(RefCell::new(tree))),
                Some(left_child) => {
                    left_child.borrow_mut().add_node(new_left, new_right)
                },
            };
        } else if new_left >= right {
            let node = Node::new(new_left, new_right);
            let tree = Tree::from_node(node);
            match &self.right_child {
                None => self.right_child = Some(Rc::new(RefCell::new(tree))),
                Some(right_child) => right_child.borrow_mut().add_node(new_left, new_right),
            };
        }
        else {
            let mut set = HashSet::new();
            set.insert(left);
            set.insert(right);
            set.insert(new_left);
            set.insert(new_right);
            let mut inters = set.iter().map(|v|*v).collect::<Vec<i32>>();// vec![left, right, new_left, new_right];
            inters.sort();
            let mut lid = 0;
            for i in 0..inters.len() - 1 {
                if inters[i] == left {
                    self.node.borrow_mut().right = inters[i + 1];
                    lid = i;
                    break;
                }
            }
            for i in 0..inters.len() - 1 {
                if i != lid {
                    self.add_node(inters[i], inters[i + 1]);
                }
            }
        }
    }

    fn display(&self) {
        match &self.left_child {
            None => {},
            Some(left_child) => left_child.borrow().display(),
        };
        println!("[{}, {}]: {}", self.node.borrow().left, self.node.borrow().right, self.node.borrow().value);
        match &self.right_child {
            None => {},
            Some(right_child) => right_child.borrow().display(),
        };
    }
}

fn main() {
    let mut tree = Tree::new();
    tree.add_node(-10000000, 1000000000);
    tree.add_node(-1321561113, 1321561113);
    tree.add_node(1, 2);
    tree.add_node(1, 3);
    tree.add_node(1, 5);
    tree.add_node(1, 6);
    tree.add_node(1, 7);
    tree.add_node(1, 90);
    tree.add_node(-10, 10);
    tree.add_node(-10, 10);
    tree.add_node(-10, 10);
    tree.add_node(-10, 10);
    tree.add_node(6, 8);
    tree.add_node(3, 4);
    tree.add_node(1, 2);
    tree.add_node(1, 1);
    tree.add_node(600, 800);
    tree.add_node(1, 2);
    tree.add_node(3, 4);
    tree.add_node(-2, -1);
    tree.add_node(3, 4);
    tree.display();
}