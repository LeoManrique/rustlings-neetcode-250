include!("../src/lib.rs");

#[test]
fn test_1() {
    assert_eq!(Solution::min_distance("park".to_string(), "spake".to_string()), 3);
}

#[test]
fn test_2() {
    assert_eq!(Solution::min_distance("algorithm".to_string(), "altruistic".to_string()), 6);
}

#[test]
fn test_3() {
    assert_eq!(Solution::min_distance("abc".to_string(), "".to_string()), 3);
}

#[test]
fn test_4() {
    assert_eq!(Solution::min_distance("horse".to_string(), "ros".to_string()), 3);
}

#[test]
fn test_5() {
    assert_eq!(Solution::min_distance("a".to_string(), "b".to_string()), 1);
}

#[test]
fn test_6() {
    assert_eq!(Solution::min_distance("kitten".to_string(), "sitting".to_string()), 3);
}

#[test]
fn test_7() {
    assert_eq!(Solution::min_distance("a".to_string(), "a".to_string()), 0);
}

#[test]
fn test_8() {
    assert_eq!(Solution::min_distance("a".to_string(), "".to_string()), 1);
}

#[test]
fn test_9() {
    assert_eq!(Solution::min_distance("abcdefghij".to_string(), "jihgfedcba".to_string()), 10);
}

#[test]
fn test_10() {
    assert_eq!(Solution::min_distance("".to_string(), "a".to_string()), 1);
}

#[test]
fn test_11() {
    assert_eq!(Solution::min_distance("".to_string(), "abc".to_string()), 3);
}

#[test]
fn test_12() {
    assert_eq!(Solution::min_distance("abc".to_string(), "abc".to_string()), 0);
}

#[test]
fn test_13() {
    assert_eq!(Solution::min_distance("intention".to_string(), "execution".to_string()), 5);
}

#[test]
fn test_14() {
    assert_eq!(Solution::min_distance("abcd".to_string(), "dcba".to_string()), 4);
}

#[test]
fn test_15() {
    assert_eq!(Solution::min_distance("".to_string(), "".to_string()), 0);
}

#[test]
fn test_16() {
    assert_eq!(Solution::min_distance("abc".to_string(), "def".to_string()), 3);
}

#[test]
fn test_17() {
    assert_eq!(Solution::min_distance("flaw".to_string(), "lawn".to_string()), 2);
}

#[test]
fn test_18() {
    assert_eq!(Solution::min_distance("example".to_string(), "samples".to_string()), 3);
}

#[test]
fn test_19() {
    assert_eq!(Solution::min_distance("abbreviation".to_string(), "acceleration".to_string()), 6);
}

#[test]
fn test_20() {
    assert_eq!(Solution::min_distance("abcdefghijklmnopqrstuvwxyz".to_string(), "zyxwvutsrqponmlkjihgfedcba".to_string()), 26);
}

#[test]
fn test_21() {
    assert_eq!(Solution::min_distance("acknowledgment".to_string(), "acknowledge".to_string()), 3);
}

#[test]
fn test_22() {
    assert_eq!(Solution::min_distance("xylophone".to_string(), "xylophone".to_string()), 0);
}

#[test]
fn test_23() {
    assert_eq!(Solution::min_distance("dynamic".to_string(), "programming".to_string()), 8);
}

#[test]
fn test_24() {
    assert_eq!(Solution::min_distance("embarrassing".to_string(), "aberration".to_string()), 7);
}

#[test]
fn test_25() {
    assert_eq!(Solution::min_distance("supercalifragilisticexpialidocious".to_string(), "supercalifragilisticexpialidociousness".to_string()), 4);
}

#[test]
fn test_26() {
    assert_eq!(Solution::min_distance("sequential".to_string(), "sequentialization".to_string()), 7);
}

#[test]
fn test_27() {
    assert_eq!(Solution::min_distance("algorithm".to_string(), "alligator".to_string()), 6);
}

#[test]
fn test_28() {
    assert_eq!(Solution::min_distance("distance".to_string(), "distinction".to_string()), 5);
}

#[test]
fn test_29() {
    assert_eq!(Solution::min_distance("irreversible".to_string(), "reversible".to_string()), 2);
}

#[test]
fn test_30() {
    assert_eq!(Solution::min_distance("serendipity".to_string(), "inspiration".to_string()), 10);
}

#[test]
fn test_31() {
    assert_eq!(Solution::min_distance("karolin".to_string(), "kathrin".to_string()), 3);
}

#[test]
fn test_32() {
    assert_eq!(Solution::min_distance("representative".to_string(), "reproductive".to_string()), 6);
}

#[test]
fn test_33() {
    assert_eq!(Solution::min_distance("congratulations".to_string(), "congratulate".to_string()), 4);
}

