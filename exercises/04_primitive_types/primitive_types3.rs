fn main() {
    // TODO: Create an array called `a` with at least 100 elements in it.
    let mut a = [0; 101];
    for (i, elem) in a.iter_mut().enumerate() {
        *elem = i as i32;
    }

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
        panic!("Array not big enough, more elements needed");
    }
}
