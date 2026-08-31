//! MATE-2 R2 review probes (PR #1417, head c27ecb5a). NOT part of the
//! unit under review — adversarial rows attacking the `Placement::
//! Elsewhere` widening (claim 2), the narrower-class / full-period
//! claim (claim 5), and the never-silent contract under partial or
//! wrong declarations.
//!
//! Every row's contract is the R1 probe's: a TYPED refusal or an
//! exactly-additive, tier-3-valid union — never a silently wrong body.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Affine3, Point2, Point3, Tol, Vec2, Vec3};
use profile::{Profile, ProfileLoop, ProfileVertex, RawLoop, SketchPlane};
use topo::{
    Body, BooleanDeclarations, BooleanResult, ContactClass, FacePairDeclaration, mass_properties,
};

fn p2(x: f64, y: f64) -> Point2<f64> {
    Point2::new(x, y)
}

/// A circle at the origin as three 120° arcs, first joint at `deg0`.
fn three_arc(radius: f64, deg0: f64) -> ProfileLoop<f64> {
    let b120 = (core::f64::consts::PI / 6.0).tan();
    let at = |deg: f64| {
        let th: f64 = deg.to_radians();
        p2(radius * th.cos(), radius * th.sin())
    };
    ProfileLoop::new(vec![
        ProfileVertex::new(at(deg0), b120),
        ProfileVertex::new(at(deg0 + 120.0), b120),
        ProfileVertex::new(at(deg0 + 240.0), b120),
    ])
}

fn extruded(loops: Vec<ProfileLoop<f64>>, z0: f64, h: f64) -> Body<f64> {
    let plane = SketchPlane::new(Affine3::translation(Vec3::new(0.0, 0.0, z0)));
    let profile = Profile::new(plane, loops).validate(Tol::witness()).unwrap();
    sweep::extrude(&profile, sweep::Extrusion::Distance(h), Tol::witness())
        .unwrap()
        .body
}

/// The collar of the unit's fixture: annulus outer r = 1.5, bore
/// r = 0.5, z ∈ [1, 2], both rims three 120° arcs at joints `deg0`.
fn collar_at(deg0: f64) -> Body<f64> {
    extruded(vec![three_arc(1.5, deg0), three_arc(0.5, deg0)], 1.0, 1.0)
}

fn peg_at(deg0: f64, z0: f64, h: f64) -> Body<f64> {
    extruded(vec![three_arc(0.5, deg0)], z0, h)
}

fn walls_at(body: &Body<f64>, r: f64) -> Vec<topo::FaceKey> {
    body.faces()
        .filter(|(_, f)| {
            matches!(
                body.get_surface(f.surface),
                Some(geom::Surface::Cylinder { radius, .. }) if (radius - r).abs() < 1e-9
            )
        })
        .map(|(k, _)| k)
        .collect()
}

fn volume(b: &Body<f64>) -> f64 {
    mass_properties(b, Tol::witness()).unwrap().volume
}

/// The never-silent contract, shared by every row here: refusal is
/// fine (typed by the error enum's construction); an `Ok` body must be
/// exactly additive (8-ULP relative, the unit's own oracle) AND
/// tier-3 valid AND pseudomanifold-clean.
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
                (v - sum).abs() <= 8.0 * f64::EPSILON * sum.abs(),
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
    let lp = ProfileLoop::polygon([p2(0.5, 1.0), p2(1.5, 1.0), p2(1.5, 2.0), p2(0.5, 2.0)]);
    let vp = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tol::witness())
        .unwrap();
    sweep::revolve(
        &vp,
        sweep::RevolveAxis {
            origin: p2(0.0, 0.0),
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
    let plane = SketchPlane::from_frame(Point3::new(0.0, y0, 0.0), Vec3::unit_z(), Vec3::unit_x());
    let profile = Profile::new(plane, vec![three_arc(0.5, 0.0)])
        .validate(Tol::witness())
        .unwrap();
    sweep::extrude(&profile, sweep::Extrusion::Distance(h), Tol::witness())
        .unwrap()
        .body
}

/// Full-period BORE against a 3-arc peg: the PR's narrower-class claim
/// says this must still refuse (the containment door's full-period
/// remainder ⇒ `Undecided` ⇒ the frontier), even though the same mate
/// with an arc-split bore now unions.
#[test]
fn r2_full_period_bore_still_refuses_typed() {
    let c = revolved_collar();
    let p = peg_along_y(0.5, 2.0);
    let bore = walls_at(&c, 0.5);
    eprintln!("revolved collar: {} bore face(s) at r = 0.5", bore.len());
    let mut decls = BooleanDeclarations::none();
    for &fa in &bore {
        for &fb in &walls_at(&p, 0.5) {
            decls
                .coincident_faces
                .push(FacePairDeclaration::new(fa, fb, ContactClass::Rest));
        }
    }
    let e = never_silent("full-period bore x 3-arc peg", &c, &p, &decls);
    match e {
        Some(topo::BooleanError::CurvedPierceUnsupported { .. }) => {}
        Some(other) => eprintln!("NOTE: refused, but not the pinned kind: {other:?}"),
        None => panic!("the narrower-class claim is FALSE: a full-period bore unioned"),
    }
}

/// The mirror: arc-split collar against a FULL-REVOLVE peg (one
/// full-period wall face on the peg side).
#[test]
fn r2_full_period_peg_still_refuses_typed() {
    // The collar along Y, arc-split (extruded on the peg's plane).
    let plane = SketchPlane::from_frame(Point3::new(0.0, 1.0, 0.0), Vec3::unit_z(), Vec3::unit_x());
    let profile = Profile::new(plane, vec![three_arc(1.5, 0.0), three_arc(0.5, 0.0)])
        .validate(Tol::witness())
        .unwrap();
    let c = sweep::extrude(&profile, sweep::Extrusion::Distance(1.0), Tol::witness())
        .unwrap()
        .body;
    // The peg as a full revolve: rectangle x ∈ (0, 0.5], y ∈ [0.5, 2.5].
    let lp = ProfileLoop::polygon([p2(0.0, 0.5), p2(0.5, 0.5), p2(0.5, 2.5), p2(0.0, 2.5)]);
    let vp = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tol::witness())
        .unwrap();
    let p = sweep::revolve(
        &vp,
        sweep::RevolveAxis {
            origin: p2(0.0, 0.0),
            dir: Vec2::new(0.0, 1.0),
        },
        sweep::Revolution::Full,
        Tol::witness(),
    )
    .unwrap()
    .body;
    let pw = walls_at(&p, 0.5);
    eprintln!("revolved peg: {} wall face(s) at r = 0.5", pw.len());
    let mut decls = BooleanDeclarations::none();
    for &fa in &walls_at(&c, 0.5) {
        for &fb in &pw {
            decls
                .coincident_faces
                .push(FacePairDeclaration::new(fa, fb, ContactClass::Rest));
        }
    }
    let e = never_silent("3-arc collar x full-period peg", &c, &p, &decls);
    match e {
        Some(topo::BooleanError::CurvedPierceUnsupported { .. }) => {}
        Some(other) => eprintln!("NOTE: refused, but not the pinned kind: {other:?}"),
        None => panic!("the narrower-class claim is FALSE: a full-period peg unioned"),
    }
}
