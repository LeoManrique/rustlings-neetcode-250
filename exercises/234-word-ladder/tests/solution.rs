include!("../src/lib.rs");

#[test]
fn test_1() {
    assert_eq!(Solution::ladder_length("leet".to_string(), "code".to_string(), vec!["lest".to_string(), "leet".to_string(), "lose".to_string(), "code".to_string(), "lode".to_string(), "robe".to_string(), "home".to_string(), "dote".to_string(), "cake".to_string()]), 0);
}

#[test]
fn test_2() {
    assert_eq!(Solution::ladder_length("talk".to_string(), "tell".to_string(), vec!["talk".to_string(), "tell".to_string(), "tall".to_string(), "toll".to_string(), "toll".to_string()]), 3);
}

#[test]
fn test_3() {
    assert_eq!(Solution::ladder_length("talk".to_string(), "walk".to_string(), vec!["talk".to_string(), "walk".to_string(), "tall".to_string(), "tale".to_string(), "tali".to_string(), "wali".to_string(), "wali".to_string(), "wale".to_string(), "wall".to_string(), "walk".to_string()]), 2);
}

#[test]
fn test_4() {
    assert_eq!(Solution::ladder_length("leet".to_string(), "code".to_string(), vec!["lest".to_string(), "leet".to_string(), "lose".to_string(), "code".to_string(), "lode".to_string(), "robe".to_string(), "lost".to_string()]), 6);
}

#[test]
fn test_5() {
    assert_eq!(Solution::ladder_length("red".to_string(), "tax".to_string(), vec!["ted".to_string(), "tex".to_string(), "red".to_string(), "tax".to_string(), "tad".to_string(), "den".to_string(), "rex".to_string(), "pee".to_string()]), 4);
}

#[test]
fn test_6() {
    assert_eq!(Solution::ladder_length("cat".to_string(), "dog".to_string(), vec!["bat".to_string(), "rat".to_string(), "hat".to_string(), "hot".to_string(), "dot".to_string(), "dog".to_string()]), 5);
}

#[test]
fn test_7() {
    assert_eq!(Solution::ladder_length("hit".to_string(), "cog".to_string(), vec!["hot".to_string(), "dot".to_string(), "dog".to_string(), "lot".to_string(), "log".to_string()]), 0);
}

#[test]
fn test_8() {
    assert_eq!(Solution::ladder_length("hit".to_string(), "cog".to_string(), vec!["hot".to_string(), "dot".to_string(), "dog".to_string(), "lot".to_string(), "log".to_string(), "cog".to_string()]), 5);
}

#[test]
fn test_9() {
    assert_eq!(Solution::ladder_length("abcf".to_string(), "aefh".to_string(), vec!["abcf".to_string(), "aefg".to_string(), "aefh".to_string(), "aegh".to_string(), "cefh".to_string(), "cefh".to_string(), "aegh".to_string(), "cefg".to_string(), "abcf".to_string(), "abef".to_string()]), 0);
}

#[test]
fn test_10() {
    assert_eq!(Solution::ladder_length("machine".to_string(), "natural".to_string(), vec!["machene".to_string(), "machenr".to_string(), "machrne".to_string(), "machren".to_string(), "machenl".to_string(), "machenm".to_string(), "machene".to_string(), "machrne".to_string(), "machren".to_string(), "machenl".to_string(), "machenm".to_string(), "machane".to_string(), "machrne".to_string(), "machren".to_string(), "machenl".to_string(), "machenm".to_string(), "machenl".to_string(), "machene".to_string(), "machrne".to_string(), "machren".to_string(), "machenl".to_string(), "machenm".to_string(), "nachenl".to_string(), "nachene".to_string(), "nachrne".to_string(), "nachren".to_string(), "nachenl".to_string(), "nachenm".to_string(), "nachene".to_string(), "nachrne".to_string(), "nachren".to_string(), "nachenl".to_string(), "nachenm".to_string(), "nachene".to_string(), "nachrne".to_string(), "nachren".to_string(), "nachenl".to_string(), "nachenm".to_string(), "nachene".to_string(), "nachrne".to_string(), "nachren".to_string(), "nachenl".to_string(), "nachenm".to_string(), "nachene".to_string(), "nachrne".to_string(), "nachren".to_string(), "nachenl".to_string(), "nachenm".to_string(), "nachene".to_string(), "nachrne".to_string(), "nachren".to_string(), "nachenl".to_string(), "nachenm".to_string(), "nachene".to_string(), "nachrne".to_string(), "nachren".to_string(), "nachenl".to_string(), "nachenm".to_string(), "nachene".to_string(), "nachrne".to_string(), "nachren".to_string(), "nachenl".to_string(), "nachenm".to_string(), "natural".to_string()]), 0);
}

#[test]
fn test_11() {
    assert_eq!(Solution::ladder_length("listen".to_string(), "silent".to_string(), vec!["lisen".to_string(), "litesn".to_string(), "litens".to_string(), "listne".to_string(), "listen".to_string(), "siltne".to_string(), "silent".to_string(), "linset".to_string(), "lintes".to_string(), "sleint".to_string(), "ltsine".to_string(), "lintse".to_string(), "lisnet".to_string(), "lsitne".to_string(), "lnties".to_string(), "lintes".to_string(), "lintes".to_string(), "linsat".to_string(), "slient".to_string(), "lsinte".to_string(), "linset".to_string()]), 0);
}

