use std::char;
use std::borrow::Cow;
use std::string::String;

const PERSIAN_TO_ENGLISH_DIFF: u32 = '۰' as u32 - '0' as u32;

pub fn persian_to_english_digits<'a>(s: &'a str) -> Cow<'a, str> {
    let mut buf = String::with_capacity(s.len());

    for c in s.chars() {
        let c_ = match c {
            '۰' ... '۹' => char::from_u32(c as u32 - PERSIAN_TO_ENGLISH_DIFF).unwrap(),
            _ => c,
        };

        buf.push(c_)
    }

    return buf.into();
}

pub fn english_to_persian_digits<'a>(s: &'a str) -> Cow<'a, str> {
    let mut buf = String::with_capacity(s.len());

    for c in s.chars() {
        let c_ = match c {
            '0' ... '9' => char::from_u32(c as u32 + PERSIAN_TO_ENGLISH_DIFF).unwrap(),
            _ => c,
        };

        buf.push(c_)
    }

    return buf.into();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn persian_to_english_digits_test() {
        assert_eq!(persian_to_english_digits("۰۱۲۳۴۵۶۷۸۹"), "0123456789");
        assert_eq!(persian_to_english_digits("0123456789"), "0123456789");
        assert_eq!(persian_to_english_digits("0۱2۳4۵6۷8۹"), "0123456789");
        assert_eq!(persian_to_english_digits("۰1۲3۴5۶7۸9"), "0123456789");
        assert_eq!(persian_to_english_digits("hello ۰۰۱"), "hello 001");
        assert_eq!(persian_to_english_digits("۰۰۰۰"), "0000");
        assert_eq!(persian_to_english_digits("۰۰۰۰"), "0000");
    }

    #[test]
    fn persian_to_english_digits_arabic_test() {
        assert_eq!(persian_to_english_digits("٠١٢٣٤٥٦٧٨٩"), "٠١٢٣٤٥٦٧٨٩");
        assert_eq!(persian_to_english_digits("۰۱۲۳۴۵۶۷۸۹ ٠١٢٣٤٥٦٧٨٩"), "0123456789 ٠١٢٣٤٥٦٧٨٩");
    }

    #[test]
    fn english_to_persian_digits_test() {
        assert_eq!(english_to_persian_digits("0123456789"), "۰۱۲۳۴۵۶۷۸۹");
        assert_eq!(english_to_persian_digits("۰۱۲۳۴۵۶۷۸۹"), "۰۱۲۳۴۵۶۷۸۹");
        assert_eq!(english_to_persian_digits("0۱2۳4۵6۷8۹"), "۰۱۲۳۴۵۶۷۸۹");
        assert_eq!(english_to_persian_digits("۰1۲3۴5۶7۸9"), "۰۱۲۳۴۵۶۷۸۹");
        assert_eq!(english_to_persian_digits("hello 001"), "hello ۰۰۱");
        assert_eq!(english_to_persian_digits("0000"), "۰۰۰۰");
        assert_eq!(english_to_persian_digits("0000"), "۰۰۰۰");
    }
}

