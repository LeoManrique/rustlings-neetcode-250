include!("../src/lib.rs");

#[test]
fn test_1() {
    assert_eq!(Solution::most_booked(5, vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![5, 6], vec![6, 7], vec![7, 8], vec![8, 9], vec![9, 10], vec![10, 11], vec![11, 12], vec![12, 13], vec![13, 14], vec![14, 15]]), 0);
}

#[test]
fn test_2() {
    assert_eq!(Solution::most_booked(5, vec![vec![0, 2], vec![1, 3], vec![2, 4], vec![3, 5], vec![4, 6]]), 0);
}

#[test]
fn test_3() {
    assert_eq!(Solution::most_booked(10, vec![vec![0, 1], vec![0, 1], vec![0, 1], vec![0, 1], vec![0, 1], vec![0, 1], vec![0, 1], vec![0, 1], vec![0, 1], vec![0, 1]]), 0);
}

#[test]
fn test_4() {
    assert_eq!(Solution::most_booked(5, vec![vec![0, 30], vec![5, 10], vec![15, 20], vec![5, 10]]), 1);
}

#[test]
fn test_5() {
    assert_eq!(Solution::most_booked(2, vec![vec![0, 10], vec![1, 5], vec![2, 7], vec![3, 4]]), 0);
}

#[test]
fn test_6() {
    assert_eq!(Solution::most_booked(4, vec![vec![0, 2], vec![0, 3], vec![1, 2], vec![2, 4], vec![3, 5]]), 0);
}

#[test]
fn test_7() {
    assert_eq!(Solution::most_booked(1, vec![vec![1, 2], vec![3, 4], vec![5, 6]]), 0);
}

#[test]
fn test_8() {
    assert_eq!(Solution::most_booked(5, vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5]]), 0);
}

#[test]
fn test_9() {
    assert_eq!(Solution::most_booked(1, vec![vec![0, 1], vec![2, 3], vec![4, 5]]), 0);
}

#[test]
fn test_10() {
    assert_eq!(Solution::most_booked(10, vec![vec![0, 10], vec![10, 20], vec![20, 30], vec![30, 40], vec![40, 50], vec![50, 60], vec![60, 70], vec![70, 80], vec![80, 90], vec![90, 100]]), 0);
}

#[test]
fn test_11() {
    assert_eq!(Solution::most_booked(4, vec![vec![0, 5], vec![5, 10], vec![10, 15], vec![15, 20], vec![20, 25], vec![25, 30]]), 0);
}

#[test]
fn test_12() {
    assert_eq!(Solution::most_booked(2, vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5]]), 0);
}

#[test]
fn test_13() {
    assert_eq!(Solution::most_booked(3, vec![vec![1, 20], vec![2, 10], vec![3, 5], vec![4, 9], vec![6, 8]]), 1);
}

#[test]
fn test_14() {
    assert_eq!(Solution::most_booked(3, vec![vec![0, 100], vec![101, 200], vec![201, 300], vec![301, 400]]), 0);
}

#[test]
fn test_15() {
    assert_eq!(Solution::most_booked(1, vec![vec![0, 100], vec![100, 200], vec![200, 300]]), 0);
}

#[test]
fn test_16() {
    assert_eq!(Solution::most_booked(4, vec![vec![1, 10], vec![2, 15], vec![3, 20], vec![4, 25], vec![5, 30]]), 0);
}

#[test]
fn test_17() {
    assert_eq!(Solution::most_booked(10, vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![5, 6], vec![6, 7], vec![7, 8], vec![8, 9], vec![9, 10], vec![10, 11], vec![11, 12], vec![12, 13], vec![13, 14], vec![14, 15], vec![15, 16], vec![16, 17], vec![17, 18], vec![18, 19], vec![19, 20], vec![20, 21], vec![21, 22], vec![22, 23], vec![23, 24], vec![24, 25], vec![25, 26], vec![26, 27], vec![27, 28], vec![28, 29], vec![29, 30]]), 0);
}

#[test]
fn test_18() {
    assert_eq!(Solution::most_booked(7, vec![vec![1, 5], vec![1, 10], vec![1, 15], vec![1, 20], vec![2, 6], vec![2, 11], vec![2, 16], vec![2, 21], vec![3, 7], vec![3, 12], vec![3, 17], vec![3, 22], vec![4, 8], vec![4, 13], vec![4, 18], vec![4, 23], vec![5, 9], vec![5, 14], vec![5, 19], vec![5, 24]]), 4);
}

