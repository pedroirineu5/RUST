fn main(){

    let v1: u8 = 251_u16 + 8+u8;
    let v2 = i8 ::checked_add(251, 8).unwrap();// uma forma mais segura

    println!("{},{}",v1,v2);
}