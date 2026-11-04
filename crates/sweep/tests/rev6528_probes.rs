//! Review probes for PR 6528: the refined interior arc's degree-1 loft
//! at the other scalars.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Interval, Point2, Sym, Tol};
use profile::test_support::bulge_loop;
use sweep::test_support::{lift_places, lift_sections, stacked_at};
use sweep::{Section, loft_body};

fn sections(bulges: &[f64]) -> Vec<Section> {
    let corners = [(-1.0, -1.0), (1.0, -1.0), (1.0, 1.0), (-1.0, 1.0)];
    bulges
        .iter()
        .map(|&b| {
            let chain = (0..4)
                .map(|i| {
                    let (x, y) = corners[i];
                    (Point2::new(x, y), if i == 1 { b } else { 0.0 })
                })
                .collect();
            vec![bulge_loop(chain)]
        })
        .collect()
}

#[test]
fn probe_refined_loft_at_each_scalar() {
    let tol = Tol::witness();
    for bulges in [&[1.0, 0.2, 1.0][..], &[1.0, 0.2, 0.0, 0.7, 1.0][..]] {
        let s = sections(bulges);
        #[allow(clippy::cast_precision_loss)]
        let zs: Vec<f64> = (0..bulges.len()).map(|i| i as f64).collect();
        let places = stacked_at(&zs);
        let f = loft_body::<f64>(&s, &places, 1, tol).map(|_| ());
        eprintln!("{bulges:?} f64: {:?}", f.map_err(|e| e.render(geom_brep::recourse::Reading::Build)));
        let i = loft_body::<Interval>(
            &lift_sections::<Interval>(&s),
            &lift_places::<Interval>(&places, tol),
            1,
            tol,
        )
        .map(|_| ());
        eprintln!("{bulges:?} Interval: {:?}", i.map_err(|e| e.render(geom_brep::recourse::Reading::Build)));
        let budget = geom_core::SymBudget { max_terms: 4096, max_degree: 128 };
        let y = geom_core::sym::with_session(budget, || {
            loft_body::<Sym<f64>>(
                &lift_sections::<Sym<f64>>(&s),
                &lift_places::<Sym<f64>>(&places, tol),
                1,
                tol,
            )
            .map(|_| ())
            .map_err(|e| e.render(geom_brep::recourse::Reading::Build))
        });
        eprintln!("{bulges:?} Sym: {:?}", y);
    }
}

/// Does the second refined case mesh (the PR body says it gives
/// 18.794023; the row's comment says it is not meshed)?
#[test]
fn probe_second_case_meshes() {
    let tol = Tol::witness();
    for bulges in [&[1.0, 0.2, 0.0, 0.7, 1.0][..], &[0.0, 1.0, 0.3, 0.0][..]] {
        let s = sections(bulges);
        #[allow(clippy::cast_precision_loss)]
        let zs: Vec<f64> = (0..bulges.len()).map(|i| i as f64).collect();
        let lofted = loft_body::<f64>(&s, &stacked_at(&zs), 1, tol).unwrap();
        let fin = sweep::test_support::finished("p", topo::Body::clone(&lofted.body), tol);
        let mp = topo::mass_properties(&fin, tol, topo::test_support::pinned_resolution(tol)).unwrap();
        for d in [1e-3, 2e-3, 5e-3] {
            let m = mesh::tessellate(&lofted.body, d, tol);
            eprintln!(
                "{bulges:?} vol {} ± {} MESH δ={d}: {:?}",
                mp.enclosure.volume, mp.enclosure.volume_pad,
                m.map(|m| mesh::validate::signed_volume(&m)).map_err(|e| format!("{e:?}").chars().take(60).collect::<String>())
            );
        }
    }
}
