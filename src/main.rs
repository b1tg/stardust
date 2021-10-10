use clipboard::ClipboardContext;
use clipboard::ClipboardProvider;
use cmd_lib::run_cmd;
use colored::*;
use stardict_rs::*;
const MAX_RESULTS: u8 = 2;

fn main() {
    let mut bar = sysbar::Sysbar::new("stardust");
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

    bar.add_item(
        "translate clipboard",
        Box::new(move || {
            let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
            let clip = format!("{}", ctx.get_contents().unwrap());
            println!("translate clipboard: {}", clip);
            let result = translate(&dicts, &clip);
            notification(&clip, &result);
        }),
    );
    bar.add_quit_item("Quit");

    bar.display();

    // loop {
    //     let mut cmd = String::new();
    //     print!(">> ");
    //     let _ = io::stdout().flush();
    //     let _ = io::stdin().read_line(&mut cmd);
    //     translate(&dicts, &cmd);
    // }
}

fn notification(title: &str, content: &str) {
    let greet = format!(
        "display notification \"{}\" with title \"{}\"",
        content, title
    );
    // let greet = "display notification \"greet\"";
    let _ = run_cmd!(osascript -e $greet);
}

fn translate(dicts: &Vec<Dict>, word: &str) -> String {
    let cmd = word.trim().to_string().to_ascii_lowercase();
    let mut word_found_times = 0;
    let mut result = "".to_string();
    for dict in dicts {
        let name = &dict.name;
        let a = adjust_word_suffix_iter(&cmd);
        for i in a {
            let res = dict.find_word(&i);
            if res.is_some() {
                word_found_times += 1;
                // let pp = format!("{} => {}", name, i);
                // result += &format!("{}\n", pp.bold());
                if name == "oxford-gb" {
                    result += &format!("{}\n", format_oxford_gb(res.unwrap()));
                } else {
                    result += &format!("{}\n", res.unwrap());
                }
                result += &format!("{}\n", "==========".bold());
                break; // max 1 hit per dictionary
            }
        }
        if word_found_times >= MAX_RESULTS {
            break;
        }
    }
    if word_found_times == 0 {
        result += &format!("{} not found\n", &cmd);
    }
    return result;
}
