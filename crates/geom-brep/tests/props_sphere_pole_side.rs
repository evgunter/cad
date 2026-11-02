//! **A sphere face lies on the side of its loop that holds no chart
//! pole** (issues 1250 and 1598, and the sphere arm's one closed form).
//!
//! A chart singularity inside a face is a vertex of it, so a one-loop
//! sphere face is whichever side of its loop holds neither pole, and
//! `curved::sphere_loop_side` reads that side off each pole off the
//! loop. The flux lane measures by Gauss–Bonnet against the face's
//! sense bit and refuses a bit that side contradicts. These rows pin
//! the faces the rim-traversal arm used to read:
//!
//! * **The rim-only polar cap.** One rim circle, no meridian, the pole
//!   interior: a pole lies on each side of the rim, so under either bit
//!   the face holds one and the loop encodes no side. It measures under
//!   its bit, `2πR²(1 − sin v₀)`; refusing it is validity's (the form
//!   STEP import normalises). Its seamed form, the pole a vertex,
//!   encodes its side and measures the same.
//! * **The L-shaped complement.** A half-cap and the rest of its sphere
//!   share both edges traversed opposite ways; the complement holds the
//!   south pole, so the bit that claims it is refused, and the bit that
//!   claims the half-cap measures it.
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
use geom_brep::props::{
    LoopEdge, MaterialSign, PropsError, boundary_material_sign, boundary_material_sign_loops,
    curved_face, curved_face_loops,
};
use geom_core::Sign;

/// The sphere under every row: R = 10 mm about +Z at the origin.
const RS: f64 = 0.010;
const PI: f64 = core::f64::consts::PI;
const TAU: f64 = core::f64::consts::TAU;
const HALF: f64 = core::f64::consts::FRAC_PI_2;

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

/// **A cap bounded by ONE rim circle and nothing else encodes no side,
/// and measures under its bit.** The rim separates the two poles, so
/// whichever side is material holds one: the reading says nothing about
/// the bit (`Unencoded`), and the face measures the cap `2πR²(1 − sin v₀)`
/// under the bit that claims it and the ball minus the cap under the
/// other — on both poles' caps, both traversals, and with the rim
/// stated as one arc or three.
#[test]
fn a_rim_only_cap_encodes_no_side_and_measures_under_its_bit() {
    let third = TAU / 3.0;
    for v0 in [0.5_f64, -0.5, 0.0, 1.2, -1.2] {
        let cap = TAU * RS * RS * (1.0 - v0.sin());
        let rest = TAU * RS * RS * (1.0 + v0.sin());
        for (label, edges, above) in [
            ("+u", vec![rim(v0, 0.0, TAU, 0, 0)], cap),
            ("-u", vec![rim(v0, TAU, 0.0, 0, 0)], rest),
            (
                "three arcs",
                vec![
                    rim(v0, 0.0, third, 0, 1),
                    rim(v0, third, 2.0 * third, 1, 2),
                    rim(v0, 2.0 * third, TAU, 2, 0),
                ],
                cap,
            ),
        ] {
            assert_eq!(
                boundary_material_sign(&sphere(), &edges, band()),
                Ok(MaterialSign::Unencoded),
                "v0={v0}, {label}: the gate"
            );
            measures_outward(&format!("v0={v0}, {label}"), &edges, above);
            let below = curved_face(&sphere(), &edges, false, band())
                .unwrap_or_else(|e| panic!("v0={v0}, {label}, reversed bit: {e:?}"));
            let other = cap + rest - above;
            assert!(
                (below.area - other).abs() / other < 1e-12,
                "v0={v0}, {label}: the reversed bit measures the other side, {} vs {other}",
                below.area
            );
        }
    }
}

/// The half-cap `[u0, u0 + π] × [v₀, π/2]` with the north pole a
/// vertex: half a rim at latitude `v₀` traversed `+u`, up the meridian
/// at `u0 + π` to the pole, and down the one at `u0`.
fn seamed_half_cap(v0: f64, u0: f64) -> Vec<LoopEdge<f64>> {
    vec![
        rim(v0, u0, u0 + PI, 0, 1),
        great(u0 + PI, v0, HALF, 1, 2),
        great(u0, HALF, v0, 2, 0),
    ]
}