#[test]
fn test_19() {
    assert_eq!(Solution::most_booked(6, vec![vec![0, 100], vec![1, 99], vec![2, 98], vec![3, 97], vec![4, 96], vec![5, 95], vec![6, 94], vec![7, 93], vec![8, 92], vec![9, 91], vec![10, 90], vec![11, 89], vec![12, 88], vec![13, 87], vec![14, 86], vec![15, 85], vec![16, 84], vec![17, 83], vec![18, 82], vec![19, 81], vec![20, 80], vec![21, 79], vec![22, 78], vec![23, 77], vec![24, 76], vec![25, 75], vec![26, 74], vec![27, 73], vec![28, 72], vec![29, 71], vec![30, 70], vec![31, 69], vec![32, 68], vec![33, 67], vec![34, 66], vec![35, 65], vec![36, 64], vec![37, 63], vec![38, 62], vec![39, 61], vec![40, 60], vec![41, 59], vec![42, 58], vec![43, 57], vec![44, 56], vec![45, 55], vec![46, 54], vec![47, 53], vec![48, 52], vec![49, 51]]), 0);
}

#[test]
fn test_20() {
    assert_eq!(Solution::most_booked(10, vec![vec![1, 11], vec![2, 12], vec![3, 13], vec![4, 14], vec![5, 15], vec![6, 16], vec![7, 17], vec![8, 18], vec![9, 19], vec![10, 20], vec![11, 21], vec![12, 22], vec![13, 23], vec![14, 24], vec![15, 25], vec![16, 26], vec![17, 27], vec![18, 28], vec![19, 29], vec![20, 30]]), 0);
}

#[test]
fn test_21() {
    assert_eq!(Solution::most_booked(6, vec![vec![1, 3], vec![2, 5], vec![4, 7], vec![6, 9], vec![8, 11], vec![10, 13], vec![12, 15], vec![14, 17], vec![16, 19], vec![18, 21]]), 0);
}

#[test]
fn test_22() {
    assert_eq!(Solution::most_booked(4, vec![vec![1, 10], vec![2, 8], vec![3, 6], vec![4, 5], vec![5, 15], vec![6, 12], vec![7, 10], vec![8, 20], vec![9, 11], vec![10, 13]]), 1);
}

#[test]
fn test_23() {
    assert_eq!(Solution::most_booked(5, vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![5, 6], vec![6, 7], vec![7, 8], vec![8, 9], vec![9, 10], vec![10, 11], vec![11, 12], vec![12, 13], vec![13, 14], vec![14, 15], vec![15, 16], vec![16, 17], vec![17, 18], vec![18, 19], vec![19, 20]]), 0);
}

#[test]
fn test_24() {
    assert_eq!(Solution::most_booked(7, vec![vec![1, 3], vec![2, 4], vec![3, 5], vec![4, 6], vec![5, 7], vec![6, 8], vec![7, 9], vec![8, 10], vec![9, 11], vec![10, 12], vec![11, 13], vec![12, 14], vec![13, 15], vec![14, 16], vec![15, 17], vec![16, 18], vec![17, 19], vec![18, 20]]), 0);
}

#[test]
fn test_25() {
    assert_eq!(Solution::most_booked(8, vec![vec![1, 10], vec![2, 11], vec![3, 12], vec![4, 13], vec![5, 14], vec![6, 15], vec![7, 16], vec![8, 17], vec![9, 18], vec![10, 19], vec![11, 20], vec![12, 21], vec![13, 22], vec![14, 23], vec![15, 24], vec![16, 25], vec![17, 26], vec![18, 27], vec![19, 28], vec![20, 29], vec![21, 30]]), 0);
}

#[test]
fn test_26() {
    assert_eq!(Solution::most_booked(3, vec![vec![10, 15], vec![20, 25], vec![30, 35], vec![5, 10], vec![15, 20], vec![25, 30], vec![35, 40], vec![5, 15], vec![10, 20], vec![20, 30]]), 0);
}

#[test]
fn test_27() {
    assert_eq!(Solution::most_booked(10, vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![5, 6], vec![6, 7], vec![7, 8], vec![8, 9], vec![9, 10], vec![10, 11], vec![11, 12], vec![12, 13], vec![13, 14], vec![14, 15], vec![15, 16], vec![16, 17], vec![17, 18], vec![18, 19], vec![19, 20]]), 0);
}

