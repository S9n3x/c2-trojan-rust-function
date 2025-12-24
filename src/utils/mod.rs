use encoding_rs::{GBK, UTF_8};

pub fn auto_decode(bytes: &[u8]) -> String {
    if let Ok(s) = std::str::from_utf8(bytes) {
        return s.to_string();
    }

    let (utf8, _, _) = UTF_8.decode(bytes);
    let utf8_bad = utf8.matches('�').count();

    let (gbk, _, _) = GBK.decode(bytes);
    let gbk_bad = gbk.matches('�').count();

    if gbk_bad < utf8_bad {
        gbk.into_owned()
    } else {
        utf8.into_owned()
    }
}

