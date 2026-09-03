//! **The iso-rectangle SHAPE door in front of the curved lane, and the
//! two questions it separates from the walk-consistency check
//! (issues 727 and 726).**
//!
//! Two bodies reach `tessellate` here that no public construction
//! mints — the boolean refuses `CurvedPierceUnsupported` and
//! `import_step`'s tier 3 refuses `NotIsoRectangle` first — so they are
//! assembled through the Euler doors, the one route the walk's own docs
//! name as fronted by no certification:
//!
//! * **the keyway**: a cylinder wall whose iso domain is a U (a notch
//!   cut into the top rim) — iso-bounded, every edge a rim or a
//!   generator, and not a rectangle;
//! * **the oblique lens**: a sphere face bounded by two plane sections
//!   that are neither coaxial rims nor great circles, meeting off the
//!   axis — the `walk::iso_side_starts` qualification's own case, in
//!   its CARRIER half.
//!
//! The qualification's other half — a certified iso CARRIER whose
//! traversed ARC leaves the branch — is not this door's and is not
//! pinned in this file: it is `props::require_one_chart_branch`'s, and
//! its rows are `mesh/tests/mesh11_arc_branch.rs` (issue 1571). Both
//! doors are cited by `curved::require_iso_rectangle_face`, so a face
//! reaching the walk has passed both; this file is about the first.
//!
//! Per direction: a notched iso domain refuses at the SHAPE door with
//! props' predicate name; the spatial check, asked directly about the
//! same walk, also refuses it (the two derivations agree on a real
//! notch, by a feature-sized distance); the lens refuses at the shape
//! door, and the walk it would otherwise have received collapses onto
//! one rim level and IS its own bounding box — the spatial check admits
//! it, which is the defeat the qualification recorded and the door now
//! closes. A rimless lune (the partial sphere wedge) keeps meshing:
//! the door is the shape predicate, not the flux lane's `Δu = π`
//! premise.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::common;

use common::witness_bodies::{keyway, oblique_lens, slit};
use common::*;
use geom::Surface;
use geom_brep::props::PropsError;
use geom_core::Tol;
use mesh::TessellateError;

/// Direction 1 — **a notched iso domain refuses at the SHAPE door, with
/// props' own predicate name**, before any walk runs.
#[test]
fn the_keyway_refuses_at_the_shape_door_by_props_rim_level() {
    let (body, face) = keyway();
    let got = mesh::tessellate(&body, 0.05, Tol::witness()).map(|_| ());
    assert_eq!(
        got,
        Err(TessellateError::UnsupportedCurvedShape {
            face,
            source: PropsError::NotIsoRectangle {
                what: "props_rim_level"
            },
        }),
        "a U-shaped iso domain is refused by the ONE named predicate"
    );
}

/// Direction 2 — **the oblique lens refuses at the SHAPE door**: props'
/// sphere classification admits a circle only as a coaxial rim or a
/// great circle, and a tilted section is neither. This is the face the
/// `walk::iso_side_starts` qualification named as the one that could
/// collapse past the spatial check; it never reaches the walk now.
/// (The qualification's ARC half is the branch door's, and refuses
/// elsewhere — `mesh/tests/mesh11_arc_branch.rs`.)
#[test]
fn the_oblique_lens_refuses_at_the_shape_door() {
    let (body, face) = oblique_lens();
    let got = mesh::tessellate(&body, 0.05, Tol::witness()).map(|_| ());
    assert_eq!(
        got,
        Err(TessellateError::UnsupportedCurvedShape {
            face,
            source: PropsError::NotIsoRectangle {
                what: "props_rim_axis_parallel"
            },
        }),
        "a tilted plane section of a sphere is not an iso curve; the door refuses it typed"
    );
}

