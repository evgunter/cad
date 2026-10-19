//! **A curved keel lying in the plane splits sound and sections with no
//! spur.** A 270° partial revolve of the annulus `0.5..3 × 0..2` about
//! `y`, less a 150° partial revolve of a V whose tip lies at `y = 1` (to
//! within `0.9 ε`), turned about `y`. The keel is a circular arc of
//! radius 2 in the plane `y = 1`. Turned so that one end of the keel
//! opens through the annulus's end face, the material above the plane
//! runs round its closed end; turned so that both ends close inside the
//! annulus, it runs round both. Split by that plane either way up, each
//! side keeps its volume and passes tier 3; `plane_section` reads one
//! region, the 270° annular sector, which the keel's arc leaves whole.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common::bores::halves_at_rest;
use core::f64::consts::PI;
use geom_core::{Affine3, Point2, Point3, Tol, Vec3};
use sweep::Revolution;
use sweep::test_support::{finished, revolved_about_y};
use topo::splitting::plane_section;
use topo::{Body, BooleanResult, mass_properties, subtract, transform_rigid};

fn tol() -> Tol {
    Tol::witness()
}

/// The annulus less the V, its tip `s·ε` above `y = 1`, the V turned
/// `turn` about `y`.
fn keeled(s: f64, turn: f64) -> Body<f64> {
    let loop_of = |pts: &[(f64, f64)]| pts.iter().map(|&(x, y)| (Point2::new(x, y), 0.0)).collect();
    let ring = revolved_about_y(
        loop_of(&[(0.5, 0.0), (3.0, 0.0), (3.0, 2.0), (0.5, 2.0)]),
        Revolution::Partial(1.5 * PI),
        tol(),
    );
    let tip = 1.0 + s * tol().eps();
    let v = revolved_about_y(
        loop_of(&[(1.7, 2.5), (2.0, tip), (2.3, 2.5)]),
        Revolution::Partial(150f64.to_radians()),
        tol(),
    );
    let turn = Affine3::rotation_about_axis(Point3::origin(), Vec3::unit_y(), turn.to_radians());
    let v = transform_rigid(&v, &geom_core::test_support::rigid(&turn, tol()), tol()).unwrap();
    match subtract(
        &finished("the annulus", ring, tol()),
        &finished("the V", v, tol()),
        tol(),
    ) {
        Ok(BooleanResult::Body(b)) => b.body.into_body(),
        other => panic!("the annulus less the V: {other:?}"),
    }
}

#[test]
fn a_curved_keel_in_the_plane_splits_sound_and_sections_with_no_spur() {
    let eps = tol().eps();
    // The 270° sector of the annulus between radii 0.5 and 3.
    let section = 0.75 * PI * (3.0f64.powi(2) - 0.5f64.powi(2));
    for turn in [-20.0, 160.0, -160.0] {
        for s in [0.0, 0.6, -0.6, 0.9, -0.9] {
            let what = format!("the V turned {turn}°, its tip {s} ε off the plane");
            let body = keeled(s, turn);
            let whole = mass_properties(&body, tol()).unwrap().volume;
            for up in [1.0, -1.0] {
                let what = format!("{what}, the plane's normal {up} y");
                let plane = topo::test_support::split_plane(
                    Point3::new(0.0, 1.0, 0.0),
                    Vec3::new(0.0, up, 0.0),
                    tol(),
                );
                let halves = halves_at_rest(&what, &body, &plane);
                let sum: f64 = halves
                    .iter()
                    .map(|h| mass_properties(h, tol()).unwrap().volume)
                    .sum();
                assert!(
                    (sum - whole).abs() < 1e-9 * whole,
                    "{what}: the halves hold {sum}, the body {whole}"
                );
                let cut = plane_section(&finished("the body", body.clone(), tol()), &plane, tol())
                    .unwrap_or_else(|e| panic!("{what}: the section refused: {e:?}"));
                let [region] = cut.regions.as_slice() else {
                    panic!("{what}: {} regions, not one", cut.regions.len());
                };
                assert!(
                    (region.area() - section).abs() < 1e-9 * section + eps,
                    "{what}: section area {}, closed form {section}",
                    region.area()
                );
                let points = region.outline.points();
                // Two corners closer than the escalation distance are one.
                let near = eps * tol().k();
                let repeated = (0..points.len())
                    .any(|i| (i + 1..points.len()).any(|j| points[i].distance(points[j]) <= near));
                // The sector's four, and the keel's open end on the
                // annulus's end face.
                assert!(
                    points.len() == 5 && !repeated,
                    "{what}: the outline {points:?}, not 5 distinct corners"
                );
            }
        }
    }
}

