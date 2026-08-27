//! The classic boolean leg (M3 PR 5 heritage): the die and the table.
//! Planar-only (F5) — every operand is a box, so every op runs the
//! planar lanes; curved-operand booleans are M5.
//!
//! Every scene carries an EXACT expected volume (box arithmetic) via
//! `booleans::check`. This oracle first caught (pre-PR-5-fix-pass) a
//! silent wrong-component defect on single-face seams; the PR 5 fix
//! pass made every failure a typed refusal, and PR 5.5's seam
//! discipline closed the R1 orientation-dependent refusals entirely.
//!
//! The open box and void box stops retired at the #91 refresh: the
//! project box (`crate::projectbox`) is the cavity story now, and the
//! cutaway split (`crate::cutaway`) shows interiors honestly — the
//! two-shell void story stays as narration ([`voidbox_narration`]).
//!
//! All boxes come from ONE helper (`slab`) so that wherever
//! coincidence is intended, the coincident planes arise from
//! bit-identical shared values.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use pncad::geom_core::{Point3, Vec3};
use pncad::profile::{Profile, SketchPlane};
use pncad::sweep::{Extrusion, extrude};
use pncad::topo::{Body, BooleanResultKind};

use crate::booleans::{Verdict, check, describe, expect_seamed};
use crate::scalar::Scalar;
use crate::{SceneBody, Stop, View};
use pncad::geom_core::Tol;

/// The one box builder: axis-aligned `[x0,x1] x [y0,y1] x [z0,z1]`,
/// a rectangle on a z-offset xy sketch plane extruded up.
pub fn slab<S: Scalar>(x: (f64, f64), y: (f64, f64), z: (f64, f64), tol: Tol) -> Body<S> {
    // Algebra-authored (LIB-U2 PR-2): the same four corners, said as
    // a `line_to` chain closing at `Start`.
    let lp = crate::paths::path_polygon(&[(x.0, y.0), (x.1, y.0), (x.1, y.1), (x.0, y.1)], tol);
    let plane = SketchPlane::from_frame(
        Point3::new(S::from_f64(0.0), S::from_f64(0.0), S::from_f64(z.0)),
        Vec3::new(S::from_f64(1.0), S::from_f64(0.0), S::from_f64(0.0)),
        Vec3::new(S::from_f64(0.0), S::from_f64(1.0), S::from_f64(0.0)),
    );
    let profile = Profile::new(plane, vec![lp])
        .validate(tol)
        .expect("slab profile validation");
    // Raw extrude output IS boolean-consumable since PR 5's
    // extrude-operand description remap (proven by the PR 5.5 review's
    // die e2e).
    extrude(&profile, Extrusion::Distance(S::from_f64(z.1 - z.0)), tol)
        .expect("extrude slab")
        .body
}

// ---------------------------------------------------------------
// The die — 21 pip pockets across all six faces.
// ---------------------------------------------------------------

