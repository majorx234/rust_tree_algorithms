#[derive(Debug)]
pub enum BinTree {
    Branch(i64, Box<BinTree>, Box<BinTree>),
    Leaf(i64),
}

// second implementation
#[derive(Debug, Default)]
struct Node<T> {
    item: T,
    left: Option<Box<Node<T>>>,
    right: Option<Box<Node<T>>>,
}

impl BinTree {
    pub fn generate_tree(depth: i64) -> Self {
        if depth == 0 {
            BinTree::Leaf(depth)
        } else {
            let left = BinTree::generate_tree(depth - 1);
            let right = BinTree::generate_tree(depth - 1);
            BinTree::Branch(depth, Box::new(left), Box::new(right))
        }
    }

    fn print_tree_level(&self, level: u32) {
        match self {
            BinTree::Branch(item, left_tree, right_tree) => {
                left_tree.print_tree_level(level + 1);
                for _ in 0..level {
                    print!("  ");
                }
                println!("B{}", item);
                right_tree.print_tree_level(level + 1);
            }
            BinTree::Leaf(item) => {
                for _ in 0..level {
                    print!("  ");
                }
                println!("L{}", item);
            }
        }
    }

    pub fn print_tree(&self) {
        self.print_tree_level(0);
    }
}
