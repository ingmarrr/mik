use clap::Parser;

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
            let asm = mik::gen();
            mik::write(&file, &asm);
            mik::run(&file).unwrap();
        }
    }
}
