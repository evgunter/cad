//! **A cone face bounded by one rim and nothing else is the apex cap**
//! (`work/props/cone-apex-cap-refuses-degenerateface.md`).
//!
//! The flux lane reads the boundary's vector area, which is the cap's
//! whatever the boundary's shape. The shape door's iso parse has an
//! extent to find: its levels hold one signed slant, and the missing
//! extreme is the APEX — level `0`, the only candidate a cone has,
//! where the sphere's rim-only cap chooses between two poles and needs
//! the face's sense bit to do it. So the fold reads no bit, and these
//! rows pin what both must and must not admit:
//!
//! * the cap measures `sin α·Δu·|v_hi² − v_lo²|/2` on both nappes and
//!   both traversals;
//! * a rim that does not CLOSE around the apex bounds no cap — at the
//!   flux lane a loop that does not close (`ChartLoopOpen`) or winds
//!   the axis twice (`LoopsWindChart`), at the shape
//!   door `props_rim_only_closed` and `props_rim_only_join`, the
//!   sphere's guard at the cone's own azimuthal arm;
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
use geom_brep::props::{
    LoopEdge, MaterialSign, PropsError, boundary_material_sign, curved_face, require_iso_rectangle,
};

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
/// at one level, and each arc's vector area is its share.
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
/// Unguarded, the four shapes the sphere's arm was caught on answer a
/// definite AREA here too — half a rim at half the cap's, a quarter at
/// a quarter, the same full rim stated twice at double, and a full rim
/// plus a half arc at 1.5×. The flux lane refuses a loop whose joints
/// do not close ([`PropsError::ChartLoopOpen`]) and one that winds the
/// axis twice ([`PropsError::LoopsWindChart`]); the shape door's fold decides
/// `(Δu − τ)` at the cone's own azimuthal arm (`props_rim_only_closed`).
/// Each shape refuses at both, by name.
#[test]
fn an_apex_cap_refuses_a_rim_that_does_not_close() {
    let cap = closed_form(0.0, SL, TAU);
    let mut answered = Vec::new();
    for (name, edges, flux_guard) in [
        (
            "half a rim only (du = pi)",
            vec![rim(SL, 0.0, PI, 0, 1)],
            PropsError::ChartLoopOpen,
        ),
        (
            "quarter rim only",
            vec![rim(SL, 0.0, PI / 2.0, 0, 1)],
            PropsError::ChartLoopOpen,
        ),
        (
            "the same full rim stated twice",
            vec![rim(SL, 0.0, TAU, 0, 0), rim(SL, 0.0, TAU, 0, 0)],
            PropsError::LoopsWindChart,
        ),
        (
            "full rim + an extra half arc, same direction",
            vec![rim(SL, 0.0, TAU, 0, 0), rim(SL, 0.0, PI, 0, 1)],
            PropsError::ChartLoopOpen,
        ),
    ] {
        assert_eq!(
            require_iso_rectangle(&cone(), &edges, band()),
            Err(PropsError::NotIsoRectangle {
                what: "props_rim_only_closed"
            }),
            "{name}: the shape door's closure guard is what must refuse it"
        );
        match curved_face(&cone(), &edges, true, band()) {
            Ok(fc) => {
                println!(
                    "  {name:<44} ACCEPT area={:.15e} (whole cap {cap:.15e}, ratio {:.4})",
                    fc.area,
                    fc.area / cap
                );
                answered.push(name);
            }
            Err(e) => {
                println!("  {name:<44} REFUSE {e:?}");
                assert_eq!(
                    e, flux_guard,
                    "{name}: the flux lane's closure guard is what must refuse it"
                );
            }
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

/// **Zero-length generators change nothing.** Two half rims at one
/// level, run the same way and joined by generators of no length, are
/// the full rim: the cap. The flux lane reads the vector area, to which
/// a generator of no length contributes nothing; run opposite ways the
/// same edges bound nothing and stay degenerate.
#[test]
fn zero_length_generators_between_rims_at_one_level_change_nothing() {
    measures(
        "two half rims joined by zero-length generators",
        &[
            rim(SL, 0.0, PI, 0, 1),
            generator(PI, SL, SL, 1, 2),
            rim(SL, PI, TAU, 2, 3),
            generator(0.0, SL, SL, 3, 0),
        ],
        closed_form(0.0, SL, TAU),
    );
    let got = curved_face(
        &cone(),
        &[
            rim(SL, 0.0, PI, 0, 1),
            generator(PI, SL, SL, 1, 2),
            rim(SL, PI, 0.0, 2, 3),
            generator(0.0, SL, SL, 3, 0),
        ],
        true,
        band(),
    );
    assert!(
        matches!(got, Err(PropsError::DegenerateFace)),
        "opposite traversals bound nothing; got {got:?}"
    );
}

/// **A frustum band measures with its seam, and two rims in one loop
/// are no loop.** The seam generator closes the walk; without it the
/// two circles do not chain, and the vector area of edges that do not
/// chain is no region's ([`PropsError::ChartLoopOpen`]).
#[test]
fn a_frustum_band_measures_with_its_seam() {
    let (v0, v1) = (SL, 3.0 * SL);
    measures(
        "frustum band with its seam",
        &[
            rim(v0, 0.0, TAU, 0, 0),
            generator(0.0, v0, v1, 0, 1),
            rim(v1, TAU, 0.0, 1, 1),
            generator(0.0, v1, v0, 1, 0),
        ],
        closed_form(v0, v1, TAU),
    );
    assert_eq!(
        curved_face(
            &cone(),
            &[rim(v0, 0.0, TAU, 0, 0), rim(v1, TAU, 0.0, 1, 1)],
            true,
            band()
        )
        .map(|_| ()),
        Err(PropsError::ChartLoopOpen),
        "two rims in one loop do not chain"
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

// ---------------------------------------------------------------------
// The recorded population
// ---------------------------------------------------------------------

/// The `(predicate, sign)` multiset one `curved_face` call records.
fn verdict_multiset(edges: &[LoopEdge<f64>]) -> Vec<(String, usize)> {
    let bracket = geom_core::k_stats::Bracket::open();
    let _ = curved_face(&cone(), edges, true, band());
    let log = bracket.finish();
    let mut got: Vec<(String, usize)> = Vec::new();
    for v in &log.verdicts {
        let key = format!("{} {:?}", v.predicate, v.sign);
        match got.iter_mut().find(|(k, _)| *k == key) {
            Some((_, n)) => *n += 1,
            None => got.push((key, 1)),
        }
    }
    got.sort();
    got
}

/// **The apex cap's own verdicts at the flux lane, named.** The rim's
/// incidence on the cone (`props_circle_axis_class`, `props_rim_fit`,
/// `props_cone_circle_tilt`, `props_cone_circle_off_axis`); the nappe's
/// two ends (`props_cone_nappe`); the side (`props_cone_area_side`); the
/// winding's own reading — the rim's two ends off the apex
/// (`props_cone_loop_off_apex`) and its sense (`props_cone_arc_sense`; a
/// rim turns through its stored span, so no turn is decided); and the
/// periodic chart's closure: one junction (`props_chart_junction_closed`),
/// the loop's turns at the half and three-half turn thresholds and their
/// sense (`props_chart_loop_turns`, `props_chart_loop_sense`), and its
/// whole-turn residual (`props_chart_loop_whole`). The rim's span is
/// decided once, under certification's span bound, a full period
/// sitting AT it (`props_rim_span_forward` Positive,
/// `props_rim_span_winding` Zero). One edge has no two non-adjacent
/// vertices, and no iso decide is asked.
#[test]
fn the_apex_cap_records_its_named_decides() {
    let got = verdict_multiset(&[rim(SL, 0.0, TAU, 0, 0)]);
    let want: Vec<(String, usize)> = [
        ("props_chart_junction_closed Zero", 1),
        ("props_chart_loop_sense Positive", 1),
        ("props_chart_loop_turns Negative", 1),
        ("props_chart_loop_turns Positive", 1),
        ("props_chart_loop_whole Zero", 1),
        ("props_circle_axis_class Positive", 1),
        ("props_cone_arc_sense Positive", 1),
        ("props_cone_area_side Negative", 1),
        ("props_cone_circle_off_axis Zero", 1),
        ("props_cone_circle_tilt Zero", 1),
        ("props_cone_loop_off_apex Positive", 2),
        ("props_cone_nappe Positive", 2),
        ("props_rim_fit Zero", 1),
        ("props_rim_span_forward Positive", 1),
        ("props_rim_span_winding Zero", 1),
    ]
    .into_iter()
    .map(|(k, n)| (k.to_string(), n))
    .collect();
    assert_eq!(got, want, "the apex cap's recorded population");
}

// ---------------------------------------------------------------------
// A sum is not a cover
// ---------------------------------------------------------------------

/// **Red-first, adopted from the R2 review probe
/// (`r2_the_closure_guard_is_a_sum_not_a_cover`).** `Δu` is a SUM of
/// the group's spans, so a guard that only compares the sum to a turn
/// admits any multiset of same-direction arcs totalling one: the same
/// HALF rim stated twice covers half the circle and answered the whole
/// cap's area, and two overlapping arcs totalling a turn covered three
/// quarters and answered the same. Both are the sphere unit's own
/// MAJOR — an area answered for a rim that does not close — reached
/// past the sum. The shape door's `props_rim_only_join` and the flux
/// lane's junction closure (`props_chart_junction_closed`) each require
/// the arcs to TILE: each
/// edge's traversal end is the next one's traversal start, cyclically.
#[test]
fn a_sum_that_makes_a_turn_is_not_a_cover() {
    let cap = closed_form(0.0, SL, TAU);
    let mut answered = Vec::new();
    for (name, edges) in [
        (
            "the same HALF rim stated twice (du = tau)",
            vec![rim(SL, 0.0, PI, 0, 1), rim(SL, 0.0, PI, 0, 1)],
        ),
        (
            "two overlapping arcs, spans summing to tau",
            vec![
                rim(SL, 0.0, 0.75 * TAU, 0, 1),
                rim(SL, 0.5 * TAU, 0.75 * TAU, 1, 2),
            ],
        ),
    ] {
        assert_eq!(
            require_iso_rectangle(&cone(), &edges, band()),
            Err(PropsError::NotIsoRectangle {
                what: "props_rim_only_join"
            }),
            "{name}: the tiling half of the shape door's guard is what must refuse it"
        );
        match curved_face(&cone(), &edges, true, band()) {
            Ok(fc) => {
                println!(
                    "  {name:<44} ACCEPT area={:.15e} ratio={:.4}",
                    fc.area,
                    fc.area / cap
                );
                answered.push(name);
            }
            Err(PropsError::ChartLoopOpen) => {
                println!("  {name:<44} REFUSE ChartLoopOpen");
            }
            Err(e) => panic!("{name}: refused by something else: {e:?}"),
        }
    }
    assert!(
        answered.is_empty(),
        "arcs that total a turn without tiling bound no cap, and these answered an area: \
         {answered:?}"
    );
}

/// **The property, generated: an apex cap is a TILING of the rim.**
///
/// For each `n` the covering boundary is the `n` arcs
/// `[k·τ/n, (k+1)·τ/n]` in loop order, and it must measure the whole
/// cap. Four families derived from those same arcs must not measure at
/// all — one short (sum under), one repeated (sum over), `n` copies of
/// one arc (sum exactly `τ`, covering `1/n` of the circle) and the
/// tiling with two adjacent arcs exchanged (sum exactly `τ`, chain
/// broken). The last two are the ones a sum cannot see.
///
/// **A cyclic ROTATION is not in the non-covering list, deliberately**:
/// it is the same loop anchored elsewhere and it is asserted to
/// measure, because a verdict that moved under re-anchoring would be a
/// fact about the flattening rather than about the face.
#[test]
fn an_apex_cap_is_a_tiling_and_the_families_that_are_not_refuse() {
    let cap = closed_form(0.0, SL, TAU);
    let tiling = |n: usize| -> Vec<LoopEdge<f64>> {
        (0..n)
            .map(|k| {
                let (a, b) = (k as f64 * TAU / n as f64, (k + 1) as f64 * TAU / n as f64);
                rim(SL, a, b, k as u32, ((k + 1) % n) as u32)
            })
            .collect()
    };
    let measures =
        |name: &str, edges: &[LoopEdge<f64>]| match curved_face(&cone(), edges, true, band()) {
            Ok(fc) => assert!(
                (fc.area - cap).abs() / cap < 1e-12,
                "{name}: area {:.15e} != the whole cap {cap:.15e}",
                fc.area
            ),
            Err(e) => panic!("{name}: a tiling of the rim bounds the cap, got {e:?}"),
        };
    let refuses = |name: &str, edges: &[LoopEdge<f64>]| {
        match require_iso_rectangle(&cone(), edges, band()) {
            Err(PropsError::NotIsoRectangle { what })
                if what == "props_rim_only_closed" || what == "props_rim_only_join" =>
            {
                println!("  {name:<40} door REFUSE {what}")
            }
            other => panic!("{name}: the shape door finds no cap in it: {other:?}"),
        }
        match curved_face(&cone(), edges, true, band()) {
            Err(e @ (PropsError::ChartLoopOpen | PropsError::LoopsWindChart)) => {
                println!("  {name:<40} flux REFUSE {e:?}")
            }
            other => panic!("{name}: a boundary that does not tile bounds no cap: {other:?}"),
        }
    };

    for n in 1..=6usize {
        let full = tiling(n);
        measures(&format!("tiling of {n}"), &full);
        // Every rotation of the same cycle is the same face.
        for r in 0..n {
            let rotated: Vec<LoopEdge<f64>> = (0..n).map(|i| full[(i + r) % n].clone()).collect();
            measures(&format!("tiling of {n}, anchored at {r}"), &rotated);
        }
        if n >= 2 {
            refuses(&format!("{n} arcs less one"), &full[..n - 1]);
            let mut over = full.clone();
            over.push(full[0].clone());
            refuses(&format!("{n} arcs plus a repeat"), &over);
            let same: Vec<LoopEdge<f64>> = (0..n).map(|_| full[0].clone()).collect();
            refuses(&format!("{n} copies of one arc (sum = tau)"), &same);
        }
        if n >= 3 {
            let mut swapped = full.clone();
            swapped.swap(0, 1);
            refuses(&format!("tiling of {n} with two arcs exchanged"), &swapped);
        }
    }
}

// ---------------------------------------------------------------------
// A cone that is not the fixture
// ---------------------------------------------------------------------

/// A cone with an arbitrary apex, axis and half-angle — the fixture
/// the rows below vary, where every other row in this file holds it
/// fixed.
#[derive(Clone, Copy)]
struct GCone {
    apex: geom_core::Point3<f64>,
    axis: geom_core::Vec3<f64>,
    alpha: f64,
}

impl GCone {
    fn surface(self) -> Surface<f64> {
        Surface::Cone {
            apex: self.apex,
            axis: self.axis,
            half_angle: self.alpha,
            u_ref: if self.axis.x.abs() < 0.9 {
                v3(1.0, 0.0, 0.0)
            } else {
                v3(0.0, 1.0, 0.0)
            },
        }
    }

    /// The rim at signed slant `v`, traversed `u0 -> u1`.
    fn rim(self, v: f64, u0: f64, u1: f64, a: u32, b: u32) -> LoopEdge<f64> {
        let (s, c) = self.alpha.sin_cos();
        let n = self.axis;
        topo::edge(
            Curve3::Circle {
                center: self.apex + n * (v * c),
                axis: n,
                radius: v.abs() * s,
                u_ref: if n.x.abs() < 0.9 {
                    v3(1.0, 0.0, 0.0).cross(n).normalize()
                } else {
                    v3(0.0, 1.0, 0.0).cross(n).normalize()
                },
            },
            u0,
            u1,
            a,
            b,
        )
    }
}

/// **The closed form on cones that are NOT the fixture** — adopted
/// from the R2 review probe (`r2_the_apex_cap_measures_on_a_general_cone`).
///
/// Every other row in this file sits at `α = 45°`, apex at the origin,
/// axis `+Z`: there `sin α = cos α`, so a swap of the two is
/// invisible, and the anchored term is identically zero, so no flux
/// assertion can fail. These 54 rows vary the half-angle (shallow,
/// steep), the apex, the axis, the nappe and the traversal, and assert
/// `sin α·Δu·|v_hi² − v_lo²|/2` and the anchored flux `apex·n̂·πr²`
/// derived here rather than read from the fold — including that the
/// two traversals anchor OPPOSITE flux, which is the sign the fixture
/// rows cannot see.
///
/// **The smallest slant comes from the run's own band, not from a
/// literal.** Adopting a probe puts it on CI's ε matrix, where the
/// probe itself ran at one ε: the rim radius `|v|·sin α` a literal
/// states is decidable at one ε and inside the ambiguity band at
/// another, and `props_circle_axis_class` is metered at exactly that
/// radius.
#[test]
fn the_apex_cap_measures_on_a_general_cone() {
    for alpha in [0.05_f64, PI / 6.0, 1.4] {
        for apex in [p3(0.0, 0.0, 0.0), p3(0.02, -0.03, 0.05)] {
            for axis in [v3(0.0, 0.0, 1.0), v3(0.0, 0.0, -1.0), v3(1.0, 2.0, 3.0)] {
                let n = axis.normalize();
                let cone = GCone {
                    apex,
                    axis: n,
                    alpha,
                };
                // The SMALLEST cap this row can state at the run's ε,
                // from the run's own band rather than a literal: the
                // rim's own radius `|v|·sin α` is the lever
                // `props_circle_axis_class` is metered at, so a rim
                // narrower than the escalate band is undecidable by
                // construction and the fold never gets to answer. A
                // literal `1e-4` here passed at the default ε and
                // escalated at 1e-6, where a 5 µm rim sits inside a
                // 10 µm band.
                let tiny = 1e3 * band().escalate() / alpha.sin();
                for v in [0.010_f64, -0.010, tiny] {
                    let surf = cone.surface();
                    let (s, _c) = alpha.sin_cos();
                    let exact = s * TAU * (v * v).abs() * 0.5;
                    let r = v.abs() * s;
                    let anchored = (apex - geom_core::Point3::origin()).dot(n) * PI * r * r;
                    let plus = curved_face(&surf, &[cone.rim(v, 0.0, TAU, 0, 0)], true, band());
                    let minus = curved_face(&surf, &[cone.rim(v, TAU, 0.0, 0, 0)], true, band());
                    let (fp, fm) = match (plus, minus) {
                        (Ok(a), Ok(b)) => (a, b),
                        other => panic!("alpha={alpha} apex={apex:?} axis={n:?} v={v}: {other:?}"),
                    };
                    // The FIXTURE's own conditioning, not a fudge. The
                    // kernel recovers `v` from `(center − apex)·axis`,
                    // and this row deliberately places the apex far
                    // from the origin relative to the cap's own size:
                    // that subtraction cancels `|apex| / (|v|·cos α)`
                    // of its digits and the area squares what is left.
                    // Where the apex IS the origin the factor is 1 and
                    // the threshold is the flat `1e-12` this row was
                    // written with.
                    let cancel =
                        1.0 + (apex - geom_core::Point3::origin()).norm() / (v.abs() * alpha.cos());
                    for (tag, fc) in [("+u", &fp), ("-u", &fm)] {
                        let rel = (fc.area - exact).abs() / exact;
                        assert!(
                            rel < 1e-12 * cancel,
                            "{tag} alpha={alpha} axis={n:?} v={v}: area {:.17e} != {exact:.17e} \
                             (rel {rel:.3e}, conditioning {cancel:.3e})",
                            fc.area
                        );
                    }
                    assert!(
                        (fp.flux + fm.flux).abs() <= 1e-12 * (fp.flux.abs() + 1e-30),
                        "the two traversals must anchor opposite flux: {} {}",
                        fp.flux,
                        fm.flux
                    );
                    let (got, want) = (fp.flux.abs(), anchored.abs());
                    assert!(
                        (got - want).abs() <= 1e-10 * want.max(1e-30),
                        "flux magnitude {got:.17e} != |apex·n̂|·πr² {want:.17e}"
                    );
                }
            }
        }
    }
}

// ---------------------------------------------------------------------
// One extent, three doors
// ---------------------------------------------------------------------

/// **A boundary that does not close is refused at every door.** The
/// shape door's apex fold refuses by its own guard. The material-side
/// gate and the flux lane read the boundary's vector area, which is the
/// region's only for a closed loop, and refuse on the closure decide
/// they share. Left unchecked, a half rim answered a definite `Encoded`
/// side at the gate for a boundary `curved_face` refused — an unpaired
/// derivation, which is what raises `CurvedSenseInverted` on a face
/// whose honest answer is a refusal.
#[test]
fn a_boundary_that_does_not_close_refuses_at_every_door() {
    for (name, edges) in [
        ("a half rim only", vec![rim(SL, 0.0, PI, 0, 1)]),
        (
            "the same half rim twice",
            vec![rim(SL, 0.0, PI, 0, 1), rim(SL, 0.0, PI, 0, 1)],
        ),
    ] {
        let door = require_iso_rectangle(&cone(), &edges, band());
        let gate = boundary_material_sign(&cone(), &edges, band()).map(|_| ());
        let flux = curved_face(&cone(), &edges, true, band()).map(|_| ());
        println!("  {name:<24} door={door:?} gate={gate:?} flux={flux:?}");
        assert!(
            matches!(
                door,
                Err(PropsError::NotIsoRectangle {
                    what: "props_rim_only_closed" | "props_rim_only_join"
                })
            ),
            "{name}: the door must refuse by the fold's guard, got {door:?}"
        );
        for (which, got) in [("gate", &gate), ("flux", &flux)] {
            assert_eq!(
                *got,
                Err(PropsError::ChartLoopOpen),
                "{name}: the {which} refuses the open loop"
            );
        }
    }
    // The floor, unmoved: the true zero-extent patch is not folded at
    // all (its rims run opposite ways), so the door still admits it —
    // extent is not a shape question — and the flux lane still refuses
    // `DegenerateFace`.
    let zero_extent = vec![rim(SL, 0.0, PI, 0, 1), rim(SL, PI, 0.0, 1, 0)];
    assert_eq!(require_iso_rectangle(&cone(), &zero_extent, band()), Ok(()));
    assert!(matches!(
        curved_face(&cone(), &zero_extent, true, band()),
        Err(PropsError::DegenerateFace)
    ));
}

/// **The cone gate's side is a face fact under a re-anchoring**
/// (`rim-side-and-rim-dir-group-signs-are-facts-about-cycle-order`, closed by PR 4573).
/// The gate reads the sign of the face's area against the chart normal
/// off its axial vector area (`props_cone_area_side`), the quantity the
/// flux integrates, so no rim is read first: a frustum patch — two rims
/// and two generators — answers one side at every rotation of its edge
/// list, on both nappes and both traversals, and the population it
/// records is the same at every anchor and holds no `props_rim_side`.
#[test]
fn the_cone_gate_reads_one_side_at_every_anchor() {
    let verdicts = |edges: &[LoopEdge<f64>]| {
        let bracket = geom_core::k_stats::Bracket::open();
        let side = boundary_material_sign(&cone(), edges, band());
        let mut got: Vec<String> = bracket
            .finish()
            .verdicts
            .iter()
            .map(|v| format!("{} {:?}", v.predicate, v.sign))
            .collect();
        got.sort();
        (side, got)
    };
    for nappe in [1.0, -1.0] {
        let (lo, hi) = (nappe * SL, nappe * 2.0 * SL);
        // Along the rim at `lo`, out along the generator at `u = 1`,
        // back along the rim at `hi`, in along the generator at `u = 0`:
        // counter-clockwise in the chart `(u, v)` where `hi > lo`, the
        // `v > 0` nappe, and clockwise on the mirrored `v < 0` one. There
        // a generator's points lie at azimuth `u + π`, where its rims'
        // arcs are read from.
        let du = if nappe > 0.0 { 0.0 } else { PI };
        let patch = [
            rim(lo, du, du + 1.0, 0, 1),
            generator(1.0, lo, hi, 1, 2),
            rim(hi, du + 1.0, du, 2, 3),
            generator(0.0, hi, lo, 3, 0),
        ];
        let want = if nappe > 0.0 {
            geom_core::Sign::Positive
        } else {
            geom_core::Sign::Negative
        };
        let rotated = |k: usize, reversed: bool| -> Vec<LoopEdge<f64>> {
            let mut e: Vec<LoopEdge<f64>> = (0..patch.len())
                .map(|i| patch[(i + k) % patch.len()].clone())
                .collect();
            if reversed {
                e = e
                    .into_iter()
                    .rev()
                    .map(|x| LoopEdge {
                        forward: !x.forward,
                        start: x.end,
                        end: x.start,
                        ..x
                    })
                    .collect();
            }
            e
        };
        for reversed in [false, true] {
            let want = if reversed { want.flip() } else { want };
            let (_, first) = verdicts(&rotated(0, reversed));
            assert!(
                first.iter().all(|v| !v.starts_with("props_rim_side ")),
                "the gate reads no rim side: {first:?}"
            );
            for k in 0..patch.len() {
                let (side, got) = verdicts(&rotated(k, reversed));
                assert_eq!(
                    side,
                    Ok(MaterialSign::Encoded(want)),
                    "nappe {nappe}, reversed {reversed}, k {k}"
                );
                assert_eq!(got, first, "nappe {nappe}, reversed {reversed}, k {k}");
            }
        }
    }
}
