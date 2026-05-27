include!("../src/lib.rs");

#[test]
fn test_1() {
    assert_eq!(Solution::find_maximized_capital(1, 10, vec![5, 6], vec![4, 0]), 16);
}

#[test]
fn test_2() {
    assert_eq!(Solution::find_maximized_capital(5, 10, vec![5, 4, 3, 2, 1], vec![1, 2, 3, 4, 5]), 25);
}

#[test]
fn test_3() {
    assert_eq!(Solution::find_maximized_capital(2, 3, vec![8, 10, 6], vec![2, 5, 0]), 21);
}

#[test]
fn test_4() {
    assert_eq!(Solution::find_maximized_capital(4, 0, vec![1, 1, 1, 1], vec![0, 0, 0, 0]), 4);
}

#[test]
fn test_5() {
    assert_eq!(Solution::find_maximized_capital(5, 0, vec![1, 2, 3, 4, 5], vec![0, 1, 2, 3, 4]), 15);
}

#[test]
fn test_6() {
    assert_eq!(Solution::find_maximized_capital(5, 0, vec![10, 20, 30, 40, 50], vec![0, 0, 0, 0, 0]), 150);
}

#[test]
fn test_7() {
    assert_eq!(Solution::find_maximized_capital(1, 10, vec![1, 2, 3], vec![5, 5, 5]), 13);
}

#[test]
fn test_8() {
    assert_eq!(Solution::find_maximized_capital(1, 5, vec![1, 2, 3], vec![3, 4, 5]), 8);
}

#[test]
fn test_9() {
    assert_eq!(Solution::find_maximized_capital(3, 0, vec![1, 2, 3], vec![0, 1, 2]), 6);
}

#[test]
fn test_10() {
    assert_eq!(Solution::find_maximized_capital(3, 0, vec![10, 20, 30], vec![0, 0, 0]), 60);
}

#[test]
fn test_11() {
    assert_eq!(Solution::find_maximized_capital(2, 1, vec![1, 2, 3], vec![1, 1, 2]), 6);
}

#[test]
fn test_12() {
    assert_eq!(Solution::find_maximized_capital(1, 100, vec![1000, 2000, 3000], vec![1000, 2000, 3000]), 100);
}

#[test]
fn test_13() {
    assert_eq!(Solution::find_maximized_capital(3, 3, vec![1, 2, 3, 4, 5], vec![0, 1, 2, 3, 4]), 15);
}

#[test]
fn test_14() {
    assert_eq!(Solution::find_maximized_capital(2, 0, vec![1, 2, 3], vec![0, 1, 1]), 4);
}

#[test]
fn test_15() {
    assert_eq!(Solution::find_maximized_capital(10, 1, vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1], vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 11);
}

#[test]
fn test_16() {
    assert_eq!(Solution::find_maximized_capital(2, 3, vec![5, 6, 4, 3, 2], vec![2, 3, 4, 5, 6]), 14);
}

#[test]
fn test_17() {
    assert_eq!(Solution::find_maximized_capital(3, 2, vec![5, 4, 3, 2, 1], vec![5, 4, 3, 2, 1]), 13);
}

#[test]
fn test_18() {
    assert_eq!(Solution::find_maximized_capital(2, 5, vec![100, 200, 300, 400, 500], vec![0, 0, 0, 0, 0]), 905);
}

#[test]
fn test_19() {
    assert_eq!(Solution::find_maximized_capital(5, 20, vec![100, 200, 300, 400, 500, 600, 700, 800, 900, 1000], vec![0, 10, 20, 30, 40, 50, 60, 70, 80, 90]), 3720);
}

#[test]
fn test_20() {
    assert_eq!(Solution::find_maximized_capital(6, 1, vec![1, 2, 3, 4, 5, 6, 7], vec![0, 1, 2, 3, 4, 5, 6]), 28);
}

