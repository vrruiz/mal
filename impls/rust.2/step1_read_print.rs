use std::io::{self, Write};

fn repl_read(param: &str) -> &str {
    param
}

fn repl_eval(param: &str) -> &str {
    param
}

fn repl_print(param: &str) -> &str {
    param
}

fn repl_rep(param: &str) -> &str {
    param
}

fn main() -> io::Result<()> {
    let mut buffer = String::new();
    let mut cont = true;

    while cont == true {
        print!("user> ");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut buffer)?;
        if buffer.starts_with("exit") {
            cont = false;
        } else {
            println!("{}", buffer);
            buffer.clear();
        }
    }
    Ok(())
}
