const TT_ELEMENT: &str = "ELEMENT";
const TT_CLASS_NAME: &str = "CLASS_NAME";
const TT_TEXT: &str = "TEXT";
const TT_NUMBER: &str = "NUMBER";
const TT_ATTRIB_NAME: &str = "ATTRIB_NAME";
const TT_LPARAN: &str = "LPARAN";
const TT_RPARAN: &str = "RPARAN";
const TT_LCURLY: &str = "LCURLY";
const TT_RCURLY: &str = "RCURLY";
const TT_DOT: &str = "DOT";
const TT_HASH: &str = "HASH";
const TT_NEWLINE: &str = "NEWLINE";
const TT_EOF: &str = "EOF";

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

pub struct Token<'a, 'b> {
    token_type: &'a str,
    token_val: Option<&'b str>,
}

impl<'a, 'b> Token<'a, 'b> {}

pub fn tokenize(content: String) -> Vec<Token<'static, 'static>> {
    let mut tokens: Vec<Token> = Vec::new();
    let content_chars: Vec<char> = content.chars().collect();
    let mut index = 0;
    while index < content_chars.len() {
        println!("{}", content_chars[index]);
        let character = content_chars[index];
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
            tokens.push(Token {
                token_type: token_type,
                token_val: None,
            });
        }
        index += 1;
    }
    println!("[TOKENIZING]");
    return tokens;
}
