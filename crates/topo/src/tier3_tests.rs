//! In-crate tier-3 tests (M2 PR 3): the corruption directions only the
//! raw arenas can reach — a certified body whose stored geometry is
//! then made wrong must be caught by [`crate::validate_geometric`]'s
//! re-checks at rest (the other half of the attachment/at-rest pair;
//! the attachment-side rejections live in `tests/geometric_cube.rs`).
//!
//! The corruptions here replace whole arena entries (an [`EdgeCurve`]
//! is only constructible certified, so "a wrong cache" means "a
//! certified cache for *different* data sitting at this edge's slot" —
//! exactly what re-certification against the edge's own endpoints
//! exists to catch).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::Surface;
use geom_brep::EdgeCurveSpec;
use geom_core::{Point3, Vec3};

use crate::contact::{ContactClass, DeclaredContact};
use crate::euler::FaceSurface;
use crate::fixtures::test_curve;
use crate::validate::{
    MaterialArmOutcome, ValidationError, material_arm_outcome, validate, validate_geometric,
    validate_geometric_declared,
};
use crate::{Body, MefSite, MevSite};
use geom_brep::MaterialWedge;
use geom_core::Indeterminate;
use geom_core::Tol;

fn pt(x: f64, y: f64, z: f64) -> Point3<f64> {
    Point3::new(x, y, z)
}

/// A geometric digon pillow: two vertices, two chord edges, two
/// coplanar faces (the z = 0 plane on both sides) — the minimal
/// tier-3-clean body, and the coplanar-split smooth-dihedral case.
fn coplanar_pillow(tol: Tol) -> (Body<f64>, crate::MefCreated) {
    let mut body = Body::<f64>::new();
    let seed = body.mvfs(pt(0.0, 0.0, 0.0)).unwrap();
    let seg = body
        .mev_line(
            MevSite::Lone {
                r#loop: seed.r#loop,
            },
            pt(1.0, 0.0, 0.0),
            tol,
        )
        .unwrap();
    let plane = Surface::Plane {
        origin: pt(0.0, 0.0, 0.0),
        normal: Vec3::unit_z(),
        u_ref: Vec3::unit_x(),
    };
    let split = body
        .mef(
            MefSite::Chords {
                he1: seg.he_plus,
                he2: seg.he_minus,
            },
            EdgeCurveSpec::line_between(pt(0.0, 0.0, 0.0), pt(1.0, 0.0, 0.0)),
            FaceSurface::New(plane.clone()),
            tol,
        )
        .unwrap();
    // The seed face shares the same geometric plane under its own key
    // (identical-by-construction would share the KEY in a sweep; here
    // the point is the smooth-dihedral classification, which compares
    // the surfaces' tangent planes, not their keys).
    body.set_face_surface(seed.face, FaceSurface::New(plane))
        .unwrap();
    // Both chords were built through the SCAFFOLDING door, because
    // neither face's surface existed when its `mev`/`mef` ran. The
    // body is at rest now and both faces have charts, so both edges
    // are re-described where they rest (D3's transience fence — tier
    // 3 refuses a scaffold on a body with faces).
    let chart = body.get_face(split.face).unwrap().surface;
    for e in [seg.edge, split.edge] {
        let spec = EdgeCurveSpec::line_between(pt(0.0, 0.0, 0.0), pt(1.0, 0.0, 0.0))
            .at_rest_in_chart(chart, false);
        body.set_edge_curve(e, spec, tol).unwrap();
    }
    (body, split)
}

#[test]
fn coplanar_split_is_smooth_and_tier3_clean() {
    let tol = Tol::witness();
    let (body, _) = coplanar_pillow(tol);
    assert_eq!(validate_geometric(&body, tol), Ok(()));
}

/// **The transience fence** (U2's Q2 as corrected, 2026-08-27), red
/// then green on ONE edge of one body.
///
/// RED: the scaffolding door describes a locus for an edge whose
/// surfaces do not exist yet. Put that description back on an edge of
/// a body AT REST — two faces, two charts — and tier 3 names it.
///
/// GREEN: the same edge, same carrier, same interval, described where
/// it rests (an image in the chart it lies in) validates clean. Only
/// the description moves, which is the whole content of the fence.
#[test]
fn a_scaffold_at_rest_is_refused_and_the_chart_description_is_not() {
    let tol = Tol::witness();
    let (mut body, split) = coplanar_pillow(tol);
    assert_eq!(validate_geometric(&body, tol), Ok(()));

    // RED — back through the scaffolding door.
    let scaffolded = EdgeCurveSpec::line_between(pt(0.0, 0.0, 0.0), pt(1.0, 0.0, 0.0));
    body.set_edge_curve(split.edge, scaffolded.clone(), tol)
        .expect("the door itself is legal: certification is not where the fence lives");
    let errors = validate_geometric(&body, tol).expect_err("a scaffold at rest is refused");
    assert!(
        errors.contains(&ValidationError::ScaffoldAtRest { edge: split.edge }),
        "tier 3 must name the scaffolded edge, got {errors:?}",
    );

    // GREEN — the same edge described where it rests.
    let chart = body.get_face(split.face).unwrap().surface;
    body.set_edge_curve(split.edge, scaffolded.at_rest_in_chart(chart, false), tol)
        .unwrap();
    assert_eq!(validate_geometric(&body, tol), Ok(()));
}