/// The divergence between the SHAPE door and the flux lane, pinned in
/// both directions: the partial sphere wedge is a rimless lune — a
/// chart rectangle `[0, θ] × [−π/2, π/2]` — so the door admits it and
/// it meshes exactly as before, while `mass_properties` refuses the
/// same body for the flux lane's own reason (`Δu = π`,
/// `props_band_coplanar`), which is a closed-form premise and not a
/// statement about the shape.
#[test]
fn a_rimless_lune_meshes_through_the_door_that_the_flux_lane_refuses() {
    let body = sphere_wedge(2.0);
    let mesh = mesh::tessellate(&body, 0.05, Tol::witness()).expect("the lune meshes");
    mesh::validate::check_mesh(&mesh).expect("watertight");
    let sphere_face = body
        .faces()
        .find(|(_, f)| matches!(body.get_surface(f.surface), Some(Surface::Sphere { .. })))
        .map(|(fk, f)| (fk, f.outer))
        .expect("the wedge has a sphere face");
    let (outer, _) = topo::props::loop_edges(&body, sphere_face.1).unwrap();
    let surface = body
        .get_surface(body.get_face(sphere_face.0).unwrap().surface)
        .unwrap();
    let band = geom_core::Band::linear(Tol::witness()).unwrap();
    assert_eq!(
        geom_brep::props::require_iso_rectangle(surface, &outer, band),
        Ok(()),
        "a rimless lune is a chart rectangle"
    );
    assert!(
        matches!(
            topo::mass_properties(&body, Tol::witness()),
            Err(topo::MassPropsError::Face {
                source: PropsError::NotIsoRectangle {
                    what: "props_band_coplanar"
                },
                ..
            })
        ),
        "the flux lane refuses the same face for its own Δu = π premise"
    );
}

/// The tour donut with its seam meridian (edge 0: a minor circle,
/// radius 0.5, spanning `0..π`) split at the given fractions of its
/// stored interval, plus that edge's unsplit stored interval.
fn split_seam_donut(fracs: &[f64]) -> (topo::Body<f64>, (f64, f64)) {
    let tol = Tol::witness();
    let mut body = donut();
    let (seam, edge) = body.edges().next().unwrap();
    let curve = body
        .get_curve_geom(edge.curve)
        .unwrap()
        .certified()
        .unwrap();
    assert!(
        matches!(curve.carrier(), geom::Curve3::Circle { radius, .. } if (*radius - 0.5).abs() < 1e-12),
        "the fixture's first edge is the seam minor circle"
    );
    let (t0, t1) = curve.params();
    for f in fracs {
        body.split_edge(seam, t0 + f * (t1 - t0), tol)
            .expect("splitting the seam meridian");
    }
    (body, (t0, t1))
}

/// What every props consumer answers on each face of `body`, keyed by
/// face: the shape door, the flux lane's `(flux, area)` bit patterns,
/// and the boundary's material side.
type FaceReceipt = (
    topo::FaceKey,
    Result<(), PropsError>,
    Result<(u64, u64), PropsError>,
    Result<geom_brep::props::MaterialSign, PropsError>,
);
fn receipts(body: &topo::Body<f64>) -> Vec<FaceReceipt> {
    let band = geom_core::Band::linear(Tol::witness()).unwrap();
    body.faces()
        .map(|(fk, f)| {
            let (outer, _) = topo::props::loop_edges(body, f.outer).unwrap();
            let surface = body.get_surface(f.surface).unwrap();
            let sense = if f.sense { 1.0 } else { -1.0 };
            (
                fk,
                geom_brep::props::require_iso_rectangle(surface, &outer, band),
                geom_brep::props::curved_face(surface, &outer, sense, band)
                    .map(|c| (c.flux.to_bits(), c.area.to_bits())),
                geom_brep::props::boundary_material_sign(surface, &outer, band),
            )
        })
        .collect()
}

