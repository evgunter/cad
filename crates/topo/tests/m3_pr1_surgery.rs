//! M3 PR 1 integration: the Euler-inventory extensions working
//! together — the multi-shell lifecycle (mfkrh shell split → movefac
//! distribution → cross-shell kfmrh fusion), the revert pins
//! (involution, determinism, tier 3), split_edge at rest on real
//! geometry, and merge_coplanar_faces on the geometric cube.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::Point3;
use topo::{Body, FaceSurface, MefSite, MevSite, validate, validate_closed, validate_geometric};

use crate::common;
use common::{describe_as_intersections, geometric_cube, line};
use geom_core::Tol;

fn pt(x: f64, y: f64, z: f64) -> Point3<f64> {
    Point3::new(x, y, z)
}

/// Builds a cube with a detached closed inner box grown from an empty
/// ring on the top face, closed by mfkrh (the cross-shell lmfkrh
/// motion). Returns (body, shell, top_face, promoted_inner_face).
fn cube_with_inner_box() -> (Body<f64>, topo::ShellKey, topo::FaceKey, topo::FaceKey) {
    let cube = {
        // The ops cube from the crate example (structural geometry:
        // chord lines + placeholder surfaces; tiers 1–2 only).
        let p = pt;
        let mut body = Body::<f64>::new();
        let seed = body.mvfs(p(0.0, 0.0, 0.0)).unwrap();
        let e_ab = body
            .mev_line(
                MevSite::Lone {
                    r#loop: seed.r#loop,
                },
                p(1.0, 0.0, 0.0),
                Tol::witness(),
            )
            .unwrap();
        let strut = |he| MevSite::Fan { he1: he, he2: he };
        let e_bc = body
            .mev_line(strut(e_ab.he_minus), p(1.0, 1.0, 0.0), Tol::witness())
            .unwrap();
        let e_cd = body
            .mev_line(strut(e_bc.he_minus), p(0.0, 1.0, 0.0), Tol::witness())
            .unwrap();
        let he_dc = body
            .find_half_edge(seed.face, e_cd.vertex, e_bc.vertex)
            .unwrap();
        let f_bot = body
            .mef_chord(
                MefSite::Chords {
                    he1: he_dc,
                    he2: e_ab.he_plus,
                },
                Tol::witness(),
            )
            .unwrap();
        let e_aa = body
            .mev_line(strut(e_ab.he_plus), p(0.0, 0.0, 1.0), Tol::witness())
            .unwrap();
        let e_bb = body
            .mev_line(strut(e_bc.he_plus), p(1.0, 0.0, 1.0), Tol::witness())
            .unwrap();
        let e_cc = body
            .mev_line(strut(e_cd.he_plus), p(1.0, 1.0, 1.0), Tol::witness())
            .unwrap();
        let e_dd = body
            .mev_line(strut(f_bot.he_plus), p(0.0, 1.0, 1.0), Tol::witness())
            .unwrap();
        let chord = |he1, he2| MefSite::Chords { he1, he2 };
        let f_front = body
            .mef_chord(chord(e_aa.he_minus, e_bb.he_minus), Tol::witness())
            .unwrap();
        body.mef_chord(chord(e_bb.he_minus, e_cc.he_minus), Tol::witness())
            .unwrap();
        body.mef_chord(chord(e_cc.he_minus, e_dd.he_minus), Tol::witness())
            .unwrap();
        body.mef_chord(chord(e_dd.he_minus, f_front.he_plus), Tol::witness())
            .unwrap();
        (body, seed)
    };
    let (mut body, seed) = cube;
    let top = seed.face; // the seed face survives as the top
    // Plant a detached empty ring on the top face: strut + kemr, at a
    // half-edge of the TOP loop (the strut lands in its site's loop).
    let top_he = {
        let topo::LoopBoundary::Cycle { first } = body
            .get_loop(body.get_face(top).unwrap().outer)
            .unwrap()
            .boundary
        else {
            panic!("top loop is a cycle");
        };
        first
    };
    let strut = body
        .mev_line(
            MevSite::Fan {
                he1: top_he,
                he2: top_he,
            },
            pt(0.25, 0.25, 1.25),
            Tol::witness(),
        )
        .unwrap();
    let planted = body.kemr(strut.he_plus, strut.he_minus).unwrap();
    // Grow a closed inner box from the ring (the cube sequence rooted
    // in the ring loop; the ring itself becomes the box's last face by
    // promotion).
    let p = |x: f64, y: f64| pt(0.25 + x, 0.25 + y, 1.5);
    let q = |x: f64, y: f64| pt(0.25 + x, 0.25 + y, 1.75);
    let i_ab = body
        .mev_line(
            MevSite::Lone {
                r#loop: planted.ring,
            },
            p(0.25, 0.0),
            Tol::witness(),
        )
        .unwrap();
    let strut_at = |he| MevSite::Fan { he1: he, he2: he };
    let i_bc = body
        .mev_line(strut_at(i_ab.he_minus), p(0.25, 0.25), Tol::witness())
        .unwrap();
    let i_cd = body
        .mev_line(strut_at(i_bc.he_minus), p(0.0, 0.25), Tol::witness())
        .unwrap();
    // The ring loop's face is the TOP face; find_half_edge searches a
    // face's loops, so address the chord through the top face.
    let he_dc = body.find_half_edge(top, i_cd.vertex, i_bc.vertex).unwrap();
    let f_bot = body
        .mef_chord(
            MefSite::Chords {
                he1: he_dc,
                he2: i_ab.he_plus,
            },
            Tol::witness(),
        )
        .unwrap();
    let i_aa = body
        .mev_line(strut_at(i_ab.he_plus), q(0.0, 0.0), Tol::witness())
        .unwrap();
    let i_bb = body
        .mev_line(strut_at(i_bc.he_plus), q(0.25, 0.0), Tol::witness())
        .unwrap();
    let i_cc = body
        .mev_line(strut_at(i_cd.he_plus), q(0.25, 0.25), Tol::witness())
        .unwrap();
    let i_dd = body
        .mev_line(strut_at(f_bot.he_plus), q(0.0, 0.25), Tol::witness())
        .unwrap();
    let chord = |he1, he2| MefSite::Chords { he1, he2 };
    let f_front = body
        .mef_chord(chord(i_aa.he_minus, i_bb.he_minus), Tol::witness())
        .unwrap();
    body.mef_chord(chord(i_bb.he_minus, i_cc.he_minus), Tol::witness())
        .unwrap();
    body.mef_chord(chord(i_cc.he_minus, i_dd.he_minus), Tol::witness())
        .unwrap();
    body.mef_chord(chord(i_dd.he_minus, f_front.he_plus), Tol::witness())
        .unwrap();
    // Promote the ring: the cross-shell lmfkrh motion — the shell's
    // surface splits into two closed components (cube; inner box).
    // Inherit: the promoted face shares the demoting face's surface
    // key (the section-face convention).
    let promoted = body.mfkrh(planted.ring, FaceSurface::Inherit).unwrap();
    (body, seed.shell, top, promoted.face)
}

