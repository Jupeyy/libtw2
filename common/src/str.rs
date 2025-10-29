use arrayvec::ArrayString;

pub fn truncated_arraystring<const A: usize>(mut s: &str) -> ArrayString<A> {
    let mut result = ArrayString::new();
    if s.len() > result.capacity() {
        for n in (0..result.capacity() + 1).rev() {
            if s.is_char_boundary(n) {
                s = &s[..n];
                break;
            }
        }
    }
    result.push_str(s);
    result
}

#[cfg(test)]
mod test {
    use super::truncated_arraystring;
    use quickcheck::quickcheck;
    quickcheck! {
        fn truncated_arraystring0(v: String) -> bool {
            truncated_arraystring::<0>(&v);
            true
        }
        fn truncated_arraystring1(v: String) -> bool {
            truncated_arraystring::<1>(&v);
            true
        }
        fn truncated_arraystring2(v: String) -> bool {
            truncated_arraystring::<2>(&v);
            true
        }
        fn truncated_arraystring4(v: String) -> bool {
            truncated_arraystring::<4>(&v);
            true
        }
        fn truncated_arraystring8(v: String) -> bool {
            truncated_arraystring::<8>(&v);
            true
        }
        fn truncated_arraystring16(v: String) -> bool {
            truncated_arraystring::<16>(&v);
            true
        }
    }
}
