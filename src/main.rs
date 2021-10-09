use colored::*;
use stardict_rs::*;
use std::io;
use std::io::Write;

const MAX_RESULTS: u8 = 2;

fn main() {
    let mut dicts: Vec<Dict> = vec![];

    dicts.push(Dict::new(
        "langdao-ec-gb",
        "~/Downloads/stardict-files/stardict-langdao-ec-gb-2.4.2/langdao-ec-gb",
    ));

    // http://download.huzheng.org/zh_CN/stardict-oxford-gb-2.4.2.tar.bz2
    dicts.push(Dict::new(
        "oxford-gb",
        "~/Downloads/stardict-files/stardict-oxford-gb-2.4.2/oxford-gb",
    ));

    loop {
        let mut cmd = String::new();
        print!(">> ");
        let _ = io::stdout().flush();
        let _ = io::stdin().read_line(&mut cmd);
        cmd = cmd.trim().to_string().to_ascii_lowercase();
        let mut word_found_times = 0;
        for dict in &dicts {
            let name = &dict.name;
            let a = adjust_word_suffix_iter(&cmd);
            for i in a {
                let res = dict.find_word(&i);
                if res.is_some() {
                    word_found_times += 1;
                    let pp = format!("{} => {}", name, i);
                    println!("{}", pp.bold());
                    if name == "oxford-gb" {
                        println!("{}", format_oxford_gb(res.unwrap()));
                    } else {
                        println!("{}", res.unwrap());
                    }
                    println!("{}", "==========".bold());
                    break; // max 1 hit per dictionary
                }
            }
            if word_found_times >= MAX_RESULTS {
                break;
            }
        }
        if word_found_times == 0 {
            println!("{} not found", &cmd);
        }
    }
}
