//! The boolean leg of the tour (M3 PR 5): bodies built by composing
//! extruded boxes through the public boolean ops. Planar-only (F5) —
//! every operand here is a box, so every op runs the planar lanes;
//! curved-operand booleans are M5.
//!
//! Every scene carries an EXACT expected volume (box arithmetic), and
//! the builders check the ops' results against it. This oracle first
//! caught (pre-PR-5-fix-pass) a silent wrong-component defect on
//! single-face seams; the fix pass resolved it — single-ring pockets
//! now either return the EXACT body or REFUSE with a typed error, never
//! a silent wrong body. What remains is orientation-dependent (review
//! finding R1, see [`die_blocked`]): an identical single-face pip
//! pocket succeeds on a brick's {+z, −x, −y} faces and refuses
//! `SeamOrientation` on {−z, +x, +y}. The tour narrates each attempt
//! honestly (working orientation vs. typed refusal) and ships the
//! variants whose seams are exact.
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
    let mut body = extrude(&profile, Extrusion::Distance(z.1 - z.0))
        .expect("extrude slab")
        .body;
    // Boolean-operand posture: downgrade the extruder's Intersection
    // edge descriptions to self-contained chords (booleans module
    // docs — the DanglingDescription finding).
    booleans::normalize_edges_to_chords(&mut body);
    body
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
// Stop 7: the die — PARTIALLY blocked (R1 orientation-dependent), live.
// ---------------------------------------------------------------

