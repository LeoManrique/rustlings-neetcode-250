include!("../src/lib.rs");

#[test]
fn test_1() {
    let got = Solution::calc_equation(vec![vec!["a1".to_string(), "b1".to_string()], vec!["a2".to_string(), "b2".to_string()], vec!["a3".to_string(), "b3".to_string()]], vec![1.1, 2.2, 3.3], vec![vec!["a1".to_string(), "b1".to_string()], vec!["a2".to_string(), "b2".to_string()], vec!["a3".to_string(), "b3".to_string()], vec!["b1".to_string(), "a1".to_string()]]);
    let want: Vec<f64> = vec![1.1, 2.2, 3.3, 0.9090909090909091];
    assert_eq!(got.len(), want.len());
    for (g, w) in got.iter().zip(want.iter()) {
        if w.is_nan() {
            assert!(g.is_nan());
        } else {
            assert!((g - w).abs() < 1e-5, "got {} want {}", g, w);
        }
    }
}

#[test]
fn test_2() {
    let got = Solution::calc_equation(vec![vec!["a".to_string(), "b".to_string()], vec!["b".to_string(), "c".to_string()]], vec![2.0, 3.0], vec![vec!["a".to_string(), "c".to_string()], vec!["b".to_string(), "a".to_string()], vec!["a".to_string(), "e".to_string()], vec!["a".to_string(), "a".to_string()], vec!["x".to_string(), "x".to_string()]]);
    let want: Vec<f64> = vec![6.0, 0.5, -1.0, 1.0, -1.0];
    assert_eq!(got.len(), want.len());
    for (g, w) in got.iter().zip(want.iter()) {
        if w.is_nan() {
            assert!(g.is_nan());
        } else {
            assert!((g - w).abs() < 1e-5, "got {} want {}", g, w);
        }
    }
}

#[test]
fn test_3() {
    let got = Solution::calc_equation(vec![vec!["a".to_string(), "b".to_string()]], vec![0.5], vec![vec!["a".to_string(), "b".to_string()], vec!["b".to_string(), "a".to_string()], vec!["a".to_string(), "c".to_string()], vec!["x".to_string(), "y".to_string()]]);
    let want: Vec<f64> = vec![0.5, 2.0, -1.0, -1.0];
    assert_eq!(got.len(), want.len());
    for (g, w) in got.iter().zip(want.iter()) {
        if w.is_nan() {
            assert!(g.is_nan());
        } else {
            assert!((g - w).abs() < 1e-5, "got {} want {}", g, w);
        }
    }
}

#[test]
fn test_4() {
    let got = Solution::calc_equation(vec![vec!["x1".to_string(), "x2".to_string()], vec!["x3".to_string(), "x4".to_string()]], vec![2.0, 3.0], vec![vec!["x1".to_string(), "x2".to_string()], vec!["x3".to_string(), "x4".to_string()], vec!["x1".to_string(), "x3".to_string()]]);
    let want: Vec<f64> = vec![2.0, 3.0, -1.0];
    assert_eq!(got.len(), want.len());
    for (g, w) in got.iter().zip(want.iter()) {
        if w.is_nan() {
            assert!(g.is_nan());
        } else {
            assert!((g - w).abs() < 1e-5, "got {} want {}", g, w);
        }
    }
}

#[test]
fn test_5() {
    let got = Solution::calc_equation(vec![vec!["a".to_string(), "b".to_string()], vec!["b".to_string(), "c".to_string()], vec!["bc".to_string(), "cd".to_string()]], vec![1.5, 2.5, 5.0], vec![vec!["a".to_string(), "c".to_string()], vec!["c".to_string(), "b".to_string()], vec!["bc".to_string(), "cd".to_string()], vec!["cd".to_string(), "bc".to_string()]]);
    let want: Vec<f64> = vec![3.75, 0.4, 5.0, 0.2];
    assert_eq!(got.len(), want.len());
    for (g, w) in got.iter().zip(want.iter()) {
        if w.is_nan() {
            assert!(g.is_nan());
        } else {
            assert!((g - w).abs() < 1e-5, "got {} want {}", g, w);
        }
    }
}

#[test]
fn test_6() {
    let got = Solution::calc_equation(vec![vec!["x1".to_string(), "x2".to_string()], vec!["x2".to_string(), "x3".to_string()], vec!["x3".to_string(), "x4".to_string()]], vec![2.0, 3.0, 4.0], vec![vec!["x1".to_string(), "x4".to_string()], vec!["x4".to_string(), "x1".to_string()], vec!["x1".to_string(), "x5".to_string()]]);
    let want: Vec<f64> = vec![24.0, 0.041666666666666664, -1.0];
    assert_eq!(got.len(), want.len());
    for (g, w) in got.iter().zip(want.iter()) {
        if w.is_nan() {
            assert!(g.is_nan());
        } else {
            assert!((g - w).abs() < 1e-5, "got {} want {}", g, w);
        }
    }
}

#[test]
fn test_7() {
    let got = Solution::calc_equation(vec![vec!["m1".to_string(), "m2".to_string()], vec!["m2".to_string(), "m3".to_string()], vec!["m3".to_string(), "m4".to_string()], vec!["m4".to_string(), "m1".to_string()], vec!["m1".to_string(), "m5".to_string()]], vec![1.2, 2.3, 3.4, 4.5, 5.6], vec![vec!["m1".to_string(), "m3".to_string()], vec!["m3".to_string(), "m1".to_string()], vec!["m2".to_string(), "m4".to_string()], vec!["m4".to_string(), "m2".to_string()], vec!["m1".to_string(), "m6".to_string()]]);
    let want: Vec<f64> = vec![2.76, 0.3623188405797102, 7.8199999999999985, 0.1278772378516624, -1.0];
    assert_eq!(got.len(), want.len());
    for (g, w) in got.iter().zip(want.iter()) {
        if w.is_nan() {
            assert!(g.is_nan());
        } else {
            assert!((g - w).abs() < 1e-5, "got {} want {}", g, w);
        }
    }
}

#[test]
fn test_8() {
    let got = Solution::calc_equation(vec![vec!["a".to_string(), "b".to_string()], vec!["b".to_string(), "c".to_string()], vec!["c".to_string(), "d".to_string()], vec!["d".to_string(), "a".to_string()]], vec![2.0, 3.0, 0.5, 0.333], vec![vec!["a".to_string(), "c".to_string()], vec!["c".to_string(), "a".to_string()], vec!["b".to_string(), "d".to_string()], vec!["d".to_string(), "b".to_string()], vec!["a".to_string(), "a".to_string()], vec!["e".to_string(), "e".to_string()]]);
    let want: Vec<f64> = vec![6.0, 0.16666666666666666, 1.5, 0.6666666666666666, 1.0, -1.0];
    assert_eq!(got.len(), want.len());
    for (g, w) in got.iter().zip(want.iter()) {
        if w.is_nan() {
            assert!(g.is_nan());
        } else {
            assert!((g - w).abs() < 1e-5, "got {} want {}", g, w);
        }
    }
}

#[test]
fn test_9() {
    let got = Solution::calc_equation(vec![vec!["alpha".to_string(), "beta".to_string()], vec!["beta".to_string(), "gamma".to_string()], vec!["gamma".to_string(), "delta".to_string()], vec!["delta".to_string(), "epsilon".to_string()], vec!["epsilon".to_string(), "zeta".to_string()], vec!["zeta".to_string(), "eta".to_string()], vec!["eta".to_string(), "theta".to_string()], vec!["theta".to_string(), "iota".to_string()], vec!["iota".to_string(), "kappa".to_string()], vec!["kappa".to_string(), "lambda".to_string()]], vec![2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0], vec![vec!["alpha".to_string(), "lambda".to_string()], vec!["lambda".to_string(), "alpha".to_string()], vec!["beta".to_string(), "kappa".to_string()], vec!["kappa".to_string(), "beta".to_string()], vec!["gamma".to_string(), "iota".to_string()], vec!["iota".to_string(), "gamma".to_string()], vec!["delta".to_string(), "theta".to_string()], vec!["theta".to_string(), "delta".to_string()], vec!["epsilon".to_string(), "eta".to_string()], vec!["eta".to_string(), "epsilon".to_string()], vec!["alpha".to_string(), "mu".to_string()]]);
    let want: Vec<f64> = vec![39916800.0, 2.505210838544172e-08, 1814400.0, 5.511463844797178e-07, 60480.0, 1.6534391534391536e-05, 1680.0, 0.0005952380952380953, 42.0, 0.023809523809523808, -1.0];
    assert_eq!(got.len(), want.len());
    for (g, w) in got.iter().zip(want.iter()) {
        if w.is_nan() {
            assert!(g.is_nan());
        } else {
            assert!((g - w).abs() < 1e-5, "got {} want {}", g, w);
        }
    }
}

