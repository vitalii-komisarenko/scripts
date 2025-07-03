#[derive(PartialEq)]
#[derive(Debug)]
enum NodeType {
    TopLevel,
}

#[derive(Debug)]
struct Node {
    node_type: NodeType,
    children: Vec<NodeType>,
}

fn build_statement_tree(file_content: &str) -> Node {
    let result: Node = Node {
        node_type: NodeType::TopLevel,
        children: Vec::new(),
    };
    return result;
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_empty() {
        let tree = build_statement_tree("");
        assert_eq!(tree.node_type, NodeType::TopLevel);
        assert_eq!(tree.children.len(), 0);
    }
}