/// **The split-seam donut is the unsplit donut to every consumer.** A
/// donut whose seam meridian is carried by two edges after
/// `split_edge` is the chart rectangle it always was, and props folds
/// the pieces of a split edge back into the meridian they carry —
/// identity through the split lineage, never through the stored
/// circle's values — so the shape door, the flux lane,
/// `boundary_material_sign` and `mass_properties` answer for the
/// split body exactly what they answer for the unsplit one, BITWISE
/// (V = 9.8696…, A = 39.478…, each face's flux and area, each face's
/// side: zero ulps apart). The mesh is the unsplit donut's up to the
/// seam column, which is chorded per sub-edge: every position the two
/// meshes do not share lies on the seam minor circle, and both are
/// watertight.
#[test]
fn a_split_seam_donut_meshes_and_measures_as_the_unsplit_donut() {
    let tol = Tol::witness();
    let base = donut();
    let (body, _) = split_seam_donut(&[0.5]);
    let mp0 = topo::mass_properties(&base, tol).expect("the unsplit donut measures");
    let mp = topo::mass_properties(&body, tol).expect("the split-seam donut measures");
    assert_eq!(
        (mp.volume.to_bits(), mp.surface_area.to_bits()),
        (mp0.volume.to_bits(), mp0.surface_area.to_bits()),
        "mass properties bitwise: split {mp:?} vs unsplit {mp0:?}"
    );
    assert!(
        (mp.volume - 9.869_604_401_089_358).abs() < 1e-12,
        "V = π² for R = 2, r = 0.5"
    );
    let (r0, r) = (receipts(&base), receipts(&body));
    assert_eq!(r.len(), r0.len(), "the split mints no face");
    for ((fk, door, flux, side), (fk0, door0, flux0, side0)) in r.iter().zip(&r0) {
        assert_eq!(fk, fk0);
        assert_eq!(
            door,
            &Ok(()),
            "face {fk:?}: the door admits the split-seam face"
        );
        assert_eq!(door0, &Ok(()));
        assert_eq!(
            flux, flux0,
            "face {fk:?}: the flux lane's (flux, area) bitwise"
        );
        assert_eq!(side, side0, "face {fk:?}: the material side");
        assert!(matches!(
            side,
            Ok(geom_brep::props::MaterialSign::Encoded(_))
        ));
    }
    let m0 = mesh::tessellate(&base, 0.1, tol).expect("the unsplit donut meshes");
    let m = mesh::tessellate(&body, 0.1, tol).expect("the split-seam donut meshes");
    mesh::validate::check_mesh(&m).expect("watertight");
    let bits = |m: &mesh::Mesh| {
        let mut v: Vec<[u64; 3]> = m
            .positions
            .iter()
            .map(|p| [p.x.to_bits(), p.y.to_bits(), p.z.to_bits()])
            .collect();
        v.sort_unstable();
        v.dedup();
        v
    };
    let (a, b) = (bits(&m0), bits(&m));
    let on_seam = |x: &[u64; 3]| {
        let (px, py, pz) = (
            f64::from_bits(x[0]),
            f64::from_bits(x[1]),
            f64::from_bits(x[2]),
        );
        pz.abs() < 1e-12 && (((px - 2.0).powi(2) + py * py).sqrt() - 0.5).abs() < 1e-12
    };
    let off: Vec<_> = a
        .iter()
        .filter(|x| b.binary_search(x).is_err())
        .chain(b.iter().filter(|x| a.binary_search(x).is_err()))
        .filter(|x| !on_seam(x))
        .collect();
    assert!(
        off.is_empty(),
        "{} positions differ off the seam column (of {} / {} distinct): {off:?}",
        off.len(),
        a.len(),
        b.len()
    );
    // The split column carries the split vertex, which the unsplit
    // column need not: one extra position, on the seam.
    assert_eq!(m.positions.len(), m0.positions.len() + 1);
}

