// GC
// bank account

fn main() {
    let mut acc: BankAccount = BankAccount::new(String::from("abcd1234"));

    acc.deposit(100.);
    acc.withdraw(50.);
    println! {"{}", acc.get_balance()};
    assert_eq!(acc.get_balance(), 50. );

    println!{"{:?}", acc};

    // map transactions to dollar values
    let dollars_tranasctions: Vec<f64> = acc.transactions.iter().map(| euro | euro * 1.1).collect();
    println! {"dollars {:?}", dollars_tranasctions};
}

#[derive(Debug)]
struct BankAccount {
    iban: String,
    transactions: Vec<f64>,                         // euro
}

impl BankAccount {
    fn new(iban: String) -> BankAccount {
        BankAccount {
            iban: iban,
            transactions: vec![]
        }
    }

    // get the balance
    fn get_balance(&self) -> f64 {
        self.transactions.iter().sum()
    }

    // deposit some money
    fn deposit(&mut self, amt: f64) {
        self.transactions.push(amt);
    }

    // withdraw	some money
    fn withdraw(&mut self, amt: f64) {
        self.transactions.push(amt * -1.);
    }
}
