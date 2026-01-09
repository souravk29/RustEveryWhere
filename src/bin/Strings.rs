fn main() {

    let s = "hello world !!";
    let k =     s.to_string();
    d(k);
    fn d (a: String) -> String{
        a
    }


    let s1 = String::from("this is the begin");
    let s2 = String::from("ning");
    let s3 = String::from(" of main");

    let s4 = String::from("Здравствуйте");

    for c in s4.chars(){
        println!("{c}");
    }

    println!("");

    for b in s4.bytes(){
        print!("{b} ");
    }

    println!("");

    for b in s1.bytes(){
        print!("{b} ");
    }

    let output_after_format = format!("{s1}{s2}{s3}");

   // print!("{output_after_format}");

    let l = s4.len();
  //  println!("{l}");

}