#[test]
fn test_10() {
    let got = Solution::calc_equation(vec![vec!["nodeA".to_string(), "nodeB".to_string()], vec!["nodeB".to_string(), "nodeC".to_string()], vec!["nodeC".to_string(), "nodeD".to_string()], vec!["nodeD".to_string(), "nodeE".to_string()], vec!["nodeE".to_string(), "nodeF".to_string()], vec!["nodeF".to_string(), "nodeG".to_string()], vec!["nodeG".to_string(), "nodeH".to_string()], vec!["nodeH".to_string(), "nodeI".to_string()], vec!["nodeI".to_string(), "nodeJ".to_string()], vec!["nodeJ".to_string(), "nodeA".to_string()]], vec![1.2, 1.3, 1.4, 1.5, 1.6, 1.7, 1.8, 1.9, 2.0, 2.1], vec![vec!["nodeA".to_string(), "nodeJ".to_string()], vec!["nodeJ".to_string(), "nodeA".to_string()], vec!["nodeB".to_string(), "nodeD".to_string()], vec!["nodeD".to_string(), "nodeB".to_string()], vec!["nodeE".to_string(), "nodeH".to_string()], vec!["nodeH".to_string(), "nodeE".to_string()], vec!["nodeF".to_string(), "nodeG".to_string()], vec!["nodeG".to_string(), "nodeF".to_string()], vec!["nodeA".to_string(), "nodeK".to_string()]]);
    let want: Vec<f64> = vec![60.9493248, 0.016407072650622703, 1.8199999999999998, 0.5494505494505495, 4.896000000000001, 0.20424836601307186, 1.7, 0.5882352941176471, -1.0];
    assert_eq!(got.len(), want.len());
    for (g, w) in got.iter().zip(want.iter()) {
        if w.is_nan() {
            assert!(g.is_nan());
        } else {
            assert!((g - w).abs() < 1e-5, "got {} want {}", g, w);
        }
    }
}

#[test]
fn test_11() {
    let got = Solution::calc_equation(vec![vec!["a".to_string(), "b".to_string()], vec!["b".to_string(), "c".to_string()], vec!["c".to_string(), "d".to_string()], vec!["d".to_string(), "e".to_string()], vec!["e".to_string(), "f".to_string()], vec!["f".to_string(), "g".to_string()]], vec![2.0, 3.0, 4.0, 5.0, 6.0, 7.0], vec![vec!["a".to_string(), "g".to_string()], vec!["g".to_string(), "a".to_string()], vec!["b".to_string(), "f".to_string()], vec!["f".to_string(), "b".to_string()], vec!["c".to_string(), "e".to_string()], vec!["e".to_string(), "c".to_string()], vec!["a".to_string(), "h".to_string()], vec!["h".to_string(), "a".to_string()]]);
    let want: Vec<f64> = vec![5040.0, 0.0001984126984126984, 360.0, 0.002777777777777778, 20.0, 0.05, -1.0, -1.0];
    assert_eq!(got.len(), want.len());
    for (g, w) in got.iter().zip(want.iter()) {
        if w.is_nan() {
            assert!(g.is_nan());
        } else {
            assert!((g - w).abs() < 1e-5, "got {} want {}", g, w);
        }
    }
}

#[test]
fn test_12() {
    let got = Solution::calc_equation(vec![vec!["alpha".to_string(), "beta".to_string()], vec!["beta".to_string(), "gamma".to_string()], vec!["gamma".to_string(), "delta".to_string()], vec!["delta".to_string(), "epsilon".to_string()]], vec![1.2, 1.3, 1.4, 1.5], vec![vec!["alpha".to_string(), "epsilon".to_string()], vec!["epsilon".to_string(), "alpha".to_string()], vec!["beta".to_string(), "delta".to_string()], vec!["gamma".to_string(), "beta".to_string()], vec!["alpha".to_string(), "zeta".to_string()]]);
    let want: Vec<f64> = vec![3.2759999999999994, 0.30525030525030533, 1.8199999999999996, 0.7692307692307693, -1.0];
    assert_eq!(got.len(), want.len());
    for (g, w) in got.iter().zip(want.iter()) {
        if w.is_nan() {
            assert!(g.is_nan());
        } else {
            assert!((g - w).abs() < 1e-5, "got {} want {}", g, w);
        }
    }
}

#[test]
fn test_13() {
    let got = Solution::calc_equation(vec![vec!["a".to_string(), "b".to_string()], vec!["b".to_string(), "c".to_string()], vec!["a".to_string(), "d".to_string()], vec!["d".to_string(), "e".to_string()], vec!["e".to_string(), "f".to_string()]], vec![2.0, 3.0, 4.0, 5.0, 6.0], vec![vec!["a".to_string(), "f".to_string()], vec!["f".to_string(), "a".to_string()], vec!["b".to_string(), "e".to_string()], vec!["c".to_string(), "d".to_string()], vec!["x".to_string(), "y".to_string()]]);
    let want: Vec<f64> = vec![120.0, 0.008333333333333333, 10.0, 0.6666666666666666, -1.0];
    assert_eq!(got.len(), want.len());
    for (g, w) in got.iter().zip(want.iter()) {
        if w.is_nan() {
            assert!(g.is_nan());
        } else {
            assert!((g - w).abs() < 1e-5, "got {} want {}", g, w);
        }
    }
}

#[test]
fn test_14() {
    let got = Solution::calc_equation(vec![vec!["var1".to_string(), "var2".to_string()], vec!["var2".to_string(), "var3".to_string()], vec!["var3".to_string(), "var4".to_string()], vec!["var4".to_string(), "var5".to_string()], vec!["var5".to_string(), "var6".to_string()], vec!["var6".to_string(), "var7".to_string()]], vec![2.0, 2.0, 2.0, 2.0, 2.0, 2.0], vec![vec!["var1".to_string(), "var7".to_string()], vec!["var7".to_string(), "var1".to_string()], vec!["var2".to_string(), "var6".to_string()], vec!["var3".to_string(), "var5".to_string()], vec!["x".to_string(), "y".to_string()]]);
    let want: Vec<f64> = vec![64.0, 0.015625, 16.0, 4.0, -1.0];
    assert_eq!(got.len(), want.len());
    for (g, w) in got.iter().zip(want.iter()) {
        if w.is_nan() {
            assert!(g.is_nan());
        } else {
            assert!((g - w).abs() < 1e-5, "got {} want {}", g, w);
        }
    }
}

#[test]
fn test_15() {
    let got = Solution::calc_equation(vec![vec!["alpha".to_string(), "beta".to_string()], vec!["beta".to_string(), "gamma".to_string()], vec!["gamma".to_string(), "delta".to_string()], vec!["delta".to_string(), "epsilon".to_string()], vec!["epsilon".to_string(), "zeta".to_string()]], vec![1.2, 1.3, 1.4, 1.5, 1.6], vec![vec!["alpha".to_string(), "zeta".to_string()], vec!["zeta".to_string(), "alpha".to_string()], vec!["alpha".to_string(), "epsilon".to_string()], vec!["epsilon".to_string(), "alpha".to_string()], vec!["alpha".to_string(), "delta".to_string()], vec!["delta".to_string(), "alpha".to_string()], vec!["alpha".to_string(), "gamma".to_string()], vec!["gamma".to_string(), "alpha".to_string()], vec!["alpha".to_string(), "beta".to_string()], vec!["beta".to_string(), "alpha".to_string()], vec!["zeta".to_string(), "beta".to_string()], vec!["beta".to_string(), "zeta".to_string()], vec!["zeta".to_string(), "gamma".to_string()], vec!["gamma".to_string(), "zeta".to_string()], vec!["zeta".to_string(), "delta".to_string()], vec!["delta".to_string(), "zeta".to_string()], vec!["zeta".to_string(), "epsilon".to_string()], vec!["epsilon".to_string(), "zeta".to_string()], vec!["a".to_string(), "a".to_string()], vec!["b".to_string(), "b".to_string()]]);
    let want: Vec<f64> = vec![5.2416, 0.19078144078144077, 3.276, 0.3052503052503053, 2.1839999999999997, 0.4578754578754579, 1.5599999999999998, 0.6410256410256411, 1.2, 0.8333333333333334, 0.22893772893772893, 4.368, 0.2976190476190476, 3.3600000000000003, 0.41666666666666663, 2.4000000000000004, 0.625, 1.6, -1.0, -1.0];
    assert_eq!(got.len(), want.len());
    for (g, w) in got.iter().zip(want.iter()) {
        if w.is_nan() {
            assert!(g.is_nan());
        } else {
            assert!((g - w).abs() < 1e-5, "got {} want {}", g, w);
        }
    }
}

