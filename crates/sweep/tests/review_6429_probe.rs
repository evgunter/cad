//! Review 6429 evidence: the lofted cylinder's far union, read face by
//! face against the bare operand; the operand at a tighter resolution;
//! and displaced-cap mutants held to PR 6429's band.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, clippy::print_stderr)]

use geom_core::Tol;
use topo::{AtRestBody, BooleanResult, Settle};

use sweep::test_support::{finished, lofted_cylinder};
use topo::test_support::{brick_with_fresh_keys, mass_properties_with_face_terms, pinned_volume};

fn far_union(cyl: &AtRestBody<f64>, tol: Tol) -> AtRestBody<f64> {
    let brick = finished(
        "brick",
        brick_with_fresh_keys::<f64>((5.0, 6.0), (5.0, 6.0), (5.0, 6.0), tol),
        tol,
    );
    match topo::union(cyl, &brick, tol).expect("far union builds") {
        BooleanResult::Body(b) => b.body,
        BooleanResult::Empty => panic!("empty"),
    }
}

#[test]
fn review_6429_far_union_reads_its_operand() {
    let tol = Tol::witness();
    let eps = tol.eps();
    let cyl = finished("cyl", lofted_cylinder(2.0, tol), tol);
    let uni = far_union(&cyl, tol);
    let pin = topo::test_support::pinned_resolution::<f64>(tol);
    let (_, ct) = mass_properties_with_face_terms(&cyl, tol, pin).unwrap();
    let (_, ut) = mass_properties_with_face_terms(&uni, tol, pin).unwrap();
    eprintln!("eps {eps:e}");
    for (f, flux, pad) in &ct {
        let u = ut.iter().find(|(g, _, _)| g == f);
        eprintln!("operand {f:?}: flux/3 {:.15} flux_pad {pad:.3e}; union same key: {:?}", flux / 3.0, u.map(|(_, x, p)| (x / 3.0, *p)));
        if let Some((_, x, p)) = u {
            assert_eq!((x.to_bits(), p.to_bits()), (flux.to_bits(), pad.to_bits()), "union face {f:?} reads differently");
        }
    }
    let c = pinned_volume("cyl", &cyl, tol);
    let u = pinned_volume("uni", &uni, tol);
    let tau = 2.0 * core::f64::consts::PI;
    eprintln!("operand V {:.15} ± {:.3e} (off 2π by {:.3e}); union V {:.15} ± {:.3e}; union − operand − 1 = {:.3e}",
        c.volume, c.volume_pad, c.volume - tau, u.volume, u.volume_pad, u.volume - c.volume - 1.0);
    // Tighter resolutions: does the midpoint converge on 2π (a reading
    // excess), or stay put (a geometric one)?
    for k in [1024.0, 64.0, 8.0, 1.0] {
        let len = quantity::Length::from_meters(k * eps);
        match topo::mass_properties(&cyl, tol, Settle::Resolution(len)) {
            Ok(m) => match m.met() {
                Ok(e) => eprintln!("  res {k}·ε: V {:.15} ± {:.3e}, off 2π {:.3e}", e.volume, e.volume_pad, e.volume - tau),
                Err(unmet) => eprintln!("  res {k}·ε: unmet: {unmet}"),
            },
            Err(r) => eprintln!("  res {k}·ε: refused {r}"),
        }
    }
}

/// A far union whose cylinder is `2 + δ` tall (a cap displaced by δ),
/// held to PR 6429's band against the closed form `2π + 1`.
#[test]
fn review_6429_displaced_cap_mutants() {
    let tol = Tol::witness();
    let eps = tol.eps();
    let want = 2.0 * core::f64::consts::PI + 1.0;
    for delta in [1e-7, 1e-6, 1e-5, 1e-4, 2e-4, 5e-4, 1e-3, 3e-3] {
        let cyl = finished("cyl", lofted_cylinder(2.0 + delta, tol), tol);
        let uni = far_union(&cyl, tol);
        let m = pinned_volume("uni", &uni, tol);
        let old = (m.volume - want).abs() <= 1e-6 * want;
        let new = (m.volume - want).abs() <= 1e-6 * want + m.volume_pad;
        eprintln!(
            "eps {eps:e} δ {delta:e} (= {:.0}ε): V {:.12} ± {:.3e}, off {:.3e}; old band passes: {old}; PR band passes: {new}",
            delta / eps, m.volume, m.volume_pad, m.volume - want
        );
    }
}
