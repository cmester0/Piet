
use super::test_helper::test_stk_vs_piet;
use super::test_helper::run_stk;

#[test]
pub fn test_set_var_get_var() {
    test_stk_vs_piet("./tests/test_set_var_get_var", "", "42");
}

#[test]
pub fn test_length() {
    test_stk_vs_piet("./tests/test_length", "", "7\n");
}

#[test]
pub fn test_eq0() {
    test_stk_vs_piet("./tests/test_eq0", "", "0\n");
}

#[test]
pub fn test_eq1() {
    test_stk_vs_piet("./tests/test_eq1", "", "1\n");
}

#[test]
pub fn test_append() {
    test_stk_vs_piet("./tests/test_append", "", "[114,119,113,115,110,]\n");
}

#[test]
#[allow(non_snake_case)]
pub fn test_print_listC() {
    test_stk_vs_piet("./tests/test_print_listC", "", "hello world\n");
}

#[test]
pub fn test_readlines() {
    test_stk_vs_piet("./tests/test_readlines",
                     "hello\nworld how\nare you\ndoing",
                     "hello\nworld how\nare you\ndoing");
}

#[test]
pub fn test_swap() {
    test_stk_vs_piet("./tests/test_swap", "23 43\n", "23 43");
}

#[test]
pub fn test_set_elem_get_elem() {
    test_stk_vs_piet("./tests/test_set_elem_get_elem", "", "14\n11\n");
}

#[test]
pub fn test_dup_at_depth() {
    test_stk_vs_piet("./tests/test_dup_at_depth", "", "43\n");
}

#[test]
pub fn test_in() {
    test_stk_vs_piet("./tests/test_in", "", "10\n");
}

#[test]
pub fn test_in_tuple() {
    test_stk_vs_piet("./tests/test_in_tuple", "", "10\n");
}