#[test]
fn test_16() {
    let got = Solution::calc_equation(vec![vec!["node1".to_string(), "node2".to_string()], vec!["node2".to_string(), "node3".to_string()], vec!["node3".to_string(), "node4".to_string()], vec!["node4".to_string(), "node1".to_string()], vec!["node1".to_string(), "node5".to_string()]], vec![3.0, 4.0, 5.0, 6.0, 7.0], vec![vec!["node1".to_string(), "node5".to_string()], vec!["node5".to_string(), "node1".to_string()], vec!["node2".to_string(), "node4".to_string()], vec!["node3".to_string(), "node2".to_string()], vec!["node4".to_string(), "node3".to_string()], vec!["node1".to_string(), "node6".to_string()]]);
    let want: Vec<f64> = vec![7.0, 0.14285714285714285, 20.0, 0.25, 0.19999999999999998, -1.0];
    assert_eq!(got.len(), want.len());
    for (g, w) in got.iter().zip(want.iter()) {
        if w.is_nan() {
            assert!(g.is_nan());
        } else {
            assert!((g - w).abs() < 1e-5, "got {} want {}", g, w);
        }
    }
}

#[test]
fn test_17() {
    let got = Solution::calc_equation(vec![vec!["v1".to_string(), "v2".to_string()], vec!["v2".to_string(), "v3".to_string()], vec!["v3".to_string(), "v4".to_string()], vec!["v4".to_string(), "v1".to_string()]], vec![3.0, 4.0, 5.0, 6.0], vec![vec!["v1".to_string(), "v4".to_string()], vec!["v4".to_string(), "v1".to_string()], vec!["v2".to_string(), "v3".to_string()], vec!["v3".to_string(), "v2".to_string()], vec!["v5".to_string(), "v5".to_string()]]);
    let want: Vec<f64> = vec![60.0, 0.016666666666666666, 4.0, 0.25, -1.0];
    assert_eq!(got.len(), want.len());
    for (g, w) in got.iter().zip(want.iter()) {
        if w.is_nan() {
            assert!(g.is_nan());
        } else {
            assert!((g - w).abs() < 1e-5, "got {} want {}", g, w);
        }
    }
}

#[test]
fn test_18() {
    let got = Solution::calc_equation(vec![vec!["var1".to_string(), "var2".to_string()], vec!["var2".to_string(), "var3".to_string()], vec!["var3".to_string(), "var4".to_string()], vec!["var4".to_string(), "var5".to_string()], vec!["var5".to_string(), "var6".to_string()], vec!["var6".to_string(), "var7".to_string()], vec!["var7".to_string(), "var8".to_string()], vec!["var8".to_string(), "var9".to_string()], vec!["var9".to_string(), "var10".to_string()]], vec![2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0], vec![vec!["var1".to_string(), "var10".to_string()], vec!["var10".to_string(), "var1".to_string()], vec!["var2".to_string(), "var9".to_string()], vec!["var9".to_string(), "var2".to_string()], vec!["var3".to_string(), "var8".to_string()], vec!["var8".to_string(), "var3".to_string()], vec!["var4".to_string(), "var7".to_string()], vec!["var7".to_string(), "var4".to_string()], vec!["var5".to_string(), "var6".to_string()], vec!["var6".to_string(), "var5".to_string()]]);
    let want: Vec<f64> = vec![3628800.0, 2.755731922398589e-07, 181440.0, 5.5114638447971785e-06, 6720.0, 0.00014880952380952382, 210.0, 0.004761904761904762, 6.0, 0.16666666666666666];
    assert_eq!(got.len(), want.len());
    for (g, w) in got.iter().zip(want.iter()) {
        if w.is_nan() {
            assert!(g.is_nan());
        } else {
            assert!((g - w).abs() < 1e-5, "got {} want {}", g, w);
        }
    }
}

#[test]
fn test_19() {
    let got = Solution::calc_equation(vec![vec!["a".to_string(), "b".to_string()], vec!["b".to_string(), "c".to_string()], vec!["c".to_string(), "d".to_string()], vec!["d".to_string(), "e".to_string()], vec!["e".to_string(), "f".to_string()]], vec![1.0, 2.0, 3.0, 4.0, 5.0], vec![vec!["a".to_string(), "f".to_string()], vec!["f".to_string(), "a".to_string()], vec!["b".to_string(), "d".to_string()], vec!["d".to_string(), "b".to_string()], vec!["c".to_string(), "e".to_string()], vec!["e".to_string(), "c".to_string()]]);
    let want: Vec<f64> = vec![120.0, 0.008333333333333333, 6.0, 0.16666666666666666, 12.0, 0.08333333333333333];
    assert_eq!(got.len(), want.len());
    for (g, w) in got.iter().zip(want.iter()) {
        if w.is_nan() {
            assert!(g.is_nan());
        } else {
            assert!((g - w).abs() < 1e-5, "got {} want {}", g, w);
        }
    }
}

#[test]
fn test_20() {
    let got = Solution::calc_equation(vec![vec!["u1".to_string(), "u2".to_string()], vec!["u2".to_string(), "u3".to_string()], vec!["u3".to_string(), "u4".to_string()], vec!["u4".to_string(), "u5".to_string()], vec!["u5".to_string(), "u6".to_string()], vec!["u6".to_string(), "u1".to_string()], vec!["u1".to_string(), "u7".to_string()], vec!["u7".to_string(), "u8".to_string()]], vec![1.1, 2.2, 3.3, 4.4, 5.5, 6.6, 7.7, 8.8], vec![vec!["u1".to_string(), "u8".to_string()], vec!["u8".to_string(), "u1".to_string()], vec!["u2".to_string(), "u7".to_string()], vec!["u7".to_string(), "u2".to_string()], vec!["u3".to_string(), "u6".to_string()], vec!["u6".to_string(), "u3".to_string()]]);
    let want: Vec<f64> = vec![67.76, 0.014757969303423848, 7.0, 0.14285714285714288, 79.86, 0.01252191334835963];
    assert_eq!(got.len(), want.len());
    for (g, w) in got.iter().zip(want.iter()) {
        if w.is_nan() {
            assert!(g.is_nan());
        } else {
            assert!((g - w).abs() < 1e-5, "got {} want {}", g, w);
        }
    }
}

#[test]
fn test_21() {
    let got = Solution::calc_equation(vec![vec!["var1".to_string(), "var2".to_string()], vec!["var2".to_string(), "var3".to_string()], vec!["var3".to_string(), "var4".to_string()], vec!["var4".to_string(), "var5".to_string()]], vec![2.0, 3.0, 4.0, 5.0], vec![vec!["var1".to_string(), "var5".to_string()], vec!["var5".to_string(), "var1".to_string()], vec!["var2".to_string(), "var4".to_string()], vec!["var3".to_string(), "var2".to_string()], vec!["var1".to_string(), "var6".to_string()]]);
    let want: Vec<f64> = vec![120.0, 0.008333333333333333, 12.0, 0.3333333333333333, -1.0];
    assert_eq!(got.len(), want.len());
    for (g, w) in got.iter().zip(want.iter()) {
        if w.is_nan() {
            assert!(g.is_nan());
        } else {
            assert!((g - w).abs() < 1e-5, "got {} want {}", g, w);
        }
    }
}

#[test]
fn test_22() {
    let got = Solution::calc_equation(vec![vec!["q1".to_string(), "q2".to_string()], vec!["q2".to_string(), "q3".to_string()], vec!["q3".to_string(), "q4".to_string()], vec!["q4".to_string(), "q5".to_string()], vec!["q5".to_string(), "q6".to_string()], vec!["q6".to_string(), "q1".to_string()], vec!["q1".to_string(), "q7".to_string()], vec!["q7".to_string(), "q8".to_string()], vec!["q8".to_string(), "q9".to_string()]], vec![0.5, 1.2, 1.8, 2.5, 3.0, 3.5, 4.0, 4.5, 5.0], vec![vec!["q1".to_string(), "q9".to_string()], vec!["q9".to_string(), "q1".to_string()], vec!["q2".to_string(), "q8".to_string()], vec!["q8".to_string(), "q2".to_string()], vec!["q3".to_string(), "q7".to_string()], vec!["q7".to_string(), "q3".to_string()]]);
    let want: Vec<f64> = vec![90.00000000000001, 0.01111111111111111, 36.00000000000001, 0.027777777777777773, 6.666666666666668, 0.14999999999999997];
    assert_eq!(got.len(), want.len());
    for (g, w) in got.iter().zip(want.iter()) {
        if w.is_nan() {
            assert!(g.is_nan());
        } else {
            assert!((g - w).abs() < 1e-5, "got {} want {}", g, w);
        }
    }
}

