use anyhow::Result;
use chrono::prelude::*;
use clap::Parser;
use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::prelude::*;

#[derive(Parser, Debug)]
struct Args {
    /// Command to execute
    #[command(subcommand)]
    cmd: Command,

    /// Filename to use
    #[arg(short, long, default_value = "capital.act")]
    file: String,
}

#[derive(Parser, Debug)]
enum Command {
    /// Deposit value to an account
    Deposit(ModifyOpt),
    /// Withdrawl value from an account
    Withdrawl(ModifyOpt),
    /// List all accounts and their values
    List,
    /// Update investment value
    Investments(InvestOpt),
    NewUser(ModifyOpt),
    Create(ModifyOpt),
    RemoveUser(RemoveOpt),
    Update(InvestOpt),
}

#[derive(Parser, Debug)]
struct ModifyOpt {
    /// User account to modify
    #[arg(short, long)]
    user: String,

    /// Value of modification
    #[arg(short, long)]
    value: f64,
}

#[derive(Parser, Debug)]
struct InvestOpt {
    /// Value of investments gain/loss
    #[clap(allow_negative_numbers = true)]
    #[arg(short, long)]
    value: f64,
}

#[derive(Parser, Debug)]
struct RemoveOpt {
    /// User account to remove
    #[arg(short, long)]
    user: String,

    /// Method of value removal
    #[command(subcommand)]
    method: Method,
}

#[derive(Parser, Debug)]
enum Method {
    /// Disperse the funds in the account to the other accounts according to their percentages
    Disperse,
    /// Withdrawl the funds in the account being removed
    Withdrawl,
}

struct Ledger {
    filename: String,
    file: File,
}

impl Ledger {
    pub fn load(file: &str) -> Result<Self> {
        let filename = format!("{file}.log");
        let file = File::options().append(true).create(true).open(&filename)?;
        Ok(Self { filename, file })
    }

    pub fn log(&mut self, msg: String) -> Result<()> {
        let message = format!("{}: {msg}\n", Local::now());
        self.file.write_all(message.as_bytes())?;
        println!("Action logged to {}", self.filename);
        Ok(())
    }
}

#[derive(Debug, Serialize, Deserialize)]
struct Account {
    users: Vec<User>,
}

impl Account {
    pub fn new_user(&mut self, name: &str, deposit: f64) {
        // Attempt to deposit first to make sure there isn't already that account
        if self.deposit(name, deposit) {
            // Success, this user already exists
            println!("User {name} already exists!");
            return;
        }
        // Create new user
        let mut user = User::new(name);
        user.deposit(deposit);
        self.users.push(user);
    }

    pub fn get_total(&self) -> f64 {
        let mut total = 0f64;
        for each in &self.users {
            total += each.value;
        }
        total
    }

    pub fn get_percentages(&self) -> Vec<f64> {
        let total = self.get_total();
        let mut p = Vec::new();

        for each in &self.users {
            p.push(each.value / total);
        }
        p
    }

    pub fn investments(&mut self, value: f64) -> bool {
        let mut percentages = self.get_percentages();
        println!("Investments: ${:.02}", value);
        let save_values = self.users.clone();
        for each in &mut self.users {
            let p = percentages.remove(0);
            if (each.value + (p * value)) < 0f64 {
                println!(
                    "This investment causes {} to go negative! Aborting!",
                    each.name
                );
                self.users = save_values;
                return false;
            }
            each.value += p * value;
        }
        true
    }

    pub fn deposit(&mut self, name: &str, value: f64) -> bool {
        for each in &mut self.users {
            if each.name == name {
                each.deposit(value);
                println!("Modified Account: {name}");
                println!("This deposit: ${:.02}", value);
                return true;
            }
        }
        println!("Error: No user \"{name}\"");
        false
    }

    pub fn withdrawl(&mut self, name: &str, value: f64) -> bool {
        for each in &mut self.users {
            if each.name == name {
                if !each.withdrawl(value) {
                    println!("Withdrawl failed! Not enough funds in {name}'s account.");
                    return false;
                }
                println!("Modified Account: {name}");
                println!("This withdrawl: ${:.02}", value);
                return true;
            }
        }
        println!("Error: No user \"{name}\"");
        false
    }

    pub fn display(&self) {
        let percentages = self.get_percentages();
        let total = self.get_total();

        println!("---");
        println!("Account Totals:");
        println!("  Total Value: ${total:.02}");
        println!();
        for (n, each) in self.users.iter().enumerate() {
            println!("Name: {}", each.name);
            println!(
                "  Percent: {:.02}%",
                percentages.get(n).unwrap_or(&0f64) * 100f64
            );
            println!("  Value: ${:.02}", each.value);
        }
        println!("---");
    }

