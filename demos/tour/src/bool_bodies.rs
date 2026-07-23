//! The boolean leg of the tour (M3 PR 5): bodies built by composing
//! extruded boxes through the public boolean ops. Planar-only (F5) —
//! every operand here is a box, so every op runs the planar lanes;
//! curved-operand booleans are M5.
//!
//! Every scene carries an EXACT expected volume (box arithmetic), and
//! the builders check the ops' results against it. This oracle first
//! caught (pre-PR-5-fix-pass) a silent wrong-component defect on
//! single-face seams; the PR 5 fix pass made every failure a typed
//! refusal, and PR 5.5's seam discipline closed the R1
//! orientation-dependent refusals entirely — the full 21-pip [`die`]
//! (this leg's original blocked goal) now builds with the exact volume
//! after every op. Remaining refusals (boundary-on-boundary seams,
//! reflex-corner tilted crossings) are typed and narrated where the
//! tour touches them.
//!
//! All boxes come from ONE helper (`slab`) so that wherever
//! coincidence is intended (the table's coplanar attempts), the
//! coincident planes arise from bit-identical shared values.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point3, Tolerance, Vec3};
use profile::{Profile, ProfileLoop, SketchPlane};
use sweep::{Extrusion, extrude};
use topo::{Body, BooleanError, BooleanResult, BooleanResultKind};

use crate::Stop;
use crate::booleans;

fn p2(x: f64, y: f64) -> geom_core::Point2<f64> {
    geom_core::Point2::new(x, y)
}

/// The one box builder: axis-aligned `[x0,x1] x [y0,y1] x [z0,z1]`,
/// a rectangle on a z-offset xy sketch plane extruded up.
fn slab(x: (f64, f64), y: (f64, f64), z: (f64, f64)) -> Body<f64> {
    let lp = ProfileLoop::polygon([p2(x.0, y.0), p2(x.1, y.0), p2(x.1, y.1), p2(x.0, y.1)]);
    let plane = SketchPlane::from_frame(
        Point3::new(0.0, 0.0, z.0),
        Vec3::new(1.0, 0.0, 0.0),
        Vec3::new(0.0, 1.0, 0.0),
    );
    let profile = Profile::new(plane, vec![lp])
        .validate(Tolerance::get())
        .expect("slab profile validation");
    // Raw extrude output IS boolean-consumable since PR 5's
    // extrude-operand description remap (proven by the PR 5.5 review's
    // die e2e); the chord-normalization workaround is retired.
    extrude(&profile, Extrusion::Distance(z.1 - z.0))
        .expect("extrude slab")
        .body
}

/// The oracle: volume of a boolean result vs the exact box-arithmetic
/// expectation. `Good` carries the body; the two failure shapes carry
/// what actually happened, for narration.
// Size skew vs the slim failure variants is inherent (same posture as
// the kernel's own `BooleanResult`).
#[allow(clippy::large_enum_variant)]
enum Verdict {
    Good(Body<f64>, BooleanResultKind),
    /// Op "succeeded" (tier 1+2 legal) with the WRONG volume — the
    /// silent wrong-component defect.
    Wrong(f64, BooleanResultKind),
    Refused(BooleanError),
}

fn check(r: Result<BooleanResult<f64>, BooleanError>, expected: f64) -> Verdict {
    match r {
        Ok(BooleanResult::Body(b)) => {
            let v = topo::mass_properties(&b.body)
                .expect("mass properties")
                .volume;
            if (v - expected).abs() <= 1e-9 {
                Verdict::Good(b.body, b.kind)
            } else {
                Verdict::Wrong(v, b.kind)
            }
        }
        Ok(BooleanResult::Empty) => Verdict::Refused(BooleanError::UnrepresentableResult),
        Err(e) => Verdict::Refused(e),
    }
}

fn describe(v: &Verdict, expected: f64) -> String {
    match v {
        Verdict::Good(_, kind) => format!("OK (kind {kind:?}, volume exact {expected})"),
        Verdict::Wrong(vol, kind) => format!(
            "SILENT WRONG RESULT (kind {kind:?}): tier 1+2 passed but volume = {vol} \
             instead of {expected} — caught by the tour's volume oracle"
        ),
        Verdict::Refused(e) => format!("typed refusal (fail-loud): {e:?}"),
    }
}

// ---------------------------------------------------------------
// Stop 7: the die — 21 pip pockets across all six faces.
// ---------------------------------------------------------------

