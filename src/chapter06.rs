enum IpAddrKind {
    V4(u8, u8, u8, u8), 
    V6(String),
}

fn main() {
    let ipversion = IpAddrKind::V4;
    let ipversion6 = IpAddrKind::V6;

}