//! **A cone face bounded by one rim and nothing else is the apex cap**
//! (`work/props/cone-apex-cap-refuses-degenerateface.md`).
//!
//! Its levels hold one signed slant, so `min_max` gave `lo == hi` and
//! `require_extent` refused `DegenerateFace` for a face that plainly
//! has an area. The missing extreme is the APEX — level `0`, the only
//! candidate a cone has, where the sphere's rim-only cap chooses
//! between two poles and needs the face's sense bit to do it. So the
//! fold reads no bit, and these rows pin what it must and must not
//! admit:
//!
//! * the cap measures `sin α·Δu·|v_hi² − v_lo²|/2` on both nappes and
//!   both traversals;
//! * a rim that does not CLOSE around the apex bounds no cap —
//!   `props_rim_only_closed`, the sphere's guard at the cone's own
//!   azimuthal arm, red-first on a partial rim and a doubled one;
//! * rims at one level traversed OPPOSITE ways point at opposite sides
//!   and keep `DegenerateFace`;
//! * the traversal that bounds the rest of the nappe — unbounded, no
//!   finite face of any solid — is told apart from the cap by the
//!   material side the boundary encodes, which is tier 3's check 6
//!   (executed on a body in `topo`'s `cone_apex_cap_body`).
//!
//! Areas are asserted against closed forms, never against a capture,
//! and every offset comes from the run's own `Band` — this file is on
//! CI's `eps ∈ {default, 1e-6, 1e-12}` matrix.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::shared::point::{p3, v3};
use crate::shared::tol::band;
use crate::shared::topo;
use geom::{Curve3, Surface};
use geom_brep::props::{LoopEdge, MaterialSign, PropsError, boundary_material_sign, curved_face};

/// The cone under every row: apex at the origin, axis `+Z`, half-angle
/// 45°, so `sin α = cos α = 1/√2`.
const SIN_A: f64 = core::f64::consts::FRAC_1_SQRT_2;
/// The rim's signed slant: 10 mm of slant length from the apex.
const SL: f64 = 0.010;
const PI: f64 = core::f64::consts::PI;
const TAU: f64 = core::f64::consts::TAU;

fn cone() -> Surface<f64> {
    Surface::Cone {
        apex: p3(0.0, 0.0, 0.0),
        axis: v3(0.0, 0.0, 1.0),
        half_angle: core::f64::consts::FRAC_PI_4,
        u_ref: v3(1.0, 0.0, 0.0),
    }
}

/// The rim at signed slant `v`, traversed in azimuth from `u0` to
/// `u1`: the coaxial circle of radius `|v|·sin α` at height
/// `v·cos α`.
fn rim(v: f64, u0: f64, u1: f64, a: u32, b: u32) -> LoopEdge<f64> {
    topo::edge(
        Curve3::Circle {
            center: p3(0.0, 0.0, v * SIN_A),
            axis: v3(0.0, 0.0, 1.0),
            radius: v.abs() * SIN_A,
            u_ref: v3(1.0, 0.0, 0.0),
        },
        u0,
        u1,
        a,
        b,
    )
}

/// The generator through the apex at azimuth `u`; its parameter is the
/// signed slant.
fn generator(u: f64, t0: f64, t1: f64, a: u32, b: u32) -> LoopEdge<f64> {
    topo::edge(
        Curve3::Line {
            origin: p3(0.0, 0.0, 0.0),
            dir: v3(u.cos() * SIN_A, u.sin() * SIN_A, SIN_A),
        },
        t0,
        t1,
        a,
        b,
    )
}

/// The closed form for a cone face spanning `[lo, hi]` in signed slant
/// over `Δu`: `sin α·Δu·|v_hi² − v_lo²|/2`.
fn closed_form(lo: f64, hi: f64, du: f64) -> f64 {
    SIN_A * du * (hi * hi - lo * lo).abs() * 0.5
}

/// The face measures `exact_area`, and its flux is exactly zero: the
/// apex is at the origin, so the anchored term `apex·A⃗` — the only
/// term a cone's flux has — vanishes identically. A nonzero flux here
/// would be a radial term the closed form has no business carrying.
fn measures(kind: &str, edges: &[LoopEdge<f64>], exact_area: f64) {
    match curved_face(&cone(), edges, true, band()) {
        Ok(fc) => {
            let rel = (fc.area - exact_area).abs() / exact_area;
            assert!(
                rel < 1e-12,
                "{kind}: area {:.15e} != exact {exact_area:.15e} (rel {rel:.3e})",
                fc.area
            );
            assert_eq!(
                fc.flux, 0.0,
                "{kind}: an apex at the origin anchors no flux, got {:.15e}",
                fc.flux
            );
        }
        Err(e) => panic!("{kind}: refused: {e:?}"),
    }
}

