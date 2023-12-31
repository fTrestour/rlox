#[derive(Debug, PartialEq, Clone)]
pub enum Expression {
    Number(f64),
    String(String),
    True,
    False,
    Nil,
    Not(Box<Expression>),
    Equal(Box<Expression>, Box<Expression>),
    NotEqual(Box<Expression>, Box<Expression>),
    Less(Box<Expression>, Box<Expression>),
    LessEqual(Box<Expression>, Box<Expression>),
    Greater(Box<Expression>, Box<Expression>),
    GreaterEqual(Box<Expression>, Box<Expression>),
    Plus(Box<Expression>, Box<Expression>),
    Minus(Box<Expression>, Box<Expression>),
    Multiply(Box<Expression>, Box<Expression>),
    Divide(Box<Expression>, Box<Expression>),
    Paren(Box<Expression>),
    Variable(String),
    Assignment(String, Box<Expression>),
    And(Box<Expression>, Box<Expression>),
    Or(Box<Expression>, Box<Expression>),
    Call(Box<Expression>, Vec<Expression>),
}

#[derive(Debug, PartialEq, Clone)]
pub enum Declaration {
    Expression(Expression),
    Print(Expression),
    Var(String, Option<Expression>),
    Block(Vec<Declaration>),
    If(Expression, Box<Declaration>, Option<Box<Declaration>>),
    While(Expression, Box<Declaration>),
    Function(String, Vec<String>, Box<Declaration>),
    Return(Expression),
}
