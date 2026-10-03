//! MATE-2 R2 review probes (PR #1417, head c27ecb5a). NOT part of the
//! unit under review — adversarial rows attacking the `Placement::
//! Elsewhere` widening (claim 2), the narrower-class / full-period
//! claim (claim 5), and the never-silent contract under partial or
//! wrong declarations.
//!
//! Every row's contract is the R1 probe's: a TYPED refusal or an
//! exactly-additive, tier-3-valid union — never a silently wrong body.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::mate2_common;

use crate::common::three_arc;
use geom_core::{OrthoFrame, Point2, Point3, Tol, Vec2};
use mate2_common::*;
use profile::{Profile, ProfileLoop, RawLoop, SketchPlane};
use sweep::test_support::extruded;
use topo::{Body, BooleanDeclarations, BooleanResult, ContactClass, FacePairDeclaration};

/// The never-silent contract, shared by every row here: refusal is
/// fine (typed by the error enum's construction); an `Ok` body must be
/// exactly additive (4-ULP relative, the unit's own oracle — tightened
/// from 8 once these probes measured the real distance) AND tier-3
/// valid AND pseudomanifold-clean.
fn never_silent(
    label: &str,
    a: &Body<f64>,
    b: &Body<f64>,
    decls: &BooleanDeclarations,
) -> Option<topo::BooleanError> {
    match topo::union_with(a, b, decls, Tol::witness()) {
        Ok(BooleanResult::Empty) => panic!("{label}: a threaded mate cannot be empty"),
        Ok(BooleanResult::Body(bb)) => {
            let v = volume(&bb.body);
            let sum = volume(a) + volume(b);
            eprintln!("{label}: unioned, v = {v:.17e} vs sum = {sum:.17e}");
            assert!(
                (v - sum).abs() <= 4.0 * f64::EPSILON * sum.abs(),
                "{label}: SILENTLY WRONG BODY — {v} vs {sum}"
            );
            if let Err(errs) = topo::validate_geometric(&bb.body, Tol::witness()) {
                panic!("{label}: SILENTLY INVALID BODY — {errs:?}");
            }
            if let Err(errs) = topo::validate_pseudomanifold(&bb.body, &bb.contacts, Tol::witness())
            {
                panic!("{label}: NOT PSEUDOMANIFOLD — {errs:?}");
            }
            None
        }
        Err(e) => {
            eprintln!("{label}: refused {e:?}");
            Some(e)
        }
    }
}

/// ATTACK (claim 2): the declaration names only a PARTIAL cover — one
/// bore face is left out entirely. The seam endpoints of ITS rim arcs
/// still land `Elsewhere` on the two covered neighbours; the widened
/// arm must not let the uncovered pair's incidence vanish. Expect a
/// typed refusal (the uncovered pair keeps both doors) — and above
/// all, never a wrong body.
#[test]
fn r2_partial_cover_one_bore_face_undeclared_is_never_silent() {
    let c = collar_at(0.0);
    let p = peg_at(0.0, 0.5, 2.0);
    let bore = walls_at(&c, 0.5);
    let pegw = walls_at(&p, 0.5);
    let mut decls = BooleanDeclarations::none();
    for &fa in &bore[..2] {
        // one bore face dropped
        for &fb in &pegw {
            decls
                .coincident_faces
                .push(FacePairDeclaration::new(fa, fb, ContactClass::Rest));
        }
    }
    let e = never_silent("partial cover (2 of 3 bore faces)", &c, &p, &decls);
    assert!(e.is_some(), "an uncovered live pair must not union");
}

/// ATTACK (claim 2): a "diagonal" declaration — each bore face against
/// exactly ONE peg face (3 pairs, not 9). Every seam endpoint's
/// HOLDING face is declared against a different bore face than the arc
/// being swept, so if coverage were consulted per holding-face the
/// record could be dropped. Never silent is the bar.
#[test]
fn r2_diagonal_declaration_is_never_silent() {
    let c = collar_at(0.0);
    let p = peg_at(0.0, 0.5, 2.0);
    let bore = walls_at(&c, 0.5);
    let pegw = walls_at(&p, 0.5);
    let mut decls = BooleanDeclarations::none();
    for i in 0..3 {
        decls.coincident_faces.push(FacePairDeclaration::new(
            bore[i],
            pegw[(i + 1) % 3],
            ContactClass::Rest,
        ));
    }
    never_silent("diagonal declaration (3 of 9)", &c, &p, &decls);
}

