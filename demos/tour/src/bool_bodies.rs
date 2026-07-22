//! The boolean leg of the tour (M3 PR 5): bodies built by composing
//! extruded boxes through the public boolean ops. Planar-only (F5) —
//! every operand here is a box, so every op runs the planar lanes;
//! curved-operand booleans are M5.
//!
//! Every scene carries an EXACT expected volume (box arithmetic), and
//! the builders check the ops' results against it — the oracle that
//! caught this branch's cookie-cutter defect (see [`die_blocked`]):
//! when the seam ring closes within a SINGLE face of an operand
//! (pocket, boss, through-pillar, inset leg), the finish stage keeps
//! the wrong component and returns it as a tier-1/2-legal body —
//! silently. The tour narrates each attempt honestly and ships the
//! variants whose seams cross multiple faces (which are exact).
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
// Stop 7: the die — BLOCKED on this branch, demonstrated live.
// ---------------------------------------------------------------

/// The die needs pip pockets: a pip box pierces exactly ONE cube face,
/// so its seam ring closes within that face — the cookie-cutter
/// configuration, which this branch's finish stage gets silently wrong
/// (it keeps the inverted cut-out fragment and discards the die).
/// There is no planar design-around: a pocket interior to a face is
/// single-face by nature, and raised pips (union boss) or through
/// pillars are the same configuration. So the die stop demonstrates
/// the defect with exact numbers instead of faking a die.
pub fn die_blocked() {
    println!("\n== die (blocked: the cookie-cutter defect, demonstrated) ==");
    println!("   the recipe: [-1,1]^3 cube minus 21 pip pockets (21 subtract nodes),");
    println!("   opposite faces summing to 7; pips as straight square pockets — the");
    println!("   sweep inventory's extrude is straight-only (frustum taper is a typed");
    println!("   refusal) and SPHERICAL pips await M5's curved booleans.");
    let cube = slab((-1.0, 1.0), (-1.0, 1.0), (-1.0, 1.0));
    // One pip: 0.34 square, 0.18 deep, protruding 0.25 out of the top.
    let pip = slab((-0.17, 0.17), (-0.17, 0.17), (0.82, 1.25));
    let expected = 8.0 - 0.34 * 0.34 * 0.18;
    let first = check(booleans::try_subtract(&cube, &pip), expected);
    println!(
        "   pip subtract (seam ring inside ONE face): {}",
        describe(&first, expected)
    );
    let pillar = slab((-0.17, 0.17), (-0.17, 0.17), (-1.25, 1.25));
    let through = check(
        booleans::try_subtract(&cube, &pillar),
        8.0 - 0.34 * 0.34 * 2.0,
    );
    println!(
        "   through-pillar subtract (two such rings): {}",
        describe(&through, 8.0 - 0.34 * 0.34 * 2.0)
    );
    println!("   verdict: pocket/boss/through-hole seams confined to single faces are a");
    println!("   PR 5 fix-pass item — no die STL from this branch; the scenes below use");
    println!("   seams that cross multiple faces, which are exact.");
    assert!(
        !matches!(first, Verdict::Good(..)),
        "cookie-cutter subtract now works — promote the die to a real stop!"
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
///     intersection) — but the seam ring still closes within the
///     underside face (cookie-cutter);
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

/// Open box: outer minus a cavity cutter protruding through the top.
/// The fully-interior cutter (opening only through the top face) is
/// the cookie-cutter configuration — attempted and narrated — so the
/// shipped variant's cutter ALSO overhangs the +y wall: the seam
/// crosses top, +y side, and underside-of-rim faces, and the result is
/// an open scoop (container open on top, one wall cut down).
fn open_box() -> (Body<f64>, Option<String>) {
    let outer = slab((-1.0, 1.0), (-1.0, 1.0), (0.0, 1.2));
    let interior = slab((-0.8, 0.8), (-0.8, 0.8), (0.25, 1.5));
    let want_box = 4.8 - 1.6 * 1.6 * 0.95;
    let pure = check(booleans::try_subtract(&outer, &interior), want_box);

    let scoop_cutter = slab((-0.8, 0.8), (-0.8, 1.2), (0.25, 1.5));
    let want_scoop = 4.8 - 1.6 * 1.8 * 0.95;
    match check(booleans::try_subtract(&outer, &scoop_cutter), want_scoop) {
        Verdict::Good(body, kind) => {
            assert_eq!(kind, BooleanResultKind::Seamed);
            let note = format!(
                "the pure open box (cutter interior to the top face) is cookie-cutter \
                 blocked: {}; SHIPPED variant: the cutter overhangs the +y wall too, a \
                 multi-face through-cut — an open container with one wall scooped down \
                 (volume exact {want_scoop})",
                describe(&pure, want_box)
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
            story: "an open container: cavity cutter subtracted through top and +y wall",
            ops: "extrude 2 boxes -> 1 subtract node (Seamed multi-face through-cut)",
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
