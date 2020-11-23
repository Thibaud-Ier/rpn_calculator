#[cfg(test)]
mod tests {
    #[test]
    fn sample() {
        assert_eq!(crate::engine::calcul_rpn("20 5 /"), "4");
        assert_eq!(crate::engine::calcul_rpn("4 2 + 3 - "), "3");
        assert_eq!(crate::engine::calcul_rpn("3 5 8 * 7 + *"), "141");
    }

    #[test]
    fn sqrt() {
        assert_eq!(crate::engine::calcul_rpn("9 SQRT"), "3");
    }
}