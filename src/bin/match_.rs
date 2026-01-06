enum Direction{
    North,
    East,
    South,
    West,
}

fn for_matching(direction: Direction) -> i32{
    match direction {
        Direction::North => {
            println!("This is North");
            1
        },
        Direction::East => 2,
        Direction::South => 3,
        Direction::West => 4,                                       // no error bacause all from Direction has been defined for matching here
    }
}

fn for_matching_2( match_this: u8 ) -> u8 {
    match match_this {
        1 => 5,
        2 => 10,
        3 => 15,
        _ => 0,                                                             // "_" = 4..=u8::MAX Since all possible u8 type values must be handled
    }
}

fn main(){

    let key = 10;
    match key{
        10 => fun1(),
        12 => fun2(),
        _ => (),                                // to not return anything
    }

    fn fun1(){}
    fn fun2(){}

    let output1: i32 = for_matching(Direction::East);
    let output2: i32 = for_matching(Direction::West);
    let output3: i32 = for_matching(Direction::North);
    println!("{output1}");
    println!("{output2}");
    println!("{output3}");

    let output3 = for_matching_2(2);

    println!("");
    println!("{output3}");






}