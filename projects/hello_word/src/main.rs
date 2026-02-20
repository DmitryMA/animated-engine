fn main() {
    println!("Hello World Next Level");

    let tuple: (u8, f32, bool) = (1, 1.1, true);
    println!("{1} {2} {0}", tuple.0, tuple.1, tuple.2);

    let nums: [u8; 5] = [1, 2, 3, 4, 5];
    println!("{:?}", nums);
    println!("{:#?}", nums);

    let nums = [0; 5];

    println!("{}", nums[1]);

    // let x = y = 6;

    // println!("{x}");

    fn getFive() -> u8 {
        22;
        255;
        33
    }

    println!("{}", getFive());

    let x = 5;
}
