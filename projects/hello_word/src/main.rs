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

    fn get_five() -> u8 {
        22;
        255;
        33
    }

    println!("{}", get_five());

    let x: i32 = 5;

    if x == 3 {
        println!("{x} ==> 3");
    } else {
        println!("{x} ==> 5");
    }

    let nums: [u8; 4] = [10, 20, 30, 40];

    let mut idx: usize = 0;

    while idx < nums.len() {
        println!("{}", nums[idx]);
        idx += 1;
    }

    // for num in nums {
    //     println!("{}", num * 10);
    // }

    for num in 1..4 {
        println!("last: {}", num);
    }

    let s1 = "Hello";
    let s2 = s1;
    let is_equal = s1 == s2;
    println!("{s2}{s1}{is_equal}");

    let s3 = String::from("Hello World!");
    let s4 = s3.clone();

    let is_equal = s3 == s4;

    println!("{is_equal}");
}