#[test]
fn test_28() {
    assert_eq!(Solution::most_booked(3, vec![vec![0, 10], vec![1, 5], vec![2, 7], vec![3, 4], vec![4, 6], vec![5, 8], vec![6, 9], vec![7, 10], vec![8, 11], vec![9, 12]]), 1);
}

#[test]
fn test_29() {
    assert_eq!(Solution::most_booked(10, vec![vec![0, 1000], vec![500, 1500], vec![1000, 2000], vec![1500, 2500], vec![2000, 3000], vec![2500, 3500], vec![3000, 4000], vec![3500, 4500], vec![4000, 5000], vec![4500, 5500]]), 0);
}

#[test]
fn test_30() {
    assert_eq!(Solution::most_booked(2, vec![vec![0, 100], vec![50, 150], vec![100, 200], vec![150, 250], vec![200, 300], vec![250, 350], vec![300, 400], vec![350, 450], vec![400, 500], vec![450, 550]]), 0);
}

#[test]
fn test_31() {
    assert_eq!(Solution::most_booked(2, vec![vec![0, 10], vec![10, 20], vec![20, 30], vec![30, 40], vec![40, 50], vec![50, 60], vec![60, 70], vec![70, 80], vec![80, 90], vec![90, 100], vec![0, 5], vec![5, 10], vec![10, 15], vec![15, 20], vec![20, 25], vec![25, 30], vec![30, 35], vec![35, 40], vec![40, 45], vec![45, 50], vec![50, 55], vec![55, 60], vec![60, 65], vec![65, 70], vec![70, 75], vec![75, 80], vec![80, 85], vec![85, 90], vec![90, 95], vec![95, 100]]), 0);
}

#[test]
fn test_32() {
    assert_eq!(Solution::most_booked(5, vec![vec![0, 2], vec![0, 4], vec![0, 6], vec![0, 8], vec![0, 10], vec![1, 3], vec![1, 5], vec![1, 7], vec![1, 9], vec![2, 4], vec![2, 6], vec![2, 8], vec![3, 5], vec![3, 7], vec![4, 6], vec![4, 8], vec![5, 7], vec![6, 8], vec![7, 9], vec![8, 10]]), 0);
}

#[test]
fn test_33() {
    assert_eq!(Solution::most_booked(7, vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![5, 6], vec![6, 7], vec![7, 8], vec![8, 9], vec![9, 10], vec![10, 11], vec![11, 12], vec![12, 13], vec![13, 14], vec![14, 15]]), 0);
}

#[test]
fn test_34() {
    assert_eq!(Solution::most_booked(3, vec![vec![0, 3], vec![1, 2], vec![2, 5], vec![3, 6], vec![4, 7], vec![5, 8], vec![6, 9], vec![7, 10], vec![8, 11], vec![9, 12], vec![10, 13]]), 0);
}

#[test]
fn test_35() {
    assert_eq!(Solution::most_booked(4, vec![vec![0, 10], vec![1, 5], vec![2, 7], vec![3, 4], vec![8, 12], vec![10, 15], vec![13, 18]]), 1);
}

#[test]
fn test_36() {
    assert_eq!(Solution::most_booked(3, vec![vec![0, 20], vec![1, 19], vec![2, 18], vec![3, 17], vec![4, 16], vec![5, 15], vec![6, 14], vec![7, 13], vec![8, 12], vec![9, 11], vec![10, 10], vec![11, 9], vec![12, 8], vec![13, 7], vec![14, 6], vec![15, 5], vec![16, 4], vec![17, 3], vec![18, 2], vec![19, 1]]), 1);
}

#[test]
fn test_37() {
    assert_eq!(Solution::most_booked(6, vec![vec![0, 3], vec![3, 6], vec![6, 9], vec![9, 12], vec![12, 15], vec![15, 18], vec![18, 21], vec![21, 24], vec![24, 27], vec![27, 30], vec![30, 33], vec![33, 36], vec![36, 39], vec![39, 42], vec![42, 45]]), 0);
}

#[test]
fn test_38() {
    assert_eq!(Solution::most_booked(2, vec![vec![100, 105], vec![101, 106], vec![102, 107], vec![103, 108], vec![104, 109], vec![105, 110], vec![106, 111], vec![107, 112], vec![108, 113], vec![109, 114]]), 0);
}

#[test]
fn test_39() {
    assert_eq!(Solution::most_booked(5, vec![vec![1, 3], vec![2, 4], vec![3, 5], vec![4, 6], vec![5, 7], vec![6, 8], vec![7, 9], vec![8, 10], vec![9, 11], vec![10, 12], vec![11, 13], vec![12, 14], vec![13, 15], vec![14, 16], vec![15, 17]]), 0);
}

