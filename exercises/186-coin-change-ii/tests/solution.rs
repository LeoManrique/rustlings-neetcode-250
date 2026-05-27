include!("../src/lib.rs");

#[test]
fn test_1() {
    assert_eq!(Solution::change(20, vec![1, 5, 10]), 9);
}

#[test]
fn test_2() {
    assert_eq!(Solution::change(1, vec![1]), 1);
}

#[test]
fn test_3() {
    assert_eq!(Solution::change(0, vec![1, 2, 5]), 1);
}

#[test]
fn test_4() {
    assert_eq!(Solution::change(5000, vec![1, 5, 10, 25, 50]), 432699251);
}

#[test]
fn test_5() {
    assert_eq!(Solution::change(100, vec![3, 7, 40, 9]), 48);
}

#[test]
fn test_6() {
    assert_eq!(Solution::change(6249, vec![186, 419, 83, 408]), 19);
}

#[test]
fn test_7() {
    assert_eq!(Solution::change(15, vec![1, 3, 5, 7]), 19);
}

#[test]
fn test_8() {
    assert_eq!(Solution::change(10, vec![1, 3, 4, 5]), 12);
}

#[test]
fn test_9() {
    assert_eq!(Solution::change(10, vec![10]), 1);
}

#[test]
fn test_10() {
    assert_eq!(Solution::change(50, vec![1, 18, 27, 34, 50]), 7);
}

#[test]
fn test_11() {
    assert_eq!(Solution::change(5000, vec![3, 5, 7, 8, 9, 10]), 351757492460);
}

#[test]
fn test_12() {
    assert_eq!(Solution::change(100, vec![3, 6, 9]), 0);
}

#[test]
fn test_13() {
    assert_eq!(Solution::change(3, vec![2]), 0);
}

#[test]
fn test_14() {
    assert_eq!(Solution::change(25, vec![1, 2, 5]), 42);
}

#[test]
fn test_15() {
    assert_eq!(Solution::change(5, vec![1, 2, 5]), 4);
}

#[test]
fn test_16() {
    assert_eq!(Solution::change(100, vec![1, 2, 5, 10, 20, 50]), 4562);
}

#[test]
fn test_17() {
    assert_eq!(Solution::change(1000, vec![3, 5, 7, 8, 9, 10, 11]), 1952879228);
}

#[test]
fn test_18() {
    assert_eq!(Solution::change(3000, vec![4, 12, 23, 34, 51]), 1913606);
}

#[test]
fn test_19() {
    assert_eq!(Solution::change(1800, vec![1, 2, 4, 8, 16, 32, 64, 128, 256, 512]), 120836624716);
}

#[test]
fn test_20() {
    assert_eq!(Solution::change(3500, vec![50, 100, 150, 200, 250, 300, 350]), 94425);
}

#[test]
fn test_21() {
    assert_eq!(Solution::change(2500, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]), 0);
}

#[test]
fn test_22() {
    assert_eq!(Solution::change(2500, vec![13, 17, 19, 23, 29, 31]), 10673219);
}

#[test]
fn test_23() {
    assert_eq!(Solution::change(4500, vec![23, 47, 61, 83, 97, 101]), 358270);
}

#[test]
fn test_24() {
    assert_eq!(Solution::change(4200, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]), 0);
}

#[test]
fn test_25() {
    assert_eq!(Solution::change(4000, vec![3, 9, 18, 36, 72, 144]), 0);
}

#[test]
fn test_26() {
    assert_eq!(Solution::change(8000, vec![3, 6, 9, 12, 15, 18, 21, 24, 27, 30]), 0);
}

#[test]
fn test_27() {
    assert_eq!(Solution::change(3333, vec![3, 6, 9, 12, 15, 18, 21, 24, 27]), 186150339220814);
}

#[test]
fn test_28() {
    assert_eq!(Solution::change(1200, vec![2, 5, 10, 25, 50, 100]), 2459925);
}

#[test]
fn test_29() {
    assert_eq!(Solution::change(4500, vec![11, 22, 33, 44, 55]), 0);
}

#[test]
fn test_30() {
    assert_eq!(Solution::change(2000, vec![11, 17, 29, 31]), 8465);
}

#[test]
fn test_31() {
    assert_eq!(Solution::change(3000, vec![4, 9, 15, 25, 30, 50, 75]), 816870821);
}

#[test]
fn test_32() {
    assert_eq!(Solution::change(3750, vec![3, 6, 9, 12, 15, 18, 21, 24, 27, 30]), 6877248457909551);
}

#[test]
fn test_33() {
    assert_eq!(Solution::change(4800, vec![1, 4, 9, 16, 25, 36, 49, 64, 81, 100]), 402020904793077);
}

#[test]
fn test_34() {
    assert_eq!(Solution::change(3500, vec![1, 2, 5, 10, 20, 50, 100]), 298945088016);
}