#[test]
fn test_23() {
    let got = Solution::calc_equation(vec![vec!["alpha".to_string(), "beta".to_string()], vec!["beta".to_string(), "gamma".to_string()], vec!["gamma".to_string(), "delta".to_string()], vec!["delta".to_string(), "epsilon".to_string()], vec!["epsilon".to_string(), "zeta".to_string()], vec!["zeta".to_string(), "eta".to_string()]], vec![0.1, 0.2, 0.3, 0.4, 0.5, 0.6], vec![vec!["alpha".to_string(), "zeta".to_string()], vec!["zeta".to_string(), "alpha".to_string()], vec!["beta".to_string(), "epsilon".to_string()], vec!["epsilon".to_string(), "beta".to_string()], vec!["gamma".to_string(), "eta".to_string()], vec!["eta".to_string(), "gamma".to_string()]]);
    let want: Vec<f64> = vec![0.0012000000000000001, 833.3333333333333, 0.024, 41.666666666666664, 0.036, 27.77777777777778];
    assert_eq!(got.len(), want.len());
    for (g, w) in got.iter().zip(want.iter()) {
        if w.is_nan() {
            assert!(g.is_nan());
        } else {
            assert!((g - w).abs() < 1e-5, "got {} want {}", g, w);
        }
    }
}

#[test]
fn test_24() {
    let got = Solution::calc_equation(vec![vec!["a".to_string(), "b".to_string()], vec!["b".to_string(), "c".to_string()], vec!["c".to_string(), "d".to_string()], vec!["d".to_string(), "e".to_string()]], vec![2.0, 0.5, 3.0, 0.25], vec![vec!["a".to_string(), "e".to_string()], vec!["e".to_string(), "a".to_string()], vec!["a".to_string(), "d".to_string()], vec!["d".to_string(), "a".to_string()], vec!["b".to_string(), "c".to_string()], vec!["c".to_string(), "b".to_string()], vec!["a".to_string(), "x".to_string()]]);
    let want: Vec<f64> = vec![0.75, 1.3333333333333333, 3.0, 0.3333333333333333, 0.5, 2.0, -1.0];
    assert_eq!(got.len(), want.len());
    for (g, w) in got.iter().zip(want.iter()) {
        if w.is_nan() {
            assert!(g.is_nan());
        } else {
            assert!((g - w).abs() < 1e-5, "got {} want {}", g, w);
        }
    }
}

#[test]
fn test_25() {
    let got = Solution::calc_equation(vec![vec!["alpha".to_string(), "beta".to_string()], vec!["beta".to_string(), "gamma".to_string()], vec!["gamma".to_string(), "delta".to_string()], vec!["delta".to_string(), "epsilon".to_string()], vec!["epsilon".to_string(), "zeta".to_string()], vec!["zeta".to_string(), "eta".to_string()], vec!["eta".to_string(), "theta".to_string()], vec!["theta".to_string(), "iota".to_string()]], vec![1.2, 1.3, 1.4, 1.5, 1.6, 1.7, 1.8, 1.9], vec![vec!["alpha".to_string(), "iota".to_string()], vec!["iota".to_string(), "alpha".to_string()], vec!["beta".to_string(), "theta".to_string()], vec!["gamma".to_string(), "delta".to_string()], vec!["z".to_string(), "z".to_string()]]);
    let want: Vec<f64> = vec![30.4746624, 0.032814145301245407, 13.366080000000002, 1.4, -1.0];
    assert_eq!(got.len(), want.len());
    for (g, w) in got.iter().zip(want.iter()) {
        if w.is_nan() {
            assert!(g.is_nan());
        } else {
            assert!((g - w).abs() < 1e-5, "got {} want {}", g, w);
        }
    }
}

#[test]
fn test_26() {
    let got = Solution::calc_equation(vec![vec!["v1".to_string(), "v2".to_string()], vec!["v2".to_string(), "v3".to_string()], vec!["v3".to_string(), "v4".to_string()], vec!["v4".to_string(), "v5".to_string()], vec!["v5".to_string(), "v6".to_string()], vec!["v6".to_string(), "v7".to_string()], vec!["v7".to_string(), "v8".to_string()], vec!["v8".to_string(), "v9".to_string()], vec!["v9".to_string(), "v10".to_string()], vec!["v10".to_string(), "v11".to_string()], vec!["v11".to_string(), "v12".to_string()], vec!["v12".to_string(), "v13".to_string()], vec!["v13".to_string(), "v14".to_string()], vec!["v14".to_string(), "v15".to_string()]], vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0], vec![vec!["v1".to_string(), "v15".to_string()], vec!["v15".to_string(), "v14".to_string()], vec!["v14".to_string(), "v13".to_string()], vec!["v13".to_string(), "v12".to_string()], vec!["v12".to_string(), "v11".to_string()], vec!["v11".to_string(), "v10".to_string()], vec!["v10".to_string(), "v9".to_string()], vec!["v9".to_string(), "v8".to_string()], vec!["v8".to_string(), "v7".to_string()], vec!["v7".to_string(), "v6".to_string()], vec!["v6".to_string(), "v5".to_string()], vec!["v5".to_string(), "v4".to_string()], vec!["v4".to_string(), "v3".to_string()], vec!["v3".to_string(), "v2".to_string()], vec!["v2".to_string(), "v1".to_string()], vec!["v1".to_string(), "v16".to_string()]]);
    let want: Vec<f64> = vec![87178291200.0, 0.07142857142857142, 0.07692307692307693, 0.08333333333333333, 0.09090909090909091, 0.1, 0.1111111111111111, 0.125, 0.14285714285714285, 0.16666666666666666, 0.2, 0.25, 0.3333333333333333, 0.5, 1.0, -1.0];
    assert_eq!(got.len(), want.len());
    for (g, w) in got.iter().zip(want.iter()) {
        if w.is_nan() {
            assert!(g.is_nan());
        } else {
            assert!((g - w).abs() < 1e-5, "got {} want {}", g, w);
        }
    }
}

#[test]
fn test_27() {
    let got = Solution::calc_equation(vec![vec!["m1".to_string(), "m2".to_string()], vec!["m2".to_string(), "m3".to_string()], vec!["m3".to_string(), "m4".to_string()], vec!["m4".to_string(), "m5".to_string()], vec!["m5".to_string(), "m6".to_string()]], vec![1.1, 1.2, 1.3, 1.4, 1.5], vec![vec!["m1".to_string(), "m6".to_string()], vec!["m6".to_string(), "m1".to_string()], vec!["m2".to_string(), "m4".to_string()], vec!["m3".to_string(), "m5".to_string()], vec!["m1".to_string(), "m7".to_string()]]);
    let want: Vec<f64> = vec![3.6035999999999997, 0.2775002775002775, 1.56, 1.8199999999999996, -1.0];
    assert_eq!(got.len(), want.len());
    for (g, w) in got.iter().zip(want.iter()) {
        if w.is_nan() {
            assert!(g.is_nan());
        } else {
            assert!((g - w).abs() < 1e-5, "got {} want {}", g, w);
        }
    }
}

#[test]
fn test_28() {
    let got = Solution::calc_equation(vec![vec!["x1".to_string(), "y1".to_string()], vec!["y1".to_string(), "z1".to_string()], vec!["x2".to_string(), "y2".to_string()], vec!["y2".to_string(), "z2".to_string()], vec!["x3".to_string(), "y3".to_string()], vec!["y3".to_string(), "z3".to_string()], vec!["x4".to_string(), "y4".to_string()], vec!["y4".to_string(), "z4".to_string()]], vec![0.5, 2.0, 1.5, 3.0, 2.5, 4.0, 3.5, 5.0], vec![vec!["x1".to_string(), "z4".to_string()], vec!["z4".to_string(), "x1".to_string()], vec!["y2".to_string(), "z2".to_string()], vec!["x3".to_string(), "y3".to_string()], vec!["a".to_string(), "b".to_string()]]);
    let want: Vec<f64> = vec![-1.0, -1.0, 3.0, 2.5, -1.0];
    assert_eq!(got.len(), want.len());
    for (g, w) in got.iter().zip(want.iter()) {
        if w.is_nan() {
            assert!(g.is_nan());
        } else {
            assert!((g - w).abs() < 1e-5, "got {} want {}", g, w);
        }
    }
}

