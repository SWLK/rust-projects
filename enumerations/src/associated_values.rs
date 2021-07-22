// We can put data directly into each enum variant.
// Both V4 and V6 variants have associated `String` values.
enum IpAddr_v1 {
    V4(String),
    V6(String),
}

// If we wanted store V4 addresses as four u8 values,
// but still express V6 addresses as one `String` value,
// we wouldn't be able to do it with `struct`.
enum IpAddr_v2 {
    V4(u8, u8, u8, u8),
    V6(String),
}

fn main() {
    let home = IpAddr_v1::V4(String::from("127.0.0.1"));

    let loopback = IpAddr_v1::V6(String::from("::1"));

    let home = IpAddr_v2::V4(127, 0, 0, 1);

    let loopback = IpAddr::V6(String::from("::1"));
}
