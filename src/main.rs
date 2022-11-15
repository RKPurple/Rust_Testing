use rand::Rng;
fn make_ext() -> String {
    let mut str = String::new();
    for _ in 0..5 {
        let random_letter: u8 = rand::thread_rng().gen_range(97..123); // generate a 8bit unsigned int that corresponds to a-z in ascii
        let random_number: u8 = rand::thread_rng().gen_range(48..58); // generate a 8bit unsigned int that corresponds to 0-9 in ascii
        let n = rand::thread_rng().gen_range(0..2); // pick 1 or 0
        if n == 0 {
            str.push(random_letter as char); // convert the ascii value to a character and add to the string
        } else {
            str.push(random_number as char); // convert the ascii value to a character and add to the string
        }
    }
    return str;
}

fn main() {
    let mut path = String::from("https://i.imgur.com/");
    path.push_str(&make_ext());
    open::that(&path).unwrap();
}