#[test]
fn test_35() {
    assert_eq!(Solution::change(10000, vec![2, 3, 5, 7, 11, 13, 17, 19, 23]), 11569288968418829417);
}

#[test]
fn test_36() {
    assert_eq!(Solution::change(3750, vec![125, 250, 500, 1000, 2000]), 166);
}

#[test]
fn test_37() {
    assert_eq!(Solution::change(2750, vec![4, 9, 14, 19, 24, 29, 34, 39, 44, 49]), 1980385268761);
}

#[test]
fn test_38() {
    assert_eq!(Solution::change(6000, vec![1, 2, 4, 8, 16, 32, 64, 128, 256, 512]), 1608946754650872);
}

#[test]
fn test_39() {
    assert_eq!(Solution::change(4500, vec![2, 5, 10, 20, 50, 100, 200, 500, 1000, 2000]), 17776167706);
}

#[test]
fn test_40() {
    assert_eq!(Solution::change(4500, vec![5, 10, 20, 50, 100, 200]), 94862488);
}

#[test]
fn test_41() {
    assert_eq!(Solution::change(6000, vec![11, 22, 33, 44, 55, 66]), 0);
}

#[test]
fn test_42() {
    assert_eq!(Solution::change(5555, vec![7, 14, 21, 28, 35, 42, 49, 56, 63]), 0);
}

#[test]
fn test_43() {
    assert_eq!(Solution::change(2048, vec![2, 4, 8, 16, 32, 64, 128, 256, 512, 1024]), 2320518947);
}

#[test]
fn test_44() {
    assert_eq!(Solution::change(4000, vec![1, 11, 13, 19, 21, 31, 37]), 95941704933);
}

#[test]
fn test_45() {
    assert_eq!(Solution::change(2700, vec![3, 6, 9, 12, 15, 18, 21, 24, 27, 30]), 385226325096527);
}

#[test]
fn test_46() {
    assert_eq!(Solution::change(4999, vec![1, 2, 5, 10, 25, 50, 100, 200, 500]), 14978475244405);
}

#[test]
fn test_47() {
    assert_eq!(Solution::change(700, vec![3, 7, 11, 13, 17, 19, 23, 29]), 45005473);
}

#[test]
fn test_48() {
    assert_eq!(Solution::change(4500, vec![1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024, 2048]), 171076578684392);
}

#[test]
fn test_49() {
    assert_eq!(Solution::change(2500, vec![2, 3, 5, 7, 11, 13]), 28227697415);
}

#[test]
fn test_50() {
    assert_eq!(Solution::change(500, vec![3, 6, 9, 12, 15, 18, 21, 24, 27, 30]), 0);
}

#[test]
fn test_51() {
    assert_eq!(Solution::change(3500, vec![7, 11, 13, 17, 19, 23, 29, 31, 37, 41, 43, 47]), 1916917910649679);
}

#[test]
fn test_52() {
    assert_eq!(Solution::change(500, vec![11, 22, 33, 44, 55]), 0);
}

#[test]
fn test_53() {
    assert_eq!(Solution::change(8000, vec![1, 2, 5, 10, 20, 50, 100]), 39042657518121);
}

#[test]
fn test_54() {
    assert_eq!(Solution::change(1500, vec![3, 5, 7, 11, 13, 17, 19, 23]), 38017360391);
}

#[test]
fn test_55() {
    assert_eq!(Solution::change(3500, vec![17, 23, 29, 31, 37]), 519429);
}

#[test]
fn test_56() {
    assert_eq!(Solution::change(2700, vec![3, 6, 9, 12, 15, 18, 21, 24, 27]), 35818551286815);
}

#[test]
fn test_57() {
    assert_eq!(Solution::change(1500, vec![50, 200, 500, 1000]), 22);
}

#[test]
fn test_58() {
    assert_eq!(Solution::change(500, vec![11, 19, 29, 37, 47, 59, 71, 83]), 3292);
}

#[test]
fn test_59() {
    assert_eq!(Solution::change(2000, vec![1, 3, 7, 11]), 5867703);
}

#[test]
fn test_60() {
    assert_eq!(Solution::change(3000, vec![15, 25, 35, 45, 55]), 582580);
}

#[test]
fn test_61() {
    assert_eq!(Solution::change(5000, vec![1, 4, 9, 16, 25, 36, 49]), 928443384056);
}

#[test]
fn test_62() {
    assert_eq!(Solution::change(2500, vec![7, 17, 27, 37, 47, 57, 67, 77]), 115681732);
}

#[test]
fn test_63() {
    assert_eq!(Solution::change(2000, vec![1, 3, 4, 12, 24, 48]), 1799991438);
}

