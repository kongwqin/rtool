extern crate clipboard;
use clap::{Args, Parser, Subcommand};
use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;
use serde_json::{self};

#[derive(Parser)]
struct Config{
  #[command(subcommand)]
   command:Option<Commands>,
}

#[derive(Subcommand)]
enum Commands {
    Clip(Clip),
    Excel(Clip)
}

#[derive(Args)]
struct Clip {
    #[arg(short, long)]
    output:String
}

fn operate_clipboard(output:&String){
    let mut ctx = ClipboardContext::new().unwrap();
    let content = ctx.get_contents().unwrap();
    if !content.is_empty() {
        let mut array: Vec<String> = Vec::new();
        let iter = content.split("\n");
        for item in iter {
            let data = item.to_string();
            array.push(data);
        }
        let mut res: String = String::from("");
        if output == "json" {
            res = serde_json::to_string(&array).unwrap()
        } else if output == "sql" {
           let mut result_list: Vec<String> = Vec::new();
           for item in  array.iter() {
               let dd: String = format!("'{}'", item);
               result_list.push(dd);
           }
           res = result_list.join(",");
        }
        println!("{}, {}", res, output);
        ctx.set_contents(res).unwrap();
    }
}

fn main() {
    let config: Config = Config::parse();
     match &config.command {
         Some(Commands::Clip(args)) => {
            let output = &args.output;
            operate_clipboard(output);
         }
         Some(Commands::Excel(args)) => {
            let output = &args.output;
            operate_clipboard(output);
         }
         None => {
            print!("ssd");
         },
     }
}