// ---------------------------------------------------------------------
// The fold
// ---------------------------------------------------------------------

/// **The apex cap measures the cone exactly, on both nappes and both
/// traversals.**
///
/// One rim circle and nothing else, the apex interior: `lo == hi`
/// before the fold, `DegenerateFace`. With the apex folded in, the
/// extent is `[0, v]` (or `[v, 0]` on the lower nappe) and the closed
/// form is the cone of slant length `|v|` — `π·v²·sin α` over a whole
/// turn. Both traversals answer it: a cone's flux needs no material
/// side, so the closed form is direction-blind, and which of the two
/// bounds a finite face is the ORIENTATION question check 6 asks
/// (`the_two_traversals_encode_opposite_material_sides`).
#[test]
fn the_apex_cap_measures_the_cone_on_both_nappes_and_traversals() {
    for v in [SL, 2.0 * SL, -SL] {
        let exact = closed_form(0.0, v, TAU);
        assert!(
            (exact - PI * v * v * SIN_A).abs() < 1e-18,
            "the closed form is the cone of slant |v|"
        );
        measures(
            &format!("apex cap at v={v}, +u"),
            &[rim(v, 0.0, TAU, 0, 0)],
            exact,
        );
        measures(
            &format!("apex cap at v={v}, -u"),
            &[rim(v, TAU, 0.0, 0, 0)],
            exact,
        );
    }
}

/// **The rim split into several arcs is the same face** — a
/// boolean-cut or re-merged rim legitimately arrives as several arcs
/// at one level, and `du_of_rims` sums their spans per group.
#[test]
fn an_apex_cap_whose_rim_is_three_arcs_measures_the_same() {
    let third = TAU / 3.0;
    measures(
        "apex cap whose rim is three arcs",
        &[
            rim(SL, 0.0, third, 0, 1),
            rim(SL, third, 2.0 * third, 1, 2),
            rim(SL, 2.0 * third, TAU, 2, 0),
        ],
        closed_form(0.0, SL, TAU),
    );
}

/// **The apex is interior to a rim only if the rim CLOSES around it.**
///
/// The fold pushes level `0` on a traversal DIRECTION's say-so, and
/// nothing else on the arm watches how far the rim goes: the levels
/// hold one slant however much of it the boundary states,
/// `props_rim_level` places a rim at an extreme whatever its span, and
/// `du_of_rims` sums a group's spans without comparing the sum to a
/// whole turn. Unguarded, the four shapes the sphere's arm was caught
/// on answer a definite AREA here too — half a rim at half the cap's,
/// a quarter at a quarter, the same full rim stated twice at double,
/// and a full rim plus a half arc at 1.5×. `props_rim_only_closed`
/// decides `(Δu − τ)` at the cone's own azimuthal arm (the rim's
/// radius), and each of them refuses by that name.
#[test]
fn an_apex_cap_refuses_a_rim_that_does_not_close() {
    let cap = closed_form(0.0, SL, TAU);
    let mut answered = Vec::new();
    for (name, edges) in [
        ("half a rim only (du = pi)", vec![rim(SL, 0.0, PI, 0, 1)]),
        ("quarter rim only", vec![rim(SL, 0.0, PI / 2.0, 0, 1)]),
        (
            "the same full rim stated twice",
            vec![rim(SL, 0.0, TAU, 0, 0), rim(SL, 0.0, TAU, 0, 0)],
        ),
        (
            "full rim + an extra half arc, same direction",
            vec![rim(SL, 0.0, TAU, 0, 0), rim(SL, 0.0, PI, 0, 1)],
        ),
    ] {
        match curved_face(&cone(), &edges, true, band()) {
            Ok(fc) => {
                println!(
                    "  {name:<44} ACCEPT area={:.15e} (whole cap {cap:.15e}, ratio {:.4})",
                    fc.area,
                    fc.area / cap
                );
                answered.push(name);
            }
            Err(PropsError::NotIsoRectangle { what }) => {
                println!("  {name:<44} REFUSE {what}");
                assert_eq!(
                    what, "props_rim_only_closed",
                    "{name}: the closure guard is what must refuse it"
                );
            }
            Err(e) => panic!("{name}: refused by something else: {e:?}"),
        }
    }
    assert!(
        answered.is_empty(),
        "a rim that does not close bounds no cap, and these answered an area: {answered:?}"
    );
    // The refusal above must not have cost the shape the fold is for.
    measures("the closed rim", &[rim(SL, 0.0, TAU, 0, 0)], cap);
}

