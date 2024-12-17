use super::test_helper::*;

#[test]
pub fn test_push() {
    test_simpl_vs_stk_vs_piet("./tests/smpl/test_push", "", "42", 0);
}

#[test]
pub fn test_pop() {
    test_simpl_vs_stk_vs_piet("./tests/smpl/test_pop", "", "41", 0);
}

#[test]
pub fn test_add() {
    test_simpl_vs_stk_vs_piet("./tests/smpl/test_add", "", "67", 0);
}

#[test]
pub fn test_sub() {
    test_simpl_vs_stk_vs_piet("./tests/smpl/test_sub", "", "37", 0);
}

#[test]
pub fn test_mul() {
    test_simpl_vs_stk_vs_piet("./tests/smpl/test_mul", "", "780", 0);
}

#[test]
pub fn test_div() {
    test_simpl_vs_stk_vs_piet("./tests/smpl/test_div", "", "3", 0);
}

#[test]
pub fn test_mod() {
    test_simpl_vs_stk_vs_piet("./tests/smpl/test_mod", "", "7", 0);
}

#[test]
pub fn test_roll() {
    test_simpl_vs_stk_vs_piet("./tests/smpl/test_roll", "", "021", 0);
}

#[test]
pub fn test_dup() {
    test_simpl_vs_stk_vs_piet("./tests/smpl/test_dup", "", "15 15", 0);
}

#[test]
pub fn test_malloc() {
    // (reverse) Stack/heap layout after push 4 x 4 and malloc 10
    test_simpl_vs_stk_vs_piet(
        "./tests/smpl/test_malloc",
        "",
        "5,4,4,4,4,10,0,0,0,0,0,0,0,0,0,0",
        0,
    );
}

#[test]
pub fn test_stack_size() {
    // stack_layout not stack_size..
    test_simpl_vs_stk_vs_piet("./tests/smpl/test_stack_size", "", "11 0123456789", 0);
}

#[test]
pub fn test_set_heap() {
    test_simpl_vs_stk_vs_piet("./tests/smpl/test_set_heap", "", "1,1,77", 0); // (reverse) Stack/heap layout after setting heap location 0
}

#[test]
pub fn test_get_heap() {
    test_simpl_vs_stk_vs_piet("./tests/smpl/test_get_heap", "", "77", 0); // set heap, get heap returns value set.
}

#[test]
pub fn test_copy_memory() {
    test_simpl_vs_stk_vs_piet("./tests/smpl/test_copy_memory", "", "nr", 0); // copy memory.
}

#[test]
pub fn test_var_num() {
    test_simpl_vs_stk_vs_piet("./tests/smpl/test_var_num", "", "2,0,0", 0); // variables are at the bottom of the stack (frame)
}

#[test]
pub fn test_var_list() {
    test_simpl_vs_stk_vs_piet("./tests/smpl/test_var_list", "", "2,-1,0", 0); // list variables index into memory, and are -1 when not allocated / nill
}

#[test]
pub fn test_length() {
    test_simpl_vs_stk_vs_piet("./tests/smpl/test_length", "", "7\n", 0);
}

#[test]
pub fn test_set_elem_get_elem() {
    test_simpl_vs_stk_vs_piet("./tests/smpl/test_set_elem_get_elem", "", "14\n11\n", 0);
}

#[test]
pub fn test_get_list() {
    test_simpl_vs_stk_vs_piet(
        "./tests/smpl/test_get_list",
        "",
        "[114,119,113,115,110,]\n",
        0,
    );
}

#[test]
pub fn test_append() {
    test_simpl_vs_stk_vs_piet(
        "./tests/smpl/test_append",
        "",
        "[114,119,113,115,110,]\n",
        0,
    );
}

#[test]
pub fn test_set_var_get_var() {
    test_simpl_vs_stk_vs_piet("./tests/smpl/test_set_var_get_var", "", "42\n15\n13", 5);
}

#[test]
pub fn test_eq0() {
    test_simpl_vs_stk_vs_piet("./tests/smpl/test_eq0", "", "0\n", 0);
}

#[test]
pub fn test_eq1() {
    test_simpl_vs_stk_vs_piet("./tests/smpl/test_eq1", "", "1\n", 0);
}

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

#[test]
pub fn test_inC() {
    test_simpl_vs_stk_vs_piet("./tests/smpl/test_inC", "abcd", "a", 0);
}

#[test]
pub fn test_registers() {
    test_simpl_vs_stk_vs_piet("./tests/smpl/test_registers", "", "101112", 5);
}

#[test]
#[allow(non_snake_case)]
pub fn test_readC_until() {
    test_simpl_vs_stk_vs_piet(
        "./tests/smpl/test_readC_until",
        "hello\nworld how\nare you\ndoing\n",
        "hello\nworld",
        1,
    );
}

#[test]
pub fn test_readlines() {
    test_simpl_vs_stk_vs_piet(
        "./tests/smpl/test_readlines",
        "hello\nworld how\nare you\ndoing\n",
        "hello\nworld how\nare you\ndoing\n",
        1,
    );
}

#[test]
pub fn test_swap() {
    test_simpl_vs_stk_vs_piet("./tests/smpl/test_swap", "23 43\n", "23 43", 0);
}

#[test]
pub fn test_dup_at_depth() {
    test_simpl_vs_stk_vs_piet("./tests/smpl/test_dup_at_depth", "", "43\n", 0);
}

#[test]
pub fn test_in() {
    test_simpl_vs_stk_vs_piet("./tests/smpl/test_in", "", "1 3 -1 -1", 2);
}

/////////////////////
// Fails in Python //
/////////////////////

// // #[test]
// // pub fn test_in_tuple() {
// //     test_stk_vs_piet("./tests/test_in_tuple", "", "10\n");
// // }