/// **The cap with its pole a vertex measures exactly.** The kernel
/// states a cap as two half-caps meeting on a seam through the pole;
/// each has the pole on its loop and the far pole to its right, so it
/// encodes its side and measures `πR²(1 − sin v₀)`, and the two make
/// the cap.
#[test]
fn the_seamed_cap_measures_the_spherical_cap() {
    for v0 in [0.5_f64, -0.5, 0.0, 1.2, -1.2] {
        let half = PI * RS * RS * (1.0 - v0.sin());
        for u0 in [0.0, PI] {
            let edges = seamed_half_cap(v0, u0);
            measures_outward(&format!("half-cap v0={v0} u0={u0}"), &edges, half);
            assert_eq!(
                boundary_material_sign(&sphere(), &edges, band()),
                Ok(MaterialSign::Encoded(Sign::Positive)),
                "v0={v0} u0={u0}"
            );
            assert_eq!(
                curved_face(&sphere(), &edges, false, band()).map(|_| ()),
                Err(PropsError::SenseContradicted),
                "v0={v0} u0={u0}: the flipped bit"
            );
        }
    }
}

/// **A rim that does not close bounds nothing.** Half a rim, a quarter,
/// a full rim with an extra half arc: each loop's last arc ends away
/// from its first one's start, and the arm refuses on its own closure
/// decide. The same full rim stated twice closes but runs round twice:
/// no simple loop, which the arm does not decide (`curved::sphere_loop`'s
/// docs). It answers the formula's value, `R²·2π(1 − 2 sin v₀)` — the
/// residual this row pins, red the day simplicity is decided.
#[test]
fn a_rim_that_does_not_close_bounds_nothing() {
    let v0 = 0.5_f64;
    let open = Err(PropsError::SphereLoop {
        what: "props_loop_closed",
    });
    for (name, edges) in [
        ("half a rim only", vec![rim(v0, 0.0, PI, 0, 1)]),
        ("quarter rim only", vec![rim(v0, 0.0, PI / 2.0, 0, 1)]),
        (
            "full rim + an extra half arc, same direction",
            vec![rim(v0, 0.0, TAU, 0, 0), rim(v0, 0.0, PI, 0, 1)],
        ),
    ] {
        assert_eq!(
            curved_face(&sphere(), &edges, true, band()).map(|_| ()),
            open,
            "{name}"
        );
    }
    let twice = curved_face(
        &sphere(),
        &[rim(v0, 0.0, TAU, 0, 0), rim(v0, 0.0, TAU, 0, 0)],
        true,
        band(),
    )
    .expect("the doubled rim is answered")
    .area;
    let formula = TAU * RS * RS * (1.0 - 2.0 * v0.sin());
    assert!(
        (twice - formula).abs() / formula < 1e-12,
        "the same full rim stated twice: {twice} vs {formula}"
    );
}

/// **Arcs that total a turn without tiling do not close.** The same
/// half rim stated twice, two overlapping arcs, and a tiling with two
/// arcs exchanged each leave a gap between one arc's end and the next
/// one's start; the floor is the same three arcs in loop order, which
/// close — shown on the seamed half-cap, whose rim is two arcs here.
#[test]
fn arcs_that_total_a_turn_without_tiling_do_not_close() {
    let v0 = 0.5_f64;
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
        assert_eq!(
            curved_face(&sphere(), &edges, true, band()).map(|_| ()),
            Err(PropsError::SphereLoop {
                what: "props_loop_closed"
            }),
            "{name}"
        );
    }
    let split = vec![
        rim(v0, 0.0, PI / 3.0, 0, 1),
        rim(v0, PI / 3.0, PI, 1, 2),
        great(PI, v0, HALF, 2, 3),
        great(0.0, HALF, v0, 3, 0),
    ];
    measures_outward(
        "the half-cap whose rim is two arcs",
        &split,
        PI * RS * RS * (1.0 - v0.sin()),
    );
}

/// **Two rims at one level traversed opposite ways turn back on
/// themselves** — the zero-extent patch. At each junction the departing
/// tangent is the arriving one reversed, a cusp with no turning angle,
/// refused by name.
#[test]
fn rims_at_one_level_with_opposite_traversals_are_a_cusp() {
    let v0 = 0.5_f64;
    assert_eq!(
        curved_face(
            &sphere(),
            &[rim(v0, 0.0, PI, 0, 1), rim(v0, PI, 0.0, 1, 0)],
            true,
            band(),
        )
        .map(|_| ()),
        Err(PropsError::SphereLoop {
            what: "props_sphere_loop_cusp"
        })
    );
}

