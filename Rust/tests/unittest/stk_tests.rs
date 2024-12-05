use super::test_helper::*; // test_stk_vs_piet;

#[test]
pub fn test_roll() {
    // let stk_str = run_stk(format!("{}.stk", "./tests/stk/test_roll").as_str(), "");

    // assert_eq!(stk_str, "");

    test_stk_vs_piet("./tests/stk/test_roll", "1 2", "1 2");
}
