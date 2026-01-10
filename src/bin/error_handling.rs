use std::fs::File;
use std::io::ErrorKind;

fn main(){

    /*         UNRECOVERABLE ERRORS                     */

    let v = vec![1,2,3,4];
    //let value_at_index = &v[100];
    // print!("{value_at_index}")


    /*           RECOVERABLE ERRORS     1                 */

    let to_open_file = File::open("Hello.txt");

    let custom_result = match to_open_file {
        Ok(file) => file,
        Err(error) => panic!("The program panicked due to: {error:?}"),
    };


    /*           RECOVERABLE ERRORS     2                 */

    let open_file = File::open("World.txt");

    let handling_success_or_error = match open_file {
        Ok( req_file) => req_file,
        Err(error) => match error.kind(){
            ErrorKind::NotFound => match File::create("World.txt"){
                Ok(rf) => rf ,
                Err(e) => panic!("couldn't create file due to : {e:?}"),
            }
            _ => panic!("Some other error occured"),
        },

    };




}