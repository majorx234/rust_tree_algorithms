use tree_lib::tree::BinTree;

fn main() {
    let mytree = BinTree::generate_tree(5);
    mytree.print_tree();
}
