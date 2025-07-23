  struct BankAccount {
    balance: i32,
    verified: bool,
}

fn print_balance(account: &BankAccount) {
    println!("Balance: {}", account.balance);
}

fn print_verified(account: &BankAccount) {
    println!("Verified: {}", account.verified);
}

fn main() {
    let my_account = BankAccount {
        balance: 1000,
        verified: true,
    };

    print_balance(&my_account);
    print_verified(&my_account);
}