/// The full multi-shell lifecycle the M3 plan names: mfkrh makes the
/// two-component transient (tier 1 yes, tier 2 no), movefac
/// distributes it into a genuine multi-shell solid (tier 2 yes —
/// multi-shell-per-solid is at-rest legal), cross-shell kfmrh fuses
/// the shells back and the body reaches tier 2 again as one shell.
#[test]
fn multi_shell_lifecycle_roundtrip() {
    let (mut body, shell, top, inner) = cube_with_inner_box();
    // Transient: one shell entity, two surface components.
    assert_eq!(validate(&body), Ok(()));
    assert!(matches!(
        validate_closed(&body).unwrap_err()[..],
        [topo::ValidationError::ShellDisconnected { components: 2, .. }]
    ));
    // Distribute: a real multi-shell solid, tier-2 legal at rest.
    let shells = body.movefac(shell).unwrap();
    assert_eq!(shells.len(), 2);
    assert_eq!(validate_closed(&body), Ok(()));
    let (_, solid) = body.solids().next().unwrap();
    assert_eq!(solid.shells.len(), 2);
    assert_eq!(body.get_face(inner).unwrap().shell, shells[1]);
    // Component-aware E–P on the multi-shell solid: both shells are
    // closed genus-0 pieces — the validator's pass 11 accepts each
    // (already implied by tier 2 above, asserted for the record).
    assert_eq!(body.shells().count(), 2);
    // Fuse back: cross-shell kfmrh re-homes the inner box's faces and
    // demotes its promoted face to a ring of the top face again.
    let fused = body.kfmrh(top, inner).unwrap();
    assert_eq!(fused.killed_shell, Some(shells[1]));
    assert_eq!(body.shells().count(), 1);
    assert_eq!(validate_closed(&body), Ok(()));
    assert!(body.get_face(top).unwrap().rings.contains(&fused.ring));
}

