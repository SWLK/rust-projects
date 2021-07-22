enum IpAddrKind {
    V4,
    V6,
}

struct IpAddr {
    kind: IpAddrKind,
    address: String,
}

fn main() {
    // We can create instances of each of the two variants of IpAddrKind
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    // Using a struct to bundle the `kind` and `address` values together
    let home = IpAddr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1"),
    };

    let loopback = IpAddr {
        kind: IpAddrKind::V6,
        address: String::from("::1"),
    };
}
