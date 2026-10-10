//! LIB-U3 acceptance rows: the profile-vocabulary loft/sweep doors.
//!
//! Two things are pinned here that the migrated corpus cannot pin:
//!
//! 1. **An exactly-tangent section joint lofts.** A section loop's
//!    tangent joints are derived at the profile door from its
//!    carriers (D1), so a joint no constructor made is a tangent joint
//!    decided from values, recorded rather than refused.
//!
//! 2. **The zero-diff contract as a regression test (U2 PR-2 review
//!    NOTE-3 rider).** The corpus `loft_prism` rebuilt from
//!    profile-vocabulary sections must reproduce the retired chain
//!    vocabulary's body BIT-FOR-BIT: volume bits and entity census
//!    pinned against constants recorded from the pre-U3 build at the
//!    merge base, so the "migration moved nothing" proof outlives the
//!    PR evidence.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::Point2;
use profile::{Profile, SketchPlane, test_support::bulge_loop};
use sweep::test_support::{loft_prism, loft_prism_sections, stacked_at};
use sweep::{Section, loft_body};

use geom_core::Tol;

/// A square with a semicircular bite: three lines and one bulge-1
/// (half-turn) arc from `(2, 0)` to `(2, 2)` whose carrier circle
/// (centre `(2, 1)`, radius 1) is EXACTLY tangent to the incoming
/// bottom edge at `(2, 0)` and to the outgoing top edge at `(2, 2)`
/// — two joints no constructor made, which the profile door decides
/// tangent from their carriers.
fn tangent_bite() -> Section {
    let v = |x: f64, y: f64, bulge: f64| (Point2::new(x, y), bulge);
    vec![bulge_loop(vec![
        v(0.0, 0.0, 0.0),
        v(2.0, 0.0, 1.0),
        v(2.0, 2.0, 0.0),
        v(0.0, 2.0, 0.0),
    ])]
}

/// The section's two tangent joints are decided from values and
/// recorded, and the section pair LOFTS. (Tier 3 volume stays refused
/// on the rational arc walls — the rational patch-flux lane's round
/// budget, not this unit's — so the pin is tiers 1/2 on the assembled
/// body.)
///
/// Red if the profile door refuses a tangency no constructor made (the
/// loft refuses `SkinError::SectionProfile`), or derives the joints
/// without recording them (no decided joint).
#[test]
fn an_exactly_tangent_section_loop_lofts_and_records_its_joints() {
    let section = Profile::new(SketchPlane::xy(), tangent_bite())
        .validate(Tol::witness())
        .expect("the tangent bite validates");
    let decided: Vec<usize> = section.loops()[0]
        .decided_joints()
        .iter()
        .map(|d| d.joint)
        .collect();
    assert_eq!(decided, vec![1, 2], "both bite joints recorded");
    let lofted = loft_body::<f64>(
        &[tangent_bite(), tangent_bite()],
        &stacked_at(&[0.0, 1.0]),
        1,
        Tol::witness(),
    )
    .expect("the tangent section pair lofts");
    assert_eq!(topo::validate(&lofted.body), Ok(()), "tier 1");
    assert_eq!(topo::validate_closed(&lofted.body), Ok(()), "tier 2");
}

/// NOTE-3 rider: the corpus `loft_prism` from profile-vocabulary
/// sections, its volume BITS and entity census pinned against the
/// merge-base (pre-U3, chain-vocabulary) build's recorded constants.
/// Any drift here is a geometry change the LIB-U3 contract forbids.
///
/// **The vocabulary is asserted, not assumed.** The body comes from
/// `sweep::test_support`, so the shape of its sections is one edit
/// away from this suite; the claim this row makes is about the
/// vocabulary they are spelled in, and a home that quietly moved to
/// another one would leave the bit pin green while the header above
/// went false. So the sections are read back first: three loops, four
/// straight segments each — the plainest
/// profile-vocabulary loop there is, which is what the pre-U3 chain
/// vocabulary could not express and what the recorded bits were taken
/// from.
#[test]
fn u3_differential_loft_prism_is_bit_identical_to_the_recorded_base() {
    /// `mass_properties(...).volume.to_bits()` of the chain-built
    /// loft_prism.
    ///
    /// **Re-pinned when the C9 ring became a newtype over the
    /// backend** (`0x4022_0000_0000_0004` before, recorded in the
    /// LIB-U3 PR): the ring padded one representable step outward on
    /// every operation and the backend pads only where the operation
    /// is inexact, so the prism's volume — exactly `9` in ℝ — lands on
    /// exactly `9`. The claim is unmoved; the number it reads is
    /// tighter.
    const BASE_VOLUME_BITS: u64 = 0x4022_0000_0000_0000;
    /// (solids, shells, faces, edges, vertices) of the same body.
    const BASE_CENSUS: (usize, usize, usize, usize, usize) = (1, 1, 6, 12, 8);

    let sections = loft_prism_sections();
    assert_eq!(sections.len(), 3, "the pinned body is a three-section loft");
    for (i, section) in sections.iter().enumerate() {
        assert_eq!(section.len(), 1, "section {i} is one closed loop");
        let lp = &section[0];
        assert_eq!(lp.vertices().len(), 4, "section {i} is a quad");
        assert!(
            lp.segments()
                .iter()
                .all(|s| matches!(s, profile::Segment::Line)),
            "section {i} carries an arc: the pinned bits are a POLYLINE loft's"
        );
    }

    let body = &loft_prism(Tol::witness());
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
