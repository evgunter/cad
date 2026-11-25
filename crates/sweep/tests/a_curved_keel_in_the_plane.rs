//! **A curved keel lying in the plane splits sound and sections with no
//! spur.** A 270° partial revolve of the annulus `0.5..3 × 0..2` about
//! `y`, less a 150° partial revolve of a V whose tip lies at `y = 1` (to
//! within `0.9 ε`), turned about `y`. The keel is a circular arc of
//! radius 2 in the plane `y = 1`. Turned so that one end of the keel
//! opens through the annulus's end face, the material above the plane
//! runs round its closed end; turned so that both ends close inside the
//! annulus, it runs round both. Split by that plane either way up: with
//! one end open, each side keeps its volume and passes tier 3, and
//! `plane_section` reads one region, the 270° annular sector, which the
//! keel's arc leaves whole. Either way, the side round the keel holds it
//! as two coincident arcs, a slit or a zero-area ring, and passes tier 3′
//! on the edge–edge record the split makes of them.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common::bores::halves_at_rest;
use core::f64::consts::PI;
use geom_core::{Affine3, Point2, Point3, Tol, Vec3};
use sweep::Revolution;
use sweep::test_support::{finished, revolved_about_y};
use topo::splitting::plane_section;
use topo::{Body, BooleanResult, subtract, transform_rigid};

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
    let v = transform_rigid(&v, &geom_core::test_support::rigid(&turn, tol())).unwrap();
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
    // Pinned red at 1e-6: with the tip 0.6 ε off the plane the halves
    // hold 5.2e-7 less than the body, where the row wants them to keep
    // its volume to 1e-9 of it
    // (`work/onedoor/main-red-four-sweep-rows-hold-volumes-the-one-surface-record-moves-at-1e-6.md`).
    test_utils::pinned_red::row(
        tol().eps(),
        &[1e-6],
        "the curved keel in the plane",
        &[
            "the V turned -20°, its tip 0.6 ε off the plane, the plane's normal 1 y",
            "the halves hold 40.32583257",
            "the body 40.32583309",
        ],
        a_curved_keel_in_the_plane_splits_sound_and_sections_with_no_spur_row,
    );
}

