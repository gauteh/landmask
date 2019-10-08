#[cfg(test)]
use super::*;


#[test]
fn generate_rtree() {
    let _tree = make_rtree_wkb("coarse.wkb").unwrap();
}

#[test]
fn ocean_is_false() {
    let tree = make_rtree_wkb("coarse.wkb").unwrap();
    assert_eq! (contains(&tree, 5., 65.), false);
}

#[test]
fn land_is_true() {
    let tree = make_rtree_wkb("coarse.wkb").unwrap();
    assert_eq! (contains(&tree, 15., 65.), true);
}

#[cfg(test)]
mod bench {
    use super::*;
    use test::Bencher;

    #[bench]
    fn bench_check_land_coarse(b: &mut Bencher) {
        let tree = make_rtree_wkb("coarse.wkb").unwrap();
        b.iter(|| contains(&tree, 15., 65.));
    }

    #[bench]
    fn bench_check_many(b: &mut Bencher) {
        let tree = make_rtree_wkb("coarse.wkb").unwrap();

        let x: Vec<_> = (-180..180).map(f64::from).collect();
        let y: Vec<_> = (-90..90).map(f64::from).collect();

        let mut xx: Vec<f64> = vec![];
        let mut yy: Vec<f64> = vec![];

        for xp in &x {
            for yp in &y {
                xx.push (*xp);
                yy.push (*yp);
            }
        }

        b.iter (|| contains_many(&tree, xx.as_slice(), yy.as_slice()));
    }
}

