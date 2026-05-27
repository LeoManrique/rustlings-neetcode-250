include!("../src/lib.rs");

#[test]
fn test_1() {
    assert_eq!(Solution::combination_sum4(vec![1, 5, 10], 100), 8437020668201);
}

#[test]
fn test_2() {
    assert_eq!(Solution::combination_sum4(vec![10, 20, 30], 100), 274);
}

#[test]
fn test_3() {
    assert_eq!(Solution::combination_sum4(vec![1, 2, 3], 4), 7);
}

#[test]
fn test_4() {
    assert_eq!(Solution::combination_sum4(vec![5, 50, 75], 95), 16);
}

#[test]
fn test_5() {
    assert_eq!(Solution::combination_sum4(vec![5, 1, 3], 8), 19);
}

#[test]
fn test_6() {
    assert_eq!(Solution::combination_sum4(vec![5, 10, 20], 100), 46754);
}

#[test]
fn test_7() {
    assert_eq!(Solution::combination_sum4(vec![2, 1, 5], 10), 128);
}

#[test]
fn test_8() {
    assert_eq!(Solution::combination_sum4(vec![4, 2, 1], 32), 39882198);
}

#[test]
fn test_9() {
    assert_eq!(Solution::combination_sum4(vec![9], 3), 0);
}

#[test]
fn test_10() {
    assert_eq!(Solution::combination_sum4(vec![1], 100), 1);
}

#[test]
fn test_11() {
    assert_eq!(Solution::combination_sum4(vec![2, 5, 10, 20], 25), 119);
}

#[test]
fn test_12() {
    assert_eq!(Solution::combination_sum4(vec![1, 2, 3, 4, 5], 50), 256641310658978);
}

#[test]
fn test_13() {
    assert_eq!(Solution::combination_sum4(vec![3, 33, 333], 33333), 536100562017203100794406478687994186977009406170595938319227925094657614236234534558497196506399133621225707174188255190262018651187114351423083457111666117940448620061113293355456633188716465870563000681683793687796224312428969140578915943823395340466419187662119723474059441129216706040810975420885186072698747890615839765847357654198486205747679514159771503546254190608498526147691685468717339503106831214240554923976550226131423821899358941678177012767249757000151516175351605183646515289758127657594483351312820284135270121698193471863249963552248305706529133764410797262033127315996385879310273241411999616601791470886920119787124471456014799082324567146236951447774466096290384104870153132091954155668459053293520897201265795242933843325164440604889046037137996023050079472589183707593322481815544873436357989);
}

#[test]
fn test_14() {
    assert_eq!(Solution::combination_sum4(vec![2, 1, 5], 8), 44);
}

#[test]
fn test_15() {
    assert_eq!(Solution::combination_sum4(vec![15, 25, 5, 50, 100], 100), 4600);
}

#[test]
fn test_16() {
    assert_eq!(Solution::combination_sum4(vec![7, 14], 300), 0);
}

#[test]
fn test_17() {
    assert_eq!(Solution::combination_sum4(vec![1, 2, 5], 50), 237139442616);
}

#[test]
fn test_18() {
    assert_eq!(Solution::combination_sum4(vec![1, 2, 3, 4, 5], 10), 464);
}

#[test]
fn test_19() {
    assert_eq!(Solution::combination_sum4(vec![2, 2, 2, 2, 2, 2, 2, 2, 2, 2], 20), 10000000000);
}

#[test]
fn test_20() {
    assert_eq!(Solution::combination_sum4(vec![4, 11, 3, 4, 1], 21), 46333);
}

#[test]
fn test_21() {
    assert_eq!(Solution::combination_sum4(vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1], 20), 104857600000000000000000000);
}

#[test]
fn test_22() {
    assert_eq!(Solution::combination_sum4(vec![10, 20, 30], 50), 13);
}

#[test]
fn test_23() {
    assert_eq!(Solution::combination_sum4(vec![1, 2, 5], 25), 382396);
}

#[test]
fn test_24() {
    assert_eq!(Solution::combination_sum4(vec![18, 23, 50, 51], 200), 493);
}

#[test]
fn test_25() {
    assert_eq!(Solution::combination_sum4(vec![33, 29, 40, 12, 54, 23, 67, 34, 74], 1000), 1004803376074700418269474690624);
}

#[test]
fn test_26() {
    assert_eq!(Solution::combination_sum4(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 50), 551742150354112);
}

#[test]
fn test_27() {
    assert_eq!(Solution::combination_sum4(vec![1, 2, 3, 4, 5], 50), 256641310658978);
}

#[test]
fn test_28() {
    assert_eq!(Solution::combination_sum4(vec![7, 14, 21, 28], 100), 0);
}

