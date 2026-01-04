fn main(){

    let s:String = String::from("hello world !!");
    // s.push_str("to everyone");                                // above we haven't declared s to be mutable

    let mut s1: String = String::from("hello ");
    s1.push_str("world !!");

    println!("{s1}");
    assert_eq!(s, s1);


    let literal:&str = "hello world !!";                    // immutable but very fast because hardcoded and the size required is known at compile time

    let Strr: String = String::from("this is a string");
    let reference = find_length(&Strr);

    println!("{reference}");

    fn find_length(a: &String) -> usize{
        a.len()
    }



}