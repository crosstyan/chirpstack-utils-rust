use rand::Rng;

fn get_rand_half_octet() -> u8 {
    let mut rng = rand::thread_rng();
    let mut r = rng.gen::<u8>();
    r = r & 0x0F;
    r
}

/// Generate a random octet letter from 0x00 to 0x0F (0 to F)
/// The captial letter is used default
fn get_rand_half_octet_hex() -> String {
    let r = get_rand_half_octet();
    format!("{:X}", r)
}

pub fn get_rand_hex_str(bits: u8) -> String {
    let mut r = String::new();
    let half_octet = bits / 4;
    for _ in 0..half_octet {
        r.push_str(&get_rand_half_octet_hex());
    }
    r
}

pub fn get_rand_dev_eui() -> String {
    get_rand_hex_str(64)
}

pub fn get_rand_app_key() -> String {
    get_rand_hex_str(128)
}

fn allow_char(c: char) -> bool {
    match c {
        '0'..='9' | 'a'..='f' | 'A'..='F' => return true,
        _ => return false,
    };
}

pub fn is_hex<T>(str:&T) -> bool
where
    T: AsRef<str> + ?Sized,
{
    for char in str.as_ref().chars() {
        if !allow_char(char) {
            return false;
        }
    }
    return true;
}

/// Check the string is a valid hex string.
fn verify_hex_str<T>(str: &T, bits: u8) -> bool
where
    T: AsRef<str> + ?Sized,
{
    for char in str.as_ref().chars() {
        if !allow_char(char) {
            return false;
        }
    }
    let len = usize::from(bits / 4);
    if str.as_ref().len() != len {
        return false;
    }
    return true;
}

/// Check the string is a valid hex AppKey
pub fn verify_app_key<T>(str: &T) -> bool
where
    T: AsRef<str> + ?Sized,
{
    verify_hex_str(str, 128)
}

/// Check the string is a valid hex DevEUI
pub fn verify_dev_eui<T>(str: &T) -> bool
where
    T: AsRef<str> + ?Sized,
{
    verify_hex_str(str, 64)
}
