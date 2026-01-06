

fn to_check_match_with_option( opt: Option<i32> ) -> Option<i32>{
    match opt {
        Some(T) => {
            Some(T +1);
            println!("value of T is: {:?}", T);
            Some(T +1)
        },
        None => None,
    }
}



fn dice_roll( dice_output: i32 ) -> i32 {
    match dice_output{
        1 => 1,
        2 => 2,
        3 => 3,
        4 => dice_output,
        other => reroll(other),
    }
}

fn reroll( re : i32) -> i32 {
    5
}

fn main() {
    let value_1 = to_check_match_with_option( Option::Some(17) );
    let value_2 = Some(10) ;                                                        // Some matches the pattern

    to_check_match_with_option(value_2);

    let i_got = dice_roll(6);
    let i_got1 = dice_roll(8);
    let i_got2 = dice_roll(9);
    let i_got3 = dice_roll(4);

    println!("{i_got}");
    println!("{i_got1}");
    println!("{i_got2}");

}