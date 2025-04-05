use logos::{skip, Logos};
use snailquote::{unescape, UnescapeError};

use std::num::{ParseFloatError, ParseIntError};

#[derive(Debug, PartialEq, Clone)]
pub struct LexError {
    pub msg: String,
}

impl From<ParseIntError> for LexError {
    fn from(value: ParseIntError) -> Self {
        Self {
            msg: value.to_string(),
        }
    }
}

impl From<ParseFloatError> for LexError {
    fn from(value: ParseFloatError) -> Self {
        Self {
            msg: value.to_string(),
        }
    }
}

impl From<UnescapeError> for LexError {
    fn from(value: UnescapeError) -> Self {
        Self {
            msg: value.to_string(),
        }
    }
}

impl Default for LexError {
    fn default() -> Self {
        Self {
            msg: "".to_string(),
        }
    }
}

#[derive(Logos, Clone, Debug, PartialEq)]
#[logos(error = LexError)]
pub enum Token {
    #[regex(r"[A-Za-z_][A-Za-z1-9_]*", |lex| lex.slice().to_string(), priority = 1)]
    Ident(String),

    // TODO: Char!
    /// Regex inspired by <https://stackoverflow.com/questions/32155133/regex-to-match-a-json-string>
    #[regex(r#""([^"\\]|\\["\\bnfrt]|u[a-fA-F0-9]{4})*""#, |lex| unescape(lex.slice()))]
    Str(String),

    #[regex(r"-?(?:0|[1-9]\d*)(?:[eE]?\d+)?", |lex| lex.slice().parse::<i64>())]
    Int(i64),

    #[regex(r"-?(?:0|[1-9]\d*)\.\d+(?:[eE][+-]?\d+)?", |lex| lex.slice().parse::<f64>())]
    Double(f64),

    #[regex(r"[ \n\t\f]+", skip)]
    #[regex(r"//[^\n]*\n?", skip)]
    #[regex(r"/\*(?:[^*]|\*[^/])*\*/", skip)] // Can't be nested
    #[token("(")]
    LeftParen,

    #[token(")")]
    RightParen,

    #[token("{")]
    LeftBrace,

    #[token("}")]
    RightBrace,

    #[token("[")]
    LeftBrak,

    #[token("]")]
    RightBrak,

    #[token("^")]
    Caret,

    #[token(".")]
    Dot,

    #[token("..")]
    DDot,

    #[token("+")]
    Plus,

    #[token("+=")]
    PlusEq,

    #[token("-")]
    Minus,

    #[token("-=")]
    MinusEq,

    #[token("*")]
    Mul,

    #[token("*=")]
    MulEq,

    #[token("/")]
    Div,

    #[token("/=")]
    DivEq,

    #[token("%")]
    Mod,

    #[token("%=")]
    ModEq,

    #[token(",")]
    Comma,

    #[token("->")]
    RightArrow,

    #[token(">")]
    GT,

    #[token(">=")]
    GE,

    #[token("<")]
    LT,

    #[token("<=")]
    LE,

    #[token("!=")]
    NotEq,

    #[token("=")]
    Eq,

    #[token("==")]
    DEq,

    #[token(";")]
    SemiColon,

    #[token(":")]
    Colon,

    #[token("::")]
    DColon,

    // Keywords
    #[token("import")]
    Import,

    #[token("let")]
    Let,

    #[token("const")]
    Const,

    #[token("func")]
    Func,

    #[token("if")]
    If,

    #[token("and")]
    And,

    #[token("or")]
    Or,

    #[token("not")]
    Not,

    #[token("else")]
    Else,

    #[token("true")]
    True,

    #[token("false")]
    False,

    #[token("for")]
    For,

    #[token("while")]
    While,

    #[token("in")]
    In,

    #[token("return")]
    Return,

    #[token("defer")]
    Defer,

    #[token("make")]
    Make,

    #[token("destroy")]
    Destroy,

    #[token("new")]
    New,

    #[token("free")]
    Free,

    #[token("enum")]
    Enum,

    #[token("struct")]
    Struct,

    #[token("union")]
    Union,

    #[token("alias")]
    Alias,