/// The full die: [-1,1]^3 cube minus 21 square pip pockets (0.25 span,
/// 0.125 deep, centers on the {−0.5, 0, 0.5} face grid, opposite faces
/// summing to 7), one sequential subtract per pip with the exact
/// dyadic volume oracle asserted after every op (pip volume 1/128).
/// Pips are straight square pockets (extrude is straight-only;
/// spherical pips await M5 curved booleans — the ratified OQ6
/// acceptance target). Stays translation-authored by ruling.
pub(crate) fn die<S: Scalar>(tol: Tol) -> (pncad::topo::BooleanBody<S>, String) {
    const H: f64 = 0.125; // pip half-span; pocket 0.25 square, 0.125 deep
    const IN: (f64, f64) = (0.875, 1.5); // pocket span outward through +face
    const OUT: (f64, f64) = (-1.5, -0.875); // …and through the −face
    let (n, z, p) = (-0.5, 0.0, 0.5); // the pip-center grid
    let one = [(z, z)].to_vec();
    let two = [(n, n), (p, p)].to_vec();
    let three = [(n, n), (z, z), (p, p)].to_vec();
    let four = [(n, n), (n, p), (p, n), (p, p)].to_vec();
    let five = [(n, n), (n, p), (p, n), (p, p), (z, z)].to_vec();
    let six = [(n, n), (n, z), (n, p), (p, n), (p, z), (p, p)].to_vec();
    type PipBox<S> = fn((f64, f64), (f64, f64), Tol) -> Body<S>;
    let px: PipBox<S> = |a, b, tol| slab(IN, a, b, tol);
    let nx: PipBox<S> = |a, b, tol| slab(OUT, a, b, tol);
    let py: PipBox<S> = |a, b, tol| slab(a, IN, b, tol);
    let ny: PipBox<S> = |a, b, tol| slab(a, OUT, b, tol);
    let pz: PipBox<S> = |a, b, tol| slab(a, b, IN, tol);
    let nz: PipBox<S> = |a, b, tol| slab(a, b, OUT, tol);
    type Face<'a, S> = (&'a str, Vec<(f64, f64)>, PipBox<S>);
    let faces: [Face<'_, S>; 6] = [
        ("+z=1", one, pz),
        ("-z=6", six, nz),
        ("+x=2", two, px),
        ("-x=5", five, nx),
        ("+y=3", three, py),
        ("-y=4", four, ny),
    ];
    let cube = slab((-1.0, 1.0), (-1.0, 1.0), (-1.0, 1.0), tol);
    let mut acc: Option<pncad::topo::BooleanBody<S>> = None;
    let mut pips = 0u32;
    for (label, centers, make) in faces {
        for (a, b) in centers {
            pips += 1;
            let exp = 8.0 - f64::from(pips) * 0.25 * 0.25 * 0.125;
            let pip = make((a - H, a + H), (b - H, b + H), tol);
            let base = acc.as_ref().map_or(&cube, |bb| &bb.body);
            acc = Some(expect_seamed(
                &format!("die pip {pips} ({label})"),
                check(crate::booleans::try_subtract(base, &pip, tol), exp, tol),
                exp,
            ));
        }
    }
    assert_eq!(pips, 21);
    let acc = acc.expect("21 pips");
    let note = format!(
        "21 sequential single-ring pip subtracts across ALL SIX faces (opposite \
         faces sum to 7), volume matches the dyadic oracle after every op \
         (observed bit-exact, gated 1e-9), final V = {}; tier 3' runs \
         on the RESULT with its declared contacts (the #91 tier-3' modernization)",
        8.0 - 21.0 * 0.25 * 0.25 * 0.125
    );
    (acc, note)
}

// ---------------------------------------------------------------
// The table.
// ---------------------------------------------------------------

/// Shared construction values: tabletop extents, leg half-width, and
/// the corner coordinates the legs share with the top.
const TOP_X: (f64, f64) = (-2.0, 2.0);
const TOP_Y: (f64, f64) = (-1.4, 1.4);
const TOP_Z: (f64, f64) = (1.4, 1.7);
const LEG_HALF: f64 = 0.13;

/// A leg centered at `(cx, cy)`, floor to `z_top`.
fn leg<S: Scalar>(cx: f64, cy: f64, z_top: f64, tol: Tol) -> Body<S> {
    slab(
        (cx - LEG_HALF, cx + LEG_HALF),
        (cy - LEG_HALF, cy + LEG_HALF),
        (0.0, z_top),
        tol,
    )
}

