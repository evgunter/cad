//! **Reviewer probes for FILLET-H7** (the ruled band's transverse
//! cut-off), lane r2 on PR 1897. Three rows the shipped suite does not
//! carry:
//!
//! 1. **The prism factor `L` is measured.** `test_support::ROD_L` is
//!    `1.0`, so the shipped closed-form row compares `ΔV` against
//!    `A_section · 1.0` — numerically the same number as `A_section`,
//!    which cannot tell a prism from a per-crease constant. This row
//!    carves the same D-profile rod at a length that is NOT one and
//!    asserts `ΔV = 2 · A_section · L` there, so the length factor of
//!    C2's closed form is the thing under test.
//! 2. **Naming totality on the PLANAR open band.**
//!    `assert_naming_totality` was widened to the open bands' rows
//!    (`trims`, `arcs`, `feet`, `blends`) — rows the BLANK phase fills
//!    too — but every caller is a closed rim or the ruled band, so the
//!    widening's other half is unexercised. The die (twelve open
//!    plane–plane chains and eight corner patches) walks it.
//! 3. **Where the mutant arc is actually refused.** The shipped mutant
//!    row asserts `attach.is_err() || tier3.is_err()`, a disjunction
//!    that is green if only one arm fires; this row says which — the
//!    attachment gate refuses both mutants outright, so "red through
//!    tier 3" is a claim about the gate, not about the validator.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::Curve3;
use geom_brep::{EdgeCurveSpec, EdgeDescription, EdgeDescriptionSpec};
use geom_core::{Tol, Vec3};
use sweep::blend::fillet_edges;
use sweep::test_support::{
    ROD_FILLET, ROD_FLAT, ROD_R, assert_naming_totality, cube, rod_creases,
    rod_d_profile_of_length_at, rod_section_cut, rod_with_flat,
};
use topo::query;
use topo::{Body, mass_properties, validate_geometric};

const R: f64 = ROD_FILLET;

fn tol() -> Tol {
    Tol::witness()
}

fn census(body: &Body<f64>) -> (usize, usize, usize) {
    (
        body.vertices().count(),
        body.edges().count(),
        body.faces().count(),
    )
}

fn volume(body: &Body<f64>) -> f64 {
    let p = mass_properties(body, tol()).expect("closed-form props");
    assert_eq!(p.volume_pad, 0.0, "the inventory is closed-form");
    p.volume
}

/// The D-profile rod of [`ROD_R`] and [`ROD_FLAT`] at an ARBITRARY
/// length. `test_support::rod_d_profile_at` is the same body pinned at
/// `ROD_L = 1.0`; this row's claim is about the length factor, so it
/// takes the length through that fixture's one home,
/// `rod_d_profile_of_length_at`.
fn d_profile_rod_of_length(length: f64, tol: Tol) -> Body<f64> {
    rod_d_profile_of_length_at(length, tol)
}

/// **The prism closed form scales with the rod's LENGTH.** At
/// `ROD_L = 1.0` the shipped row's `A_section · L` is the same float as
/// `A_section`, so the `· L` is unmeasured. Here `L = 2.5`: both
/// creases carve, the census delta is the same `+4/+6/+2` (the walk is
/// per crease, not per unit length), and the volume removed is
/// `2 · A_section · 2.5`. A carve whose band did not run the whole
/// length of the rod, or whose closed form dropped `L`, goes red.
#[test]
fn r2_the_prism_closed_form_scales_with_the_rod_length() {
    const L: f64 = 2.5;
    let source = d_profile_rod_of_length(L, tol());
    let creases = rod_creases(&source);
    assert_eq!(creases.len(), 2, "two ruling creases at any length");
    let (v0, e0, f0) = census(&source);
    let vol0 = volume(&source);

    let out = fillet_edges(&source, &creases, R, tol())
        .unwrap_or_else(|e| panic!("the long rod's creases carve, got {e}"));
    assert_eq!(
        census(&out.body),
        (v0 + 4, e0 + 6, f0 + 2),
        "the census delta is per crease, not per length"
    );
    validate_geometric(&out.body, tol()).expect("tier 3");

    let cut = 2.0 * rod_section_cut(ROD_R, ROD_FLAT, R) * L;
    let got = vol0 - volume(&out.body);
    assert!(
        (got - cut).abs() < 1e-12,
        "ΔV = 2·A_section·L at L = {L}: measured {got} vs closed form {cut}"
    );
    // And the length factor is load-bearing: the same closed form read
    // at unit length is a DIFFERENT number, so this row could not pass
    // with `L` dropped.
    let unit = 2.0 * rod_section_cut(ROD_R, ROD_FLAT, R);
    assert!(
        (got - unit).abs() > 1e-6,
        "the L factor must matter: {got} vs the unit-length form {unit}"
    );
    assert_naming_totality(&source, &out, &creases, "the long rod");
}

