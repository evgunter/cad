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
//! The outer-loop classes the arm decides — no arc, and one circle —
//! are measured by such pairs. The classes it is silent on (arcs over
//! three or more vertices, and arcs over fewer that are not one
//! circle) are measured here too, as a pair whose honest body
//! validates and whose inversion, with the silent class as its outer
//! loop, draws no check-9 word at all: that is check 9's stated
//! residue, and a residue nothing measures is a claim. The
//! `ArcParity` gate is also asserted crate-side, in
//! `validate::tests::an_arc_bearing_outer_loop_is_the_gates_residue`.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point2, Tol, Vec2};
use profile::{Profile, ProfileLoop, ProfileVertex, RawLoop, SketchPlane};
use sweep::{Extrusion, Revolution, RevolveAxis, extrude, revolve};
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
/// glue is refused by name, on that face and that loop.
fn nested_then_inverted(name: &str, body: &Body<f64>) {
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
    assert!(
        words.iter().any(|w| w.contains("RingOutsideOuter")
            && w.contains(&format!("{face:?}"))
            && w.contains(&format!("{outer:?}"))),
        "[{name}] the inverted glue must be refused by name; got {words:?}"
    );
}

// ---- the arm decides, and refuses no valid body -------------------------

#[test]
fn a_plate_with_a_hole_at_one_end() {
    let body = plate(
        &[&rect(0.0, 0.0, 10.0, 0.2), &rect(9.6, 0.05, 9.7, 0.15)],
        0.1,
    );
    nested_then_inverted("10 x 0.2 plate, end hole", &body);
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
    nested_then_inverted("L-plate, ring up the reflex corner", &body);
    let body = plate(&[&l, &rect(1.1, 0.1, 1.9, 0.9)], 0.5);
    nested_then_inverted("L-plate, ring along the reflex corner", &body);
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
    nested_then_inverted("400:1 plate, hole at the end", &body);
    let body = plate(
        &[
            &rect(0.0, 0.0, 20.0, 0.05),
            &rect(9.99, 0.015, 10.01, 0.035),
        ],
        0.05,
    );
    nested_then_inverted("400:1 plate, hole at the middle", &body);
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
    nested_then_inverted("two-ring plate", &body);
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
    nested_then_inverted("bowed-end plate, one arc over four vertices", &body);
}

// ---- the disc class: every edge of the outer loop an arc of one circle ---

/// **An annular face is decided, in both directions.** A loop every
/// edge of which is an arc of ONE circle bounds that circle's disc,
/// which the parity polygon does not express — the polygon through two
/// semicircle endpoints has zero area — and which `boolean::contain`'s
/// `disc_side` decides exactly. The honest annulus validates; its
/// inversion, whose outer loop is the small circle and whose ring is
/// the large one, is refused by name.
#[test]
fn an_annular_face_is_decided() {
    let body = plate(&[&circle(0.0, 0.0, 2.0), &circle(0.0, 0.0, 0.5)], 0.3);
    nested_then_inverted("annulus face: both loops discs", &body);
    // An off-centre hole: the two circles do not share a centre, so
    // the inversion's radial margin is taken about the hole's centre,
    // not about the origin its ring (the large circle) is centred on.
    let body = plate(&[&circle(0.0, 0.0, 2.0), &circle(1.2, 0.3, 0.4)], 0.3);
    nested_then_inverted("annulus face, eccentric hole", &body);
    // A polygonal outer loop with a round hole: the honest face is the
    // polygon class; its INVERSION, whose outer loop becomes the
    // circle, is the disc class, and a square ring's corners lie
    // outside it.
    let body = plate(&[&rect(-2.0, -2.0, 2.0, 2.0), &circle(0.0, 0.0, 0.5)], 0.3);
    nested_then_inverted("round hole in a square plate", &body);
    // The converse: a disc outer loop holding a POLYGONAL ring. The
    // honest face is decided through the disc; its inversion's outer
    // loop is the square.
    let body = plate(&[&circle(0.0, 0.0, 2.0), &rect(-0.5, -0.5, 0.5, 0.5)], 0.3);
    nested_then_inverted("square hole in a round plate", &body);
}

/// **A hole as close to its outer loop as a clear hole can be at this
/// run's ε certifies**, built the way a user would: the `Profile`
/// door, then `extrude`. The gap is `4·K·ε` — past the band's
/// escalation threshold `K·ε`, and down to `4e-11` m at `ε = 1e-12`.
/// Check 9's contact half decides each placement by a margin of
/// exactly that gap: a round hole beside a round rim (two whole
/// circles, one inside the other, whose vertices also sit `4·K·ε`
/// apart), and a round hole beside a straight edge (the hole's arc
/// against the edge's line). A contact arm that read a near miss as a
/// crossing or a tangency would refuse a body the profile door
/// accepted.
#[test]
fn a_hole_one_clear_gap_from_its_rim_certifies() {
    let gap = 4.0 * tol().k() * tol().eps();
    for (name, body) in [
        (
            "round hole beside a round rim",
            plate(&[&circle(0.0, 0.0, 5.0), &circle(4.0 - gap, 0.0, 1.0)], 0.3),
        ),
        (
            "round hole beside a straight edge",
            plate(
                &[&rect(0.0, 0.0, 10.0, 10.0), &circle(9.0 - gap, 5.0, 1.0)],
                0.3,
            ),
        ),
    ] {
        let words = check_9_words(&body);
        assert!(
            words.is_empty(),
            "[{name}] check 9 refused a clear hole: {words:?}"
        );
        assert_eq!(
            topo::validate_geometric(&body, tol()),
            Ok(()),
            "[{name}] the body validates"
        );
    }
}

