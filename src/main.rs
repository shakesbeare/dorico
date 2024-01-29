use clap::Parser;
use std::process::Command;

const GITIGNORE: &'static str = r#"
out/
*.pdf
*.wav
"#;

#[derive(Debug, Parser)]
struct Cli {
    #[clap(subcommand)]
    subcmd: SubCommand,
}

#[derive(Debug, Parser)]
enum SubCommand {
    #[clap(name = "init")]
    Init,
    #[clap(name = "new")]
    New {
        name: String,
    },
}

// project file structure
// project-name/
//     .git
//     .gitignore
//     out/
//     project_name.dorico

fn init() {
    // get the name of the current directory and use it as the project name
    let name = std::env::current_dir()
        .unwrap()
        .file_name()
        .unwrap()
        .to_str()
        .unwrap()
        .to_string();

    // initialize git
    Command::new("git")
        .arg("init")
        .output()
        .expect("failed to initialize git");
    
    // create .gitignore file
    std::fs::write(".gitignore", GITIGNORE).expect("failed to create .gitignore file");

    // create ./out/ directory
    std::fs::create_dir("out").expect("failed to create out directory");

    // install git lfs
    Command::new("git")
        .arg("lfs")
        .arg("install")
        .output()
        .expect("failed to install git lfs");

}

fn new(name: String) {
    // create new dir with the project name
    std::fs::create_dir(&name).expect("failed to create new project directory");

    // change to the new directory
    std::env::set_current_dir(&name).expect("failed to change to new project directory");

    init();
}

fn main() {
    let app = Cli::parse();

    match app.subcmd {
        SubCommand::Init => init(),
        SubCommand::New { name } => new(name),
    }
}
