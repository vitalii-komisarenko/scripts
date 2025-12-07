use std::rc::Rc;

#[derive(Copy, Clone, Debug, PartialEq)]
enum NodeType {
    None,
    Round,
    Square,
    Curly,
}

#[derive(Clone)]
struct Node<'a> {
    node_type: NodeType,
    text: String,
    children: Vec<Rc<Node<'a>>>,
    parent: Option<&'a Node<'a>>,
}

/// Returns: a tuple
/// 1st element is the first opening bracket
/// 2nd element is the string slice before the first opening bracket
/// 3rd element is the string slice after the first opening bracket
///
/// ```
/// let (bracket, before, after) = find_first_bracket("int main() {}", b'\0');
/// assert_eq!(bracket.unwrap(), b'(');
/// assert_eq!(before, "int main");
/// assert_eq!(after, "() {}");
/// ```
fn find_first_bracket(buf: &str, closing_bracket: u8) -> (Option<u8>, &str, &str) {
    for (i, &ch) in buf.as_bytes().into_iter().enumerate() {
        if (ch == b'{') || (ch == b'(') || (ch == b'[') || (ch == closing_bracket) {
            return (Some(ch), &buf[..i], &buf[i+1..]);
        }
    }
    return (None, &buf, "");
}

/// Returns: a tuple
/// 1st element is the last closing bracket
/// 2nd element is the string slice before the last closing bracket
/// 3rd element is the string slice after the last closing bracket
///
/// ```
/// let (bracket, before, after) = find_last_closing_bracket("int main() {}");
/// assert_eq!(bracket.unwrap(), b'}');
/// assert_eq!(before, "int main() {");
/// assert_eq!(after, "");
/// ```
fn find_last_closing_bracket(buf: &str) -> (Option<u8>, &str, &str) {
    for (i, &ch) in buf.as_bytes().into_iter().rev().enumerate() {
        if (ch == b'}') || (ch == b')') || (ch == b']') {
            let pos = buf.len() - i;
            return (Some(ch), &buf[..pos], &buf[pos+1..]);
        }
    }
    return (None, "", &buf);
}

// fn _parse(buf: &str, node_type: NodeType) -> (Result<Node, String>, &str) {
//     let closing_bracket = match node_type {
//         NodeType::None => b'\0',
//         NodeType::Round => b')',
//         NodeType::Square => b']',
//         NodeType::Curly => b'}',
//     };

//     let (first_bracket, before_first_bracket, after_first_bracket) =
//         find_first_bracket(buf, closing_bracket);

//     if first_bracket == None {
//         if node_type != NodeType::None {
//             return Err("Missing closing bracket");
//         }

//         return (Ok(Node {
//             node_type: node_type.clone(),
//             text: buf,
//             children: Vec::new(),
//         }), "");
//     }

//     let Some(first_bracket) = first_bracket;

//     if first_bracket == closing_bracket {
//         return (Ok(Node {
//             node_type: node_type.clone(),
//             text: before_first_bracket,
//             children: Vec::new(),
//         }), after_first_bracket);
//     }

//     let mut result = Node {
//         node_type: node_type.clone(),
//         text: "",
//         children: Vec::new(),
//     };

//     let mut remaining_buf = buf;
//     while true {
//         let (first_bracket, before_first_bracket, after_first_bracket) =
//             find_first_bracket(remaining_buf, closing_bracket);

//         let subnode_type = match first_bracket {
//             Some(b'{') => NodeType::Curly,
//             Some(b'(') => NodeType::Round,
//             Some(b'[') => NodeType::Square,
//             _ => panic!(),
//         };

//         result.push(Node {
//             node_type
//         });
//     }

