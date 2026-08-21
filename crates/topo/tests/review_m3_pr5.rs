//! M3 PR 5 adversarial review suite (booleans part 2). Falsification
//! targets: geometric role resolution (cookie-cutter family widened:
//! deep asymmetric pocket, side-face boss/pockets, two pockets in one
//! face, edge notch), refusal-boundary honesty (non-collinear versions
//! of the refused shapes must WORK; refusals must be deterministic and
//! typed with operands untouched), nearest-site collinear germ lines
//! (four sites on one line), `point_in_solid` on complements, voids,
//! and on-entity probes, and D9 replay through the graft door.
//! Every constructive fixture carries an EXACT dyadic volume/area
//! oracle (mass properties are exact on dyadic prisms).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod common;

use common::{flush_declarations, prism_z};
use geom_core::{Band, Decide, Point3};
use topo::{
    Body, BooleanBody, BooleanError, BooleanResult, BooleanResultKind, SolidContainment,
    mass_properties, point_in_solid, subtract, subtract_with, union_with, validate,
    validate_closed,
};
use geom_core::Tol;

fn brick<T: Decide>(x: (f64, f64), y: (f64, f64), z: (f64, f64)) -> Body<T> {
    prism_z::<T>(&[(x.0, y.0), (x.1, y.0), (x.1, y.1), (x.0, y.1)], z.0, z.1).body
}

/// The R2/R3 U-slab (nonconvex caps, two prongs).
fn uslab() -> Body<f64> {
    prism_z::<f64>(
        &[
            (0.0, 0.0),
            (3.0, 0.0),
            (3.0, 3.0),
            (2.0, 3.0),
            (2.0, 1.0),
            (1.0, 1.0),
            (1.0, 3.0),
            (0.0, 3.0),
        ],
        0.0,
        1.0,
    )
    .body
}

type BoolOp<T> =
    fn(&Body<T>, &Body<T>, &topo::BooleanDeclarations, Tol) -> Result<BooleanResult<T>, BooleanError>;

/// Functional run with the author's flush contacts declared (M4
/// PR 5): operands bitwise untouched, result tier-1+2 valid.
fn run<T: Decide>(op: BoolOp<T>, a: &Body<T>, b: &Body<T>) -> BooleanResult<T> {
    let (a0, b0) = (format!("{a:?}"), format!("{b:?}"));
    let r = op(a, b, &flush_declarations(a, b), Tol::witness()).unwrap();
    assert_eq!(format!("{a:?}"), a0, "operand A untouched");
    assert_eq!(format!("{b:?}"), b0, "operand B untouched");
    if let BooleanResult::Body(body) = &r {
        assert_eq!(validate(&body.body), Ok(()), "tier 1");
        assert_eq!(validate_closed(&body.body), Ok(()), "tier 2");
    }
    r
}

fn body_of<T: Decide>(r: &BooleanResult<T>) -> &BooleanBody<T> {
    match r {
        BooleanResult::Body(b) => b,
        BooleanResult::Empty => panic!("expected a body"),
    }
}

fn assert_props(body: &Body<f64>, volume: f64, area: f64) {
    let m = mass_properties(body, Tol::witness()).unwrap();
    assert_eq!(m.volume, volume, "exact volume");
    assert_eq!(m.surface_area, area, "exact area");
}

/// A refusal must be typed, deterministic, and leave operands bitwise
/// untouched — never a silent wrong body.
fn assert_typed_refusal<T: Decide>(op: BoolOp<T>, a: &Body<T>, b: &Body<T>) -> String {
    let (a0, b0) = (format!("{a:?}"), format!("{b:?}"));
    let d = flush_declarations(a, b);
    let e1 = op(a, b, &d, Tol::witness()).map(|_| ()).unwrap_err();
    let e2 = op(a, b, &d, Tol::witness()).map(|_| ()).unwrap_err();
    assert_eq!(format!("{a:?}"), a0, "operand A untouched by refusal");
    assert_eq!(format!("{b:?}"), b0, "operand B untouched by refusal");
    assert_eq!(
        format!("{e1:?}"),
        format!("{e2:?}"),
        "refusal deterministic"
    );
    format!("{e1:?}")
}

