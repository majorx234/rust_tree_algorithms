use rand::Rng;

#[derive(Debug, Clone)]
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

    pub fn print_tree_nonrec(&self) {
        let mut stack = Vec::<(&Self, usize)>::new();
        stack.push((&self, 0));
        let mut print_stack = Vec::<(i64, usize)>::new();
        while let Some((bin_tree, level)) = stack.pop() {
            match bin_tree {
                BinTree::Branch(item, left_tree, right_tree) => {
                    print_stack.push((*item, level));
                    stack.push((right_tree, level + 1));
                    stack.push((left_tree, level + 1));
                }
                BinTree::Leaf(item) => {
                    for _ in 0..level {
                        print!("   ");
                    }
                    println!("L{}", item);
                    match print_stack.pop() {
                        Some((item, level)) => {
                            for _ in 0..level {
                                print!("   ");
                            }
                            println!("B{}", item);
                        }
                        None => (),
                    }
                }
            }
        }
    }

    pub fn print_tree(&self) {
        self.print_tree_level_recursive(0);
    }

    pub fn deep_search_tree_nonrec(&self, item_to_search: i64) -> bool {
        let mut stack = Vec::<(&Self, usize)>::new();
        stack.push((&self, 0));
        while let Some((bin_tree, level)) = stack.pop() {
            match bin_tree {
                BinTree::Branch(item, left_tree, right_tree) => {
                    if *item == item_to_search {
                        return true;
                    }
                    stack.push((right_tree, level + 1));
                    stack.push((left_tree, level + 1));
                }
                BinTree::Leaf(item) => {
                    if *item == item_to_search {
                        return true;
                    }
                    println!("L{}", item);
                }
            }
        }
        false
    }

    pub fn prune_tree(&self, size: u32) -> Self {
        match self {
            BinTree::Leaf(x) => BinTree::Leaf(*x),
            BinTree::Branch(x, lt, rt) => {
                if size <= 1 {
                    BinTree::Leaf(*x)
                } else {
                    let new_lt = lt.prune_tree(size / 2);
                    let new_rt = rt.prune_tree(size / 2);
                    BinTree::Branch(*x, Box::new(new_lt), Box::new(new_rt))
                }
            }
        }
    }

    fn find_largest_complete_subtree_recursive_aux(
        &self,
    ) -> ((Self, u32), (Self, u32), (Self, u32)) {
        match self {
            BinTree::Leaf(x) => (
                (BinTree::Leaf(*x), 1),
                (BinTree::Leaf(*x), 0),
                (BinTree::Leaf(*x), 0),
            ),
            BinTree::Branch(x, lt, rt) => {
                let (
                    (biggest_tree_left, last_biggest_size_left),
                    (biggest_tree_left_left, size_left_left),
                    (biggest_tree_left_right, size_left_right),
                ) = lt.find_largest_complete_subtree_recursive_aux();
                let (
                    (biggest_tree_right, last_biggest_size_right),
                    (biggest_tree_right_left, size_right_left),
                    (biggest_tree_right_right, size_right_right),
                ) = rt.find_largest_complete_subtree_recursive_aux();
                let min_branch = last_biggest_size_left.min(last_biggest_size_right);
                let current_tree = BinTree::Branch(
                    *x,
                    Box::new(biggest_tree_left.clone()),
                    Box::new(biggest_tree_right.clone()),
                );
                let size_min_branch = 2 * min_branch + 1;
                let (new_biggest_tree_left_side, size_left_side) =
                    if size_left_left > size_left_right {
                        (biggest_tree_left_left, size_left_left)
                    } else {
                        (biggest_tree_left_right, size_left_right)
                    };
                let (new_biggest_tree_right_side, size_right_side) =
                    if size_right_left > size_right_right {
                        (biggest_tree_right_left, size_right_left)
                    } else {
                        (biggest_tree_right_right, size_right_right)
                    };
                let (new_biggest_tree_left, size_left) = if last_biggest_size_left > size_left_side
                {
                    (biggest_tree_left, last_biggest_size_left)
                } else {
                    (new_biggest_tree_left_side, size_left_side)
                };
                let (new_biggest_tree_right, size_right) =
                    if last_biggest_size_right > size_right_side {
                        (biggest_tree_right, last_biggest_size_right)
                    } else {
                        (new_biggest_tree_right_side, size_right_side)
                    };
                (
                    (current_tree, size_min_branch),
                    (new_biggest_tree_left, size_left),
                    (new_biggest_tree_right, size_right),
                )
            }
        }
    }

    pub fn find_largest_complete_subtree_recursive(&self) -> (Self, u32) {
        match self {
            BinTree::Leaf(x) => (BinTree::Leaf(*x), 1),
            BinTree::Branch(_, _, _) => {
                let (
                    (current_tree, size_min_branch),
                    (new_biggest_tree_left, size_left),
                    (new_biggest_tree_right, size_right),
                ) = self.find_largest_complete_subtree_recursive_aux();
                let (unpruned_tree, size) = if size_min_branch < size_left {
                    if size_left < size_right {
                        (new_biggest_tree_right, size_right)
                    } else {
                        (new_biggest_tree_left, size_left)
                    }
                } else {
                    if size_min_branch < size_right {
                        (new_biggest_tree_right, size_right)
                    } else {
                        (current_tree, size_min_branch)
                    }
                };
                (unpruned_tree.prune_tree(size), size)
            }
        }
    }
}
