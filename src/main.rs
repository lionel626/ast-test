
enum NodeType {
    LITERAL,
    PLUS,
    MINUS,
    MUL,
    DIV
}

trait ASTNode<T> {
    fn get_type(&self) -> NodeType;
    fn get_value(&self) -> &T;
    fn get_parent(&self) -> &Option<Box<dyn ASTNode<T>>>;
    fn get_children(&self) -> &Vec<Box<dyn ASTNode<T>>>;
}
struct ASTNodeLiteral<T> {
    value: T,
    parent: Option<Box<dyn ASTNode<T>>>,
    children: Vec<Box<dyn ASTNode<T>>>
}

impl<T> ASTNodeLiteral<T> {
    fn new(value: T, parent: Option<Box<dyn ASTNode<T>>>, children: Vec<Box<dyn ASTNode<T>>>) -> ASTNodeLiteral<T> {
        ASTNodeLiteral { value, parent, children }
    }
}

impl<T> ASTNode<T> for ASTNodeLiteral<T> {
    fn get_type(&self) -> NodeType {
        NodeType::LITERAL
    }

    fn get_value(&self) -> &T {
        &self.value
    }

    fn get_parent(&self) -> &Option<Box<dyn ASTNode<T>>> {
        &self.parent
    }

    fn get_children(&self) -> &Vec<Box<dyn ASTNode<T>>> {
        &self.children
    }
}

struct ASTNodePlus<T> {
    value: T,
    parent: Option<Box<dyn ASTNode<T>>>,
    children: Vec<Box<dyn ASTNode<T>>>
}

impl<T> ASTNodePlus<T> {
    fn new(value: T, parent: Option<Box<dyn ASTNode<T>>>, children: Vec<Box<dyn ASTNode<T>>>) -> ASTNodePlus<T> {
        ASTNodePlus { value, parent, children }
    }
}

impl<T> ASTNode<T> for ASTNodePlus<T> {
    fn get_type(&self) -> NodeType {
        NodeType::PLUS
    }

    fn get_value(&self) -> &T {
        &self.value
    }

    fn get_parent(&self) -> &Option<Box<dyn ASTNode<T>>> {
        &self.parent
    }

    fn get_children(&self) -> &Vec<Box<dyn ASTNode<T>>> {
        &self.children
    }
}

struct ASTNodeMinus<T> {
    value: T,
    parent: Option<Box<dyn ASTNode<T>>>,
    children: Vec<Box<dyn ASTNode<T>>>
}

impl<T> ASTNodeMinus<T> {
    fn new(value: T, parent: Option<Box<dyn ASTNode<T>>>, children: Vec<Box<dyn ASTNode<T>>>) -> ASTNodeMinus<T> {
        ASTNodeMinus { value, parent, children }
    }
}

impl<T> ASTNode<T> for ASTNodeMinus<T> {
    fn get_type(&self) -> NodeType {
        NodeType::MINUS
    }

    fn get_value(&self) -> &T {
        &self.value
    }

    fn get_parent(&self) -> &Option<Box<dyn ASTNode<T>>> {
        &self.parent
    }

    fn get_children(&self) -> &Vec<Box<dyn ASTNode<T>>> {
        &self.children
    }
}

struct ASTNodeMul<T> {
    value: T,
    parent: Option<Box<dyn ASTNode<T>>>,
    children: Vec<Box<dyn ASTNode<T>>>
}

impl<T> ASTNodeMul<T> {
    fn new(value: T, parent: Option<Box<dyn ASTNode<T>>>, children: Vec<Box<dyn ASTNode<T>>>) -> ASTNodeMul<T> {
        ASTNodeMul { value, parent, children }
    }
}

impl<T> ASTNode<T> for ASTNodeMul<T> {
    fn get_type(&self) -> NodeType {
        NodeType::MUL
    }

    fn get_value(&self) -> &T {
        &self.value
    }

    fn get_parent(&self) -> &Option<Box<dyn ASTNode<T>>> {
        &self.parent
    }

    fn get_children(&self) -> &Vec<Box<dyn ASTNode<T>>> {
        &self.children
    }
}

struct ASTNodeDiv<T> {
    value: T,
    parent: Option<Box<dyn ASTNode<T>>>,
    children: Vec<Box<dyn ASTNode<T>>>
}

impl<T> ASTNodeDiv<T> {
    fn new(value: T, parent: Option<Box<dyn ASTNode<T>>>, children: Vec<Box<dyn ASTNode<T>>>) -> ASTNodeDiv<T> {
        ASTNodeDiv { value, parent, children }
    }
}

impl<T> ASTNode<T> for ASTNodeDiv<T> {
    fn get_type(&self) -> NodeType {
        NodeType::DIV
    }

    fn get_value(&self) -> &T {
        &self.value
    }

    fn get_parent(&self) -> &Option<Box<dyn ASTNode<T>>> {
        &self.parent
    }

    fn get_children(&self) -> &Vec<Box<dyn ASTNode<T>>> {
        &self.children
    }
}


// AST Visitor

trait ASTVisitor<T> {
    fn visit(&self, node: &dyn ASTNode<T>);
}

// Create a visitor that print the AST 2 + 3 * 4 as + (2 (* 3 4))

struct ASTPrintVisitor;

impl ASTVisitor<i32> for ASTPrintVisitor {
    fn visit(&self, node: &dyn ASTNode<i32>) {
        match node.get_type() {
            NodeType::LITERAL => {
                print!("{} ", node.get_value());
            },
            NodeType::PLUS => {
                print!("+ (");
                for child in node.get_children() {
                    self.visit(child.as_ref());
                }
                print!(")");
            },
            NodeType::MINUS => {
                print!("- (");
                for child in node.get_children() {
                    self.visit(child.as_ref());
                }
                print!(")");
            },
            NodeType::MUL => {
                print!("* (");
                for child in node.get_children() {
                    self.visit(child.as_ref());
                }
                print!(")");
            },
            NodeType::DIV => {
                print!("/ (");
                for child in node.get_children() {
                    self.visit(child.as_ref());
                }
                print!(")");
            }
        }
    }
}






fn main() {
    // Make 2 + 3 * 4
    let root = Box::new(ASTNodePlus::new(
        0,
        None,
        vec![
            Box::new(ASTNodeLiteral::new(
                2,
                None,
                vec![]
            )),
            Box::new(ASTNodeMul::new(
                0,
                None,
                vec![
                    Box::new(ASTNodeLiteral::new(
                        3,
                        None,
                        vec![]
                    )),
                    Box::new(ASTNodeLiteral::new(
                        4,
                        None,
                        vec![]
                    ))
                ]
            ))
        ]
    ));

    let visitor = ASTPrintVisitor;
    visitor.visit(root.as_ref());


    println!("Hello, world!");
}
