use serenity::framework::standard::{
    Args,
};

use cylon_ast::CylonRoot;

#[derive(Debug)]
pub struct YololConfig
{
    pub input: InputFlag,
    pub output: OutputFlag,
    pub time_execution: bool,
    pub num_ticks: u32,
}

#[derive(Debug)]
pub enum InputFlag
{
    Yolol,
    CylonAst
}

// #[derive(Debug)]
pub enum YololInput
{
    Yolol(String),
    CylonAst(CylonRoot)
}

#[derive(Debug)]
pub enum OutputFlag
{
    Execution,
    Yolol,
    CylonAst,
    Ast,
    Tokens
}

impl YololConfig
{
    pub fn new() -> Self
    {
        YololConfig {
            input: InputFlag::Yolol,
            output: OutputFlag::Execution,
            time_execution: true,
            num_ticks: 1000,
        }
    }

    pub fn parse_args(args: &mut Args) -> Result<Self, String>
    {
        let mut config = YololConfig::new();

        loop
        {
            let current = match args.current()
            {
                Some(args) => args,
                None => return Err("Unable to get an argument! Might not have supplied any args...".to_owned())
            };

            println!("Current arg: {}", current);

            match current
            {
                "--input=cylon_ast" |
                "-ic" => config.input = InputFlag::CylonAst,

                "--output=yolol" |
                "--output=code" |
                "-oy" => config.output = OutputFlag::Yolol,

                "--output=cylon_ast" |
                "-oc" => config.output = OutputFlag::CylonAst,

                "--output=ast" |
                "--output=parsed" |
                "-oa" => config.output = OutputFlag::Ast,

                "--output=tokens" |
                "-ot" => config.output = OutputFlag::Tokens,

                "--no-time-execution" |
                "-nte" => config.time_execution = false,

                "--time-execution" |
                "-te" => config.time_execution = true,

                "--ticks" => {
                    args.advance();
                    let num_ticks = match args.current()
                    {
                        Some(arg) => {
                            println!("Ticks: {:?}", arg);
                            match arg.parse::<u32>() {
                                Ok(num) => num,
                                Err(_error) => return Err("Ticks were not specified in a way that could be turned into a u32! Please give a real number".to_owned())
                            }
                        },
                        None => return Err("Number of ticks not specified after `--ticks` argument! Use it like `--ticks 20`, which would run 20 ticks".to_owned())
                    };

                    config.num_ticks = num_ticks;
                }

                _ => break
            }

            args.advance();
        }

        Ok(config)
    }
}

