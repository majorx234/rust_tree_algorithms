use rand::Rng;

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
    fn generate_tree_recursive(depth: i64, index: &mut i64) -> Self {
        if depth == 0 {
            *index += 1;
            BinTree::Leaf(*index)
        } else {
            *index += 1;
            let item = *index;
            let left = Self::generate_tree_recursive(depth - 1, index);
            let right = Self::generate_tree_recursive(depth - 1, index);

            BinTree::Branch(item, Box::new(left), Box::new(right))
        }
    }

    pub fn generate_tree(depth: i64) -> Self {
        let mut rng = rand::thread_rng();
        Self::generate_tree_recursive(depth, &mut 0)
    }

    fn generate_random_tree_recursive<R: Rng + ?Sized>(
        depth: i64,
        index: &mut i64,
        rng: &mut R,
    ) -> Self {
        let rnd_number: f32 = rng.gen();
        if depth == 0 || rnd_number <= 0.5 {
            *index += 1;
            BinTree::Leaf(*index)
        } else {
            *index += 1;
            let item = *index;
            let left = Self::generate_random_tree_recursive(depth - 1, index, rng);
            let right = Self::generate_random_tree_recursive(depth - 1, index, rng);

            BinTree::Branch(item, Box::new(left), Box::new(right))
        }
    }

    pub fn generate_random_tree(depth: i64) -> Self {
        let mut rng = rand::thread_rng();
        Self::generate_random_tree_recursive(depth, &mut 0, &mut rng)
    }

    fn print_tree_level_recursive(&self, level: u32) {
        match self {
            BinTree::Branch(item, left_tree, right_tree) => {
                left_tree.print_tree_level_recursive(level + 1);
                for _ in 0..level {
                    print!("  ");
                }
                println!("B{}", item);
                right_tree.print_tree_level_recursive(level + 1);
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
        self.print_tree_level_recursive(0);
    }
}
