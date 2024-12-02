pub const fn count_lines(text: &str) -> usize {
    let len = text.len();
    if len == 0 {
        return 0;
    }

    let bytes = text.as_bytes();
    let mut count = 1;
    let mut i = 0;
    let mut i_last_nl = 0;
    while i < len {
        let b = bytes[i];
        if b == b'\n' {
            count += 1;
            i_last_nl = i;
        }
        i += 1;
    }

    // Account for an optional trailing newline
    if i_last_nl == len - 1 {
        count -= 1;
    }

    count
}