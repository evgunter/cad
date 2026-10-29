//! Review rows for the spline × plane arm (PR 6026, review lane r2).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use super::search_kit::band;
use super::section_cert_rows::{IN, at, p, plane, v};
use super::*;
use geom::Surface;
use geom_core::spline::KnotVector;

fn read(f: &Surface<f64>, g: &Surface<f64>) -> Section<f64> {
    classify_with(
        f,
        g,
        Reach {
            centre: p(0.0, 0.0, 0.0),
            radius: 1.0,
        },
        Some(geom_brep::SsiLane::marched()),
        band(),
    )
}

fn level(h: f64) -> Surface<f64> {
    plane(p(0.5, 0.5, h), v(0.0, 0.0, 1.0))
}

/// The dome `z = −(x − ½)² − (y − ½)²` over `[0, 1]²`.
fn dome() -> Surface<f64> {
    let k = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
    let g = [-0.25, 0.25, -0.25];
    let control = (0..9)
        .map(|n| {
            let (i, j) = (n / 3, n % 3);
            #[allow(clippy::cast_precision_loss)]
            p(0.5 * i as f64, 0.5 * j as f64, g[i] + g[j])
        })
        .collect();
    Surface::Nurbs(std::sync::Arc::new(
        geom::NurbsSurface::new(k.clone(), k, control, vec![1.0; 9]).unwrap(),
    ))
}

/// A hill near `u = 0.2` (control columns 1, 2, rows 2–4 at height 1)
/// and the wall's `u = 1` column raised to 1, everything else 0: at
/// `z = ½` the plane meets it in one closed loop round the hill and one
/// open branch near the raised side, on disjoint supports.
fn hill_and_ramp() -> Surface<f64> {
    let k = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.2, 0.4, 0.6, 0.8, 1.0, 1.0, 1.0], 2).unwrap();
    let control = (0..49)
        .map(|n| {
            let (i, j) = (n / 7, n % 7);
            let hill = (1..=2).contains(&i) && (2..=4).contains(&j);
            let z = if hill || i == 6 { 1.0 } else { 0.0 };
            #[allow(clippy::cast_precision_loss)]
            p(i as f64 / 6.0, j as f64 / 6.0, z)
        })
        .collect();
    Surface::Nurbs(std::sync::Arc::new(
        geom::NurbsSurface::new(k.clone(), k, control, vec![1.0; 49]).unwrap(),
    ))
}

/// **A loop beside an open branch is not cleared by an event on the
/// pair.** The event may lie on the open branch, so W4 must not read the
/// loop as the lone component: the count includes the open branch. Red
/// against `single` counting only closed components (the loop clears
/// `LoneEvented`).
#[test]
fn a_loop_beside_an_open_branch_is_not_cleared_by_an_event() {
    let s = read(&level(0.5), &hill_and_ramp());
    let Section::Components { parts, single } = &s else {
        panic!("components: {s:?}")
    };
    let loops = parts.iter().filter(|c| c.reaches.is_none()).count();
    let open = parts.iter().filter(|c| c.reaches == Some(Side::G)).count();
    assert_eq!((loops, open, *single), (1, 1, false), "{s:?}");
    assert_eq!(
        certify(&s, true, |_| Sheet::Not, at(IN, IN)),
        Err(Refusal::Undecided),
        "an evented pair with a loop beside an open branch"
    );
}

/// **A spline witness stands within its residual of the section.** The
/// dome cut at `z = h` meets the plane in the circle of radius `√−h`
/// about `(½, ½)`, so the witness's distance from the section is known
/// in closed form; the residual the placement is widened by must bound
/// it. Red against a residual that under-states the reach.
#[test]
fn a_spline_witness_stands_within_its_residual_of_the_section() {
    for h in [-0.1, -0.2, -0.03] {
        let (wall, cut) = (dome(), level(h));
        let s = read(&cut, &wall);
        let Section::Components { parts, .. } = &s else {
            panic!("components: {s:?}")
        };
        assert_eq!(parts.len(), 1, "{s:?}");
        let c = parts[0];
        let w = c.witness.expect("a witness");
        let r = ((w.x - 0.5).powi(2) + (w.y - 0.5).powi(2)).sqrt();
        let d = ((r - (-h).sqrt()).powi(2) + (w.z - h).powi(2)).sqrt();
        let built = match geom_brep::pair_section_with(
            &cut,
            &wall,
            Some(geom_brep::SsiLane::marched()),
            band(),
        ) {
            Ok(geom_brep::SectionRead::Built(b)) => b,
            other => panic!("{other:?}"),
        };
        if let geom_brep::ClassCarrier::Fitted(f) = &built.components()[0].carrier {
            eprintln!(
                "h {h}: witness distance {d:e}, residual {:e}, hull_sup {:e}, tube {:?}, band {:?}",
                c.residual,
                f.certificate.hull_sup,
                f.certificate.tube,
                band()
            );
        }
        assert!(
            d <= c.residual,
            "h {h}: witness {d:e} off, residual {:e}",
            c.residual
        );
    }
}
