


enum MultiType {
    EnumKind1(i32, String, u64, f64, i32),
    EnumKind2(String, i32),
    Message {a: i32, b: String},
}
enum IPaddressKind {
    Ipv4,
    Ipv6,
}

struct IPaddr{
    kind: IPaddressKind,
    address: String,
}


enum IPaddressKind1 {
    Ipv4(String),
    Ipv6(String),
}


fn main() {

    let home2 = MultiType::EnumKind1(10, String::from("glamoss !!"), 5, 10.65, 30);
    let work2 = MultiType::EnumKind2(String::from("parrr !!"), 1000);
    let msg = MultiType::Message { a: 10 , b: String::from("is it ok ??")};


    let home1 = IPaddressKind1::Ipv4(String::from("192.55.9.0"));
    let work1 = IPaddressKind1::Ipv6(String::from("171.9.22.10"));




    let home = IPaddr{
        kind: IPaddressKind::Ipv4,
        address: String::from("192.110.9.2"),
    };

    let work = IPaddr{
        kind: IPaddressKind::Ipv6,
        address: String::from("172.8.16.0"),
    };




    let four = IPaddressKind::Ipv4;
    let six = IPaddressKind::Ipv6;
    net_type(IPaddressKind::Ipv4);


}

fn net_type( ip: IPaddressKind ){



}