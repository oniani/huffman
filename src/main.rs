mod lib;

use lib::{annotate, build_tree, compress, decompress};
use lib::{FrequencyTable, Mapping, Node};

use std::env;
use std::fs;

/// Generate the Huffman codes for a given string
fn generate_encoding(message: String) -> (String, Mapping, Mapping) {
    // Get the root and weights
    let (root, _) = build_tree(&message)
        .unwrap_or((Node::default(), FrequencyTable::default()));

    // Initialize variables
    let mut char_to_code: Mapping = Mapping::new();
    let mut code_to_char: Mapping = Mapping::new();
    let node: Option<Box<Node>> = Some(Box::new(root));
    let code: String = "".to_string();

    // Annotate the tree
    annotate(&mut char_to_code, &mut code_to_char, node, code);

    // Encode a message
    let encoding: String = compress(&message, &char_to_code);

    (encoding, char_to_code, code_to_char)
}

/// Generate decoding
fn generate_decoding(message: String) -> String {
    // Read the metadata
    let metadata: String =
        fs::read_to_string("metadata.json").expect("Could not open a file!");

    // Read the metadata
    let code_to_char: Mapping =
        serde_json::from_str(&metadata).expect("Could not deserialize!");

    // Decompress the file
    let decoding: String = decompress(&message, &code_to_char);

    decoding
}

/// Run everything here
fn main() {
    // Command line arguments
    let args: Vec<_> = env::args().collect();

    // Make decisions based on the number of arguments
    match args.len() {
        1 => panic!("Provide input and output files!"),

        2 => panic!("Provide input and output files!"),

        3 => {
            // Read the input data from a file
            let input: String = args[1]
                .parse::<String>()
                .expect("Could not parse the argument!");

            // Read the input file
            let content: String =
                fs::read_to_string(input).expect("Could not open a file!");

            // Compress a file
            let (encoding, _, code_to_char): (String, Mapping, Mapping) =
                generate_encoding(content);

            // Create an output file out of the second argument
            let output: String = args[2]
                .parse::<String>()
                .expect("Could not parse the argument!");

            // Write the compressed content of the input file
            fs::write(output, &encoding).expect("Could not write to a file!");

            // Serialize the data
            let json: String = serde_json::to_string(&code_to_char)
                .expect("Could not serialize!");

            // Write the metadata
            fs::write("metadata.json", json)
                .expect("Could not write to a file!");
        }

        4 => {
            // Check the flag
            let flag: String = args[3]
                .parse::<String>()
                .expect("Could not parse the argument!");

            // Make sure that the decompression argument is specified
            if flag == "--decompress" {
                // Read the input data from a file
                let input: String = args[1]
                    .parse::<String>()
                    .expect("Could not parse the argument!");

                // Read the input file (raw bytes)
                let content: String =
                    fs::read_to_string(input).expect("Could not open a file!");

                // Decompress a file
                let decoding: String = generate_decoding(content);

                // Write the decoded file content
                let output: String = args[2]
                    .parse::<String>()
                    .expect("Could not parse the argument!");

                // Write the decoded file
                fs::write(output, decoding)
                    .expect("Could not write to a file!");
            }
        }

        _ => panic!("Redundant argument!"),
    }
}
