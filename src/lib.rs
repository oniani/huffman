use std::cmp::Ordering;
use std::collections::{BinaryHeap, HashMap};

#[derive(Debug, Eq, PartialEq, PartialOrd)]
pub struct Node {
    value: Option<char>,
    weight: u64,
    left: Option<Box<Node>>,
    right: Option<Box<Node>>,
}

impl Default for Node {
    fn default() -> Self {
        Node {
            value: None,
            weight: 0,
            left: None,
            right: None,
        }
    }
}

impl Ord for Node {
    fn cmp(&self, other: &Self) -> Ordering {
        self.weight.cmp(&other.weight).reverse()
    }
}

impl Node {
    fn is_leaf(&self) -> bool {
        self.left.is_none() && self.right.is_none()
    }
}

pub type FrequencyTable = HashMap<char, u64>;

/// Build the Huffman tree
///
/// Construct a Huffman tree from the message using the following algorithm:
///     1. Calculate frequency of each character in the message and store the
///        result in a frequency table
///     2. Turn each character into an object of type Node and add to a heap
///     3. While the heap contains 2 or more items:
///         3.1 Remove two items (`left` and `right`) from the heap
///         3.2 Make a new node with the weight equal to the sum of weight of
///             the two items from 3.0
///         3.2 Assign left and right as left and right children of the new
///             node respectively
///         3.3 Add the new node to the heap
///     4. Remove the only remaining node from the heap. This is the root of
///        the Huffman tree
///     5. Return the (root, frequency table) tuple
pub fn build_tree(message: &String) -> Option<(Node, FrequencyTable)> {
    // Build the frequency table
    let mut frequency_table: FrequencyTable =
        HashMap::with_capacity(message.len());
    for char in message.chars().into_iter() {
        *frequency_table.entry(char).or_insert(0) += 1
    }

    // Build the heap
    let mut heap: BinaryHeap<Node> =
        BinaryHeap::with_capacity(frequency_table.len());
    for (char, frequency) in frequency_table.iter() {
        heap.push(Node {
            value: Some(*char),
            weight: *frequency,
            left: None,
            right: None,
        })
    }

    // While there are at least 2 items in the heap
    while heap.len() >= 2 {
        // Pop the left and right nodes
        let left: Node = heap.pop()?;
        let right: Node = heap.pop()?;

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
    let root: Node = heap.pop()?;

    Some((root, frequency_table))
}

/// A HashMap that is used for storing conversion results
pub type Mapping = HashMap<String, String>;

/// Traverse the Huffman tree and create mappings out of Huffman Codes.
///
/// Generate code for each character in the message using the following
/// algorithm:
///     1. If the root is empty, `return None`
///     2. If the current node is a leaf node and represents a valid character:
///         2.0 Add the `value: path` mapping to `char_to_code`
///         2.1 Add the `path: value` mapping to `code_to_char`
///     3. Recursively mark nodes in the left subtree and add 0 to `code`)
///     4. Recursively mark nodes in the right subtree and add 1 to `code`)
///     5. Return `Some(())`
pub fn annotate(
    char_to_code: &mut Mapping,
    code_to_char: &mut Mapping,
    node: Option<Box<Node>>,
    code: String,
) -> Option<()> {
    let node: Box<Node> = node?;

    if node.is_leaf() {
        let val: String = node.value?.to_string();

        char_to_code.entry(val.clone()).or_insert(code.clone());
        code_to_char.entry(code.clone()).or_insert(val);
    }

    annotate(char_to_code, code_to_char, node.left, format!("{}0", code));
    annotate(char_to_code, code_to_char, node.right, format!("{}1", code));

    Some(())
}

/// Compress a message
pub fn compress(message: &String, char_to_code: &Mapping) -> String {
    let mut encoding: Vec<String> = Vec::with_capacity(message.len());

    for char in message.chars().into_iter() {
        encoding.push(char_to_code[&char.to_string()].clone());
    }

    encoding.into_iter().collect::<String>()
}

/// Decompress a message
pub fn decompress(message: &String, code_to_char: &Mapping) -> String {
    let mut decoding: Vec<String> = Vec::with_capacity(message.len());

    let mut index: usize = 0;
    while index <= message.len() - 1 {
        for (code, _) in code_to_char.iter() {
            if message[index..].starts_with(code) {
                decoding.push(code_to_char[&code.to_owned()].clone());
                index += code.len();
            }
        }
    }

    decoding.into_iter().collect::<String>()
}