/// The other half of the fence: the door it exists to keep open. An
/// edge whose surfaces genuinely do not exist yet — a `mev` chord in a
/// half-built ring — carries a scaffolding description and is NOT
/// refused, because the fence is TRANSIENCE and this edge is transient.
#[test]
fn the_scaffolding_door_still_passes_mid_construction() {
    let tol = Tol::witness();
    let mut body = Body::<f64>::new();
    let seed = body.mvfs(pt(0.0, 0.0, 0.0)).unwrap();
    body.mev_line(
        MevSite::Lone {
            r#loop: seed.r#loop,
        },
        pt(1.0, 0.0, 0.0),
        tol,
    )
    .unwrap();
    // No surface anywhere yet, so the chord could not name a chart
    // even in principle — and tier 3 says nothing about it.
    assert_eq!(validate(&body), Ok(()));
}

#[test]
fn wrong_cache_at_rest_is_rejected_by_tier3() {
    let tol = Tol::witness();
    // Certified body; then swap one edge's stored curve for a certified
    // curve of DIFFERENT data (the scaffolding circle at a far point).
    // Attachment can't see it (raw arenas); tier 3's re-certification
    // must.
    let (mut body, split) = coplanar_pillow(tol);
    let curve_key = body.get_edge(split.edge).unwrap().curve;
    *body.curves.get_mut(curve_key).unwrap() =
        crate::null::CurveGeom::Certified(test_curve(pt(50.0, 0.0, 0.0), tol));
    assert_eq!(validate(&body), Ok(()), "structurally still coherent");
    let errs = validate_geometric(&body, tol).unwrap_err();
    assert!(
        errs.iter().any(|e| matches!(
            e,
            ValidationError::EdgeCertification { edge, .. } if *edge == split.edge
        )),
        "{errs:?}"
    );
}

#[test]
fn offset_plane_at_rest_fails_planar_residuals() {
    let tol = Tol::witness();
    // Move a face's stored plane 100·ε off its vertices: the
    // Newell-cache re-check (tier 3, check 3) reports every vertex of
    // that face.
    let (mut body, split) = coplanar_pillow(tol);
    let eps = tol.eps();
    let surface_key = body.get_face(split.face).unwrap().surface;
    *body.surfaces.get_mut(surface_key).unwrap() = Surface::Plane {
        origin: pt(0.0, 0.0, 100.0 * eps),
        normal: Vec3::unit_z(),
        u_ref: Vec3::unit_x(),
    };
    let errs = validate_geometric(&body, tol).unwrap_err();
    let off_plane = errs
        .iter()
        .filter(|e| matches!(e, ValidationError::PlanarFaceResidual { face, .. } if *face == split.face))
        .count();
    assert_eq!(off_plane, 2, "both pillow vertices reported: {errs:?}");
}

#[test]
fn sliver_dihedral_at_rest_is_rejected() {
    let tol = Tol::witness();
    // Tilt one face's plane by the run-scaled sliver angle 3ε: both
    // MappedCurve attachments were legal (conventional descriptions
    // carry no dihedral requirement), but at rest the wedge is
    // indeterminate — the material wedge-angle predicate escalates.
    let (mut body, split) = coplanar_pillow(tol);
    let eps = tol.eps();
    let theta = 3.0 * eps;
    let surface_key = body.get_face(split.face).unwrap().surface;
    *body.surfaces.get_mut(surface_key).unwrap() = Surface::Plane {
        origin: pt(0.0, 0.0, 0.0),
        normal: Vec3::new(0.0, theta.sin(), theta.cos()),
        u_ref: Vec3::unit_x(),
    };
    let errs = validate_geometric(&body, tol).unwrap_err();
    assert!(
        errs.iter()
            .any(|e| matches!(e, ValidationError::SliverDihedral { .. })),
        "{errs:?}"
    );
}

