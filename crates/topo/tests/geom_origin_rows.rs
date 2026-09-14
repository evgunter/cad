//! **The origin channel**: `Body::surface_origin` and its siblings
//! answer *which* absence a missing `GeomSource` is.
//!
//! The channel these rows test is `topo::GeomOrigin`, read beside the
//! N6 `GeomSource` maps and never inside them — so every row here also
//! states what the `GeomSource` maps say, and those statements are the
//! merge base's whole answer, unchanged by this channel's existence.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use common::prism_z;
use geom_core::{Affine3, Tol, Vec3};
use topo::{Body, GeomOrigin, GeomSource, graft_disjoint, transform_rigid};

const SQUARE: [(f64, f64); 4] = [(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)];

fn brick() -> Body<f64> {
    prism_z::<f64>(&SQUARE, 0.0, 1.0).body
}

fn aside() -> Affine3<f64> {
    Affine3::translation(Vec3::new(3.0, 0.0, 0.0))
}

/// Every description's origin, in arena order, as a printable readout —
/// surfaces, then curves, then points.
fn origins(b: &Body<f64>) -> Vec<String> {
    let mut v: Vec<String> = b
        .surfaces()
        .map(|(k, _)| format!("{:?}", b.surface_origin(k).unwrap()))
        .collect();
    v.extend(
        b.curves()
            .map(|(k, _)| format!("{:?}", b.curve_origin(k).unwrap())),
    );
    v.extend(
        b.points()
            .map(|(k, _)| format!("{:?}", b.point_origin(k).unwrap())),
    );
    v
}

/// [`origins`] reduced to the arm names — what a reader asking "which
/// absence is this" sees, with the `Recipe` arm's payload dropped.
fn arms(b: &Body<f64>) -> Vec<String> {
    origins(b)
        .into_iter()
        .map(|o| o.split('(').next().unwrap().to_string())
        .collect()
}

/// The same readout through the MERGE BASE's only reader: the
/// `GeomSource` maps. Every row that changes an origin asserts this
/// too, because N6 decides on these maps and on nothing else.
fn sources(b: &Body<f64>) -> Vec<String> {
    let mut v: Vec<String> = b
        .surfaces()
        .map(|(k, _)| format!("{:?}", b.surface_source(k)))
        .collect();
    v.extend(
        b.curves()
            .map(|(k, _)| format!("{:?}", b.curve_source(k))),
    );
    v.extend(
        b.points()
            .map(|(k, _)| format!("{:?}", b.point_source(k))),
    );
    v
}

/// Stamps every description of `b` with a distinct `minted(node, i)`
/// — the recipe layer's `stamp_minted`, spelled through the same
/// public doors it uses.
fn stamp_all(b: &mut Body<f64>, node: u64) {
    let sk: Vec<_> = b.surfaces().map(|(k, _)| k).collect();
    let ck: Vec<_> = b.curves().map(|(k, _)| k).collect();
    let pk: Vec<_> = b.points().map(|(k, _)| k).collect();
    let mut idx: u32 = 0;
    for k in sk {
        b.set_surface_source(k, GeomSource::minted(node, idx)).unwrap();
        idx += 1;
    }
    for k in ck {
        b.set_curve_source(k, GeomSource::minted(node, idx)).unwrap();
        idx += 1;
    }
    for k in pk {
        b.set_point_source(k, GeomSource::minted(node, idx)).unwrap();
        idx += 1;
    }
}

/// CONTROL: a body built through the kernel's own doors, with no
/// recipe above it, reads `KernelDirect` at every description — a
/// positive origin, not an inference from a missing row.
#[test]
fn a_hand_built_body_reads_kernel_direct() {
    let b = brick();
    assert!(!origins(&b).is_empty());
    assert!(
        origins(&b).iter().all(|o| o == "KernelDirect"),
        "{:?}",
        origins(&b)
    );
    // The merge base's whole answer, unchanged.
    assert!(sources(&b).iter().all(|s| s == "None"));
}

/// **RED-FIRST.** `transform_rigid` on a stamped body with no
/// re-stamp: at the merge base the cleared body's provenance readout
/// is byte-for-byte the hand-built body's (both all-`None`, measured
/// — see the PR body's merge-base probe), so the defect is invisible.
/// Here the two absences are different absences by name.
#[test]
fn a_cleared_body_and_a_hand_built_one_are_no_longer_one_answer() {
    let tol = Tol::witness();
    let hand = brick();
    let mut stamped = brick();
    stamp_all(&mut stamped, 9001);
    // No `compose_placed` follows: this is exactly the lost re-stamp.
    let placed = transform_rigid(&stamped, &aside(), tol).unwrap();

    // What the merge base could see, still true and still identical:
    // the N6 channel is empty on both, so nothing N6 decides moved.
    assert_eq!(
        sources(&placed),
        sources(&hand),
        "the GeomSource maps must stay indistinguishable — that is the N6 channel, untouched"
    );
    assert!(sources(&placed).iter().all(|s| s == "None"));

    // What the head can see: two different absences.
    assert!(
        origins(&placed).iter().all(|o| o == "Cleared"),
        "{:?}",
        origins(&placed)
    );
    assert!(origins(&hand).iter().all(|o| o == "KernelDirect"));
    assert_ne!(origins(&placed), origins(&hand));
}

