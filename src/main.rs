fn main() {
    let mut result1:u8 = 0b0;
    let mut result2:u8 = 0b0;
    let mut count = 0;

    loop {
        let num = magic_num::magic_number();
        count += 1;

        result1 = result1 << 1;
        result1 |= num;

        if count == 4 || count == 5 {
            result2 = result2 << 1;
            continue;
        }

        result2 = result2 << 1;
        result2 |= num;
        
        if count == 8 {
            break;
        }
    }

    println!("{} , {}", result1, result2);
}