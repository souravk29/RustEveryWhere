use std::collections::HashMap;

fn main(){

    let mut map = HashMap::new();

    map.insert("india", None );
    map.insert("england", Some(200));

    let find_from_key: &str = "india";
    let value_for_key = map.get(&find_from_key); //copied().unwrap_or(0);

    println!("{value_for_key:?}");


    let value_for_first = map.keys();
    let values = map.values();

    println!("{value_for_first:?}");
    println!("{values:?}");

    let field_name = String::from("Favorite color");
    let field_value = String::from("Blue");

    let mut map = HashMap::new();
    map.insert(&field_name, &field_value);

    for (key, value ) in map {

        println!("{key}: {value}");


    }
    println!("{field_name:?}");

    // field_name and field_value are invalid at this point, try using them and
    // see what compiler error you get!




}