#[test]
fn test_21() {
    assert_eq!(Solution::find_maximized_capital(3, 0, vec![10, 10, 10, 10, 10], vec![0, 0, 0, 0, 0]), 30);
}

#[test]
fn test_22() {
    assert_eq!(Solution::find_maximized_capital(5, 100, vec![10, 20, 30, 40, 50], vec![10, 20, 30, 40, 50]), 250);
}

#[test]
fn test_23() {
    assert_eq!(Solution::find_maximized_capital(4, 0, vec![100, 200, 300, 400, 500], vec![0, 1, 2, 3, 4]), 1300);
}

#[test]
fn test_24() {
    assert_eq!(Solution::find_maximized_capital(1, 1, vec![100, 200, 300, 400, 500], vec![0, 0, 0, 0, 0]), 501);
}

#[test]
fn test_25() {
    assert_eq!(Solution::find_maximized_capital(10, 10000, vec![1000, 2000, 3000, 4000, 5000, 6000, 7000, 8000, 9000, 10000], vec![0, 1000, 2000, 3000, 4000, 5000, 6000, 7000, 8000, 9000]), 65000);
}

#[test]
fn test_26() {
    assert_eq!(Solution::find_maximized_capital(4, 0, vec![10, 20, 30, 40], vec![0, 10, 20, 30]), 100);
}

#[test]
fn test_27() {
    assert_eq!(Solution::find_maximized_capital(20, 1000, vec![100, 200, 300, 400, 500, 600, 700, 800, 900, 1000, 1100, 1200, 1300, 1400, 1500, 1600, 1700, 1800, 1900, 2000], vec![100, 200, 300, 400, 500, 600, 700, 800, 900, 1000, 1100, 1200, 1300, 1400, 1500, 1600, 1700, 1800, 1900, 2000]), 22000);
}

#[test]
fn test_28() {
    assert_eq!(Solution::find_maximized_capital(3, 1, vec![10, 20, 30, 40, 50], vec![1, 2, 3, 4, 5]), 101);
}

#[test]
fn test_29() {
    assert_eq!(Solution::find_maximized_capital(5, 0, vec![1, 10, 100, 1000, 10000], vec![0, 0, 0, 0, 0]), 11111);
}

#[test]
fn test_30() {
    assert_eq!(Solution::find_maximized_capital(4, 10, vec![100, 200, 300, 400], vec![50, 50, 50, 50]), 10);
}

#[test]
fn test_31() {
    assert_eq!(Solution::find_maximized_capital(3, 50, vec![100, 200, 300, 400, 500], vec![50, 50, 50, 50, 50]), 1250);
}

#[test]
fn test_32() {
    assert_eq!(Solution::find_maximized_capital(5, 0, vec![100, 200, 300, 400, 500, 600, 700, 800, 900, 1000], vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0]), 4000);
}

#[test]
fn test_33() {
    assert_eq!(Solution::find_maximized_capital(7, 50, vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100], vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100]), 540);
}

#[test]
fn test_34() {
    assert_eq!(Solution::find_maximized_capital(7, 5, vec![5, 10, 15, 20, 25, 30, 35, 40], vec![0, 1, 2, 3, 4, 5, 6, 7]), 180);
}

#[test]
fn test_35() {
    assert_eq!(Solution::find_maximized_capital(10, 50000, vec![10000, 20000, 30000, 40000, 50000, 60000, 70000, 80000, 90000, 100000], vec![1000, 2000, 3000, 4000, 5000, 6000, 7000, 8000, 9000, 10000]), 600000);
}

#[test]
fn test_36() {
    assert_eq!(Solution::find_maximized_capital(3, 3, vec![1, 1, 1, 1, 1], vec![0, 1, 2, 3, 4]), 6);
}

#[test]
fn test_37() {
    assert_eq!(Solution::find_maximized_capital(10, 5, vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1], vec![5, 5, 5, 5, 5, 5, 5, 5, 5, 5]), 15);
}