// ---------------------------------------------------------------------
// Issue 1598 — the L-shaped complement
// ---------------------------------------------------------------------

/// The half-cap `[0, π] × [v₀, π/2]`: one rim half at latitude `v₀`
/// traversed `+u` and one great-circle arc over the north pole; or the
/// same two edges traversed the other way.
fn half_cap(v0: f64, forward: bool) -> Vec<LoopEdge<f64>> {
    if forward {
        vec![rim(v0, 0.0, PI, 0, 1), great(0.0, PI - v0, v0, 1, 0)]
    } else {
        vec![rim(v0, PI, 0.0, 1, 0), great(0.0, v0, PI - v0, 0, 1)]
    }
}

/// **The half-cap measures and its complement refuses, on each bit**
/// (issue 1598). Both faces are bounded by the SAME two edges traversed
/// opposite ways. The north pole lies on the great-circle arc; the
/// south pole lies right of the forward loop and left of the reversed
/// one. So the forward loop under `sense = true` and the reversed loop
/// under `sense = false` are the half-cap, measured exactly, and the
/// other two pairings name the complement, which holds the south pole,
/// and are refused.
#[test]
fn the_l_shaped_complement_holds_the_south_pole() {
    let v0 = 0.5_f64;
    let cap_half = PI * RS * RS * (1.0 - v0.sin());
    for (sense, measuring, refusing) in [(true, true, false), (false, false, true)] {
        let label = format!("sense={sense}");
        let fc = curved_face(&sphere(), &half_cap(v0, measuring), sense, band())
            .unwrap_or_else(|e| panic!("{label}: the half-cap must measure: {e:?}"));
        let rel = (fc.area - cap_half).abs() / cap_half;
        assert!(rel < 1e-12, "{label}: half-cap area {}", fc.area);
        assert_eq!(
            curved_face(&sphere(), &half_cap(v0, refusing), sense, band()).map(|_| ()),
            Err(PropsError::SenseContradicted),
            "{label}: the L-shaped complement"
        );
    }
}

/// **The gate sees the L-shaped complement.** The side reader reads no
/// sense bit and answers opposite sides for the two traversals, so tier
/// 3's check 6 holds each against the face's bit and names the one that
/// claims the complement.
#[test]
fn the_gate_sees_the_l_shaped_complement() {
    let v0 = 0.5_f64;
    assert_eq!(
        boundary_material_sign(&sphere(), &half_cap(v0, true), band()),
        Ok(MaterialSign::Encoded(Sign::Positive))
    );
    assert_eq!(
        boundary_material_sign(&sphere(), &half_cap(v0, false), band()),
        Ok(MaterialSign::Encoded(Sign::Negative))
    );
}

/// **A face whose rims run the same way has a pole on each side.** The
/// staircase — rim at `lo` over `[0, π]` `+u`, a meridian up, rim at
/// `hi` over `[π, 2π]` `+u`, a meridian down — is a closed simple curve
/// whose left region holds the north pole and whose right region holds
/// the south one. Its loop encodes no side, and at every rotation the
/// flux lane measures the left region under `sense = true`: the
/// half-zone plus the whole cap above `hi`, the area the iso arm once
/// answered 57% of. The consistent zone, its two rims as two loops
/// traversed opposite ways, measures and encodes one side whichever
/// loop comes first.
#[test]
fn a_face_whose_rims_run_the_same_way_has_a_pole_on_each_side() {
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
        measures_outward(&format!("staircase k={k}"), &e, true_area);
        assert_eq!(
            boundary_material_sign(&sphere(), &e, band()),
            Ok(MaterialSign::Unencoded),
            "k={k}: the gate"
        );
    }
    let (low, high) = (rim(lo, 0.0, TAU, 0, 0), rim(hi, TAU, 0.0, 1, 1));
    let exact = TAU * RS * RS * (hi.sin() - lo.sin());
    for loops in [
        [&[low.clone()][..], &[high.clone()][..]],
        [&[high][..], &[low][..]],
    ] {
        let fc = curved_face_loops(&sphere(), &loops, true, band()).expect("the zone");
        let rel = (fc.area - exact).abs() / exact;
        assert!(rel < 1e-12, "zone area {} vs {exact}", fc.area);
        assert_eq!(
            boundary_material_sign_loops(&sphere(), &loops, band()),
            Ok(MaterialSign::Encoded(Sign::Positive))
        );
    }
}