/// The row [`a_curved_keel_in_the_plane_splits_sound_and_sections_with_no_spur`]
/// runs, under its pin.
fn a_curved_keel_in_the_plane_splits_sound_and_sections_with_no_spur_row() {
    let eps = tol().eps();
    // The 270° sector of the annulus between radii 0.5 and 3.
    let section = 0.75 * PI * (3.0f64.powi(2) - 0.5f64.powi(2));
    for turn in [-20.0, 160.0, -160.0] {
        for s in [0.0, 0.6, -0.6, 0.9, -0.9] {
            let what = format!("the V turned {turn}°, its tip {s} ε off the plane");
            let body = topo::test_support::finished("body", keeled(s, turn), tol());
            let whole = topo::test_support::pinned_volume("the measured body", &body, tol()).volume;
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
                    .map(|h| {
                        topo::test_support::pinned_volume(
                            "the measured body",
                            &topo::test_support::finished(
                                "the measured body",
                                topo::Body::clone(h),
                                tol(),
                            ),
                            tol(),
                        )
                        .volume
                    })
                    .sum();
                assert!(
                    (sum - whole).abs() < 1e-9 * whole,
                    "{what}: the halves hold {sum}, the body {whole}"
                );
                let cut = plane_section(
                    &finished("the body", topo::Body::clone(&body.clone()), tol()),
                    &plane,
                    tol(),
                )
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

/// **A curved keel's slit and ring pass tier 3′ on the split's records.**
/// The V turned 60° about `y` lies inside the annulus's 270°, so the
/// material on the walls' side of `y = 1` joins round both ends of its
/// keel and carries it as a zero-area ring of two coincident arcs; turned
/// −20°, the keel opens through the end face and the arcs are a slit of
/// the section's outline. Either way up, with the V's tip on the plane
/// and `0.6 ε` either side, each side passes tiers 1, 3 and 3′, and the
/// walls' side holds one
/// edge–edge record of its two arcs, citing the split's row of the
/// keel ON the plane (`DecisionSite::SplitOn`, an operand edge and the
/// plane). *Breaks if* the split mints the arcs unrecorded (3′ refuses an
/// `EdgeEdgeOverlap`) or records a pair whose interiors do not meet (the
/// gate refuses it stale).
#[test]
fn a_curved_keel_slit_or_ring_passes_tier_3_prime_on_its_records() {
    for turn in [60.0, -20.0] {
        for s in [0.0, 0.6, -0.6] {
            let body = finished("the body", keeled(s, turn), tol());
            for up in [1.0, -1.0] {
                let what =
                    format!("the V turned {turn}°, its tip {s} ε off the plane, normal {up} y");
                let plane = topo::test_support::split_plane(
                    Point3::new(0.0, 1.0, 0.0),
                    Vec3::new(0.0, up, 0.0),
                    tol(),
                );
                let cut = topo::split(&body, &plane, tol())
                    .unwrap_or_else(|e| panic!("{what}: splits: {e:?}"));
                let mut recorded = Vec::new();
                for (side, part) in [("above", &cut.above), ("below", &cut.below)] {
                    let topo::SplitPart::Body(half) = part else {
                        panic!("{what} {side}: material on both sides");
                    };
                    assert_eq!(topo::validate(half), Ok(()), "{what} {side}: tier 1");
                    assert_eq!(
                        topo::validate_geometric(half, tol()),
                        Ok(()),
                        "{what} {side}: tier 3"
                    );
                    assert_eq!(
                        topo::validate_pseudomanifold(half, half.records(), tol()),
                        Ok(()),
                        "{what} {side}: tier 3′ on its records {:?}",
                        half.records()
                    );
                    recorded.extend(half.records().rows().iter().map(|r| (side, r.clone())));
                }
                let [(side, record)] = recorded.as_slice() else {
                    panic!("{what}: records {recorded:?}, not one");
                };
                let walls = if up > 0.0 { "above" } else { "below" };
                assert_eq!(*side, walls, "{what}: the record is on the walls' side");
                assert!(
                    matches!(record.kind(), topo::ContactKind::EdgeEdge(..)),
                    "{what}: {record:?} is no edge–edge record"
                );
                let rows: Vec<_> = record
                    .cites
                    .iter()
                    .map(|b| match b {
                        topo::Backing::Decided(k) => cut.coincidences[k as usize],
                        other => panic!("{what}: {record:?} cites {other:?}, not the split's row"),
                    })
                    .collect();
                let [row] = rows.as_slice() else {
                    panic!("{what}: {record:?} cites {rows:?}, not one row");
                };
                assert!(
                    row.site == topo::DecisionSite::SplitOn
                        && row.relation == topo::Relation::OnCarrier
                        && matches!(
                            row.cells,
                            [
                                topo::RowCell::Input {
                                    cell: topo::Cell::Edge(_),
                                    ..
                                },
                                topo::RowCell::Tool
                            ]
                        ),
                    "{what}: {row:?} is not the keel's ON row"
                );
            }
        }
    }
}

/// The full-revolution twin of [`keeled`]: the whole annulus less the
/// whole V, so the keel is one closed circle of radius 2 in `y = 1`.
fn keeled_round(s: f64) -> Body<f64> {
    let loop_of = |pts: &[(f64, f64)]| pts.iter().map(|&(x, y)| (Point2::new(x, y), 0.0)).collect();
    let ring = revolved_about_y(
        loop_of(&[(0.5, 0.0), (3.0, 0.0), (3.0, 2.0), (0.5, 2.0)]),
        Revolution::Full,
        tol(),
    );
    let tip = 1.0 + s * tol().eps();
    let v = revolved_about_y(
        loop_of(&[(1.7, 2.5), (2.0, tip), (2.3, 2.5)]),
        Revolution::Full,
        tol(),
    );
    match subtract(
        &finished("the annulus", ring, tol()),
        &finished("the V", v, tol()),
        tol(),
    ) {
        Ok(BooleanResult::Body(b)) => b.body.into_body(),
        other => panic!("the annulus less the V: {other:?}"),
    }
}

/// **A round keel splits sound, and its section reads the side it is
/// read from.** The [`keeled_round`] body, its tip on `y = 1` and `0.6 ε`
/// either side, split by that plane either way up. The halves keep the
/// body's volume.
///
/// - **Read from below the keel** (the plane's normal `+y`, the side
///   whose material runs on under the circle): one annulus, `0.5..3`.
///   That side's two section loops at the circle are its one edge walked
///   both ways and bound no area (`section::closed_keels`), as the finish
///   merges that half's section into one face. *Breaks if* the reading
///   keeps the two loops, which reads the two annuli either side of the
///   circle.
/// - **Read from the V's side** (normal `−y`): that side is two pieces
///   touching along the circle, and the section reads them as they are
///   built, the two annuli `0.5..2` and `2..3`, by `plane_section`'s
///   contract: it reads the reading side's section faces, and takes out
///   only the slits where a piece touches itself (`drop_slits`), which
///   two pieces touching each other are not.
#[test]
fn a_round_keel_in_the_plane_reads_the_side_it_is_read_from() {
    // Pinned red at 1e-6, read from the V's side alone: with the tip
    // 0.6 ε off the plane each piece's boundary is its wall's trace on the
    // plane, 0.2 × 0.6 ε off the keel, so the two regions overlap and each
    // reads 1.5e-6 over its annulus, where the row holds them to 1e-8
    // (`work/onehome/the-v-sides-keel-regions-read-their-walls-plane-traces-off-the-keel.md`).
    test_utils::pinned_red::row(
        tol().eps(),
        &[1e-6],
        "the round keel in the plane",
        &[
            "the V's tip 0.6 ε off the plane, the plane's normal -1 y",
            "the section's regions hold [11.78097395",
            "15.70796477",
        ],
        a_round_keel_in_the_plane_reads_the_side_it_is_read_from_row,
    );
}

/// The row [`a_round_keel_in_the_plane_reads_the_side_it_is_read_from`]
/// runs, under its pin.
fn a_round_keel_in_the_plane_reads_the_side_it_is_read_from_row() {
    // Read from under the keel first, at every tip, then from the V's
    // side: the pin at 1e-6 is the V's side's alone.
    for up in [1.0, -1.0] {
        for s in [0.0, 0.6, -0.6] {
            let body = keeled_round(s);
            let whole = topo::test_support::pinned_volume(
                "the measured body",
                &finished("the body", body.clone(), tol()),
                tol(),
            )
            .volume;
            let what = format!("the V's tip {s} ε off the plane, the plane's normal {up} y");
            let plane = topo::test_support::split_plane(
                Point3::new(0.0, 1.0, 0.0),
                Vec3::new(0.0, up, 0.0),
                tol(),
            );
            let sum: f64 = halves_at_rest(&what, &body, &plane)
                .iter()
                .map(|h| {
                    topo::test_support::pinned_volume(
                        "the measured body",
                        &finished("the measured body", h.clone(), tol()),
                        tol(),
                    )
                    .volume
                })
                .sum();
            assert!(
                (sum - whole).abs() < 1e-9 * whole,
                "{what}: the halves hold {sum}, the body {whole}"
            );
            let cut = plane_section(&finished("the body", body.clone(), tol()), &plane, tol())
                .unwrap_or_else(|e| panic!("{what}: the section refused: {e:?}"));
            let mut areas: Vec<f64> = cut.regions.iter().map(|r| r.area()).collect();
            areas.sort_by(f64::total_cmp);
            let want: &[f64] = if up > 0.0 {
                &[PI * (9.0 - 0.25)]
            } else {
                &[PI * (4.0 - 0.25), PI * (9.0 - 4.0)]
            };
            assert!(
                areas.len() == want.len()
                    && areas.iter().zip(want).all(|(a, b)| (a - b).abs() < 1e-8)
                    && cut.regions.iter().all(|r| r.holes.len() == 1),
                "{what}: the section's regions hold {areas:?}, not {want:?}"
            );
        }
    }
}

/// **The curved ring's walls side holds its arcs' record and passes 3′ on
/// it**, in the fast set: the V turned 60°, its tip on `y = 1`, split by
/// that plane, normal `+y`. The split's gate reads the side at tier 3
/// with its records confirmed, so a split that stops recording the arcs
/// still builds; this row reads the record and tier 3′. Red on a split
/// that mints the ring's arcs unrecorded.
#[test]
fn the_curved_rings_side_holds_its_arcs_record() {
    let body = finished("the body", keeled(0.0, 60.0), tol());
    let plane = topo::test_support::split_plane(
        Point3::new(0.0, 1.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
        tol(),
    );
    let cut = topo::split(&body, &plane, tol()).unwrap();
    let topo::SplitPart::Body(walls) = &cut.above else {
        panic!("the walls lie above");
    };
    assert_eq!(
        walls.records().edge_edge().count(),
        1,
        "the ring's two arcs are one edge–edge record: {:?}",
        walls.records()
    );
    assert_eq!(
        topo::validate_pseudomanifold(walls, walls.records(), tol()),
        Ok(()),
        "the walls side at tier 3′ on its records"
    );
}