#[test]
fn test_64() {
    assert_eq!(Solution::change(3000, vec![2, 5, 10, 20, 50, 100]), 238303231);
}

#[test]
fn test_65() {
    assert_eq!(Solution::change(5000, vec![20, 25, 50, 100, 200, 500]), 831886);
}

#[test]
fn test_66() {
    assert_eq!(Solution::change(4000, vec![2, 4, 6, 8, 10, 12, 14, 16, 18]), 19131722793601739);
}

#[test]
fn test_67() {
    assert_eq!(Solution::change(3000, vec![33, 66, 99, 132, 165, 198, 231, 264, 297]), 0);
}

#[test]
fn test_68() {
    assert_eq!(Solution::change(2222, vec![2, 4, 6, 8, 10, 12, 14, 16, 18]), 186150339220814);
}

#[test]
fn test_69() {
    assert_eq!(Solution::change(200, vec![1, 3, 5, 7, 9]), 89740);
}

#[test]
fn test_70() {
    assert_eq!(Solution::change(4567, vec![2, 5, 10, 20, 50]), 188174063);
}

#[test]
fn test_71() {
    assert_eq!(Solution::change(800, vec![10, 20, 50, 100, 200, 500]), 2064);
}

#[test]
fn test_72() {
    assert_eq!(Solution::change(3333, vec![13, 23, 33, 43, 53, 63, 73, 83, 93, 103]), 3570286683);
}

#[test]
fn test_73() {
    assert_eq!(Solution::change(2000, vec![10, 25, 50, 100, 200, 500]), 29674);
}

#[test]
fn test_74() {
    assert_eq!(Solution::change(1500, vec![5, 11, 17, 23, 29]), 378081);
}

#[test]
fn test_75() {
    assert_eq!(Solution::change(6000, vec![1, 2, 3, 5, 11, 23, 37, 41]), 5179949524521356);
}

#[test]
fn test_76() {
    assert_eq!(Solution::change(3000, vec![1, 10, 25, 50, 100, 200]), 11051256);
}

#[test]
fn test_77() {
    assert_eq!(Solution::change(4999, vec![1, 3, 9, 27, 81, 243, 729, 2187, 6561]), 3946672836);
}

#[test]
fn test_78() {
    assert_eq!(Solution::change(1200, vec![4, 8, 15, 16, 23, 42]), 3481694);
}

#[test]
fn test_79() {
    assert_eq!(Solution::change(999, vec![1, 4, 9, 16, 25, 36, 49, 64, 81]), 528165615);
}

#[test]
fn test_80() {
    assert_eq!(Solution::change(1111, vec![1, 4, 9, 16, 25, 36, 49, 64, 81]), 1122054398);
}

#[test]
fn test_81() {
    assert_eq!(Solution::change(6666, vec![1, 2, 3, 5, 8, 13, 21, 34, 55]), 858790214643388609);
}

#[test]
fn test_82() {
    assert_eq!(Solution::change(4000, vec![3, 5, 7, 9, 11, 13, 15]), 2941614132296);
}

#[test]
fn test_83() {
    assert_eq!(Solution::change(1234, vec![13, 21, 34, 55, 89, 144, 233, 377, 610, 987]), 10948);
}

#[test]
fn test_84() {
    assert_eq!(Solution::change(3500, vec![1, 2, 4, 8, 16, 32, 64]), 1356535733168);
}

#[test]
fn test_85() {
    assert_eq!(Solution::change(1500, vec![1, 4, 9, 16, 25, 36, 49, 64, 81, 100]), 23221589514);
}

#[test]
fn test_86() {
    assert_eq!(Solution::change(1200, vec![5, 15, 25, 50, 100, 200]), 115862);
}

#[test]
fn test_87() {
    assert_eq!(Solution::change(3000, vec![1, 3, 4, 10, 20, 50]), 18144427661);
}

#[test]
fn test_88() {
    assert_eq!(Solution::change(3000, vec![2, 5, 11, 23, 47]), 30080239);
}

#[test]
fn test_89() {
    assert_eq!(Solution::change(3456, vec![2, 3, 5, 7, 11, 13, 17, 19, 23, 29]), 35362292367933393);
}

#[test]
fn test_90() {
    assert_eq!(Solution::change(800, vec![5, 10, 20, 50, 100, 200]), 38835);
}

#[test]
fn test_91() {
    assert_eq!(Solution::change(1234, vec![1, 2, 5, 10, 20, 50, 100]), 754797589);
}

#[test]
fn test_92() {
    assert_eq!(Solution::change(4000, vec![11, 22, 33, 44, 55]), 0);
}

#[test]
fn test_93() {
    assert_eq!(Solution::change(2000, vec![7, 11, 13, 17, 19, 23]), 40067247);
}

#[test]
fn test_94() {
    assert_eq!(Solution::change(1800, vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19]), 1067122993784732);
}

