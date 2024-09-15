fn main (){
    let my_string = String::from("Hello, There!");
    let length = get_string_length_chars(&my_string);
    println!("The length of the string is : {}" , length) ;
}

fn get_string_length_chars(s: &str) -> usize {
    s.chars().count()
}