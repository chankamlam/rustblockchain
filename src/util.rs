pub fn vec2hex(v : &Vec<u8>) -> String{
    v.iter()
        .map(|b| format!("{:02x?}", b))
        .collect::<String>()
}