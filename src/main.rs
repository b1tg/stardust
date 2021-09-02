use colored::*;
use stardict_rs::*;
use std::collections::HashMap;
use std::io;
use std::io::Write;

fn main() {
    let mut dictmap: HashMap<String, Dict> = HashMap::new();

    let mut dict = Dict::default();
    // http://download.huzheng.org/zh_CN/stardict-oxford-gb-2.4.2.tar.bz2
    dict.add_dict_file("/tmp/stardict-oxford-gb-2.4.2/oxford-gb");
    dictmap.insert("oxford-gb".to_string(), dict);

    loop {
        let mut cmd = String::new();
        print!(">> ");
        let _ = io::stdout().flush();
        let _ = io::stdin().read_line(&mut cmd);
        cmd = cmd.trim().to_string();
        let mut word_exist = false;
        loop {
            for (name, dict) in &dictmap {
                let res = dict.find_word(&cmd);
                if res.is_some() {
                    word_exist = true;
                    let pp = format!("{}=>", name);
                    println!("{}", pp.bold());
                    if name == "oxford-gb" {
                        println!("{}", format_oxford_gb(res.unwrap()));
                    } else {
                        println!("{}", res.unwrap());
                    }
                    println!("{}", "==========".bold());
                }
            }
            if !word_exist {
                println!("{} not found", &cmd);
                if cmd.ends_with("ted") {
                    cmd = format!("{}te", cmd[..cmd.len() - 3].to_string());
                } else {
                    break;
                }
            } else {
                break;
            }
        }
    }
}
