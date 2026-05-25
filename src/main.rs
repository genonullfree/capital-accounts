
#[derive(Debug)]
struct Account {
    total: f64,
    investment: f64,
    users: Vec<User>,
}

impl Account {
    pub fn new_user(&mut self, name: &str, deposit: f64) {
        let mut user = User::new(name, deposit);
        user.deposit(self.total, deposit);
        self.users.push(user);
        self.update();
    }

    pub fn update(&mut self) {
        let mut total = 0f64;
        let mut withdrawl = 0f64;
        for each in &self.users {
            total += each.deposit;
            withdrawl += each.withdrawl;
        }

        for each in &mut self.users {
            let p = (each.deposit - each.withdrawl) / total;
            each.percent = p;
        }

        self.total = total + self.investment - withdrawl;

    }

    pub fn totals(&self) {
        println!("Account Total Balance: ${:.02}", self.total);
        println!("Investment gains/losses: ${:.02}", self.investment);
        println!("---");
        let mut total_withdrawl = 0f64;
        for each in &self.users {
            total_withdrawl += each.withdrawl;
        }

        let mut check = 0f64;
        for each in &self.users {
            let p = each.percent;
            println!("Account: {}", each.name);
            println!("Account Percent: {:.02?}%", p * 100f64);
            println!("Account Balance: ${:.02}", (self.total * p) - each.withdrawl + (total_withdrawl * p));
            check += (self.total * p) - each.withdrawl + (total_withdrawl * p);
            println!();
        }
        println!("Check value: ${check:.02}");
        println!("---");
    }

    pub fn investments(&mut self, value: f64) {
        self.investment += value;
        self.update();
    }

    pub fn deposit(&mut self, name: &str, value: f64) {
        for each in &mut self.users {
            if each.name == name {
                each.deposit += value;
                println!("Modified Account: {name}");
                println!("Total deposit: {:.02}", each.deposit);
                self.update();
                return;
            }
        }
        println!("Error: No user \"{name}\"");
    }

    pub fn withdrawl(&mut self, name: &str, value: f64) {
        for each in &mut self.users {
            if each.name == name {
                if each.withdrawl + value > each.deposit {
                    println!("Withdrawl failed! Not enough funds in {name}'s account.");
                    return;
                }
                each.withdrawl += value;
                println!("Modified Account: {name}");
                println!("Total withdrawl: {:.02}", each.withdrawl);
                self.update();
                return;
            }
        }
        println!("Error: No user \"{name}\"");
    }
}


#[derive(Debug)]
struct User {
    name: String,
    deposit: f64,
    withdrawl: f64,
    percent: f64,
}

impl User {
    pub fn new(name: &str, deposit: f64) -> Self {
        Self{
            name: name.to_string(),
            deposit,
            withdrawl: 0f64,
            percent: 0f64,
        }
    }

    pub fn deposit(&mut self, total: f64, deposit: f64) {
        let value = self.value(total);
        self.percent = (value + deposit) / (total + deposit);
    }

    pub fn withdrawl(&mut self, total: f64, withdrawl: f64) -> bool {
        let value = self.value(total);
        if value > withdrawl {
            self.percent = (value - withdrawl) / (total - withdrawl);
            true
        } else {
            false
        }
    }

    pub fn value(&self, total: f64) -> f64 {
        total * self.percent
    }
}

fn main() {
    let mut acct = Account{ total: 5000f64, investment: 0f64, users: Vec::new()};
    
    acct.new_user("geno", 52000f64);
    acct.totals();
    acct.new_user("Micah", 1300f64);
    acct.totals();
    acct.new_user("Killian", 1000f64);
    acct.totals();
    acct.new_user("Ashton", 500f64);
    acct.totals();
    acct.new_user("Areli", 300f64);
    acct.totals();

    /*
    let users = vec![user0, user1, user2, user3, user4];
    acct.users = users;
    acct.update();
    */

    acct.investments(10000.00);
    acct.totals();
    acct.investments(-3000.00);
    acct.totals();
    acct.deposit("Areli", 1000.00);
    acct.totals();
    acct.withdrawl("Micah", 1000.00);
    acct.totals();
}
