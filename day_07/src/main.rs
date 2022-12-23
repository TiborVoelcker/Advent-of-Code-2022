mod tree;

use tree::init_tree;

fn main() {
    let contents = include_str!("../input.txt");

    let binding = init_tree(contents.lines());

    let total = binding.borrow_mut().calc_size();
    println!("Total size: {total}");
    let unused = 70000000 - total;
    let mut smallest = 70000000;
    for node in binding.borrow().to_owned().into_iter() {
        let mut borrowed_node = node.borrow_mut();
        borrowed_node.calc_size();
        if unused + borrowed_node.size.unwrap() > 30000000
            && borrowed_node.size.unwrap() < smallest
            && borrowed_node.children.first() != None
        {
            smallest = borrowed_node.size.unwrap();
        }
        println!("{borrowed_node:?}");
    }

    println!("Smallest to delete: {smallest}")
}