/// ATTACK (claim 2): seam MISMATCH — the peg's arc joints rotated 60°,
/// so its seams fall in the interiors of the bore faces' windows and
/// vice versa. Each rim-arc endpoint is now `OnEdge`-in-the-interior
/// for one face and `Out` for its neighbours; the split-and-zip is the
/// hard case for dropped events. Full 9-pair declaration.
#[test]
fn r2_rotated_seams_partial_engagement_is_never_silent() {
    let c = collar_at(0.0);
    let p = peg_at(60.0, 0.5, 2.0);
    let mut decls = BooleanDeclarations::none();
    for &fa in &walls_at(&c, 0.5) {
        for &fb in &walls_at(&p, 0.5) {
            decls
                .coincident_faces
                .push(FacePairDeclaration::new(fa, fb, ContactClass::Rest));
        }
    }
    never_silent("rotated seams (60°), partial engagement", &c, &p, &decls);
}

/// ATTACK (claim 2): OFFSET engagement — the peg spans z ∈ [0.5, 1.5],
/// so its top cap sits MID-BORE: the peg's top rim is interior to the
/// bore faces' windows (a partial patch), while the collar's top rim
/// (z = 2) has no peg material at all — its arcs lie on the shared
/// carrier with BOTH endpoints `Out` of every peg wall window in z.
/// The fix's own comment says that pair must stay loud. Never silent.
#[test]
fn r2_offset_engagement_rim_mid_face_is_never_silent() {
    let c = collar_at(0.0);
    let p = peg_at(0.0, 0.5, 1.0);
    let mut decls = BooleanDeclarations::none();
    for &fa in &walls_at(&c, 0.5) {
        for &fb in &walls_at(&p, 0.5) {
            decls
                .coincident_faces
                .push(FacePairDeclaration::new(fa, fb, ContactClass::Rest));
        }
    }
    never_silent("offset engagement (peg ends mid-bore)", &c, &p, &decls);
}

/// ATTACK (claim 2): proud at ONE end only — flush at the bottom
/// (vertex-coincidence rescue live there), proud at the top
/// (`Elsewhere` live there). The mixed case.
#[test]
fn r2_proud_one_end_is_never_silent() {
    let c = collar_at(0.0);
    let p = peg_at(0.0, 1.0, 1.5);
    let mut decls = BooleanDeclarations::none();
    for &fa in &walls_at(&c, 0.5) {
        for &fb in &walls_at(&p, 0.5) {
            decls
                .coincident_faces
                .push(FacePairDeclaration::new(fa, fb, ContactClass::Rest));
        }
    }
    never_silent("proud at the top only", &c, &p, &decls);
}

// ---------------------------------------------------------------------
// Claim 5: the narrower class. A FULL-PERIOD wall face on either side
// keeps the refusal (issue 1416's framing) — through the public door.
// ---------------------------------------------------------------------

/// The collar as a FULL revolve about the sketch y-axis: rectangle
/// x ∈ [0.5, 1.5], y ∈ [1, 2] — a full-period bore wall (one face).
fn revolved_collar() -> Body<f64> {
    let lp = ProfileLoop::polygon([
        Point2::new(0.5, 1.0),
        Point2::new(1.5, 1.0),
        Point2::new(1.5, 2.0),
        Point2::new(0.5, 2.0),
    ]);
    let vp = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tol::witness())
        .unwrap();
    sweep::revolve(
        &vp,
        sweep::RevolveAxis {
            origin: Point2::new(0.0, 0.0),
            dir: Vec2::new(0.0, 1.0),
        },
        sweep::Revolution::Full,
        Tol::witness(),
    )
    .unwrap()
    .body
}

/// A three-arc peg along the world Y axis, y ∈ [y0, y0 + h].
fn peg_along_y(y0: f64, h: f64) -> Body<f64> {
    let plane = SketchPlane::from_frame(OrthoFrame::axes_zx(Point3::new(0.0, y0, 0.0)));
    extruded(
        plane,
        vec![three_arc(Point2::new(0.0, 0.0), 0.5, 0.0)],
        h,
        Tol::witness(),
    )
}