/// **Rims at one level traversed OPPOSITE ways keep `DegenerateFace`**
/// — the row that tells the fix apart from "push the apex whenever the
/// extent collapses".
///
/// σ is `d_u_sign` under the face's one sense bit, so unanimity of σ is
/// unanimity of `d_u_sign` and the cone can require it without the bit.
/// Two half rims run opposite ways encode opposite sides: there is no
/// side the boundary agrees on, no apex the face contains, and the
/// levels stay silent exactly as before.
#[test]
fn cone_rims_at_one_level_with_opposite_traversals_stay_degenerate() {
    let got = curved_face(
        &cone(),
        &[rim(SL, 0.0, PI, 0, 1), rim(SL, PI, 0.0, 1, 0)],
        true,
        band(),
    );
    assert!(
        matches!(got, Err(PropsError::DegenerateFace)),
        "opposite traversals name no apex; got {got:?}"
    );
}

/// **A zero-extent cone face with GENERATORS is not folded either.**
/// Two rims at one level joined by zero-length generators states its
/// own `v`-domain — the generators are in the levels — so the fold does
/// not apply and `require_extent` keeps its verdict. The fold's
/// premise is *a boundary of rims alone*, and this is the row that
/// says the generator test is load-bearing rather than decorative.
#[test]
fn a_generator_bearing_zero_extent_cone_face_stays_degenerate() {
    let got = curved_face(
        &cone(),
        &[
            rim(SL, 0.0, PI, 0, 1),
            generator(PI, SL, SL, 1, 2),
            rim(SL, PI, TAU, 2, 3),
            generator(0.0, SL, SL, 3, 0),
        ],
        true,
        band(),
    );
    assert!(
        matches!(got, Err(PropsError::DegenerateFace)),
        "a generator-bearing boundary states its own extent; got {got:?}"
    );
}

/// **A frustum band is untouched by the fold** — two rims at DIFFERENT
/// levels carry their own extent, so nothing is pushed and the closed
/// form is the frustum's. The negative control for the fold's
/// coincidence test.
#[test]
fn a_two_rim_frustum_band_is_unchanged() {
    let (v0, v1) = (SL, 3.0 * SL);
    measures(
        "seamless frustum band",
        &[rim(v0, 0.0, TAU, 0, 0), rim(v1, TAU, 0.0, 1, 1)],
        closed_form(v0, v1, TAU),
    );
}

// ---------------------------------------------------------------------
// The orientation the flux lane cannot ask
// ---------------------------------------------------------------------

/// **The two traversals encode OPPOSITE material sides** — which is
/// how the unbounded complement is told from the cap.
///
/// The same rim traversed the other way bounds the rest of the nappe,
/// which runs to infinity and is no finite face of any solid; the flux
/// lane cannot tell them apart, because a cone's flux needs no
/// material side and `fn cone` takes no sense bit. The boundary DOES
/// encode one, and here — unlike on the sphere's rim-only cap, where
/// the extent the side would be read against is the very thing the
/// sense bit settles — the apex is the extreme whichever way the rim
/// runs, so the side is a side and the gate answers `Encoded`. Tier
/// 3's check 6 compares it with `Face::sense` and refuses the
/// disagreement.
#[test]
fn the_two_traversals_encode_opposite_material_sides() {
    let plus = boundary_material_sign(&cone(), &[rim(SL, 0.0, TAU, 0, 0)], band());
    let minus = boundary_material_sign(&cone(), &[rim(SL, TAU, 0.0, 0, 0)], band());
    println!("apex cap +u: {plus:?}   -u: {minus:?}");
    let (a, b) = match (plus, minus) {
        (Ok(MaterialSign::Encoded(a)), Ok(MaterialSign::Encoded(b))) => (a, b),
        other => panic!("both traversals must encode a definite side: {other:?}"),
    };
    assert_eq!(a, b.flip(), "the two traversals must disagree: {a:?} {b:?}");
}