// ---------------------------------------------------------------------
// The recorded population is a face fact
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

/// **Every recorded verdict is a face fact under a re-anchoring.** A
/// patch bounded by a rim at `lo`, a meridian, a rim at `hi` and a
/// meridian, presented at every rotation of its edge list: each decide
/// reads one arc, one junction or one pole against the whole loop, so
/// the multiset of what the flux lane records is the same at every
/// anchor, and it records no `props_rim_side` at all.
#[test]
fn the_recorded_verdicts_are_a_face_fact_under_re_anchoring() {
    let (lo, hi) = (-0.3_f64, 0.5_f64);
    let patch = [
        rim(lo, 0.0, 1.0, 0, 1),
        great(1.0, lo, hi, 1, 2),
        rim(hi, 1.0, 0.0, 2, 3),
        great(0.0, hi, lo, 3, 0),
    ];
    let exact = RS * RS * (hi.sin() - lo.sin());
    let rotated = |k: usize| -> Vec<LoopEdge<f64>> {
        (0..patch.len())
            .map(|i| patch[(i + k) % patch.len()].clone())
            .collect()
    };
    let first = verdict_multiset(&rotated(0));
    assert!(
        first.iter().all(|(k, _)| !k.starts_with("props_rim_side ")),
        "the flux lane reads no rim side: {first:?}"
    );
    for k in 0..patch.len() {
        measures_outward(&format!("patch k={k}"), &rotated(k), exact);
        assert_eq!(verdict_multiset(&rotated(k)), first, "k={k}");
    }
}

/// **The rim-only cap's recorded population says why it encodes no
/// side:** the two poles read on opposite sides of the rim
/// (`props_sphere_pole_winding`, once `Positive` and once `Negative`),
/// each off a whole circle's projection (`props_sphere_pole_arc_full`)
/// from a pole off the rim's plane (`props_sphere_pole_plane`), beside
/// the loop's own premises and the area range it measures under.
#[test]
fn the_rim_only_cap_records_two_opposite_pole_readings() {
    let got = verdict_multiset(&[rim(0.5, 0.0, TAU, 0, 0)]);
    let want: Vec<(String, usize)> = [
        ("props_loop_closed Zero", 1),
        ("props_sphere_arc_span_forward Positive", 1),
        ("props_sphere_arc_span_winding Zero", 1),
        ("props_sphere_circle_fit Zero", 1),
        ("props_sphere_circle_on Zero", 1),
        ("props_sphere_loop_area Positive", 1),
        ("props_sphere_loop_cusp Positive", 1),
        ("props_sphere_pole_arc_full Zero", 2),
        ("props_sphere_pole_plane Negative", 1),
        ("props_sphere_pole_plane Positive", 1),
        ("props_sphere_pole_winding Negative", 1),
        ("props_sphere_pole_winding Positive", 1),
    ]
    .into_iter()
    .map(|(k, n)| (k.to_string(), n))
    .collect();
    assert_eq!(got, want, "the rim-only cap's recorded population");
}

// ---------------------------------------------------------------------
// The sibling kinds
// ---------------------------------------------------------------------

// **The cone apex cap is the sphere cap's sibling, served on its own
// chart** — `props_cone_apex_cap.rs`, whose fold pushes level `0`. A
// cone is bounded on the apex side only, so its fold reads no bit; what
// tells the apex cap from the rest of its nappe is the boundary's
// material side, read off its axial vector area and compared with
// `Face::sense` at tier 3's check 6.

/// **The cylinder's rim-only face is genuinely extent-less** — the
/// sweep's negative result, executed. A cylinder is unbounded along
/// its axis in BOTH directions, so one rim circle bounds no finite
/// face whichever way it is traversed: there is no missing extreme for
/// a traversal to name. It is a loop winding the cylinder alone, which
/// bounds no region of it, and that is the answer
/// ([`PropsError::LoopsWindChart`]) rather than an unbuilt lane.
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
            Err(PropsError::LoopsWindChart)
        ));
    }
}

// ---------------------------------------------------------------------
// The certifying scalar
// ---------------------------------------------------------------------

