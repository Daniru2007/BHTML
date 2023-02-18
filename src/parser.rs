use std::sync::Arc;

use crate::lexer::Token;

pub trait Node {
    fn clone(&self) -> Box<dyn Node>;
    fn clone_box(&self) -> Box<dyn Node>;
}

struct ElementNode {}
struct ClassNode {}
struct IdNode {}
struct AttribNode {}
struct TextNode {}

impl Node for ElementNode {
    fn clone(&self) -> Box<dyn Node> {
        return Box::new(ElementNode {});
    }
    fn clone_box(&self) -> Box<dyn Node> {
        return Box::new(ElementNode {});
    }
}
impl Node for ClassNode {
    fn clone(&self) -> Box<dyn Node> {
        return Box::new(ClassNode {});
    }
    fn clone_box(&self) -> Box<dyn Node> {
        return Box::new(ClassNode {});
    }
}
impl Node for IdNode {
    fn clone(&self) -> Box<dyn Node> {
        return Box::new(IdNode {});
    }
    fn clone_box(&self) -> Box<dyn Node> {
        return Box::new(IdNode {});
    }
}
impl Node for AttribNode {
    fn clone(&self) -> Box<dyn Node> {
        return Box::new(AttribNode {});
    }
    fn clone_box(&self) -> Box<dyn Node> {
        return Box::new(AttribNode {});
    }
}
impl Node for TextNode {
    fn clone(&self) -> Box<dyn Node> {
        return Box::new(TextNode {});
    }
    fn clone_box(&self) -> Box<dyn Node> {
        return Box::new(TextNode {});
    }
}

#[derive(Clone)]
pub struct Parser {
    index: usize,
    nodes: Arc<Vec<Box<dyn Node>>>,
}

impl Parser {
    fn advance(&mut self) {
        self.index += 1;
    }

    fn add_tok(&mut self, node: Box<dyn Node>) {
        if let Some(nodes) = Arc::get_mut(&mut self.nodes) {
            nodes.push(node);
        }
    }
    fn parse(&mut self, tokens: Vec<Token>) {}
}

pub fn parse(tokens: Vec<Token>) -> Arc<Vec<Box<dyn Node>>> {
    let mut parser = Parser {
        index: 0,
        nodes: Arc::from(vec![]),
    };
    parser.parse(tokens);
    return parser.nodes.clone();
}