/// **The two classes the arm is silent on stay silent, in both
/// directions.** A rectangular plate carrying a hole whose loop bears
/// arcs and is not one circle: the honest body is decided (its outer
/// loop is the rectangle) and validates; its inversion's outer loop is
/// the hole, whose region no exact instrument expresses, so the ring
/// lying outside it draws no check-9 word. A half-disc (an arc and its
/// chord, two vertices) is `NoWalk`; a slot (two bowed ends, arcs of two
/// different circles meeting the flanks at a corner, four vertices) is
/// `ArcParity`. What closes
/// both is `work/atrest/check-9-nesting-arc-parity-and-no-walk-wait-on-the-arc-aware-walk`.
#[test]
fn the_silent_classes_are_silent_in_both_directions() {
    let half_disc = vec![(-1.0, 0.0, 0.0), (1.0, 0.0, 1.0)];
    let slot = vec![
        (-0.5, -0.2, 0.0),
        (0.5, -0.2, 0.5),
        (0.5, 0.2, 0.0),
        (-0.5, 0.2, 0.5),
    ];
    for (name, hole) in [("NoWalk: half-disc", half_disc), ("ArcParity: slot", slot)] {
        let body = plate(&[&rect(-2.0, -2.0, 2.0, 2.0), &hole], 0.3);
        let words = check_9_words(&body);
        assert!(words.is_empty(), "[{name}] honest: {words:?}");
        assert_eq!(
            topo::validate_geometric(&body, tol()),
            Ok(()),
            "[{name}] the honest body validates"
        );
        let (inverted, _, _) = invert_the_glue(&body);
        let words = check_9_words(&inverted);
        assert!(
            words.is_empty(),
            "[{name}] the arm's residue: the inversion must draw no check-9 word; got {words:?}"
        );
    }
}

/// **Every shelled vessel of revolution carries an annular rim, and it
/// certifies.** The shape the disc class is named for: a revolved cup
/// opened through `shell_open`, whose mouth is the annulus between the
/// wall's two radii. A false refusal here would cost every such part,
/// so the honest cup must validate; the rim with its host and guest
/// roles inverted — the pick `shell_open`'s glue must not make — is
/// refused by name at rest.
#[test]
fn a_shelled_vessel_of_revolution_certifies_and_its_inverted_rim_does_not() {
    let (r, h, t) = (1.0, 2.0, 0.2);
    let meridian = ProfileLoop::new(vec![
        ProfileVertex::new(Point2::new(0.0, 0.0), 0.0),
        ProfileVertex::new(Point2::new(r, 0.0), 0.0),
        ProfileVertex::new(Point2::new(r, h), 0.0),
        ProfileVertex::new(Point2::new(0.0, h), 0.0),
    ]);
    let profile = Profile::new(SketchPlane::xy(), vec![meridian])
        .validate(tol())
        .expect("the meridian profile");
    let vessel = revolve(
        &profile,
        RevolveAxis {
            origin: Point2::new(0.0, 0.0),
            dir: Vec2::new(0.0, 1.0),
        },
        Revolution::Full,
        tol(),
    )
    .expect("the meridian revolves")
    .body;
    // The axis is `y`: the top cap is every planar face at `y = h`.
    let top: Vec<FaceKey> = vessel
        .faces()
        .filter(|(_, f)| {
            matches!(vessel.get_surface(f.surface),
                Some(geom::Surface::Plane { origin, .. }) if (origin.y - h).abs() < 1e-9)
        })
        .map(|(k, _)| k)
        .collect();
    let cup = topo::shell_open(&vessel, t, &top, tol())
        .expect("the vessel opens")
        .body;
    let ringed: Vec<FaceKey> = cup
        .faces()
        .filter(|(_, f)| !f.rings.is_empty())
        .map(|(k, _)| k)
        .collect();
    assert_eq!(ringed.len(), 1, "one ringed face: the rim");
    nested_then_inverted("shelled revolved cup, its rim", &cup);
}

// ---- orientation ---------------------------------------------------------

/// **`revert` does not move the finding.** It reverses every cycle and
/// flips every sense; the chart normal reaches the containment walk
/// unmultiplied by the face's sense, and that walk's verdict is
/// invariant under the normal's sign — and the disc class's radial
/// decide reads no orientation at all. The WITNESS may move — the walk
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
        (
            "annulus: the disc class",
            plate(&[&circle(0.0, 0.0, 2.0), &circle(0.4, -0.3, 0.5)], 0.3),
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
                variants(&body).is_empty(),
                tag == "honest",
                "[{name}/{tag}] the honest body is silent and its inversion refused"
            );
            assert_eq!(
                variants(&body),
                variants(&reverted),
                "[{name}/{tag}] revert moved check 9's verdict"
            );
        }
    }
}