/// Deterministic replay of the whole lifecycle is byte-identical (D9).
#[test]
fn multi_shell_lifecycle_replay_is_byte_identical() {
    let run = || {
        let (mut body, shell, top, inner) = cube_with_inner_box();
        body.movefac(shell).unwrap();
        body.kfmrh(top, inner).unwrap();
        format!("{body:?}")
    };
    assert_eq!(run(), run());
}

/// revert on the geometric cube: bitwise involution, determinism, and
/// tier-3 validity of the reverted body (the complement solid is
/// geometrically coherent — planes negated, loops reversed, every
/// certification still green).
#[test]
fn revert_involution_and_tiers() {
    let mut cube = geometric_cube::<f64>();
    describe_as_intersections(&mut cube.body);
    assert_eq!(validate_geometric(&cube.body, Tol::witness()), Ok(()));
    let original = format!("{:?}", cube.body);
    let reverted = cube.body.revert().unwrap();
    // Reverted bodies are tier-2 currency: every structural invariant
    // and certification survives, and the ONLY tier-3 complaint is the
    // +V invariant — the reverted body bounds the complement BY
    // DESIGN (it is ∖'s transient operand, never an at-rest solid).
    assert_eq!(validate_closed(&reverted), Ok(()));
    assert_eq!(
        validate_geometric(&reverted, Tol::witness()),
        Err(vec![topo::ValidationError::NegativeVolume])
    );
    // Genuinely reversed: some plane normal is negated.
    assert_ne!(format!("{reverted:?}"), original);
    // Involution, bitwise (Debug is shortest-roundtrip on f64 — the
    // D9 dump channel).
    let back = reverted.revert().unwrap();
    assert_eq!(format!("{back:?}"), original);
    // Determinism: replaying the revert is byte-identical.
    assert_eq!(
        format!("{:?}", cube.body.revert().unwrap()),
        format!("{reverted:?}")
    );
}

/// The reverted cube bounds the complement: signed volume negates
/// exactly, area is unchanged.
#[test]
fn revert_negates_volume() {
    let mut cube = geometric_cube::<f64>();
    describe_as_intersections(&mut cube.body);
    let props = topo::mass_properties(&cube.body, Tol::witness()).unwrap();
    let rev_props = topo::mass_properties(&cube.body.revert().unwrap(), Tol::witness()).unwrap();
    assert_eq!(rev_props.volume.to_bits(), (-props.volume).to_bits());
    assert_eq!(
        rev_props.surface_area.to_bits(),
        props.surface_area.to_bits()
    );
}

/// split_edge on the geometric cube at rest: after the prefer-intrinsic
/// upgrade, splitting an Intersection-described edge yields two
/// certified Intersection children whose adjacency obligations
/// transfer — the split body passes tier 3.
#[test]
fn split_edge_preserves_tier3_at_rest() {
    let mut cube = geometric_cube::<f64>();
    describe_as_intersections(&mut cube.body);
    assert_eq!(validate_geometric(&cube.body, Tol::witness()), Ok(()));
    let edge = cube.mevs[0].edge; // A → B chord, params [0, 1]
    let created = cube.body.split_edge(edge, 0.5, Tol::witness()).unwrap();
    assert_eq!(validate_geometric(&cube.body, Tol::witness()), Ok(()));
    // The new vertex sits at the carrier midpoint.
    let p = cube.body.get_point(created.point).unwrap();
    assert_eq!((p.x, p.y, p.z), (0.5, 0.0, 0.0));
}

