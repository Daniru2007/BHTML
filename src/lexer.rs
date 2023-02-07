static TT_ELEMENT: &str = "ELEMENT";
static TT_CLASS_NAME: &str = "CLASS_NAME";
static TT_TEXT: &str = "TEXT";
static TT_NUMBER: &str = "NUMBER";
static TT_ATTRIB_NAME: &str = "ATTRIB_NAME";
static TT_LPARAN: &str = "LPARAN";
static TT_RPARAN: &str = "RPARAN";
static TT_LCURLY: &str = "LCURLY";
static TT_RCURLY: &str = "RCURLY";
static TT_DOT: &str = "DOT";
static TT_HASH: &str = "HASH";
static TT_NEWLINE: &str = "NEWLINE";
static TT_EOF: &str = "EOF";

fn is_element(element: &str) -> bool {
    let elements: Vec<&str> = vec![
        "a",
        "abbr",
        "acronym",
        "address",
        "applet",
        "area",
        "article",
        "aside",
        "audio",
        "b",
        "base",
        "basefont",
        "bdi",
        "bdo",
        "big",
        "blockquote",
        "body",
        "br",
        "button",
        "canvas",
        "caption",
        "center",
        "cite",
        "code",
        "col",
        "colgroup",
        "data",
        "datalist",
        "dd",
        "del",
        "details",
        "dfn",
        "dialog",
        "dir",
        "div",
        "dl",
        "dt",
        "em",
        "embed",
        "fieldset",
        "figcaption",
        "figure",
        "font",
        "footer",
        "form",
        "frame",
        "frameset",
        "h1",
        "head",
        "header",
        "hr",
        "html",
        "i",
        "iframe",
        "img",
        "input",
        "ins",
        "kbd",
        "label",
        "legend",
        "li",
        "link",
        "main",
        "map",
        "mark",
        "meta",
        "meter",
        "nav",
        "noframes",
        "noscript",
        "object",
        "ol",
        "optgroup",
        "option",
        "output",
        "p",
        "param",
        "picture",
        "pre",
        "progress",
        "q",
        "rp",
        "rt",
        "ruby",
        "s",
        "samp",
        "script",
        "section",
        "select",
        "small",
        "source",
        "span",
        "strike",
        "strong",
        "style",
        "sub",
        "summary",
        "sup",
        "svg",
        "table",
        "tbody",
        "td",
        "template",
        "textarea",
        "tfoot",
        "th",
        "thead",
        "time",
        "title",
        "tr",
        "track",
        "tt",
        "u",
        "ul",
        "var",
        "video",
        "wbr",
    ];
    return elements.contains(&element);
}

#[derive(Clone, Copy)]
pub struct Token<'a, 'b> {
    token_type: &'a str,
    token_val: Option<&'b str>,
}

impl<'a, 'b> Token<'a, 'b> {}

pub struct Lexer {
    tokens: Vec<Token<'static, 'static>>,
    content_chars: Vec<char>,
    index: usize,
}

impl Lexer {
    pub fn new(content: String) -> Lexer {
        let tokens = Vec::new();
        let content_chars = content.chars().collect();
        let index = 0;
        return Lexer {
            tokens: tokens,
            content_chars: content_chars,
            index: index,
        };
    }
    pub fn tokenize(&mut self) {
        while self.index < self.content_chars.len() {
            let character = self.content_chars[self.index];
            let token_type;
            match character {
                '(' => token_type = TT_LPARAN,
                ')' => token_type = TT_RPARAN,
                '{' => token_type = TT_LCURLY,
                '}' => token_type = TT_RCURLY,
                '#' => token_type = TT_HASH,
                '.' => token_type = TT_DOT,
                '\n' => token_type = "",
                '\t' => token_type = "",
                _ => token_type = "",
            }
            if token_type != "" {
                println!("{}", token_type);
                self.tokens.push(Token {
                    token_type: token_type.clone(),
                    token_val: None,
                });
            }
            self.index += 1;
        }
        println!("[TOKENIZING]");
    }
}

pub fn tokenize(content: String) -> Vec<Token<'static, 'static>> {
    let mut lexer = Lexer::new(content);
    lexer.tokenize();
    return lexer.tokens.clone();
}
