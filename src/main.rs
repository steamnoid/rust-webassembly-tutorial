fn main() {
    let items = [1, 2, 3, 4, 5];
    println!("Items: {:?}", items);

    let vector_items = vec![1, 2, 3, 4, 5];
    println!("Vector Items: {:?}", vector_items);

    let mut vector_items_2 = Vec::new();
    vector_items_2.push(1);
    vector_items_2.push(2);
    vector_items_2.push(3);
    vector_items_2.push(4);
    vector_items_2.push(5);
    println!("Vector Items 2: {:?}", vector_items_2);
}