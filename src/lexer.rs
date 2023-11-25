pub static TT_ELEMENT: &str = "ELEMENT";
pub static TT_CLASS_NAME: &str = "CLASS_NAME";
pub static TT_ID_NAME: &str = "ID_NAME";
pub static TT_TEXT: &str = "TEXT";
pub static TT_ATTRIB_NAME: &str = "ATTRIB_NAME";
pub static TT_LPARAN: &str = "LPARAN";
pub static TT_RPARAN: &str = "RPARAN";
pub static TT_LCURLY: &str = "LCURLY";
pub static TT_RCURLY: &str = "RCURLY";
pub static TT_NEWLINE: &str = "NEWLINE";
pub static TT_EQ: &str = "EQ";
pub static TT_EOF: &str = "EOF";

static STRING_CHARS: [char; 91] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L',
    'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '-', '_', '=', '~', '.',
    ' ', ';', ':', '[', ']', '<', '>', '/', '?', '`', '!', '@', '#', '%', '^', '&', '*', '(', ')',
    '"', '\\', '\n', '\t', '1', '2', '3', '4', '5', '6', '7', '8', '9', '0', '\r',
];

static TEXT_CHARS: [char; 65] = [
    'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r', 's',
    't', 'u', 'v', 'w', 'x', 'y', 'z', 'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L',
    'M', 'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z', '-', '_', '~', '1', '2',
    '3', '4', '5', '6', '7', '8', '9', '0',
];

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

fn is_attrib(attrib: &str) -> bool {
    let attribs: Vec<&str> = vec![
        "accept",
        "accept-charset",
        "accesskey",
        "action",
        "align",
        "allow",
        "alt",
        "async",
        "autocapitalize",
        "autocomplete",
        "autofocus",
        "autoplay",
        "background",
        "bgcolor",
        "border",
        "buffered",
        "capture",
        "challenge",
        "charset",
        "checked",
        "cite",
        "class",
        "code",
        "codebase",
        "color",
        "cols",
        "colspan",
        "content",
        "contenteditable",
        "contextmenu",
        "controls",
        "coords",
        "crossorigin",
        "csp Experimental",
        "data",
        "data-*",
        "datetime",
        "decoding",
        "default",
        "defer",
        "dir",
        "dirname",
        "disabled",
        "download",
        "draggable",
        "enctype",
        "enterkeyhint",
        "for",
        "form",
        "formaction",
        "formenctype",
        "formmethod",
        "formnovalidate",
        "formtarget",
        "headers",
        "height",
        "hidden",
        "high",
        "href",
        "hreflang",
        "http-equiv",
        "id",
        "integrity",
        "intrinsicsize",
        "inputmode",
        "ismap",
        "itemprop",
        "keytype",
        "kind",
        "label",
        "lang",
        "language",
        "loading",
        "list",
        "loop",
        "low",
        "max",
        "maxlength",
        "minlength",
        "media",
        "method",
        "min",
        "multiple",
        "muted",
        "name",
        "novalidate",
        "open",
        "optimum",
        "pattern",
        "ping",
        "placeholder",
        "playsinline",
        "poster",
        "preload",
        "readonly",
        "referrerpolicy",
        "rel",
        "required",
        "reversed",
        "role",
        "rows",
        "rowspan",
        "sandbox",
        "scope",
        "scoped",
        "selected",
        "shape",
        "size",
        "sizes",
        "slot",
        "span",
        "spellcheck",
        "src",
        "srcdoc",
        "srclang",
        "srcset",
        "start",
        "step",
        "style",
        "tabindex",
        "target",
        "title",
        "translate",
        "type",
        "usemap",
        "value",
        "width",
        "wrap",
    ];
    return attribs.contains(&attrib);
}

fn is_text(character: char) -> bool {
    return TEXT_CHARS.contains(&character);
}

fn is_string(character: char) -> bool {
    return STRING_CHARS.contains(&character);
}

#[derive(Clone, Debug)]
pub struct Token {
    pub token_type: String,
    pub token_val: Option<String>,
}

impl Token {}

#[derive(Debug)]
pub struct Lexer {
    tokens: Vec<Token>,
    content_chars: Vec<char>,
    index: usize,
}

impl Lexer {
    pub fn new(content: String) -> Lexer {
        let tokens = Vec::new();
        let content_chars = content.chars().collect();
        let index = 0;
        return Lexer {
            tokens,
            content_chars,
            index,
        };
    }

    fn advance(&mut self) {
        self.index += 1;
    }

    fn add_tok(&mut self, token_type: &str, token_val: Option<&str>) {
        let value = match token_val {
            Some(t) => Some(String::from(t)),
            None => None,
        };
        self.tokens.push(Token {
            token_type: String::from(token_type),
            token_val: value,
        })
    }

    fn find_text(&mut self) -> String {
        let mut value = String::new();
        while is_text(self.content_chars[self.index]) {
            value.push(self.content_chars[self.index]);
            self.advance();
        }
        return value;
    }

    fn find_string(&mut self) -> String {
        let mut value = String::new();
        while is_string(self.content_chars[self.index]) {
            if self.content_chars[self.index] == '"' {
                self.advance();
                break;
            }
            if self.content_chars[self.index] == '\\' {
                self.advance();
                match self.content_chars[self.index] {
                    'n' => value.push('\n'),
                    't' => value.push('\t'),
                    'r' => value.push('\r'),
                    '\\' => value.push('\\'),
                    '\'' => value.push('\''),
                    '"' => value.push('"'),
                    _ => value.push(self.content_chars[self.index]),
                };
                self.advance();
                continue;
            }
            value.push(self.content_chars[self.index]);
            self.advance();
        }
        return value;
    }

    pub fn tokenize(&mut self) {
        while self.index < self.content_chars.len() {
            let character = self.content_chars[self.index];
            if character == '(' {
                self.add_tok(TT_LPARAN, None);
                self.advance();
            } else if character == ')' {
                self.add_tok(TT_RPARAN, None);
                self.advance();
            } else if character == '{' {
                self.add_tok(TT_LCURLY, None);
                self.advance();
            } else if character == '}' {
                self.add_tok(TT_RCURLY, None);
                self.advance();
            } else if character == '.' {
                self.advance();
                let value = self.find_text();
                self.add_tok(TT_CLASS_NAME, Some(&value[..]));
            } else if character == '#' {
                self.advance();
                let value = self.find_text();
                self.add_tok(TT_ID_NAME, Some(&value[..]));
            } else if character == '=' {
                self.advance();
                self.add_tok(TT_EQ, None);
            } else if character == '"' {
                self.advance();
                let value = self.find_string();
                self.add_tok(TT_TEXT, Some(&value[..]));
            } else if is_text(character) {
                let mut t_type = "";
                let value = self.find_text();
                if is_element(&value[..]) {
                    t_type = TT_ELEMENT;
                } else if is_attrib(&value[..]) {
                    t_type = TT_ATTRIB_NAME;
                }
                // assert_eq!(t_type, "");
                if value != "" {
                    self.add_tok(t_type, Some(&value[..]));
                }
            } else if character == ' ' {
                self.advance();
            } else if character == '\t' {
                self.advance();
            } else if character == '\n' {
                self.add_tok(TT_NEWLINE, None);
                self.advance();
            } else {
                self.advance();
            }
        }
        self.add_tok(TT_EOF, None);
        println!("[TOKENIZING]");
    }
}

// Tokenizer Function
pub fn tokenize(content: String) -> Vec<Token> {
    let mut lexer = Lexer::new(content);
    lexer.tokenize();
    return lexer.tokens.clone();
}