#[test]
fn dangling_description_is_a_tier1_error() {
    let tol = Tol::witness();
    // An Intersection description whose surface key is ripped out of
    // the arena: the geometry-to-geometry reference check (tier 1,
    // pass 1) fires — alongside the face's own dangling reference.
    let (mut body, split) = coplanar_pillow(tol);
    let s_plus = body.get_face(split.face).unwrap().surface;
    let seed_face = body
        .get_loop(body.get_half_edge(split.he_plus).unwrap().parent_loop)
        .unwrap()
        .face;
    let s_seed = body.get_face(seed_face).unwrap().surface;
    let mut spec = EdgeCurveSpec::line_between(pt(0.0, 0.0, 0.0), pt(1.0, 0.0, 0.0));
    spec.description = geom_brep::EdgeDescriptionSpec::Intersection {
        s1: s_seed,
        s2: s_plus,
        witness: pt(0.5, 0.0, 0.0),
    };
    // NB: coincident planes would fail transversality — so tilt the
    // split face definitively first (a legal corner), then upgrade.
    *body.surfaces.get_mut(s_plus).unwrap() = Surface::Plane {
        origin: pt(0.0, 0.0, 0.0),
        normal: Vec3::unit_y(),
        u_ref: Vec3::unit_x(),
    };
    body.set_edge_curve(split.edge, spec, tol).unwrap();
    // Rip the referenced surface out (raw removal past the hygiene
    // guard).
    body.surfaces.remove(s_plus);
    let errs = validate(&body).unwrap_err();
    assert!(
        errs.iter()
            .any(|e| matches!(e, ValidationError::DanglingDescription { .. })),
        "{errs:?}"
    );
}

#[test]
fn description_references_keep_a_surface_alive() {
    let tol = Tol::witness();
    // A surface referenced ONLY by an edge description is not orphaned:
    // the hygiene guard refuses to remove it and tier 1 does not report
    // OrphanGeometry.
    let (mut body, split) = coplanar_pillow(tol);
    let s_plus = body.get_face(split.face).unwrap().surface;
    let seed_face = body
        .get_loop(body.get_half_edge(split.he_plus).unwrap().parent_loop)
        .unwrap()
        .face;
    let s_seed = body.get_face(seed_face).unwrap().surface;
    // Make the corner genuine, then describe the edge intrinsically.
    *body.surfaces.get_mut(s_plus).unwrap() = Surface::Plane {
        origin: pt(0.0, 0.0, 0.0),
        normal: Vec3::unit_y(),
        u_ref: Vec3::unit_x(),
    };
    let mut spec = EdgeCurveSpec::line_between(pt(0.0, 0.0, 0.0), pt(1.0, 0.0, 0.0));
    spec.description = geom_brep::EdgeDescriptionSpec::Intersection {
        s1: s_seed,
        s2: s_plus,
        witness: pt(0.5, 0.0, 0.0),
    };
    body.set_edge_curve(split.edge, spec, tol).unwrap();
    // Repoint the split face to a NEW surface: the old one is now
    // referenced only by the description — and must survive.
    body.set_face_surface(
        split.face,
        FaceSurface::New(Surface::Plane {
            origin: pt(0.0, 0.0, 0.0),
            normal: Vec3::unit_y(),
            u_ref: Vec3::unit_x(),
        }),
    )
    .unwrap();
    assert!(body.get_surface(s_plus).is_some(), "kept alive");
    assert_eq!(validate(&body), Ok(()), "no OrphanGeometry");
    // (Tier 3 now reports the adjacency incoherence — the description
    // names a surface that is no longer the face's — which is exactly
    // the loud trail this state should leave.)
    let errs = validate_geometric(&body, tol).unwrap_err();
    assert!(
        errs.iter().any(|e| matches!(
            e,
            ValidationError::DescriptionNotAdjacent { edge } if *edge == split.edge
        )),
        "{errs:?}"
    );
}