/// merge_coplanar_faces: a same-key coplanar split (mef + Inherit) is
/// merged back — the structural rung; the result is the 6-face cube
/// again, tier-3 valid; replay is byte-identical.
#[test]
fn merge_coplanar_same_key_pair() {
    let mut cube = geometric_cube::<f64>();
    // Split the top face along the diagonal A′C′ (same surface key).
    let a1 = cube.mevs[3].vertex; // A′
    let c1 = cube.mevs[5].vertex; // C′
    let top = cube.seed.face;
    let he1 = {
        // half-edge of the top loop starting at A′
        let f = cube.body.get_face(top).unwrap();
        let topo::LoopBoundary::Cycle { first } = cube.body.get_loop(f.outer).unwrap().boundary
        else {
            panic!("top loop is a cycle");
        };
        cube.body
            .loop_cycle(first)
            .unwrap()
            .into_iter()
            .find(|&he| cube.body.get_half_edge(he).unwrap().start == a1)
            .unwrap()
    };
    let he2 = cube
        .body
        .loop_cycle(he1)
        .unwrap()
        .into_iter()
        .find(|&he| cube.body.get_half_edge(he).unwrap().start == c1)
        .unwrap();
    let pa = *cube
        .body
        .get_point(cube.body.get_vertex(a1).unwrap().point)
        .unwrap();
    let pc = *cube
        .body
        .get_point(cube.body.get_vertex(c1).unwrap().point)
        .unwrap();
    let diagonal = cube
        .body
        .mef(
            MefSite::Chords { he1, he2 },
            line(pa, pc),
            FaceSurface::Inherit,
            Tol::witness(),
        )
        .unwrap();
    assert_eq!(cube.body.faces().count(), 7);
    assert_eq!(validate_closed(&cube.body), Ok(()));
    // Merge back.
    let outcome = cube.body.merge_coplanar_faces(Tol::witness()).unwrap();
    assert_eq!(outcome.groups.len(), 1);
    assert_eq!(outcome.groups[0].killed_edges, vec![diagonal.edge]);
    assert!(outcome.groups[0].rings_made.is_empty());
    assert_eq!(cube.body.faces().count(), 6);
    assert_eq!(cube.body.edges().count(), 12);
    assert_eq!(validate_closed(&cube.body), Ok(()));
    // Deterministic no-op on the already-maximal result.
    let before = format!("{:?}", cube.body);
    assert_eq!(
        cube.body
            .merge_coplanar_faces(Tol::witness())
            .unwrap()
            .groups,
        vec![]
    );
    assert_eq!(format!("{:?}", cube.body), before);
}

