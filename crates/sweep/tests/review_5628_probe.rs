//! Review probe (PR 5628, R1): does the check-7 certificate's continuation
//! still equal the measurement door, bit for bit, on a body whose sign is
//! decided only after round 0?
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
use crate::common::{quad_rounds, stacked, strip_section};
use geom_core::Tol;
use sweep::loft_body;

fn strip(s: f64, delta: f64) -> Option<topo::Body<f64>> {
    loft_body::<f64>(
        &[strip_section(s, delta, false), strip_section(s, delta, false)],
        &stacked(&[0.0, 1.0], s),
        1,
        Tol::witness(),
    )
    .ok()
    .map(|l| l.body)
}

#[test]
fn review_5628_continuation_vs_measurement() {
    let tol = Tol::witness();
    let eps = tol.get().eps;
    let mut differ = 0;
    for k in [1.0e6, 1.0e7, 1.0e8, 1.0e9] {
        for ratio in [1e-1, 1e-2, 1e-3, 1e-4, 1e-5, 1e-6] {
            let s = k * eps;
            let Some(body) = strip(s, ratio * s) else {
                eprintln!("PROBE k {k:e} ratio {ratio:e}: no body");
                continue;
            };
            let mut gated = None;
            let gate = quad_rounds(|| gated = Some(topo::validate_geometric_certificate(&body, tol)));
            let gated = match gated.unwrap() {
                Ok(g) => g,
                Err(e) => {
                    eprintln!("PROBE k {k:e} ratio {ratio:e}: gate refuses {e:?}");
                    continue;
                }
            };
            let enc = gated.enclosure();
            let mut cont = None;
            let refine = quad_rounds(|| cont = Some(gated.refine_to_target()));
            let mut meas = None;
            let one = quad_rounds(|| meas = Some(topo::test_support::unmigrated_measure(&body, tol)));
            let (c, m) = (cont.unwrap(), meas.unwrap());
            let same = match (&c, &m) {
                (Ok(c), Ok(m)) => {
                    (c.volume.to_bits(), c.volume_pad.to_bits(), c.surface_area.to_bits(), c.area_pad.to_bits())
                        == (m.volume.to_bits(), m.volume_pad.to_bits(), m.surface_area.to_bits(), m.area_pad.to_bits())
                }
                (Err(a), Err(b)) => format!("{a:?}") == format!("{b:?}"),
                _ => false,
            };
            if !same {
                differ += 1;
            }
            eprintln!(
                "PROBE k {k:e} ratio {ratio:e}: gate {gate} refine {refine} one {one} enc [{:e},{:e}] same {same}\n   cont {c:?}\n   meas {m:?}",
                enc.volume_lo, enc.volume_hi
            );
        }
    }
    assert_eq!(differ, 0, "continuation differs from the measurement on {differ} bodies");
}

#[test]
fn review_5628_cost() {
    let tol = Tol::witness();
    let eps = tol.get().eps;
    for (k, ratio) in [(1.0e11, 1e-10), (1.0e11, 1e-11), (1.0e12, 1e-10), (1.0e12, 1e-11)] {
        let s = k * eps;
        for reversed in [false] {
            let Some(body) = loft_body::<f64>(
                &[strip_section(s, ratio * s, reversed), strip_section(s, ratio * s, reversed)],
                &stacked(&[0.0, 1.0], s),
                1,
                tol,
            )
            .ok()
            .map(|l| l.body) else {
                eprintln!("COST k {k:e} ratio {ratio:e} rev {reversed}: no body");
                continue;
            };
            let t = std::time::Instant::now();
            let got = topo::validate_geometric(&body, tol);
            eprintln!(
                "COST k {k:e} ratio {ratio:e} rev {reversed}: validate_geometric {:.2} s -> {:?}",
                t.elapsed().as_secs_f64(),
                got.map_err(|e| e.into_iter().map(|e| format!("{e:?}").chars().take(160).collect::<String>()).collect::<Vec<_>>())
            );
        }
    }
}