/// **The ruling's own figure**: two kissing cylinders with one side cut
/// away, the smallest SOLID that carries a cusp edge.
///
/// Cross-section in `z = const`: the crescent between two internally
/// tangent circles — inner centre `(0, 1)` radius 1, outer centre
/// `(0, 2)` radius 2, kissing at the origin — cut by the plane `x = 0`
/// so the material is the `x ≥ 0` lip only and the body stays a
/// manifold (the two-lipped form is the doubled cusp, which is F2's
/// coincident-distinct-edges class, not one edge). Extruded along `z`
/// from 0 to 1, that is a triangular prism's topology exactly: three
/// walls (inner cylinder, the flat cut, outer cylinder) and two caps.
///
/// The cusp edge is the vertical line at the kissing point, `ev[0]`:
/// its two faces are tangent there, and their material sides oppose —
/// the inner wall's material is OUTSIDE its cylinder (`sense: false`),
/// the outer wall's INSIDE its own. Every other edge is a definite
/// corner.
pub(crate) fn cusp_prism(tol: Tol) -> crate::fixtures::Prism {
    let mut p = crate::fixtures::prism(3, tol);
    // Cross-section corners, in the winding the fixture's caps expect
    // (counterclockwise from +z): the kiss, the outer circle's far
    // point, the inner circle's far point.
    let xy = [(0.0, 0.0), (0.0, 4.0), (0.0, 2.0)];
    for (i, (x, y)) in xy.into_iter().enumerate() {
        for (v, z) in [(p.t[i], 1.0), (p.u[i], 0.0)] {
            let point = p.body.get_vertex(v).unwrap().point;
            *p.body.points.get_mut(point).unwrap() = pt(x, y, z);
        }
    }
    let inner = Surface::Cylinder {
        origin: pt(0.0, 1.0, 0.0),
        axis: Vec3::unit_z(),
        radius: 1.0,
        u_ref: Vec3::unit_x(),
    };
    let outer = Surface::Cylinder {
        origin: pt(0.0, 2.0, 0.0),
        axis: Vec3::unit_z(),
        radius: 2.0,
        u_ref: Vec3::unit_x(),
    };
    let flat = Surface::Plane {
        origin: pt(0.0, 2.0, 0.0),
        normal: -Vec3::unit_x(),
        u_ref: Vec3::unit_y(),
    };
    let cap_top = Surface::Plane {
        origin: pt(0.0, 0.0, 1.0),
        normal: Vec3::unit_z(),
        u_ref: Vec3::unit_x(),
    };
    let cap_bottom = Surface::Plane {
        origin: pt(0.0, 0.0, 0.0),
        normal: -Vec3::unit_z(),
        u_ref: Vec3::unit_x(),
    };
    for (face, surface) in [
        (p.face_top, cap_top),
        (p.face_bottom, cap_bottom),
        (p.face_side[0], outer),
        (p.face_side[1], flat),
        (p.face_side[2], inner),
    ] {
        p.body
            .set_face_surface(face, FaceSurface::New(surface))
            .unwrap();
    }
    // The inner wall's material is OUTSIDE its cylinder, so its outward
    // normal is the chart normal reversed — the one sense bit this
    // body needs, and the reason the kissing edge is a cusp rather
    // than a seam.
    p.body.set_face_sense(p.face_side[2], false).unwrap();
    for (e, kind) in [
        (p.et[0], Carrier::Arc(2.0)),
        (p.et[1], Carrier::Segment),
        (p.et[2], Carrier::Arc(1.0)),
        (p.eb[0], Carrier::Arc(2.0)),
        (p.eb[1], Carrier::Segment),
        (p.eb[2], Carrier::Arc(1.0)),
        (p.ev[0], Carrier::Segment),
        (p.ev[1], Carrier::Segment),
        (p.ev[2], Carrier::Segment),
    ] {
        let spec = edge_spec(&p.body, e, kind);
        p.body.set_edge_curve(e, spec, tol).unwrap();
    }
    p
}

/// Which carrier an edge of [`cusp_prism`] takes: the straight ones
/// (the three vertical meridians and the cut face's side) or an arc of
/// one of the two kissing circles, traversed through `x > 0`.
#[derive(Clone, Copy)]
enum Carrier {
    Segment,
    Arc(f64),
}