/// The table: tabletop ∪ 4 legs (4 sequential union nodes). Three
/// variants are attempted in honesty order:
///  1. leg INSET under the top, its top face EXACTLY coplanar with the
///     underside (shared value `TOP_Z.0`) — the Eq 15.3 coplanar lane;
///  2. leg inset and overlapping 0.05 INTO the top (proper
///     intersection);
///  3. legs straddling the top's CORNERS (shared values `TOP_X.1`
///     etc.), so each seam crosses the underside AND two side faces.
///
/// All three are narrated LIVE from their actual outcomes; since M4
/// PR 5 the SHIPPED variant is the TRUE corner-ALIGNED table (Evan,
/// PR #71 — the first `demo_tripwires.rs` wire FIRED): each leg's
/// outer faces lie exactly in the top's side planes, DECLARED per
/// union, and the declared rung glues them — every later union sees
/// maximal operands. Attempt 1 (coplanar touch, undeclared) now
/// refuses at the coincidence door — rung (b), value equality never
/// classifies — and the narration says so.
pub(crate) fn table<S: Scalar>(tol: Tol) -> (pncad::topo::BooleanBody<S>, String) {
    let top: Body<S> = slab(TOP_X, TOP_Y, TOP_Z, tol);
    let leg_vol = |z_top: f64| (2.0 * LEG_HALF) * (2.0 * LEG_HALF) * z_top;

    // Attempt 1: exact-coplanar touching, leg inset under the top.
    let (icx, icy) = (TOP_X.1 - 0.45, TOP_Y.1 - 0.45);
    let coplanar = check(
        crate::booleans::try_union(&top, &leg(icx, icy, TOP_Z.0, tol), tol),
        top_vol() + leg_vol(TOP_Z.0),
        tol,
    );
    // Attempt 2: inset leg overlapping 0.05 into the top.
    let overlap = check(
        crate::booleans::try_union(&top, &leg(icx, icy, TOP_Z.0 + 0.05, tol), tol),
        top_vol() + leg_vol(TOP_Z.0),
        tol,
    );
    // Attempt 3 (the pre-PR 5 shipping variant, kept as narration):
    // one corner-straddling leg — still works, no declarations needed
    // (transversal seams only).
    let z_top = TOP_Z.0 + 0.05;
    let per_straddle_gain = leg_vol(z_top) - LEG_HALF * LEG_HALF * 0.05;
    let straddle = check(
        crate::booleans::try_union(&top, &leg(TOP_X.1, TOP_Y.1, z_top, tol), tol),
        top_vol() + per_straddle_gain,
        tol,
    );

    // SHIPPED (M4 PR 5, the fired table wire): TRUE corner-aligned
    // legs — outer faces exactly IN the top's side planes, the flush
    // contacts DECLARED per union, glued by the declared rung.
    let aligned_leg = |cx: f64, cy: f64| {
        let (x0, x1) = if cx > 0.0 {
            (cx - 2.0 * LEG_HALF, cx)
        } else {
            (cx, cx + 2.0 * LEG_HALF)
        };
        let (y0, y1) = if cy > 0.0 {
            (cy - 2.0 * LEG_HALF, cy)
        } else {
            (cy, cy + 2.0 * LEG_HALF)
        };
        slab((x0, x1), (y0, y1), (0.0, z_top), tol)
    };
    let corners = [
        (TOP_X.1, TOP_Y.1),
        (TOP_X.1, TOP_Y.0),
        (TOP_X.0, TOP_Y.1),
        (TOP_X.0, TOP_Y.0),
    ];
    // Each aligned leg: FULL footprint under the top, overlapping
    // 0.05 up into it.
    let per_leg_gain = leg_vol(z_top) - (2.0 * LEG_HALF) * (2.0 * LEG_HALF) * 0.05;
    let mut acc: Option<pncad::topo::BooleanBody<S>> = None;
    let mut expected = top_vol();
    for (cx, cy) in corners {
        expected += per_leg_gain;
        let base = acc.as_ref().map_or(&top, |b| &b.body);
        acc = Some(expect_seamed(
            "corner-ALIGNED leg union (declared)",
            check(
                crate::booleans::try_union_declared(base, &aligned_leg(cx, cy), tol),
                expected,
                tol,
            ),
            expected,
        ));
    }
    let note = format!(
        "variants narrated live — (1) leg EXACTLY coplanar-touching the underside, \
         UNDECLARED (shared value {}): {}; (2) leg inset, overlapping 0.05 into the \
         top: {}; (3) the pre-PR 5 straddle workaround: {}; (4) SHIPPED: TRUE \
         corner-ALIGNED legs, flush side planes DECLARED per union (M4 PR 5) — the \
         straddle narration is retired",
        TOP_Z.0,
        describe(&coplanar, top_vol() + leg_vol(TOP_Z.0)),
        describe(&overlap, top_vol() + leg_vol(TOP_Z.0)),
        describe(&straddle, top_vol() + per_straddle_gain),
    );
    (acc.expect("four legs"), note)
}

