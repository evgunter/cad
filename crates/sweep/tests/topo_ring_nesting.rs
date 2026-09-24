//! Tier 3's check 9 states ring-INSIDE-outer, measured from an outside
//! consumer's seat: every body here is built through the public doors a
//! user would use — a `Profile` with holes, `extrude`, and the Euler
//! operators `mfkrh`/`kfmrh` — rather than by editing arenas.
//!
//! Each row is a pair. The honest body must VALIDATE, because a
//! nesting decide that refuses a valid model is the failure to fear;
//! the same body with one ringed face's host and guest roles inverted
//! must be refused by name, on that face and that loop, because a row
//! whose honest half alone is green cannot see the arm switched off.
//!
//! Where the arm is silent the pair asserts the silence in BOTH
//! directions: that is check 9's stated residue, and a residue nothing
//! measures is a claim.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point2, Tol};
use profile::{Profile, ProfileLoop, ProfileVertex, RawLoop, SketchPlane};
use sweep::{Extrusion, extrude};
use topo::{Body, FaceKey, FaceSurface, LoopKey, ValidationError};

fn tol() -> Tol {
    Tol::witness()
}

/// A plate: an outer loop and any number of holes, each a run of
/// `(x, y, bulge)`, extruded `h` along the sketch normal.
fn plate(loops: &[&[(f64, f64, f64)]], h: f64) -> Body<f64> {
    let loops = loops
        .iter()
        .map(|lp| {
            ProfileLoop::new(
                lp.iter()
                    .map(|&(x, y, b)| ProfileVertex::new(Point2::new(x, y), b))
                    .collect(),
            )
        })
        .collect();
    let profile = Profile::new(SketchPlane::xy(), loops)
        .validate(tol())
        .expect("a valid profile");
    extrude(&profile, Extrusion::Distance(h), tol())
        .expect("the plate extrudes")
        .body
}

fn rect(x0: f64, y0: f64, x1: f64, y1: f64) -> Vec<(f64, f64, f64)> {
    vec![(x0, y0, 0.0), (x1, y0, 0.0), (x1, y1, 0.0), (x0, y1, 0.0)]
}

/// A circle as one loop of two semicircular arcs.
fn circle(cx: f64, cy: f64, r: f64) -> Vec<(f64, f64, f64)> {
    vec![(cx - r, cy, 1.0), (cx + r, cy, 1.0)]
}

/// Check 9's four words in a structural report.
fn check_9_words(body: &Body<f64>) -> Vec<String> {
    match topo::validate_geometric_structural(body, tol()) {
        Ok(()) => Vec::new(),
        Err(errors) => errors
            .iter()
            .filter(|e| {
                matches!(
                    e,
                    ValidationError::RingMeetsOuter { .. }
                        | ValidationError::RingContactEscalated { .. }
                        | ValidationError::RingOutsideOuter { .. }
                        | ValidationError::RingNestingUndecided { .. }
                )
            })
            .map(|e| format!("{e:?}"))
            .collect(),
    }
}

fn first_ringed(body: &Body<f64>) -> (FaceKey, LoopKey, Vec<LoopKey>) {
    body.faces()
        .filter(|(_, f)| !f.rings.is_empty())
        .map(|(k, f)| (k, f.outer, f.rings.clone()))
        .next()
        .expect("a ring-bearing face")
}

