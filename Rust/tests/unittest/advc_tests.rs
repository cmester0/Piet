use super::test_helper::*;

#[test]
pub fn test_full() {
    test_advc_no_file("./tests/advc/test.advc", "", "987654321", 5);
}
