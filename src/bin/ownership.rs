fn main(){

    let s:String = String::from("hello world !!");
    // s.push_str("to everyone");                                // above we haven't declared s to be mutable

    let mut s1: String = String::from("hello ");
    s1.push_str("world !!");

    println!("{s1}");
    assert_eq!(s, s1);


    let literal:&str = "hello world !!";                    // immutable but very fast because hardcoded and the size required is known at compile time


    /*************************************************/

    let mut  Strr: String = String::from("this is a string");
    let reference = find_length(&Strr);
    let modif = change(&mut Strr);
    let modif2 = change1(&mut Strr);   // not allowed

    println!("{:?} {:?}", modif, modif2);

    println!("{reference}");
    println!("{Strr}");

    fn find_length(a: &String) -> usize{
        a.len()
    }

    fn change(c: &mut String){
        c.push_str(" immutable");

    }


    fn change1(d: &mut String) {
        d.push_str(" immutability test");

    }


    let mut p = String::from("distro");
    let q = &mut p;
    let r = &mut p;

   // println!("{q}, {r}")


    let mut s5 = String::from("hello");

    let r1 = &s5; // no problem
    let r2 = &s5; // no problem
    println!("{r1} and {r2}");
    println!("{r1} and {r2}");
    // Variables r1 and r2 will not be used after this point.

    let r3 = &mut s5; // no problem
    println!("{r3}");





}