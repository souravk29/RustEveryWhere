
enum Option<T>{
    Some(T),
    None,
}


fn main(){

    let var1 = Option::Some(22);
    let absent_number: Option<i32> = Option::None;

}