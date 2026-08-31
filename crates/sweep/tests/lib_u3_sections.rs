//! LIB-U3 acceptance rows: the profile-vocabulary loft/sweep doors.
//!
//! Two things are pinned here that the migrated corpus cannot pin:
//!
//! 1. **The declared-tangency capability gain (spec §3 delta 2),
//!    explicit rather than smuggled.** The retired chain vocabulary
//!    dropped tangency declarations on the floor (`end_profile` built
//!    empty `tangent_joints`), so an exactly-tangent section joint
//!    could not loft at all — the profile door refused
//!    `UndeclaredTangency` and there was no way to declare. Native
//!    `ProfileLoop` sections declare; the declared loop builds, and
//!    the undeclared twin still refuses (the #101 discipline is
//!    unchanged — declarations are verified intent, not trust).
//!
//! 2. **The zero-diff contract as a regression test (U2 PR-2 review
//!    NOTE-3 rider).** The corpus `loft_prism` rebuilt from
//!    profile-vocabulary sections must reproduce the retired chain
//!    vocabulary's body BIT-FOR-BIT: volume bits and entity census
//!    pinned against constants recorded from the pre-U3 build at the
//!    merge base, so the "migration moved nothing" proof outlives the
//!    PR evidence.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Affine3, Point2, Vec3};
use profile::ProfileError;
use profile::RawLoop;
use sweep::skin::SkinError;
use sweep::{LoftError, ProfileLoop, ProfileVertex, Section, loft_body};

mod common;
use common::quad;
use geom_core::Tol;

/// A square with a semicircular bite: three lines and one bulge-1
/// (half-turn) arc from `(2, 0)` to `(2, 2)` whose carrier circle
/// (centre `(2, 1)`, radius 1) is EXACTLY tangent to the incoming
/// bottom edge at `(2, 0)` and to the outgoing top edge at `(2, 2)`
/// — two joints the profile door demands declarations for.
fn tangent_bite(declared: bool) -> Section {
    let v = |x: f64, y: f64, bulge: f64| ProfileVertex::new(Point2::new(x, y), bulge);
    let mut lp = ProfileLoop::new(vec![
        v(0.0, 0.0, 0.0),
        v(2.0, 0.0, 1.0),
        v(2.0, 2.0, 0.0),
        v(0.0, 2.0, 0.0),
    ]);
    if declared {
        lp = lp.with_tangent_joints(vec![1, 2]);
    }
    vec![lp]
}

fn at_z(zs: &[f64]) -> Vec<Affine3<f64>> {
    zs.iter()
        .map(|z| Affine3::translation(Vec3::new(0.0, 0.0, *z)))
        .collect()
}

/// Delta 2, the gain: a declared-tangent section loop LOFTS now.
/// (Tier 3 volume stays refused on the rational arc walls — the
/// rational patch-flux lane's round budget, not this unit's — so the
/// pin is tiers 1/2 on the assembled body.)
#[test]
fn declared_tangent_section_loops_now_loft() {
    let lofted = loft_body::<f64>(
        &[tangent_bite(true), tangent_bite(true)],
        &at_z(&[0.0, 1.0]),
        1,
        Tol::witness(),
    )
    .expect("the declared-tangent section pair lofts");
    assert_eq!(topo::validate(&lofted.body), Ok(()), "tier 1");
    assert_eq!(topo::validate_closed(&lofted.body), Ok(()), "tier 2");
}

/// Delta 2, the unchanged half: the SAME loop undeclared still
/// refuses `UndeclaredTangency` — now at the geometry door itself
/// (`SkinError::SectionProfile`), since every section passes the
/// profile gate at the door (LIB-U3: fail loud, all four body ops
/// through one vocabulary).
#[test]
fn undeclared_tangent_section_loops_still_refuse() {
    match loft_body::<f64>(
        &[tangent_bite(false), tangent_bite(false)],
        &at_z(&[0.0, 1.0]),
        1,
        Tol::witness(),
    ) {
        Err(LoftError::Skin(SkinError::SectionProfile {
            section: 0,
            source: ProfileError::UndeclaredTangency { .. },
        })) => {}
        other => panic!("undeclared exact tangency must still refuse, got {other:?}"),
    }
}

/// NOTE-3 rider: the corpus `loft_prism` from profile-vocabulary
/// sections, its volume BITS and entity census pinned against the
/// merge-base (pre-U3, chain-vocabulary) build's recorded constants.
/// Any drift here is a geometry change the LIB-U3 contract forbids.
#[test]
fn u3_differential_loft_prism_is_bit_identical_to_the_recorded_base() {
    /// `mass_properties(...).volume.to_bits()` of the chain-built
    /// loft_prism at the U3 merge base (recorded in the LIB-U3 PR).
    const BASE_VOLUME_BITS: u64 = 0x4022_0000_0000_0004;
    /// (solids, shells, faces, edges, vertices) of the same body.
    const BASE_CENSUS: (usize, usize, usize, usize, usize) = (1, 1, 6, 12, 8);

    let square = [(-1.0, -1.0), (1.0, -1.0), (1.0, 1.0), (-1.0, 1.0)];
    let trapezoid = [(-1.375, -1.0), (1.375, -1.0), (1.0, 1.0), (-1.0, 1.0)];
    let lofted = loft_body::<f64>(
        &[quad(square), quad(trapezoid), quad(square)],
        &at_z(&[0.0, 1.0, 2.0]),
        2,
        Tol::witness(),
    )
    .expect("the corpus loft_prism builds from profile-vocabulary sections");
    let body = &lofted.body;
    let m = topo::props::mass_properties(body, Tol::witness()).expect("mass properties");
    assert_eq!(
        m.volume.to_bits(),
        BASE_VOLUME_BITS,
        "volume moved: {} (bits {:#x}) vs the recorded base",
        m.volume,
        m.volume.to_bits()
    );
    let census = (
        body.solids().count(),
        body.shells().count(),
        body.faces().count(),
        body.edges().count(),
        body.vertices().count(),
    );
    assert_eq!(census, BASE_CENSUS, "entity census moved");
}
