use crate::scanner;

pub fn run(program: &String) {
    let s = scanner::Scanner::new(program);
    let toks = s.scan_tokens();

    toks.iter().for_each(|t| {
        println!("{}", t);
    })
}
