//! Review r2 probe for PR 5907: prints nurbs_pair_limbs on rims that are boundary rows (exact images), interior rows of both charts (fitted only), and mixed. Interior rims also refuse TubeNotOneArc Undecided at 1e-6 (m>=64) and 1e-9 (m=512). PROBE_M=256,512 selects ring sub-arc counts.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
use crate::shared::tol::band;
use geom::{Curve3, NurbsSurface};
use geom_brep::edge_nurbs::sub_arc_joint;
use geom_brep::nurbs_pair_limbs;
use geom_core::spline::KnotVector;
use geom_core::{Point3, Vec3};

fn ring(m: usize, turn: f64, profile: [(f64, f64); 2]) -> NurbsSurface<f64> {
    let h = turn / m as f64;
    let mut ku = vec![0.0; 3];
    for k in 1..m { ku.extend([sub_arc_joint(k, m); 2]); }
    ku.extend([1.0; 3]);
    let (mut control, mut weights) = (Vec::new(), Vec::new());
    for j in 0..=2 * m {
        let theta = j as f64 * h / 2.0;
        let (reach, w) = if j % 2 == 1 { (1.0 / (h / 2.0).cos(), (h / 2.0).cos()) } else { (1.0, 1.0) };
        for (r, z) in profile {
            let r = r * reach;
            control.push(Point3::new(r * theta.cos(), r * theta.sin(), z));
            weights.push(w);
        }
    }
    NurbsSurface::new(KnotVector::clamped(ku, 2).unwrap(),
        KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap(), control, weights).unwrap()
}
fn rim(r: f64) -> Curve3<f64> {
    Curve3::Circle { center: Point3::new(0.0, 0.0, 0.0), axis: Vec3::new(0.0, 0.0, 1.0), radius: r, u_ref: Vec3::new(1.0, 0.0, 0.0) }
}
fn show<E: core::fmt::Debug, O>(what: &str, r: &Result<O, E>) {
    match r { Ok(_) => eprintln!("PROBE {what}: OK"), Err(e) => { let s = format!("{e:?}"); eprintln!("PROBE {what}: {}", &s[..s.len().min(220)]); } }
}
#[test]
fn probe_rows() {
    let t = 1.2;
    let c = rim(1.0);
    let cyl = |m| ring(m, t, [(1.0, 0.0), (1.0, 1.0)]);
    let ann = |m| ring(m, t, [(1.0, 0.0), (2.0, 0.0)]);
    // rim as an INTERIOR row of a cylinder z in [-1, 1] and annulus r in [0.5, 2].
    let cyl_mid = |m| ring(m, t, [(1.0, -1.0), (1.0, 1.0)]);
    let ann_mid = |m| ring(m, t, [(0.5, 0.0), (2.0, 0.0)]);
    show("both exact cyl×ann", &nurbs_pair_limbs(&c, (0.0, t), &cyl(1), &ann(1), 1.0, band()));
    for m in std::env::var("PROBE_M").map(|v| v.split(',').map(|x| x.parse().unwrap()).collect::<Vec<usize>>()).unwrap_or(vec![1usize, 4, 16, 64]) {
        show(&format!("both interior m={m}"), &nurbs_pair_limbs(&c, (0.0, t), &cyl_mid(m), &ann_mid(m), 1.0, band()));
        show(&format!("mixed ann exact × cyl_mid m={m}"), &nurbs_pair_limbs(&c, (0.0, t), &ann(1), &cyl_mid(m), 1.0, band()));
        show(&format!("mixed cyl exact × ann_mid m={m}"), &nurbs_pair_limbs(&c, (0.0, t), &cyl(1), &ann_mid(m), 1.0, band()));
    }
    // half-edge (partial row) on both exact
    show("partial row both exact", &nurbs_pair_limbs(&c, (0.3, 0.9), &cyl(1), &ann(1), 1.0, band()));
}
