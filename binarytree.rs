#![feature(macro_rules,box_syntax, box_patterns)]

struct Node {
    value: i32,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn new(value: i32) -> Node {
        Node {
            value: value,
            left: None,
            right: None,
        }
    }

    fn insert(&mut self, value: i32) {
        let new_node = Some(Box::new(Node::new(value)));
        if value < self.value {
            match self.left.as_mut() {
                None => self.left = new_node,
                Some(left) => left.insert(value),
            }
        } else {
            match self.right.as_mut() {
                None => self.right = new_node,
                Some(right) => right.insert(value),
            }
        }
    }

    fn search(&self, target: i32) -> Option<i32> {
        match self.value {
            value if target == value => Some(value),
            value if target < value => self.left.as_ref()?.search(target),
            value if target > value => self.right.as_ref()?.search(target),
            _ => None,
        }
    }
}

fn main () {
    let mut my_tree: Node = Node::new(3);

    for key in vec!(2, 4, 0, 8, 11, 18, 22, 16, 12, 7, 10).iter() {
        my_tree.insert(*key);
    }

    match my_tree.search(15) {
        None => println!("not found"),
        Some(val) => {println!("{} found",val)},
    }

}