/// The full die: [-1,1]^3 cube minus 21 square pip pockets (0.25 span,
/// 0.125 deep, centers on the {−0.5, 0, 0.5} face grid, opposite faces
/// summing to 7), one sequential subtract per pip with the exact
/// dyadic volume oracle asserted after every op (pip volume 1/128).
/// This stop's ancestor (`die_blocked`) demonstrated the R1
/// orientation-dependent refusal matrix live and carried a
/// self-promoting guard; PR 5.5's seam discipline closed R1 on all six
/// orientations and the guard fired — this is the promotion. Pips are
/// straight square pockets (extrude is straight-only; spherical pips
/// await M5 curved booleans).
fn die() -> (Body<f64>, Option<String>) {
    const H: f64 = 0.125; // pip half-span; pocket 0.25 square, 0.125 deep
    const IN: (f64, f64) = (0.875, 1.5); // pocket span outward through +face
    const OUT: (f64, f64) = (-1.5, -0.875); // …and through the −face
    let (n, z, p) = (-0.5, 0.0, 0.5); // the pip-center grid
    // Standard die layouts, opposite faces summing to 7.
    let one = [(z, z)];
    let two = [(n, n), (p, p)];
    let three = [(n, n), (z, z), (p, p)];
    let four = [(n, n), (n, p), (p, n), (p, p)];
    let five = [(n, n), (n, p), (p, n), (p, p), (z, z)];
    let six = [(n, n), (n, z), (n, p), (p, n), (p, z), (p, p)];
    // (face label, pip boxes) — (a, b) are the in-face pip centers.
    let faces: [(&str, Vec<Body<f64>>); 6] = [
        (
            "+z=1",
            one.iter()
                .map(|&(a, b)| slab((a - H, a + H), (b - H, b + H), IN))
                .collect(),
        ),
        (
            "-z=6",
            six.iter()
                .map(|&(a, b)| slab((a - H, a + H), (b - H, b + H), OUT))
                .collect(),
        ),
        (
            "+x=2",
            two.iter()
                .map(|&(a, b)| slab(IN, (a - H, a + H), (b - H, b + H)))
                .collect(),
        ),
        (
            "-x=5",
            five.iter()
                .map(|&(a, b)| slab(OUT, (a - H, a + H), (b - H, b + H)))
                .collect(),
        ),
        (
            "+y=3",
            three
                .iter()
                .map(|&(a, b)| slab((a - H, a + H), IN, (b - H, b + H)))
                .collect(),
        ),
        (
            "-y=4",
            four.iter()
                .map(|&(a, b)| slab((a - H, a + H), OUT, (b - H, b + H)))
                .collect(),
        ),
    ];
    let mut acc = slab((-1.0, 1.0), (-1.0, 1.0), (-1.0, 1.0));
    let mut pips = 0u32;
    for (label, pip_boxes) in faces {
        for pip in pip_boxes {
            pips += 1;
            let exp = 8.0 - f64::from(pips) * 0.25 * 0.25 * 0.125;
            acc = match check(booleans::try_subtract(&acc, &pip), exp) {
                Verdict::Good(b, kind) => {
                    assert_eq!(kind, BooleanResultKind::Seamed);
                    b
                }
                v => panic!("die pip {pips} ({label}) failed: {}", describe(&v, exp)),
            };
        }
    }
    assert_eq!(pips, 21);
    let note = format!(
        "21 sequential single-ring pip subtracts across ALL SIX faces (opposite \
         faces sum to 7), exact volume after every op, final V = {} — blocked \
         pre-PR 5.5 by the R1 orientation-dependent refusals; the ancestor \
         die_blocked's self-promoting guard fired on the PR 5 fix pass and \
         PR 5.5 closed the rest",
        8.0 - 21.0 * 0.25 * 0.25 * 0.125
    );
    (acc, Some(note))
}

// ---------------------------------------------------------------
// Stop 8: the table.
// ---------------------------------------------------------------

/// Shared construction values: tabletop extents, leg half-width, and
/// the corner coordinates the legs share with the top.
const TOP_X: (f64, f64) = (-2.0, 2.0);
const TOP_Y: (f64, f64) = (-1.4, 1.4);
const TOP_Z: (f64, f64) = (1.4, 1.7);
const LEG_HALF: f64 = 0.13;

/// A leg centered at `(cx, cy)`, floor to `z_top`.
fn leg(cx: f64, cy: f64, z_top: f64) -> Body<f64> {
    slab(
        (cx - LEG_HALF, cx + LEG_HALF),
        (cy - LEG_HALF, cy + LEG_HALF),
        (0.0, z_top),
    )
}