    #[token("break")]
    Break,

    #[token("continue")]
    Continue,

    #[token("void")]
    TVoid,

    #[token("double")]
    TDouble,

    #[token("float")]
    TFloat,

    #[token("char")]
    TChar,

    #[token("str")]
    TStr,

    #[token("i8")]
    TInt8,

    #[token("i16")]
    TInt16,

    #[token("i32")]
    TInt32,

    #[token("i64")]
    TInt64,

    #[token("u8")]
    TUInt8,

    #[token("u16")]
    TUInt16,

    #[token("u32")]
    TUInt32,

    #[token("u64")]
    TUInt64,

    #[token("bool")]
    TBool,
}

impl std::fmt::Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Token::*;

        match self {
            Ident(ident) => write!(f, "identifier '{}'", ident),
            Str(string) => write!(f, "string literal '{}'", string),
            Int(integer) => write!(f, "integer literal '{}'", integer),
            Double(double) => write!(f, "double literal '{}'", double),
            LeftParen => write!(f, "'('"),
            RightParen => write!(f, "')'"),
            LeftBrace => write!(f, "{{"),
            RightBrace => write!(f, "}}"),
            LeftBrak => write!(f, "'['"),
            RightBrak => write!(f, "']'"),
            Caret => write!(f, "'^'"),
            Dot => write!(f, "'.'"),
            DDot => write!(f, "'..'"),
            Plus => write!(f, "'+'"),
            PlusEq => write!(f, "'+='"),
            Minus => write!(f, "'-'"),
            MinusEq => write!(f, "'-='"),
            Mul => write!(f, "'*'"),
            MulEq => write!(f, "'*='"),
            Div => write!(f, "'/'"),
            DivEq => write!(f, "'/='"),
            Mod => write!(f, "'%'"),
            ModEq => write!(f, "'%='"),
            Comma => write!(f, "'"),
            RightArrow => write!(f, "'->'"),
            GT => write!(f, "'>'"),
            GE => write!(f, "'>='"),
            LT => write!(f, "'<'"),
            LE => write!(f, "'<='"),
            NotEq => write!(f, "'!='"),
            Eq => write!(f, "'='"),
            DEq => write!(f, "'=='"),
            SemiColon => write!(f, "';'"),
            Colon => write!(f, "':'"),
            Not => write!(f, "'not'"),
            DColon => write!(f, "'::'"),
            Import => write!(f, "'import'"),
            Let => write!(f, "'let'"),
            Const => write!(f, "'const'"),
            Func => write!(f, "'func'"),
            Break => write!(f, "'break'"),
            Continue => write!(f, "'continue'"),
            If => write!(f, "'if'"),
            And => write!(f, "'and'"),
            Or => write!(f, "'or'"),
            Else => write!(f, "'else'"),
            True => write!(f, "'true'"),
            False => write!(f, "'false'"),
            For => write!(f, "'for'"),
            While => write!(f, "'while'"),
            In => write!(f, "'in'"),
            Return => write!(f, "'return'"),
            Defer => write!(f, "'defer'"),
            Make => write!(f, "'make'"),
            Destroy => write!(f, "'destroy'"),
            New => write!(f, "'new'"),
            Free => write!(f, "'free'"),
            Enum => write!(f, "'enum'"),
            Struct => write!(f, "'struct'"),
            Union => write!(f, "'union'"),
            Alias => write!(f, "'alias'"),

            // Types
            TVoid => write!(f, "'void'"),
            TDouble => write!(f, "'double'"),
            TFloat => write!(f, "'float'"),
            TChar => write!(f, "'char'"),
            TStr => write!(f, "'str'"),
            TInt8 => write!(f, "'i8'"),
            TInt16 => write!(f, "'i16'"),
            TInt32 => write!(f, "'i32'"),
            TInt64 => write!(f, "'i64'"),
            TUInt8 => write!(f, "'u8'"),
            TUInt16 => write!(f, "'u16'"),
            TUInt32 => write!(f, "'u32'"),
            TUInt64 => write!(f, "'u64'"),
            TBool => write!(f, "'bool'"),
        }
    }
}
