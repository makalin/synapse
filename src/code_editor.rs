// use gpui::*; // Commented out for CLI version
use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum SyntaxLanguage {
    Rust,
    Python,
    JavaScript,
    TypeScript,
    Go,
    C,
    Cpp,
    Java,
    Markdown,
    Json,
    Yaml,
    Shell,
    Plain,
}

pub struct SyntaxHighlighter {
    keywords: HashMap<SyntaxLanguage, Vec<&'static str>>,
}

impl SyntaxHighlighter {
    pub fn new() -> Self {
        let mut highlighter = Self {
            keywords: HashMap::new(),
        };
        
        highlighter.initialize_keywords();
        highlighter
    }

    fn initialize_keywords(&mut self) {
        // Rust keywords
        self.keywords.insert(SyntaxLanguage::Rust, vec![
            "fn", "let", "mut", "const", "struct", "enum", "impl", "trait",
            "pub", "use", "mod", "async", "await", "match", "if", "else",
            "for", "while", "loop", "return", "break", "continue",
        ]);

        // Python keywords
        self.keywords.insert(SyntaxLanguage::Python, vec![
            "def", "class", "if", "else", "elif", "for", "while", "return",
            "import", "from", "as", "try", "except", "finally", "with",
            "async", "await", "yield", "lambda", "pass", "break", "continue",
        ]);

        // JavaScript keywords
        self.keywords.insert(SyntaxLanguage::JavaScript, vec![
            "function", "const", "let", "var", "if", "else", "for", "while",
            "return", "async", "await", "class", "extends", "import", "export",
            "try", "catch", "finally", "throw", "new", "this", "super",
        ]);
    }

    pub fn highlight(&self, code: &str, language: &SyntaxLanguage) -> Vec<Token> {
        let keywords = self.keywords.get(language).cloned().unwrap_or_default();
        let mut tokens = Vec::new();
        let lines: Vec<&str> = code.lines().collect();

        for (line_num, line) in lines.iter().enumerate() {
            let mut current_pos = 0;
            let words: Vec<&str> = line.split_whitespace().collect();

            for word in words {
                let start = line[current_pos..].find(word).unwrap_or(0) + current_pos;
                let end = start + word.len();

                let token_type = if keywords.iter().any(|&kw| kw == word) {
                    TokenType::Keyword
                } else if word.starts_with('"') || word.starts_with('\'') {
                    TokenType::String
                } else if word.parse::<f64>().is_ok() {
                    TokenType::Number
                } else if word.starts_with("//") || word.starts_with("#") {
                    TokenType::Comment
                } else {
                    TokenType::Text
                };

                tokens.push(Token {
                    line: line_num,
                    start,
                    end,
                    text: word.to_string(),
                    token_type,
                });

                current_pos = end;
            }
        }

        tokens
    }
}

#[derive(Debug, Clone)]
pub struct Token {
    pub line: usize,
    pub start: usize,
    pub end: usize,
    pub text: String,
    pub token_type: TokenType,
}

#[derive(Debug, Clone, PartialEq)]
pub enum TokenType {
    Keyword,
    String,
    Number,
    Comment,
    Text,
}

impl TokenType {
    pub fn color_code(&self) -> u32 {
        match self {
            TokenType::Keyword => 0x88c0d0,
            TokenType::String => 0xa3be8c,
            TokenType::Number => 0xb48ead,
            TokenType::Comment => 0x5e81ac,
            TokenType::Text => 0xd8dee9,
        }
    }
}

pub struct CodeEditor {
    content: String,
    language: SyntaxLanguage,
    highlighter: SyntaxHighlighter,
    line_numbers: bool,
    word_wrap: bool,
}

impl CodeEditor {
    pub fn new() -> Self {
        Self {
            content: String::new(),
            language: SyntaxLanguage::Rust,
            highlighter: SyntaxHighlighter::new(),
            line_numbers: true,
            word_wrap: false,
        }
    }

    pub fn set_language(&mut self, language: SyntaxLanguage) {
        self.language = language;
    }

    pub fn set_content(&mut self, content: String) {
        self.content = content;
    }

    pub fn get_content(&self) -> &str {
        &self.content
    }

    pub fn get_tokens(&self) -> Vec<Token> {
        self.highlighter.highlight(&self.content, &self.language)
    }
}

impl Default for SyntaxHighlighter {
    fn default() -> Self {
        Self::new()
    }
}

// UI rendering code commented out for CLI version