#[test]
fn test_38() {
    assert_eq!(Solution::find_maximized_capital(5, 5, vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100], vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]), 405);
}

#[test]
fn test_39() {
    assert_eq!(Solution::find_maximized_capital(1, 0, vec![10000, 9000, 8000, 7000, 6000], vec![0, 1000, 2000, 3000, 4000]), 10000);
}

#[test]
fn test_40() {
    assert_eq!(Solution::find_maximized_capital(4, 1500, vec![500, 1000, 1500, 2000, 2500, 3000, 3500, 4000, 4500], vec![1000, 1200, 1400, 1600, 1800, 2000, 2200, 2400, 2600]), 15000);
}

#[test]
fn test_41() {
    assert_eq!(Solution::find_maximized_capital(10, 1, vec![100, 200, 300, 400, 500, 600, 700, 800, 900, 1000], vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1]), 5501);
}

#[test]
fn test_42() {
    assert_eq!(Solution::find_maximized_capital(5, 1, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], vec![100, 200, 300, 400, 500, 600, 700, 800, 900, 1000]), 1);
}

#[test]
fn test_43() {
    assert_eq!(Solution::find_maximized_capital(5, 1, vec![1, 2, 3, 4, 5], vec![0, 1, 2, 3, 4]), 16);
}

#[test]
fn test_44() {
    assert_eq!(Solution::find_maximized_capital(10, 0, vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1], vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0]), 10);
}

#[test]
fn test_45() {
    assert_eq!(Solution::find_maximized_capital(10, 100, vec![100, 200, 300, 400, 500, 600, 700, 800, 900, 1000], vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100]), 5600);
}

#[test]
fn test_46() {
    assert_eq!(Solution::find_maximized_capital(7, 5, vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100], vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]), 495);
}

#[test]
fn test_47() {
    assert_eq!(Solution::find_maximized_capital(10, 5, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]), 60);
}

#[test]
fn test_48() {
    assert_eq!(Solution::find_maximized_capital(3, 10, vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100], vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 280);
}

#[test]
fn test_49() {
    assert_eq!(Solution::find_maximized_capital(3, 15, vec![5, 10, 15, 20, 25], vec![5, 10, 15, 20, 25]), 75);
}

#[test]
fn test_50() {
    assert_eq!(Solution::find_maximized_capital(3, 20, vec![10, 10, 10, 10, 10], vec![15, 15, 15, 15, 15]), 50);
}

#[test]
fn test_51() {
    assert_eq!(Solution::find_maximized_capital(5, 1000, vec![5000, 1000, 2000, 1500, 3000, 4000, 6000, 7000, 8000, 9000], vec![0, 500, 1000, 1500, 2000, 2500, 3000, 3500, 4000, 4500]), 36000);
}

#[test]
fn test_52() {
    assert_eq!(Solution::find_maximized_capital(15, 10, vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 110, 120, 130, 140, 150], vec![5, 10, 15, 20, 25, 30, 35, 40, 45, 50, 55, 60, 65, 70, 75]), 1210);
}

#[test]
fn test_53() {
    assert_eq!(Solution::find_maximized_capital(5, 0, vec![500, 400, 300, 200, 100], vec![100, 200, 300, 400, 500]), 0);
}

#[test]
fn test_54() {
    assert_eq!(Solution::find_maximized_capital(4, 10, vec![5, 2, 10, 8, 1], vec![3, 4, 1, 10, 5]), 35);
}

#[test]
fn test_55() {
    assert_eq!(Solution::find_maximized_capital(3, 1, vec![5, 4, 3, 2, 1], vec![0, 1, 2, 3, 4]), 13);
}

#[test]
fn test_56() {
    assert_eq!(Solution::find_maximized_capital(4, 2, vec![1, 2, 3, 4, 5, 6, 7, 8], vec![0, 1, 2, 3, 4, 5, 6, 7]), 26);
}

#[test]
fn test_57() {
    assert_eq!(Solution::find_maximized_capital(5, 10, vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1], vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10]), 15);
}

