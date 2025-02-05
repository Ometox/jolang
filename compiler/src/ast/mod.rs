mod builder;
use std::str::FromStr;

// mod generators;
pub use builder::AstBuilder;
use either::Either;
use anyhow::Result;
use crate::{compiler_error::{CompilerError, CompilerErrorKind}, lexer::Token};

pub type Ident<'a> = Token<'a>;

#[derive(Debug, PartialEq, Clone)]
pub struct Program<'a> (Vec<Statement<'a>>);

#[derive(Debug, PartialEq, Clone)]
pub struct Block<'a> {
    pub lcurly : Token<'a>,
    pub body : Vec<Statement<'a>>,
    pub ret : Option<Box<Expr<'a>>>,
    pub rcurly : Token<'a>
}

#[derive(Debug, PartialEq, Clone)]
pub struct While<'a> {
    pub while_kw : Token<'a>,
    pub lparen : Token<'a>,
    pub cond : Box<Expr<'a>>,
    pub rparen : Token<'a>,
    pub body : Box<Expr<'a>>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Loop<'a> {
    pub loop_kw : Token<'a>,
    pub body : Box<Expr<'a>>,
}

#[derive(Debug, PartialEq, Clone)]
pub struct Break<'a> {
    pub break_kw : Token<'a>,
    pub semicolon : Token<'a>
}

#[derive(Debug, PartialEq, Clone)]
pub struct Continue<'a> {
    pub continue_kw : Token<'a>,
    pub semicolon : Token<'a>
}

#[derive(Debug, PartialEq, Clone)]
pub struct Return<'a> {
    pub return_kw : Token<'a>,
    pub value : Expr<'a>,
    pub semicolon : Token<'a>
}

#[derive(Debug, PartialEq, Clone)]
pub struct VarDecl<'a> {
    pub let_kw : Token<'a>,
    pub name : Ident<'a>,
    pub colon_token : Option<Token<'a>>,
    pub type_name : Option<Ident<'a>>,
    pub eq_token : Option<Token<'a>>,
    pub value : Option<Expr<'a>>,
    pub semicolon : Token<'a>
}

#[derive(Debug, PartialEq, Clone)]
pub struct ExprStmt<'a> {
    pub expr : Box<Expr<'a>>,
    pub semicolon : Option<Token<'a>>
}

#[derive(Debug, PartialEq, Clone)]
pub struct Noop<'a> {
    pub semicolon : Token<'a>
}

#[derive(Debug, PartialEq, Clone)]
pub enum Statement<'a> {
    Return(Return<'a>),
    Break(Break<'a>),
    Continue(Continue<'a>),
    Noop(Noop<'a>),
    VarDecl(VarDecl<'a>),
    Expr(ExprStmt<'a>)
}

#[derive(Debug, PartialEq, Clone)]
pub enum Expr<'a> {
    WhileExpr(While<'a>),
    LoopExpr(Loop<'a>),
    BlockExpr(Block<'a>),
    IfExpr(If<'a>),
    AssignExpr(Assignment<'a>),
    BinExpr(BinExpr<'a>),
    UnaryExpr(UnaryExpr<'a>),
    PrimaryExpr(PrimaryExpr<'a>)
}

#[derive(Debug, PartialEq, Clone)]
pub struct BinExpr<'a> {
    pub left : Box<Expr<'a>>,
    pub right : Box<Expr<'a>>,
    pub op : BinOp<'a>
}

#[derive(Debug, PartialEq, Clone)]
pub struct BinOp<'a> {
    pub token : Token<'a>,
    pub kind : BinOpKind
}

#[derive(Debug, PartialEq, Clone)]
pub enum BinOpKind {
    Add,
    Sub,
    Mul,
    Div,
    Equal,
    NotEqual,
    Greater,
    GreaterEqual,
    LesserEqual,
    Lesser,
    LShift,
    RShift
}

#[derive(Debug, PartialEq, Clone)]
pub struct UnaryExpr<'a> {
    pub primary : PrimaryExpr<'a>,
    pub op : UnaryOp<'a>
}

#[derive(Debug, PartialEq, Clone)]
pub struct UnaryOp<'a> {
    pub token : Token<'a>,
    pub kind : UnaryOpKind
}

#[derive(Debug, PartialEq, Clone)]
pub enum UnaryOpKind {
    Plus,
    Minus
}

#[derive(Debug, PartialEq, Clone)]
pub struct ParenExpr<'a> {
    pub lparen : Token<'a>,
    pub expr : Box<Expr<'a>>,
    pub rparen : Token<'a>
}

#[derive(Debug, PartialEq, Clone)]
pub enum PrimaryExpr<'a> {
    Call(Call<'a>),
    Ident(Ident<'a>),
    IntLit(IntLit<'a>),
    VoidLit(VoidLit<'a>),
    /// (Expr) (e. g. (5 + 5))
    Paren(ParenExpr<'a>)
}

#[derive(Debug, PartialEq, Clone)]
pub struct Call<'a> {
    pub name : Ident<'a>,
    pub lparen : Token<'a>,
    pub first_arg : Option<Box<Expr<'a>>>,
    // (colon, value)
    pub other_args : Vec<(Token<'a>, Expr<'a>)>,
    pub rparen : Token<'a>
}

#[derive(Debug, PartialEq, Clone)]
pub struct If<'a> {
    pub if_kw : Token<'a>,
    pub lparen : Token<'a>,
    pub cond : Box<Expr<'a>>,
    pub rparen : Token<'a>,
    pub then : Box<Expr<'a>>,
    pub else_kw : Option<Token<'a>>,
    pub _else : Option<Box<Expr<'a>>>
}

#[derive(Debug, PartialEq, Clone)]
pub struct Assignment<'a> {
    pub name : Ident<'a>,
    pub eq_token : Token<'a>,
    pub value : Box<Expr<'a>>
}

#[derive(Debug, PartialEq, Clone)]
pub struct IntLit<'a> {
    pub token : Token<'a>,
    pub value : i128
}

#[derive(Debug, PartialEq, Clone)]
pub struct VoidLit<'a> {
    pub lparen : Token<'a>,
    pub rparen : Token<'a>
}

impl Expr<'_> {
    pub fn require_semicolon(&self) -> bool{
        match self {
            Expr::WhileExpr(expr) => expr.body.require_semicolon(),
            Self::LoopExpr(expr) => expr.body.require_semicolon(),
            Self::IfExpr(expr) => expr._else.as_ref().map_or_else(
                || expr.then.require_semicolon(),
                |e| e.require_semicolon()
            ),
            Self::BlockExpr(_) => false,
            _ => true
        }
    }
}

impl BinOpKind {
    pub fn precedence(&self) -> u8 {
        match self {
            Self::Add => 1,
            Self::Sub => 1,
            Self::Mul => 2,
            Self::Div => 2,
            Self::Equal => 0,
            Self::NotEqual => 0,
            Self::Greater => 0,
            Self::GreaterEqual => 0,
            Self::LesserEqual => 0,
            Self::Lesser => 0,
            Self::LShift => 0,
            Self::RShift => 0,
        }
    }
}