// =====================================================================
// Target 1: resolve_roles_geometric — widened cookie-cutter family.
// =====================================================================

/// Deep, strongly asymmetric blind pocket (nearly through): the IN
/// region is a long thin well, the probe geometry far from symmetric.
#[test]
fn deep_asymmetric_pocket_subtract() {
    let a = brick::<f64>((0.0, 2.0), (0.0, 2.0), (0.0, 2.0));
    let b = brick::<f64>((0.25, 0.75), (0.25, 1.75), (0.25, 2.5));
    let r = run(subtract_with, &a, &b);
    let body = body_of(&r);
    assert_eq!(body.kind, BooleanResultKind::Seamed);
    // vol = 8 − 0.5·1.5·1.75; area = 24 − 0.75 + walls 7 + floor 0.75.
    assert_props(&body.body, 8.0 - 1.3125, 31.0);
}

/// Boss/pocket on the −x face (a WORKING orientation per R1): the
/// axis-rotated cookie-cutter shapes carry exact oracles.
#[test]
fn boss_and_pocket_on_minus_x_face() {
    let a = brick::<f64>((0.0, 2.0), (0.0, 2.0), (0.0, 2.0));
    let b = brick::<f64>((-0.75, 0.5), (0.75, 1.25), (0.75, 1.25));
    let r = run(union_with, &a, &b);
    let body = body_of(&r);
    assert_eq!(body.kind, BooleanResultKind::Seamed);
    // vol = 8 + 0.75·0.25; area = 24 + 4·(0.5·0.75) + cap .25 − opening .25.
    assert_props(&body.body, 8.1875, 25.5);
    let r = run(subtract_with, &a, &b);
    let body = body_of(&r);
    assert_eq!(body.kind, BooleanResultKind::Seamed);
    // vol = 8 − 0.5·0.25; area = 24 − .25 + 4·(0.5·0.5) + floor .25.
    assert_props(&body.body, 7.875, 25.0);
}

/// R1 witness, +x face, FLIPPED by PR 5.5 (the ring-lane role rule):
/// the identical pillar into the +x face now succeeds with exact
/// oracles for BOTH ∖ and ∪ (formerly SeamOrientation).
#[test]
fn plus_x_face_cookie_refusals_pinned() {
    let a = brick::<f64>((0.0, 2.0), (0.0, 2.0), (0.0, 2.0));
    let b = brick::<f64>((1.5, 2.75), (0.75, 1.25), (0.75, 1.25));
    let r = run(subtract_with, &a, &b);
    let body = body_of(&r);
    assert_eq!(body.kind, BooleanResultKind::Seamed);
    // vol = 8 − 0.5·0.25; area = 24 − .25 + 4·(0.5·0.5) + floor .25.
    assert_props(&body.body, 7.875, 25.0);
    let r = run(union_with, &a, &b);
    let body = body_of(&r);
    assert_eq!(body.kind, BooleanResultKind::Seamed);
    // vol = 8 + 0.75·0.25; area = 24 + 4·(0.5·0.75) + cap .25 − opening .25.
    assert_props(&body.body, 8.1875, 25.5);
}