/// The table: tabletop ∪ 4 legs (4 sequential union nodes). Three
/// variants are attempted in honesty order:
///  1. leg INSET under the top, its top face EXACTLY coplanar with the
///     underside (shared value `TOP_Z.0`) — the Eq 15.3 coplanar lane;
///  2. leg inset and overlapping 0.05 INTO the top (proper
///     intersection) — but the seam ring closes within the −z
///     underside face, a single-ring pocket on a REFUSING R1
///     orientation ({−z, +x, +y}), so it refuses `SeamOrientation`;
///  3. legs straddling the top's CORNERS (shared values `TOP_X.1`
///     etc.), so each seam crosses the underside AND two side faces.
///
/// The first correct variant ships; the attempts are narrated.
fn table() -> (Body<f64>, Option<String>) {
    let top = slab(TOP_X, TOP_Y, TOP_Z);
    let leg_vol = |z_top: f64| (2.0 * LEG_HALF) * (2.0 * LEG_HALF) * z_top;

    // Attempt 1: exact-coplanar touching, leg inset under the top.
    let (icx, icy) = (TOP_X.1 - 0.45, TOP_Y.1 - 0.45);
    let coplanar = check(
        booleans::try_union(&top, &leg(icx, icy, TOP_Z.0)),
        top_vol() + leg_vol(TOP_Z.0),
    );
    // Attempt 2: inset leg overlapping 0.05 into the top.
    let overlap = check(
        booleans::try_union(&top, &leg(icx, icy, TOP_Z.0 + 0.05)),
        top_vol() + leg_vol(TOP_Z.0),
    );
    // Attempt 3: corner-straddling legs (the shipping variant).
    let z_top = TOP_Z.0 + 0.05;
    let corners = [
        (TOP_X.1, TOP_Y.1),
        (TOP_X.1, TOP_Y.0),
        (TOP_X.0, TOP_Y.1),
        (TOP_X.0, TOP_Y.0),
    ];
    // Each leg: quarter footprint under the top, overlapping 0.05 up.
    let per_leg_gain = leg_vol(z_top) - LEG_HALF * LEG_HALF * 0.05;
    let mut body = top;
    let mut expected = top_vol();
    for (cx, cy) in corners {
        expected += per_leg_gain;
        match check(booleans::try_union(&body, &leg(cx, cy, z_top)), expected) {
            Verdict::Good(b, kind) => {
                assert_eq!(kind, BooleanResultKind::Seamed);
                body = b;
            }
            v => panic!(
                "corner-straddle leg union failed: {}",
                describe(&v, expected)
            ),
        }
    }
    let note = format!(
        "three variants attempted — (1) leg EXACTLY coplanar-touching the underside \
         (shared value {}): {}; (2) leg inset, overlapping 0.05 into the top: {}; \
         (3) SHIPPED: legs straddling the top's corners (shared corner values), \
         each seam crossing underside + two side faces: exact",
        TOP_Z.0,
        describe(&coplanar, top_vol() + leg_vol(TOP_Z.0)),
        describe(&overlap, top_vol() + leg_vol(TOP_Z.0)),
    );
    (body, Some(note))
}

fn top_vol() -> f64 {
    (TOP_X.1 - TOP_X.0) * (TOP_Y.1 - TOP_Y.0) * (TOP_Z.1 - TOP_Z.0)
}

// ---------------------------------------------------------------
// Stop 9: the open box.
// ---------------------------------------------------------------

/// Open box: outer minus a fully-interior cavity cutter, opening only
/// through the +z top face — a single-ring pocket. Refused before the
/// PR 5 fix pass (a multi-face "scoop" variant shipped in its place
/// until the post-5.5 refresh); a rendered stop since.
fn open_box() -> (Body<f64>, Option<String>) {
    let outer = slab((-1.0, 1.0), (-1.0, 1.0), (0.0, 1.2));
    let interior = slab((-0.8, 0.8), (-0.8, 0.8), (0.25, 1.5));
    let want_box = 4.8 - 1.6 * 1.6 * 0.95;
    match check(booleans::try_subtract(&outer, &interior), want_box) {
        Verdict::Good(body, kind) => {
            assert_eq!(kind, BooleanResultKind::Seamed);
            let note = format!(
                "the PURE open box: cutter interior to the +z top face — a single-ring \
                 pocket on a WORKING R1 orientation ({{+z, −x, −y}}); refused before the \
                 PR 5 fix pass, a rendered stop since (volume exact {want_box})"
            );
            (body, Some(note))
        }
        v => panic!("pure open-box subtract failed: {}", describe(&v, want_box)),
    }
}