#[test]
fn test_40() {
    assert_eq!(Solution::most_booked(3, vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![5, 6], vec![6, 7], vec![7, 8], vec![8, 9], vec![9, 10], vec![10, 11], vec![11, 12], vec![12, 13], vec![13, 14], vec![14, 15], vec![15, 16], vec![16, 17], vec![17, 18], vec![18, 19], vec![19, 20], vec![20, 21], vec![21, 22], vec![22, 23], vec![23, 24], vec![24, 25]]), 0);
}

#[test]
fn test_41() {
    assert_eq!(Solution::most_booked(4, vec![vec![0, 50], vec![10, 60], vec![20, 70], vec![30, 80], vec![40, 90], vec![50, 100], vec![60, 110], vec![70, 120], vec![80, 130], vec![90, 140]]), 0);
}

#[test]
fn test_42() {
    assert_eq!(Solution::most_booked(6, vec![vec![1, 5], vec![2, 6], vec![3, 7], vec![4, 8], vec![5, 9], vec![6, 10], vec![7, 11], vec![8, 12], vec![9, 13], vec![10, 14], vec![11, 15], vec![12, 16], vec![13, 17], vec![14, 18], vec![15, 19]]), 0);
}

#[test]
fn test_43() {
    assert_eq!(Solution::most_booked(2, vec![vec![0, 10], vec![10, 20], vec![20, 30], vec![30, 40], vec![40, 50], vec![0, 15], vec![15, 25], vec![25, 35], vec![35, 45], vec![45, 55]]), 0);
}

#[test]
fn test_44() {
    assert_eq!(Solution::most_booked(7, vec![vec![1, 2], vec![1, 3], vec![1, 4], vec![1, 5], vec![1, 6], vec![1, 7], vec![1, 8], vec![2, 3], vec![2, 4], vec![2, 5], vec![2, 6], vec![2, 7], vec![2, 8], vec![3, 4], vec![3, 5], vec![3, 6], vec![3, 7], vec![3, 8], vec![4, 5], vec![4, 6], vec![4, 7], vec![4, 8], vec![5, 6], vec![5, 7], vec![5, 8], vec![6, 7], vec![6, 8], vec![7, 8]]), 1);
}

#[test]
fn test_45() {
    assert_eq!(Solution::most_booked(7, vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![5, 6], vec![6, 7], vec![7, 8], vec![8, 9], vec![9, 10], vec![0, 11], vec![1, 12], vec![2, 13], vec![3, 14], vec![4, 15]]), 0);
}

#[test]
fn test_46() {
    assert_eq!(Solution::most_booked(5, vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![5, 6], vec![6, 7], vec![7, 8], vec![8, 9], vec![9, 10], vec![10, 11], vec![11, 12], vec![12, 13], vec![13, 14], vec![14, 15], vec![15, 16], vec![16, 17], vec![17, 18], vec![18, 19], vec![19, 20]]), 0);
}

#[test]
fn test_47() {
    assert_eq!(Solution::most_booked(10, vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![5, 6], vec![6, 7], vec![7, 8], vec![8, 9], vec![9, 10], vec![10, 11], vec![11, 12], vec![12, 13], vec![13, 14], vec![14, 15], vec![15, 16], vec![16, 17], vec![17, 18], vec![18, 19], vec![19, 20], vec![20, 21], vec![21, 22], vec![22, 23], vec![23, 24], vec![24, 25]]), 0);
}

#[test]
fn test_48() {
    assert_eq!(Solution::most_booked(4, vec![vec![0, 30], vec![5, 10], vec![15, 20], vec![25, 35], vec![10, 25], vec![20, 30], vec![30, 40]]), 1);
}

#[test]
fn test_49() {
    assert_eq!(Solution::most_booked(7, vec![vec![1, 5], vec![2, 8], vec![3, 9], vec![4, 10], vec![5, 11], vec![6, 12], vec![7, 13], vec![8, 14], vec![9, 15], vec![10, 16], vec![11, 17], vec![12, 18], vec![13, 19], vec![14, 20], vec![15, 21], vec![16, 22], vec![17, 23], vec![18, 24], vec![19, 25], vec![20, 26], vec![21, 27], vec![22, 28], vec![23, 29], vec![24, 30], vec![25, 31], vec![26, 32], vec![27, 33], vec![28, 34], vec![29, 35], vec![30, 36], vec![31, 37], vec![32, 38], vec![33, 39], vec![34, 40], vec![35, 41], vec![36, 42], vec![37, 43], vec![38, 44], vec![39, 45], vec![40, 46], vec![41, 47], vec![42, 48], vec![43, 49], vec![44, 50], vec![45, 51], vec![46, 52], vec![47, 53], vec![48, 54], vec![49, 55]]), 0);
}

