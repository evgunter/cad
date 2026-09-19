//! **The origin channel**: `Body::surface_origin` and its siblings
//! answer *which* absence a missing `GeomSource` is.
//!
//! The channel these rows test is `topo::GeomOrigin`, the one
//! provenance row a body keeps per description — so every row here also
//! states what the `GeomSource` projection says, and those statements
//! are the merge base's whole answer, unchanged by the other three arms
//! existing.
//!
//! **Rows that assert on the origin channel run on a body whose
//! non-recipe arms are NON-EMPTY.** A fully stamped body carries
//! nothing but `Recipe`, so a door that dropped every `Imported` and
//! `Cleared` row would leave such a row green: [`mixed`] is the fixture
//! that refuses that, and `arms_are_mixed` is the assertion that keeps
//! it honest.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use common::brick;
use geom_core::{Affine3, Tol, Vec3};
use topo::{Body, GeomOrigin, GeomSource, graft_disjoint, transform_rigid};

/// The unit cube — the one body every row here runs on, since what
/// these rows read is the origin channel and never the shape.
fn unit_brick() -> Body<f64> {
    brick((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), Tol::witness())
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
/// `GeomSource` projection. Every row that changes an origin asserts
/// this too, because N6 decides on it and on nothing else.
fn sources(b: &Body<f64>) -> Vec<String> {
    let mut v: Vec<String> = b
        .surfaces()
        .map(|(k, _)| format!("{:?}", b.surface_source(k)))
        .collect();
    v.extend(b.curves().map(|(k, _)| format!("{:?}", b.curve_source(k))));
    v.extend(b.points().map(|(k, _)| format!("{:?}", b.point_source(k))));
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
        b.set_surface_source(k, GeomSource::minted(node, idx))
            .unwrap();
        idx += 1;
    }
    for k in ck {
        b.set_curve_source(k, GeomSource::minted(node, idx))
            .unwrap();
        idx += 1;
    }
    for k in pk {
        b.set_point_source(k, GeomSource::minted(node, idx))
            .unwrap();
        idx += 1;
    }
}

/// Stamps the SURFACES only, leaving curves and points on whatever arm
/// they already hold — the half-stamped shape [`mixed`] is built from.
fn stamp_surfaces(b: &mut Body<f64>, node: u64) {
    let sk: Vec<_> = b.surfaces().map(|(k, _)| k).collect();
    for (i, k) in sk.into_iter().enumerate() {
        b.set_surface_source(k, GeomSource::minted(node, i as u32))
            .unwrap();
    }
}

/// A body carrying all three non-`KernelDirect` arms at once:
/// `Imported` on every curve and point (adopted, never stamped),
/// `Cleared` on the surfaces a recipe stamped and a placement cleared,
/// and `Recipe` on the one surface re-stamped afterwards.
///
/// Every row that asserts about the origin channel runs on this rather
/// than on a fully stamped body, because a fully stamped body's only
/// arm is `Recipe` and a door that dropped the other two would go
/// unseen.
fn mixed() -> Body<f64> {
    let tol = Tol::witness();
    let mut adopted = unit_brick();
    adopted.mark_imported();
    stamp_surfaces(&mut adopted, 9001);
    let mut b = transform_rigid(&adopted, &aside(), tol).unwrap();
    let first = b.surfaces().map(|(k, _)| k).next().unwrap();
    b.set_surface_source(first, GeomSource::minted(9001, 0).placed(77, 0))
        .unwrap();
    b
}

/// Asserts `b` exercises all three arms `mixed` promises — the guard
/// that keeps a row over [`mixed`] from quietly becoming vacuous.
fn arms_are_mixed(b: &Body<f64>) {
    let a = arms(b);
    for arm in ["Imported", "Cleared", "Recipe"] {
        assert!(a.iter().any(|s| s == arm), "no {arm} arm in {a:?}");
    }
}