/// TWO pockets opened in ONE face of A by a single U-shaped B whose
/// prongs pierce the x = 2 face (web outside A): two seam rings closing
/// inside one face — the resolution must fix each ring's roles
/// independently and the joining must not cross-wire the rings.
#[test]
fn two_pockets_one_face_subtract() {
    let a = brick::<f64>((0.0, 2.0), (0.0, 2.0), (0.0, 2.0));
    // U opening toward +x, prongs piercing A's −x face (a WORKING
    // orientation per R1): prongs y∈[0.25,0.75] and [1.25,1.75],
    // x∈[−1,0.5]; web x∈[−1,−0.5].
    let b = prism_z::<f64>(
        &[
            (-1.0, 0.25),
            (0.5, 0.25),
            (0.5, 0.75),
            (-0.5, 0.75),
            (-0.5, 1.25),
            (0.5, 1.25),
            (0.5, 1.75),
            (-1.0, 1.75),
        ],
        0.75,
        1.25,
    )
    .body;
    let r = run(subtract_with, &a, &b);
    let body = body_of(&r);
    assert_eq!(body.kind, BooleanResultKind::Seamed);
    // Each pocket 0.5³; per pocket: walls 1.0 + floor .25 − opening .25.
    assert_props(&body.body, 8.0 - 0.25, 26.0);
}

