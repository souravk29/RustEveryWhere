
#[derive(Debug)]
struct dim_of_rec{
    leng: u32,
    breadth: u32,
}

struct UnitStruct;                              // unit type


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

    /****************************************************************/

    let values_of_dim = dim_of_rec{
        leng: 10,
        breadth: 5,
    };

    println!("{values_of_dim:?}");

    println!("the area of a rectangle of length {length} and width {width} is : {}", calculate_area_by_value_of_dim(&values_of_dim) );

    fn calculate_area_by_value_of_dim( D: &dim_of_rec ) -> u32{
        D.leng * D.breadth
    }
}

/********************************   METHODS: TODO     ******************************/