#[test]
fn test_50() {
    assert_eq!(Solution::most_booked(6, vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![5, 6], vec![6, 7], vec![7, 8], vec![8, 9], vec![9, 10], vec![10, 11], vec![11, 12], vec![12, 13], vec![13, 14], vec![14, 15], vec![15, 16], vec![16, 17], vec![17, 18], vec![18, 19], vec![19, 20]]), 0);
}

#[test]
fn test_51() {
    assert_eq!(Solution::most_booked(7, vec![vec![1, 2], vec![1, 3], vec![1, 4], vec![1, 5], vec![1, 6], vec![1, 7], vec![2, 3], vec![2, 4], vec![2, 5], vec![2, 6], vec![2, 7], vec![3, 4], vec![3, 5], vec![3, 6], vec![3, 7], vec![4, 5], vec![4, 6], vec![4, 7], vec![5, 6], vec![5, 7], vec![6, 7]]), 0);
}

#[test]
fn test_52() {
    assert_eq!(Solution::most_booked(3, vec![vec![0, 100], vec![50, 150], vec![100, 200], vec![150, 250], vec![200, 300], vec![250, 350], vec![300, 400]]), 0);
}

#[test]
fn test_53() {
    assert_eq!(Solution::most_booked(4, vec![vec![0, 3], vec![1, 5], vec![2, 6], vec![3, 9], vec![4, 10], vec![5, 12], vec![6, 13], vec![7, 15], vec![8, 17], vec![9, 18]]), 0);
}

#[test]
fn test_54() {
    assert_eq!(Solution::most_booked(8, vec![vec![0, 15], vec![15, 30], vec![30, 45], vec![45, 60], vec![60, 75], vec![75, 90], vec![90, 105], vec![105, 120], vec![120, 135], vec![135, 150], vec![150, 165], vec![165, 180], vec![180, 195], vec![195, 210], vec![210, 225]]), 0);
}

#[test]
fn test_55() {
    assert_eq!(Solution::most_booked(3, vec![vec![0, 5], vec![5, 10], vec![10, 15], vec![15, 20], vec![20, 25], vec![25, 30], vec![30, 35], vec![35, 40], vec![40, 45], vec![45, 50], vec![0, 30], vec![10, 40], vec![20, 50]]), 0);
}

#[test]
fn test_56() {
    assert_eq!(Solution::most_booked(8, vec![vec![0, 1], vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![5, 6], vec![6, 7], vec![7, 8], vec![8, 9], vec![9, 10], vec![10, 11], vec![11, 12], vec![12, 13], vec![13, 14], vec![14, 15], vec![15, 16], vec![16, 17], vec![17, 18], vec![18, 19], vec![19, 20]]), 0);
}

#[test]
fn test_57() {
    assert_eq!(Solution::most_booked(5, vec![vec![10, 20], vec![20, 30], vec![30, 40], vec![40, 50], vec![50, 60], vec![15, 25], vec![25, 35], vec![35, 45], vec![45, 55], vec![55, 65]]), 0);
}

#[test]
fn test_58() {
    assert_eq!(Solution::most_booked(5, vec![vec![0, 2], vec![2, 4], vec![4, 6], vec![6, 8], vec![8, 10], vec![1, 3], vec![3, 5], vec![5, 7], vec![7, 9], vec![9, 11], vec![12, 14], vec![14, 16], vec![16, 18], vec![18, 20]]), 0);
}

#[test]
fn test_59() {
    assert_eq!(Solution::most_booked(5, vec![vec![0, 10], vec![1, 11], vec![2, 12], vec![3, 13], vec![4, 14], vec![5, 15], vec![6, 16], vec![7, 17], vec![8, 18], vec![9, 19], vec![10, 20], vec![11, 21], vec![12, 22], vec![13, 23], vec![14, 24], vec![15, 25], vec![16, 26], vec![17, 27], vec![18, 28], vec![19, 29]]), 0);
}

