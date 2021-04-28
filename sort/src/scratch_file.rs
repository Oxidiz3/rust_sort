pub fn scratch(){
    let original:[u8; 8] = [1,2,3,4,5,6,7,8];
    let mut len:usize = 0;

    len = original.len();

    for number in &original[2..len]{
        println!("Number is {}", number);
    }
}