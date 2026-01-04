fn main(){
    let mut num: i32 = 10;

    while num != 0 {
        println!("{}", num);
        num -= 1;
    }

    println!(" ");
    println!(" ");
    println!(" ");

    let array:[i32; 5] = [1,2,3,4,5];

    let mut i = 0;
    while i<=4 {
        println!("{}", array[i]);
        i+=1;
    }


    println!(" ");  println!(" ");  println!(" ");

    for element in array{
        println!("{element}");
    }
    println!(" "); println!(" "); println!(" ");

    for element in (0..=4){
        println!("{}" , array[element]);
    }



}