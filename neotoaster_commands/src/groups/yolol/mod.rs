use std::convert::TryInto;

use serenity::prelude::*;
use serenity::model::channel::Message;

use serenity::framework::standard::{
    CommandResult,
    Args,
    macros::{
        command,
        group
    }
};

use yoloxide::{
    environment::{
        Environment,
        ContextMap,
    },
    types::{
        Token,
        VecWindow,
        ast::program::Program,
        ast::value::LiteralValue,
    }
};

use tokio::{spawn, task::spawn_blocking};

use cylon_ast::CylonRoot;

use regex::Regex;
use lazy_static::lazy_static;

mod config;
use config::{
    YololConfig,

    InputFlag,
    YololInput,

    OutputFlag
};

#[group]
#[commands(yolol)]
struct Yolol;

lazy_static! {
    static ref CODE_MATCHER: Regex = Regex::new(r"\A(?s:\n*)```(?s:[a-z]*\n)?((?s).*)\n?```\z").expect("Code matching regex failed to compile!");
}

async fn extract_input(input: &str) -> Result<&str, &str>
{
    // The regex ensures the input was formatted into a code block and has a capture group for the text of the input
    let captures = match CODE_MATCHER.captures(input)
    {
        Some(captures) => captures,
        None => {
            return Err("Your supplied code isn't properly put into a code block. Be sure to surround it with triple backticks!")
        }
    };

    // Capture 0 is the whole thing. The input capture is the only capture, meaning it's capture 1
    match captures.get(1)
    {
        Some(capture) => Ok(capture.as_str()),
        None => Err("Something is wrong with extracting input from code block! Someone might have broken the regex we use...")
    }
}

async fn output_execution(input: YololInput, num_ticks: u32, env: &mut Environment) -> Result<(), String>
{
    let tick_limit = num_ticks;

    let code = match input
    {
        YololInput::Yolol(code) => code,
        YololInput::CylonAst(_) => {
            return Err("Execution from Cylon AST not yet supported! Psst, try outputting yolol then using it as input ;)".to_owned())
        }
    };

    let lines: Vec<String> = code.lines().map(String::from).collect();
    let line_len: i64 = lines.len().try_into().unwrap();

    let now = std::time::Instant::now();

    for _ in 0..tick_limit
    {
        // This is a stupid line but I can't find a better way to do it for some reason...
        let next_line = if env.next_line > line_len || env.next_line <= 0 { 1 } else { env.next_line };
        env.next_line = next_line;

        let next_line: usize = next_line.try_into().unwrap();

        yoloxide::execute_line(env, lines[next_line - 1].clone());
    }

    let elapsed = now.elapsed();

    env.set_val("::execution_time".to_owned(), LiteralValue::StringVal(format!("{} nanoseconds, {} microseconds, {} milliseconds, {} seconds", elapsed.as_nanos(), elapsed.as_micros(), elapsed.as_millis(), elapsed.as_secs())));

    Ok(())
}

async fn output_yolol(input: YololInput) -> Result<String, String>
{
    match parse_yolol(input).await
    {
        Ok(prog) => Ok(format!("{}", prog)),
        Err(e) => Err(e)
    }
}

async fn output_cylon_ast(input: YololInput) -> Result<String, String>
{
    let cylon_root = match input
    {
        YololInput::CylonAst(root) => root,

        yolol => match parse_yolol(yolol).await {
            Ok(prog) => CylonRoot::new(prog.into()),
            Err(e) => return Err(e)
        }
    };

    match serde_json::to_string(&cylon_root)
    {
        Ok(ast) => Ok(ast),
        Err(error) => Err(format!("Converting AST to Cylon AST failed with error: ```{}```", error))
    }
}

async fn output_ast(input: YololInput) -> Result<String, String>
{
    match parse_yolol(input).await
    {
        Ok(prog) => Ok(format!("{:?}", prog)),
        Err(e) => Err(e)
    }
}

async fn output_tokens(input: YololInput) -> Result<String, String>
{
    match tokenize_yolol(input).await
    {
        Ok(tokens) => Ok(format!("{:?}", tokens)),
        Err(e) => Err(e)
    }
}

async fn parse_yolol(input: YololInput) -> Result<Program, String>
{
    if let YololInput::CylonAst(root) = input
    {
        return root.program.try_into();
    }

    let tokens = tokenize_yolol(input).await?;
    let mut window = VecWindow::new(tokens, 0);

    match spawn_blocking(move || yoloxide::parser::parse_program(&mut window)).await.unwrap()
    {
        Ok(prog) => Ok(prog),
        Err(error) => Err(format!("Parser failure: ```{}```", error)),
    }
}

