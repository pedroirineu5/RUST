fn main(){

    let v1: u16 = 251_u16 + 8;
    let v2: i16 = i16::checked_add(251, 8).unwrap();// uma forma mais segura

    println!("{},{}",v1,v2);
} 