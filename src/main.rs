use icfp2019::prelude::Result;
use loggerv;
use structopt::StructOpt;

#[derive(StructOpt, Debug)]
struct Opt {
    #[structopt(short = "v", parse(from_occurrences))]
    verbose: u64,
    #[structopt(subcommand)]
    cmd: Command,
}

#[derive(StructOpt, Debug)]
enum Command {
    #[structopt(name = "run")]
    Run {
        #[structopt(long = "id")]
        id: Option<u64>,
    },
    #[structopt(name = "run-all")]
    RunAll,
    #[structopt(name = "test-run")]
    TestRun {
        #[structopt(long = "id")]
        id: Option<u64>,
    },
    #[structopt(name = "report")]
    Report,
    #[structopt(name = "update-best")]
    UpdateBest,
    #[structopt(name = "ci")]
    Ci,
}

fn main() -> Result<()> {
    let opt = Opt::from_args();
    loggerv::init_with_verbosity(opt.verbose).unwrap();
    match opt.cmd {
        Command::Run { id } => icfp2019::run::run(id.unwrap_or(0)),
        Command::TestRun { id } => icfp2019::run::test_run(id.unwrap_or(0)),
        Command::RunAll => icfp2019::run::run_all(),
        Command::Report => icfp2019::run::report(),
        Command::UpdateBest => icfp2019::run::update_best(),
        Command::Ci => unimplemented!(),
    }
}