#[test]
fn test_29() {
    assert_eq!(Solution::combination_sum4(vec![1, 2, 4, 8, 16, 32, 64], 256), 1054787931172699885204409659786242855354657339703519197691828406);
}

#[test]
fn test_30() {
    assert_eq!(Solution::combination_sum4(vec![10, 15, 20, 25, 30, 35, 40, 45, 50], 500), 154640480926757717819);
}

#[test]
fn test_31() {
    assert_eq!(Solution::combination_sum4(vec![999, 1000], 1999), 2);
}

#[test]
fn test_32() {
    assert_eq!(Solution::combination_sum4(vec![3, 6, 9, 12], 100), 0);
}

#[test]
fn test_33() {
    assert_eq!(Solution::combination_sum4(vec![2, 5, 10, 20], 30), 417);
}

#[test]
fn test_34() {
    assert_eq!(Solution::combination_sum4(vec![5, 10, 25, 50], 100), 27517);
}

#[test]
fn test_35() {
    assert_eq!(Solution::combination_sum4(vec![1, 2, 4, 8, 16], 32), 47350055);
}

#[test]
fn test_36() {
    assert_eq!(Solution::combination_sum4(vec![10, 20, 30, 40], 100), 401);
}

#[test]
fn test_37() {
    assert_eq!(Solution::combination_sum4(vec![1, 10, 25, 50], 100), 37971048);
}

#[test]
fn test_38() {
    assert_eq!(Solution::combination_sum4(vec![4, 8, 15, 16, 23, 42], 100), 878907);
}

#[test]
fn test_39() {
    assert_eq!(Solution::combination_sum4(vec![1, 2, 3, 4, 5], 200), 27870089767928389254900226744638057842249669417272614584184);
}

#[test]
fn test_40() {
    assert_eq!(Solution::combination_sum4(vec![2, 3, 5], 30), 15778);
}

#[test]
fn test_41() {
    assert_eq!(Solution::combination_sum4(vec![2, 6, 10, 14, 18, 22, 26, 30], 300), 9697818790261047025454697667566);
}

#[test]
fn test_42() {
    assert_eq!(Solution::combination_sum4(vec![1, 2], 100), 573147844013817084101);
}

#[test]
fn test_43() {
    assert_eq!(Solution::combination_sum4(vec![1, 3, 6, 9, 12, 15, 18], 100), 9103419367826082596);
}

#[test]
fn test_44() {
    assert_eq!(Solution::combination_sum4(vec![1, 2, 4, 5], 15), 4185);
}

#[test]
fn test_45() {
    assert_eq!(Solution::combination_sum4(vec![1, 2, 5, 10, 20, 50], 1000), 55418612077363252135298307641829396865446477611253299547057401338234391162164785712545427782637930869475332856306540999148459057137429772224203347607997227631647056712144679002353744573917660746726513112253869324256573236618890333589);
}

#[test]
fn test_46() {
    assert_eq!(Solution::combination_sum4(vec![3, 7, 15], 120), 92796380);
}

#[test]
fn test_47() {
    assert_eq!(Solution::combination_sum4(vec![2, 3, 6, 7], 100), 2724220005886008);
}

#[test]
fn test_48() {
    assert_eq!(Solution::combination_sum4(vec![3, 6, 9, 12, 15, 18, 21, 24, 27, 30], 120), 541461698816);
}

#[test]
fn test_49() {
    assert_eq!(Solution::combination_sum4(vec![7, 14, 21, 28, 35], 100), 0);
}

#[test]
fn test_50() {
    assert_eq!(Solution::combination_sum4(vec![4, 10, 40, 100], 150), 3504031);
}

#[test]
fn test_51() {
    assert_eq!(Solution::combination_sum4(vec![3, 7, 8], 11), 2);
}

#[test]
fn test_52() {
    assert_eq!(Solution::combination_sum4(vec![2, 3, 6, 7, 9], 20), 576);
}

#[test]
fn test_53() {
    assert_eq!(Solution::combination_sum4(vec![2, 3, 7, 8, 10], 200), 3100714643409982098043972616729);
}

#[test]
fn test_54() {
    assert_eq!(Solution::combination_sum4(vec![50, 25, 75, 20, 10], 200), 197308);
}

#[test]
fn test_55() {
    assert_eq!(Solution::combination_sum4(vec![1, 5, 10, 25], 100), 8577828731901);
}

#[test]
fn test_56() {
    assert_eq!(Solution::combination_sum4(vec![3, 33, 333], 1000), 0);
}

#[test]
fn test_57() {
    assert_eq!(Solution::combination_sum4(vec![1, 10, 100, 1000], 1111), 367666448776500415514786890083984627942520976458183669725570752318436882775368470747436);
}

