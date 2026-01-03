//! Лексер PlantUML на базе logos

use logos::Logos;

/// Токены PlantUML
#[derive(Logos, Debug, Clone, PartialEq)]
#[logos(skip r"[ \t]+")]
pub enum Token {
    // === Структурные токены ===
    #[token("@startuml")]
    StartUml,

    #[token("@enduml")]
    EndUml,

    #[token("\n")]
    Newline,

    // === Ключевые слова sequence diagram ===
    #[token("participant")]
    Participant,

    #[token("actor")]
    Actor,

    #[token("boundary")]
    Boundary,

    #[token("control")]
    Control,

    #[token("entity")]
    Entity,

    #[token("database")]
    Database,

    #[token("collections")]
    Collections,

    #[token("queue")]
    Queue,

    #[token("activate")]
    Activate,

    #[token("deactivate")]
    Deactivate,

    #[token("destroy")]
    Destroy,

    #[token("create")]
    Create,

    // === Фрагменты ===
    #[token("alt")]
    Alt,

    #[token("else")]
    Else,

    #[token("opt")]
    Opt,

    #[token("loop")]
    Loop,

    #[token("par")]
    Par,

    #[token("break")]
    Break,

    #[token("critical")]
    Critical,

    #[token("group")]
    Group,

    #[token("end")]
    End,

    #[token("ref")]
    Ref,

    #[token("note")]
    Note,

    #[token("left")]
    Left,

    #[token("right")]
    Right,

    #[token("over")]
    Over,

    // === Ключевые слова class diagram ===
    #[token("class")]
    Class,

    #[token("interface")]
    Interface,

    #[token("abstract")]
    Abstract,

    #[token("enum")]
    Enum,

    #[token("annotation")]
    Annotation,

    #[token("extends")]
    Extends,

    #[token("implements")]
    Implements,

    #[token("package")]
    Package,

    #[token("namespace")]
    Namespace,

    // === Ключевые слова activity diagram ===
    #[token("start")]
    Start,

    #[token("stop")]
    Stop,

    #[token("if")]
    If,

    #[token("then")]
    Then,

    #[token("elseif")]
    ElseIf,

    #[token("endif")]
    EndIf,

    #[token("while")]
    While,

    #[token("endwhile")]
    EndWhile,

    #[token("repeat")]
    Repeat,

    #[token("fork")]
    Fork,

    #[token("again")]
    Again,

    // === Ключевые слова state diagram ===
    #[token("state")]
    State,

    // === Стрелки sequence ===
    #[regex(r"-+>")]
    ArrowRight,

    #[regex(r"<-+")]
    ArrowLeft,

    #[regex(r"\.+>")]
    DottedArrowRight,

    #[regex(r"<\.+")]
    DottedArrowLeft,

    #[regex(r"-+>>")]
    ThinArrowRight,

    #[regex(r"<<-+")]
    ThinArrowLeft,

    // === Стрелки class ===
    #[token("<|--")]
    Inheritance,

    #[token("--|>")]
    InheritanceReverse,

    #[token("<|..")]
    Realization,

    #[token("..|>")]
    RealizationReverse,

    #[token("*--")]
    Composition,

    #[token("--*")]
    CompositionReverse,

    #[token("o--")]
    Aggregation,

    #[token("--o")]
    AggregationReverse,

    #[token("--")]
    Link,

    #[token("..")]
    DottedLink,

    // === Разделители ===
    #[token(":")]
    Colon,

    #[token("{")]
    BraceOpen,

    #[token("}")]
    BraceClose,

    #[token("(")]
    ParenOpen,

    #[token(")")]
    ParenClose,

    #[token("[")]
    BracketOpen,

    #[token("]")]
    BracketClose,

    #[token("<")]
    AngleOpen,

    #[token(">")]
    AngleClose,

    #[token(",")]
    Comma,

    #[token(";")]
    Semicolon,

    #[token("|")]
    Pipe,

    #[token("=")]
    Equals,

    #[token("==")]
    DoubleEquals,

    // === Модификаторы видимости ===
    #[token("+")]
    Plus,

    #[token("-")]
    Minus,

    #[token("#")]
    Hash,

    #[token("~")]
    Tilde,

    // === Литералы ===
    #[regex(r#""[^"]*""#)]
    String,

    #[regex(r"'[^\n]*")]
    Comment,

    #[regex(r"[a-zA-Z_][a-zA-Z0-9_]*")]
    Identifier,

    #[regex(r"[0-9]+")]
    Number,

    #[regex(r"#[0-9a-fA-F]{3,8}")]
    Color,

    // === Специальные ===
    #[token("as")]
    As,

    #[token("of")]
    Of,

    #[token("end note")]
    EndNote,

    #[regex(r"<<[^>]+>>")]
    Stereotype,
}

impl Token {
    /// Возвращает строковое представление токена
    pub fn as_str(&self) -> &'static str {
        match self {
            Token::StartUml => "@startuml",
            Token::EndUml => "@enduml",
            Token::Participant => "participant",
            Token::Actor => "actor",
            Token::Class => "class",
            Token::Interface => "interface",
            _ => "token",
        }
    }
}

/// Лексер PlantUML
pub struct Lexer<'a> {
    inner: logos::Lexer<'a, Token>,
}

impl<'a> Lexer<'a> {
    /// Создаёт новый лексер
    pub fn new(source: &'a str) -> Self {
        Self {
            inner: Token::lexer(source),
        }
    }
}

impl<'a> Iterator for Lexer<'a> {
    type Item = (Token, &'a str);

    fn next(&mut self) -> Option<Self::Item> {
        loop {
            match self.inner.next()? {
                Ok(token) => {
                    let slice = self.inner.slice();
                    return Some((token, slice));
                }
                Err(_) => {
                    // Пропускаем нераспознанные токены
                    continue;
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_tokens() {
        let source = "@startuml\nAlice -> Bob: Hello\n@enduml";
        let lexer = Lexer::new(source);
        let tokens: Vec<_> = lexer.collect();

        assert!(tokens.iter().any(|(t, _)| *t == Token::StartUml));
        assert!(tokens.iter().any(|(t, _)| *t == Token::EndUml));
        assert!(tokens.iter().any(|(t, _)| *t == Token::ArrowRight));
    }

    #[test]
    fn test_class_tokens() {
        let source = "class User {\n  - id: Long\n  + getName(): String\n}";
        let lexer = Lexer::new(source);
        let tokens: Vec<_> = lexer.collect();

        assert!(tokens.iter().any(|(t, _)| *t == Token::Class));
        assert!(tokens.iter().any(|(t, _)| *t == Token::BraceOpen));
        assert!(tokens.iter().any(|(t, _)| *t == Token::Minus));
        assert!(tokens.iter().any(|(t, _)| *t == Token::Plus));
    }

    #[test]
    fn test_stereotype() {
        let source = "class User <<Entity>>";
        let lexer = Lexer::new(source);
        let tokens: Vec<_> = lexer.collect();

        assert!(tokens.iter().any(|(t, _)| *t == Token::Stereotype));
    }
}
