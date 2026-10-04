//! Delta-2 reviewer probe (reach-delta2-3977): `validate_geometric`'s
//! cost on head and main, release, median of 200 runs per body.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
use geom_core::{Point2, Tol};
use sweep::test_support::{bored_block_of_arcs, brick, cylinder_of_arcs_at, revolved_about_y};
use topo::validate_geometric;

#[test]
#[ignore = "timing probe: cargo nextest run --release --run-ignored only -E 'test(d2_cost)'"]
fn d2_cost() {
    let tol = Tol::witness();
    let bodies = vec![
        ("brick", brick::<f64>((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), tol)),
        ("bored block 8 arcs", bored_block_of_arcs(8, 4.0, 1.0, 1.0, tol)),
        ("disc 4 arcs 5 km", cylinder_of_arcs_at(4, 1e-3, Point2::new(3e3, 2.4e3), 3.2e3, 1e-6, tol)),
        (
            "ball",
            revolved_about_y(
                vec![(Point2::new(0.0, -1.0), 1.0), (Point2::new(0.0, 1.0), 0.0)],
                sweep::Revolution::Full,
                tol,
            ),
        ),
    ];
    for (name, b) in &bodies {
        assert!(validate_geometric(b, tol).is_ok(), "{name} valid");
        let mut t: Vec<f64> = (0..200)
            .map(|_| {
                let s = std::time::Instant::now();
                let _ = std::hint::black_box(validate_geometric(b, tol));
                s.elapsed().as_secs_f64() * 1e3
            })
            .collect();
        t.sort_by(f64::total_cmp);
        println!("D2COST|{name}|median_ms={:.3}|p10={:.3}|p90={:.3}", t[100], t[20], t[180]);
    }
}