/// **Naming totality holds on the PLANAR open band too.** The walk was
/// widened to `trims` / `arcs` / `feet` / `blends` for the ruled band;
/// the blank phase fills the same four rows for every open plane–plane
/// link and its corner patches, and no row walked it there. The die —
/// twelve open chains, eight uniform-convex trihedra, struts minted and
/// retired — is that fixture.
#[test]
fn r2_naming_is_total_on_the_planar_open_band_and_its_corners() {
    let source = cube(1.0, tol());
    let edges = query::all_edges(&source);
    let out = fillet_edges(&source, &edges, 0.15, tol()).expect("the die carves");
    assert_eq!(out.blend_faces.len(), 12, "one band per box edge");
    assert_eq!(out.corner_faces.len(), 8, "one patch per corner");
    assert_naming_totality(&source, &out, &edges, "the die");
}

/// **The mutant cut-off arc is refused by the ATTACHMENT gate**, not by
/// the validator afterwards. The shipped row accepts either, which is
/// green whenever only one of them fires; this row records which, so a
/// change that moved the refusal from the gate to tier 3 (or lost the
/// gate entirely and leaned on tier 3) is visible instead of absorbed.
#[test]
fn r2_the_mutant_cut_off_arc_is_refused_at_the_attachment_gate() {
    let source = rod_with_flat(tol());
    let creases = rod_creases(&source);
    let out = fillet_edges(&source, &creases, R, tol()).expect("the rod carves");
    let rec = out.naming.as_ref().expect("birth records");
    let (arc, _, _) = rec.arcs[0];
    let c = out
        .body
        .get_curve_geom(out.body.get_edge(arc).unwrap().curve)
        .and_then(|g| g.certified())
        .unwrap()
        .clone();
    let Curve3::Circle {
        center,
        axis,
        radius,
        u_ref,
    } = *c.carrier()
    else {
        panic!("a cut-off arc is a circle");
    };
    let EdgeDescription::Intersection { s1, s2, witness } = c.description() else {
        panic!("a cut-off arc is a transverse intersection");
    };
    let (s1, s2, witness) = (*s1, *s2, *witness);
    let (t0, t1) = c.params();
    for (label, carrier) in [
        (
            "wrong radius",
            Curve3::Circle {
                center,
                axis,
                radius: radius * 1.05,
                u_ref,
            },
        ),
        (
            "wrong centre",
            Curve3::Circle {
                center: center + Vec3::new(0.01, 0.0, 0.0),
                axis,
                radius,
                u_ref,
            },
        ),
    ] {
        let mut body = out.body.clone();
        let attached = body.set_edge_curve(
            arc,
            EdgeCurveSpec {
                description: EdgeDescriptionSpec::Intersection { s1, s2, witness },
                carrier,
                param_start: t0,
                param_end: t1,
            },
            tol(),
        );
        assert!(
            attached.is_err(),
            "{label}: the attachment gate refuses the mutant arc outright"
        );
        // And the body the gate protected is still tier-3 clean, so the
        // refusal is the gate's and not a validator sweep afterwards.
        validate_geometric(&body, tol()).unwrap_or_else(|e| {
            panic!("{label}: the refused attachment left the body clean: {e:?}")
        });
    }
}