/// The die needs pip pockets on all six faces. Post PR-5 fix-pass the
/// single-ring pocket lane is ORIENTATION-DEPENDENT (review finding R1):
/// an identical blind pip pocket SUCCEEDS with the exact volume on a
/// brick's {+z, −x, −y} faces and REFUSES `SeamOrientation` on
/// {−z, +x, +y} — a handedness-correlated HALF of face orientations
/// (root cause: the cross-solid null-edge orientation discipline /
/// `choose_roles`' prefer-mirror heuristic has no consistency theorem,
/// which is PR 5.5's charter). A real die pips all six faces, so half
/// still refuse — no full-die STL yet. Crucially, the refusals are now
/// TYPED and LOUD: the pre-fix-pass silent wrong-component defect this
/// branch first caught (it kept the inverted cut-out fragment) is fixed
/// — every attempt is either an exact-volume `Seamed` body or a typed
/// refusal. The self-promoting guard below asserts a {−z, +x, +y} pip
/// STILL refuses, so the demo fails the day the full die is buildable —
/// promote it then.
pub fn die_blocked() {
    println!("\n== die (partially blocked: orientation-dependent pip pockets) ==");
    println!("   recipe: [-1,1]^3 cube minus 21 pip pockets, opposite faces summing to 7;");
    println!("   pips as straight square pockets (extrude is straight-only; spherical pips");
    println!("   await M5 curved booleans).");
    let cube = slab((-1.0, 1.0), (-1.0, 1.0), (-1.0, 1.0));
    let expected = 8.0 - 0.34 * 0.34 * 0.18;
    // The R1 orientation matrix, live: the identical 0.34-square,
    // 0.18-deep blind pip pocket on each of the six faces (single
    // subtract from a fresh cube). Works on {+z, −x, −y}; refuses
    // SeamOrientation on {−z, +x, +y}.
    let pips: [(&str, Body<f64>); 6] = [
        ("+z", slab((-0.17, 0.17), (-0.17, 0.17), (0.82, 1.25))),
        ("-x", slab((-1.25, -0.82), (-0.17, 0.17), (-0.17, 0.17))),
        ("-y", slab((-0.17, 0.17), (-1.25, -0.82), (-0.17, 0.17))),
        ("-z", slab((-0.17, 0.17), (-0.17, 0.17), (-1.25, -0.82))),
        ("+x", slab((0.82, 1.25), (-0.17, 0.17), (-0.17, 0.17))),
        ("+y", slab((-0.17, 0.17), (0.82, 1.25), (-0.17, 0.17))),
    ];
    for (name, pip) in &pips {
        let v = check(booleans::try_subtract(&cube, pip), expected);
        println!("   pip pocket on {name}: {}", describe(&v, expected));
    }
    // The three working faces DO compose: a genuine three-face pocketed
    // cube with exact volume (the promotion payload the day the other
    // half lands).
    let mut acc = cube.clone();
    let mut exp = 8.0;
    for (_, pip) in [&pips[0], &pips[1], &pips[2]] {
        exp -= 0.34 * 0.34 * 0.18;
        acc = match check(booleans::try_subtract(&acc, pip), exp) {
            Verdict::Good(b, _) => b,
            v => panic!(
                "working-face pocket composition regressed: {}",
                describe(&v, exp)
            ),
        };
    }
    let _ = acc;
    println!("   the three working faces compose: cube with 3 pockets, V = {exp} exact");
    // Through-pillar: a double-ring single-face seam (both {+z, −z}
    // rings) — refuses SeamOrientation.
    let pillar = slab((-0.17, 0.17), (-0.17, 0.17), (-1.25, 1.25));
    let through = check(
        booleans::try_subtract(&cube, &pillar),
        8.0 - 0.34 * 0.34 * 2.0,
    );
    println!(
        "   through-pillar (two single-face rings): {}",
        describe(&through, 8.0 - 0.34 * 0.34 * 2.0)
    );
    println!("   verdict: single-ring pips WORK on {{+z, −x, −y}} and REFUSE SeamOrientation");
    println!("   on {{−z, +x, +y}} (R1, orientation-dependent half) — no full-die STL until");
    println!("   the refusing half lands (PR 5.5); refusals are typed and loud, never silent.");
    // Self-promoting guard: a {−z, +x, +y} pip MUST still refuse. The
    // day it succeeds, a full die is buildable — promote the die.
    let refusing = check(booleans::try_subtract(&cube, &pips[3].1), expected); // −z
    assert!(
        matches!(
            refusing,
            Verdict::Refused(BooleanError::SeamOrientation { .. })
        ),
        "the −z single-ring pip no longer refuses SeamOrientation — the R1 refusing \
         half is fixed and a full die is buildable: promote the die to a real stop!"
    );
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
/// through the +z top face — a single-ring pocket on a WORKING R1
/// orientation ({+z, −x, −y}). Refused before the PR 5 fix pass (the
/// scoop variant below shipped in its place); a rendered stop since.
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

/// The scooped container: same outer box, but the cutter ALSO overhangs
/// the +y wall — the seam crosses top, +y side, and underside-of-rim
/// faces (a multi-face through-cut), a different seam class from
/// `open_box`'s single-ring pocket, and the variant that shipped while
/// the pure box was R1-refused.
fn scoop_box() -> (Body<f64>, Option<String>) {
    let outer = slab((-1.0, 1.0), (-1.0, 1.0), (0.0, 1.2));
    let scoop_cutter = slab((-0.8, 0.8), (-0.8, 1.2), (0.25, 1.5));
    let want_scoop = 4.8 - 1.6 * 1.8 * 0.95;
    match check(booleans::try_subtract(&outer, &scoop_cutter), want_scoop) {
        Verdict::Good(body, kind) => {
            assert_eq!(kind, BooleanResultKind::Seamed);
            let note = format!(
                "cutter overhangs the +y wall: a multi-face through-cut seam — a \
                 different seam class from the single-ring openbox pocket (volume \
                 exact {want_scoop})"
            );
            (body, Some(note))
        }
        v => panic!("scoop subtract failed: {}", describe(&v, want_scoop)),
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
/// The die's defect demonstration prints first (it exports no STL).
pub fn stops() -> Vec<Stop> {
    die_blocked();
    let (table_body, table_note) = table();
    let (open_body, open_note) = open_box();
    let (scoop_body, scoop_note) = scoop_box();
    let (void_body, void_note) = void_box();
    let cutaway = void_box_cutaway(&void_body);
    let mut stops = vec![
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
            name: "scoopbox",
            story: "the scooped container: cavity cutter overhanging top and +y wall",
            ops: "extrude 2 boxes -> 1 subtract node (Seamed multi-face through-cut)",
            delta: 1e-2,
            seamed: true,
            note: scoop_note,
            body: scoop_body,
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
