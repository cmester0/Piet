use super::test_helper::*;

#[test]
pub fn test_io() {
    test_advc_no_file("./tests/advc/test.advc", "100", "100", 5);
}

#[test]
pub fn test_if() {
    test_advc_no_file("./tests/advc/test_if.advc", "1", "10", 5);
    test_advc_no_file("./tests/advc/test_if.advc", "0", "11", 5);
}

#[test]
pub fn test_for() {
    test_advc_no_file("./tests/advc/test_for.advc", "1", "10", 5);
    test_advc_no_file("./tests/advc/test_for.advc", "0", "12", 5);
}

#[test]
pub fn test_in() {
    test_advc_no_file("./tests/advc/test_in.advc", "bnas\n", "2", 5);
}

#[test]
#[allow(non_snake_case)]
pub fn test_outN() {
    test_advc_no_file("./tests/advc/test_outN.advc", "bnas\n", "2", 5);
}

#[test]
#[allow(non_snake_case)]
pub fn test_eq() {
    test_advc_no_file("./tests/advc/test_eq.advc", "237069504000: 96 30 30 28 139 705", "0", 5);
}

///////////////////////////////////////
// Real programs / Integration tests //
///////////////////////////////////////

#[test]
pub fn test_full() {
    // First 1000 primes
    test_advc_no_file("./tests/advc/primes.advc", "1000", "2,3,5,7,11,13,17,19,23,29,31,37,41,43,47,53,59,61,67,71,73,79,83,89,97,101,103,107,109,113,127,131,137,139,149,151,157,163,167,173,179,181,191,193,197,199,211,223,227,229,233,239,241,251,257,263,269,271,277,281,283,293,307,311,313,317,331,337,347,349,353,359,367,373,379,383,389,397,401,409,419,421,431,433,439,443,449,457,461,463,467,479,487,491,499,503,509,521,523,541,547,557,563,569,571,577,587,593,599,601,607,613,617,619,631,641,643,647,653,659,661,673,677,683,691,701,709,719,727,733,739,743,751,757,761,769,773,787,797,809,811,821,823,827,829,839,853,857,859,863,877,881,883,887,907,911,919,929,937,941,947,953,967,971,977,983,991,997,", 5);
}

#[test]
pub fn test_hashmap() {
    test_advc_no_file("./tests/advc/hashmap.advc", "", "initial 12\nrepeat_ 66\nchange_ 22\nsearch\ninitial 12\nchange_ 22\noutside -1\n", 5);
}

#[test]
pub fn test_call_fail() {
    test_advc_no_file("./tests/advc/call_fail.advc", "", "", 5);
}

#[test]
pub fn test_call_fail2() {
    test_advc_no_file("./tests/advc/call_fail2.advc", "", "", 5);
}
