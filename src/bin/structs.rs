#[derive(Debug)]
struct student{
    name: String,
    enroll: i32,
    address: String,
    weight: f64
}

fn main(){

    let mut student_data = student{
        name: String::from("ashish"),
        enroll: 2938417,
        address: String::from("Delhi"),
        weight: 69.75
    };
    student_data.address =  String::from("Noida");          // updated

    println!("{:?}", student_data);

    let student_data_2 = student{       // Note that the struct update syntax uses = like an assignment; this is because it moves the data
        weight: 65.5,
        ..student_data                                      // update syntax

    };
    println!("{:?}", student_data_2);

    let student_data_3 = student{
        name: student_data_2.name,
        enroll: 2938417,
        address: student_data_2.address,
        weight: 6.75

    };
    println!("{:?}", student_data_3);

/*

   moving case was only for the string, that made whole student_data moved, if it was integer or float
                    type then it would still be valid because they implement copy trait

*/

}