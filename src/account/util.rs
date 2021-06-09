pub fn color_int_to_hex(color: u32) -> String {
    let res = format!("{:x}", color + 0x1000000);
    format!("#{}", &res.split_at(1).1)
}