#[test]
fn test_60() {
    assert_eq!(Solution::most_booked(3, vec![vec![1, 100], vec![101, 200], vec![201, 300], vec![10, 110], vec![110, 210], vec![210, 310], vec![20, 120], vec![120, 220], vec![220, 320], vec![30, 130], vec![130, 230], vec![230, 330]]), 0);
}

#[test]
fn test_61() {
    assert_eq!(Solution::most_booked(6, vec![vec![0, 2], vec![1, 4], vec![2, 6], vec![3, 8], vec![4, 10], vec![5, 12], vec![6, 14], vec![7, 16], vec![8, 18], vec![9, 20], vec![10, 22], vec![11, 24], vec![12, 26], vec![13, 28], vec![14, 30], vec![15, 32], vec![16, 34], vec![17, 36], vec![18, 38], vec![19, 40]]), 0);
}

#[test]
fn test_62() {
    assert_eq!(Solution::most_booked(2, vec![vec![0, 5], vec![5, 10], vec![10, 15], vec![15, 20], vec![20, 25], vec![25, 30], vec![30, 35], vec![35, 40], vec![40, 45], vec![45, 50], vec![50, 55], vec![55, 60], vec![60, 65], vec![65, 70], vec![70, 75], vec![75, 80], vec![80, 85], vec![85, 90], vec![90, 95], vec![95, 100]]), 0);
}

#[test]
fn test_63() {
    assert_eq!(Solution::most_booked(4, vec![vec![1, 15], vec![2, 3], vec![5, 12], vec![8, 10], vec![6, 14], vec![16, 20], vec![17, 19], vec![21, 25], vec![22, 24], vec![23, 26]]), 1);
}

#[test]
fn test_64() {
    assert_eq!(Solution::most_booked(6, vec![vec![0, 5], vec![1, 6], vec![2, 7], vec![3, 8], vec![4, 9], vec![5, 10], vec![6, 11], vec![7, 12], vec![8, 13], vec![9, 14], vec![10, 15], vec![11, 16], vec![12, 17], vec![13, 18], vec![14, 19], vec![15, 20], vec![16, 21], vec![17, 22], vec![18, 23], vec![19, 24], vec![20, 25], vec![21, 26], vec![22, 27], vec![23, 28], vec![24, 29], vec![25, 30]]), 0);
}

#[test]
fn test_65() {
    assert_eq!(Solution::most_booked(5, vec![vec![0, 10], vec![10, 20], vec![20, 30], vec![30, 40], vec![40, 50], vec![1, 11], vec![11, 21], vec![21, 31], vec![31, 41], vec![41, 51]]), 0);
}

#[test]
fn test_66() {
    assert_eq!(Solution::most_booked(6, vec![vec![1, 1000], vec![2, 2000], vec![3, 3000], vec![4, 4000], vec![5, 5000], vec![6, 6000], vec![7, 7000], vec![8, 8000], vec![9, 9000], vec![10, 10000]]), 0);
}

#[test]
fn test_67() {
    assert_eq!(Solution::most_booked(3, vec![vec![0, 100], vec![50, 150], vec![100, 200], vec![150, 250], vec![200, 300], vec![250, 350], vec![300, 400], vec![350, 450], vec![400, 500], vec![450, 550], vec![500, 600], vec![550, 650], vec![600, 700], vec![650, 750], vec![700, 800]]), 0);
}

#[test]
fn test_68() {
    assert_eq!(Solution::most_booked(3, vec![vec![0, 2], vec![0, 5], vec![0, 10], vec![5, 10], vec![10, 15], vec![5, 15], vec![10, 20], vec![15, 25], vec![20, 30], vec![25, 35]]), 0);
}

#[test]
fn test_69() {
    assert_eq!(Solution::most_booked(7, vec![vec![1, 3], vec![2, 5], vec![3, 7], vec![4, 9], vec![5, 11], vec![6, 13], vec![7, 15], vec![8, 17], vec![9, 19], vec![10, 21], vec![11, 23], vec![12, 25], vec![13, 27], vec![14, 29], vec![15, 31], vec![16, 33], vec![17, 35], vec![18, 37], vec![19, 39]]), 0);
}

#[test]
fn test_70() {
    assert_eq!(Solution::most_booked(4, vec![vec![1, 10], vec![2, 15], vec![3, 12], vec![4, 11], vec![5, 14], vec![6, 13], vec![7, 16], vec![8, 17], vec![9, 18], vec![10, 19]]), 0);
}

