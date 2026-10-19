//! **A curved keel resting on the floor subtracts as the slit form.** A
//! 270° partial revolve about `y` of the slab `0.5..3 × 1..2`, less a
//! 150° partial revolve of a V whose tip lies on its floor `y = 1`,
//! turned about `y`. The keel is a circular arc of radius 2 lying in the
//! floor, and the material about it is two wedges, floor | wall a and
//! wall b | floor (`crates/topo/README.md`, tier 3). Turned 60°, the V
//! lies inside the slab's 270° and the floor carries the keel as a
//! zero-area ring of two distinct arcs; turned −20°, the keel leaves
//! through the slab's end face and the floor carries it as a slit of its
//! outline, the open end one vertex per wedge.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::PI;
use geom_core::{Affine3, Point2, Point3, Tol, Vec3};
use sweep::Revolution;
use sweep::test_support::{finished, revolved_about_y};
use topo::{BooleanResult, LoopBoundary, mass_properties, subtract, transform_rigid};

fn tol() -> Tol {
    Tol::witness()
}

/// **Each turn subtracts as its slit form**: tiers 1, 2 and 3 with no
/// records, the closed-form volume, and the keel as two distinct arcs
/// between the same two points, each between the floor and its own wall,
/// a ring of the floor or a slit of its outline. Tier 3′ reads the two
/// arcs, on two curve entries, as an overlap no record backs, as it does
/// the split's curved ring.
#[test]
fn a_curved_keel_on_the_floor_subtracts_as_its_slit_form() {
    let loop_of = |pts: &[(f64, f64)]| pts.iter().map(|&(x, y)| (Point2::new(x, y), 0.0)).collect();
    let slab = finished(
        "the slab",
        revolved_about_y(
            loop_of(&[(0.5, 1.0), (3.0, 1.0), (3.0, 2.0), (0.5, 2.0)]),
            Revolution::Partial(1.5 * PI),
            tol(),
        ),
        tol(),
    );
    let sector = 0.75 * PI * (3.0f64.powi(2) - 0.5f64.powi(2));
    // The V's section in the slab: half-width `0.2 (y − 1)` up to `y = 2`,
    // area 0.2, about its centroid's radius 2.
    for (turn, inside, ring) in [(60.0, 150.0f64, true), (-20.0, 130.0, false)] {
        let what = format!("the V turned {turn}°");
        let v = revolved_about_y(
            loop_of(&[(1.7, 2.5), (2.0, 1.0), (2.3, 2.5)]),
            Revolution::Partial(150f64.to_radians()),
            tol(),
        );
        let turned =
            Affine3::rotation_about_axis(Point3::origin(), Vec3::unit_y(), f64::to_radians(turn));
        let v =
            transform_rigid(&v, &geom_core::test_support::rigid(&turned, tol()), tol()).unwrap();
        let Ok(BooleanResult::Body(r)) = subtract(&slab, &finished("the V", v, tol()), tol())
        else {
            panic!("{what}: the slab less the V");
        };
        let b = &r.body;
        assert_eq!(topo::validate(b), Ok(()), "{what}: tier 1");
        assert_eq!(topo::validate_closed(b), Ok(()), "{what}: tier 2");
        assert_eq!(topo::validate_geometric(b, tol()), Ok(()), "{what}: tier 3");
        assert!(r.contacts.rows().is_empty(), "{what}: records");
        let volume = sector - 0.2 * 2.0 * inside.to_radians();
        let got = mass_properties(b, tol()).unwrap().volume;
        assert!(
            (got - volume).abs() < 1e-9 * volume,
            "{what}: volume {got}, closed form {volume}"
        );
        let on_keel =
            |p: &Point3<f64>| (p.y - 1.0).abs() < 1e-9 && (p.x.hypot(p.z) - 2.0).abs() < 1e-9;
        let ends = |e: topo::EdgeKey| {
            let he = b.get_edge(e).unwrap().he_plus;
            let mut v = [
                b.get_half_edge(he).unwrap().start,
                b.half_edge_end(he).unwrap(),
            ]
            .map(|v| b.get_vertex(v).unwrap().point);
            v.sort();
            v
        };
        let keel: Vec<topo::EdgeKey> = b
            .edges()
            .map(|(e, _)| e)
            .filter(|&e| {
                let mid = match b.get_curve_geom(b.get_edge(e).unwrap().curve) {
                    Some(topo::CurveGeom::Certified(c)) => c.mid_point(),
                    _ => return false,
                };
                on_keel(&mid) && ends(e).iter().all(|&k| on_keel(b.get_point(k).unwrap()))
            })
            .collect();
        let [ka, kb] = keel[..] else {
            panic!("{what}: the keel is {keel:?}, not two arcs");
        };
        assert_eq!(
            ends(ka),
            ends(kb),
            "{what}: the keel's arcs end on different points"
        );
        let edges_of = |l| match b.get_loop(l).unwrap().boundary {
            LoopBoundary::Cycle { first } => b
                .loop_cycle(first)
                .unwrap()
                .into_iter()
                .map(|he| b.get_half_edge(he).unwrap().edge)
                .collect::<Vec<_>>(),
            LoopBoundary::Empty { .. } => Vec::new(),
        };
        let holding: Vec<_> = b
            .faces()
            .filter(|(_, f)| {
                core::iter::once(f.outer)
                    .chain(f.rings.iter().copied())
                    .any(|l| {
                        let es = edges_of(l);
                        es.contains(&ka) && es.contains(&kb)
                    })
            })
            .collect();
        let [(_, floor)] = holding[..] else {
            panic!(
                "{what}: {} faces hold both arcs, not the floor alone",
                holding.len()
            );
        };
        let rings: Vec<_> = floor.rings.iter().map(|&l| edges_of(l)).collect();
        if ring {
            assert!(
                rings
                    .iter()
                    .any(|r| r.len() == 2 && r.contains(&ka) && r.contains(&kb)),
                "{what}: the floor's rings {rings:?}, no ring of the keel's arcs"
            );
        } else {
            assert!(
                rings.is_empty() && edges_of(floor.outer).contains(&ka),
                "{what}: the floor's rings {rings:?}, not a slit of its outline"
            );
        }
        // The census reads the two arcs, on two curve entries, as an
        // overlap no record backs
        // (`work/cleave/a-curved-slit-fails-tier-3-prime-on-its-two-coincident-arcs.md`).
        match topo::validate_pseudomanifold(b, &r.contacts, tol())
            .as_ref()
            .map_err(Vec::as_slice)
        {
            Err(
                [
                    topo::ValidationError::UnrecordedContact {
                        contact: topo::CensusContact::EdgeEdgeOverlap { a, b: o },
                        ..
                    },
                ],
            ) if [*a, *o] == [ka, kb] || [*a, *o] == [kb, ka] => {}
            other => panic!("{what}: tier 3′ {other:?}, not the keel's unrecorded overlap"),
        }
    }
}