#[test]
fn test_58() {
    assert_eq!(Solution::combination_sum4(vec![2, 3, 6, 7], 12), 26);
}

#[test]
fn test_59() {
    assert_eq!(Solution::combination_sum4(vec![3, 6, 9, 12], 30), 401);
}

#[test]
fn test_60() {
    assert_eq!(Solution::combination_sum4(vec![1, 5, 10], 25), 915);
}

#[test]
fn test_61() {
    assert_eq!(Solution::combination_sum4(vec![2, 3, 6, 7, 10], 300), 1885220157798436983841368320280668830805551392497);
}

#[test]
fn test_62() {
    assert_eq!(Solution::combination_sum4(vec![100, 200, 300, 400, 500], 1000), 464);
}

#[test]
fn test_63() {
    assert_eq!(Solution::combination_sum4(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 500), 1286219641702273366742255356065111897930066144490719649670364060629213518690220561526989033769595304532418065557510304805412593714309986391307281509696);
}

#[test]
fn test_64() {
    assert_eq!(Solution::combination_sum4(vec![3, 7, 11, 15, 19, 23, 27, 31], 300), 14374578728716948696671279);
}

#[test]
fn test_65() {
    assert_eq!(Solution::combination_sum4(vec![5, 8, 12, 16, 20, 24, 28, 32, 36, 40], 500), 42034412043805359239945977920635459);
}

#[test]
fn test_66() {
    assert_eq!(Solution::combination_sum4(vec![2, 5, 10, 20], 100), 11826629775);
}

#[test]
fn test_67() {
    assert_eq!(Solution::combination_sum4(vec![10, 20, 30, 40, 50], 150), 13624);
}

#[test]
fn test_68() {
    assert_eq!(Solution::combination_sum4(vec![1, 2, 5, 10], 100), 119179977936469835383520);
}

#[test]
fn test_69() {
    assert_eq!(Solution::combination_sum4(vec![3, 8, 10], 120), 317567468);
}

#[test]
fn test_70() {
    assert_eq!(Solution::combination_sum4(vec![1, 2, 3, 4, 5], 15), 13624);
}

#[test]
fn test_71() {
    assert_eq!(Solution::combination_sum4(vec![1, 3, 5, 7, 9], 25), 70464);
}

#[test]
fn test_72() {
    assert_eq!(Solution::combination_sum4(vec![33, 39, 45, 51, 60], 150), 29);
}

#[test]
fn test_73() {
    assert_eq!(Solution::combination_sum4(vec![3, 5, 7, 8, 9], 50), 252672);
}

#[test]
fn test_74() {
    assert_eq!(Solution::combination_sum4(vec![3, 6, 9, 12, 15, 18, 21, 24, 27, 30], 100), 0);
}

#[test]
fn test_75() {
    assert_eq!(Solution::combination_sum4(vec![5, 50, 500], 1000), 1816691941426948);
}

#[test]
fn test_76() {
    assert_eq!(Solution::combination_sum4(vec![1, 2, 3, 4, 5], 10), 464);
}

#[test]
fn test_77() {
    assert_eq!(Solution::combination_sum4(vec![5, 10, 15, 20, 25, 30, 35, 40, 45, 50], 250), 551742150354112);
}

#[test]
fn test_78() {
    assert_eq!(Solution::combination_sum4(vec![15, 25, 35, 45], 200), 6259);
}

#[test]
fn test_79() {
    assert_eq!(Solution::combination_sum4(vec![1, 5, 10], 50), 1919938);
}

#[test]
fn test_80() {
    assert_eq!(Solution::combination_sum4(vec![10, 20, 30], 100), 274);
}

#[test]
fn test_81() {
    assert_eq!(Solution::combination_sum4(vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23, 25, 27, 29], 300), 222217726411850455445265344483183426006475036048672509776477817);
}

#[test]
fn test_82() {
    assert_eq!(Solution::combination_sum4(vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], 100), 606147434557459526483161067501);
}

#[test]
fn test_83() {
    assert_eq!(Solution::combination_sum4(vec![50, 150, 250, 350, 450], 1000), 6475);
}

#[test]
fn test_84() {
    assert_eq!(Solution::combination_sum4(vec![50, 25, 75, 125], 250), 331);
}

#[test]
fn test_85() {
    assert_eq!(Solution::combination_sum4(vec![5, 10, 15, 20, 25, 30], 100), 463968);
}

#[test]
fn test_86() {
    assert_eq!(Solution::combination_sum4(vec![1, 5, 10, 25, 50, 100], 1000), 3861392372529553044883315945420265874789274045115117175074863427403559146375333246873046775291634059748511012129487397547119298455362);
}