#[test]
fn test_34() {
    assert_eq!(Solution::min_distance("supercalifragilisticexpialidocious".to_string(), "pseudopseudohypoparathyroidism".to_string()), 30);
}

#[test]
fn test_35() {
    assert_eq!(Solution::min_distance("qwerty".to_string(), "azerty".to_string()), 2);
}

#[test]
fn test_36() {
    assert_eq!(Solution::min_distance("mississippi".to_string(), "mispelling".to_string()), 7);
}

#[test]
fn test_37() {
    assert_eq!(Solution::min_distance("levenshtein".to_string(), "levenshtein".to_string()), 0);
}

#[test]
fn test_38() {
    assert_eq!(Solution::min_distance("sequential".to_string(), "concurrent".to_string()), 9);
}

#[test]
fn test_39() {
    assert_eq!(Solution::min_distance("psychological".to_string(), "psychologist".to_string()), 3);
}

#[test]
fn test_40() {
    assert_eq!(Solution::min_distance("interpretation".to_string(), "interpretive".to_string()), 4);
}

#[test]
fn test_41() {
    assert_eq!(Solution::min_distance("transient".to_string(), "transitory".to_string()), 4);
}

#[test]
fn test_42() {
    assert_eq!(Solution::min_distance("abcdefgh".to_string(), "xyz".to_string()), 8);
}

#[test]
fn test_43() {
    assert_eq!(Solution::min_distance("algorithmic".to_string(), "algebraic".to_string()), 6);
}

#[test]
fn test_44() {
    assert_eq!(Solution::min_distance("transformation".to_string(), "transfiguration".to_string()), 4);
}

#[test]
fn test_45() {
    assert_eq!(Solution::min_distance("edit".to_string(), "editing".to_string()), 3);
}

#[test]
fn test_46() {
    assert_eq!(Solution::min_distance("levenshtein".to_string(), "levenschtein".to_string()), 1);
}

#[test]
fn test_47() {
    assert_eq!(Solution::min_distance("abracadabra".to_string(), "barbarian".to_string()), 8);
}

#[test]
fn test_48() {
    assert_eq!(Solution::min_distance("encyclopedia".to_string(), "encyclopedic".to_string()), 1);
}

#[test]
fn test_49() {
    assert_eq!(Solution::min_distance("transformation".to_string(), "transmogrification".to_string()), 6);
}

#[test]
fn test_50() {
    assert_eq!(Solution::min_distance("complexity".to_string(), "simplicity".to_string()), 4);
}

#[test]
fn test_51() {
    assert_eq!(Solution::min_distance("transformation".to_string(), "transformation".to_string()), 0);
}

#[test]
fn test_52() {
    assert_eq!(Solution::min_distance("transformation".to_string(), "transform".to_string()), 5);
}

#[test]
fn test_53() {
    assert_eq!(Solution::min_distance("abcdexyz".to_string(), "abcd".to_string()), 4);
}

#[test]
fn test_54() {
    assert_eq!(Solution::min_distance("zoology".to_string(), "botany".to_string()), 5);
}

#[test]
fn test_55() {
    assert_eq!(Solution::min_distance("computation".to_string(), "computational".to_string()), 2);
}

#[test]
fn test_56() {
    assert_eq!(Solution::min_distance("pharmaceuticals".to_string(), "pharmacology".to_string()), 8);
}

#[test]
fn test_57() {
    assert_eq!(Solution::min_distance("distance".to_string(), "difference".to_string()), 5);
}

#[test]
fn test_58() {
    assert_eq!(Solution::min_distance("reciprocal".to_string(), "perpendicular".to_string()), 10);
}

#[test]
fn test_59() {
    assert_eq!(Solution::min_distance("development".to_string(), "evolution".to_string()), 7);
}

#[test]
fn test_60() {
    assert_eq!(Solution::min_distance("amplification".to_string(), "attenuation".to_string()), 7);
}

#[test]
fn test_61() {
    assert_eq!(Solution::min_distance("thermodynamics".to_string(), "thermodynamic".to_string()), 1);
}

#[test]
fn test_62() {
    assert_eq!(Solution::min_distance("metamorphosis".to_string(), "metaphysics".to_string()), 5);
}

#[test]
fn test_63() {
    assert_eq!(Solution::min_distance("quantum".to_string(), "quark".to_string()), 4);
}

#[test]
fn test_64() {
    assert_eq!(Solution::min_distance("repetition".to_string(), "repetitions".to_string()), 1);
}

#[test]
fn test_65() {
    assert_eq!(Solution::min_distance("transform".to_string(), "transformation".to_string()), 5);
}

#[test]
fn test_66() {
    assert_eq!(Solution::min_distance("sequence".to_string(), "consequence".to_string()), 3);
}