/// Full-period BORE against a 3-arc peg: the bore is one face, the peg
/// three, and two of the peg's seam rulings cross the bore's rims away
/// from any vertex. `never_silent` holds the union to additivity, tier 3
/// and the pseudomanifold census.
#[test]
fn r2_full_period_bore_unions() {
    let c = revolved_collar();
    let p = peg_along_y(0.5, 2.0);
    let bore = walls_at(&c, 0.5);
    assert_eq!(bore.len(), 1, "the revolved collar's bore is one face");
    let mut decls = BooleanDeclarations::none();
    for &fa in &bore {
        for &fb in &walls_at(&p, 0.5) {
            decls
                .coincident_faces
                .push(FacePairDeclaration::new(fa, fb, ContactClass::Rest));
        }
    }
    let e = never_silent("full-period bore x 3-arc peg", &c, &p, &decls);
    assert!(e.is_none(), "the full-period bore mate refused: {e:?}");
}

/// The mirror: arc-split collar against a FULL-REVOLVE peg, held to
/// the same `never_silent` checks as the bore row.
#[test]
fn r2_full_period_peg_unions() {
    // The collar along Y, arc-split (extruded on the peg's plane).
    let plane = SketchPlane::from_frame(OrthoFrame::axes_zx(Point3::new(0.0, 1.0, 0.0)));
    let o = Point2::new(0.0, 0.0);
    let c = extruded(
        plane,
        vec![three_arc(o, 1.5, 0.0), three_arc(o, 0.5, 0.0)],
        1.0,
        Tol::witness(),
    );
    // The peg as a full revolve: rectangle x ∈ (0, 0.5], y ∈ [0.5, 2.5].
    let lp = ProfileLoop::polygon([
        Point2::new(0.0, 0.5),
        Point2::new(0.5, 0.5),
        Point2::new(0.5, 2.5),
        Point2::new(0.0, 2.5),
    ]);
    let vp = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tol::witness())
        .unwrap();
    let p = sweep::revolve(
        &vp,
        sweep::RevolveAxis {
            origin: Point2::new(0.0, 0.0),
            dir: Vec2::new(0.0, 1.0),
        },
        sweep::Revolution::Full,
        Tol::witness(),
    )
    .unwrap()
    .body;
    let pw = walls_at(&p, 0.5);
    let mut decls = BooleanDeclarations::none();
    for &fa in &walls_at(&c, 0.5) {
        for &fb in &pw {
            decls
                .coincident_faces
                .push(FacePairDeclaration::new(fa, fb, ContactClass::Rest));
        }
    }
    let join = topo::test_support::boolean_join_refusal(
        topo::BooleanOp::Union,
        &c,
        &p,
        &decls,
        Tol::witness(),
    );
    assert!(
        matches!(join, Ok(Some(_))),
        "the declared-REST zip builds the mate: the join refuses it, got {join:?}"
    );
    let e = never_silent("3-arc collar x full-period peg", &c, &p, &decls);
    assert!(e.is_none(), "the full-period peg mate refused: {e:?}");
}

/// Claim-7 measurement: how far from BITWISE is the unit's partial-
/// engagement additivity, in ULPs of the sum? (The unit loosened
/// fixture (i)'s bitwise oracle to a relative bound on the argument that
/// the π terms cannot cancel.)
#[test]
fn r2_measure_additivity_ulp_gap() {
    let c = collar_at(0.0);
    let p = peg_at(0.0, 0.5, 2.0);
    let mut decls = BooleanDeclarations::none();
    for &fa in &walls_at(&c, 0.5) {
        for &fb in &walls_at(&p, 0.5) {
            decls
                .coincident_faces
                .push(FacePairDeclaration::new(fa, fb, ContactClass::Rest));
        }
    }
    let out = topo::union_with(&c, &p, &decls, Tol::witness()).unwrap();
    let BooleanResult::Body(bb) = out else {
        panic!()
    };
    let (v, sum) = (volume(&bb.body), volume(&c) + volume(&p));
    let ulp = (v - sum).abs() / (f64::EPSILON * sum.abs());
    eprintln!(
        "R2 additivity gap: v = {v:.17e}, sum = {sum:.17e}, gap = {ulp:.3} ULP(s), bitwise = {}",
        v == sum
    );
}
