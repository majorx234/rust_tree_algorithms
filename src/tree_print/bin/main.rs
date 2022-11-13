use tree_lib::tree::BinTree;

fn main() {
    let mytree = BinTree::generate_random_tree(3);
    mytree.print_tree_nonrec();
}
