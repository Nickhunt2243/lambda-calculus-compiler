#[derive(Debug, Clone)]
pub enum Keyword {
    Fn,
    Let,
    LetRec,
    In,
    If,
    Then,
    Else
}
#[derive(Debug, Clone)]
pub enum AdditiveOps {
    Add,
    Sub
}
#[derive(Debug, Clone)]
pub enum MultiplicativeOps {
    Mul,
    Div
}
#[derive(Debug, Clone)]
pub enum BooleanOps {
    Equality,
    LessThan,
    LessThanEqualTo,
    GreaterThan,
    GreaterThanEqualTo
}
#[derive(Debug, Clone)]
pub enum Token {
    IntegerLiteral(i64),
    BooleanLiteral(bool),
    Identifier(String),
    Keyword(Keyword),
    AdditiveOps(AdditiveOps),
    MultiplicativeOps(MultiplicativeOps),
    BooleanOps(BooleanOps),
    EqualSign,
    Arrow,
    LParen,
    RParen
}

