fn add(num_one: i32, num_two: i32) -> i32 {
    num_one + num_two 
}

fn main() {
    let total = add(10, 15);

    if total > 50 {
        println!("You are qualified for the bonus!");
    } else if total > 20 {
        println!("You are qualified for the promotion!");
    } else {
        println!("You are not qualified for any bonus or promotion.");
    }

}