/// The carrier and description for one edge of [`cusp_prism`], read off
/// the body so the forward contract (increasing parameter runs
/// `he_plus`) holds whichever way the fixture wound it.
fn edge_spec(body: &Body<f64>, edge: crate::entity::EdgeKey, kind: Carrier) -> EdgeCurveSpec<f64> {
    let he = body.get_edge(edge).unwrap().he_plus;
    let start_v = body.get_half_edge(he).unwrap().start;
    let end_v = body.half_edge_end(he).unwrap();
    let p0 = *body
        .get_point(body.get_vertex(start_v).unwrap().point)
        .unwrap();
    let p1 = *body
        .get_point(body.get_vertex(end_v).unwrap().point)
        .unwrap();
    let (s1, s2) = adjacent_surfaces(body, edge);
    let (carrier, t0, t1) = match kind {
        Carrier::Segment => {
            let len = p0.distance(p1);
            (
                geom::Curve3::Line {
                    origin: p0,
                    dir: (p1 - p0) / len,
                },
                0.0,
                len,
            )
        }
        // θ = 0 is placed at the start point and the axis is chosen so
        // that `v_ref = axis × u_ref` points at +x: the half turn from
        // θ = 0 to θ = π is then the lip's arc, never its mirror.
        Carrier::Arc(radius) => {
            let center = pt(0.0, radius, p0.z);
            let u_ref = (p0 - center) / radius;
            let axis = if u_ref.y < 0.0 {
                Vec3::unit_z()
            } else {
                -Vec3::unit_z()
            };
            (
                geom::Curve3::Circle {
                    center,
                    axis,
                    radius,
                    u_ref,
                },
                0.0,
                std::f64::consts::PI,
            )
        }
    };
    let witness = carrier.eval(0.5 * (t0 + t1));
    // The kissing edge is the one whose two faces are the two
    // cylinders; every other edge here is a definite corner.
    let description = if adjacent_are_the_two_cylinders(body, edge) {
        geom_brep::EdgeDescriptionSpec::TangentIntersection { s1, s2, witness }
    } else {
        geom_brep::EdgeDescriptionSpec::Intersection { s1, s2, witness }
    };
    EdgeCurveSpec {
        description,
        carrier,
        param_start: t0,
        param_end: t1,
    }
}

fn adjacent_surfaces(
    body: &Body<f64>,
    edge: crate::entity::EdgeKey,
) -> (geom_brep::SurfaceKey, geom_brep::SurfaceKey) {
    let e = body.get_edge(edge).unwrap();
    let face_of = |he| {
        let l = body.get_half_edge(he).unwrap().parent_loop;
        body.get_loop(l).unwrap().face
    };
    (
        body.get_face(face_of(e.he_plus)).unwrap().surface,
        body.get_face(face_of(e.he_minus)).unwrap().surface,
    )
}

fn adjacent_are_the_two_cylinders(body: &Body<f64>, edge: crate::entity::EdgeKey) -> bool {
    let (s1, s2) = adjacent_surfaces(body, edge);
    [s1, s2]
        .iter()
        .all(|&k| matches!(body.get_surface(k), Some(geom::Surface::Cylinder { .. })))
}

// ------------------------------------------------------------------
// D1's material-wedge verdict table (the #131 ruling), one row per
// arm. Every row that can be red-first is: the arms that refuse are
// pinned on bodies that validated clean before the arm existed, and
// the two legal arms are pinned green on the same fixtures.
// ------------------------------------------------------------------

/// **Row: transverse, legal at the θ = ε/r margin.** The cusp prism's
/// other eight edges are definite corners — cylinder against plane,
/// plane against plane, cylinder against cap — and none of them earns
/// a wedge refusal: with the kissing edge declared, the whole body is
/// tier-3 clean.
#[test]
fn transverse_wedges_stay_legal_and_earn_no_wedge_verdict() {
    let tol = Tol::witness();
    let p = cusp_prism(tol);
    assert_eq!(p.body.edges().count(), 9);
    assert_eq!(
        validate_geometric_declared(&p.body, &kiss_declared(&p), tol),
        Ok(())
    );
}

/// **Row: wedge π, legal.** Two coplanar faces whose material sides
/// AGREE continue one another across the seam — the legal smooth case,
/// unchanged by the material arm.
///
/// **Row: the lamina, refused.** The same geometry with one face's
/// material side flipped is a zero-thickness sheet: the sides oppose
/// and the jets osculate exactly (one plane against another). It
/// validated clean before this arm existed — the unsigned dihedral
/// pass cannot tell it from the seam above, and that is the whole
/// content of "unsigned" — and now refuses per edge, with no
/// declaration able to cure it.
#[test]
fn the_seam_is_legal_and_the_same_geometry_flipped_is_a_lamina() {
    let tol = Tol::witness();
    let (body, split) = coplanar_pillow(tol);
    assert_eq!(validate_geometric(&body, tol), Ok(()));
    let flipped = body.flipped_face_sense_for_tests(split.face).unwrap();
    let errs = validate_geometric(&flipped, tol).unwrap_err();
    assert!(
        errs.iter()
            .all(|e| matches!(e, ValidationError::LaminaWedge { .. })),
        "{errs:?}"
    );
    assert_eq!(errs.len(), 2, "one per edge of the digon: {errs:?}");
    // A declaration is not a cure: conformal contact is not the
    // curve-locus tangency the declared arm admits.
    let faces: Vec<_> = flipped.faces().map(|(k, _)| k).collect();
    let declared = [DeclaredContact {
        a: faces[0],
        b: faces[1],
        class: ContactClass::Tangent,
    }];
    assert_eq!(
        validate_geometric_declared(&flipped, &declared, tol).unwrap_err(),
        errs
    );
}

