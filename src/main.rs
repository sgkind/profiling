use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about, long_about = None)]
struct Args {
    #[arg(short, long)]
    pid: u32,
}

fn main() {
    let args = Args::parse();

    let path = match profiling::process::process_exists(args.pid) {
        Some(path) => path,
        None => {
            eprintln!("process-{} does not exist", args.pid);
            return;
        }
    };

    profiling::process::read_process(path);
}
