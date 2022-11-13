use tree_lib::tree::BinTree;

fn main() {
    let mytree = BinTree::generate_random_tree(5);
    mytree.print_tree_nonrec();
    let (largest_complete_subtree, size) = mytree.find_largest_complete_subtree_recursive();
    println!("Found subtree of size {}", size);
    largest_complete_subtree.print_tree_nonrec();
}
