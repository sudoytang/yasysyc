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

    /// Generate Koopa IR (otherwise generate target code)
    #[arg(long)]
    koopa: bool,

    /// Generate RISC-V target code
    #[arg(long)]
    riscv: bool,

    #[arg(long)]
    debug: bool,
}



fn main() -> Result<()> {
    // Replace -koopa with --koopa for clap compatibility
    // (test tool uses -koopa, but clap expects --koopa for long options)
    let args = std::env::args()
        .map(|arg| {
            match arg.as_str() {
                "-koopa" => "--koopa".to_string(),
                "-riscv" => "--riscv".to_string(),
                _ => arg,
            }
        });
    let cli = Cli::try_parse_from(args).unwrap_or_else(|e| e.exit());



    // // >>>> DEBUG
    // let debug_cli = Cli {
    //     input: "test.c".into(),
    //     output: None,
    //     debug: false,
    // };

    // let cli = debug_cli;
    // // <<<< DEBUG

    let input = read_to_string(&cli.input)?;

    let ast = sysy::CompUnitParser::new().parse(&input)
        .map_err(|e| anyhow::anyhow!("Failed to parse input: {}", e))?;


    if cli.debug {
        println!("{:#?}", ast);
        return Ok(());
    }

    let koopa_ir = ast.emit();

    if cli.koopa {
        let mut writer = Vec::new();
        KoopaGenerator::new(&mut writer).generate_on(&koopa_ir)?;
        let ir = String::from_utf8(writer)?;
        if let Some(output) = cli.output {
            std::fs::write(output, ir.as_bytes())?;
        } else {
            println!("{}", ir);
        }
        return Ok(());
    }

    if cli.riscv {
        unimplemented!()
    }

    Ok(())
}