#[test]
fn test_58() {
    assert_eq!(Solution::find_maximized_capital(7, 0, vec![100, 200, 300, 400, 500, 600, 700, 800, 900, 1000], vec![100, 200, 300, 400, 500, 600, 700, 800, 900, 1000]), 0);
}

#[test]
fn test_59() {
    assert_eq!(Solution::find_maximized_capital(3, 1000, vec![100, 200, 300, 400, 500, 600, 700, 800, 900, 1000], vec![1000, 2000, 3000, 4000, 5000, 6000, 7000, 8000, 9000, 10000]), 1100);
}

#[test]
fn test_60() {
    assert_eq!(Solution::find_maximized_capital(3, 0, vec![10, 20, 30, 40, 50], vec![0, 0, 0, 0, 0]), 120);
}

#[test]
fn test_61() {
    assert_eq!(Solution::find_maximized_capital(3, 5, vec![10, 15, 20, 25, 30], vec![0, 0, 0, 0, 0]), 80);
}

#[test]
fn test_62() {
    assert_eq!(Solution::find_maximized_capital(3, 100, vec![50, 100, 150, 200, 250, 300, 350, 400, 450, 500], vec![50, 50, 50, 50, 50, 50, 50, 50, 50, 50]), 1450);
}

#[test]
fn test_63() {
    assert_eq!(Solution::find_maximized_capital(1, 0, vec![10000, 20000, 30000], vec![10000, 20000, 30000]), 0);
}

#[test]
fn test_64() {
    assert_eq!(Solution::find_maximized_capital(5, 1000, vec![500, 750, 1000, 1250, 1500], vec![250, 500, 750, 1000, 1250]), 6000);
}

#[test]
fn test_65() {
    assert_eq!(Solution::find_maximized_capital(4, 10, vec![5, 5, 5, 5, 5, 5], vec![0, 0, 0, 0, 0, 0]), 30);
}

#[test]
fn test_66() {
    assert_eq!(Solution::find_maximized_capital(3, 20, vec![100, 200, 300, 400, 500], vec![10, 20, 30, 40, 50]), 1120);
}

#[test]
fn test_67() {
    assert_eq!(Solution::find_maximized_capital(10, 100, vec![5, 10, 15, 20, 25, 30, 35, 40, 45, 50], vec![0, 10, 20, 30, 40, 50, 60, 70, 80, 90]), 375);
}

#[test]
fn test_68() {
    assert_eq!(Solution::find_maximized_capital(10, 10, vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100], vec![5, 10, 15, 20, 25, 30, 35, 40, 45, 50]), 560);
}

#[test]
fn test_69() {
    assert_eq!(Solution::find_maximized_capital(4, 10, vec![5, 8, 7, 10, 9, 12], vec![0, 5, 4, 9, 8, 10]), 49);
}

#[test]
fn test_70() {
    assert_eq!(Solution::find_maximized_capital(3, 10, vec![50, 40, 30, 20, 10], vec![5, 10, 15, 20, 25]), 130);
}

#[test]
fn test_71() {
    assert_eq!(Solution::find_maximized_capital(5, 20, vec![100, 200, 300, 400, 500, 600], vec![150, 250, 350, 450, 550, 650]), 20);
}

#[test]
fn test_72() {
    assert_eq!(Solution::find_maximized_capital(2, 0, vec![5, 5, 5, 5, 5, 5], vec![1, 2, 3, 4, 5, 6]), 0);
}

#[test]
fn test_73() {
    assert_eq!(Solution::find_maximized_capital(3, 5, vec![1, 3, 2, 5, 4], vec![0, 2, 3, 6, 4]), 17);
}

#[test]
fn test_74() {
    assert_eq!(Solution::find_maximized_capital(3, 10, vec![5, 4, 3, 2, 1], vec![10, 20, 30, 40, 50]), 15);
}