/// The declared rung (bit-identical plane, distinct keys) merges; a
/// merely numerically-coincident plane (same geometry, different
/// origin field) does NOT — coincidence is never inferred from values.
#[test]
fn merge_coplanar_declared_vs_numeric() {
    use common::plane;
    let build = |surface_for_split: fn(&topo::Body<f64>) -> FaceSurface<f64>| {
        let mut cube = geometric_cube::<f64>();
        let a1 = cube.mevs[3].vertex;
        let c1 = cube.mevs[5].vertex;
        let top = cube.seed.face;
        let f = cube.body.get_face(top).unwrap();
        let topo::LoopBoundary::Cycle { first } = cube.body.get_loop(f.outer).unwrap().boundary
        else {
            panic!("cycle");
        };
        let he1 = cube
            .body
            .loop_cycle(first)
            .unwrap()
            .into_iter()
            .find(|&he| cube.body.get_half_edge(he).unwrap().start == a1)
            .unwrap();
        let he2 = cube
            .body
            .loop_cycle(he1)
            .unwrap()
            .into_iter()
            .find(|&he| cube.body.get_half_edge(he).unwrap().start == c1)
            .unwrap();
        let pa = *cube
            .body
            .get_point(cube.body.get_vertex(a1).unwrap().point)
            .unwrap();
        let pc = *cube
            .body
            .get_point(cube.body.get_vertex(c1).unwrap().point)
            .unwrap();
        let surface = surface_for_split(&cube.body);
        cube.body
            .mef(
                MefSite::Chords { he1, he2 },
                line(pa, pc),
                surface,
                Tol::witness(),
            )
            .unwrap();
        cube.body
    };
    // Bit-identical description on a fresh key, NO source and NO
    // declaration: stays unmerged post-retirement (M4 PR 5, ladder
    // rung (b) — value equality never glues; the M3-era bit rung is
    // gone).
    let mut bit_equal = build(|_| {
        FaceSurface::New(plane(&[
            pt(0.0, 0.0, 1.0),
            pt(1.0, 0.0, 1.0),
            pt(1.0, 1.0, 1.0),
            pt(0.0, 1.0, 1.0),
        ]))
    });
    let outcome = bit_equal.merge_coplanar_faces(Tol::witness()).unwrap();
    assert_eq!(outcome.groups, vec![]);
    assert_eq!(bit_equal.faces().count(), 7);
    // Declared, N6 same-source rung: stamp BOTH descriptions with one
    // GeomSource — the provenance lookup merges with zero numerics
    // and zero per-call declarations.
    let mut same_source = build(|_| {
        FaceSurface::New(plane(&[
            pt(0.0, 0.0, 1.0),
            pt(1.0, 0.0, 1.0),
            pt(1.0, 1.0, 1.0),
            pt(0.0, 1.0, 1.0),
        ]))
    });
    let src = topo::GeomSource::minted(42, 0);
    let coplanar_keys: Vec<_> = same_source
        .faces()
        .filter_map(|(_, f)| match same_source.get_surface(f.surface) {
            Some(geom::Surface::Plane { origin, normal, .. })
                if origin.z == 1.0 && normal.z == 1.0 =>
            {
                Some(f.surface)
            }
            _ => None,
        })
        .collect();
    // The seed top face and its chord twin both describe z = 1 with
    // +z normals; stamp exactly those two.
    assert_eq!(coplanar_keys.len(), 2, "{coplanar_keys:?}");
    for k in &coplanar_keys {
        same_source.set_surface_source(*k, src.clone()).unwrap();
    }
    let outcome = same_source.merge_coplanar_faces(Tol::witness()).unwrap();
    assert_eq!(outcome.groups.len(), 1);
    assert_eq!(same_source.faces().count(), 6);
    // Declared, per-call surface pair (F5): same geometry, fresh
    // build, intent supplied by the call — merges after verification.
    let mut declared = build(|_| {
        FaceSurface::New(plane(&[
            pt(0.0, 0.0, 1.0),
            pt(1.0, 0.0, 1.0),
            pt(1.0, 1.0, 1.0),
            pt(0.0, 1.0, 1.0),
        ]))
    });
    let pair: Vec<_> = declared
        .faces()
        .filter_map(|(_, f)| match declared.get_surface(f.surface) {
            Some(geom::Surface::Plane { origin, normal, .. })
                if origin.z == 1.0 && normal.z == 1.0 =>
            {
                Some(f.surface)
            }
            _ => None,
        })
        .collect();
    assert_eq!(pair.len(), 2, "{pair:?}");
    let outcome = declared
        .merge_coplanar_faces_declared(&[(pair[0], pair[1])], Tol::witness())
        .unwrap();
    assert_eq!(outcome.groups.len(), 1);
    assert_eq!(declared.faces().count(), 6);
    // Numeric-only: geometrically the same plane, but the description
    // differs (another origin on the plane) — stays unmerged BY
    // DESIGN: coincidence is structural or declared, never inferred.
    let mut numeric = build(|_| {
        FaceSurface::New(geom::Surface::Plane {
            origin: pt(0.25, 0.75, 1.0),
            normal: Point3::new(0.0, 0.0, 1.0) - Point3::new(0.0, 0.0, 0.0),
            u_ref: Point3::new(1.0, 0.0, 0.0) - Point3::new(0.0, 0.0, 0.0),
        })
    });
    let outcome = numeric.merge_coplanar_faces(Tol::witness()).unwrap();
    assert_eq!(outcome.groups, vec![]);
    assert_eq!(numeric.faces().count(), 7);
    assert_eq!(validate_closed(&numeric), Ok(()));
}

/// merge_coplanar_faces refuses non-tier-2 input, typed and untouched.
#[test]
fn merge_coplanar_refuses_open_input() {
    let mut body = Body::<f64>::new();
    let seed = body.mvfs(pt(0.0, 0.0, 0.0)).unwrap();
    let _ = seed;
    let before = format!("{body:?}");
    let err = body.merge_coplanar_faces(Tol::witness()).unwrap_err();
    assert!(matches!(
        err,
        topo::MergeCoplanarError::InputNotClosed { .. }
    ));
    assert_eq!(format!("{body:?}"), before);
}
