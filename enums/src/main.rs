#[derive(Debug)]
enum IpAddrKind {
    V4(u8, u8, u8, u8),
    V6(String)
}

impl IpAddrKind { // enums também podem ter métodos
    fn call(&self) {
        println!("{:?}", &self);
    }
}

struct IpAddr {
    kind: IpAddrKind,
    address: String
}

fn main() {

    let four = IpAddrKind::V4(127, 0, 0, 1); // assim vc instancia/pega o valor

    let home = IpAddr {
        kind: four,
        address: String::from("127.0.0.1")
    };

    let loopback = IpAddrKind::V6(String::from("::1")); //  aqui a gente coloca um valor no próprio enum

    loopback.call();
}
