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
//! keel's arc leaves whole; with both ends closed, the split refuses.
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

/// **A curved keel closed at both ends refuses, as a construction gap.**
/// The V turned 60° about `y`, so its 150° lies inside the annulus's
/// 270° and the material above `y = 1` joins round both ends of its
/// keel. The ring that side would carry is two coincident arcs, which no
/// contact record backs and tier 3′ refuses, so the split refuses the
/// trace instead of shipping it
/// (`work/cleave/a-curved-slit-fails-tier-3-prime-on-its-two-coincident-arcs.md`):
/// either way up, with the V's tip on the plane and `0.6 ε` either side.
#[test]
fn a_curved_keel_closed_at_both_ends_refuses_as_a_construction_gap() {
    for s in [0.0, 0.6, -0.6] {
        let body = finished("the body", keeled(s, 60.0), tol());
        for up in [1.0, -1.0] {
            let what = format!("the V's tip {s} ε off the plane, the plane's normal {up} y");
            let plane = topo::test_support::split_plane(
                Point3::new(0.0, 1.0, 0.0),
                Vec3::new(0.0, up, 0.0),
                tol(),
            );
            match topo::split(&body, &plane, tol()) {
                Err(topo::SplitError::Join(topo::SplitJoinError::CurvedSlitTrace { .. })) => {}
                other => panic!(
                    "{what}: {:?}, not the curved slit trace's refusal",
                    other.map(drop)
                ),
            }
        }
    }
}