#[test]
fn test_87() {
    assert_eq!(Solution::combination_sum4(vec![15, 25, 35], 100), 25);
}

#[test]
fn test_88() {
    assert_eq!(Solution::combination_sum4(vec![1, 2, 4, 8, 16], 100), 3001542533296576909276367);
}

#[test]
fn test_89() {
    assert_eq!(Solution::combination_sum4(vec![2, 4, 6, 8, 10], 20), 464);
}

#[test]
fn test_90() {
    assert_eq!(Solution::combination_sum4(vec![1, 3, 4, 5, 6], 20), 22750);
}

#[test]
fn test_91() {
    assert_eq!(Solution::combination_sum4(vec![5, 15, 25, 35, 45, 55, 65, 75, 85, 95], 500), 353368918335207375428);
}

#[test]
fn test_92() {
    assert_eq!(Solution::combination_sum4(vec![1, 3, 4], 10), 64);
}

#[test]
fn test_93() {
    assert_eq!(Solution::combination_sum4(vec![1, 10, 100, 1000], 1000), 753729672939848505929078336767198285353713318174400841581842662144837938547270);
}

#[test]
fn test_94() {
    assert_eq!(Solution::combination_sum4(vec![1, 5, 10, 25], 50), 1931845);
}

#[test]
fn test_95() {
    assert_eq!(Solution::combination_sum4(vec![2, 3, 6, 7], 15), 78);
}

#[test]
fn test_96() {
    assert_eq!(Solution::combination_sum4(vec![1, 3, 4, 5], 13), 424);
}

#[test]
fn test_97() {
    assert_eq!(Solution::combination_sum4(vec![13, 17, 19, 23, 29], 100), 256);
}

#[test]
fn test_98() {
    assert_eq!(Solution::combination_sum4(vec![3, 5, 7, 10], 20), 35);
}

#[test]
fn test_99() {
    assert_eq!(Solution::combination_sum4(vec![2, 3, 6, 7], 30), 19096);
}

#[test]
fn test_100() {
    assert_eq!(Solution::combination_sum4(vec![7, 14, 28], 1000), 0);
}

#[test]
fn test_101() {
    assert_eq!(Solution::combination_sum4(vec![2, 3, 6, 7, 9], 120), 14544576409221128865);
}

#[test]
fn test_102() {
    assert_eq!(Solution::combination_sum4(vec![7, 14, 21, 28, 35], 105), 13624);
}

#[test]
fn test_103() {
    assert_eq!(Solution::combination_sum4(vec![7, 14, 28, 35, 56], 100), 0);
}

#[test]
fn test_104() {
    assert_eq!(Solution::combination_sum4(vec![4, 7, 9], 20), 7);
}

#[test]
fn test_105() {
    assert_eq!(Solution::combination_sum4(vec![3, 5, 10, 15], 50), 12043);
}

#[test]
fn test_106() {
    assert_eq!(Solution::combination_sum4(vec![15, 25, 35], 1000), 48781528629177572921);
}

#[test]
fn test_107() {
    assert_eq!(Solution::combination_sum4(vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19], 50), 12574441114);
}

#[test]
fn test_108() {
    assert_eq!(Solution::combination_sum4(vec![3, 5, 7], 15), 8);
}

#[test]
fn test_109() {
    assert_eq!(Solution::combination_sum4(vec![1, 2, 4, 8, 16, 32, 64, 128], 256), 1054787931172699885204409659788147413348784265452313995416385159);
}

#[test]
fn test_110() {
    assert_eq!(Solution::combination_sum4(vec![3, 5, 10, 20], 1000), 214391693407662746979155838789092372951985494367397337570037572730823254282910703400057102167);
}

#[test]
fn test_111() {
    assert_eq!(Solution::combination_sum4(vec![1, 5, 10, 25, 50, 100], 200), 169121161302579810731290745);
}

#[test]
fn test_112() {
    assert_eq!(Solution::combination_sum4(vec![3, 5, 7, 9], 150), 12883204547325702);
}

#[test]
fn test_113() {
    assert_eq!(Solution::combination_sum4(vec![4, 10, 40, 25], 100), 18984);
}

#[test]
fn test_114() {
    assert_eq!(Solution::combination_sum4(vec![1, 2, 4, 8, 16], 31), 26805983);
}

#[test]
fn test_115() {
    assert_eq!(Solution::combination_sum4(vec![10, 20, 30, 40, 50], 200), 400096);
}

#[test]
fn test_116() {
    assert_eq!(Solution::combination_sum4(vec![1, 100, 101, 102], 300), 45158);
}
