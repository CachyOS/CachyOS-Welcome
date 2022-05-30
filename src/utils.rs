use std::fs::File;
use std::{fs, slice, str};

#[inline]
pub fn fix_path(path: &str) -> String {
    if !path.starts_with('~') {
        return String::from(path);
    }
    path.replace('~', glib::home_dir().as_path().to_str().unwrap())
}

#[inline]
pub fn read_json(path: &str) -> serde_json::Value {
    let buf = fix_path(path);
    let data = fs::read_to_string(buf).expect("Unable to read file");
    serde_json::from_str(&data).expect("Unable to parse")
}

#[inline]
pub fn write_json(path: &str, content: &serde_json::Value) {
    let output = File::create(fix_path(path)).expect("Unable to open file for writing");
    serde_json::to_writer(output, content).expect("Unable to write json to file");
}

#[inline]
pub const fn const_min(v1: usize, v2: usize) -> usize {
    if v1 <= v2 {
        v1
    } else {
        v2
    }
}

#[inline]
pub const fn string_substr(src_str: &str, pos: usize, n: usize) -> Result<&str, str::Utf8Error> {
    let rlen = const_min(n, src_str.len() - pos);
    let s = unsafe {
        // First, we build a &[u8]...
        let slice = slice::from_raw_parts(src_str.as_ptr().add(pos), rlen);

        // ... and then convert that slice into a string slice
        str::from_utf8(slice)
    };
    s
}

#[inline]
pub fn check_regular_file(path: &str) -> bool {
    let metadata = fs::metadata(path);
    if let Ok(meta) = metadata {
        meta.file_type().is_file()
    } else {
        false
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn check_file() {
        assert_eq!(check_regular_file("/etc/fstab"), true);
        assert_eq!(check_regular_file("/etc"), false);
    }

    #[test]
    fn check_substr() {
        let text = "The test string is here";
        assert_eq!(string_substr(text, 0, 3).unwrap(), "The");
        assert_eq!(string_substr(text, 4, 4).unwrap(), "test");
        assert_eq!(string_substr(text, 19, 4).unwrap(), "here");
    }

    #[test]
    fn check_min() {
        assert_eq!(const_min(1, 2), 1);
        assert_eq!(const_min(2, 2), 2);
        assert_eq!(const_min(3, 2), 2);
    }
}