#[test]
fn test_29() {
    let got = Solution::calc_equation(vec![vec!["a1".to_string(), "b1".to_string()], vec!["b1".to_string(), "c1".to_string()], vec!["c1".to_string(), "d1".to_string()], vec!["d1".to_string(), "e1".to_string()], vec!["e1".to_string(), "f1".to_string()], vec!["f1".to_string(), "g1".to_string()], vec!["g1".to_string(), "h1".to_string()]], vec![1.2, 2.3, 3.4, 4.5, 5.6, 6.7, 7.8], vec![vec!["a1".to_string(), "h1".to_string()], vec!["h1".to_string(), "a1".to_string()], vec!["b1".to_string(), "g1".to_string()], vec!["g1".to_string(), "b1".to_string()], vec!["c1".to_string(), "f1".to_string()], vec!["f1".to_string(), "c1".to_string()]]);
    let want: Vec<f64> = vec![12358.277567999996, 8.09174251425909e-05, 1320.3287999999995, 0.0007573870993346508, 85.67999999999999, 0.011671335200746967];
    assert_eq!(got.len(), want.len());
    for (g, w) in got.iter().zip(want.iter()) {
        if w.is_nan() {
            assert!(g.is_nan());
        } else {
            assert!((g - w).abs() < 1e-5, "got {} want {}", g, w);
        }
    }
}

#[test]
fn test_30() {
    let got = Solution::calc_equation(vec![vec!["m1".to_string(), "n1".to_string()], vec!["n1".to_string(), "o1".to_string()], vec!["o1".to_string(), "p1".to_string()], vec!["p1".to_string(), "q1".to_string()]], vec![2.5, 3.5, 4.5, 5.5], vec![vec!["m1".to_string(), "q1".to_string()], vec!["q1".to_string(), "m1".to_string()], vec!["n1".to_string(), "p1".to_string()], vec!["p1".to_string(), "n1".to_string()], vec!["m1".to_string(), "x1".to_string()], vec!["x1".to_string(), "m1".to_string()]]);
    let want: Vec<f64> = vec![216.5625, 0.004617604617604618, 15.75, 0.06349206349206349, -1.0, -1.0];
    assert_eq!(got.len(), want.len());
    for (g, w) in got.iter().zip(want.iter()) {
        if w.is_nan() {
            assert!(g.is_nan());
        } else {
            assert!((g - w).abs() < 1e-5, "got {} want {}", g, w);
        }
    }
}

#[test]
fn test_31() {
    let got = Solution::calc_equation(vec![vec!["a".to_string(), "b".to_string()], vec!["b".to_string(), "c".to_string()], vec!["a".to_string(), "d".to_string()], vec!["d".to_string(), "e".to_string()]], vec![2.0, 3.0, 4.0, 5.0], vec![vec!["a".to_string(), "c".to_string()], vec!["c".to_string(), "a".to_string()], vec!["a".to_string(), "e".to_string()], vec!["e".to_string(), "a".to_string()], vec!["x".to_string(), "x".to_string()]]);
    let want: Vec<f64> = vec![6.000000000000001, 0.16666666666666666, 20.0, 0.05, -1.0];
    assert_eq!(got.len(), want.len());
    for (g, w) in got.iter().zip(want.iter()) {
        if w.is_nan() {
            assert!(g.is_nan());
        } else {
            assert!((g - w).abs() < 1e-5, "got {} want {}", g, w);
        }
    }
}

#[test]
fn test_32() {
    let got = Solution::calc_equation(vec![vec!["x1".to_string(), "x2".to_string()], vec!["x2".to_string(), "x3".to_string()], vec!["x3".to_string(), "x4".to_string()], vec!["x4".to_string(), "x5".to_string()], vec!["x5".to_string(), "x6".to_string()]], vec![1.1, 1.2, 1.3, 1.4, 1.5], vec![vec!["x1".to_string(), "x6".to_string()], vec!["x6".to_string(), "x1".to_string()], vec!["x1".to_string(), "x5".to_string()], vec!["x5".to_string(), "x1".to_string()], vec!["x1".to_string(), "x4".to_string()], vec!["x4".to_string(), "x1".to_string()], vec!["x1".to_string(), "x3".to_string()], vec!["x3".to_string(), "x1".to_string()], vec!["x1".to_string(), "x2".to_string()], vec!["x2".to_string(), "x1".to_string()], vec!["x6".to_string(), "x2".to_string()], vec!["x2".to_string(), "x6".to_string()], vec!["x6".to_string(), "x3".to_string()], vec!["x3".to_string(), "x6".to_string()], vec!["x6".to_string(), "x4".to_string()], vec!["x4".to_string(), "x6".to_string()], vec!["x6".to_string(), "x5".to_string()], vec!["x5".to_string(), "x6".to_string()]]);
    let want: Vec<f64> = vec![3.6035999999999997, 0.2775002775002775, 2.4023999999999996, 0.4162504162504163, 1.7160000000000002, 0.5827505827505827, 1.32, 0.7575757575757575, 1.1, 0.909090909090909, 0.30525030525030533, 3.2759999999999994, 0.3663003663003664, 2.7299999999999995, 0.4761904761904763, 2.0999999999999996, 0.6666666666666666, 1.5];
    assert_eq!(got.len(), want.len());
    for (g, w) in got.iter().zip(want.iter()) {
        if w.is_nan() {
            assert!(g.is_nan());
        } else {
            assert!((g - w).abs() < 1e-5, "got {} want {}", g, w);
        }
    }
}

#[test]
fn test_33() {
    let got = Solution::calc_equation(vec![vec!["p1".to_string(), "q1".to_string()], vec!["p2".to_string(), "q2".to_string()], vec!["q1".to_string(), "q2".to_string()], vec!["q2".to_string(), "p1".to_string()], vec!["p1".to_string(), "p2".to_string()]], vec![1.1, 2.2, 3.3, 4.4, 5.5], vec![vec!["p1".to_string(), "p2".to_string()], vec!["p2".to_string(), "p1".to_string()], vec!["q1".to_string(), "q2".to_string()], vec!["q2".to_string(), "q1".to_string()], vec!["p1".to_string(), "q3".to_string()]]);
    let want: Vec<f64> = vec![1.65, 0.6060606060606061, 3.3, 0.30303030303030304, -1.0];
    assert_eq!(got.len(), want.len());
    for (g, w) in got.iter().zip(want.iter()) {
        if w.is_nan() {
            assert!(g.is_nan());
        } else {
            assert!((g - w).abs() < 1e-5, "got {} want {}", g, w);
        }
    }
}

#[test]
fn test_34() {
    let got = Solution::calc_equation(vec![vec!["p1".to_string(), "p2".to_string()], vec!["p2".to_string(), "p3".to_string()], vec!["p3".to_string(), "p4".to_string()], vec!["p4".to_string(), "p5".to_string()]], vec![0.5, 1.5, 2.5, 3.5], vec![vec!["p1".to_string(), "p5".to_string()], vec!["p5".to_string(), "p1".to_string()], vec!["p2".to_string(), "p4".to_string()], vec!["p4".to_string(), "p2".to_string()], vec!["p1".to_string(), "p6".to_string()]]);
    let want: Vec<f64> = vec![6.5625, 0.1523809523809524, 3.75, 0.26666666666666666, -1.0];
    assert_eq!(got.len(), want.len());
    for (g, w) in got.iter().zip(want.iter()) {
        if w.is_nan() {
            assert!(g.is_nan());
        } else {
            assert!((g - w).abs() < 1e-5, "got {} want {}", g, w);
        }
    }
}

#[test]
fn test_35() {
    let got = Solution::calc_equation(vec![vec!["n1".to_string(), "n2".to_string()], vec!["n2".to_string(), "n3".to_string()], vec!["n3".to_string(), "n4".to_string()], vec!["n4".to_string(), "n1".to_string()], vec!["n1".to_string(), "n5".to_string()], vec!["n5".to_string(), "n6".to_string()]], vec![0.7, 1.1, 1.3, 1.7, 2.1, 2.3], vec![vec!["n1".to_string(), "n4".to_string()], vec!["n4".to_string(), "n1".to_string()], vec!["n2".to_string(), "n5".to_string()], vec!["n5".to_string(), "n2".to_string()], vec!["n3".to_string(), "n6".to_string()], vec!["n6".to_string(), "n3".to_string()]]);
    let want: Vec<f64> = vec![1.0010000000000001, 0.9990009990009989, 3.0000000000000004, 0.3333333333333333, 6.2727272727272725, 0.15942028985507248];
    assert_eq!(got.len(), want.len());
    for (g, w) in got.iter().zip(want.iter()) {
        if w.is_nan() {
            assert!(g.is_nan());
        } else {
            assert!((g - w).abs() < 1e-5, "got {} want {}", g, w);
        }
    }
}

