use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::collections::HashMap;

#[derive(Debug, Eq, PartialOrd, PartialEq)]
pub struct Node {
    value: Option<char>,
    weight: u64,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Node {
    fn is_leaf(&self) -> bool {
        self.left.is_none() && self.right.is_none()
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.weight.cmp(&other.weight).reverse()
    }
}

/// Build the Huffman Tree
///
/// Construct a Huffman tree from the message using the following algorithm:
///     0. Calculate frequency of each letter in the message and store the
///        result in a frequency table
///     1. Turn each letter into an object of type Node and add to a heap
///     2. While the heap contains 2 or more items:
///         2.0 Remove two items (`left` and `right`) from the heap
///         2.1 Make a new node with the weight equal to the sum of weight of
///             the two items from 2.0
///         2.2 Assign left and right as left and right children of the new
///             node respectively
///         2.3 Add the new node to the heap
///     3. Remove the only remaining node from the heap. This is the root of
///        the Huffman tree
///     4. Return the (root, frequency table) tuple
pub fn build_tree(message: &String) -> (Node, HashMap<char, u64>) {
    // Build the frequency table
    let mut frequency_table: HashMap<char, u64> = HashMap::with_capacity(message.len());
    for letter in message.chars().into_iter() {
        *frequency_table.entry(letter).or_insert(0) += 1
    }

    // Build the heap
    let mut heap: BinaryHeap<Node> = BinaryHeap::new();
    for (letter, frequency) in frequency_table.iter() {
        heap.push(Node {
            value: Some(*letter),
            weight: *frequency,
            left: None,
            right: None,
        })
    }

    // While there are at least 2 items in the heap
    while heap.len() >= 2 {
        // Pop the left and right nodes
        let left: Node = heap.pop().unwrap();
        let right: Node = heap.pop().unwrap();

        // Create a new node
        let node: Node = Node {
            value: None,
            weight: left.weight + right.weight,
            left: Some(Box::new(left)),
            right: Some(Box::new(right)),
        };

        // Push the node onto the heap
        heap.push(node);
    }

    // The only node left in the heap is the root node
    let root: Node = heap.pop().unwrap();

    (root, frequency_table)
}

/// A HashMap that is used for storing conversion results
pub type M = HashMap<String, String>;

/// Traverse the Huffman Tree and store Huffman Codes in hashmaps.
///
/// Generate code for each letter in the message using the following algorithm:
///     0. If the root is empty, `return None`
///     1. If the current node is a leaf node and represents a valid character:
///         1.0 Add the `value: path` mapping to `hm1`
///         1.1 Add the `path: value` mapping to `hm2`
///     2. Recursively mark nodes in the left subtree and add 0 to `path`)
///     3. Recursively mark nodes in the right subtree and add 1 to `path`)
///     4. Return `Some(())`
pub fn annotate(
    char_to_code: &mut M,
    code_to_char: &mut M,
    node: Option<Box<Node>>,
    code: String,
) -> Option<()> {
    if node.is_none() {
        return None;
    }

    let node = node.unwrap();

    if node.is_leaf() {
        let val: String = node.value.unwrap().to_string();

        char_to_code
            .entry(val.to_owned())
            .or_insert(code.to_owned());
        code_to_char.entry(code.to_owned()).or_insert(val);
    }

    annotate(char_to_code, code_to_char, node.left, format!("{}0", code));
    annotate(char_to_code, code_to_char, node.right, format!("{}1", code));

    Some(())
}

/// Compress a message
pub fn compress(message: &String, char_to_code: &M) -> String {
    let mut compression: Vec<String> = Vec::with_capacity(message.len());

    for letter in message.chars().into_iter() {
        let item: &String = &char_to_code[&letter.to_string()];
        compression.push(item.to_owned());
    }

    compression.into_iter().collect::<String>()
}

/// Decompress a message
pub fn decompress(message: &String, code_to_char: &M) -> String {
    let mut decompression: Vec<String> = Vec::with_capacity(message.len());
    let mut index: usize = 0;

    while index <= message.len() - 1 {
        for (code, _) in code_to_char.iter() {
            if message[index..].starts_with(code) {
                let item: &String = &code_to_char[&code.to_string()];
                decompression.push(item.to_owned());
                index += code.len();
            }
        }
    }

    decompression.into_iter().collect::<String>()
}
