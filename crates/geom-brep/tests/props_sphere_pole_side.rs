//! **A rim's traversal says which side of it the face's interior
//! lies on** (issues 1250 and 1598 — one gap, two faces of it).
//!
//! A sphere face's levels record the latitudes its boundary touches;
//! they never record which side of a latitude the material is on, and
//! the two faces a rim separates touch the same levels. That side is
//! σ — the rim's own `u`-traversal direction under the face's sense
//! bit — and these rows pin the two inputs the silence broke:
//!
//! * **The rim-only polar cap.** One rim circle, no meridian, the pole
//!   interior: the levels hold one latitude, so the extent collapsed
//!   and the face was refused `DegenerateFace`. The missing extreme is
//!   the pole σ points at, and the cap measures `2πR²(1 − sin v₀)`.
//! * **The L-shaped complement.** A half-cap and the rest of its
//!   sphere share both edges traversed opposite ways and parse to the
//!   same levels; the complement's rim sits at `lo` while its
//!   traversal says the interior lies BELOW it, so the rectangle
//!   premise does not hold and the face refuses
//!   `props_rim_interior_side`.
//!
//! Areas are asserted against closed forms, never against a capture,
//! and every offset comes from the run's own `Band` — this file is on
//! CI's `eps ∈ {default, 1e-6, 1e-12}` matrix.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::shared::point::{p3, v3};
use crate::shared::surf;
use crate::shared::tol::band;
use crate::shared::topo;
use geom::{Curve3, Surface};
use geom_brep::props::{LoopEdge, MaterialSign, PropsError, boundary_material_sign, curved_face};

/// The sphere under every row: R = 10 mm about +Z at the origin.
const RS: f64 = 0.010;
const PI: f64 = core::f64::consts::PI;
const TAU: f64 = core::f64::consts::TAU;

fn sphere() -> Surface<f64> {
    surf::sphere(RS)
}
/// The rim at latitude `v`, traversed in azimuth from `u0` to `u1`.
fn rim(v: f64, u0: f64, u1: f64, a: u32, b: u32) -> LoopEdge<f64> {
    topo::sphere_rim(RS, v, u0, u1, a, b)
}
/// The meridian great circle at azimuth `u`; its parameter IS the
/// latitude on the `u` side.
fn great(u: f64, t0: f64, t1: f64, a: u32, b: u32) -> LoopEdge<f64> {
    topo::sphere_great(RS, u, t0, t1, a, b)
}

/// The face measures `exact_area`, and its flux is the radial term
/// `s_f·R·Area` with `s_f = +1` — the sphere is centred at the origin,
/// so the anchored term is exactly zero and the flux states the
/// material side on its own.
fn measures_outward(kind: &str, edges: &[LoopEdge<f64>], exact_area: f64) {
    match curved_face(&sphere(), edges, true, band()) {
        Ok(fc) => {
            let rel = (fc.area - exact_area).abs() / exact_area;
            assert!(
                rel < 1e-12,
                "{kind}: area {:.15e} != exact {exact_area:.15e} (rel {rel:.3e})",
                fc.area
            );
            let want_flux = RS * exact_area;
            let rel_f = (fc.flux - want_flux).abs() / want_flux;
            assert!(
                rel_f < 1e-12,
                "{kind}: flux {:.15e} != s_f·R·Area {want_flux:.15e} (rel {rel_f:.3e})",
                fc.flux
            );
        }
        Err(e) => panic!("{kind}: refused: {e:?}"),
    }
}

// ---------------------------------------------------------------------
// Issue 1250 — the rim-only polar cap
// ---------------------------------------------------------------------