#[test]
fn test_71() {
    assert_eq!(Solution::most_booked(3, vec![vec![0, 1], vec![0, 1], vec![0, 1], vec![0, 1], vec![0, 1], vec![0, 1], vec![0, 1], vec![0, 1], vec![0, 1], vec![0, 1]]), 0);
}

#[test]
fn test_72() {
    assert_eq!(Solution::most_booked(5, vec![vec![0, 2], vec![1, 4], vec![2, 6], vec![3, 8], vec![4, 10], vec![5, 12], vec![6, 14], vec![7, 16], vec![8, 18], vec![9, 20], vec![10, 22], vec![11, 24], vec![12, 26], vec![13, 28], vec![14, 30]]), 0);
}

#[test]
fn test_73() {
    assert_eq!(Solution::most_booked(5, vec![vec![0, 10], vec![10, 20], vec![20, 30], vec![30, 40], vec![40, 50], vec![5, 15], vec![15, 25], vec![25, 35], vec![35, 45], vec![45, 55]]), 0);
}

#[test]
fn test_74() {
    assert_eq!(Solution::most_booked(3, vec![vec![0, 10], vec![10, 20], vec![20, 30], vec![30, 40], vec![40, 50], vec![50, 60], vec![60, 70], vec![70, 80], vec![80, 90], vec![90, 100], vec![100, 110], vec![110, 120], vec![120, 130], vec![130, 140], vec![140, 150]]), 0);
}

#[test]
fn test_75() {
    assert_eq!(Solution::most_booked(3, vec![vec![1, 30], vec![2, 25], vec![3, 20], vec![4, 15], vec![5, 10], vec![6, 5], vec![7, 4], vec![8, 3], vec![9, 2], vec![10, 1], vec![11, 30], vec![12, 25], vec![13, 20], vec![14, 15], vec![15, 10], vec![16, 5], vec![17, 4], vec![18, 3], vec![19, 2], vec![20, 1]]), 2);
}

#[test]
fn test_76() {
    assert_eq!(Solution::most_booked(2, vec![vec![0, 100], vec![50, 60], vec![20, 30], vec![10, 20], vec![30, 40], vec![70, 80], vec![110, 120], vec![90, 100], vec![130, 140], vec![150, 160]]), 1);
}

#[test]
fn test_77() {
    assert_eq!(Solution::most_booked(4, vec![vec![1, 18], vec![2, 15], vec![3, 12], vec![4, 9], vec![5, 6], vec![7, 8], vec![9, 10], vec![11, 12], vec![13, 14], vec![15, 16], vec![17, 18]]), 3);
}

#[test]
fn test_78() {
    assert_eq!(Solution::most_booked(10, vec![vec![0, 2], vec![1, 3], vec![2, 4], vec![3, 5], vec![4, 6], vec![5, 7], vec![6, 8], vec![7, 9], vec![8, 10], vec![9, 11], vec![10, 12], vec![11, 13], vec![12, 14], vec![13, 15], vec![14, 16], vec![15, 17], vec![16, 18], vec![17, 19], vec![18, 20], vec![19, 21]]), 0);
}

#[test]
fn test_79() {
    assert_eq!(Solution::most_booked(7, vec![vec![0, 100], vec![100, 200], vec![200, 300], vec![300, 400], vec![400, 500], vec![500, 600], vec![600, 700], vec![700, 800], vec![800, 900], vec![900, 1000], vec![100, 500], vec![200, 600], vec![300, 700], vec![400, 800], vec![500, 900], vec![600, 1000]]), 0);
}

#[test]
fn test_80() {
    assert_eq!(Solution::most_booked(4, vec![vec![0, 5], vec![1, 10], vec![2, 15], vec![3, 20], vec![4, 25], vec![5, 30], vec![6, 35], vec![7, 40], vec![8, 45], vec![9, 50]]), 0);
}

#[test]
fn test_81() {
    assert_eq!(Solution::most_booked(8, vec![vec![0, 2], vec![0, 4], vec![0, 6], vec![0, 8], vec![0, 10], vec![0, 12], vec![0, 14], vec![0, 16], vec![0, 18], vec![0, 20], vec![1, 3], vec![1, 5], vec![1, 7], vec![1, 9], vec![1, 11], vec![1, 13], vec![1, 15], vec![1, 17], vec![1, 19], vec![1, 21]]), 2);
}

