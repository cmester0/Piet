
use super::test_helper::*;

#[test]
pub fn test_push() {
    test_simpl_vs_stk_vs_piet("./tests/smpl/test_push", "", "42");
}

#[test]
pub fn test_pop() {
    test_simpl_vs_stk_vs_piet("./tests/smpl/test_pop", "", "41");
}

#[test]
pub fn test_add() {
    test_simpl_vs_stk_vs_piet("./tests/smpl/test_add", "", "67");
}

#[test]
pub fn test_sub() {
    test_simpl_vs_stk_vs_piet("./tests/smpl/test_sub", "", "37");
}

#[test]
pub fn test_mul() {
    test_simpl_vs_stk_vs_piet("./tests/smpl/test_mul", "", "780");
}

#[test]
pub fn test_div() {
    test_simpl_vs_stk_vs_piet("./tests/smpl/test_div", "", "3");
}

#[test]
pub fn test_mod() {
    test_simpl_vs_stk_vs_piet("./tests/smpl/test_mod", "", "7");
}

#[test]
pub fn test_roll() {
    test_simpl_vs_stk_vs_piet("./tests/smpl/test_roll", "", "021");
}

#[test]
pub fn test_dup() {
    test_simpl_vs_stk_vs_piet("./tests/smpl/test_dup", "", "15 15");
}

#[test]
pub fn test_malloc() {
    // (reverse) Stack/heap layout after push 4 x 4 and malloc 10
    test_simpl_vs_stk_vs_piet("./tests/smpl/test_malloc", "", "5,4,4,4,4,10,0,0,0,0,0,0,0,0,0,0");
}

#[test]
pub fn test_stack_size() { // stack_layout not stack_size..
    test_simpl_vs_stk_vs_piet("./tests/smpl/test_stack_size", "", "11 0123456789");
}

#[test]
pub fn test_set_heap() {
    test_simpl_vs_stk_vs_piet("./tests/smpl/test_set_heap", "", "1,1,77"); // (reverse) Stack/heap layout after setting heap location 0
}

#[test]
pub fn test_get_heap() {
    test_simpl_vs_stk_vs_piet("./tests/smpl/test_get_heap", "", "77"); // set heap, get heap returns value set.
}

// #[test]
// pub fn test_copy_memory() {
//     test_simpl_vs_stk_vs_piet("./tests/smpl/test_copy_memory", "", "77"); // set heap, get heap returns value set.
// }

#[test]
pub fn test_var_num() {
    test_simpl_vs_stk_vs_piet("./tests/smpl/test_var_num", "", "2,0,0"); // variables are at the bottom of the stack (frame)
}

#[test]
pub fn test_var_list() {
    test_simpl_vs_stk_vs_piet("./tests/smpl/test_var_list", "", "2,-1,0"); // list variables index into memory, and are -1 when not allocated / nill
}

#[test]
pub fn test_length() {
    test_simpl_vs_stk_vs_piet("./tests/smpl/test_length", "", "7\n");
}

#[test]
pub fn test_set_elem_get_elem() {
    test_simpl_vs_stk_vs_piet("./tests/smpl/test_set_elem_get_elem", "", "14\n11\n");
}

// #[test]
// pub fn test_get_list() {
//     test_simpl_vs_stk_vs_piet("./tests/smpl/test_get_list", "", "[114,119,113,115,110,]\n");
// }

// #[test]
// pub fn test_append() {
//     test_simpl_vs_stk_vs_piet("./tests/smpl/test_append", "", "[114,119,113,115,110,]\n");
// }

// #[test]
// pub fn test_roll() {
//     test_simpl_vs_stk_vs_piet("./tests/smpl/test_roll", "", "");
// }

// #[test]
// pub fn test_set_var_get_var() {
//     test_simpl_vs_stk_vs_piet("./tests/smpl/test_set_var_get_var", "", "42");
// }

// #[test]
// pub fn test_eq0() {
//     test_simpl_vs_stk_vs_piet("./tests/smpl/test_eq0", "", "0\n");
// }

// #[test]
// pub fn test_eq1() {
//     test_stk_vs_piet("./tests/test_eq1", "", "1\n");
// }

// #[test]
// #[allow(non_snake_case)]
// pub fn test_print_listC() {
//     test_stk_vs_piet("./tests/test_print_listC", "", "hello world\n");
// }

// // #[test]
// // pub fn test_readlines_fail() {
// //     test_stk_vs_piet("./tests/test_readlines",
// //                      "hello\nworld how\nare you\ndoing",
// //                      "hello\nworld how\nare you\ndoing"); // Fails, ish?
// // }

// #[test]
// pub fn test_readlines() {
//     test_stk_vs_piet("./tests/test_readlines",
//                      "hello\nworld how\nare you\ndoing\n",
//                      "hello\nworld how\nare you\ndoing\n");
// }

// #[test]
// pub fn test_swap() {
//     test_simpl_vs_stk_vs_piet("./tests/smpl/test_swap", "23 43\n", "23 43");
// }

// #[test]
// pub fn test_dup_at_depth() {
//     test_simpl_vs_stk_vs_piet("./tests/smpl/test_dup_at_depth", "", "43\n");
// }

// /////////////////////
// // Fails in Python //
// /////////////////////

// // #[test]
// // pub fn test_in() {
// //     test_simpl_vs_stk_vs_piet("./tests/smpl/test_in", "", "10\n");
// // }

// // #[test]
// // pub fn test_in_tuple() {
// //     test_stk_vs_piet("./tests/test_in_tuple", "", "10\n");
// // }
