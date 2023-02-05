pub enum ASTNode {
    StringLiteral(String),
    Identifier(String),
    MemberInvocation(Box<ASTNode>),
    InvocationExpression(Box<ASTNode>, Box<ASTNode>),
    Function(Box<ASTNode>, Box<ASTNode>),
    ParamList(Box<ASTNode>),
}