/// **A cap bounded by ONE rim circle and nothing else measures the
/// spherical cap exactly, on both poles and both traversals.**
///
/// The face's only level is its rim's latitude, so `min_max` gave
/// `lo == hi` and `require_extent` refused `DegenerateFace` — the
/// executed disposition of issue 1250, for a face a ball cut by one
/// plane produces. σ names the missing extreme: with the rim traversed
/// `+u` the interior is above it and the extent is `[sin v₀, +1]`;
/// traversed `−u` the same rim bounds the ball MINUS that cap and the
/// extent is `[−1, sin v₀]`. Both are closed forms of the same
/// `R²·Δu·(sin v_hi − sin v_lo)`, with `Δu = 2π` summed from the rim's
/// own span.
#[test]
fn a_rim_only_cap_measures_the_spherical_cap_on_both_sides() {
    for v0 in [0.5_f64, -0.5, 0.0, 1.2, -1.2] {
        let cap = TAU * RS * RS * (1.0 - v0.sin());
        let rest = TAU * RS * RS * (1.0 + v0.sin());
        measures_outward(
            &format!("cap above v0={v0}"),
            &[rim(v0, 0.0, TAU, 0, 0)],
            cap,
        );
        measures_outward(
            &format!("ball minus the cap above v0={v0}"),
            &[rim(v0, TAU, 0.0, 0, 0)],
            rest,
        );
    }
}

/// **The rim split into several arcs is the same face.** A boolean-cut
/// or re-merged rim legitimately arrives as several arcs at one level;
/// `du_of_rims` sums their spans per (level, direction) group, and the
/// interior side is decided per arc. Nothing about the answer depends
/// on how many edges state the circle.
#[test]
fn a_rim_only_cap_stated_as_three_arcs_measures_the_same() {
    let v0 = 0.5_f64;
    let cap = TAU * RS * RS * (1.0 - v0.sin());
    let third = TAU / 3.0;
    measures_outward(
        "cap whose rim is three arcs",
        &[
            rim(v0, 0.0, third, 0, 1),
            rim(v0, third, 2.0 * third, 1, 2),
            rim(v0, 2.0 * third, TAU, 2, 0),
        ],
        cap,
    );
}

/// **A pole is interior to a rim only if the rim CLOSES around it.**
///
/// Adopted from the R1 review lane's `probe_c1b_partial_and_doubled_rims`
/// (PR 2741's dual), which is what found this: the fold reads a
/// traversal DIRECTION, which says which side of the rim the material
/// is on and nothing about how far the rim goes, and nothing else on
/// the arm was watching either — the levels hold one latitude however
/// much of it the boundary states, `props_rim_level` places a rim at an
/// extreme whatever its span, and `du_of_rims` sums a group's spans
/// without comparing the sum to a whole turn. So four shapes that were
/// `DegenerateFace` before the fold answered a definite AREA through
/// this door: half a rim at half the cap's, a quarter at a quarter, the
/// same full rim stated twice at double, and a full rim plus a half arc
/// at 1.5×. `props_rim_only_closed` decides `(Δu − τ)·R`, the arc the
/// rim fails to close by, and each of them refuses by that name.
#[test]
fn a_rim_only_cap_refuses_a_rim_that_does_not_close() {
    let v0 = 0.5_f64;
    let cap = TAU * RS * RS * (1.0 - v0.sin());
    let mut answered = Vec::new();
    for (name, edges) in [
        ("half a rim only (du = pi)", vec![rim(v0, 0.0, PI, 0, 1)]),
        ("quarter rim only", vec![rim(v0, 0.0, PI / 2.0, 0, 1)]),
        (
            "the same full rim stated twice",
            vec![rim(v0, 0.0, TAU, 0, 0), rim(v0, 0.0, TAU, 0, 0)],
        ),
        (
            "full rim + an extra half arc, same direction",
            vec![rim(v0, 0.0, TAU, 0, 0), rim(v0, 0.0, PI, 0, 1)],
        ),
    ] {
        match curved_face(&sphere(), &edges, true, band()) {
            Ok(fc) => {
                println!(
                    "  {name:<44} ACCEPT area={:.15e} (whole cap {cap:.15e}, ratio {:.4})",
                    fc.area,
                    fc.area / cap
                );
                answered.push(name);
            }
            Err(e) => println!("  {name:<44} REFUSE {e:?}"),
        }
    }
    assert!(
        answered.is_empty(),
        "a rim that does not close bounds no cap, and these answered an area: {answered:?}"
    );
    // The closed rim, stated as one arc or as three, is what the fold
    // is for — the refusal above must not have cost it.
    measures_outward("the closed rim", &[rim(v0, 0.0, TAU, 0, 0)], cap);
}