/// The inverted glue, through the doors: promote the ring to a face on
/// the same chart (`mfkrh`), then glue the ORIGINAL face into it
/// (`kfmrh`) so the larger boundary becomes a ring of the smaller
/// face. The new face's sense is flipped, which is what makes both
/// loops role-correct in the inverted assignment — and therefore what
/// keeps check 6 silent.
fn invert_the_glue(body: &Body<f64>) -> (Body<f64>, FaceKey, LoopKey) {
    let mut out = body.clone();
    let (face, outer, rings) = first_ringed(&out);
    let (surface, sense) = {
        let f = out.get_face(face).unwrap();
        (f.surface, f.sense)
    };
    // Any further rings leave first, so the glue has one pair to make.
    for &extra in rings.iter().skip(1) {
        let made = out
            .mfkrh(extra, FaceSurface::Shared(surface))
            .expect("the extra ring promotes");
        out.set_face_sense(made.face, !sense).expect("its sense");
    }
    let made = out
        .mfkrh(rings[0], FaceSurface::Shared(surface))
        .expect("the ring promotes to a face");
    if !matches!(
        out.get_surface(out.get_face(made.face).unwrap().surface),
        Some(geom::Surface::Plane { .. })
    ) {
        out.set_face_surface(made.face, FaceSurface::Shared(surface))
            .expect("the promoted face shares the chart");
    }
    let glued = out.kfmrh(made.face, face).expect("the inverted glue");
    assert_eq!(glued.ring, outer, "the old outer loop is now the ring");
    out.set_face_sense(made.face, !sense).expect("the flip");
    (out, made.face, outer)
}

/// The honest body validates and check 9 is silent on it; the inverted
/// glue is refused by name when `decided`, and silent in both
/// directions when it is not (the gate's stated residue).
fn nested_then_inverted(name: &str, body: &Body<f64>, decided: bool) {
    let words = check_9_words(body);
    assert!(
        words.is_empty(),
        "[{name}] check 9 refused a valid body: {words:?}"
    );
    assert_eq!(
        topo::validate_geometric(body, tol()),
        Ok(()),
        "[{name}] the honest body validates"
    );
    let (inverted, face, outer) = invert_the_glue(body);
    let words = check_9_words(&inverted);
    if decided {
        assert!(
            words.iter().any(|w| w.contains("RingOutsideOuter")
                && w.contains(&format!("{face:?}"))
                && w.contains(&format!("{outer:?}"))),
            "[{name}] the inverted glue must be refused by name; got {words:?}"
        );
    } else {
        assert!(
            words.is_empty(),
            "[{name}] the arm's residue: it must be SILENT here; got {words:?}"
        );
    }
}

// ---- the arm decides, and refuses no valid body -------------------------

#[test]
fn a_plate_with_a_hole_at_one_end() {
    let body = plate(
        &[&rect(0.0, 0.0, 10.0, 0.2), &rect(9.6, 0.05, 9.7, 0.15)],
        0.1,
    );
    nested_then_inverted("10 x 0.2 plate, end hole", &body, true);
}

#[test]
fn a_ring_beside_a_concave_corner() {
    let l = vec![
        (0.0, 0.0, 0.0),
        (4.0, 0.0, 0.0),
        (4.0, 1.0, 0.0),
        (1.0, 1.0, 0.0),
        (1.0, 4.0, 0.0),
        (0.0, 4.0, 0.0),
    ];
    let body = plate(&[&l, &rect(0.1, 1.1, 0.9, 1.9)], 0.5);
    nested_then_inverted("L-plate, ring up the reflex corner", &body, true);
    let body = plate(&[&l, &rect(1.1, 0.1, 1.9, 0.9)], 0.5);
    nested_then_inverted("L-plate, ring along the reflex corner", &body, true);
}

#[test]
fn a_ring_in_a_long_thin_face() {
    let body = plate(
        &[
            &rect(0.0, 0.0, 20.0, 0.05),
            &rect(19.9, 0.015, 19.92, 0.035),
        ],
        0.05,
    );
    nested_then_inverted("400:1 plate, hole at the end", &body, true);
    let body = plate(
        &[
            &rect(0.0, 0.0, 20.0, 0.05),
            &rect(9.99, 0.015, 10.01, 0.035),
        ],
        0.05,
    );
    nested_then_inverted("400:1 plate, hole at the middle", &body, true);
}

#[test]
fn two_rings_on_one_face() {
    let body = plate(
        &[
            &rect(0.0, 0.0, 4.0, 2.0),
            &rect(0.5, 0.5, 1.0, 1.0),
            &rect(3.0, 1.0, 3.5, 1.5),
        ],
        0.5,
    );
    nested_then_inverted("two-ring plate", &body, true);
}

