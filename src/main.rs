use std::cell::RefCell;
use std::collections::HashSet;
use std::rc::Rc;

trait Value {
    fn display(&self) -> String;
}

impl Value for String {
    fn display(&self) -> String {
        self.to_string()
    }
}

impl Value for &str {
    fn display(&self) -> String {
        self.to_string()
    }
}

struct Node<T: Value + Clone + Default> {
    left: i32,
    right: i32,
    value: T,
}

impl<T: Value + Clone + Default> Node<T> {
    fn new(left: i32, right: i32, value: &T) -> Node<T> {
        Node {
            left,
            right,
            value: value.clone(),
        }
    }
}

struct Tree<T: Value + Clone + Default> {
    node: Rc<RefCell<Node<T>>>,
    left_child: Option<Rc<RefCell<Tree<T>>>>,
    right_child: Option<Rc<RefCell<Tree<T>>>>,
}

impl<T: Value + Clone + Default> Tree<T> {
    fn new() -> Tree<T> {
        Tree {
            node: Rc::new(RefCell::new(Node::new(0, 1, &Default::default()))),
            left_child: None,
            right_child: None,
        }
    }

    fn from_node(node: Node<T>) -> Tree<T> {
        let node = Rc::new(RefCell::new(node));
        Tree {
            node,
            left_child: None,
            right_child: None,
        }
    }

    // fn add_node(&mut self, new_left: i32, new_right: i32) {
    //     if new_left >= new_right { return }
    //     let node = &self.node;
    //     let left = node.borrow().left;
    //     let right = node.borrow().right;
    //     if new_left == left && new_right == right {
    //         return;
    //     }
    //     println!("Adding [{new_left}, {new_right}]");
    //     if new_right <= left {
    //         let node = Node::new(new_left, new_right);
    //         let tree = Tree::from_node(node);
    //         match &self.left_child {
    //             None => self.left_child = Some(Rc::new(RefCell::new(tree))),
    //             Some(left_child) => {
    //                 left_child.borrow_mut().add_node(new_left, new_right)
    //             },
    //         };
    //     } else if new_left >= right {
    //         let node = Node::new(new_left, new_right);
    //         let tree = Tree::from_node(node);
    //         match &self.right_child {
    //             None => self.right_child = Some(Rc::new(RefCell::new(tree))),
    //             Some(right_child) => right_child.borrow_mut().add_node(new_left, new_right),
    //         };
    //     }
    //     else {
    //         let mut set = HashSet::new();
    //         set.insert(left);
    //         set.insert(right);
    //         set.insert(new_left);
    //         set.insert(new_right);
    //         let mut inters = set.iter().map(|v|*v).collect::<Vec<i32>>();// vec![left, right, new_left, new_right];
    //         inters.sort();
    //         let mut lid = 0;
    //         for i in 0..inters.len() - 1 {
    //             if inters[i] == left {
    //                 self.node.borrow_mut().right = inters[i + 1];
    //                 lid = i;
    //                 break;
    //             }
    //         }
    //         for i in 0..inters.len() - 1 {
    //             if i != lid {
    //                 self.add_node(inters[i], inters[i + 1]);
    //             }
    //         }
    //     }
    // }

    fn update_node(&mut self, new_left: i32, new_right: i32, value: &T) {
        if new_left >= new_right { return }
        let node = &self.node;
        let left = node.borrow().left;
        let right = node.borrow().right;
        let old_value = &self.node.borrow().value.clone();
        if left >= new_left && right <= new_right {
            self.node.borrow_mut().value = value.clone();
        }
        if left == new_left && right == new_right {
            return;
        }
        // println!("Adding [{new_left}, {new_right}], {old_value} -> {}", value.to_string());
        if new_right <= left {
            let node = Node::new(new_left, new_right, value);
            let tree = Tree::from_node(node);
            match &self.left_child {
                None => self.left_child = Some(Rc::new(RefCell::new(tree))),
                Some(left_child) => {
                    left_child.borrow_mut().update_node(new_left, new_right, value);
                },
            };
        } else if new_left >= right {
            let node = Node::new(new_left, new_right, value);
            let tree = Tree::from_node(node);
            match &self.right_child {
                None => self.right_child = Some(Rc::new(RefCell::new(tree))),
                Some(right_child) => right_child.borrow_mut().update_node(new_left, new_right, value),
            };
        } else {
            let mut set = HashSet::new();
            set.insert(left);
            set.insert(right);
            set.insert(new_left);
            set.insert(new_right);
            let mut inters = set.iter().map(|v| *v).collect::<Vec<i32>>();// vec![left, right, new_left, new_right];
            inters.sort();
            let mut lid = 0;
            for i in 0..inters.len() - 1 {
                if inters[i] == left {
                    self.node.borrow_mut().right = inters[i + 1];
                    if inters[i] >= new_left && inters[i + 1] <= new_right {
                        self.node.borrow_mut().value = value.clone();
                    }
                    lid = i;
                    break;
                }
            }
            for i in 0..inters.len() - 1 {
                if i != lid {
                    if inters[i] >= new_left && inters[i + 1] <= new_right {
                        self.update_node(inters[i], inters[i + 1], value);
                    } else {
                        self.update_node(inters[i], inters[i + 1], old_value);
                    }
                }
            }
        }
    }

    fn display(&self) {
        match &self.left_child {
            None => {},
            Some(left_child) => left_child.borrow().display(),
        };
        println!("[{}, {}]: {}", self.node.borrow().left, self.node.borrow().right, self.node.borrow().value.display());
        match &self.right_child {
            None => {},
            Some(right_child) => right_child.borrow().display(),
        };
    }
}

impl<T: Value + Clone + Default> Tree<T> {
    fn update(&mut self, left: i32, right: i32, value: &T) -> Option<()> {
        if right - left < 1 {
            None
        } else {
            self.update_node(left, right, value);
            Some(())
        }
    }
}

fn main() {
    let mut tree = Tree::new();
    tree.update(0, 1, &"0..1");
    tree.update(-100, 0, &"-100..0");
    tree.update(1, 2, &"1..2");
    tree.update(1, 3, &"1..3");
    tree.update(1, 5, &"1..5");
    tree.update(1, 6, &"1..6");
    tree.update(1, 7, &"1..7");
    tree.update(0, 4, &"0..4");
    tree.update(1, 90, &"1..90");
    tree.update(-10, 10, &"-10..10");
    tree.update(-10, 10, &"-10..10");
    tree.update(-10, 10, &"-10..10");
    tree.update(-10, 10, &"-10..10");
    tree.update(6, 8, &"6..8");
    tree.update(3, 4, &"3..4");
    tree.update(1, 2, &"1..2");
    tree.update(1, 1, &"1..1"); // can't update
    tree.update(600, 800, &"600..800");
    tree.update(1, 2, &"1..2");
    tree.update(3, 4, &"3..4");
    tree.update(-2, -1, &"-2..-1");
    tree.update(3, 4, &"3..4");
    tree.update(-10, 10, &"-10..10");
    tree.display();
}