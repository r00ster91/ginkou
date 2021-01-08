use crate::{
    lang,
    menu::{self, terminal},
    Language,
};
use std::{
    collections::HashMap,
    fs,
    io::{self, BufRead, Write},
    process,
};

#[derive(Debug)]
pub struct Account {
    name: String,
    password: String,
    balance: String,
}

#[macro_export]
macro_rules! account_file_name {
    () => {
        "銀行アカウント"
    };
}

pub fn read_accounts() -> Result<HashMap<String, Account>, ()> {
    let mut accounts = HashMap::<String, Account>::new();
    if let Ok(file) = fs::File::open(account_file_name!()) {
        let reader = io::BufReader::new(file);
        for line in reader.lines() {
            let mut id: Option<String> = None;
            let mut name: Option<String> = None;
            let mut password: Option<String> = None;
            let mut balance: Option<String> = None;
            for (index, part) in line.unwrap().split('\0').enumerate() {
                let part = part.to_owned();
                match index {
                    0 => id = Some(part),
                    1 => name = Some(part),
                    2 => password = Some(part),
                    _ => balance = Some(part),
                }
            }
            if let (Some(id), Some(name), Some(password), Some(balance)) =
                (id, name, password, balance)
            {
                accounts.insert(
                    id,
                    Account {
                        name,
                        password,
                        balance,
                    },
                );
            } else {
                return Err(());
            }
        }
        Ok(accounts)
    } else {
        Ok(accounts)
    }
}

fn write_accounts(l: &Language, accounts: &HashMap<String, Account>) {
    let mut file = fs::File::create(account_file_name!()).unwrap_or_else(|_| {
        eprintln!("{}", l.file_error);
        process::exit(1);
    });

    for (id, account) in accounts {
        writeln!(
            file,
            "{}\0{}\0{}\0{}",
            id, account.name, account.password, account.balance
        )
        .unwrap_or_else(|_| {
            eprintln!("{}", l.write_failed);
            process::exit(1);
        });
    }
}

pub fn sign_in(l: &Language, accounts: &mut HashMap<String, Account>) {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    let mut id = String::new();
    read(&l, "ID", &mut id, &stdin, &mut stdout);
    let mut password = String::new();
    read(&l, l.password, &mut password, &stdin, &mut stdout);

    if let Some(mut account) = accounts.get_mut(&id) {
        if account.password == password {
            println!("{}\n", l.signed_in);
            loop {
                println!(
                    "{}: {}\n{}: {}",
                    l.name, &account.name, l.balance, &account.balance
                );
                let mut menu =
                    menu::Menu::new(vec![l.deposit, l.withdraw, l.delete_account, l.done]);
                let option = menu.read();
                if option == l.deposit {
                    let mut input = String::new();
                    read(&l, l.amount, &mut input, &stdin, &mut stdout);
                    if let Ok(amount) = input.parse::<usize>() {
                        account.balance =
                            (account.balance.parse::<i32>().unwrap() + amount as i32).to_string();
                    } else {
                        println!("{}", l.invalid_amount);
                    }
                } else if option == l.withdraw {
                    let mut input = String::new();
                    read(&l, l.amount, &mut input, &stdin, &mut stdout);
                    if let Ok(amount) = input.parse::<usize>() {
                        account.balance =
                            (account.balance.parse::<i32>().unwrap() - amount as i32).to_string();
                    } else {
                        println!("{}", l.invalid_amount);
                    }
                } else if option == l.delete_account {
                    let mut name = String::new();
                    read(&l, l.name, &mut name, &stdin, &mut stdout);
                    if name == account.name {
                        accounts.remove(&id);
                        break;
                    } else {
                        println!("{}", l.name_mismatch);
                    }
                } else if option == l.done {
                    break;
                }
            }
            write_accounts(&l, &accounts);
        } else {
            println!("{}", l.password_mismatch);
        }
    } else {
        println!("{}", l.invalid_id);
    }
}

pub fn sign_up(l: &Language, accounts: &mut HashMap<String, Account>) {
    let stdin = io::stdin();
    let mut stdout = io::stdout();

    let mut name = String::new();
    read(&l, l.name, &mut name, &stdin, &mut stdout);

    let mut password = String::new();
    let mut confirmation_password = String::new();

    terminal::echo(&stdin, false);
    loop {
        read(&l, l.password, &mut password, &stdin, &mut stdout);
        println!();
        read(
            &l,
            l.password_confirmation,
            &mut confirmation_password,
            &stdin,
            &mut stdout,
        );

        if password == confirmation_password {
            break;
        }

        println!("{}", l.password_mismatch);
        password.clear();
        confirmation_password.clear();
    }
    terminal::echo(&stdin, true);

    let id = accounts.len().to_string();

    let desu = if *l == lang::japanese::JAPANESE {
        "です。"
    } else {
        "."
    };

    println!("\n{}{}{}{}", l.registration_finish, l.id_is, id, desu);

    accounts.insert(
        id,
        Account {
            name,
            password,
            balance: "0".to_string(),
        },
    );

    write_accounts(&l, &accounts);
}

fn read(
    l: &Language,
    thing: &str,
    /*into*/ buffer: &mut String,
    /*from*/ stdin: &io::Stdin,
    stdout: &mut io::Stdout,
) -> usize {
    let mut length;

    loop {
        print!("{}: ", thing);
        stdout.flush().unwrap();
        if stdin.read_line(buffer).is_ok() {
            *buffer = buffer.trim().to_string();
            length = buffer.chars().count();
            if length < 1 {
                println!("{}{}", thing, l.length_minimum);
                stdout.flush().unwrap();
                buffer.clear();
                continue;
            }
            break;
        }
    }

    length
}