async fn tokenize_yolol(input: YololInput) -> Result<Vec<Token>, String>
{
    let code = match input
    {
        YololInput::Yolol(code) => code,
        YololInput::CylonAst(_) => {
            return Err("Can't tokenize a Cylon AST!".to_owned())
        }
    };

    match spawn_blocking(|| yoloxide::tokenizer::tokenize(code)).await.unwrap()
    {
        Ok(tokens) => Ok(tokens),
        Err(error) => Err(format!("Tokenizer failure: ```{}```", error)),
    }
}

#[command]
async fn yolol(context: &Context, message: &Message, args: Args) -> CommandResult
{
    // Wrap the provided args in a new args struct, so we can control the delimiters used
    let mut args = {
        use serenity::framework::standard::{Args, Delimiter};
        Args::new(args.message(), &[Delimiter::Single('\n'), Delimiter::Single(' ')])
    };

    // Parse arguments into a YololConfig
    let config = match YololConfig::parse_args(&mut args)
    {
        Ok(config) => config,
        Err(error) => {
            message.channel_id.say(&context.http, error).await?;
            return Ok(())
        }
    };

    // Anything after the flags is expected to be the input
    let input = match extract_input(args.rest()).await
    {
        Ok(input) => input,
        Err(error) => {
            message.channel_id.say(&context.http, error).await?;
            return Ok(())
        }
    };

    // Quickly checks to make sure there's no backticks in the code, since they can break output formatting
    if input.contains('`')
    {
        message.channel_id.say(&context.http, "Your supplied code contains some backticks! No trying to break the output code blocks ;)").await?;
        return Ok(())
    }

    let input = match config.input
    {
        InputFlag::Yolol => YololInput::Yolol(input.to_owned()),

        InputFlag::CylonAst => match serde_json::from_str(input) {
            Ok(root) => YololInput::CylonAst(root),
            Err(error) => {
                message.channel_id.say(&context.http, format!("Converting Cylon AST json to internal representation failed with error: ```{}```", error)).await?;
                return Ok(())
            }
        },
    };

    println!("Output: {:?}, input: {:?}", config.input, config.output);

    match config.output
    {
        OutputFlag::Execution => {
            let mut env = Environment::new("Bot");

            match output_execution(input, config.num_ticks, &mut env).await
            {
                Ok(_) => (),
                Err(e) => {
                    message.channel_id.say(&context.http, e).await?;
                    return Ok(())
                }
            }

            let output = env.to_string();
            if output.len() > 1900
            {
                use serenity::http::AttachmentType;
                let attachment = vec![AttachmentType::Bytes {
                    data: output.as_bytes().into(),
                    filename: "toaster_output.txt".to_string(),
                }];

                message.channel_id.send_files(&context.http, attachment, |m| m.content("The output was too long! Here's a file instead")).await?;
            }
            else
            {
                let output = format!("Output environment from execution: ```{}```", output);
                message.channel_id.say(&context.http, output).await?;
            }
        },
        OutputFlag::Yolol => {
            let output = match output_yolol(input).await
            {
                Ok(o) => o,
                Err(e) => {
                    message.channel_id.say(&context.http, e).await?;
                    return Ok(());
                } 
            };

            let output = format!("Reconstructed code: ```{}```", output);
            message.channel_id.say(&context.http, output).await?;
        },
        OutputFlag::CylonAst => {
            let output = match output_cylon_ast(input).await
            {
                Ok(o) => o,
                Err(e) => {
                    message.channel_id.say(&context.http, e).await?;
                    return Ok(());
                }
            };

            use serenity::http::AttachmentType;

            if output.len() > 1900
            {
                let attachment = vec![AttachmentType::Bytes {
                    data: output.as_bytes().into(),
                    filename: "cylon_ast.json".to_string(),
                }];

                message.channel_id.send_files(&context.http, attachment, |m| m.content("The code was too long! Here's a file instead")).await?;
            }
            else
            {
                let output = format!("Cylon AST of program:\n```json\n{}\n```", output);
                message.channel_id.say(&context.http, output).await?;
            }

        },
        OutputFlag::Ast => {
            let output = match output_ast(input).await
            {
                Ok(o) => o,
                Err(e) => {
                    message.channel_id.say(&context.http, e).await?;
                    return Ok(());
                }
            };

            let output = format!("Parsed program: ```{:?}```", output);
            message.channel_id.say(&context.http, output).await?;
        },
        OutputFlag::Tokens => {
            let output = match output_tokens(input).await
            {
                Ok(o) => o,
                Err(e) => {
                    message.channel_id.say(&context.http, e).await?;
                    return Ok(());
                }
            };

            let output = format!("Tokenized program: ```{:?}```", output);
            message.channel_id.say(&context.http, output).await?;
        }
    }

    Ok(())
}