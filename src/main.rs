use anyhow::Result;
use serde::{Deserialize, Serialize};

const TEST_LOAD: &str = r#"{"users":[{"name":"geno","value":58606.17059891107},{"name":"Micah","value":465.1542649727769},{"name":"Killian","value":1127.0417422867513},{"name":"Ashton","value":563.5208711433756},{"name":"Areli","value":1338.1125226860254}]}"#;

#[derive(Debug, Serialize, Deserialize)]
struct Account {
    users: Vec<User>,
}

impl Account {
    pub fn new_user(&mut self, name: &str, deposit: f64) {
        // Attempt to deposit first to make sure there isn't already that account
        if self.deposit(name, deposit) {
            // Success, this user already exists
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

    pub fn investments(&mut self, value: f64) {
        let mut percentages = self.get_percentages();
        println!("Investments: ${:.02}", value);
        for each in &mut self.users {
            each.value += percentages.remove(0) * value;
        }
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

    pub fn save(&self) -> Result<()> {
        let output = serde_json::to_string(&self)?;
        println!("Saved: {output}");
        Ok(())
    }

    pub fn load() -> Result<Self> {
        let acct: Account = serde_json::from_str(TEST_LOAD)?;
        Ok(acct)
    }
}

#[derive(Debug, Serialize, Deserialize)]
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
    //let mut acct = Account { users: Vec::new() };
    let mut acct = Account::load()?;

    acct.new_user("geno", 52000f64);
    acct.display();
    acct.new_user("Micah", 1300f64);
    acct.display();
    acct.new_user("Killian", 1000f64);
    acct.display();
    acct.new_user("Ashton", 500f64);
    acct.display();
    acct.new_user("Areli", 300f64);
    acct.display();

    /*
    let users = vec![user0, user1, user2, user3, user4];
    acct.users = users;
    */

    acct.investments(10000.00);
    acct.display();
    acct.investments(-3000.00);
    acct.display();
    acct.deposit("Areli", 1000.00);
    acct.display();
    acct.withdrawl("Micah", 1000.00);
    acct.display();

    acct.save();

    Ok(())
}
