use color_eyre::Result;

pub fn add_with_res(left: u64, right: u64) -> Result<u64> {
    // if I start typing `left.bl` here, I get suggestions to use `black()`
    // which comes from `owo_colors::OwoColorize`
    Ok(left + right)
}

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
