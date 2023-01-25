use tree_node::tree_node::*;

fn main() {
    let right_right = TreeNode::new(6, None, None);
    let left_right = TreeNode::new(5, None, None);
    let left_left = TreeNode::new(4, None, None);
    let right = TreeNode::new(3, None, right_right);
    let left = TreeNode::new(2, left_left, left_right);
    let root = TreeNode::new(1, left, right);
    match &root {
        Some(node) => {
            print!("\nPreorder print : ");
            node.borrow().preorder_print();
            print!("\nPostorder print: ");
            node.borrow().postorder_print();
            print!("\nInorder print  : ");
            node.borrow().inorder_print();
            print!("\n");
        }
        _ => (),
    }
}
