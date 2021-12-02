use structopt::StructOpt;
mod lib;

#[derive(StructOpt, Debug)]
#[structopt(name = "adventofcode")]
struct Opt {
    #[structopt(short, long)]
    day: i8,
}

fn main() {
    let opt = Opt::from_args();
    lib::run(opt.day);
}
