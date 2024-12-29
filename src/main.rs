use clap::Parser;

#[derive(Parser)]
struct Args {
    #[clap(short, long)]
    token: Option<String>,
}

fn main() {
    let args = Args::parse();
    let mut bot_token = String::new();

    if let Some(token) = args.token {
        bot_token = token;
    }
}
