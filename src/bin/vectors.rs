fn main() {


    let mut v = vec![1,2,3];

    v.push(4);
    v.push(5);

    let access_specific_vector = &v[2];
    println!("{access_specific_vector}");

    let access_by_get = v.get(1);
    println!("{:?}", access_by_get);

    match access_by_get {
        Some(T) => Some(T),
         None => None,
    };


    let mut v1 = vec![1,2,3,4,5];
    let twist = &v[1];

   



}