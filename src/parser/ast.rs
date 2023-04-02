#[derive(PartialEq, Debug)]
pub enum ASTNode {
    StringLiteral(String),
    Identifier(String),
    MemberInvocation(Box<ASTNode>),
    InvocationExpression(Box<ASTNode>, Box<ASTNode>),
    Function(Box<ASTNode>, Box<ASTNode>),
    ParamList(Box<ASTNode>),
    Union(Box<ASTNode>, Box<ASTNode>),
}

impl ASTNode {
    pub fn identifier(s: &str) -> Box<Self> {
        Box::new(ASTNode::Identifier(s.to_string()))
    }

    pub fn invocation(left: Box<ASTNode>, right: Box<ASTNode>) -> Box<Self> {
        Box::new(ASTNode::InvocationExpression(left, right))
    }
}
