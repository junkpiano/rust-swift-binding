#[no_mangle]
pub extern fn rust_munchausen_numbers() -> *mut [i32; 4] {
    let cache = [0; 10];
    let mut index = 0;
    let mut munchausen_num: [i32; 4] = [0; 4];
    let munchausen_num_ptr: *mut [i32; 4] = &mut munchausen_num;

    for n in 0..500000000 {
        if is_munchausen_number(n, &cache) {
            munchausen_num[index] = n;
            index += 1;
        }
    }

    munchausen_num_ptr
}

fn is_munchausen_number(number: i32, cache: &[i32; 10]) -> bool {
    let mut current_number = number;
    let mut sum = 0;

    while current_number > 0 {
        let digit = current_number % 10;
        sum += cache[digit as usize];

        if sum > number {
            return false;
        }

        current_number /= 10;
    }

    sum == number
}


