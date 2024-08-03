use ndarray::Array2;

pub fn make_byte_grid(raw_inp: &str) -> Array2<u8> {
    let columns = raw_inp
        .trim()
        .bytes()
        .position(|c| c == b'\n')
        .expect("can't get column count");

    Array2::from_shape_vec(
        ((raw_inp.trim().len() + 1) / (columns + 1), columns),
        raw_inp.bytes().filter(|&x| x != b'\n').collect(),
    )
    .expect("can't make array")
}

pub fn make_bool_grid<const TRUE_CHAR: u8>(raw_inp: &str) -> Array2<bool> {
    let columns = raw_inp
        .trim()
        .bytes()
        .position(|c| c == b'\n')
        .expect("can't get column count");

    Array2::from_shape_vec(
        ((raw_inp.trim().len() + 1) / (columns + 1), columns),
        raw_inp
            .bytes()
            .filter(|&x| x != b'\n')
            .map(|b| b == TRUE_CHAR)
            .collect(),
    )
    .expect("can't make array")
}
