use clap::Parser;
use mik::lexer::Lexer;

#[derive(clap::Parser)]
struct App {
    #[command(subcommand)]
    cmd: Cmd,
}

#[derive(clap::Subcommand)]
enum Cmd {
    #[clap(name = "run", alias = "r")]
    Run {
        #[clap(value_parser)]
        file: String,
    },
}

fn main() {
    let app = App::parse();

    match app.cmd {
        Cmd::Run { file } => {
            let source = std::fs::read_to_string(&file).expect("failed to read file");
            let lexer = Lexer::new_file(&file, &source);
            let tokens = lexer.tokenize();
            for token in tokens.into_iter() {
                // print alignment
                println!(
                    "[INFO] {:.<30} '{:}'",
                    tokens.kind_of(&token).to_string(),
                    tokens.str_of(&token).escape_debug(),
                )
            }
        }
    }
}