#[test]
fn test_36() {
    let got = Solution::calc_equation(vec![vec!["p".to_string(), "q".to_string()], vec!["q".to_string(), "r".to_string()], vec!["r".to_string(), "s".to_string()], vec!["s".to_string(), "t".to_string()], vec!["t".to_string(), "u".to_string()]], vec![1.1, 1.2, 1.3, 1.4, 1.5], vec![vec!["p".to_string(), "u".to_string()], vec!["u".to_string(), "p".to_string()], vec!["q".to_string(), "t".to_string()], vec!["r".to_string(), "s".to_string()], vec!["z".to_string(), "z".to_string()]]);
    let want: Vec<f64> = vec![3.6035999999999997, 0.2775002775002775, 2.1839999999999997, 1.3, -1.0];
    assert_eq!(got.len(), want.len());
    for (g, w) in got.iter().zip(want.iter()) {
        if w.is_nan() {
            assert!(g.is_nan());
        } else {
            assert!((g - w).abs() < 1e-5, "got {} want {}", g, w);
        }
    }
}

#[test]
fn test_37() {
    let got = Solution::calc_equation(vec![vec!["m1".to_string(), "n1".to_string()], vec!["n1".to_string(), "o1".to_string()], vec!["o1".to_string(), "p1".to_string()], vec!["p1".to_string(), "q1".to_string()]], vec![1.5, 2.5, 3.5, 4.5], vec![vec!["m1".to_string(), "o1".to_string()], vec!["o1".to_string(), "m1".to_string()], vec!["n1".to_string(), "q1".to_string()], vec!["q1".to_string(), "n1".to_string()], vec!["p1".to_string(), "p1".to_string()]]);
    let want: Vec<f64> = vec![3.75, 0.26666666666666666, 39.375, 0.025396825396825397, 1.0];
    assert_eq!(got.len(), want.len());
    for (g, w) in got.iter().zip(want.iter()) {
        if w.is_nan() {
            assert!(g.is_nan());
        } else {
            assert!((g - w).abs() < 1e-5, "got {} want {}", g, w);
        }
    }
}

#[test]
fn test_38() {
    let got = Solution::calc_equation(vec![vec!["p".to_string(), "q".to_string()], vec!["q".to_string(), "r".to_string()], vec!["r".to_string(), "s".to_string()], vec!["s".to_string(), "t".to_string()]], vec![0.5, 2.0, 3.0, 4.0], vec![vec!["p".to_string(), "t".to_string()], vec!["t".to_string(), "p".to_string()], vec!["q".to_string(), "s".to_string()], vec!["s".to_string(), "q".to_string()], vec!["r".to_string(), "p".to_string()], vec!["x".to_string(), "y".to_string()]]);
    let want: Vec<f64> = vec![12.0, 0.08333333333333333, 6.0, 0.16666666666666666, 1.0, -1.0];
    assert_eq!(got.len(), want.len());
    for (g, w) in got.iter().zip(want.iter()) {
        if w.is_nan() {
            assert!(g.is_nan());
        } else {
            assert!((g - w).abs() < 1e-5, "got {} want {}", g, w);
        }
    }
}

#[test]
fn test_39() {
    let got = Solution::calc_equation(vec![vec!["u1".to_string(), "v1".to_string()], vec!["v1".to_string(), "w1".to_string()], vec!["u2".to_string(), "v2".to_string()], vec!["v2".to_string(), "w2".to_string()], vec!["u3".to_string(), "v3".to_string()], vec!["v3".to_string(), "w3".to_string()], vec!["u4".to_string(), "v4".to_string()], vec!["v4".to_string(), "w4".to_string()]], vec![2.1, 2.2, 3.1, 3.2, 4.1, 4.2, 5.1, 5.2], vec![vec!["u1".to_string(), "w4".to_string()], vec!["w4".to_string(), "u1".to_string()], vec!["v2".to_string(), "w2".to_string()], vec!["u3".to_string(), "v3".to_string()], vec!["a".to_string(), "b".to_string()]]);
    let want: Vec<f64> = vec![-1.0, -1.0, 3.2, 4.1, -1.0];
    assert_eq!(got.len(), want.len());
    for (g, w) in got.iter().zip(want.iter()) {
        if w.is_nan() {
            assert!(g.is_nan());
        } else {
            assert!((g - w).abs() < 1e-5, "got {} want {}", g, w);
        }
    }
}

#[test]
fn test_40() {
    let got = Solution::calc_equation(vec![vec!["x0".to_string(), "y0".to_string()], vec!["y0".to_string(), "z0".to_string()], vec!["z0".to_string(), "x1".to_string()], vec!["x1".to_string(), "y1".to_string()], vec!["y1".to_string(), "z1".to_string()], vec!["z1".to_string(), "w1".to_string()], vec!["w1".to_string(), "x2".to_string()]], vec![1.1, 2.2, 3.3, 4.4, 5.5, 6.6, 7.7], vec![vec!["x0".to_string(), "x2".to_string()], vec!["x2".to_string(), "x0".to_string()], vec!["y0".to_string(), "w1".to_string()], vec!["w1".to_string(), "y0".to_string()], vec!["z0".to_string(), "z1".to_string()], vec!["z1".to_string(), "z0".to_string()], vec!["x1".to_string(), "y1".to_string()]]);
    let want: Vec<f64> = vec![9821.534184000002, 0.00010181708695053704, 1159.5672000000002, 0.0008623907264710488, 79.86, 0.012521913348359628, 4.4];
    assert_eq!(got.len(), want.len());
    for (g, w) in got.iter().zip(want.iter()) {
        if w.is_nan() {
            assert!(g.is_nan());
        } else {
            assert!((g - w).abs() < 1e-5, "got {} want {}", g, w);
        }
    }
}

#[test]
fn test_41() {
    let got = Solution::calc_equation(vec![vec!["a".to_string(), "b".to_string()], vec!["b".to_string(), "c".to_string()], vec!["c".to_string(), "d".to_string()], vec!["d".to_string(), "e".to_string()], vec!["e".to_string(), "f".to_string()], vec!["f".to_string(), "g".to_string()], vec!["g".to_string(), "h".to_string()], vec!["h".to_string(), "i".to_string()], vec!["i".to_string(), "j".to_string()]], vec![2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0], vec![vec!["a".to_string(), "j".to_string()], vec!["j".to_string(), "a".to_string()], vec!["b".to_string(), "d".to_string()], vec!["d".to_string(), "b".to_string()], vec!["e".to_string(), "h".to_string()], vec!["h".to_string(), "e".to_string()], vec!["f".to_string(), "g".to_string()], vec!["g".to_string(), "f".to_string()], vec!["a".to_string(), "k".to_string()]]);
    let want: Vec<f64> = vec![3628800.0, 2.755731922398589e-07, 12.0, 0.08333333333333333, 336.0, 0.002976190476190476, 7.0, 0.14285714285714285, -1.0];
    assert_eq!(got.len(), want.len());
    for (g, w) in got.iter().zip(want.iter()) {
        if w.is_nan() {
            assert!(g.is_nan());
        } else {
            assert!((g - w).abs() < 1e-5, "got {} want {}", g, w);
        }
    }
}

#[test]
fn test_42() {
    let got = Solution::calc_equation(vec![vec!["a".to_string(), "b".to_string()], vec!["b".to_string(), "c".to_string()], vec!["c".to_string(), "a".to_string()], vec!["d".to_string(), "e".to_string()], vec!["e".to_string(), "f".to_string()], vec!["f".to_string(), "d".to_string()]], vec![2.0, 3.0, 0.5, 4.0, 5.0, 0.25], vec![vec!["a".to_string(), "f".to_string()], vec!["f".to_string(), "a".to_string()], vec!["b".to_string(), "e".to_string()], vec!["c".to_string(), "d".to_string()], vec!["x".to_string(), "y".to_string()]]);
    let want: Vec<f64> = vec![-1.0, -1.0, -1.0, -1.0, -1.0];
    assert_eq!(got.len(), want.len());
    for (g, w) in got.iter().zip(want.iter()) {
        if w.is_nan() {
            assert!(g.is_nan());
        } else {
            assert!((g - w).abs() < 1e-5, "got {} want {}", g, w);
        }
    }
}

#[test]
fn test_43() {
    let got = Solution::calc_equation(vec![vec!["m1".to_string(), "n1".to_string()], vec!["n1".to_string(), "o1".to_string()], vec!["m2".to_string(), "n2".to_string()], vec!["n2".to_string(), "o2".to_string()], vec!["m3".to_string(), "n3".to_string()], vec!["n3".to_string(), "o3".to_string()]], vec![1.5, 2.5, 3.5, 4.5, 5.5, 6.5], vec![vec!["m1".to_string(), "o3".to_string()], vec!["o3".to_string(), "m1".to_string()], vec!["n2".to_string(), "o2".to_string()], vec!["m3".to_string(), "n3".to_string()], vec!["a".to_string(), "b".to_string()]]);
    let want: Vec<f64> = vec![-1.0, -1.0, 4.5, 5.5, -1.0];
    assert_eq!(got.len(), want.len());
    for (g, w) in got.iter().zip(want.iter()) {
        if w.is_nan() {
            assert!(g.is_nan());
        } else {
            assert!((g - w).abs() < 1e-5, "got {} want {}", g, w);
        }
    }
}

