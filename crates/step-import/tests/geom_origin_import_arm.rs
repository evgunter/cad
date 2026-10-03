//! **The origin channel's import arm.** An adopted body carries no
//! recipe `GeomSource` and never did — no `Node::Import` exists, so
//! `stamp_minted` never runs on one — and at the merge base that
//! absence was the same `None` a hand-built body and a
//! cleared-and-not-re-stamped one gave. `import_step` is the one door
//! in a position to say where these descriptions came from, and it
//! does (`topo::GeomOrigin::Imported`).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use common::import_body;
use topo::GeomOrigin;

/// Every description of an adopted body reads `Imported` — surfaces,
/// curves and points alike, and the `GeomSource` channel stays empty,
/// which is what keeps N6's rungs deciding exactly what they decided.
fn every_description_is_imported(name: &str) {
    let (body, _eps) = import_body(name);
    let mut n = 0usize;
    for (k, _) in body.surfaces() {
        assert_eq!(
            body.surface_origin(k),
            Some(&GeomOrigin::Imported),
            "{name}: surface {k:?}"
        );
        assert!(body.surface_source(k).is_none(), "{name}: surface {k:?}");
        n += 1;
    }
    for (k, _) in body.curves() {
        assert_eq!(
            body.curve_origin(k),
            Some(&GeomOrigin::Imported),
            "{name}: curve {k:?}"
        );
        n += 1;
    }
    for (k, _) in body.points() {
        assert_eq!(
            body.point_origin(k),
            Some(&GeomOrigin::Imported),
            "{name}: point {k:?}"
        );
        n += 1;
    }
    assert!(n > 0, "{name}: imported nothing to mark");
}

#[test]
fn a_single_solid_import_marks_every_description() {
    every_description_is_imported("cube");
}

/// The multi-instance path, which is the one that could have gone
/// wrong: each copy goes through `transform_rigid` — whose clear reads
/// `Cleared` on a body that HELD sources — and then through
/// `graft_disjoint` into the shipped arena. An adopted description
/// held no recipe source, so the clear dropped nothing and left no
/// trace; the mark goes on afterwards and covers every grafted key.
///
/// Both halves are pinned rather than narrated, because
/// `Body::mark_imported` marks only the `KernelDirect` arm: a clear
/// that left a trace here would ship `Cleared`, and a graft that
/// dropped a row would ship a live key with no row at all.
#[test]
fn a_placed_multi_instance_import_marks_every_description() {
    every_description_is_imported("kiss_assembly");
}
