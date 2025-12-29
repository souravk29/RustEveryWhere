

fn sendHere(s: &String){
    println!("{} added to the 2nd function", s);
}

fn main(){

    let a = String::from("this is to be");
    let b = sendHere(&a);

    // print!("{}", b)

}