// ---------------------------------------------------------------
// Stop 10: the void box (+ the cutaway attempt).
// ---------------------------------------------------------------

/// Void box: the inner box is STRICTLY inside — no boundary crossing,
/// so the op takes the containment fallback and yields the two-shell
/// `Voided` body: outer shell + reverted inner void shell. The
/// milestone's first legitimate voids. Volume = outer − inner exactly.
fn void_box() -> (Body<f64>, Option<String>) {
    let outer = slab((0.0, 2.0), (0.0, 2.0), (0.0, 2.0));
    let inner = slab((0.5, 1.5), (0.5, 1.5), (0.5, 1.5));
    match check(booleans::try_subtract(&outer, &inner), 7.0) {
        Verdict::Good(body, kind) => {
            assert_eq!(kind, BooleanResultKind::Voided);
            assert_eq!(body.shells().count(), 2);
            (
                body,
                Some(
                    "kind Voided, TWO shells (outer + reverted inner); the void is \
                     INTERNAL — invisible from outside, the render shows a plain cube; \
                     volume = outer − inner = 8 − 1 = 7 exactly"
                        .to_string(),
                ),
            )
        }
        v => panic!("void subtract failed: {}", describe(&v, 7.0)),
    }
}

/// Cutaway attempt: subtract a corner-covering brick from the
/// two-shell voided body so the internal void becomes visible. A
/// genuine stress test (multi-shell operand A, knife through both
/// shells); the outcome is narrated either way.
fn void_box_cutaway(voided: &Body<f64>) -> Option<(Body<f64>, Option<String>)> {
    let knife = slab((1.0, 2.2), (-0.2, 1.0), (-0.2, 2.2));
    let expected = 7.0 - (2.0 - 0.25); // knife ∩ material = 2 − 0.25
    let v = check(booleans::try_subtract(voided, &knife), expected);
    match v {
        Verdict::Good(body, kind) => Some((
            body,
            Some(format!(
                "the VOIDED two-shell body minus a corner brick (kind {kind:?}): the cut \
                 runs through both shells, opening the internal void to view"
            )),
        )),
        _ => {
            println!(
                "   voidbox cutaway (knife through BOTH shells of the two-shell body): {}\n\
                 \x20  — skipping the extra render; the void stays narration-only",
                describe(&v, expected)
            );
            None
        }
    }
}

/// The boolean stops, in tour order (appended after the sweep six).
pub fn stops() -> Vec<Stop> {
    let (die_body, die_note) = die();
    let (table_body, table_note) = table();
    let (open_body, open_note) = open_box();
    let (void_body, void_note) = void_box();
    let cutaway = void_box_cutaway(&void_body);
    let mut stops = vec![
        Stop {
            name: "die",
            story: "the die: 21 pip pockets across all six faces (opposite faces sum to 7)",
            ops: "extrude 22 boxes -> 21 sequential subtract nodes (Seamed single-ring pockets)",
            delta: 1e-2,
            seamed: true,
            note: die_note,
            body: die_body,
        },
        Stop {
            name: "table",
            story: "a table: tabletop unioned with four corner-straddling legs",
            ops: "extrude 5 boxes (one shared builder) -> 4 sequential union nodes (Seamed)",
            delta: 1e-2,
            seamed: true,
            note: table_note,
            body: table_body,
        },
        Stop {
            name: "openbox",
            story: "an open box: interior cavity subtracted, open only through the top",
            ops: "extrude 2 boxes -> 1 subtract node (Seamed single-ring pocket)",
            delta: 1e-2,
            seamed: true,
            note: open_note,
            body: open_body,
        },
        Stop {
            name: "voidbox",
            story: "the void box: inner box strictly inside — the first legitimate voids",
            ops: "extrude 2 boxes -> 1 subtract node (containment fallback, kind Voided)",
            delta: 1e-2,
            seamed: true,
            note: void_note,
            body: void_body,
        },
    ];
    if let Some((body, note)) = cutaway {
        stops.push(Stop {
            name: "voidbox_cutaway",
            story: "the void box cut open: corner brick subtracted from the two-shell body",
            ops: "voidbox result -> 1 more subtract node (multi-shell operand A)",
            delta: 1e-2,
            seamed: true,
            note,
            body,
        });
    }
    stops
}
