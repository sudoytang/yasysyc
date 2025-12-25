use std::fs::read_to_string;
use anyhow::Result;

use clap::Parser;
use koopa::back::KoopaGenerator;
use lalrpop_util::lalrpop_mod;

lalrpop_mod!(sysy);

#[derive(Parser)]
#[command(name = "yasysyc")]
#[command(about = "SysY compiler", long_about = None)]
struct Cli {
    /// Input SysY source file
    input: String,

    /// Output file path (optional)
    #[arg(short, long)]
    output: Option<String>,

    #[arg(long)]
    debug: bool,
}



fn main() -> Result<()> {
    // let cli = Cli::parse();

    // >>>> DEBUG
    let debug_cli = Cli {
        input: "test.c".into(),
        output: None,
        debug: false,
    };

    let cli = debug_cli;
    // <<<< DEBUG

    let input = read_to_string(&cli.input)?;

    let ast = sysy::CompUnitParser::new().parse(&input)
        .map_err(|e| anyhow::anyhow!("Failed to parse input: {}", e))?;

    // now ir is just a string
    let ir = if cli.debug {
        format!("{:#?}", ast)
    } else {
        let ir =ast.emit();
        let mut writer = Vec::new();
        KoopaGenerator::new(&mut writer).generate_on(&ir)?;
        String::from_utf8(writer)?
    };

    if let Some(output) = cli.output {
        std::fs::write(output, ir.as_bytes())?;
    } else {
        // use stdout
        println!("{}", ir);
    }

    Ok(())
}
