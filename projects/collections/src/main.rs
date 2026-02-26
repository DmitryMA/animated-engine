fn main() {
    let mut v = vec![1, 2, 3];
    v.push(12);
    v.push(22);
    v.push(42);

    println!("{:#?}", v);

    // let item_1 = &v[v.len()];
    let idx = v.len();
    let item_2 = v.get(idx);
    match item_2 {
        Some(value) => println!("{value}"),
        None => println!("There is no element with such Index: {idx}"),
    }

    // println!("1: {}", item_1);
    // println!("2: {:#?}", item_2);

    for item in &mut v {
        *item += 20;
        println!("{item}");
    }
}