/// **Row: wedge 0, legal iff declared.** The ruling's own figure — two
/// kissing cylinders with one side cut away — refuses undeclared,
/// naming the cusp; the `Tangent` declaration on the two walls
/// legalizes exactly that edge and nothing else.
///
/// The two ways to get the declaration wrong are pinned beside it,
/// because "declared" is a claim about a NAMED pair in a NAMED class:
/// the `Rest` class asserts conformal contact and cannot legalize a
/// curve locus, and a `Tangent` claim on a different pair is a
/// statement about different faces.
#[test]
fn an_undeclared_cusp_refuses_and_only_its_own_tangent_declaration_legalizes_it() {
    let tol = Tol::witness();
    let p = cusp_prism(tol);
    let kiss = kiss_edge(&p);
    assert_eq!(
        validate_geometric(&p.body, tol).unwrap_err(),
        vec![ValidationError::UndeclaredCusp {
            edge: kiss,
            wedge: MaterialWedge::Cusp,
        }]
    );
    assert_eq!(
        validate_geometric_declared(&p.body, &kiss_declared(&p), tol),
        Ok(())
    );
    let rest = [DeclaredContact {
        a: p.face_side[0],
        b: p.face_side[2],
        class: ContactClass::Rest,
    }];
    assert!(validate_geometric_declared(&p.body, &rest, tol).is_err());
    let elsewhere = [DeclaredContact {
        a: p.face_top,
        b: p.face_bottom,
        class: ContactClass::Tangent,
    }];
    assert!(validate_geometric_declared(&p.body, &elsewhere, tol).is_err());
}

/// **Rows: wedge 0 ↔ wedge 2π under `revert`.** Reverting negates
/// every face's outward normal at once, which negates the material
/// κ_rel — so the same body, same keys, same declaration reads as the
/// knife slit, and the arm's verdict is the mirror row.
///
/// **Why the reverted body is not asserted wholly green**: `revert`
/// bounds the COMPLEMENTARY volume, so tier 3's positive-volume
/// invariant refuses every reverted bounded solid. That is a fact
/// about `revert`, not about cusps, and the cube control row below is
/// the evidence. What the wedge arm owes is that it contributes
/// nothing to the reverted body's verdict once declared, and that it
/// refuses the mirror wedge when not — both pinned here, with the
/// SAME declaration array, which is what "bit-faithfully" buys: the
/// reverted arenas are key-for-key the source's.
#[test]
fn revert_maps_the_declared_cusp_to_the_declared_slit() {
    let tol = Tol::witness();
    let p = cusp_prism(tol);
    let kiss = kiss_edge(&p);
    let reverted = p.body.revert().unwrap();
    assert_eq!(validate(&reverted), Ok(()));
    assert_eq!(
        validate_geometric(&reverted, tol).unwrap_err(),
        vec![ValidationError::UndeclaredCusp {
            edge: kiss,
            wedge: MaterialWedge::Slit,
        }],
        "the cusp's revert image is the slit, and it refuses on the same terms"
    );
    assert_eq!(
        validate_geometric_declared(&reverted, &kiss_declared(&p), tol).unwrap_err(),
        vec![ValidationError::NegativeVolume],
        "declared, the wedge arm contributes nothing either way"
    );
    // That residue is `revert`'s own ratified posture — a reverted
    // body is tier-2 currency and never tier-3, failing exactly
    // `NegativeVolume` (`crate::revert` module docs, pinned on a real
    // cube by the M3 PR 1 acceptance row) — so it says nothing about
    // this body's wedges.
    // Bit-faithful: revert is an involution, so the pair really is one
    // body read two ways.
    let back = reverted.revert().unwrap();
    assert_eq!(
        crate::fixtures::deep_snapshot(&back),
        crate::fixtures::deep_snapshot(&p.body)
    );
}