#[test]
fn test_44() {
    let got = Solution::calc_equation(vec![vec!["x1".to_string(), "y1".to_string()], vec!["y1".to_string(), "z1".to_string()], vec!["z1".to_string(), "w1".to_string()], vec!["x1".to_string(), "w1".to_string()], vec!["w1".to_string(), "x1".to_string()], vec!["y1".to_string(), "w1".to_string()]], vec![2.0, 3.0, 4.0, 24.0, 0.0416667, 12.0], vec![vec!["x1".to_string(), "z1".to_string()], vec!["z1".to_string(), "x1".to_string()], vec!["y1".to_string(), "w1".to_string()], vec!["w1".to_string(), "y1".to_string()], vec!["x1".to_string(), "y1".to_string()], vec!["y1".to_string(), "x1".to_string()]]);
    let want: Vec<f64> = vec![6.0, 0.16666666666666666, 12.0, 0.08333333333333333, 2.0, 0.5];
    assert_eq!(got.len(), want.len());
    for (g, w) in got.iter().zip(want.iter()) {
        if w.is_nan() {
            assert!(g.is_nan());
        } else {
            assert!((g - w).abs() < 1e-5, "got {} want {}", g, w);
        }
    }
}

#[test]
fn test_45() {
    let got = Solution::calc_equation(vec![vec!["x1".to_string(), "x2".to_string()], vec!["x2".to_string(), "x3".to_string()], vec!["x3".to_string(), "x4".to_string()], vec!["x4".to_string(), "x5".to_string()]], vec![2.0, 3.0, 4.0, 5.0], vec![vec!["x1".to_string(), "x5".to_string()], vec!["x5".to_string(), "x1".to_string()], vec!["x2".to_string(), "x4".to_string()], vec!["x3".to_string(), "x2".to_string()], vec!["x1".to_string(), "x6".to_string()]]);
    let want: Vec<f64> = vec![120.0, 0.008333333333333333, 12.0, 0.3333333333333333, -1.0];
    assert_eq!(got.len(), want.len());
    for (g, w) in got.iter().zip(want.iter()) {
        if w.is_nan() {
            assert!(g.is_nan());
        } else {
            assert!((g - w).abs() < 1e-5, "got {} want {}", g, w);
        }
    }
}

#[test]
fn test_46() {
    let got = Solution::calc_equation(vec![vec!["x1".to_string(), "y1".to_string()], vec!["x2".to_string(), "y2".to_string()], vec!["x3".to_string(), "y3".to_string()], vec!["y1".to_string(), "y2".to_string()], vec!["y2".to_string(), "y3".to_string()], vec!["y3".to_string(), "x1".to_string()]], vec![1.1, 2.2, 3.3, 4.4, 5.5, 6.6], vec![vec!["x1".to_string(), "y3".to_string()], vec!["y3".to_string(), "x1".to_string()], vec!["x2".to_string(), "y1".to_string()], vec!["y1".to_string(), "x2".to_string()], vec!["x3".to_string(), "y2".to_string()], vec!["y2".to_string(), "x3".to_string()], vec!["x1".to_string(), "x4".to_string()]]);
    let want: Vec<f64> = vec![26.620000000000005, 0.03756574004507888, 0.5, 2.0, 0.6, 1.6666666666666667, -1.0];
    assert_eq!(got.len(), want.len());
    for (g, w) in got.iter().zip(want.iter()) {
        if w.is_nan() {
            assert!(g.is_nan());
        } else {
            assert!((g - w).abs() < 1e-5, "got {} want {}", g, w);
        }
    }
}

#[test]
fn test_47() {
    let got = Solution::calc_equation(vec![vec!["var1".to_string(), "var2".to_string()], vec!["var2".to_string(), "var3".to_string()], vec!["var3".to_string(), "var4".to_string()], vec!["var4".to_string(), "var5".to_string()], vec!["var5".to_string(), "var6".to_string()], vec!["var6".to_string(), "var7".to_string()], vec!["var7".to_string(), "var8".to_string()], vec!["var8".to_string(), "var9".to_string()]], vec![1.11, 2.22, 3.33, 4.44, 5.55, 6.66, 7.77, 8.88], vec![vec!["var1".to_string(), "var9".to_string()], vec!["var9".to_string(), "var1".to_string()], vec!["var2".to_string(), "var8".to_string()], vec!["var8".to_string(), "var2".to_string()], vec!["var3".to_string(), "var7".to_string()], vec!["var7".to_string(), "var3".to_string()]]);
    let want: Vec<f64> = vec![92918.9628750124, 1.0762065880407262e-05, 9426.889342891443, 0.00010607953096999833, 546.5053476000002, 0.0018298082615138891];
    assert_eq!(got.len(), want.len());
    for (g, w) in got.iter().zip(want.iter()) {
        if w.is_nan() {
            assert!(g.is_nan());
        } else {
            assert!((g - w).abs() < 1e-5, "got {} want {}", g, w);
        }
    }
}

#[test]
fn test_48() {
    let got = Solution::calc_equation(vec![vec!["var1".to_string(), "var2".to_string()], vec!["var2".to_string(), "var3".to_string()], vec!["var3".to_string(), "var4".to_string()], vec!["var4".to_string(), "var5".to_string()], vec!["var5".to_string(), "var1".to_string()]], vec![1.1, 2.2, 3.3, 4.4, 5.5], vec![vec!["var1".to_string(), "var3".to_string()], vec!["var3".to_string(), "var1".to_string()], vec!["var2".to_string(), "var4".to_string()], vec!["var4".to_string(), "var2".to_string()], vec!["var1".to_string(), "var5".to_string()], vec!["var5".to_string(), "var1".to_string()], vec!["var1".to_string(), "var6".to_string()]]);
    let want: Vec<f64> = vec![2.4200000000000004, 0.4132231404958677, 7.26, 0.13774104683195593, 35.138400000000004, 0.02845889397354461, -1.0];
    assert_eq!(got.len(), want.len());
    for (g, w) in got.iter().zip(want.iter()) {
        if w.is_nan() {
            assert!(g.is_nan());
        } else {
            assert!((g - w).abs() < 1e-5, "got {} want {}", g, w);
        }
    }
}

#[test]
fn test_49() {
    let got = Solution::calc_equation(vec![vec!["v1".to_string(), "v2".to_string()], vec!["v2".to_string(), "v3".to_string()], vec!["v3".to_string(), "v4".to_string()], vec!["v4".to_string(), "v5".to_string()], vec!["v5".to_string(), "v6".to_string()]], vec![1.2, 1.5, 1.7, 1.9, 2.1], vec![vec!["v1".to_string(), "v6".to_string()], vec!["v6".to_string(), "v1".to_string()], vec!["v2".to_string(), "v5".to_string()], vec!["v5".to_string(), "v2".to_string()], vec!["v3".to_string(), "v4".to_string()]]);
    let want: Vec<f64> = vec![12.209399999999997, 0.08190410667190855, 4.844999999999999, 0.20639834881320954, 1.7];
    assert_eq!(got.len(), want.len());
    for (g, w) in got.iter().zip(want.iter()) {
        if w.is_nan() {
            assert!(g.is_nan());
        } else {
            assert!((g - w).abs() < 1e-5, "got {} want {}", g, w);
        }
    }
}

#[test]
fn test_50() {
    let got = Solution::calc_equation(vec![vec!["p1".to_string(), "p2".to_string()], vec!["p2".to_string(), "p3".to_string()], vec!["p3".to_string(), "p4".to_string()], vec!["p4".to_string(), "p5".to_string()], vec!["p5".to_string(), "p1".to_string()]], vec![2.0, 3.0, 0.5, 0.25, 4.0], vec![vec!["p1".to_string(), "p3".to_string()], vec!["p3".to_string(), "p1".to_string()], vec!["p2".to_string(), "p5".to_string()], vec!["p4".to_string(), "p2".to_string()], vec!["p1".to_string(), "p6".to_string()]]);
    let want: Vec<f64> = vec![6.0, 0.16666666666666666, 0.375, 0.6666666666666666, -1.0];
    assert_eq!(got.len(), want.len());
    for (g, w) in got.iter().zip(want.iter()) {
        if w.is_nan() {
            assert!(g.is_nan());
        } else {
            assert!((g - w).abs() < 1e-5, "got {} want {}", g, w);
        }
    }
}