#[test]
fn test_82() {
    assert_eq!(Solution::most_booked(4, vec![vec![0, 2], vec![1, 3], vec![2, 4], vec![3, 5], vec![4, 6], vec![5, 7], vec![6, 8], vec![7, 9], vec![8, 10], vec![9, 11], vec![10, 12], vec![11, 13], vec![12, 14], vec![13, 15], vec![14, 16], vec![15, 17], vec![16, 18], vec![17, 19], vec![18, 20], vec![19, 21]]), 0);
}

#[test]
fn test_83() {
    assert_eq!(Solution::most_booked(5, vec![vec![0, 50], vec![10, 60], vec![20, 70], vec![30, 80], vec![40, 90], vec![50, 100], vec![60, 110], vec![70, 120], vec![80, 130], vec![90, 140]]), 0);
}

#[test]
fn test_84() {
    assert_eq!(Solution::most_booked(5, vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![5, 6], vec![6, 7], vec![7, 8], vec![8, 9], vec![9, 10], vec![10, 11], vec![11, 12], vec![12, 13], vec![13, 14], vec![14, 15], vec![15, 16], vec![16, 17], vec![17, 18], vec![18, 19], vec![19, 20], vec![20, 21]]), 0);
}

#[test]
fn test_85() {
    assert_eq!(Solution::most_booked(4, vec![vec![1, 10], vec![10, 20], vec![15, 25], vec![20, 30], vec![25, 35], vec![30, 40], vec![35, 45], vec![40, 50], vec![45, 55], vec![50, 60], vec![55, 65], vec![60, 70]]), 0);
}

#[test]
fn test_86() {
    assert_eq!(Solution::most_booked(3, vec![vec![0, 1], vec![0, 2], vec![0, 3], vec![1, 4], vec![1, 5], vec![1, 6], vec![2, 5], vec![2, 6], vec![3, 6], vec![3, 7], vec![4, 8], vec![5, 9], vec![6, 10], vec![7, 11], vec![8, 12]]), 0);
}

#[test]
fn test_87() {
    assert_eq!(Solution::most_booked(7, vec![vec![0, 100], vec![50, 150], vec![100, 200], vec![150, 250], vec![200, 300], vec![250, 350], vec![300, 400], vec![350, 450], vec![400, 500], vec![450, 550], vec![500, 600], vec![550, 650], vec![600, 700], vec![650, 750], vec![700, 800]]), 0);
}

#[test]
fn test_88() {
    assert_eq!(Solution::most_booked(6, vec![vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![5, 6], vec![6, 7], vec![7, 8], vec![8, 9], vec![9, 10], vec![10, 11], vec![11, 12], vec![12, 13], vec![13, 14], vec![14, 15]]), 0);
}

#[test]
fn test_89() {
    assert_eq!(Solution::most_booked(1, vec![vec![0, 10], vec![1, 20], vec![2, 30], vec![3, 40], vec![4, 50], vec![5, 60], vec![6, 70], vec![7, 80], vec![8, 90], vec![9, 100], vec![10, 110], vec![11, 120], vec![12, 130], vec![13, 140], vec![14, 150], vec![15, 160], vec![16, 170], vec![17, 180], vec![18, 190], vec![19, 200], vec![20, 210], vec![21, 220], vec![22, 230], vec![23, 240], vec![24, 250], vec![25, 260], vec![26, 270], vec![27, 280], vec![28, 290], vec![29, 300]]), 0);
}

#[test]
fn test_90() {
    assert_eq!(Solution::most_booked(6, vec![vec![1, 6], vec![2, 5], vec![3, 4], vec![5, 7], vec![6, 8], vec![7, 9], vec![8, 10], vec![9, 11], vec![10, 12], vec![11, 13], vec![12, 14], vec![13, 15], vec![14, 16], vec![15, 17], vec![16, 18], vec![17, 19], vec![18, 20]]), 0);
}

#[test]
fn test_91() {
    assert_eq!(Solution::most_booked(10, vec![vec![0, 10], vec![1, 2], vec![2, 3], vec![3, 4], vec![4, 5], vec![5, 6], vec![6, 7], vec![7, 8], vec![8, 9], vec![9, 10], vec![10, 20], vec![20, 30], vec![30, 40], vec![40, 50], vec![50, 60], vec![60, 70], vec![70, 80], vec![80, 90], vec![90, 100]]), 0);
}

#[test]
fn test_92() {
    assert_eq!(Solution::most_booked(4, vec![vec![0, 15], vec![5, 20], vec![10, 25], vec![15, 30], vec![20, 35], vec![25, 40]]), 0);
}
