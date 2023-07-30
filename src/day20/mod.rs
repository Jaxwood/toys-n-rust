#![allow(dead_code)]

fn day20a(_path: &str) -> i32 {
    0
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn find_groove_coordinates() {
        let actual = day20a("./data/day20.txt");
        assert_eq!(actual, 3);
    }
}