/// **Row: the second-order band, all three outcomes**, on one family
/// where only κ_rel moves: two cylinders kissing along the y axis with
/// opposed material sides, the outer radius chosen to put the jet
/// margin definitely outside the band, exactly at zero, and inside the
/// band.
///
/// - determinate (radii 1 and 2) — the wedge is decided, and refuses
///   as an undeclared cusp;
/// - osculating (radii 1 and 1) — conformal along the locus, the
///   lamina the declared arm does not admit;
/// - in-band (κ_rel = 6ε on a unit arm, so the sagitta margin is 3ε
///   at every CI ε row) — the honest escalation, naming
///   `tangent_second_order`, and NOT a refusal: ε-tightening escalates
///   an edge, it never flips a valid body to invalid.
#[test]
fn the_second_order_band_has_three_outcomes_and_they_are_three_answers() {
    let tol = Tol::witness();
    let eps = tol.get().eps;
    let determinate = kissing_cylinder_pillow(tol, 2.0);
    assert!(
        determinate.iter().all(|e| matches!(
            e,
            ValidationError::UndeclaredCusp {
                wedge: MaterialWedge::Slit,
                ..
            } | ValidationError::TangentNotIntrinsic { .. }
        )),
        "{determinate:?}"
    );
    let osculating = kissing_cylinder_pillow(tol, 1.0);
    assert!(
        osculating
            .iter()
            .all(|e| matches!(e, ValidationError::LaminaWedge { .. })),
        "{osculating:?}"
    );
    let in_band = kissing_cylinder_pillow(tol, 1.0 / (1.0 - 6.0 * eps));
    assert!(
        in_band.iter().all(|e| matches!(
            e,
            ValidationError::SliverDihedral {
                cause: Indeterminate {
                    predicate: Some("tangent_second_order"),
                    ..
                },
                ..
            }
        )),
        "{in_band:?}"
    );
}

/// The 3′ channel: a body's own C3 curve-granularity records ARE
/// `Tangent` declarations on their face pairs, so a declared cusp
/// validates through [`crate::validate::validate_pseudomanifold`] on
/// the same terms — and with no records, 3′ is tier 3 exactly,
/// undeclared cusp included.
#[test]
fn the_pseudomanifold_gate_reads_curve_records_as_the_declarations_they_are() {
    let tol = Tol::witness();
    let p = cusp_prism(tol);
    let mut records = crate::boolean::ContactRecords::default();
    records.curves.push(crate::boolean::CurveContact {
        face_a: p.face_side[0],
        face_b: p.face_side[2],
        witness: kiss_edge(&p),
    });
    assert_eq!(
        crate::validate::validate_pseudomanifold(&p.body, &records, tol),
        Ok(())
    );
    assert_eq!(
        crate::validate::validate_pseudomanifold(
            &p.body,
            &crate::boolean::ContactRecords::default(),
            tol
        )
        .unwrap_err(),
        vec![ValidationError::UndeclaredCusp {
            edge: kiss_edge(&p),
            wedge: MaterialWedge::Cusp,
        }]
    );
}

/// The kissing edge of a [`cusp_prism`]: the vertical meridian at the
/// tangency, between the two cylinder walls.
fn kiss_edge(p: &crate::fixtures::Prism) -> crate::entity::EdgeKey {
    p.ev[0]
}

/// The `Tangent` declaration on that edge's face pair.
fn kiss_declared(p: &crate::fixtures::Prism) -> [DeclaredContact; 1] {
    [DeclaredContact {
        a: p.face_side[0],
        b: p.face_side[2],
        class: ContactClass::Tangent,
    }]
}

/// The tier-3 verdict on a digon pillow whose two faces are cylinders
/// kissing along the shared chord — radius 1 against `r2` — with the
/// second face's material side flipped so the pair is the wedge-0/2π
/// arm. The body is deliberately degenerate (zero-area faces): what it
/// is for is the SECOND-ORDER band, which needs only two tangent
/// surfaces and an edge between them.
fn kissing_cylinder_pillow(tol: Tol, r2: f64) -> Vec<ValidationError> {
    let mut body = Body::<f64>::new();
    let seed = body.mvfs(pt(0.0, 0.0, 0.0)).unwrap();
    let seg = body
        .mev_line(
            MevSite::Lone {
                r#loop: seed.r#loop,
            },
            pt(0.0, 1.0, 0.0),
            tol,
        )
        .unwrap();
    let cylinder = |radius: f64| Surface::Cylinder {
        origin: pt(0.0, 0.0, radius),
        axis: Vec3::unit_y(),
        radius,
        u_ref: Vec3::unit_x(),
    };
    let split = body
        .mef(
            MefSite::Chords {
                he1: seg.he_plus,
                he2: seg.he_minus,
            },
            EdgeCurveSpec::line_between(pt(0.0, 0.0, 0.0), pt(0.0, 1.0, 0.0)),
            FaceSurface::New(cylinder(r2)),
            tol,
        )
        .unwrap();
    body.set_face_surface(seed.face, FaceSurface::New(cylinder(1.0)))
        .unwrap();
    let chart = body.get_face(split.face).unwrap().surface;
    for e in [seg.edge, split.edge] {
        let spec = EdgeCurveSpec::line_between(pt(0.0, 0.0, 0.0), pt(0.0, 1.0, 0.0))
            .at_rest_in_chart(chart, false);
        body.set_edge_curve(e, spec, tol).unwrap();
    }
    let flipped = body.flipped_face_sense_for_tests(split.face).unwrap();
    validate_geometric(&flipped, tol).unwrap_err()
}

