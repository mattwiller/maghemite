#[derive(PartialEq, Eq, Debug)]
pub enum ASTNode {
    BooleanLiteral(bool),
    StringLiteral(String),
    NumberLiteral(String),
    Identifier(String),
    MemberInvocation(Box<ASTNode>),
    InvocationExpression(Box<ASTNode>, Box<ASTNode>),
    Function(Box<ASTNode>, Box<ASTNode>),
    ParamList(Option<Box<ASTNode>>),
    Union(Box<ASTNode>, Box<ASTNode>),
}

impl ASTNode {
    pub fn identifier(s: &str) -> Box<Self> {
        Box::new(ASTNode::Identifier(s.to_string()))
    }

    pub fn invocation(left: Box<ASTNode>, right: Box<ASTNode>) -> Box<Self> {
        Box::new(ASTNode::InvocationExpression(left, right))
    }

    pub fn function(left: Box<ASTNode>, right: Box<ASTNode>) -> Box<Self> {
        Box::new(ASTNode::Function(left, right))
    }

    pub fn union(left: Box<ASTNode>, right: Box<ASTNode>) -> Box<Self> {
        Box::new(ASTNode::Union(left, right))
    }

    pub fn string(s: &str) -> Box<Self> {
        Box::new(ASTNode::StringLiteral(s.to_string()))
    }

    pub fn params(p: Box<ASTNode>) -> Box<Self> {
        Box::new(ASTNode::ParamList(Some(p)))
    }

    pub fn empty_params() -> Box<Self> {
        Box::new(ASTNode::ParamList(None))
    }
}