#[test]
fn test_75() {
    assert_eq!(Solution::find_maximized_capital(6, 5, vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1], vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1, 1]), 11);
}

#[test]
fn test_76() {
    assert_eq!(Solution::find_maximized_capital(7, 5, vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100], vec![0, 5, 10, 15, 20, 25, 30, 35, 40, 45]), 475);
}

#[test]
fn test_77() {
    assert_eq!(Solution::find_maximized_capital(3, 5, vec![6, 7, 8, 9, 10, 11], vec![0, 1, 2, 3, 4, 5]), 35);
}

#[test]
fn test_78() {
    assert_eq!(Solution::find_maximized_capital(10, 0, vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1], vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1]), 0);
}

#[test]
fn test_79() {
    assert_eq!(Solution::find_maximized_capital(2, 100, vec![50, 75, 25, 100, 125], vec![0, 100, 50, 200, 150]), 300);
}

#[test]
fn test_80() {
    assert_eq!(Solution::find_maximized_capital(5, 5, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100]), 5);
}

#[test]
fn test_81() {
    assert_eq!(Solution::find_maximized_capital(2, 3, vec![1, 2, 3, 4, 5], vec![2, 3, 4, 5, 6]), 9);
}

#[test]
fn test_82() {
    assert_eq!(Solution::find_maximized_capital(5, 100, vec![10, 20, 30, 40, 50], vec![50, 50, 50, 50, 50]), 250);
}

#[test]
fn test_83() {
    assert_eq!(Solution::find_maximized_capital(3, 50, vec![100, 200, 300, 400, 500], vec![50, 100, 150, 200, 250]), 950);
}

#[test]
fn test_84() {
    assert_eq!(Solution::find_maximized_capital(3, 0, vec![100, 200, 300, 400, 500], vec![50, 100, 150, 200, 250]), 0);
}

#[test]
fn test_85() {
    assert_eq!(Solution::find_maximized_capital(3, 10, vec![100, 200, 300, 400, 500], vec![0, 0, 0, 0, 0]), 1210);
}

#[test]
fn test_86() {
    assert_eq!(Solution::find_maximized_capital(2, 500, vec![150, 250, 350, 450, 550], vec![50, 150, 250, 350, 450]), 1500);
}

#[test]
fn test_87() {
    assert_eq!(Solution::find_maximized_capital(5, 10, vec![5, 6, 7, 8, 9], vec![2, 3, 4, 5, 6]), 45);
}

#[test]
fn test_88() {
    assert_eq!(Solution::find_maximized_capital(7, 2, vec![1, 2, 3, 4, 5, 6, 7, 8], vec![0, 1, 2, 3, 4, 5, 6, 7]), 37);
}

#[test]
fn test_89() {
    assert_eq!(Solution::find_maximized_capital(5, 0, vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1], vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0]), 5);
}

#[test]
fn test_90() {
    assert_eq!(Solution::find_maximized_capital(6, 3, vec![1, 2, 3, 4, 5, 6, 7], vec![1, 2, 3, 4, 5, 6, 7]), 30);
}

#[test]
fn test_91() {
    assert_eq!(Solution::find_maximized_capital(3, 10, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], vec![10, 10, 10, 10, 10, 10, 10, 10, 10, 10]), 37);
}

#[test]
fn test_92() {
    assert_eq!(Solution::find_maximized_capital(5, 5, vec![3, 4, 5, 1, 2], vec![1, 2, 3, 4, 5]), 20);
}

#[test]
fn test_93() {
    assert_eq!(Solution::find_maximized_capital(3, 5, vec![10, 20, 30, 40, 50], vec![0, 1, 2, 3, 4]), 125);
}

#[test]
fn test_94() {
    assert_eq!(Solution::find_maximized_capital(5, 100, vec![200, 300, 400, 500, 600], vec![50, 100, 150, 200, 250]), 2100);
}

#[test]
fn test_95() {
    assert_eq!(Solution::find_maximized_capital(100, 0, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0]), 55);
}

