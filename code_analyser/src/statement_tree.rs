// #[derive(PartialEq)]
// #[derive(Debug)]
// enum NodeType {
//     TopLevel,
//     FunctionDefinition,
// }

// #[derive(Debug)]
// struct Node<'a> {
//     node_type: NodeType,
//     children: Vec<NodeType>,
//     text: &'a str,
// }

// fn read_statement_beginning(buf: &str) -> (&str, &str) {
//     for (i, ch) in buf.as_bytes().into_iter().enumerate() {
//         match ch {
//             b'(' | b'=' | b'{' | b';' => return (&buf[..i], &buf[i..]),
//             _ => (),
//         }
//     }
//     panic("Unexpected end of statement");
// }

// fn read_statement(buf: mut &str) -> (Node, &str) {
//     let mut result: Node = Node {
//         node_type: NodeType::TopLevel,
//         children: Vec::new(),
//         text: "",
//     };

//     let beginning, buf = read_statement_beginning(_buf);
//     if buf[0] == '{' {
//         result.node_type = NodeType::FunctionDefinition;
//         result.text = beginning;
//     }
// }

// fn build_statement_tree(_file_content: &str) -> Node {
//     let mut result: Node = Node {
//         node_type: NodeType::TopLevel,
//         children: Vec::new(),
//         text: "",
//     };

//     let mut file_content = _file_content.trim();

//     while !file_content.is_empty() {
//         children.push(read_statement(file_content));
//     }
//     return result;
// }

// #[cfg(test)]
// mod test {
//     use super::*;

//     #[test]
//     fn test_empty() {
//         let tree = build_statement_tree("");
//         assert_eq!(tree.node_type, NodeType::TopLevel);
//         assert_eq!(tree.children.len(), 0);
//     }
// }
