fn main() {
    let input = "stringadldskds";
    let mut array = to_array::<14>(input);
    array.sort();
    for (i, v) in array.iter().enumerate() {
        if array[i] == array[i + 1] {
            println!(
                "duplicated char: `{}` at index: {} and index: {}",
                v,
                i,
                i + 1
            );
            break;
        }
    }
}

fn to_array<const N: usize>(s: &str) -> [char; N] {
    let mut chars = s.chars();
    [(); N].map(|_| chars.next().unwrap())
}