/// One orientation-matrix row: face name + the pillar's x/y/z spans.
type PillarRow = (&'static str, (f64, f64), (f64, f64), (f64, f64));

/// The DECISIVE orientation matrix: the identical blind pocket cut
/// into each of A's six faces. Any Ok must carry the exact volume
/// (silent-wrong = falsified); refusals must be typed+deterministic.
/// Maps how far the cookie-cutter fix actually reaches.
#[test]
fn pocket_orientation_matrix() {
    let pillars: [PillarRow; 6] = [
        ("+z", (0.75, 1.25), (0.75, 1.25), (1.5, 2.5)),
        ("-z", (0.75, 1.25), (0.75, 1.25), (-0.5, 0.5)),
        ("+x", (1.5, 2.5), (0.75, 1.25), (0.75, 1.25)),
        ("-x", (-0.5, 0.5), (0.75, 1.25), (0.75, 1.25)),
        ("+y", (0.75, 1.25), (1.5, 2.5), (0.75, 1.25)),
        ("-y", (0.75, 1.25), (-0.5, 0.5), (0.75, 1.25)),
    ];
    let mut refused = Vec::new();
    for (name, x, y, z) in pillars {
        let a = brick::<f64>((0.0, 2.0), (0.0, 2.0), (0.0, 2.0));
        let b = brick::<f64>(x, y, z);
        match subtract_with(&a, &b, &flush_declarations(&a, &b), Tol::witness()) {
            Ok(BooleanResult::Body(body)) => {
                assert_eq!(validate_closed(&body.body), Ok(()), "{name}");
                let m = mass_properties(&body.body, Tol::witness()).unwrap();
                assert_eq!(m.volume, 7.875, "{name}: SILENT WRONG VOLUME");
                assert_eq!(m.surface_area, 25.0, "{name}: SILENT WRONG AREA");
            }
            Ok(BooleanResult::Empty) => panic!("{name}: empty"),
            Err(_) => {
                assert_typed_refusal(subtract_with, &a, &b);
                refused.push(name);
            }
        }
    }
    // R1 CLOSED (PR 5.5): all six orientations succeed with the exact
    // volume/area — the ring-lane role rule replaced the handedness-
    // correlated heuristic. Any refusal here is a regression.
    assert_eq!(refused, Vec::<&str>::new(), "R1: all orientations exact");
}

/// Pocket crossing an EDGE of A (notch across the x=2 / z=2 edge):
/// the seam is an open-crossing polygon spanning two faces, adjacent
/// to the edge-corner region — the "edge-adjacent corner" attack.
#[test]
fn edge_notch_subtract() {
    let a = brick::<f64>((0.0, 2.0), (0.0, 2.0), (0.0, 2.0));
    let b = brick::<f64>((1.5, 2.5), (0.5, 1.0), (1.5, 2.5));
    let r = run(subtract_with, &a, &b);
    let body = body_of(&r);
    assert_eq!(body.kind, BooleanResultKind::Seamed);
    // vol = 8 − 0.125; area = 24 − 0.5 + 4·0.25.
    assert_props(&body.body, 7.875, 24.5);
}

// =====================================================================
// Target 3: refusal-boundary honesty.
// =====================================================================

/// The clearly-non-collinear version of the Fig. 15.1 shapes WORKS:
/// same overlap footprint, B's caps pulled strictly off A's caps.
#[test]
fn fig151_noncoplanar_intersect_works() {
    let a = brick::<f64>((0.0, 2.0), (0.0, 2.0), (0.0, 1.0));
    let b = brick::<f64>((1.0, 3.0), (1.0, 3.0), (-0.25, 1.25));
    let r = run(topo::intersect_with, &a, &b);
    let body = body_of(&r);
    assert_eq!(body.kind, BooleanResultKind::Seamed);
    assert_props(&body.body, 1.0, 6.0);
}

/// Flush rest (B standing ON A's top face, face-in-face-interior
/// contact, zero overlap): FLIPPED to a firm success (PR 5.5 — it
/// already succeeded at PR 5's review as the "positive surprise"; the
/// honesty envelope is now an assertion): the interior face-rest ∪
/// goes through the SEAM pipeline and lands the fully-regularized
/// single-shell result — exact volume AND area, interface patch
/// consumed.
#[test]
fn flush_pillar_rest_union_honest() {
    let a = brick::<f64>((0.0, 2.0), (0.0, 2.0), (0.0, 2.0));
    let b = brick::<f64>((0.75, 1.25), (0.75, 1.25), (2.0, 3.0));
    let r = run(union_with, &a, &b);
    let body = body_of(&r);
    assert_eq!(body.kind, BooleanResultKind::Seamed);
    assert_eq!(body.body.shells().count(), 1);
    assert_props(&body.body, 8.25, 26.0);
    // ∖ and ∩ on the same rest are regularized-correct.
    let r = run(subtract_with, &a, &b);
    let body = body_of(&r);
    assert_eq!(body.kind, BooleanResultKind::OperandA);
    assert_props(&body.body, 8.0, 24.0);
    assert!(matches!(
        run(topo::intersect_with, &a, &b),
        BooleanResult::Empty
    ));
}

/// Corner-flush rest: the pillar's bottom rests flush at A's top-face
/// corner, its bottom edges partly collinear with A's top edges. The
/// PR 5.5 re-adjudication pinned this a typed boundary-on-boundary
/// refusal (`UnpairedLooseEnds { count: 4 }`); its own honesty form
/// ("if it ever succeeds it must be exact") is now the pin: M5 S1's
/// declared-REST zip glues the mate with the exact volume.
#[test]
fn corner_flush_pillar_union_honest() {
    let a = brick::<f64>((0.0, 2.0), (0.0, 2.0), (0.0, 2.0));
    let b = brick::<f64>((0.0, 0.5), (0.0, 0.5), (2.0, 3.0));
    match union_with(&a, &b, &flush_declarations(&a, &b), Tol::witness()) {
        Ok(BooleanResult::Body(body)) => {
            assert_eq!(validate_closed(&body.body), Ok(()));
            let m = mass_properties(&body.body, Tol::witness()).unwrap();
            assert_eq!(m.volume, 8.25, "corner-flush union volume");
        }
        Ok(BooleanResult::Empty) => panic!("cannot be empty"),
        Err(e) => panic!("M5 S1 glues the declared corner-flush rest: {e:?}"),
    }
}

/// The ε-perturbed neighbor of the flush rest (B sunk 1/16 into A)
/// must WORK — the refusal boundary is the exact-contact set only.
#[test]
fn perturbed_flush_pillar_union_works() {
    let a = brick::<f64>((0.0, 2.0), (0.0, 2.0), (0.0, 2.0));
    let b = brick::<f64>((0.75, 1.25), (0.75, 1.25), (1.9375, 3.0));
    let r = run(union_with, &a, &b);
    let body = body_of(&r);
    assert_eq!(body.kind, BooleanResultKind::Seamed);
    // vol = 8 + 0.25·1; area = 24 + 4·(0.5·1.0) + cap .25 − opening .25.
    assert_props(&body.body, 8.25, 26.0);
}

/// The near-flush STACKED union (full-footprint overlap shrunk to an
/// interior column): the perturbed neighbor of the refused
/// collinear-seam stack must WORK.
#[test]
fn interior_column_union_works() {
    let a = brick::<f64>((0.0, 2.0), (0.0, 2.0), (0.0, 2.0));
    let b = brick::<f64>((0.0625, 1.9375), (0.0625, 1.9375), (1.9375, 4.0));
    let r = run(union_with, &a, &b);
    let body = body_of(&r);
    assert_eq!(body.kind, BooleanResultKind::Seamed);
    // vol = 8 + 1.875²·2 = 15.03125; area = 24 − 1.875² + walls
    // 4·1.875·2 (exposed above z=2) + top 1.875² = 39.
    assert_props(&body.body, 15.03125, 39.0);
}

/// The full-overlap stack, both doors (this pin's third life):
/// PR 5.5 pinned the boundary-on-boundary DECLARED refusal
/// (`UnpairedLooseEnds`); M5 S1's declared-REST zip flipped it to an
/// exact success — the union is one brick, contact faces gone. The
/// still-refusing door is the UNDECLARED one (the coincidence
/// ladder), pinned deterministic and operand-preserving, exactly as
/// the refusal contract demands.
#[test]
fn pinned_refusals_deterministic() {
    let a = brick::<f64>((0.0, 2.0), (0.0, 2.0), (0.0, 2.0));
    let b = brick::<f64>((0.0, 2.0), (0.0, 2.0), (2.0, 4.0));
    // Undeclared: typed, deterministic, operands untouched.
    let (a0, b0) = (format!("{a:?}"), format!("{b:?}"));
    let e1 = topo::union(&a, &b, Tol::witness()).map(|_| ()).unwrap_err();
    let e2 = topo::union(&a, &b, Tol::witness()).map(|_| ()).unwrap_err();
    assert_eq!(format!("{a:?}"), a0, "operand A untouched by refusal");
    assert_eq!(format!("{b:?}"), b0, "operand B untouched by refusal");
    assert_eq!(
        format!("{e1:?}"),
        format!("{e2:?}"),
        "refusal deterministic"
    );
    assert!(
        format!("{e1:?}").contains("UndeclaredCoincidence"),
        "got {e1:?}"
    );
    // Declared: the M5 S1 REST zip glues the stack — exact volume and
    // area of the (0..2)²×(0..4) brick.
    let r = run(union_with, &a, &b);
    let body = body_of(&r);
    assert_eq!(body.kind, BooleanResultKind::Seamed);
    assert_props(&body.body, 16.0, 40.0);
}

/// NEW NAMED ACCEPTANCE (PR 5.5): the Fig 15.1 coplanar-overlap ∩ —
/// shared cap planes, seam partly on them — produces the exact
/// [1,2]²×[0,1] cube (the angular strut spike order nests the
/// corner-site chords).
#[test]
fn fig151_coplanar_overlap_intersect_exact() {
    let a = brick::<f64>((0.0, 2.0), (0.0, 2.0), (0.0, 1.0));
    let b = brick::<f64>((1.0, 3.0), (1.0, 3.0), (0.0, 1.0));
    let r = run(topo::intersect_with, &a, &b);
    let body = body_of(&r);
    assert_eq!(body.kind, BooleanResultKind::Seamed);
    assert_eq!(body.body.shells().count(), 1);
    assert_props(&body.body, 1.0, 6.0);
}

// =====================================================================
// Target 5: nearest-site discipline — four sites on ONE germ line.
// =====================================================================

/// A U-slab (nonconvex caps, one face) cut by a brick crossing BOTH
/// prongs: the cap-face/B-face germ lines each carry FOUR sites in two
/// collinear segments. Facing+nearest must chord 0–1 and 2–3, never
/// bridge the gap 1–2. Result: base+stubs shell plus two floating tip
/// shells, exact volume 6, area 30.
#[test]
fn collinear_four_site_seam_subtract() {
    let a = prism_z::<f64>(
        &[
            (0.0, 0.0),
            (3.0, 0.0),
            (3.0, 3.0),
            (2.0, 3.0),
            (2.0, 1.0),
            (1.0, 1.0),
            (1.0, 3.0),
            (0.0, 3.0),
        ],
        0.0,
        1.0,
    )
    .body;
    let b = brick::<f64>((-1.0, 4.0), (2.0, 2.5), (-0.5, 1.5));
    // R2 CLOSED (PR 5.5): the match-partner separation test (replacing
    // the record-sibling proxy) lets adjacent-site chords capture a
    // whole partner pair together; the 1–2 gap is still never bridged
    // (facing + nearest). Base + stubs shell and two floating tips.
    let r = run(subtract_with, &a, &b);
    let body = body_of(&r);
    assert_eq!(body.kind, BooleanResultKind::Seamed);
    assert_eq!(body.body.shells().count(), 3);
    assert_props(&body.body, 6.0, 30.0);
}

/// The SAME U-slab cut across only ONE prong (two sites per germ
/// line, still a nonconvex single cap face): isolates R2's cause —
/// this must work if the gap is specifically multi-segment
/// collinearity, not nonconvex faces.
#[test]
fn single_prong_cut_nonconvex_cap_subtract() {
    let a = prism_z::<f64>(
        &[
            (0.0, 0.0),
            (3.0, 0.0),
            (3.0, 3.0),
            (2.0, 3.0),
            (2.0, 1.0),
            (1.0, 1.0),
            (1.0, 3.0),
            (0.0, 3.0),
        ],
        0.0,
        1.0,
    )
    .body;
    let b = brick::<f64>((-1.0, 1.5), (2.0, 2.5), (-0.5, 1.5));
    // R3 CLOSED (PR 5.5): the crossing-polygon disconnection family
    // joins under the derived sense discipline — 2 shells (body +
    // severed tip), exact mass properties.
    let r = run(subtract_with, &a, &b);
    let body = body_of(&r);
    assert_eq!(body.kind, BooleanResultKind::Seamed);
    assert_eq!(body.body.shells().count(), 2);
    assert_props(&body.body, 6.5, 30.0);
}

// =====================================================================
// Target 4: point_in_solid — complements, voids, on-entity probes.
// =====================================================================

#[test]
fn point_in_solid_complement_and_void() {
    let band = Band::linear(Tol::witness()).unwrap();
    let cube = brick::<f64>((0.0, 1.0), (0.0, 1.0), (0.0, 1.0));
    let inside = Point3::new(0.5, 0.5, 0.5);
    let outside = Point3::new(2.0, 2.0, 2.0);
    assert_eq!(
        point_in_solid(&cube, inside, band, Tol::witness()).unwrap(),
        SolidContainment::In
    );
    assert_eq!(
        point_in_solid(&cube, outside, band, Tol::witness()).unwrap(),
        SolidContainment::Out
    );
    // Complement (reverted cube): material is everything OUTSIDE.
    let comp = cube.revert().unwrap();
    assert_eq!(
        point_in_solid(&comp, inside, band, Tol::witness()).unwrap(),
        SolidContainment::Out,
        "complement: cavity is not material"
    );
    assert_eq!(
        point_in_solid(&comp, outside, band, Tol::witness()).unwrap(),
        SolidContainment::In,
        "complement: at-infinity side is material"
    );
    // Voided body (3-cube minus unit cube): void is Out, wall is In.
    let a = brick::<f64>((0.0, 3.0), (0.0, 3.0), (0.0, 3.0));
    let b = brick::<f64>((1.0, 2.0), (1.0, 2.0), (1.0, 2.0));
    let r = subtract(&a, &b, Tol::witness()).unwrap();
    let voided = &body_of(&r).body;
    assert_eq!(
        point_in_solid(voided, Point3::new(1.5, 1.5, 1.5), band, Tol::witness()).unwrap(),
        SolidContainment::Out,
        "inside the void is not material"
    );
    assert_eq!(
        point_in_solid(voided, Point3::new(0.5, 1.5, 1.5), band, Tol::witness()).unwrap(),
        SolidContainment::In,
        "wall material"
    );
    assert_eq!(
        point_in_solid(voided, Point3::new(1.0, 1.5, 1.5), band, Tol::witness()).unwrap(),
        SolidContainment::OnBoundary,
        "void wall is boundary"
    );
}

#[test]
fn point_in_solid_on_entities_and_grazes() {
    let band = Band::linear(Tol::witness()).unwrap();
    let cube = brick::<f64>((0.0, 1.0), (0.0, 1.0), (0.0, 1.0));
    // On a face interior / an edge / a vertex: OnBoundary, typed.
    for q in [
        Point3::new(0.5, 0.5, 1.0),
        Point3::new(0.5, 1.0, 1.0),
        Point3::new(1.0, 1.0, 1.0),
    ] {
        assert_eq!(
            point_in_solid(&cube, q, band, Tol::witness()).unwrap(),
            SolidContainment::OnBoundary,
            "{q:?}"
        );
    }
    // Outside, +x ray aimed exactly at an edge then a vertex: the
    // graze-retry schedule must still classify Out (never hang, never
    // misreport).
    for q in [Point3::new(-1.0, 1.0, 1.0), Point3::new(-1.0, 0.0, 0.0)] {
        assert_eq!(
            point_in_solid(&cube, q, band, Tol::witness()).unwrap(),
            SolidContainment::Out,
            "{q:?}"
        );
    }
    // Determinism.
    let q = Point3::new(-1.0, 1.0, 1.0);
    let v1 = point_in_solid(&cube, q, band, Tol::witness()).unwrap();
    let v2 = point_in_solid(&cube, q, band, Tol::witness()).unwrap();
    assert_eq!(v1, v2);
}

// =====================================================================
// Targets 6/8: D9 replay through every result kind (incl. the graft
// door) and revert-oracle corpus extension.
// =====================================================================

#[test]
fn replay_deterministic_all_kinds() {
    // Seamed (pocket, exercises resolve_roles + zip).
    let a = brick::<f64>((0.0, 2.0), (0.0, 2.0), (0.0, 2.0));
    let b = brick::<f64>((0.75, 1.25), (0.75, 1.25), (1.5, 2.5));
    let r1 = run(subtract_with, &a, &b);
    let r2 = run(subtract_with, &a, &b);
    assert_eq!(
        format!("{:?}", body_of(&r1).body),
        format!("{:?}", body_of(&r2).body),
        "Seamed replay"
    );
    // Assembly (disjoint union: graft door in the fallback lane).
    let a = brick::<f64>((0.0, 1.0), (0.0, 1.0), (0.0, 1.0));
    let b = brick::<f64>((2.0, 3.0), (2.0, 3.0), (2.0, 3.0));
    let r1 = run(union_with, &a, &b);
    let r2 = run(union_with, &a, &b);
    assert_eq!(
        format!("{:?}", body_of(&r1).body),
        format!("{:?}", body_of(&r2).body),
        "Assembly replay"
    );
    // Voided (nested subtract: graft + revert).
    let a = brick::<f64>((0.0, 3.0), (0.0, 3.0), (0.0, 3.0));
    let b = brick::<f64>((1.0, 2.0), (1.0, 2.0), (1.0, 2.0));
    let r1 = run(subtract_with, &a, &b);
    let r2 = run(subtract_with, &a, &b);
    assert_eq!(
        format!("{:?}", body_of(&r1).body),
        format!("{:?}", body_of(&r2).body),
        "Voided replay"
    );
}

/// Corpus extension for the A∖B ≡ A∩revert(B) oracle: the review's
/// new fixture family (side-face pocket, edge notch, deep pocket).
#[test]
fn revert_oracle_extended_corpus() {
    let corpus: Vec<(&str, Body<f64>, Body<f64>)> = vec![
        (
            "minus-x-pocket",
            brick((0.0, 2.0), (0.0, 2.0), (0.0, 2.0)),
            brick((-0.75, 0.5), (0.75, 1.25), (0.75, 1.25)),
        ),
        (
            "edge-notch",
            brick((0.0, 2.0), (0.0, 2.0), (0.0, 2.0)),
            brick((1.5, 2.5), (0.5, 1.0), (1.5, 2.5)),
        ),
        (
            "deep-pocket",
            brick((0.0, 2.0), (0.0, 2.0), (0.0, 2.0)),
            brick((0.25, 0.75), (0.25, 1.75), (0.25, 2.5)),
        ),
        // PR 5.5's newly-working collinear-seam configurations.
        (
            "collinear-four-site",
            uslab(),
            brick((-1.0, 4.0), (2.0, 2.5), (-0.5, 1.5)),
        ),
        (
            "single-prong-nonconvex",
            uslab(),
            brick((-1.0, 1.5), (2.0, 2.5), (-0.5, 1.5)),
        ),
    ];
    for (name, a, b) in corpus {
        let direct = subtract(&a, &b, Tol::witness()).unwrap();
        let via = topo::intersect(&a, &b.revert().unwrap(), Tol::witness()).unwrap();
        let (d, v) = (body_of(&direct), body_of(&via));
        let md = mass_properties(&d.body, Tol::witness()).unwrap();
        let mv = mass_properties(&v.body, Tol::witness()).unwrap();
        assert_eq!(md.volume, mv.volume, "{name}: volume");
        assert_eq!(md.surface_area, mv.surface_area, "{name}: area");
        assert_eq!(
            (
                d.body.faces().count(),
                d.body.edges().count(),
                d.body.vertices().count()
            ),
            (
                v.body.faces().count(),
                v.body.edges().count(),
                v.body.vertices().count()
            ),
            "{name}: census"
        );
    }
}

// =====================================================================
// Interval lane: the new constructive fixtures decide identically.
// =====================================================================

#[cfg(feature = "interval")]
mod interval {
    use super::*;
    use geom_core::Interval;

    #[test]
    fn interval_fixtures() {
        // Working orientation (−x pocket) + non-coplanar Fig 15.1.
        let a = brick::<Interval>((0.0, 2.0), (0.0, 2.0), (0.0, 2.0));
        let b = brick::<Interval>((-0.75, 0.5), (0.75, 1.25), (0.75, 1.25));
        let r = run(subtract_with, &a, &b);
        assert_eq!(body_of(&r).kind, BooleanResultKind::Seamed);
        let a = brick::<Interval>((0.0, 2.0), (0.0, 2.0), (0.0, 1.0));
        let b = brick::<Interval>((1.0, 3.0), (1.0, 3.0), (-0.25, 1.25));
        let r = run(topo::intersect_with, &a, &b);
        assert_eq!(body_of(&r).kind, BooleanResultKind::Seamed);
        // R1's former refusal orientation now succeeds on the
        // interval lane too (PR 5.5).
        let a = brick::<Interval>((0.0, 2.0), (0.0, 2.0), (0.0, 2.0));
        let b = brick::<Interval>((1.5, 2.75), (0.75, 1.25), (0.75, 1.25));
        let r = run(subtract_with, &a, &b);
        assert_eq!(body_of(&r).kind, BooleanResultKind::Seamed);
    }
}
