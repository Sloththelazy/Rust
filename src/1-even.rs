fn main() {
    let ans = is_even(10);
    println!("{}" , ans);
}

// code to check if the  function is even or odd

fn is_even (num: i32) -> bool {
    if num % 2 == 0 {
        return true ;
    } else {
        return false ;
    }
}