/// **The fold's premise, pinned.** The pieces of a split edge partition
/// the parent's OWN parametrisation: at a halving split and at a
/// three-piece split, every piece carries the unsplit edge's identity,
/// the pieces are loop-consecutive, the chain's `[lowest t0, highest
/// t1]` is the unsplit edge's stored interval bitwise, and the piece
/// at the `t0` end evaluates its carrier there to the unsplit edge's
/// own `t0` point bitwise. That is what makes the folded meridian the
/// unsplit meridian's record exactly, at every split fraction — and
/// `mass_properties` says so bitwise on both patterns. The premise is
/// ENFORCED, not assumed: a child restated through `set_edge_curve`
/// with its interval shifted by a period on its own carrier (every
/// piece still certifies) no longer meets its sibling, and every
/// consumer refuses `props_meridian_pieces_meet` — this row reds if
/// the fold ever admits it.
#[test]
fn split_children_partition_the_parent_edges_own_parametrisation() {
    let tol = Tol::witness();
    let base = donut();
    let v0 = topo::mass_properties(&base, tol).unwrap().volume.to_bits();
    let (f0, _) = base.faces().next().unwrap();
    let (outer0, _) = topo::props::loop_edges(&base, base.get_face(f0).unwrap().outer).unwrap();
    for fracs in [&[0.5][..], &[0.3129, 0.15645][..]] {
        let (body, (t0, t1)) = split_seam_donut(fracs);
        let (outer, _) = topo::props::loop_edges(&body, body.get_face(f0).unwrap().outer).unwrap();
        // The seam is the one minor circle on this face's boundary,
        // traversed once in each direction; take its forward chain.
        let seam0 = outer0
            .iter()
            .find(|e| e.forward && matches!(e.carrier, geom::Curve3::Circle { radius, .. } if (radius - 0.5).abs() < 1e-12))
            .expect("the unsplit seam, forward");
        assert!(
            seam0.carrier_id.is_some(),
            "topo stamps every edge's lineage"
        );
        assert_eq!(
            (seam0.t0.to_bits(), seam0.t1.to_bits()),
            (t0.to_bits(), t1.to_bits())
        );
        let idx: Vec<usize> = (0..outer.len())
            .filter(|&i| outer[i].forward && outer[i].carrier_id == seam0.carrier_id)
            .collect();
        assert_eq!(
            idx.len(),
            fracs.len() + 1,
            "{fracs:?}: one piece per sub-interval"
        );
        assert!(
            idx.windows(2).all(|w| w[1] == w[0] + 1),
            "{fracs:?}: the pieces are loop-consecutive: {idx:?}"
        );
        let lo = idx
            .iter()
            .map(|&i| &outer[i])
            .min_by(|a, b| a.t0.total_cmp(&b.t0))
            .unwrap();
        let hi = idx
            .iter()
            .map(|&i| &outer[i])
            .max_by(|a, b| a.t1.total_cmp(&b.t1))
            .unwrap();
        assert_eq!(
            (lo.t0.to_bits(), hi.t1.to_bits()),
            (t0.to_bits(), t1.to_bits()),
            "{fracs:?}: the chain's interval is the unsplit edge's stored interval"
        );
        let (p, p0) = (lo.carrier.eval(lo.t0), seam0.carrier.eval(seam0.t0));
        assert_eq!(
            [p.x.to_bits(), p.y.to_bits(), p.z.to_bits()],
            [p0.x.to_bits(), p0.y.to_bits(), p0.z.to_bits()],
            "{fracs:?}: the anchor point"
        );
        assert_eq!(
            topo::mass_properties(&body, tol).unwrap().volume.to_bits(),
            v0,
            "{fracs:?}: the volume, bitwise"
        );
        // The enforcement: shift the last-minted child by a period.
        let mut shifted = body.clone();
        let child = shifted.edges().last().unwrap().0;
        let cert = shifted
            .get_curve_geom(shifted.get_edge(child).unwrap().curve)
            .unwrap()
            .certified()
            .unwrap()
            .clone();
        let mut spec = cert.restated_spec();
        spec.param_start += core::f64::consts::TAU;
        spec.param_end += core::f64::consts::TAU;
        shifted
            .set_edge_curve(child, spec, tol)
            .expect("the identical arc, one period along its own carrier, certifies");
        let meet = PropsError::NotIsoRectangle {
            what: "props_meridian_pieces_meet",
        };
        let got = topo::mass_properties(&shifted, tol).map(|m| m.volume);
        assert!(
            matches!(&got, Err(topo::MassPropsError::Face { source, .. }) if *source == meet),
            "{fracs:?}: the shifted child no longer meets its sibling: {got:?}"
        );
        assert!(
            matches!(
                mesh::tessellate(&shifted, 0.1, tol),
                Err(TessellateError::UnsupportedCurvedShape { source, .. }) if source == meet
            ),
            "{fracs:?}: tessellate refuses by the same name"
        );
    }
}

/// **The zero-width slit — the case the walk-consistency check keeps,
/// demonstrated rather than argued.** Every rim at an extreme and
/// every carrier a rim circle or an axial line, so the shape door says
/// `Ok`; the walk then carries the slit's tip half a metre inside the
/// polygon's own box, and the spatial check refuses it by that
/// feature-sized distance — `UnsupportedCurvedDomain`, the payload
/// naming the tip. This is the "feature-sized distance on a face the
/// door admitted" the variant's doc describes.
#[test]
fn a_zero_width_slit_passes_the_door_and_trips_the_spatial_check_feature_sized() {
    let (body, face) = slit();
    let f = body.get_face(face).unwrap();
    let (outer, _) = topo::props::loop_edges(&body, f.outer).unwrap();
    let surface = body.get_surface(f.surface).unwrap();
    let band = geom_core::Band::linear(Tol::witness()).unwrap();
    assert_eq!(
        geom_brep::props::require_iso_rectangle(surface, &outer, band),
        Ok(()),
        "every rim of the slit face is at an extreme; the door cannot see a slit"
    );
    let got = mesh::tessellate(&body, 0.05, Tol::witness()).map(|_| ());
    match got {
        Err(TessellateError::UnsupportedCurvedDomain {
            face: fk,
            first_uv: (u, v),
            max_distance,
            ..
        }) => {
            assert_eq!(fk, face);
            assert!(
                (u - 0.75).abs() < 1e-9 && (v - 0.5).abs() < 1e-9,
                "the first off-box entry is the slit's tip; got ({u}, {v})"
            );
            assert!(
                (max_distance - 0.5).abs() < 1e-9,
                "the tip sits 0.5 m inside the box (v gap 0.5 at lever 1 m); got {max_distance}"
            );
        }
        other => panic!("the slit must refuse at the walk-consistency check; got {other:?}"),
    }
}
