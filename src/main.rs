  struct BankAccount {
    balance: i32,
    verified: bool,
}

fn main() {
    let my_account = BankAccount {
        balance: 1000,
        verified: true,
    };
    println!("Account Balance: {}", my_account.balance);
    println!("Account Verified: {}", my_account.verified);
}