//! Delta reviewer probe (reach-delta-3977): the far thin disc through
//! the other sign readers — `classify_shells` and `point_in_solid`'s
//! side at infinity — and its inside-out twin. Same fixture as
//! `probe_d_disc`; run on head and main.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
use geom_core::{Band, Point2, Point3, Tol};
use sweep::test_support::cylinder_of_arcs_at;
use topo::{classify_shells, point_in_solid};

#[test]
fn dprobe_disc_other_readers() {
    let tol = Tol::witness();
    let band = Band::linear(tol).unwrap();
    println!("DPROBE3|eps={:e}", tol.eps());
    for &(r, h) in &[(1e-3, 1e-7), (1e-3, 1e-6), (1e-2, 1e-6)] {
        if h < 15.0 * tol.eps() { continue; }
        for &d in &[0.0, 1e3, 5e3] {
            let (cx, cy, z0) = (d * 0.6, d * 0.48, d * 0.64);
            let Ok(b) = std::panic::catch_unwind(|| cylinder_of_arcs_at(4, r, Point2::new(cx, cy), z0, h, tol)) else { continue };
            let far = Point3::new(cx + 10.0 * r, cy, z0 + h / 2.0);
            let inside = Point3::new(cx, cy, z0 + h / 2.0);
            for (tag, body) in [("upright", b.clone()), ("INVERTED", b.revert().unwrap())] {
                let cls = classify_shells(&body, tol).map(|v| v.iter().map(|c| format!("{:?}", c.role)).collect::<Vec<_>>());
                let cls: String = format!("{cls:?}").chars().take(90).collect();
                let pf: String = format!("{:?}", point_in_solid(&body, far, band, tol)).chars().take(90).collect();
                let pi: String = format!("{:?}", point_in_solid(&body, inside, band, tol)).chars().take(90).collect();
                println!("DPROBE3|r={r:e} h={h:e} d={d:e}|{tag}|classify={cls}|pis(beside)={pf}|pis(inside)={pi}");
            }
        }
    }
}