/// **The true zero-extent patch keeps `DegenerateFace`** — the M2
/// verdict this unit must not spend. Two rims at ONE level traversed
/// opposite ways point at opposite poles, so there is no pole the face
/// contains: the levels stay silent and `require_extent` refuses
/// exactly as before. This is the row that tells the fix apart from
/// "push a pole whenever the extent collapses".
#[test]
fn rims_at_one_level_with_opposite_traversals_stay_degenerate() {
    let v0 = 0.5_f64;
    let got = curved_face(
        &sphere(),
        &[rim(v0, 0.0, PI, 0, 1), rim(v0, PI, 0.0, 1, 0)],
        true,
        band(),
    );
    assert!(
        matches!(got, Err(PropsError::DegenerateFace)),
        "two rims at one level traversed opposite ways bound no extent; got {got:?}"
    );
}

/// **A rim-only cap's boundary encodes no material side.** The same
/// rim traversed the same way bounds the cap under one sense bit and
/// the ball-minus-cap under the other — two valid bodies — so
/// [`boundary_material_sign`] answers `Unencoded` rather than a
/// definite ±1 tier 3's check 6 would read as disagreement. The
/// rimless band's sibling, for the same reason at a different face.
#[test]
fn a_rim_only_cap_encodes_no_material_side() {
    let v0 = 0.5_f64;
    assert_eq!(
        boundary_material_sign(&sphere(), &[rim(v0, 0.0, TAU, 0, 0)], band()),
        Ok(MaterialSign::Unencoded)
    );
    // The spherical ZONE between two rims does encode one: its levels
    // carry an extent of their own, so "which extreme is this rim at"
    // has an answer.
    assert!(matches!(
        boundary_material_sign(
            &sphere(),
            &[rim(-0.3, 0.0, TAU, 0, 0), rim(0.5, TAU, 0.0, 1, 1)],
            band()
        ),
        Ok(MaterialSign::Encoded(_))
    ));
}

// ---------------------------------------------------------------------
// Issue 1598 — the L-shaped complement
// ---------------------------------------------------------------------

/// The half-cap `[0, π] × [v₀, π/2]`: one rim half at latitude `v₀`
/// traversed `+u` and one great-circle arc over the north pole.
fn half_cap(v0: f64, forward: bool) -> Vec<LoopEdge<f64>> {
    if forward {
        vec![rim(v0, 0.0, PI, 0, 1), great(0.0, PI - v0, v0, 1, 0)]
    } else {
        vec![rim(v0, PI, 0.0, 1, 0), great(0.0, v0, PI - v0, 0, 1)]
    }
}

