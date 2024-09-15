
fn main(){
    let x: i32 = fib(4);
    println!("{}" , x);
}

fn fib(num: i32) -> i32 {
    let mut first = 0 ;
    let mut second = 1 ;

    for _ in 0..(num-2) {
        let temp = first ;
        first = second ;
        second =  temp + second ;
    }
    return second
}