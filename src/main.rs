#![feature(exact_size_is_empty)]
use std::io::*;
use std::process::*;
macro_rules! scan {
    ( $( $x:ty ),+ ) => {{
        let mut string = String::new();
        stdin().read_line(&mut string).unwrap();
        let mut iter = string.split_whitespace();
        ($(iter.next().and_then(|word| word.parse::<$x>().ok()).unwrap(),)*)
    }}
}
fn find_id_and_mid(op: String) -> (Option<i32>, Option<i32>) {
    let cl = op.clone();
    let l: String = cl
        .lines()
        .find(|&x| x.find("AT Translated Set 2 keyboard").is_some())
        .expect("Cannot find laptop keyboard")
        .into();
    let mut kid = 0;
    for i in l.split_whitespace().collect::<Vec<&str>>() {
        if i.contains("id") {
            kid = i.replace("id=", "").parse().unwrap();
        }
    }
    if l.contains("floating") {
        let mut mid = 0;
        let another = (cl.lines())
            .find(|&x| x.contains("slave  keyboard"))
            .expect("Cannot find another keyboard with kid");
        for i in another.split_whitespace() {
            if i.contains("(") || i.contains(")") || i.contains("]") {
                mid = i
                    .replace("(", "")
                    .replace(")", "")
                    .replace("]", "")
                    .parse()
                    .unwrap();
            }
        }

        return (Some(kid), Some(mid));
    } else {
        return (
            Some(kid),
            Some(
                l.split_whitespace()
                    .rfind(|&x| x.find(char::is_numeric).is_some())
                    .unwrap()
                    .replace("(", "")
                    .replace(")", "")
                    .replace("]", "")
                    .parse()
                    .unwrap(),
            ),
        );
    }
}
fn main() {
    let op1 = String::from_utf8(
        Command::new("xinput")
            .args(["list"])
            .output()
            .expect("Failed to execute the command 'xinput'")
            .stdout,
    )
    .unwrap();
    let mut choice = String::new();
    if !std::env::args().is_empty() {
        for i in std::env::args() {
            if i.contains("enable") {
                choice = "enable".into();
                break;
            }
            if i.contains("disable") {
                choice = "disable".into();
                break;
            }
        }
    } else {
        println!("laptop keyboard switch\n VERSION 1.0 by Caviar-X");
        print!("What do you want to do (enable[e]/disable[d]):");
        std::io::stdout().flush().unwrap();
        choice = scan!(String).0;
    }

    let (kid, mid) = find_id_and_mid(op1.clone());
    match choice.as_str() {
        "enable" | "e" => {
            Command::new("xinput")
                .args([
                    "reattach",
                    kid.unwrap().to_string().as_str(),
                    mid.unwrap().to_string().as_str(),
                ])
                .spawn()
                .unwrap();
            println!("Success!");
        }
        "disable" | "d" => {
            if op1
                .clone()
                .lines()
                .find(|&x| x.find("AT Translated Set 2 keyboard").is_some())
                .expect("Cannot find laptop keyboard")
                .to_string()
                .contains("floating")
            {
                println!("Success!");
                std::process::exit(0);
            }
            Command::new("xinput")
                .args(["float", kid.unwrap().to_string().as_str()])
                .spawn()
                .unwrap();
            println!("Success!");
        }
        _ => {
            println!("Unkown choice");
            std::process::exit(1);
        }
    }
}
