fn main() {
    // TODO: Create an array called `a` with at least 100 elements in it.
    // let a = ???

    let a: [i32; 100] = [0; 100];
    
    for _i in 0..100 {
        
    }

    if a.len() >= 100 {
        println!("Wow, that's a big array!");
        println!("Array is {a:#?}");
    } else {
        println!("Meh, I eat arrays like that for breakfast.");
        panic!("Array not big enough, more elements needed");
    }
}