#[test]
fn test_96() {
    assert_eq!(Solution::find_maximized_capital(3, 20, vec![10, 20, 30, 40, 50], vec![5, 10, 15, 20, 25]), 140);
}

#[test]
fn test_97() {
    assert_eq!(Solution::find_maximized_capital(3, 1, vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1], vec![0, 0, 0, 0, 0, 0, 0, 0, 0, 0]), 4);
}

#[test]
fn test_98() {
    assert_eq!(Solution::find_maximized_capital(1, 1000000000, vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1], vec![1000000000, 1000000000, 1000000000, 1000000000, 1000000000, 1000000000, 1000000000, 1000000000, 1000000000, 1000000000]), 1000000001);
}

#[test]
fn test_99() {
    assert_eq!(Solution::find_maximized_capital(1, 100, vec![1, 2, 3, 4, 5], vec![0, 0, 0, 0, 0]), 105);
}

#[test]
fn test_100() {
    assert_eq!(Solution::find_maximized_capital(3, 5, vec![2, 3, 5, 6, 8], vec![1, 2, 3, 5, 6]), 24);
}

#[test]
fn test_101() {
    assert_eq!(Solution::find_maximized_capital(5, 0, vec![5, 10, 15, 20, 25, 30, 35, 40, 45, 50], vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100]), 0);
}

#[test]
fn test_102() {
    assert_eq!(Solution::find_maximized_capital(5, 10, vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1], vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9]), 15);
}

#[test]
fn test_103() {
    assert_eq!(Solution::find_maximized_capital(2, 100, vec![100, 200, 300, 400, 500], vec![0, 0, 0, 0, 0]), 1000);
}

#[test]
fn test_104() {
    assert_eq!(Solution::find_maximized_capital(4, 2, vec![1, 2, 3, 4, 5, 6, 7, 8, 9, 10], vec![1, 3, 5, 7, 9, 11, 13, 15, 17, 19]), 12);
}

#[test]
fn test_105() {
    assert_eq!(Solution::find_maximized_capital(3, 500, vec![100, 200, 300, 400, 500, 600, 700, 800], vec![0, 100, 200, 300, 400, 500, 600, 700]), 2600);
}

#[test]
fn test_106() {
    assert_eq!(Solution::find_maximized_capital(5, 10, vec![100, 200, 300, 400, 500], vec![1, 2, 3, 4, 5]), 1510);
}

#[test]
fn test_107() {
    assert_eq!(Solution::find_maximized_capital(2, 5, vec![10, 15, 20, 25, 30], vec![0, 5, 10, 15, 20]), 50);
}

#[test]
fn test_108() {
    assert_eq!(Solution::find_maximized_capital(5, 0, vec![10, 20, 30, 40, 50], vec![100, 50, 200, 300, 400]), 0);
}

#[test]
fn test_109() {
    assert_eq!(Solution::find_maximized_capital(2, 5, vec![10, 10, 10, 10, 10], vec![0, 0, 0, 0, 0]), 25);
}

#[test]
fn test_110() {
    assert_eq!(Solution::find_maximized_capital(5, 0, vec![1, 2, 3, 4, 5, 6], vec![0, 1, 1, 2, 2, 3]), 19);
}

#[test]
fn test_111() {
    assert_eq!(Solution::find_maximized_capital(5, 500, vec![100, 200, 300, 400, 500], vec![0, 0, 0, 0, 0]), 2000);
}

#[test]
fn test_112() {
    assert_eq!(Solution::find_maximized_capital(6, 10, vec![1, 2, 3, 4, 5, 6], vec![0, 2, 4, 6, 8, 10]), 31);
}

#[test]
fn test_113() {
    assert_eq!(Solution::find_maximized_capital(5, 3, vec![10, 2, 11, 1, 4], vec![3, 1, 5, 0, 2]), 31);
}

#[test]
fn test_114() {
    assert_eq!(Solution::find_maximized_capital(4, 0, vec![100, 200, 300, 400], vec![50, 150, 250, 350]), 0);
}