/// **The half-cap measures and its complement refuses — one row per
/// traversal direction** (issue 1598).
///
/// Both faces are bounded by the SAME two edges traversed opposite
/// ways, and the parse hands both the same levels `{sin v₀, +1}`: the
/// rim's latitude, the arc's endpoints and the pole its span crosses.
/// `linear_rim_side` then reads opposite `s_f` off the two traversals
/// and their fluxes are equal and opposite, which is how a closed
/// sphere measured `0.0`. What separates them is σ: the half-cap's rim
/// sits at `lo` with its interior above, the complement's sits at `lo`
/// with its interior BELOW — its material runs past the extent, down
/// to the south pole, and the rectangle premise does not hold.
///
/// Which of the two is which is the SENSE bit's to say, and the second
/// half of this row is the same pair with the bit flipped: the
/// complement is then the cap seen from inside and measures, and the
/// half-cap is the one whose interior side points out.
#[test]
fn the_l_shaped_complement_refuses_by_its_own_name() {
    let v0 = 0.5_f64;
    let cap_half = PI * RS * RS * (1.0 - v0.sin());
    for (sense, measuring, refusing) in [(true, true, false), (false, false, true)] {
        let label = format!("sense={sense}");
        match curved_face(&sphere(), &half_cap(v0, measuring), sense, band()) {
            Ok(fc) => {
                let rel = (fc.area - cap_half).abs() / cap_half;
                assert!(rel < 1e-12, "{label}: half-cap area {} ", fc.area);
            }
            Err(e) => panic!("{label}: the half-cap must still measure: {e:?}"),
        }
        let got = curved_face(&sphere(), &half_cap(v0, refusing), sense, band());
        assert!(
            matches!(
                got,
                Err(PropsError::NotIsoRectangle {
                    what: "props_rim_interior_side"
                })
            ),
            "{label}: the L-shaped complement is refused by its own name; got {got:?}"
        );
    }
}

/// **The gate answers an EXEMPTION where the rims contradict each
/// other, instead of whichever rim came first.**
///
/// Adopted from R2's `r2_staircase_face` and R1's
/// `probe_c5_contradictory_two_rim_face` (PR 2741's dual). The
/// staircase is a closed simple curve — rim at `lo` over `[0, π]` `+u`,
/// a meridian up, rim at `hi` over `[π, 2π]` `+u`, a meridian down —
/// whose left region is the half-zone PLUS the whole cap above `hi`.
/// Every OLD premise passed it: both rims sit at an extreme, and the
/// two (level, direction) groups sum to the same `Δu = π`. The closed
/// form answered `R²π(sin hi − sin lo)` for it, 57% of the truth, with
/// `pad = 0`.
///
/// `props_rim_interior_side` refuses the flux lane's answer. What this
/// row is about is the OTHER lane: `boundary_material_sign` reads no
/// sense bit, so it cannot ask σ — but "every rim encodes the same
/// side" needs no bit and is not a tautology, and without it the gate
/// answered a DEFINITE ±1 whose value was `Positive` with the lower rim
/// first and `Negative` with the upper rim first. Not a recorded
/// verdict that moves under a re-anchoring: an ANSWER that does, which
/// is the precise failure `linear_rim_side`'s own docs say the paired
/// premise exists to prevent.
#[test]
fn the_gate_exempts_a_face_whose_rims_encode_different_sides() {
    let (lo, hi) = (-0.3_f64, 0.5_f64);
    let stair = [
        rim(lo, 0.0, PI, 0, 1),
        great(PI, lo, hi, 1, 2),
        rim(hi, PI, TAU, 2, 3),
        great(0.0, hi, lo, 3, 0),
    ];
    let true_area = RS * RS * PI * (hi.sin() - lo.sin()) + TAU * RS * RS * (1.0 - hi.sin());
    for k in 0..stair.len() {
        let e: Vec<LoopEdge<f64>> = (0..stair.len())
            .map(|i| stair[(i + k) % stair.len()].clone())
            .collect();
        let flux = curved_face(&sphere(), &e, true, band());
        let gate = boundary_material_sign(&sphere(), &e, band());
        println!("  staircase k={k}: flux {flux:?} gate {gate:?} (true area {true_area:.6e})");
        assert!(
            matches!(
                flux,
                Err(PropsError::NotIsoRectangle {
                    what: "props_rim_interior_side"
                })
            ),
            "k={k}: the flux lane refuses the notch: {flux:?}"
        );
        assert!(
            matches!(
                gate,
                Err(PropsError::NotIsoRectangle {
                    what: "props_rim_side"
                })
            ),
            "k={k}: the gate exempts rather than answering: {gate:?}"
        );
    }
    // Two full rims traversed the SAME way — R1's shape, no meridian to
    // carry the notch — reads the same way round.
    for e in [
        vec![rim(lo, 0.0, TAU, 0, 0), rim(hi, 0.0, TAU, 1, 1)],
        vec![rim(hi, 0.0, TAU, 1, 1), rim(lo, 0.0, TAU, 0, 0)],
    ] {
        assert!(matches!(
            boundary_material_sign(&sphere(), &e, band()),
            Err(PropsError::NotIsoRectangle {
                what: "props_rim_side"
            })
        ));
    }
    // And the consistent zone still ANSWERS, the same sign either way
    // round: unanimity is a property of the set.
    for e in [
        vec![rim(lo, 0.0, TAU, 0, 0), rim(hi, TAU, 0.0, 1, 1)],
        vec![rim(hi, TAU, 0.0, 1, 1), rim(lo, 0.0, TAU, 0, 0)],
    ] {
        assert_eq!(
            boundary_material_sign(&sphere(), &e, band()),
            Ok(MaterialSign::Encoded(geom_core::Sign::Positive))
        );
    }
}

