fn main() {

    let s:String = String::from("this is the slice");

    let reqSlice = extractSlice(&s);
    println!("{reqSlice}");

    fn extractSlice(p: &String) -> &str {
        let pieceOfSlice = &p[0..=3];
        pieceOfSlice
    }

    let s1: String = String::from("other slice");

    let lengthOfString = s1.len();
    let sliceOfs1 = &s1[..lengthOfString];
    println!("{}",sliceOfs1);


    let firstWord1 = first_word(&s);
    println!("{firstWord1}");

    fn first_word(l: &String) -> &str {

        let k = l.as_bytes();

        for (i, &item) in k.iter().enumerate(){
            if item ==  b' ' {
                return &l[0..i];
            }
        }
        &l[..]

    }


}