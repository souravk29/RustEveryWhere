
/*

                                "if let" is preferred to be used when we don't want to get return value of all the
                      available patterns, and want to deal with some of them. its efficient and time saving than using "match"
                                            which needs every available patterns to be defined

 */
fn main() {

    let a = Some(10);
    if let Some(g) = a{
        println!("{g}");
    } else {
        println!("thats fine as well");
    }




}