/// **A curved keel closed at both ends splits with the keel a ring.**
/// The V turned 60° about `y`, so its 150° lies inside the annulus's
/// 270° and the material above `y = 1` joins round both ends of its
/// keel. Split either way up, at the V's tip on the plane and `0.6 ε`
/// either side of it: each side keeps its closed-form volume and passes
/// tiers 1, 2 and 3, and the side below the walls tier 3′, where the
/// walls' side refuses only the ring's unrecorded overlap; that side
/// carries the keel
/// on its section face as a ring of two distinct arcs between one vertex
/// at each end, and the other side keeps no vertex at either end; and
/// `plane_section` reads one region, the 270° annular sector, with no
/// hole.
#[test]
fn a_curved_keel_closed_at_both_ends_splits_with_its_keel_a_ring() {
    let eps = tol().eps();
    let sector = 0.75 * PI * (3.0f64.powi(2) - 0.5f64.powi(2));
    // The V's part in the annulus above `y = 1`, with its tip at `tip`:
    // its section there, half-width `0.3 (y − tip) / (2.5 − tip)` up to
    // `y = 2`, about its centroid's radius 2 through 150°.
    let v = |tip: f64| {
        let low = tip.max(1.0);
        0.3 / (2.5 - tip) * ((2.0 - tip).powi(2) - (low - tip).powi(2)) * 2.0 * 150f64.to_radians()
    };
    // On the plane at the keel's radius.
    let on_keel = |p: &Point3<f64>| (p.y - 1.0).abs() < 1e-6 && (p.x.hypot(p.z) - 2.0).abs() < 1e-6;
    for s in [0.0, 0.6, -0.6] {
        let body = keeled(s, 60.0);
        for up in [1.0, -1.0] {
            let what = format!("the V's tip {s} ε off the plane, the plane's normal {up} y");
            let plane = topo::test_support::split_plane(
                Point3::new(0.0, 1.0, 0.0),
                Vec3::new(0.0, up, 0.0),
                tol(),
            );
            let [below, above] = halves_at_rest(&what, &body, &plane);
            let (walls, floor) = if up > 0.0 {
                (above, below)
            } else {
                (below, above)
            };
            for (side, b, volume) in [
                ("walls", &walls, sector - v(1.0 + s * eps)),
                ("floor", &floor, sector),
            ] {
                let what = format!("{what}, the {side} side");
                assert_eq!(topo::validate_closed(b), Ok(()), "{what}: tier 2");
                let got = mass_properties(b, tol()).unwrap().volume;
                assert!(
                    (got - volume).abs() < 1e-9 * volume,
                    "{what}: volume {got}, closed form {volume}"
                );
            }
            let keel_vertices = |b: &Body<f64>| {
                b.vertices()
                    .filter(|(_, v)| on_keel(b.get_point(v.point).unwrap()))
                    .map(|(k, _)| k)
                    .collect::<Vec<_>>()
            };
            assert!(
                keel_vertices(&floor).is_empty(),
                "{what}: the floor side keeps a vertex at the keel"
            );
            let ends = keel_vertices(&walls);
            let edges_of = |l| match walls.get_loop(l).unwrap().boundary {
                topo::LoopBoundary::Cycle { first } => walls
                    .loop_cycle(first)
                    .unwrap()
                    .into_iter()
                    .map(|he| walls.get_half_edge(he).unwrap().edge)
                    .collect(),
                topo::LoopBoundary::Empty { .. } => Vec::new(),
            };
            let rings: Vec<Vec<topo::EdgeKey>> = walls
                .faces()
                .flat_map(|(_, f)| f.rings.clone())
                .map(edges_of)
                .filter(|edges| {
                    edges.iter().any(|&e| {
                        let he = walls.get_edge(e).unwrap().he_plus;
                        ends.contains(&walls.get_half_edge(he).unwrap().start)
                    })
                })
                .collect();
            let [ring] = rings.as_slice() else {
                panic!("{what}: {} rings meet the keel, not one", rings.len());
            };
            let ring_ends = |e: topo::EdgeKey| {
                let he = walls.get_edge(e).unwrap().he_plus;
                let mut v = [
                    walls.get_half_edge(he).unwrap().start,
                    walls.half_edge_end(he).unwrap(),
                ];
                v.sort();
                v
            };
            assert!(
                ends.len() == 2
                    && ring.len() == 2
                    && ring[0] != ring[1]
                    && ring_ends(ring[0]) == ring_ends(ring[1])
                    && ring_ends(ring[0]).iter().all(|v| ends.contains(v)),
                "{what}: the keel's ring is {ring:?} over the keel's vertices {ends:?}, not two \
                 distinct edges between one vertex at each end"
            );
            let records = topo::ContactRecords::default();
            assert_eq!(
                topo::validate_pseudomanifold(&floor, &records, tol()),
                Ok(()),
                "{what}: the floor side's tier 3′"
            );
            // The census reads the ring's two arcs, on two curve entries,
            // as an overlap no record backs
            // (`work/cleave/a-curved-slit-fails-tier-3-prime-on-its-two-coincident-arcs.md`).
            match topo::validate_pseudomanifold(&walls, &records, tol())
                .as_ref()
                .map_err(Vec::as_slice)
            {
                Err(
                    [
                        topo::ValidationError::UnrecordedContact {
                            contact: topo::CensusContact::EdgeEdgeOverlap { a, b },
                            ..
                        },
                    ],
                ) if [a, b] == [&ring[0], &ring[1]] || [a, b] == [&ring[1], &ring[0]] => {}
                other => panic!(
                    "{what}: the walls side's tier 3′ {other:?}, not the ring's unrecorded overlap"
                ),
            }
            let cut = plane_section(&finished("the body", body.clone(), tol()), &plane, tol())
                .unwrap_or_else(|e| panic!("{what}: the section refused: {e:?}"));
            let [region] = cut.regions.as_slice() else {
                panic!("{what}: {} regions, not one", cut.regions.len());
            };
            assert!(
                (region.area() - sector).abs() < 1e-9 * sector + eps
                    && region.holes.is_empty()
                    && region.outline.points().len() == 4,
                "{what}: section area {} over {} corners and {} hole(s), not the sector's \
                 {sector} over 4",
                region.area(),
                region.outline.points().len(),
                region.holes.len()
            );
        }
    }
}