/// **A bowed end on the outer loop does not cost the pair its
/// refusal.** The honest face's outer loop bears an arc, so check 9's
/// nesting arm is silent on it — the polygon through an arc-bearing
/// loop's vertices is a proper region but not the LOOP's region, and
/// an arm that refuses a body on `Out` may not read one from it. The
/// INVERTED body's outer loop is the rectangular hole, which is all
/// lines, so the inverted glue is still refused by name. The gate
/// itself is asserted directly crate-side, in
/// `validate::tests::an_arc_bearing_outer_loop_is_the_gates_residue`.
#[test]
fn a_plate_with_one_rounded_end_still_refuses_its_inversion() {
    let outer = vec![
        (0.0, 0.0, 0.0),
        (3.0, 0.0, 0.5),
        (3.0, 1.0, 0.0),
        (0.0, 1.0, 0.0),
    ];
    let body = plate(&[&outer, &rect(1.0, 0.3, 1.5, 0.7)], 0.4);
    nested_then_inverted("bowed-end plate, one arc over four vertices", &body, true);
}

// ---- the gate's residue, measured in both directions ---------------------

/// **The disc class is a residue too, and it is the one with a known
/// widening.** A loop every edge of which is an arc of ONE circle has
/// a region the parity polygon does not express — the polygon through
/// two semicircle endpoints has zero area — so the arm says nothing
/// rather than refusing a valid body. `boolean::contain`'s `disc_side`
/// decides that class exactly and reaching it from tier 3 is the
/// widening `work/atrest/check-9-nesting-is-line-bounded-only.md` holds.
#[test]
fn a_disc_outer_loop_is_the_gates_residue() {
    let body = plate(&[&circle(0.0, 0.0, 2.0), &circle(0.0, 0.0, 0.5)], 0.3);
    nested_then_inverted("annulus face: both loops discs", &body, false);
    // A polygonal outer loop with a round hole IS decided; it is the
    // INVERSION of that body, whose outer loop becomes the circle,
    // that lands in the disc class.
    let body = plate(&[&rect(-2.0, -2.0, 2.0, 2.0), &circle(0.0, 0.0, 0.5)], 0.3);
    nested_then_inverted("round hole in a square plate", &body, false);
}

// ---- orientation ---------------------------------------------------------

/// **`revert` does not move the finding.** It reverses every cycle and
/// flips every sense; the chart normal reaches the containment walk
/// unmultiplied by the face's sense, and that walk's verdict is
/// invariant under the normal's sign. The WITNESS may move — the walk
/// stops at the first vertex in cycle order that reads outside, and
/// reversing the cycle changes which that is — so the comparison here
/// is by variant, as `RingOutsideOuter`'s own doc says it must be.
#[test]
fn revert_does_not_move_the_verdict() {
    for (name, body) in [
        (
            "end-hole plate",
            plate(
                &[&rect(0.0, 0.0, 10.0, 0.2), &rect(9.6, 0.05, 9.7, 0.15)],
                0.1,
            ),
        ),
        (
            "holed box",
            plate(
                &[&rect(0.0, 0.0, 1.0, 1.0), &rect(0.25, 0.25, 0.75, 0.75)],
                1.0,
            ),
        ),
    ] {
        let (inverted, _, _) = invert_the_glue(&body);
        for (tag, body) in [("honest", body.clone()), ("inverted", inverted)] {
            let variants = |b: &Body<f64>| -> Vec<String> {
                check_9_words(b)
                    .iter()
                    .map(|w| w.split_whitespace().next().unwrap_or("").to_string())
                    .collect()
            };
            let reverted = body.revert().expect("the body reverts");
            assert_eq!(
                variants(&body),
                variants(&reverted),
                "[{name}/{tag}] revert moved check 9's verdict"
            );
        }
    }
}