#[test]
fn test_115() {
    assert_eq!(Solution::find_maximized_capital(5, 50, vec![10, 20, 30, 40, 50], vec![10, 20, 30, 40, 50]), 200);
}

#[test]
fn test_116() {
    assert_eq!(Solution::find_maximized_capital(5, 50, vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100], vec![5, 10, 15, 20, 25, 30, 35, 40, 45, 50]), 450);
}

#[test]
fn test_117() {
    assert_eq!(Solution::find_maximized_capital(4, 5, vec![6, 7, 8, 9, 10], vec![0, 5, 10, 15, 20]), 39);
}

#[test]
fn test_118() {
    assert_eq!(Solution::find_maximized_capital(5, 10, vec![1, 3, 5, 7, 9], vec![0, 2, 3, 5, 8]), 35);
}

#[test]
fn test_119() {
    assert_eq!(Solution::find_maximized_capital(3, 5, vec![1, 1, 1, 1, 1, 1, 1, 1, 1, 1], vec![10, 10, 10, 10, 10, 10, 10, 10, 10, 10]), 5);
}

#[test]
fn test_120() {
    assert_eq!(Solution::find_maximized_capital(4, 10, vec![10, 20, 30, 40, 50], vec![0, 10, 20, 30, 40]), 150);
}

#[test]
fn test_121() {
    assert_eq!(Solution::find_maximized_capital(3, 10, vec![15, 25, 35, 45, 55, 65, 75, 85, 95], vec![0, 0, 0, 0, 0, 0, 0, 0, 0]), 265);
}

#[test]
fn test_122() {
    assert_eq!(Solution::find_maximized_capital(2, 1000, vec![1000, 1000, 1000, 1000, 1000], vec![0, 100, 200, 300, 400]), 3000);
}

#[test]
fn test_123() {
    assert_eq!(Solution::find_maximized_capital(4, 1, vec![4, 5, 6, 7, 8, 9], vec![2, 3, 4, 5, 6, 7]), 1);
}

#[test]
fn test_124() {
    assert_eq!(Solution::find_maximized_capital(4, 10, vec![1, 5, 7, 10, 12], vec![3, 7, 10, 14, 17]), 44);
}

#[test]
fn test_125() {
    assert_eq!(Solution::find_maximized_capital(5, 100, vec![10, 20, 30, 40, 50, 60, 70, 80, 90, 100, 110, 120, 130, 140, 150], vec![50, 40, 30, 20, 10, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]), 750);
}

#[test]
fn test_126() {
    assert_eq!(Solution::find_maximized_capital(3, 1000, vec![300, 500, 700, 100, 200, 400, 600, 800], vec![0, 500, 1000, 1500, 2000, 2500, 3000, 3500]), 2500);
}

#[test]
fn test_127() {
    assert_eq!(Solution::find_maximized_capital(4, 1000, vec![900, 800, 700, 600, 500], vec![500, 600, 700, 800, 900]), 4000);
}

#[test]
fn test_128() {
    assert_eq!(Solution::find_maximized_capital(3, 0, vec![10, 20, 30, 40, 50], vec![1, 2, 3, 4, 5]), 0);
}

#[test]
fn test_129() {
    assert_eq!(Solution::find_maximized_capital(15, 500, vec![50, 100, 150, 200, 250, 300, 350, 400, 450, 500, 550, 600, 650, 700, 750], vec![200, 150, 100, 50, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0, 0]), 6500);
}

#[test]
fn test_130() {
    assert_eq!(Solution::find_maximized_capital(2, 5, vec![10, 20, 30, 40, 50], vec![1, 2, 3, 4, 5]), 95);
}

#[test]
fn test_131() {
    assert_eq!(Solution::find_maximized_capital(5, 10, vec![10, 20, 30, 40, 50], vec![5, 15, 25, 35, 45]), 160);
}
