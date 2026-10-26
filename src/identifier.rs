//! The identifiers that this crate accepts, which never need escaping in CSS.

const fn is_name_byte(byte: u8) -> bool {
    byte.is_ascii_alphanumeric() || byte == b'-' || byte == b'_'
}

const fn are_name_bytes(bytes: &[u8], start: usize) -> bool {
    let mut index = start;
    while index < bytes.len() {
        if !is_name_byte(bytes[index]) {
            return false;
        }
        index += 1;
    }
    true
}

/// Return whether `identifier` starts with an ASCII letter or `_` and continues with ASCII letters,
/// digits, `-`, or `_`.
pub(crate) const fn is_checked_identifier(identifier: &str) -> bool {
    let bytes = identifier.as_bytes();
    !bytes.is_empty()
        && (bytes[0].is_ascii_alphabetic() || bytes[0] == b'_')
        && are_name_bytes(bytes, 1)
}

/// Return whether `name` is `--` followed by at least one ASCII letter, digit, `-`, or `_`.
///
/// CSS reserves the name `--` alone.
pub(crate) const fn is_checked_custom_property_name(name: &str) -> bool {
    let bytes = name.as_bytes();
    bytes.len() > 2 && bytes[0] == b'-' && bytes[1] == b'-' && are_name_bytes(bytes, 2)
}
