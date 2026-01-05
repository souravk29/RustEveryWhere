fn main(){
    let length: u32 = 5;
    let width: u32 = 6;

    println!("the area of a rectangle of length {length} and width {width} is : {}", calculate_area(length, width) );


    fn calculate_area(l: u32, w: u32) -> u32 {
        l*w
    }

}