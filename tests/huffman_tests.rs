use huffman::annotate;
use huffman::build_tree;
use huffman::decompress;
use huffman::compress;
use huffman::{Node, M};

#[test]
fn test_huffman_alnum() {
    // Test alphanumeric messages
    let messages: Vec<String> = vec![
        "PT3IPpBdlYhrAxlO3eYN".to_string(),
        "ezVRhK7Daqprm8RuYlQr".to_string(),
        "7DY5boVpKeeHnluOOQ6v".to_string(),
        "p1btee56FfZ9U72q6CJg".to_string(),
        "Z3HJsEc1grVQh9Av022A".to_string(),
        "786YE7BgmDFpk8ZSc3oL".to_string(),
        "fEsfOYKVKLzIXizmT0IR".to_string(),
        "yNSLIJlP1tosnyhtsUl4".to_string(),
        "iiLIgprbgb0Xy2oQ3lHm".to_string(),
        "hmo2XJAQUREOy67UJ1FE".to_string(),
        "stqCqxGt2Ort8lZRDZq4".to_string(),
        "XyQsJflp65nJhst1FxGq".to_string(),
        "xsgMi5iFvTnjsxDGYRsR".to_string(),
        "ZdfKmt1YXIwdlb3Ka3NI".to_string(),
        "WHC7rNNqdPIUsGhUHkmP".to_string(),
        "qKzPLLr2uc8C82rAXw0B".to_string(),
        "07gtEoM08KSqTKqtPIKn".to_string(),
        "igjT0Vuc6wG5ShUXkvPN".to_string(),
        "jXJEhitEJhiMULu1v3qz".to_string(),
        "XtPfOeFXIfp8Bib4FhUV".to_string(),
        "EWtrg8aGWLaMKynwxHrd".to_string(),
        "I6sBON3XyLe8RjMHR4R5".to_string(),
        "xSDJ3AczWaEsXkj7w3mQ".to_string(),
        "L2DTayFeHfcEMk5f6JfD".to_string(),
        "IKnqUswKwjUvkS31NVVQ".to_string(),
    ];

    // Iterate over all messages
    for message in messages {
        // Get the root and weights
        let (root, _) = build_tree(&message);

        // Initialize variables
        let mut char_to_code: M = M::new();
        let mut code_to_char: M = M::new();
        let node: Option<Box<Node>> = Some(Box::new(root));
        let code: String = "".to_string();

        // Annotate the tree
        annotate(&mut char_to_code, &mut code_to_char, node, code);

        // Compress a message
        let encoding: String = compress(&message, &char_to_code);

        // Decompress a message
        let decoding: String = decompress(&encoding, &code_to_char);

        // Make use of the identity
        // Notice that `decompress(compress(message))` is `message`
        assert_eq!(message, decoding);
    }
}

#[test]
fn test_huffman_utf8() {
    // Test UTF-8 message
    let messages: Vec<String> = vec![
        "Iζ񊝐慸,򚪄ޛӹ򒈚𸻓ﴡ䕖惂�񂟲񄤿ϾEϲO$,ôzᐖ夥ݵʗ枒󒂤ქ㬣".to_string(),
        "󣿬Ɇかsᷨ򆼨쮢vR񎸖􊳮ƅ駴Ǌ(Xڠퟺ󲼅%1刻XɾHǱ,򍬴󏓟䧺ڟ".to_string(),
        "Ⱥ󧌮Īā䒤򯏀✢漌狗=q󚄤忔ϗ廣񻶺Mͧ𓄐𶺱H娆𭳅�n�͉򺩣ěܚ".to_string(),
        "$2ڈʐ2́򪜛蔀ӑ1M߾ԏ򩐃𸝡OھՖ7򭅦έ󺕉ذ捆Ϣ̓Z%𔇢ٸ".to_string(),
        "ꟋៅZۋяؒ1ҫ򅝖񝖬x𼠊Җ묃㪼ة򇺮眻򚠩򉛖򭳭𷹄𥢛ѐm鹶ɒ󒙢厗򵉄е".to_string(),
        "ʭN𶾼ۡ晠󍏀ً򢖘󻥲{䘠=瘙縙͌ԉ늈▸@Ұ􄋆餒ퟗ͠.􆞻񞙕򎣬݆�Ϡ锹".to_string(),
        "ˤj𛼹󃙾ՠ񦑮Χ󝋟gն넟񭥰IԦ𷁽򢋕͌𠥎𢵄􃯒򊐦޴Øּ⩨,Fﶬ񛻎򿉥ؗ".to_string(),
        "XG񪌘ޒз󠩴񨵪򤾥󞒟󗐞?陋ۜ򜭾qشlੋ#𶛂*ɝY䗎񡤙去꼾󷟸􁜒ӗ>񾝺".to_string(),
        "񒤼슶1زƒ㮽񡜵ḓ첉ˢ9WLՔ񡒸.Ԅ򲫥ֵ鎔݃櫷$;̜񌪍𘛰󈔽�/".to_string(),
        "򔏙ϼ񽍚믄񢹫糘܈Q�񛝌ዷ܄ٳڬ.톢'-񼕙񻽳ީB4򆠝#񟪜񘛴􊺫𹈸ۻ͠".to_string(),
    ];

    // Iterate over all messages
    for message in messages {
        // Get the root and weights
        let (root, _) = build_tree(&message);

        // Initialize variables
        let mut char_to_code: M = M::new();
        let mut code_to_char: M = M::new();
        let node: Option<Box<Node>> = Some(Box::new(root));
        let code: String = "".to_string();

        // Annotate the tree
        annotate(&mut char_to_code, &mut code_to_char, node, code);

        // Compress a message
        let encoding: String = compress(&message, &char_to_code);

        // Decompress a message
        let decoding: String = decompress(&encoding, &code_to_char);

        // Make use of the identity
        // Notice that `decompress(compress(message))` is `message`
        assert_eq!(message, decoding);
    }
}
