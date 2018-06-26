pub fn is_leap_year(year: i32) -> bool {
    if is_mod_100_and_not_mod_400(year) {
        return false
    }

    if is_not_mod_4(year) {
        return false
    }

    true
}

fn is_mod_100_and_not_mod_400(num: i32) -> bool {
    num % 100 == 0 && num % 400 != 0
}

fn is_not_mod_4(num: i32) -> bool {
    num % 4 != 0
}