fn top_vol() -> f64 {
    (TOP_X.1 - TOP_X.0) * (TOP_Y.1 - TOP_Y.0) * (TOP_Z.1 - TOP_Z.0)
}

// ---------------------------------------------------------------
// The void box: narration only since the #91 refresh.
// ---------------------------------------------------------------

/// The two-shell void story, kept as live narration: inner box
/// STRICTLY inside, no boundary crossing, so the subtract takes the
/// containment fallback and yields the two-shell `Voided` body — the
/// milestone's first legitimate voids. Not a render (an opaque void
/// is indistinguishable from a cube; the old translucency hack is
/// retired in favor of `crate::cutaway`'s honest split), and STEP
/// refuses void shells typed — also narrated.
pub fn voidbox_narration(tol: Tol) {
    println!("\n== voidbox (narration only since the #91 refresh) ==");
    let b = voidbox::<f64>(tol);
    println!(
        "   kind Voided, TWO shells (outer + reverted inner); volume = 8 - 1 = 7 \
         exactly; the void is INTERNAL — the montage panel is retired for the \
         cutaway split, which shows interiors honestly"
    );
    match pncad::step_export::step_string(&b.body, &pncad::step_export::StepOptions::default(), tol)
    {
        Ok(_) => {
            println!("   STEP export of the voided body: OK (unexpected — update this narration)");
        }
        Err(e) => println!("   STEP export of the voided body refuses typed: {e:?}"),
    }
}

/// The checked void subtract itself, scalar-generic (the Probe sweep
/// samples its containment-fallback predicates too; the STEP-refusal
/// narration stays in the f64-only wrapper above — the STEP writer is
/// f64 by design).
pub(crate) fn voidbox<S: Scalar>(tol: Tol) -> pncad::topo::BooleanBody<S> {
    let outer: Body<S> = slab((0.0, 2.0), (0.0, 2.0), (0.0, 2.0), tol);
    let inner: Body<S> = slab((0.5, 1.5), (0.5, 1.5), (0.5, 1.5), tol);
    match check(crate::booleans::try_subtract(&outer, &inner, tol), 7.0, tol) {
        Verdict::Good(b, _) => {
            assert_eq!(b.kind, BooleanResultKind::Voided);
            assert_eq!(b.body.shells().count(), 2);
            b
        }
        v => panic!("void subtract failed: {}", describe(&v, 7.0)),
    }
}

/// The classic boolean stops, in tour order.
pub fn stops(tol: Tol) -> Vec<Stop> {
    let (die_bb, die_note) = die::<f64>(tol);
    let (table_bb, table_note) = table::<f64>(tol);
    vec![
        Stop {
            name: "die",
            caption: String::new(),
            // Montage cell RETIRED by the M6 curation unit. Its unique
            // content is 21 SEQUENTIAL planar subtracts with seamed
            // single-ring pockets — and chaining DEPTH is not a visual
            // property; `plate`'s holes already show the rings. The
            // fixture keeps every other role it has (corpus document,
            // latency row, STEP golden, standalone render); only the
            // sheet cell goes. `diepips` is the sheet's die now.
            montage: false,
            story: "the die: 21 pip pockets across all six faces (opposite faces sum to 7)",
            ops: "extrude 22 boxes -> 21 sequential subtract nodes (Seamed single-ring pockets)",
            delta: 1e-2,
            note: Some(die_note),
            view: View {
                elev: 28.0,
                azim: -55.0,
                up: 'z',
            },
            bodies: vec![SceneBody::seamed(
                "die",
                [0.88, 0.86, 0.80],
                die_bb.body,
                die_bb.contacts,
            )],
        },
        Stop {
            name: "table",
            caption: String::new(),
            montage: true,
            story: "a table: tabletop unioned with four corner-straddling legs",
            ops: "extrude 5 boxes (one shared builder) -> 4 sequential union nodes (Seamed)",
            delta: 1e-2,
            note: Some(table_note),
            view: View {
                elev: 18.0,
                azim: -55.0,
                up: 'z',
            },
            bodies: vec![SceneBody::seamed(
                "table",
                [0.62, 0.45, 0.28],
                table_bb.body,
                table_bb.contacts,
            )],
        },
    ]
}