/// CONTROL: a body built through the kernel's own doors, with no
/// recipe above it, reads `KernelDirect` at every description — a
/// positive origin **written at the mint**, not a default read off a
/// missing row. A description that reached an arena without one makes
/// the reader `unreachable!`, so what this row reads is a written mark.
#[test]
fn a_hand_built_body_reads_kernel_direct() {
    let b = unit_brick();
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
    let hand = unit_brick();
    let mut stamped = unit_brick();
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
/// layer actually runs — discharges every `Cleared` mark **and leaves
/// the arms it did not clear alone**. Run on a body whose curves and
/// points are `Imported`, so a clearing door that marked every live key
/// or a re-stamp that reached past its own keys is visible here.
#[test]
fn the_restamp_discharges_every_cleared_mark() {
    let tol = Tol::witness();
    let mut adopted = unit_brick();
    adopted.mark_imported();
    stamp_surfaces(&mut adopted, 9001);
    let before = sources(&adopted);
    let mut placed = transform_rigid(&adopted, &aside(), tol).unwrap();

    // The clear touched the surfaces and nothing else.
    for (k, _) in placed.surfaces() {
        assert_eq!(placed.surface_origin(k), Some(&GeomOrigin::Cleared));
    }
    for (k, _) in placed.curves() {
        assert_eq!(placed.curve_origin(k), Some(&GeomOrigin::Imported));
    }
    for (k, _) in placed.points() {
        assert_eq!(placed.point_origin(k), Some(&GeomOrigin::Imported));
    }

    // `compose_placed`'s rule, spelled through the public doors: keys
    // are stable across the map, so the input's rows map key for key.
    let s: Vec<_> = adopted
        .surfaces()
        .map(|(k, _)| (k, adopted.surface_source(k).unwrap().placed(77, 0)))
        .collect();
    for (k, src) in s {
        placed.set_surface_source(k, src).unwrap();
    }

    assert!(
        !arms(&placed).iter().any(|a| a == "Cleared"),
        "{:?}",
        arms(&placed)
    );
    for (k, _) in placed.surfaces() {
        assert!(matches!(
            placed.surface_origin(k),
            Some(GeomOrigin::Recipe(_))
        ));
        // And the stamps are the composed ones, not the pre-placement
        // ones — the channel moved exactly as N6 says it does.
        assert_eq!(
            placed.surface_origin(k),
            Some(&GeomOrigin::Recipe(
                placed.surface_source(k).unwrap().clone()
            ))
        );
    }
    for (k, _) in placed.curves() {
        assert_eq!(placed.curve_origin(k), Some(&GeomOrigin::Imported));
    }
    assert_ne!(sources(&placed), before);
}

/// CONTROL: `revert ∘ revert` leaves both channels byte-identical, and
/// one `revert` moves N6's orient tag and nothing else. The non-recipe
/// arms are untouched by a reversal — a reversal neither stamps nor
/// clears — while the `Recipe` arm's payload IS the source, so it flips
/// with it. Run on [`mixed`]: on a fully stamped body the other two
/// arms are empty and a `revert` that dropped them would go unseen.
#[test]
fn revert_twice_leaves_both_channels_identical() {
    let stamped = mixed();
    arms_are_mixed(&stamped);
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

/// A `revert` carries a non-recipe origin through untouched — the half
/// `revert_twice_leaves_both_channels_identical` could not see before
/// [`mixed`] existed, spelled on the arm with no source to flip.
#[test]
fn revert_preserves_a_non_recipe_origin() {
    let mut b = unit_brick();
    b.mark_imported();
    let once = b.revert().unwrap();
    for (k, _) in once.surfaces() {
        assert_eq!(once.surface_origin(k), Some(&GeomOrigin::Imported));
    }
    for (k, _) in once.curves() {
        assert_eq!(once.curve_origin(k), Some(&GeomOrigin::Imported));
    }
    for (k, _) in once.points() {
        assert_eq!(once.point_origin(k), Some(&GeomOrigin::Imported));
    }
}

/// The defect arm survives everything short of a re-stamp: a second
/// placement finds nothing left to clear and a reversal has no source
/// to flip, so a lost re-stamp stays nameable however far the body
/// travels afterwards.
#[test]
fn a_second_transform_and_a_revert_keep_the_cleared_mark() {
    let tol = Tol::witness();
    let mut stamped = unit_brick();
    let k = stamped.surfaces().map(|(k, _)| k).next().unwrap();
    stamped
        .set_surface_source(k, GeomSource::minted(1, 0))
        .unwrap();
    let once = transform_rigid(&stamped, &aside(), tol).unwrap();
    let twice = transform_rigid(&once, &aside(), tol).unwrap();
    assert_eq!(twice.surface_origin(k), Some(&GeomOrigin::Cleared));
    let reverted = twice.revert().unwrap();
    assert_eq!(reverted.surface_origin(k), Some(&GeomOrigin::Cleared));
}

/// CONTROL: the clearing door touches only what it dropped. A body
/// with no recipe sources goes through `transform_rigid` and keeps the
/// origin it had — `Cleared` is a claim about a source that existed,
/// not about the door having run.
#[test]
fn clearing_marks_only_the_descriptions_that_held_a_source() {
    let tol = Tol::witness();
    let mut b = unit_brick();
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
    let mut mixed = unit_brick();
    let first = mixed.surfaces().map(|(k, _)| k).next().unwrap();
    mixed
        .set_surface_source(first, GeomSource::minted(5, 0))
        .unwrap();
    let placed = transform_rigid(&mixed, &aside(), tol).unwrap();
    assert_eq!(placed.surface_origin(first), Some(&GeomOrigin::Cleared));
    let others: Vec<_> = placed
        .surfaces()
        .filter(|&(k, _)| k != first)
        .map(|(k, _)| placed.surface_origin(k))
        .collect();
    assert!(
        others.iter().all(|o| *o == Some(&GeomOrigin::KernelDirect)),
        "{others:?}"
    );
}

/// `Body::mark_imported` marks the `KernelDirect` arm and leaves the
/// defect arm alone: an import claim laid over a `Cleared` row would
/// erase, one public door away, exactly the evidence the clearing door
/// recorded. The recipe arm is left alone for the older reason — the
/// recipe is the finer identity.
#[test]
fn mark_imported_leaves_the_cleared_and_recipe_arms_alone() {
    let mut b = mixed();
    arms_are_mixed(&b);
    let before = arms(&b);
    let cleared: Vec<_> = b
        .surfaces()
        .map(|(k, _)| k)
        .filter(|&k| b.surface_origin(k) == Some(&GeomOrigin::Cleared))
        .collect();
    assert!(!cleared.is_empty());

    b.mark_imported();

    for k in cleared {
        assert_eq!(
            b.surface_origin(k),
            Some(&GeomOrigin::Cleared),
            "the defect arm did not survive mark_imported"
        );
    }
    assert_eq!(
        arms(&b),
        before,
        "every description was already Imported, Cleared or Recipe — nothing to mark"
    );
}

/// A recipe stamp over `Imported` is **lossy**: the import fact is
/// gone for good, and a later clear leaves `Cleared`, never `Imported`.
/// No caller in the tree can reach this today — nothing stamps an
/// adopted body — and `work/exch/step-import-discards-the-entity-ids-
/// that-are-its-identity-channel` (EXCH's step 2) is the change that
/// will, so the behaviour is characterised here rather than left to be
/// discovered then.
#[test]
fn stamping_an_imported_body_erases_the_import_fact() {
    let tol = Tol::witness();
    let mut b = unit_brick();
    b.mark_imported();
    let k = b.surfaces().map(|(k, _)| k).next().unwrap();
    b.set_surface_source(k, GeomSource::minted(1, 0)).unwrap();
    assert!(matches!(b.surface_origin(k), Some(GeomOrigin::Recipe(_))));
    let placed = transform_rigid(&b, &aside(), tol).unwrap();
    assert_eq!(placed.surface_origin(k), Some(&GeomOrigin::Cleared));
}

/// CONTROL: an origin rides the graft with its description, and the
/// destination's own geometry keeps its own origin — the mark is
/// per-description, which is why a body-level flag could not carry it
/// (`graft_disjoint` puts an imported solid into a native body).
#[test]
fn an_origin_rides_the_graft_and_the_destination_keeps_its_own() {
    let tol = Tol::witness();
    let mut dst = unit_brick();
    let native: Vec<_> = dst.surfaces().map(|(k, _)| k).collect();
    let mut src = unit_brick();
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
        assert_eq!(dst.surface_origin(k), Some(&GeomOrigin::Imported));
    }
    for k in native {
        assert_eq!(dst.surface_origin(k), Some(&GeomOrigin::KernelDirect));
    }
}

/// The same carry for the other two arenas, which the surface row
/// above cannot see: a graft that carried surfaces and dropped points
/// and curves would leave it green.
#[test]
fn a_graft_carries_the_point_and_curve_origins_too() {
    let tol = Tol::witness();
    let mut dst = unit_brick();
    let native_p: Vec<_> = dst.points().map(|(k, _)| k).collect();
    let native_c: Vec<_> = dst.curves().map(|(k, _)| k).collect();
    let mut src = unit_brick();
    src.mark_imported();
    let src = transform_rigid(&src, &aside(), tol).unwrap();
    graft_disjoint(&mut dst, &src, tol).unwrap();

    let mut grafted_p = 0usize;
    for (k, _) in dst.points() {
        if !native_p.contains(&k) {
            assert_eq!(dst.point_origin(k), Some(&GeomOrigin::Imported), "{k:?}");
            grafted_p += 1;
        }
    }
    let mut grafted_c = 0usize;
    for (k, _) in dst.curves() {
        if !native_c.contains(&k) {
            assert_eq!(dst.curve_origin(k), Some(&GeomOrigin::Imported), "{k:?}");
            grafted_c += 1;
        }
    }
    assert_eq!(grafted_p, native_p.len());
    assert_eq!(grafted_c, native_c.len());
    for k in native_p {
        assert_eq!(dst.point_origin(k), Some(&GeomOrigin::KernelDirect));
    }
    for k in native_c {
        assert_eq!(dst.curve_origin(k), Some(&GeomOrigin::KernelDirect));
    }
}
