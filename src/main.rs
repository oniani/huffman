mod lib;

use lib::annotate;
use lib::build_tree;
use lib::decompress;
use lib::compress;
use lib::{Node, M};

/// Run everything here
fn main() {
    // Example message
    let message: String = "Explore the universe!".to_string();
    println!("Message: {}", message);

    // Get the root and weights
    let (root, _) = build_tree(&message);

    // Initialize variables
    let mut char_to_code: M = M::new();
    let mut code_to_char: M = M::new();
    let node: Option<Box<Node>> = Some(Box::new(root));
    let code: String = "".to_string();

    // Annotate the tree
    annotate(&mut char_to_code, &mut code_to_char, node, code);

    // Encode a message
    let encoding: String = compress(&message, &char_to_code);
    println!("Compression: {}", encoding);

    // Decode a message
    let decoding: String = decompress(&encoding, &code_to_char);
    println!("Decompression: {}", decoding);
}
