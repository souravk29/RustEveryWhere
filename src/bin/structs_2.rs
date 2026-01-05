fn main(){
    let length: u32 = 5;
    let width: u32 = 6;

    println!("the area of a rectangle of length {length} and width {width} is : {}", calculate_area(length, width) );


    fn calculate_area(l: u32, w: u32) -> u32 {
        l*w
    }

    /****************************************************************/

    let dimension = (7,5);

    println!("the area of a rectangle of length {length} and width {width} is : {}", calculate_area_by_dim(dimension) );

    fn calculate_area_by_dim( d: (u32, u32) ) -> u32{
        d.0 * d.1
    }




}