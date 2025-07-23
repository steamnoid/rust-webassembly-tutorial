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

fn is_verified(account: &BankAccount) -> Result<bool, bool> {
   match account.verified {
        true => Ok(true),
        false => Err(false),
    }
}
  
fn main() {
    let my_account = BankAccount {
        balance: 1000,
        verified: true,
    };

    let verification_status = is_verified(&my_account).expect("Account verification check failed");

    print_balance(&my_account);
    print_verified(&my_account);
    println!("Verification Status: {:?}", verification_status);
}