#[test]
fn test_67() {
    assert_eq!(Solution::min_distance("abbreviation".to_string(), "contraction".to_string()), 8);
}

#[test]
fn test_68() {
    assert_eq!(Solution::min_distance("development".to_string(), "independently".to_string()), 9);
}

#[test]
fn test_69() {
    assert_eq!(Solution::min_distance("distance".to_string(), "distant".to_string()), 2);
}

#[test]
fn test_70() {
    assert_eq!(Solution::min_distance("mississippi".to_string(), "mississipi".to_string()), 1);
}

#[test]
fn test_71() {
    assert_eq!(Solution::min_distance("biographical".to_string(), "biographies".to_string()), 3);
}

#[test]
fn test_72() {
    assert_eq!(Solution::min_distance("optimization".to_string(), "maximization".to_string()), 3);
}

#[test]
fn test_73() {
    assert_eq!(Solution::min_distance("photosynthesis".to_string(), "photosynthetic".to_string()), 2);
}

#[test]
fn test_74() {
    assert_eq!(Solution::min_distance("supercalifragilisticexpialidocious".to_string(), "pneumonoultramicroscopicsilicovolcanoconiosis".to_string()), 31);
}

#[test]
fn test_75() {
    assert_eq!(Solution::min_distance("algorithm".to_string(), "altruism".to_string()), 5);
}

#[test]
fn test_76() {
    assert_eq!(Solution::min_distance("representation".to_string(), "representative".to_string()), 2);
}

#[test]
fn test_77() {
    assert_eq!(Solution::min_distance("reptile".to_string(), "reptilian".to_string()), 3);
}

#[test]
fn test_78() {
    assert_eq!(Solution::min_distance("abacaxi".to_string(), "abacax".to_string()), 1);
}

#[test]
fn test_79() {
    assert_eq!(Solution::min_distance("mississippi".to_string(), "misisippi".to_string()), 2);
}

#[test]
fn test_80() {
    assert_eq!(Solution::min_distance("electrical".to_string(), "electronic".to_string()), 4);
}

#[test]
fn test_81() {
    assert_eq!(Solution::min_distance("photograph".to_string(), "photography".to_string()), 1);
}

#[test]
fn test_82() {
    assert_eq!(Solution::min_distance("python".to_string(), "typhon".to_string()), 2);
}

#[test]
fn test_83() {
    assert_eq!(Solution::min_distance("kayak".to_string(), "racecar".to_string()), 5);
}

#[test]
fn test_84() {
    assert_eq!(Solution::min_distance("optimization".to_string(), "minimization".to_string()), 3);
}

#[test]
fn test_85() {
    assert_eq!(Solution::min_distance("biological".to_string(), "biographic".to_string()), 7);
}

#[test]
fn test_86() {
    assert_eq!(Solution::min_distance("congratulations".to_string(), "congratulations".to_string()), 0);
}

#[test]
fn test_87() {
    assert_eq!(Solution::min_distance("photography".to_string(), "photomontage".to_string()), 7);
}

#[test]
fn test_88() {
    assert_eq!(Solution::min_distance("babble".to_string(), "bubble".to_string()), 1);
}

#[test]
fn test_89() {
    assert_eq!(Solution::min_distance("transformation".to_string(), "transmute".to_string()), 7);
}

#[test]
fn test_90() {
    assert_eq!(Solution::min_distance("supercalifragilisticexpialidocious".to_string(), "antidisestablishmentarianism".to_string()), 26);
}

#[test]
fn test_91() {
    assert_eq!(Solution::min_distance("semantically".to_string(), "syntactically".to_string()), 4);
}

#[test]
fn test_92() {
    assert_eq!(Solution::min_distance("synchronous".to_string(), "asynchronous".to_string()), 1);
}

#[test]
fn test_93() {
    assert_eq!(Solution::min_distance("evolution".to_string(), "revolution".to_string()), 1);
}

#[test]
fn test_94() {
    assert_eq!(Solution::min_distance("abcdefghij".to_string(), "zyxwvutsrqponmlkjihgfedcba".to_string()), 25);
}

#[test]
fn test_95() {
    assert_eq!(Solution::min_distance("misunderstanding".to_string(), "understanding".to_string()), 3);
}

#[test]
fn test_96() {
    assert_eq!(Solution::min_distance("combinatorics".to_string(), "computation".to_string()), 7);
}

#[test]
fn test_97() {
    assert_eq!(Solution::min_distance("electromagnetic".to_string(), "electroencephalography".to_string()), 13);
}

#[test]
fn test_98() {
    assert_eq!(Solution::min_distance("permutation".to_string(), "combination".to_string()), 6);
}

#[test]
fn test_99() {
    assert_eq!(Solution::min_distance("subsequence".to_string(), "supersequence".to_string()), 3);
}

