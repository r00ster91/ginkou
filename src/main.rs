use std::{env, process};

mod menu;
mod lang {
    pub mod english;
    pub mod japanese;
}
mod bank;

// Localization
#[derive(PartialEq)]
pub struct Language {
    introduction: &'static str,
    sign_in: &'static str,
    sign_up: &'static str,
    exit: &'static str,
    password_mismatch: &'static str,
    name_mismatch: &'static str,
    registration_finish: &'static str,
    file_error: &'static str,
    invalid_id: &'static str,
    id_is: &'static str,
    name: &'static str,
    password: &'static str,
    password_confirmation: &'static str,
    balance: &'static str,
    length_minimum: &'static str,
    deposit: &'static str,
    withdraw: &'static str,
    delete_account: &'static str,
    done: &'static str,
    amount: &'static str,
    invalid_amount: &'static str,
    signed_in: &'static str,
    corrupted_account_file: &'static str,
    write_failed: &'static str,
    deletion_finish: &'static str,
}

fn main() {
    let mut args = env::args();
    args.next().unwrap();

    // TODO: rewrite this when https://github.com/rust-lang/rust/issues/53667 is implemented
    let mut l = lang::english::ENGLISH;
    if let Some(arg) = args.next() {
        if arg.starts_with("ja") {
            l = lang::japanese::JAPANESE;
        }
    } else if let Ok(lang) = env::var("LANG") {
        if lang.starts_with("ja") {
            l = lang::japanese::JAPANESE;
        }
    };

    let mut accounts = bank::read_accounts().unwrap_or_else(|_| {
        eprintln!("{}", l.corrupted_account_file);
        process::exit(1);
    });

    let mut menu = menu::Menu::new(vec![l.sign_in, l.sign_up, l.exit]);

    loop {
        println!("{}", l.introduction);
        let option = menu.read();
        if option == l.sign_in {
            bank::sign_in(&l, &mut accounts);
        } else if option == l.sign_up {
            bank::sign_up(&l, &mut accounts);
        } else {
            break;
        }
    }
}