/// **The material arm's fold, state by state** — including the two
/// states no certified geometry is known to reach.
///
/// `material_arm_outcome` is the whole of check 4's material verdict:
/// the sample loop accumulates flags, and this fold turns them into the
/// ONE outcome the edge earns. Two of its states are the reason it is a
/// separate function at all:
///
/// - **the pairing SPLIT** (`aligned == opposed`): different samples
///   along one edge disagreed about which way the material faces, or
///   no sample classified;
/// - **the end SPLIT** (`side_mixed`): the pairing agreed, the jet was
///   determinate, and different samples still called different ends.
///
/// Both previously fell to "no verdict" — silence — which validated an
/// undeclared cusp CLEAN on an edge whose own samples contradicted one
/// another. They escalate now, and because no fixture can force them,
/// calling the fold directly is the only way to pin that. The row also
/// pins the exclusivity the outcome type exists to guarantee: no input
/// yields both a lamina and a wedge.
#[test]
fn material_arm_split_states_escalate_and_the_outcomes_stay_exclusive() {
    use MaterialArmOutcome as O;
    let cusp = Some(MaterialWedge::Cusp);

    // Aligned at every sample: the legal seam, whatever the jet says
    // (a seam's legality is a first-order fact).
    assert_eq!(
        material_arm_outcome(true, false, true, None, false),
        O::Wedge(MaterialWedge::Seam)
    );
    assert_eq!(
        material_arm_outcome(true, false, false, None, false),
        O::Wedge(MaterialWedge::Seam)
    );
    // Opposed at every sample, jet determinate, one end: that end.
    assert_eq!(
        material_arm_outcome(false, true, true, cusp, false),
        O::Wedge(MaterialWedge::Cusp)
    );
    assert_eq!(
        material_arm_outcome(false, true, true, Some(MaterialWedge::Slit), false),
        O::Wedge(MaterialWedge::Slit)
    );
    // Opposed with a collapsed jet: the lamina refusal — and NOT a
    // wedge, which is why no edge can earn both refusals.
    assert_eq!(
        material_arm_outcome(false, true, false, None, false),
        O::Lamina
    );
    assert_eq!(
        material_arm_outcome(false, true, false, cusp, false),
        O::Lamina
    );
    // The two split states escalate, each naming the predicate whose
    // per-sample verdicts disagreed.
    assert_eq!(
        material_arm_outcome(false, true, true, cusp, true),
        O::Split {
            predicate: "material_cusp_side"
        },
        "the end split must escalate, never validate as the end it saw first"
    );
    assert_eq!(
        material_arm_outcome(false, true, true, None, false),
        O::Split {
            predicate: "material_cusp_side"
        },
        "opposed, determinate, and no end at all is the same non-verdict"
    );
    assert_eq!(
        material_arm_outcome(false, false, true, cusp, false),
        O::Split {
            predicate: "material_wedge_side"
        },
        "a pairing that split across samples must escalate"
    );
    assert_eq!(
        material_arm_outcome(true, true, true, cusp, false),
        O::Split {
            predicate: "material_wedge_side"
        },
        "no sample classified at all: also a non-verdict, never silence"
    );

    // Exhaustive: over every flag combination, the fold is total and
    // never returns a legal-looking wedge on a split input.
    for aligned in [false, true] {
        for opposed in [false, true] {
            for determinate in [false, true] {
                for mixed in [false, true] {
                    for side in [None, cusp, Some(MaterialWedge::Slit)] {
                        let out = material_arm_outcome(aligned, opposed, determinate, side, mixed);
                        if aligned == opposed {
                            assert!(matches!(out, O::Split { .. }), "{aligned} {opposed}");
                        }
                        if mixed && aligned != opposed && opposed && determinate {
                            assert!(matches!(out, O::Split { .. }), "a mixed end never resolves");
                        }
                    }
                }
            }
        }
    }
}