/// **The one shape no sense-free gate can see**, adopted from R2's
/// `r2_c5_gate_on_the_l_shape` and kept as this unit's recorded
/// residue. The half-cap and its L-shaped complement are the same two
/// edges traversed opposite ways, each with ONE rim, so there is
/// nothing for unanimity to compare and the gate answers a definite —
/// and for the complement, wrong — side. Only σ separates them, and σ
/// is the traversal under the face's sense bit, which is the very
/// thing tier 3's check 6 compares the gate's answer TO. The flux
/// lane's refusal is what reports this face.
#[test]
fn the_gate_cannot_see_the_l_shaped_complement() {
    let v0 = 0.5_f64;
    assert_eq!(
        boundary_material_sign(&sphere(), &half_cap(v0, true), band()),
        Ok(MaterialSign::Encoded(geom_core::Sign::Positive))
    );
    assert_eq!(
        boundary_material_sign(&sphere(), &half_cap(v0, false), band()),
        Ok(MaterialSign::Encoded(geom_core::Sign::Negative))
    );
}

// ---------------------------------------------------------------------
// The recorded sign is a face fact
// ---------------------------------------------------------------------

/// The `(predicate, sign)` multiset one `curved_face` call records.
fn verdict_multiset(edges: &[LoopEdge<f64>]) -> Vec<(String, usize)> {
    let bracket = geom_core::k_stats::Bracket::open();
    let _ = curved_face(&sphere(), edges, true, band());
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

/// The signs one named predicate recorded, in decision order.
fn signs_of(edges: &[LoopEdge<f64>], predicate: &str) -> Vec<String> {
    let bracket = geom_core::k_stats::Bracket::open();
    let _ = curved_face(&sphere(), edges, true, band());
    bracket
        .finish()
        .verdicts
        .iter()
        .filter(|v| v.predicate == predicate)
        .map(|v| format!("{:?}", v.sign))
        .collect()
}

/// **`props_rim_interior_side` does not move under a re-anchoring, and
/// `props_rim_side` still does** — the measurement, on one face.
///
/// A spherical zone bounded by two full rims, presented twice with the
/// edge list rotated: the same face, the same body, a different rim
/// handed over first. σ is per rim — this rim's stored traversal
/// direction under the face's own sense bit, reading no other rim — so
/// the multiset of what it records is identical. `props_rim_side` is
/// not: it classifies `lo + hi − 2·level` on `rims.first()` alone, so
/// rotating the anchor swaps its recorded sign while the flux it
/// underwrites is compensated downstream and does not move. That is
/// the open row `rim-side-and-rim-dir-group-signs-are-facts-about-
/// cycle-order`, whose OTHER predicate — `props_rim_dir_group` — is
/// retired here and records nothing at all any more.
#[test]
fn the_interior_side_verdicts_are_a_face_fact_under_re_anchoring() {
    let zone = |rotated: bool| {
        if rotated {
            vec![rim(0.5, TAU, 0.0, 1, 1), rim(-0.3, 0.0, TAU, 0, 0)]
        } else {
            vec![rim(-0.3, 0.0, TAU, 0, 0), rim(0.5, TAU, 0.0, 1, 1)]
        }
    };
    // The face measures the zone either way — the answer was never the
    // thing that moved.
    let exact = TAU * RS * RS * (0.5_f64.sin() - (-0.3_f64).sin());
    measures_outward("zone", &zone(false), exact);
    measures_outward("zone re-anchored", &zone(true), exact);

    assert_eq!(
        signs_of(&zone(false), "props_rim_interior_side"),
        signs_of(&zone(true), "props_rim_interior_side"),
        "σ reads one rim and the face's bit, so a re-anchoring cannot move it"
    );
    let without_rim_side = |rotated: bool| {
        let mut m = verdict_multiset(&zone(rotated));
        m.retain(|(k, _)| !k.starts_with("props_rim_side "));
        m
    };
    assert_eq!(
        without_rim_side(false),
        without_rim_side(true),
        "every recorded predicate of this face but props_rim_side is anchor-free"
    );
    assert_eq!(
        signs_of(&zone(false), "props_rim_dir_group"),
        Vec::<String>::new(),
        "props_rim_dir_group is retired: the direction is compared AS a sign"
    );
    // The measurement the open row asked for, executed: this one DOES
    // move, and nothing in this unit fixed it.
    assert_eq!(signs_of(&zone(false), "props_rim_side"), ["Positive"]);
    assert_eq!(signs_of(&zone(true), "props_rim_side"), ["Negative"]);
}

/// **The recorded population is a face fact on a REFUSING face too.**
///
/// Adopted from R1's `probe_c4_recorded_population_on_a_refusing_face`
/// (PR 2741's dual), which found a fresh instance of the class the
/// spec's own amendment is about: `require_rim_interior_sides` returned
/// at the first rim that pointed out, so a face with one agreeing rim
/// and one refusing rim recorded TWO verdicts anchored one way and ONE
/// anchored the other — each sign a face fact, the multiset not. Every
/// rim is decided before any refusal is returned now.
#[test]
fn the_refusing_faces_verdicts_are_a_face_fact_too() {
    let lo_ok = rim(-0.3, 0.0, TAU, 0, 0); // at lo, σ = +1 -> Positive
    let hi_bad = rim(0.5, 0.0, TAU, 1, 1); // at hi, σ = +1 -> Negative
    let first = signs_of(&[lo_ok.clone(), hi_bad.clone()], "props_rim_interior_side");
    let second = signs_of(&[hi_bad.clone(), lo_ok.clone()], "props_rim_interior_side");
    println!("  lo-first {first:?} / hi-first {second:?}");
    assert_eq!(first.len(), 2, "every rim is decided: {first:?}");
    let (mut a, mut b) = (first, second);
    a.sort();
    b.sort();
    assert_eq!(
        a, b,
        "the same face records the same verdicts either way round"
    );
    // And it still refuses, by the name, either way round.
    for e in [vec![lo_ok.clone(), hi_bad.clone()], vec![hi_bad, lo_ok]] {
        assert!(matches!(
            curved_face(&sphere(), &e, true, band()),
            Err(PropsError::NotIsoRectangle {
                what: "props_rim_interior_side"
            })
        ));
    }
}

/// **The rim-only cap's own verdicts, named.** The fold decides
/// `props_rim_only_extent` once per meridian-free rim boundary,
/// `props_rim_interior_side` once per rim, and `props_rim_only_closed`
/// once per folded pole, with `props_rim_only_join` once per arc. Only
/// the last reads a comparand the arm did not already have — a point
/// deviation between two vertices, `require_rim_incidence`'s
/// dimension. σ is a
/// product of two discrete signs, `props_rim_only_extent` is
/// `require_extent`'s own sphere margin asked one step earlier (hence
/// the two records here), `props_rim_interior_side` is
/// `props_rim_side`'s pointed by σ, and `props_rim_only_closed` is the
/// `Δu` angle at the azimuthal arm `props_du_consistent` already
/// meters.
#[test]
fn the_rim_only_cap_records_its_named_decides() {
    let got = verdict_multiset(&[rim(0.5, 0.0, TAU, 0, 0)]);
    let want: Vec<(String, usize)> = [
        ("props_circle_axis_class Positive", 1),
        ("props_face_extent Positive", 1),
        ("props_rim_axis_parallel Zero", 1),
        ("props_rim_center_on_axis Zero", 1),
        ("props_rim_fit Zero", 1),
        ("props_rim_interior_side Positive", 1),
        ("props_rim_level Zero", 1),
        ("props_rim_only_closed Zero", 1),
        ("props_rim_only_extent Zero", 1),
        ("props_rim_only_join Zero", 1),
        ("props_rim_side Positive", 1),
    ]
    .into_iter()
    .map(|(k, n)| (k.to_string(), n))
    .collect();
    assert_eq!(got, want, "the rim-only cap's recorded population");
}

// ---------------------------------------------------------------------
// The sibling kinds
// ---------------------------------------------------------------------

// **The cone apex cap is the sphere cap's sibling, and it is served on
// its own chart** — `props_cone_apex_cap.rs`, whose fold pushes level
// `0` where this one pushes a pole. The two differ in exactly one
// thing: the sphere's missing extreme is one of TWO poles and σ picks
// between them, so `sphere_rim_only_pole_level` reads the face's sense
// bit; a cone is bounded on the apex side only, so its fold reads no
// bit at all and `fn cone` still takes none.
//
// What a cone cannot do without a bit is tell the apex cap from the
// rest of its nappe, which is unbounded and no finite face of any
// solid. That question is the boundary's material side, answered by
// `boundary_material_sign` and compared with `Face::sense` at tier 3's
// check 6 — where this cap's is `Unencoded` and nothing compares it at
// all (`a_rim_only_cap_encodes_no_material_side` above).

/// **The cylinder's rim-only face is genuinely extent-less** — the
/// sweep's negative result, executed. A cylinder is unbounded along
/// its axis in BOTH directions, so one rim circle bounds no finite
/// face whichever way it is traversed: there is no missing extreme for
/// a traversal to name, and `DegenerateFace` is the right answer
/// rather than an unbuilt lane.
#[test]
fn a_cylinder_rim_only_face_is_extent_less() {
    let cyl = surf::cylinder::<f64>(RS);
    for (u0, u1) in [(0.0, TAU), (TAU, 0.0)] {
        let edges = vec![topo::edge(
            Curve3::Circle {
                center: p3(0.0, 0.0, 0.0),
                axis: v3(0.0, 0.0, 1.0),
                radius: RS,
                u_ref: v3(1.0, 0.0, 0.0),
            },
            u0,
            u1,
            0,
            0,
        )];
        assert!(matches!(
            curved_face(&cyl, &edges, true, band()),
            Err(PropsError::DegenerateFace)
        ));
    }
}

// ---------------------------------------------------------------------
// The certifying scalar
// ---------------------------------------------------------------------

/// The same geometry through the Interval decision scalar: the fold is
/// a discrete sign product and the two margins it decides are already
/// on the matrix, so no outcome may move.
mod interval_lane {
    use crate::shared::surf;
    use crate::shared::tol::band;
    use crate::shared::topo;
    use geom::Surface;
    use geom_brep::props::{LoopEdge, curved_face};
    use geom_core::{Interval, Real};

    const RSF: f64 = 0.010;
    const TAU: f64 = core::f64::consts::TAU;
    const PI: f64 = core::f64::consts::PI;

    fn sphere<T: Real>() -> Surface<T> {
        surf::sphere(RSF)
    }
    fn rim<T: Real>(v: f64, u0: f64, u1: f64, a: u32, b: u32) -> LoopEdge<T> {
        topo::sphere_rim(RSF, v, u0, u1, a, b)
    }
    fn great<T: Real>(u: f64, t0: f64, t1: f64, a: u32, b: u32) -> LoopEdge<T> {
        topo::sphere_great(RSF, u, t0, t1, a, b)
    }

    fn cases<T: geom_core::Decide>() -> Vec<(&'static str, Vec<LoopEdge<T>>)> {
        let v0 = 0.5;
        vec![
            ("rim-only cap, +u", vec![rim(v0, 0.0, TAU, 0, 0)]),
            ("rim-only cap, -u", vec![rim(v0, TAU, 0.0, 0, 0)]),
            (
                "rims at one level, opposite ways",
                vec![rim(v0, 0.0, PI, 0, 1), rim(v0, PI, 0.0, 1, 0)],
            ),
            (
                "half-cap",
                vec![rim(v0, 0.0, PI, 0, 1), great(0.0, PI - v0, v0, 1, 0)],
            ),
            (
                "L-shaped complement",
                vec![rim(v0, PI, 0.0, 1, 0), great(0.0, v0, PI - v0, 0, 1)],
            ),
        ]
    }

    #[test]
    fn the_pole_side_outcomes_match_at_interval() {
        let band = band();
        for ((name, ef), (_, ei)) in cases::<f64>().into_iter().zip(cases::<Interval>()) {
            let a = curved_face(&sphere::<f64>(), &ef, true, band);
            let b = curved_face(&sphere::<Interval>(), &ei, true, band);
            match (&a, &b) {
                (Ok(_), Ok(_)) => {}
                (Err(x), Err(y)) => assert_eq!(
                    core::mem::discriminant(x),
                    core::mem::discriminant(y),
                    "{name}: the two scalars refused differently: {x:?} vs {y:?}"
                ),
                _ => panic!("{name}: outcome mismatch — f64 {a:?}"),
            }
        }
    }
}

/// **The sphere twin of the cone's sum-is-not-a-cover row.**
/// `require_rim_only_closed` has two call sites and the defect was in
/// the shared function, so the shape that reached it on the cone
/// reaches it here: the same HALF rim stated twice totals a turn and
/// covers half the circle, and answered the whole cap's area through
/// `curved_face`. `props_rim_only_join` requires the arcs to TILE —
/// each edge's traversal end is the next one's traversal start,
/// cyclically — and the three-arc cap is the floor that says the rule
/// is not "refuse every multi-arc rim".
#[test]
fn a_pole_is_not_folded_for_a_rim_that_totals_a_turn_without_tiling() {
    let v0 = 0.5_f64;
    let cap = TAU * RS * RS * (1.0 - v0.sin());
    let third = TAU / 3.0;
    for (name, edges) in [
        (
            "the same HALF rim stated twice",
            vec![rim(v0, 0.0, PI, 0, 1), rim(v0, 0.0, PI, 0, 1)],
        ),
        (
            "two overlapping arcs totalling a turn",
            vec![
                rim(v0, 0.0, 0.75 * TAU, 0, 1),
                rim(v0, 0.5 * TAU, 0.75 * TAU, 1, 2),
            ],
        ),
        (
            "a tiling with two arcs exchanged",
            vec![
                rim(v0, third, 2.0 * third, 1, 2),
                rim(v0, 0.0, third, 0, 1),
                rim(v0, 2.0 * third, TAU, 2, 0),
            ],
        ),
    ] {
        let got = curved_face(&sphere(), &edges, true, band());
        assert!(
            matches!(
                &got,
                Err(PropsError::NotIsoRectangle {
                    what: "props_rim_only_join"
                })
            ),
            "{name}: arcs that total a turn without tiling bound no cap, got {got:?}"
        );
    }
    // The floor: the same three arcs in loop order still measure.
    measures_outward(
        "the cap whose rim is three arcs, in order",
        &[
            rim(v0, 0.0, third, 0, 1),
            rim(v0, third, 2.0 * third, 1, 2),
            rim(v0, 2.0 * third, TAU, 2, 0),
        ],
        cap,
    );
}