#[test]
fn test_100() {
    assert_eq!(Solution::min_distance("lumberjack".to_string(), "lumbersome".to_string()), 4);
}

#[test]
fn test_101() {
    assert_eq!(Solution::min_distance("optimization".to_string(), "information".to_string()), 7);
}

#[test]
fn test_102() {
    assert_eq!(Solution::min_distance("interpolation".to_string(), "interpretation".to_string()), 3);
}

#[test]
fn test_103() {
    assert_eq!(Solution::min_distance("magnificent".to_string(), "magnanimous".to_string()), 6);
}

#[test]
fn test_104() {
    assert_eq!(Solution::min_distance("xylophone".to_string(), "xylography".to_string()), 6);
}

#[test]
fn test_105() {
    assert_eq!(Solution::min_distance("biochemistry".to_string(), "bioinformatics".to_string()), 10);
}

#[test]
fn test_106() {
    assert_eq!(Solution::min_distance("exaggeration".to_string(), "aggrandizement".to_string()), 10);
}

#[test]
fn test_107() {
    assert_eq!(Solution::min_distance("abcde".to_string(), "fghij".to_string()), 5);
}

#[test]
fn test_108() {
    assert_eq!(Solution::min_distance("mississippi".to_string(), "mispell".to_string()), 8);
}

#[test]
fn test_109() {
    assert_eq!(Solution::min_distance("interpolation".to_string(), "internationalization".to_string()), 8);
}

#[test]
fn test_110() {
    assert_eq!(Solution::min_distance("entomology".to_string(), "herpetology".to_string()), 5);
}

#[test]
fn test_111() {
    assert_eq!(Solution::min_distance("interference".to_string(), "interferometer".to_string()), 5);
}

#[test]
fn test_112() {
    assert_eq!(Solution::min_distance("communication".to_string(), "communism".to_string()), 6);
}

#[test]
fn test_113() {
    assert_eq!(Solution::min_distance("characterization".to_string(), "categorization".to_string()), 6);
}

#[test]
fn test_114() {
    assert_eq!(Solution::min_distance("abracadabra".to_string(), "cabracadabara".to_string()), 2);
}

#[test]
fn test_115() {
    assert_eq!(Solution::min_distance("transformation".to_string(), "transference".to_string()), 7);
}

#[test]
fn test_116() {
    assert_eq!(Solution::min_distance("decomposition".to_string(), "discombobulation".to_string()), 7);
}

#[test]
fn test_117() {
    assert_eq!(Solution::min_distance("intermediate".to_string(), "interim".to_string()), 6);
}

#[test]
fn test_118() {
    assert_eq!(Solution::min_distance("interpolation".to_string(), "intrapolation".to_string()), 2);
}

#[test]
fn test_119() {
    assert_eq!(Solution::min_distance("visualization".to_string(), "representation".to_string()), 9);
}

#[test]
fn test_120() {
    assert_eq!(Solution::min_distance("abbreviation".to_string(), "elision".to_string()), 7);
}

#[test]
fn test_121() {
    assert_eq!(Solution::min_distance("metamorphism".to_string(), "metempsychosis".to_string()), 8);
}

#[test]
fn test_122() {
    assert_eq!(Solution::min_distance("encyclopedia".to_string(), "encyclopedia".to_string()), 0);
}

#[test]
fn test_123() {
    assert_eq!(Solution::min_distance("interdisciplinary".to_string(), "transdisciplinary".to_string()), 5);
}

#[test]
fn test_124() {
    assert_eq!(Solution::min_distance("unbelievable".to_string(), "believable".to_string()), 2);
}

#[test]
fn test_125() {
    assert_eq!(Solution::min_distance("abracadabra".to_string(), "cadabra".to_string()), 4);
}

#[test]
fn test_126() {
    assert_eq!(Solution::min_distance("hydrostatic".to_string(), "hydrodynamics".to_string()), 5);
}

#[test]
fn test_127() {
    assert_eq!(Solution::min_distance("parallel".to_string(), "perpendicular".to_string()), 10);
}

#[test]
fn test_128() {
    assert_eq!(Solution::min_distance("administration".to_string(), "administrative".to_string()), 2);
}

#[test]
fn test_129() {
    assert_eq!(Solution::min_distance("cosmology".to_string(), "cosmonautics".to_string()), 7);
}

#[test]
fn test_130() {
    assert_eq!(Solution::min_distance("development".to_string(), "develpoment".to_string()), 2);
}

#[test]
fn test_131() {
    assert_eq!(Solution::min_distance("photosynthesis".to_string(), "photography".to_string()), 8);
}

#[test]
fn test_132() {
    assert_eq!(Solution::min_distance("biochemistry".to_string(), "biological".to_string()), 8);
}
