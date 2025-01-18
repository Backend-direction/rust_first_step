#[derive(Debug)]
struct Account {
  id: u32,
  balance: i32,
  holder: String,
}

impl Account {
    fn new(id: u32, holder: String) -> Self {
      return Account { 
        id,
        holder,
        balance: 0};
    }

    fn summury(&self) -> String {
      format!("{} has a balance {}", self.holder, self.balance) 
    }

    fn deposit(&mut self, amount: i32) -> i32 {
      self.balance += amount;

      return  self.balance; 
    }

    fn withdraw(&mut self, amount: i32) -> i32 {
      self.balance -= amount;

      return  self.balance; 
    }
}

#[derive(Debug)]
struct Bank {
    accounts: Vec<Account>,
}

impl Bank {
    fn new() -> Self {
        return Bank { accounts: vec![] };
    }

    fn add_account(&mut self, account: Account) {
      self.accounts.push(account);
    }

    fn total_balance(&self) -> i32 {
      return self.accounts.iter().map(|account| account.balance).sum();
    }

    fn summary(&self) -> Vec<String> {
      return self.accounts
        .iter()
        .map(|account| account.summury())
        .collect::<Vec<String>>();
    }
}

fn main() {
  let mut bank = Bank::new();
  let mut account = Account::new(1, String::from("me"));

  account.deposit(500);
  account.withdraw(250);

  bank.add_account(account);

  bank.total_balance();
  println!("{}", bank.total_balance());
  println!("{:#?}", bank.summary());

  println!("{:#?}", bank);
}