//     while buf.len() != 0 {
//         for (i, ch) in buf.as_bytes().into_iter().enumerate() {
//             match ch {
//                 b'{' => {
//                     let text = &buf[..i+1];
//                     buf = &buf[i+1..];
//                     result.children.push(Node {
//                         node_type: NodeType::Curly,
//                         text,
//                         children: Vec::new(),
//                     });
//                     break;
//                 }
//                 b')' => {
//                     if node_type == NodeType::Round {
//                         break;
//                     }
//                     else {
//                         return (Err("Unexpected )".to_string()), "");
//                     }
//                 }
//                 b']' => {
//                     if node_type == NodeType::Square {
//                         break;
//                     }
//                     else {
//                         return (Err("Unexpected ]".to_string()), "");
//                     }
//                 }
//                 b'}' => {
//                     if node_type == NodeType::Curly {
//                         break;
//                     }
//                     else {
//                         return (Err("Unexpected }}".to_string()), "");
//                     }
//                 }
//                 _ => (),
//             }
//         }
//     }

//     (Ok(result), "")
// }

fn parse(file_content: &str) -> Result<Node, String> {
    let mut result = Node {
        node_type: NodeType::None,
        text: String::new(),
        children: Vec::new(),
        parent: None,
    };

    // let mut nodes_stack = Vec::new();
    // nodes_stack.push(&result);

    let mut current_node = &mut result;

    for ch in file_content.as_bytes().into_iter() {
        match ch {
            b'{' | b'(' | b'[' => {
                let new_node_type = match ch {
                    b'{' => NodeType::Curly,
                    b'[' => NodeType::Square,
                    b'(' => NodeType::Round,
                    _ => panic!(),
                };
                let mut new_node = Node {
                    node_type: new_node_type,
                    text: String::new(),
                    children: Vec::new(),
                    parent: Some(&mut current_node),
                };
                current_node.children.push(new_node.into());
                current_node = &mut new_node;
                // nodes_stack[nodes_stack.len()-1].children.push(new_node.clone());
                // nodes_stack.push(&nodes_stack[nodes_stack.len()-1].children[nodes_stack[nodes_stack.len()-1].children.len()-1]);
            }
            b'}' | b')' | b']' => {
                //nodes_stack.pop();
                current_node = &mut current_node.parent.unwrap();
            }
            _ => {
                //nodes_stack[nodes_stack.len()-1].text.push(*ch as char);
                current_node.text.push(*ch as char)
            }
        }
    }

    return Ok(result);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_empty() {
        let result = parse("");
        assert_eq!(result.clone().unwrap().node_type, NodeType::None);
        assert_eq!(result.clone().unwrap().text, "");
        assert_eq!(result.clone().unwrap().children.len(), 0);
    }

    #[test]
    fn test_empty_curly() {
        let result = parse("{}");
        assert_eq!(result.clone().unwrap().node_type, NodeType::Curly);
        assert_eq!(result.clone().unwrap().text, "");
        assert_eq!(result.clone().unwrap().children.len(), 0);
    }

    fn test_single_statement() {
        let result = parse("int i = 5;");
        assert_eq!(result.clone().unwrap().node_type, NodeType::Curly);
        assert_eq!(result.clone().unwrap().text, "int i = 5;");
        assert_eq!(result.clone().unwrap().children.len(), 0);
    }

    #[test]
    fn test_empty_function() {
        let result = parse("int main(){}");
        assert_eq!(result.clone().unwrap().node_type, NodeType::None);
        assert_eq!(result.clone().unwrap().text, "");
        assert_eq!(result.clone().unwrap().children.len(), 3);

        assert_eq!(result.clone().unwrap().children[0].node_type, NodeType::None);
        assert_eq!(result.clone().unwrap().children[0].text, "int main");
        assert_eq!(result.clone().unwrap().children[0].children.len(), 0);

        assert_eq!(result.clone().unwrap().children[1].node_type, NodeType::Round);
        assert_eq!(result.clone().unwrap().children[1].text, "");
        assert_eq!(result.clone().unwrap().children[1].children.len(), 0);

        assert_eq!(result.clone().unwrap().children[2].node_type, NodeType::Curly);
        assert_eq!(result.clone().unwrap().children[2].text, "");
        assert_eq!(result.clone().unwrap().children[2].children.len(), 0);
    }
}
