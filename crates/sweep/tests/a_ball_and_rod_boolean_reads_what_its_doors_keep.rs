//! A ball's boolean with a rod derives each certificate its doors keep
//! once, and the doors that keep one read it
//! (`geom_brep::locus_certificate`): the restatement, the rebase, the
//! graft and the gate read each chord's locus, and the gate reads each
//! row's.
#![allow(clippy::unwrap_used, clippy::expect_used)]

use geom_brep::Certified;
use geom_brep::locus_certificate::counts;
use geom_core::{Point2, Point3, Tol, Vec3};
use profile::{Profile, SketchPlane};
use sweep::test_support::{at_rest, ball_poled, finished};
use sweep::{ExtrudeSide, Extrusion, extrude};

/// The rod of `the_door_reads_seam_regions_as_tier_3_does.rs`.
fn rod(tol: Tol) -> topo::AtRestBody<f64> {
    let lp = profile::circle(Point2::new(0.1, 0.0), 0.3, tol).unwrap();
    let plane = SketchPlane::xy_at(Point3::new(0.0, 0.0, -2.0));
    let profile = Profile::new(plane, vec![lp.into()]).validate(tol).unwrap();
    let body = extrude(
        &profile,
        Extrusion::Distance {
            depth: 4.0,
            side: ExtrudeSide::Along,
        },
        tol,
    )
    .unwrap()
    .body
    .into_body();
    at_rest(&body, tol)
}

/// **The unit ball ∪ the rod reads what its doors keep**: of the
/// operation's locus certificates (its intersection edges', chords and
/// circles alike) 12 are derived, each once, at the join's edge door,
/// and the doors that keep them read 40 (restatement, rebase, graft,
/// partner copy and gate); the gate reads the 8 projected rows the mint
/// derived. The counts are this thread's, taken around the one
/// operation, as at least the reads and at most the derivations
/// measured when the rule landed. Red where a door stops reading what
/// it keeps: its read becomes a derivation.
#[test]
fn the_unit_ball_and_the_rod_read_what_their_doors_keep() {
    let tol = Tol::witness();
    let ball = finished(
        "a",
        ball_poled(1.0, Vec3::new(0.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0), tol),
        tol,
    );
    let rod = rod(tol);
    let (locus, row) = (counts::of(Certified::Locus), counts::of(Certified::Row));
    let made = topo::boolean::union(&ball, &rod, tol).expect("the union");
    assert!(made.body().is_some(), "the union is one body");
    let delta = |now: (u64, u64), then: (u64, u64)| (now.0 - then.0, now.1 - then.1);
    let locus = delta(counts::of(Certified::Locus), locus);
    let row = delta(counts::of(Certified::Row), row);
    println!("locus (reads, derivations) {locus:?}; rows {row:?}");
    assert!(
        locus.0 >= 40 && locus.1 <= 12,
        "locus (reads, derivations): {locus:?}"
    );
    assert!(row.0 >= 8, "the gate's row reads: {row:?}");
}
