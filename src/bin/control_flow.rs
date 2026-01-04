fn main(){

    /*       if else     */


    let a: i32 = 11;

    if a <=10 {                             // skipped the block since condn is false
        println!("a is <= 10");
    }


    let test_bool = false;

    let output: () = if test_bool { println!("True") } else { println!("False"); };

   // let output_2 = if test_bool { 10 } else { "heyy" };       Rust needs to know definitively at compile time what type the number variable is.
  //  println!("{}", output_2);







    /*        loop, while, for          */

    println!(" ");
    println!(" ");
    println!(" ");

    let mut count:i32 = 0;

    'counting : loop{                                               // naming loop for avoiding ambiguity in breaking loops
        println!("count = {count}");
        let mut remaining:i32 = 10;

        loop{
            println!("remaining = {remaining}");

            if remaining == 9{
                break;
            }
            if count == 2 {
                break 'counting;
            }
            remaining -= 1;
        }
        count += 1;

    }
    println!("end count : {count}")
}