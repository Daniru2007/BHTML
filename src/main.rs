mod lexer;
mod read_file;

fn main() {
    let content = read_file::read_file();
    let tokens = lexer::tokenize(content);
    for tok in tokens {
        println!("{:?}", tok);
    }
}