    pub fn rm_user(&mut self, name: &str) -> Option<User> {
        let mut user_index: Option<usize> = None;
        for (n, each) in self.users.iter().enumerate() {
            if each.name == name {
                user_index = Some(n);
                break;
            }
        }
        if let Some(n) = user_index {
            let user = self.users.remove(n);
            Some(user)
        } else {
            None
        }
    }

    pub fn save(&self, filename: &str) -> Result<()> {
        let output = serde_json::to_string(&self)?;
        let mut file = File::create(filename)?;
        file.write_all(output.as_bytes())?;
        println!("Saved: {filename}");
        Ok(())
    }

    pub fn load(filename: &str) -> Result<Self> {
        let mut file = File::open(filename)?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        let acct: Account = serde_json::from_str(&contents)?;
        println!("Loaded: {filename}");
        Ok(acct)
    }
}

#[derive(Clone, Debug, Serialize, Deserialize)]
struct User {
    name: String,
    value: f64,
}

impl User {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            value: 0f64,
        }
    }

    pub fn deposit(&mut self, deposit: f64) {
        self.value += deposit;
    }

    pub fn withdrawl(&mut self, withdrawl: f64) -> bool {
        if self.value > withdrawl {
            self.value -= withdrawl;
            true
        } else {
            false
        }
    }
}

fn main() -> Result<()> {
    let opt = Args::parse();
    let mut ledger = Ledger::load(&opt.file)?;

    match opt.cmd {
        Command::List => {
            let acct = Account::load(&opt.file)?;
            acct.display();
            acct.save(&opt.file)?;
            ledger.log("Accounts listed".to_string())?;
        }
        Command::Deposit(arg) => {
            let mut acct = Account::load(&opt.file)?;
            acct.deposit(&arg.user, arg.value);
            acct.display();
            acct.save(&opt.file)?;
            ledger.log(format!("{} deposited ${:.02}", &arg.user, arg.value))?;
        }
        Command::Withdrawl(arg) => {
            let mut acct = Account::load(&opt.file)?;
            let result = acct.withdrawl(&arg.user, arg.value);
            acct.display();
            acct.save(&opt.file)?;
            if result {
                ledger.log(format!("{} withdrawn ${:.02}", &arg.user, arg.value))?;
            } else {
                ledger.log(format!("{} not withdrawn ${:.02}", &arg.user, arg.value))?;
            }
        }
        Command::Investments(arg) => {
            let mut acct = Account::load(&opt.file)?;
            acct.investments(arg.value);
            acct.display();
            acct.save(&opt.file)?;
            ledger.log(format!("Investments gain/loss ${:.02}", arg.value))?;
        }
        Command::Update(arg) => {
            let mut acct = Account::load(&opt.file)?;
            let total = acct.get_total();
            let value = arg.value - total;
            acct.investments(value);
            acct.display();
            acct.save(&opt.file)?;
            ledger.log(format!("Investments gain/loss ${:.02}", value))?;
        }
        Command::NewUser(arg) => {
            let mut acct = Account::load(&opt.file)?;
            acct.new_user(&arg.user, arg.value);
            acct.display();
            acct.save(&opt.file)?;
            ledger.log(format!("{} deposited ${:.02}", &arg.user, arg.value))?;
        }
        Command::Create(arg) => {
            let mut acct = Account { users: Vec::new() };
            acct.new_user(&arg.user, arg.value);
            acct.display();
            acct.save(&opt.file)?;
            ledger.log(format!(
                "New account created, {} deposited ${:.02}",
                &arg.user, arg.value
            ))?;
        }
        Command::RemoveUser(arg) => {
            let mut acct = Account::load(&opt.file)?;
            let rm_user = acct.rm_user(&arg.user);
            let Some(user) = rm_user else {
                return Ok(());
            };
            acct.display();
            match arg.method {
                Method::Disperse => {
                    acct.investments(user.value);
                }
                Method::Withdrawl => {
                    println!(
                        "User {} removed and ${:.02} value withdrawn",
                        user.name, user.value
                    );
                }
            };
            acct.display();
            acct.save(&opt.file)?;
            match arg.method {
                Method::Disperse => ledger.log(format!(
                    "User {} removed, ${:.02} dispersed",
                    &user.name, user.value
                ))?,
                Method::Withdrawl => ledger.log(format!(
                    "User {} removed, ${:.02} withdrwan",
                    &user.name, user.value
                ))?,
            }
        }
    }

    Ok(())
}