#[test]
fn test_12() {
    assert_eq!(Solution::ladder_length("algorithm".to_string(), "rhythm".to_string(), vec!["alorhythm".to_string(), "alohrhythm".to_string(), "alohrhythm".to_string(), "alorhythm".to_string(), "alorhythm".to_string(), "alorhythm".to_string(), "alorhythm".to_string(), "alorhythm".to_string(), "alorhythm".to_string(), "alorhythm".to_string(), "alorhythm".to_string(), "alorhythm".to_string(), "alorhythm".to_string(), "alorhythm".to_string(), "alorhythm".to_string(), "alorhythm".to_string(), "alorhythm".to_string(), "alorhythm".to_string(), "alorhythm".to_string(), "alorhythm".to_string(), "alorhythm".to_string(), "alorhythm".to_string(), "alorhythm".to_string(), "alorhythm".to_string(), "alorhythm".to_string(), "alorhythm".to_string(), "alorhythm".to_string(), "alorhythm".to_string(), "alorhythm".to_string(), "alorhythm".to_string(), "alorhythm".to_string(), "alorhythm".to_string(), "alorhythm".to_string(), "alorhythm".to_string(), "alorhythm".to_string(), "alorhythm".to_string(), "alorhythm".to_string(), "alorhythm".to_string(), "alorhythm".to_string(), "alorhythm".to_string(), "alorhythm".to_string(), "alorhythm".to_string(), "alorhythm".to_string(), "alorhythm".to_string(), "alorhythm".to_string(), "alorhythm".to_string(), "alorhythm".to_string(), "alorhythm".to_string(), "alorhythm".to_string(), "alorhythm".to_string(), "alorhythm".to_string(), "alorhythm".to_string(), "alorhythm".to_string(), "alorhythm".to_string(), "alorhythm".to_string(), "alorhythm".to_string(), "alorhythm".to_string(), "alorhythm".to_string(), "alorhythm".to_string(), "alorhythm".to_string(), "alorhythm".to_string(), "alorhythm".to_string(), "alorhythm".to_string(), "alorhythm".to_string(), "alorhythm".to_string(), "alorhythm".to_string(), "alorhythm".to_string(), "alorhythm".to_string(), "alorhythm".to_string(), "alorhythm".to_string(), "alorhythm".to_string(), "alorhythm".to_string(), "alorhythm".to_string(), "rhythm".to_string(), "rhythm".to_string(), "rhythm".to_string(), "rhythm".to_string(), "rhythm".to_string(), "rhythm".to_string(), "rhythm".to_string(), "rhythm".to_string(), "rhythm".to_string(), "rhythm".to_string()]), 0);
}

#[test]
fn test_13() {
    assert_eq!(Solution::ladder_length("physics".to_string(), "chemist".to_string(), vec!["phyiscs".to_string(), "phyisic".to_string(), "phyisct".to_string(), "phyisci".to_string(), "phyicsi".to_string(), "physics".to_string(), "physisi".to_string(), "physcii".to_string(), "physici".to_string(), "phyiscs".to_string(), "phyiscs".to_string(), "phyiscs".to_string(), "phyiscs".to_string(), "physcis".to_string(), "physcis".to_string(), "physcis".to_string(), "physcis".to_string(), "physcis".to_string(), "physcis".to_string(), "physcis".to_string(), "physcis".to_string(), "physcis".to_string(), "physcis".to_string(), "physcis".to_string(), "physcis".to_string(), "physcis".to_string(), "physcis".to_string(), "physcis".to_string(), "physcis".to_string(), "physcis".to_string(), "physcis".to_string(), "physcis".to_string(), "physcis".to_string(), "physcis".to_string(), "physcis".to_string(), "physcis".to_string(), "physcis".to_string(), "physcis".to_string(), "physcis".to_string(), "physcis".to_string(), "physcis".to_string(), "physcis".to_string(), "physcis".to_string(), "physcis".to_string(), "physcis".to_string(), "physcis".to_string(), "physcis".to_string(), "physcis".to_string(), "physcis".to_string(), "physcis".to_string(), "physcis".to_string(), "chemics".to_string(), "chemist".to_string(), "chemist".to_string(), "chemist".to_string(), "chemist".to_string(), "chemist".to_string(), "chemist".to_string(), "chemist".to_string(), "chemist".to_string(), "chemist".to_string(), "chemist".to_string(), "chemist".to_string(), "chemist".to_string(), "chemist".to_string(), "chemist".to_string(), "chemist".to_string(), "chemist".to_string(), "chemist".to_string(), "chemist".to_string(), "chemist".to_string(), "chemist".to_string(), "chemist".to_string(), "chemist".to_string(), "chemist".to_string(), "chemist".to_string(), "chemist".to_string(), "chemist".to_string(), "chemist".to_string(), "chemist".to_string(), "chemist".to_string(), "chemist".to_string(), "chemist".to_string(), "chemist".to_string(), "chemist".to_string(), "chemist".to_string(), "chemist".to_string(), "chemist".to_string(), "chemist".to_string(), "chemist".to_string()]), 0);
}