/// The same geometry through the Interval decision scalar: every
/// outcome above is decided off margins already on the matrix, so no
/// outcome may move.
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
    const HALF: f64 = core::f64::consts::FRAC_PI_2;

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
                "seamed half-cap",
                vec![
                    rim(v0, 0.0, PI, 0, 1),
                    great(PI, v0, HALF, 1, 2),
                    great(0.0, HALF, v0, 2, 0),
                ],
            ),
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

/// The northern half of the cap of angular radius `th` about `+y`, cut
/// by the great circle `z = 0`, under a chart whose axis is `+y` turned
/// `off` radians toward `−z` — off the face, past its equator edge —
/// with its antipode far outside. The loop: the equator from `x > 0` to
/// `x < 0` about `+z`, then the cap's rim back over the north about
/// `+y`; the face is left of both.
fn half_cap_with_a_pole_off_its_equator(th: f64, off: f64) -> (Surface<f64>, Vec<LoopEdge<f64>>) {
    let axis = v3(0.0, off.cos(), -off.sin());
    let s = Surface::Sphere {
        center: p3(0.0, 0.0, 0.0),
        radius: RS,
        axis,
        u_ref: v3(1.0, 0.0, 0.0),
    };
    let equator = LoopEdge::hand_built(
        Curve3::Circle {
            center: p3(0.0, 0.0, 0.0),
            axis: v3(0.0, 0.0, 1.0),
            radius: RS,
            u_ref: v3(1.0, 0.0, 0.0),
        },
        HALF - th,
        HALF + th,
        true,
        0,
        1,
    );
    let rim = LoopEdge::hand_built(
        Curve3::Circle {
            center: p3(0.0, RS * th.cos(), 0.0),
            axis: v3(0.0, 1.0, 0.0),
            radius: RS * th.sin(),
            u_ref: v3(1.0, 0.0, 0.0),
        },
        PI,
        TAU,
        true,
        1,
        0,
    );
    (s, vec![equator, rim])
}

/// **A pole in the band of a great carrier's plane leaves the loop
/// unread** (review of PR 4573, R1 N2) — the residual
/// `MaterialSign::Unencoded` documents beside its two faces. A pole
/// whose offset from a great (or near-great) carrier's plane lands in
/// the ambiguity band escalates `props_sphere_pole_plane`, and the
/// other pole, its antipode, is the same distance from that plane on
/// the other side, so it escalates too: neither pole is read, and the
/// loop answers `Unencoded` though the face holds neither. The face
/// then measures under its bit alone, the wrong bit included.
#[test]
fn a_pole_in_the_band_of_a_great_carriers_plane_leaves_the_loop_unread() {
    let b = band();
    let th = 0.5_f64;
    let off = (b.zero() * b.escalate()).sqrt() / RS;
    let (s, edges) = half_cap_with_a_pole_off_its_equator(th, off);
    assert_eq!(
        boundary_material_sign(&s, &edges, b),
        Ok(MaterialSign::Unencoded)
    );
    let face = RS * RS * PI * (1.0 - th.cos());
    let complement = 4.0 * PI * RS * RS - face;
    for (sense, want) in [(true, face), (false, complement)] {
        let fc = curved_face(&s, &edges, sense, b).expect("measured under its bit");
        assert!(
            (fc.area - want).abs() < 1e-9 * want,
            "sense {sense}: area {} != {want}",
            fc.area
        );
    }
}

/// **A pole near the loop, but past the band, encodes the side.** The
/// same half-cap with its chart pole ten times the escalation band off
/// its equator edge: the pole reads right of the loop, its antipode
/// too, so the face is the left side, measured under that bit and
/// refused under the other.
#[test]
fn a_pole_near_but_off_the_loop_encodes_the_side() {
    let b = band();
    let th = 0.5_f64;
    let off = 10.0 * b.escalate() / RS;
    let (s, edges) = half_cap_with_a_pole_off_its_equator(th, off);
    assert_eq!(
        boundary_material_sign(&s, &edges, b),
        Ok(MaterialSign::Encoded(Sign::Positive))
    );
    let face = RS * RS * PI * (1.0 - th.cos());
    let fc = curved_face(&s, &edges, true, b).expect("the half-cap");
    assert!(
        (fc.area - face).abs() < 1e-9 * face,
        "area {} != {face}",
        fc.area
    );
    assert_eq!(
        curved_face(&s, &edges, false, b).map(|c| c.area),
        Err(PropsError::SenseContradicted)
    );
}