#[test]
fn test_51() {
    let got = Solution::calc_equation(vec![vec!["m".to_string(), "n".to_string()], vec!["n".to_string(), "o".to_string()], vec!["o".to_string(), "p".to_string()], vec!["p".to_string(), "q".to_string()], vec!["q".to_string(), "r".to_string()]], vec![2.0, 3.0, 4.0, 5.0, 6.0], vec![vec!["m".to_string(), "r".to_string()], vec!["r".to_string(), "q".to_string()], vec!["q".to_string(), "p".to_string()], vec!["p".to_string(), "o".to_string()], vec!["o".to_string(), "n".to_string()], vec!["n".to_string(), "m".to_string()]]);
    let want: Vec<f64> = vec![720.0, 0.16666666666666666, 0.2, 0.25, 0.3333333333333333, 0.5];
    assert_eq!(got.len(), want.len());
    for (g, w) in got.iter().zip(want.iter()) {
        if w.is_nan() {
            assert!(g.is_nan());
        } else {
            assert!((g - w).abs() < 1e-5, "got {} want {}", g, w);
        }
    }
}

#[test]
fn test_52() {
    let got = Solution::calc_equation(vec![vec!["x".to_string(), "y".to_string()], vec!["y".to_string(), "z".to_string()], vec!["z".to_string(), "w".to_string()], vec!["w".to_string(), "v".to_string()], vec!["v".to_string(), "u".to_string()], vec!["u".to_string(), "t".to_string()]], vec![2.0, 3.0, 4.0, 5.0, 6.0, 7.0], vec![vec!["x".to_string(), "t".to_string()], vec!["t".to_string(), "x".to_string()], vec!["y".to_string(), "u".to_string()], vec!["u".to_string(), "y".to_string()], vec!["z".to_string(), "v".to_string()], vec!["v".to_string(), "z".to_string()]]);
    let want: Vec<f64> = vec![5040.0, 0.0001984126984126984, 360.0, 0.002777777777777778, 20.0, 0.05];
    assert_eq!(got.len(), want.len());
    for (g, w) in got.iter().zip(want.iter()) {
        if w.is_nan() {
            assert!(g.is_nan());
        } else {
            assert!((g - w).abs() < 1e-5, "got {} want {}", g, w);
        }
    }
}

#[test]
fn test_53() {
    let got = Solution::calc_equation(vec![vec!["m".to_string(), "n".to_string()], vec!["n".to_string(), "o".to_string()], vec!["o".to_string(), "p".to_string()], vec!["p".to_string(), "q".to_string()], vec!["q".to_string(), "r".to_string()], vec!["r".to_string(), "s".to_string()]], vec![1.5, 2.5, 3.5, 4.5, 5.5, 6.5], vec![vec!["m".to_string(), "s".to_string()], vec!["s".to_string(), "m".to_string()], vec!["m".to_string(), "r".to_string()], vec!["r".to_string(), "m".to_string()], vec!["m".to_string(), "q".to_string()], vec!["q".to_string(), "m".to_string()], vec!["m".to_string(), "p".to_string()], vec!["p".to_string(), "m".to_string()], vec!["m".to_string(), "o".to_string()], vec!["o".to_string(), "m".to_string()], vec!["m".to_string(), "n".to_string()], vec!["n".to_string(), "m".to_string()], vec!["t".to_string(), "t".to_string()]]);
    let want: Vec<f64> = vec![2111.484375, 0.0004736004736004736, 324.84375, 0.0030784030784030783, 59.0625, 0.016931216931216932, 13.125, 0.0761904761904762, 3.75, 0.26666666666666666, 1.5, 0.6666666666666666, -1.0];
    assert_eq!(got.len(), want.len());
    for (g, w) in got.iter().zip(want.iter()) {
        if w.is_nan() {
            assert!(g.is_nan());
        } else {
            assert!((g - w).abs() < 1e-5, "got {} want {}", g, w);
        }
    }
}

#[test]
fn test_54() {
    let got = Solution::calc_equation(vec![vec!["s1".to_string(), "s2".to_string()], vec!["s2".to_string(), "s3".to_string()], vec!["s3".to_string(), "s4".to_string()], vec!["s4".to_string(), "s5".to_string()], vec!["s5".to_string(), "s1".to_string()], vec!["s1".to_string(), "s6".to_string()], vec!["s6".to_string(), "s2".to_string()]], vec![1.1, 1.2, 1.3, 1.4, 1.5, 2.0, 0.5], vec![vec!["s1".to_string(), "s5".to_string()], vec!["s5".to_string(), "s1".to_string()], vec!["s2".to_string(), "s4".to_string()], vec!["s3".to_string(), "s2".to_string()], vec!["s1".to_string(), "s7".to_string()]]);
    let want: Vec<f64> = vec![2.4024, 0.41625041625041626, 1.56, 0.8333333333333335, -1.0];
    assert_eq!(got.len(), want.len());
    for (g, w) in got.iter().zip(want.iter()) {
        if w.is_nan() {
            assert!(g.is_nan());
        } else {
            assert!((g - w).abs() < 1e-5, "got {} want {}", g, w);
        }
    }
}

#[test]
fn test_55() {
    let got = Solution::calc_equation(vec![vec!["x1".to_string(), "x2".to_string()], vec!["x2".to_string(), "x3".to_string()], vec!["x3".to_string(), "x4".to_string()], vec!["x4".to_string(), "x5".to_string()], vec!["x5".to_string(), "x6".to_string()], vec!["x6".to_string(), "x7".to_string()], vec!["x7".to_string(), "x1".to_string()]], vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0], vec![vec!["x1".to_string(), "x7".to_string()], vec!["x7".to_string(), "x1".to_string()], vec!["x2".to_string(), "x5".to_string()], vec!["x5".to_string(), "x2".to_string()], vec!["x3".to_string(), "x6".to_string()], vec!["x6".to_string(), "x3".to_string()]]);
    let want: Vec<f64> = vec![720.0, 0.001388888888888889, 24.0, 0.041666666666666664, 60.0, 0.016666666666666666];
    assert_eq!(got.len(), want.len());
    for (g, w) in got.iter().zip(want.iter()) {
        if w.is_nan() {
            assert!(g.is_nan());
        } else {
            assert!((g - w).abs() < 1e-5, "got {} want {}", g, w);
        }
    }
}

#[test]
fn test_56() {
    let got = Solution::calc_equation(vec![vec!["a".to_string(), "b".to_string()], vec!["b".to_string(), "c".to_string()], vec!["c".to_string(), "d".to_string()], vec!["d".to_string(), "e".to_string()]], vec![2.0, 3.0, 4.0, 5.0], vec![vec!["a".to_string(), "e".to_string()], vec!["e".to_string(), "a".to_string()], vec!["a".to_string(), "d".to_string()], vec!["d".to_string(), "a".to_string()], vec!["x".to_string(), "y".to_string()]]);
    let want: Vec<f64> = vec![120.0, 0.008333333333333333, 24.0, 0.041666666666666664, -1.0];
    assert_eq!(got.len(), want.len());
    for (g, w) in got.iter().zip(want.iter()) {
        if w.is_nan() {
            assert!(g.is_nan());
        } else {
            assert!((g - w).abs() < 1e-5, "got {} want {}", g, w);
        }
    }
}

#[test]
fn test_57() {
    let got = Solution::calc_equation(vec![vec!["node1".to_string(), "node2".to_string()], vec!["node2".to_string(), "node3".to_string()], vec!["node3".to_string(), "node4".to_string()], vec!["node4".to_string(), "node5".to_string()], vec!["node5".to_string(), "node6".to_string()], vec!["node6".to_string(), "node7".to_string()], vec!["node7".to_string(), "node8".to_string()], vec!["node8".to_string(), "node9".to_string()]], vec![1.1, 1.2, 1.3, 1.4, 1.5, 1.6, 1.7, 1.8], vec![vec!["node1".to_string(), "node9".to_string()], vec!["node9".to_string(), "node1".to_string()], vec!["node2".to_string(), "node8".to_string()], vec!["node8".to_string(), "node2".to_string()], vec!["node3".to_string(), "node7".to_string()], vec!["node7".to_string(), "node3".to_string()], vec!["node4".to_string(), "node6".to_string()], vec!["node6".to_string(), "node4".to_string()], vec!["node1".to_string(), "node10".to_string()]]);
    let want: Vec<f64> = vec![17.6432256, 0.0566789782476057, 8.91072, 0.11222437693025929, 4.368, 0.2289377289377289, 2.0999999999999996, 0.4761904761904762, -1.0];
    assert_eq!(got.len(), want.len());
    for (g, w) in got.iter().zip(want.iter()) {
        if w.is_nan() {
            assert!(g.is_nan());
        } else {
            assert!((g - w).abs() < 1e-5, "got {} want {}", g, w);
        }
    }
}
