use std::sync::Arc;

use crate::lexer::*;

pub trait Node {
    fn clone(&self) -> Box<dyn Node>;
    fn clone_box(&self) -> Box<dyn Node>;
    fn get_type(&self) -> String;
}
struct ElementNode {
    node_type: String,
    child_nodes: Option<Arc<Vec<Box<dyn Node>>>>,
}
struct ClassNode {
    node_type: String,
    child_nodes: Option<Arc<Vec<Box<dyn Node>>>>,
}
struct IdNode {
    node_type: String,
    child_nodes: Option<Arc<Vec<Box<dyn Node>>>>,
}
struct AttribNode {
    node_type: String,
    child_nodes: Option<Arc<Vec<Box<dyn Node>>>>,
}
struct TextNode {
    node_type: String,
    child_nodes: Option<Arc<Vec<Box<dyn Node>>>>,
}

impl Node for ElementNode {
    fn clone(&self) -> Box<dyn Node> {
        return Box::new(ElementNode {
            node_type: self.node_type.clone(),
            child_nodes: self.child_nodes.clone(),
        });
    }
    fn clone_box(&self) -> Box<dyn Node> {
        return Box::new(ElementNode {
            node_type: self.node_type.clone(),
            child_nodes: self.child_nodes.clone(),
        });
    }
    fn get_type(&self) -> String {
        return self.node_type.clone();
    }
}
impl Node for ClassNode {
    fn clone(&self) -> Box<dyn Node> {
        return Box::new(ClassNode {
            node_type: self.node_type.clone(),
            child_nodes: self.child_nodes.clone(),
        });
    }
    fn clone_box(&self) -> Box<dyn Node> {
        return Box::new(ClassNode {
            node_type: self.node_type.clone(),
            child_nodes: self.child_nodes.clone(),
        });
    }
    fn get_type(&self) -> String {
        return self.node_type.clone();
    }
}
impl Node for IdNode {
    fn clone(&self) -> Box<dyn Node> {
        return Box::new(IdNode {
            node_type: self.node_type.clone(),
            child_nodes: self.child_nodes.clone(),
        });
    }
    fn clone_box(&self) -> Box<dyn Node> {
        return Box::new(IdNode {
            node_type: self.node_type.clone(),
            child_nodes: self.child_nodes.clone(),
        });
    }
    fn get_type(&self) -> String {
        return self.node_type.clone();
    }
}
impl Node for AttribNode {
    fn clone(&self) -> Box<dyn Node> {
        return Box::new(AttribNode {
            node_type: self.node_type.clone(),
            child_nodes: self.child_nodes.clone(),
        });
    }
    fn clone_box(&self) -> Box<dyn Node> {
        return Box::new(AttribNode {
            node_type: self.node_type.clone(),
            child_nodes: self.child_nodes.clone(),
        });
    }
    fn get_type(&self) -> String {
        return self.node_type.clone();
    }
}
impl Node for TextNode {
    fn clone(&self) -> Box<dyn Node> {
        return Box::new(TextNode {
            node_type: self.node_type.clone(),
            child_nodes: self.child_nodes.clone(),
        });
    }
    fn clone_box(&self) -> Box<dyn Node> {
        return Box::new(TextNode {
            node_type: self.node_type.clone(),
            child_nodes: self.child_nodes.clone(),
        });
    }
    fn get_type(&self) -> String {
        return self.node_type.clone();
    }
}

#[derive(Clone)]
pub struct Parser {
    index: usize,
    nodes: Arc<Box<dyn Node>>,
    tokens: Vec<Token>,
}

impl Parser {
    fn advance(&mut self) {
        self.index += 1;
    }

    fn find_element(&mut self) -> Arc<Vec<Box<dyn Node>>> {
        let nodes = vec![];
        return Arc::new(nodes);
    }

    fn parse(&mut self) {
        if self.tokens[self.index].token_type == TT_ELEMENT {
            let elements = self.find_element();
        } else {
            return;
        }
    }
}

pub fn parse(tokens: Vec<Token>) -> Arc<Box<dyn Node>> {
    let mut parser = Parser {
        index: 0,
        nodes: Arc::new(Box::new(ElementNode {
            node_type: String::from("something"),
            child_nodes: None,
        })),
        tokens: tokens.clone(),
    };
    parser.parse();
    return parser.nodes.clone();
}
