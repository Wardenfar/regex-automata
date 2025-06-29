use expect_test::expect;
use regex_automata::{determine_and_minimize_nfa, dfa_to_hir, hir_to_nfa};
use regex_syntax::parse;

fn roundtrip(pattern: &str) -> String {
    let hir_in = parse(pattern).unwrap();
    let nfa = hir_to_nfa(&hir_in);
    //std::fs::write("nfa.dot", nfa.to_dot()).unwrap();
    let dfa = determine_and_minimize_nfa(nfa);
    //std::fs::write("dfa.dot", dfa.to_dot()).unwrap();
    let hir_out = dfa_to_hir(dfa);
    hir_out.to_string()
}

macro_rules! expect_test {
    ($name:ident, $regex_in:expr => $out:expr) => {
        #[test]
        fn $name() {
            $out.assert_eq(&roundtrip($regex_in));
        }
    };
}

expect_test!(letter, "a" => expect!["a"]);
expect_test!(word, "abc" => expect!["(?:abc)"]);
expect_test!(letter_choice, "a|b|c" => expect!["[a-c]"]);
expect_test!(bug1, r#"(a|b)*"# => expect!["[ab]*"]);
expect_test!(bug2, r#"(a|b)*abb(a|b)*"# => expect!["(?:b*aa*b(?:aa*b)*b[ab]*)"]);