#[test]
fn test_95() {
    assert_eq!(Solution::change(7000, vec![1, 5, 10, 25, 50, 100, 200, 500]), 399576201827);
}

#[test]
fn test_96() {
    assert_eq!(Solution::change(4000, vec![8, 16, 24, 32, 40, 48, 56]), 5078412464);
}

#[test]
fn test_97() {
    assert_eq!(Solution::change(1500, vec![2, 5, 11, 25, 40, 60]), 12079534);
}

#[test]
fn test_98() {
    assert_eq!(Solution::change(1800, vec![2, 6, 12, 18, 24, 30]), 192387751);
}

#[test]
fn test_99() {
    assert_eq!(Solution::change(1500, vec![7, 11, 13, 17, 19]), 712518);
}

#[test]
fn test_100() {
    assert_eq!(Solution::change(3500, vec![5, 15, 25, 35, 45, 55, 65, 75]), 10998232465);
}

#[test]
fn test_101() {
    assert_eq!(Solution::change(1234, vec![3, 5, 7, 11, 13, 17, 19]), 1209189166);
}

#[test]
fn test_102() {
    assert_eq!(Solution::change(750, vec![7, 14, 21, 28, 35, 42]), 0);
}

#[test]
fn test_103() {
    assert_eq!(Solution::change(2000, vec![1, 3, 5, 7, 9, 11, 13]), 707531982781);
}

#[test]
fn test_104() {
    assert_eq!(Solution::change(4000, vec![5, 11, 17, 23, 29, 35, 41, 47, 53, 59]), 7799502297011);
}

#[test]
fn test_105() {
    assert_eq!(Solution::change(4000, vec![5, 10, 25, 50, 100]), 9370181);
}

#[test]
fn test_106() {
    assert_eq!(Solution::change(1800, vec![15, 25, 35, 45, 55, 65, 75, 85, 95, 105]), 79077323);
}

#[test]
fn test_107() {
    assert_eq!(Solution::change(7777, vec![13, 26, 39, 52, 65, 78, 91, 104, 117]), 0);
}

#[test]
fn test_108() {
    assert_eq!(Solution::change(9000, vec![15, 25, 35, 45, 55, 65]), 1244874855);
}

#[test]
fn test_109() {
    assert_eq!(Solution::change(4000, vec![7, 14, 21, 28, 35]), 0);
}

#[test]
fn test_110() {
    assert_eq!(Solution::change(4321, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15]), 8412107116918783059141522248);
}

#[test]
fn test_111() {
    assert_eq!(Solution::change(2000, vec![101, 103, 107, 109]), 0);
}

#[test]
fn test_112() {
    assert_eq!(Solution::change(2500, vec![2, 5, 11, 17, 23, 29, 35, 41, 47]), 623719935720);
}

#[test]
fn test_113() {
    assert_eq!(Solution::change(500, vec![1, 2, 5, 10, 20, 50, 100]), 5824071);
}

#[test]
fn test_114() {
    assert_eq!(Solution::change(5000, vec![1, 2, 5, 10, 20, 50, 100, 200, 500]), 18682149631801);
}

#[test]
fn test_115() {
    assert_eq!(Solution::change(2000, vec![1, 2, 4, 8, 16, 32, 64, 128, 256, 512, 1024]), 264830889564);
}

#[test]
fn test_116() {
    assert_eq!(Solution::change(4500, vec![7, 13, 19, 23, 37, 41]), 275308289);
}

#[test]
fn test_117() {
    assert_eq!(Solution::change(8888, vec![11, 22, 33, 44, 55, 66, 77, 88, 99]), 15453068500547);
}

#[test]
fn test_118() {
    assert_eq!(Solution::change(1500, vec![7, 14, 21, 28, 35, 42, 49, 56, 63, 70]), 0);
}

#[test]
fn test_119() {
    assert_eq!(Solution::change(7000, vec![3, 5, 7, 11, 13, 17, 19]), 34788754338474);
}

#[test]
fn test_120() {
    assert_eq!(Solution::change(3000, vec![1, 3, 5, 10, 25, 50]), 11669586691);
}

#[test]
fn test_121() {
    assert_eq!(Solution::change(1999, vec![111, 222, 333, 444, 555, 666, 777, 888]), 0);
}

#[test]
fn test_122() {
    assert_eq!(Solution::change(999, vec![1, 2, 4, 8, 16, 32, 64]), 944362512);
}

#[test]
fn test_123() {
    assert_eq!(Solution::change(6000, vec![3, 6, 12, 24, 48, 96]), 8807842176);
}

#[test]
fn test_124() {
    assert_eq!(Solution::change(5000, vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19, 21, 23, 25, 27, 29, 31, 33, 35, 37, 39]), 1027087367648016934506457070508);
}