/// CONTROL: the transform-then-re-stamp path — the one the recipe
/// layer actually runs — leaves the channel with the composed stamps
/// and **no `Cleared` residue**: every mark the clearing door wrote is
/// discharged by the re-stamp that answers for it.
#[test]
fn the_restamp_discharges_every_cleared_mark() {
    let tol = Tol::witness();
    let mut stamped = brick();
    stamp_all(&mut stamped, 9001);
    let before = sources(&stamped);
    let mut placed = transform_rigid(&stamped, &aside(), tol).unwrap();

    // `compose_placed`'s rule, spelled through the public doors: keys
    // are stable across the map, so the input's rows map key for key.
    let s: Vec<_> = stamped
        .surfaces()
        .map(|(k, _)| (k, stamped.surface_source(k).unwrap().placed(77, 0)))
        .collect();
    for (k, src) in s {
        placed.set_surface_source(k, src).unwrap();
    }
    let c: Vec<_> = stamped
        .curves()
        .map(|(k, _)| (k, stamped.curve_source(k).unwrap().placed(77, 0)))
        .collect();
    for (k, src) in c {
        placed.set_curve_source(k, src).unwrap();
    }
    let p: Vec<_> = stamped
        .points()
        .map(|(k, _)| (k, stamped.point_source(k).unwrap().placed(77, 0)))
        .collect();
    for (k, src) in p {
        placed.set_point_source(k, src).unwrap();
    }

    assert!(
        origins(&placed).iter().all(|o| o.starts_with("Recipe(")),
        "{:?}",
        origins(&placed)
    );
    // And the stamps are the composed ones, not the pre-placement
    // ones — the channel moved exactly as N6 says it does.
    assert_ne!(sources(&placed), before);
    for (k, _) in placed.surfaces() {
        assert_eq!(
            placed.surface_origin(k),
            Some(GeomOrigin::Recipe(placed.surface_source(k).unwrap()))
        );
    }
}

/// CONTROL: `revert ∘ revert` leaves both channels byte-identical, and
/// one `revert` moves N6's orient tag and nothing else. The origin ARM
/// is untouched by a reversal — a reversal neither stamps nor clears —
/// while the `Recipe` arm's payload IS the source, so it flips with it.
#[test]
fn revert_twice_leaves_both_channels_identical() {
    let mut stamped = brick();
    stamp_all(&mut stamped, 9001);
    let s0 = sources(&stamped);
    let o0 = origins(&stamped);
    let a0 = arms(&stamped);

    let once = stamped.revert().unwrap();
    assert_eq!(arms(&once), a0, "a reversal does not move an origin's arm");
    assert_ne!(sources(&once), s0, "N6's orient tag flipped");
    assert_ne!(
        origins(&once),
        o0,
        "the Recipe arm carries the source, so it flips with it"
    );

    let twice = once.revert().unwrap();
    assert_eq!(sources(&twice), s0);
    assert_eq!(origins(&twice), o0);
}

/// CONTROL: the clearing door touches only what it dropped. A body
/// with no recipe sources goes through `transform_rigid` and keeps the
/// origin it had — `Cleared` is a claim about a source that existed,
/// not about the door having run.
#[test]
fn clearing_marks_only_the_descriptions_that_held_a_source() {
    let tol = Tol::witness();
    let mut b = brick();
    b.mark_imported();
    assert!(origins(&b).iter().all(|o| o == "Imported"));
    let placed = transform_rigid(&b, &aside(), tol).unwrap();
    assert!(
        origins(&placed).iter().all(|o| o == "Imported"),
        "an adopted description held no recipe source, so the clear had nothing to drop: {:?}",
        origins(&placed)
    );

    // And a body where only SOME descriptions carry a source: the
    // clear marks those and leaves the rest alone.
    let mut mixed = brick();
    let first = mixed.surfaces().map(|(k, _)| k).next().unwrap();
    mixed.set_surface_source(first, GeomSource::minted(5, 0)).unwrap();
    let placed = transform_rigid(&mixed, &aside(), tol).unwrap();
    assert_eq!(placed.surface_origin(first), Some(GeomOrigin::Cleared));
    let others: Vec<_> = placed
        .surfaces()
        .filter(|&(k, _)| k != first)
        .map(|(k, _)| placed.surface_origin(k))
        .collect();
    assert!(
        others.iter().all(|o| *o == Some(GeomOrigin::KernelDirect)),
        "{others:?}"
    );
}

/// CONTROL: an origin rides the graft with its description, and the
/// destination's own geometry keeps its own origin — the mark is
/// per-description, which is why a body-level flag could not carry it
/// (`graft_disjoint` puts an imported solid into a native body).
#[test]
fn an_origin_rides_the_graft_and_the_destination_keeps_its_own() {
    let tol = Tol::witness();
    let mut dst = brick();
    let native: Vec<_> = dst.surfaces().map(|(k, _)| k).collect();
    let mut src = prism_z::<f64>(&SQUARE, 0.0, 1.0).body;
    src.mark_imported();
    let src = transform_rigid(&src, &aside(), tol).unwrap();
    graft_disjoint(&mut dst, &src, tol).unwrap();

    let grafted: Vec<_> = dst
        .surfaces()
        .map(|(k, _)| k)
        .filter(|k| !native.contains(k))
        .collect();
    assert_eq!(grafted.len(), native.len());
    for k in grafted {
        assert_eq!(dst.surface_origin(k), Some(GeomOrigin::Imported));
    }
    for k in native {
        assert_eq!(dst.surface_origin(k), Some(GeomOrigin::KernelDirect));
    }
}

/// `None` from an origin reader is the key failing to resolve, not an
/// origin — the same answer every other lookup on `Body` gives a key
/// it does not hold.
#[test]
fn a_key_the_body_does_not_hold_has_no_origin() {
    let b = brick();
    let empty = Body::<f64>::new();
    let (sk, _) = b.surfaces().next().unwrap();
    let (ck, _) = b.curves().next().unwrap();
    let (pk, _) = b.points().next().unwrap();
    assert_eq!(empty.surface_origin(sk), None);
    assert_eq!(empty.curve_origin(ck), None);
    assert_eq!(empty.point_origin(pk), None);
}
