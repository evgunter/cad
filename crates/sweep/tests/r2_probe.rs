//! R2 review probe (PR 5628): does check 7's certificate, continued to
//! the reporting resolution, equal the number door bit for bit?
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
use geom_core::{Point2, Tol};
use profile::test_support::bulge_loop;
use sweep::loft_body;

fn section(s: f64, d: f64, end_bulge: f64) -> sweep::Section {
    let b = 0.414_213_562_373_095_1;
    let v = |x: f64, y: f64, bulge: f64| (Point2::new(x, y), bulge);
    vec![bulge_loop(vec![
        v(-s, 0.0, b),
        v(s, 0.0, end_bulge),
        v(s, d, -b),
        v(-s, d, 0.0),
    ])]
}

#[test]
fn r2_certificate_continuation_vs_number_door() {
    let tol = Tol::witness();
    let eps = tol.get().eps;
    let mut differ = 0;
    for k in [1.0e9, 1.0e10, 1.0e11] {
        for ratio in [1e-2, 1e-4, 1e-6, 1e-8] {
            for end_bulge in [0.0, 0.9] {
                let s = k * eps;
                let d = ratio * s;
                let sec = section(s, d, end_bulge);
                let planes = sweep::test_support::stacked_at(&[0.0, s]);
                let Ok(l) = loft_body::<f64>(&[sec.clone(), sec], &planes, 1, tol) else {
                    eprintln!("R2 k={k:e} ratio={ratio:e} eb={end_bulge}: loft refused");
                    continue;
                };
                let body = l.body;
                let t = std::time::Instant::now();
                let gated = topo::validate_geometric_certificate(&body, tol);
                let tg = t.elapsed();
                let Ok(gated) = gated else {
                    eprintln!("R2 k={k:e} ratio={ratio:e} eb={end_bulge}: gate refused ({tg:?}): {:?}", gated.err().map(|e| e.len()));
                    continue;
                };
                let enc = gated.enclosure();
                let cont = gated.refine_to_target();
                let meas = topo::test_support::unmigrated_measure(&body, tol);
                let bits = |m: &topo::MassProperties<f64>| (m.volume.to_bits(), m.volume_pad.to_bits(), m.area_pad.to_bits());
                let same = match (&cont, &meas) {
                    (Ok(c), Ok(m)) => Some(bits(c) == bits(m)),
                    _ => None,
                };
                if same == Some(false) { differ += 1; }
                eprintln!(
                    "R2 k={k:e} ratio={ratio:e} eb={end_bulge}: gate {tg:?} enc=[{:e},{:e}] cont={:?} meas={:?} same={same:?}",
                    enc.volume_lo, enc.volume_hi,
                    cont.as_ref().map(|c| (c.volume, c.volume_pad)).map_err(|e| format!("{e}")),
                    meas.as_ref().map(|m| (m.volume, m.volume_pad)).map_err(|e| format!("{e}")),
                );
            }
        }
    }
    eprintln!("R2 DIFFER {differ}");
}

/// **R2 red row (PR 5628).** A thin arc strip whose small end face is a
/// quadrature arc: check 7's sign settles past round 0 on main and here.
/// On main the gate's certificate, continued to the reporting
/// resolution, IS `mass_properties` bit for bit (the AGREEMENT claim of
/// `sign_walk_plus_v`); at ce2f1f4 the set walk has refined faces that
/// had already met `1024·ε`, so the continuation keeps a narrower,
/// different enclosure. Green on debb2a6, red at ce2f1f4.
#[test]
fn r2_a_settled_sign_continues_to_the_number_doors_bits() {
    let tol = Tol::witness();
    let eps = tol.get().eps;
    let s = 1.0e9 * eps;
    let sec = section(s, 1e-4 * s, 0.9);
    let planes = sweep::test_support::stacked_at(&[0.0, s]);
    let body = loft_body::<f64>(&[sec.clone(), sec], &planes, 1, tol)
        .expect("lofts")
        .body;
    let gated = topo::validate_geometric_certificate(&body, tol).expect("tier 3 admits it");
    let c = gated.refine_to_target().expect("the number");
    let m = topo::test_support::unmigrated_measure(&body, tol).expect("the number");
    assert_eq!(
        (c.volume.to_bits(), c.volume_pad.to_bits(), c.area_pad.to_bits()),
        (m.volume.to_bits(), m.volume_pad.to_bits(), m.area_pad.to_bits()),
        "continuation {c:?} vs number door {m:?}"
    );
}
