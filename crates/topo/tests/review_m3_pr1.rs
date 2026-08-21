//! M3 PR 1 adversarial review suite (independent consumer programs).
//!
//! Falsification targets: cross-shell kfmrh E-P/genus bookkeeping,
//! null-scaffold fail-loud audit, split_edge certification honesty
//! (witness re-mint, interiority band), revert posture, movefac vs the
//! pass-11 partition, merge_coplanar_faces coincidence teeth + staged
//! atomicity, and the f64 Debug bit-equality channel.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::Point3;
use topo::{
    Body, EulerOpError, FaceSurface, MefSite, MevSite, NewVertexSide, ValidationError, validate,
    validate_closed, validate_geometric,
};

mod common;
use common::{describe_as_intersections, geometric_cube, line, plane};
use geom_core::Tol;

fn pt(x: f64, y: f64, z: f64) -> Point3<f64> {
    Point3::new(x, y, z)
}

/// Public-API deep dump (independent of the crate's pub(crate)
/// deep_snapshot): whole-body Debug is derive-generated over every
/// arena, and f64's Debug is shortest-roundtrip (bit-faithful modulo
/// NaN — probed separately below).
fn dump(body: &Body<f64>) -> String {
    format!("{body:?}")
}

/// The Euler-Poincare probe's counts through the public iterators: six
/// arena lengths plus `rings`, which is NOT an arena length — it is the
/// ring count summed over the faces.
///
/// Not `topo::test_support::ArenaCounts` for that reason, and because
/// `rings` is the `r` of the `v - e + f - r` characteristic this suite
/// is built on, which no arena census carries.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
struct EulerCensus {
    vertices: usize,
    edges: usize,
    faces: usize,
    loops: usize,
    shells: usize,
    solids: usize,
    rings: usize,
}

/// One step's signed shift of an [`EulerCensus`]. Sites name only the nonzero
/// components and take the rest from the derived zero, which cannot
/// drift out of step with the field list.
#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
struct EulerCensusDelta {
    vertices: isize,
    edges: isize,
    faces: isize,
    loops: isize,
    shells: isize,
    solids: isize,
    rings: isize,
}

fn census(body: &Body<f64>) -> EulerCensus {
    EulerCensus {
        vertices: body.vertices().count(),
        edges: body.edges().count(),
        faces: body.faces().count(),
        loops: body.loops().count(),
        shells: body.shells().count(),
        solids: body.solids().count(),
        rings: body.faces().map(|(_, f)| f.rings.len()).sum(),
    }
}

impl EulerCensus {
    /// This census minus `before`, component by component.
    fn minus(self, before: Self) -> EulerCensusDelta {
        let d = |after: usize, before: usize| after as isize - before as isize;
        EulerCensusDelta {
            vertices: d(self.vertices, before.vertices),
            edges: d(self.edges, before.edges),
            faces: d(self.faces, before.faces),
            loops: d(self.loops, before.loops),
            shells: d(self.shells, before.shells),
            solids: d(self.solids, before.solids),
            rings: d(self.rings, before.rings),
        }
    }
}

/// Euler-Poincare characteristic v - e + f - r over the whole body
/// (valid single-solid probe: chi = sum over shells of 2(1 - g)).
fn chi(body: &Body<f64>) -> isize {
    let c = census(body);
    c.vertices as isize - c.edges as isize + c.faces as isize - c.rings as isize
}

/// Plants a detached closed box component on `face` of a tier-2 body:
/// strut + kemr (empty ring) + grow a hexahedron + mfkrh promotion of
/// the last wall. Structural geometry (chord lines, Nurbs placeholder
/// planes are NOT used: mef_chord mints placeholder surfaces) - fine
/// for tiers 1-2 topology probes. Returns the promoted (detached
/// component's) face.
fn plant_detached_box(
    body: &mut Body<f64>,
    face: topo::FaceKey,
    base: Point3<f64>,
) -> topo::FaceKey {
    let host_he = {
        let topo::LoopBoundary::Cycle { first } = body
            .get_loop(body.get_face(face).unwrap().outer)
            .unwrap()
            .boundary
        else {
            panic!("host loop must be a cycle");
        };
        first
    };
    let q = |dx: f64, dy: f64, dz: f64| pt(base.x + dx, base.y + dy, base.z + dz);
    let strut = body
        .mev_line(
            MevSite::Fan {
                he1: host_he,
                he2: host_he,
            },
            q(0.0, 0.0, 0.0),
            Tol::witness(),
        )
        .unwrap();
    let planted = body.kemr(strut.he_plus, strut.he_minus).unwrap();
    // Grow a closed box from the (empty) ring: 1 lone-mev + 6 fan-mevs
    // + 4 mef chords, then promote the ring to the last face.
    let e_ab = body
        .mev_line(
            MevSite::Lone {
                r#loop: planted.ring,
            },
            q(0.2, 0.0, 0.0),
            Tol::witness(),
        )
        .unwrap();
    let fan = |he| MevSite::Fan { he1: he, he2: he };
    let e_bc = body
        .mev_line(fan(e_ab.he_minus), q(0.2, 0.2, 0.0), Tol::witness())
        .unwrap();
    let e_cd = body
        .mev_line(fan(e_bc.he_minus), q(0.0, 0.2, 0.0), Tol::witness())
        .unwrap();
    let host = body.get_loop(planted.ring).unwrap().face;
    let he_dc = body.find_half_edge(host, e_cd.vertex, e_bc.vertex).unwrap();
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
        .mev_line(fan(e_ab.he_plus), q(0.0, 0.0, 0.2), Tol::witness())
        .unwrap();
    let e_bb = body
        .mev_line(fan(e_bc.he_plus), q(0.2, 0.0, 0.2), Tol::witness())
        .unwrap();
    let e_cc = body
        .mev_line(fan(e_cd.he_plus), q(0.2, 0.2, 0.2), Tol::witness())
        .unwrap();
    let e_dd = body
        .mev_line(fan(f_bot.he_plus), q(0.0, 0.2, 0.2), Tol::witness())
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
    body.mfkrh_plug(planted.ring).unwrap().face
}

/// Structural ops cube (chord lines, placeholder surfaces): tiers 1-2.
fn ops_cube_public() -> (Body<f64>, topo::MvfsCreated) {
    let mut body = Body::<f64>::new();
    let seed = body.mvfs(pt(0.0, 0.0, 0.0)).unwrap();
    let e_ab = body
        .mev_line(
            MevSite::Lone {
                r#loop: seed.r#loop,
            },
            pt(1.0, 0.0, 0.0),
            Tol::witness(),
        )
        .unwrap();
    let fan = |he| MevSite::Fan { he1: he, he2: he };
    let e_bc = body
        .mev_line(fan(e_ab.he_minus), pt(1.0, 1.0, 0.0), Tol::witness())
        .unwrap();
    let e_cd = body
        .mev_line(fan(e_bc.he_minus), pt(0.0, 1.0, 0.0), Tol::witness())
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
        .mev_line(fan(e_ab.he_plus), pt(0.0, 0.0, 1.0), Tol::witness())
        .unwrap();
    let e_bb = body
        .mev_line(fan(e_bc.he_plus), pt(1.0, 0.0, 1.0), Tol::witness())
        .unwrap();
    let e_cc = body
        .mev_line(fan(e_cd.he_plus), pt(1.0, 1.0, 1.0), Tol::witness())
        .unwrap();
    let e_dd = body
        .mev_line(fan(f_bot.he_plus), pt(0.0, 1.0, 1.0), Tol::witness())
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
}

/// TARGET 5: a three-component shell. movefac's partition cardinality
/// must equal the validator's pass-11 component count; comp 0 keeps the
/// shell; face-list relative order is preserved; the whole thing is
/// D9-deterministic across independent replays.
#[test]
fn movefac_three_component_partition_matches_pass11() {
    let build = || {
        let (mut body, seed) = ops_cube_public();
        let inner1 = plant_detached_box(&mut body, seed.face, pt(0.2, 0.2, 1.3));
        let inner2 = plant_detached_box(&mut body, seed.face, pt(0.6, 0.6, 1.3));
        (body, seed, inner1, inner2)
    };
    let (mut body, seed, inner1, inner2) = build();
    assert_eq!(validate(&body), Ok(()));
    // The validator's own component count for the shell.
    let errs = validate_closed(&body).unwrap_err();
    let pass11_components = errs
        .iter()
        .find_map(|e| match e {
            ValidationError::ShellDisconnected { components, .. } => Some(*components),
            _ => None,
        })
        .expect("pre-movefac body must report ShellDisconnected");
    assert_eq!(pass11_components, 3);
    let faces_before: Vec<_> = body.get_shell(seed.shell).unwrap().faces.clone();
    let shells = body.movefac(seed.shell).unwrap();
    assert_eq!(shells.len(), pass11_components, "movefac != pass-11 count");
    assert_eq!(shells[0], seed.shell, "comp 0 must keep the shell");
    assert_eq!(validate_closed(&body), Ok(()));
    // Partition sanity: the two planted components landed apart, the
    // seed face stayed home, and relative face order is preserved.
    assert_eq!(body.get_face(seed.face).unwrap().shell, shells[0]);
    assert_eq!(body.get_face(inner1).unwrap().shell, shells[1]);
    assert_eq!(body.get_face(inner2).unwrap().shell, shells[2]);
    let mut reassembled: Vec<topo::FaceKey> = Vec::new();
    for &s in &shells {
        reassembled.extend(body.get_shell(s).unwrap().faces.iter());
    }
    let mut sorted_a = faces_before.clone();
    let mut sorted_b = reassembled.clone();
    sorted_a.sort();
    sorted_b.sort();
    assert_eq!(sorted_a, sorted_b, "faces lost or duplicated");
    for &s in &shells {
        let list = &body.get_shell(s).unwrap().faces;
        let mut positions = list
            .iter()
            .map(|f| faces_before.iter().position(|g| g == f).unwrap());
        let mut prev = positions.next().unwrap_or(0);
        for p in positions {
            assert!(p > prev, "face-list relative order not preserved");
            prev = p;
        }
    }
    // D9: an independent replay is byte-identical.
    let (mut body2, seed2, _, _) = build();
    body2.movefac(seed2.shell).unwrap();
    assert_eq!(dump(&body), dump(&body2));
}

/// TARGET 5: an empty-outer face (mfkrh_plug on an *empty* ring) is a
/// dartless component of its own in both the validator's pass-11
/// relation and movefac's - the divergence candidate (vertex-glue vs
/// edge-glue) probed and pinned.
#[test]
fn movefac_empty_outer_face_is_own_component() {
    let (mut body, seed) = ops_cube_public();
    let topo::LoopBoundary::Cycle { first } = body
        .get_loop(body.get_face(seed.face).unwrap().outer)
        .unwrap()
        .boundary
    else {
        panic!("cycle");
    };
    let strut = body
        .mev_line(
            MevSite::Fan {
                he1: first,
                he2: first,
            },
            pt(0.5, 0.5, 1.5),
            Tol::witness(),
        )
        .unwrap();
    let planted = body.kemr(strut.he_plus, strut.he_minus).unwrap();
    // Promote the EMPTY ring directly: an empty-outer face, detached.
    let lone_face = body.mfkrh_plug(planted.ring).unwrap().face;
    assert_eq!(validate(&body), Ok(()));
    let errs = validate_closed(&body).unwrap_err();
    let pass11 = errs
        .iter()
        .find_map(|e| match e {
            ValidationError::ShellDisconnected { components, .. } => Some(*components),
            _ => None,
        })
        .expect("detached empty-outer face must disconnect the shell");
    let shells = body.movefac(seed.shell).unwrap();
    assert_eq!(shells.len(), pass11, "movefac diverges from pass-11");
    assert_eq!(body.get_face(lone_face).unwrap().shell, shells[1]);
    assert_eq!(validate(&body), Ok(()));
}

/// TARGET 1: cross-shell kfmrh E-P arithmetic - fusing a detached
/// genus-0 component is the connected sum: arena delta exactly
/// (shells -1, faces -1, rings +1, all else 0), chi drops by 2, and the
/// result is a tier-2 single-shell solid. Then the genus ladder: a
/// same-shell kfmrh handle (M1 form) makes the fused body genus 1, a
/// SECOND planted component distributed by movefac and fused back
/// cross-shell keeps genus 1 (genera add: 1 + 0) - the validator's
/// component-aware E-P accepts every step and the counts prove the
/// bookkeeping.
#[test]
fn cross_shell_kfmrh_connected_sum_and_genus_addition() {
    let (mut body, seed) = ops_cube_public();
    let inner = plant_detached_box(&mut body, seed.face, pt(0.2, 0.2, 1.3));
    let shells = body.movefac(seed.shell).unwrap();
    assert_eq!(shells.len(), 2);
    assert_eq!(validate_closed(&body), Ok(()));
    // Two closed genus-0 components: chi = 2 + 2.
    assert_eq!(chi(&body), 4);
    let before = census(&body);
    let fused = body.kfmrh(seed.face, inner).unwrap();
    assert_eq!(fused.killed_shell, Some(shells[1]));
    let after = census(&body);
    assert_eq!(
        after.minus(before),
        EulerCensusDelta {
            faces: -1,
            shells: -1,
            rings: 1,
            ..Default::default()
        },
        "cross-shell kfmrh arena delta"
    );
    // Connected sum: chi1 + chi2 - 2 = 2, genus 0, one shell, tier 2.
    assert_eq!(chi(&body), 2);
    assert_eq!(validate_closed(&body), Ok(()));
    // M1 same-shell form on the fused body: a handle between the cube
    // bottom and the inner box's bottom wall - genus 1 (chi = 0).
    // (E-P arithmetic is face-agnostic: any two distinct ring-free
    // faces of the one shell serve; the seed face carries the fused
    // ring, so exclude it.)
    let inner_bottom = body
        .faces()
        .map(|(k, _)| k)
        .find(|&k| k != seed.face && body.get_face(k).unwrap().rings.is_empty())
        .unwrap();
    let cube_bottom = body
        .faces()
        .map(|(k, _)| k)
        .filter(|&k| {
            k != inner_bottom && k != seed.face && body.get_face(k).unwrap().rings.is_empty()
        })
        .last()
        .unwrap();
    let handle = body.kfmrh(cube_bottom, inner_bottom).unwrap();
    assert_eq!(
        handle.killed_shell, None,
        "same-shell form must not kill a shell"
    );
    assert_eq!(chi(&body), 0, "handle: genus 1");
    assert_eq!(validate(&body), Ok(()));
    // Genus addition: plant + distribute + fuse a second genus-0
    // component onto the genus-1 body; genus must stay 1 (chi 0 again).
    let inner2 = plant_detached_box(&mut body, seed.face, pt(0.6, 0.6, 1.3));
    let shells2 = body.movefac(seed.shell).unwrap();
    assert_eq!(shells2.len(), 2);
    assert_eq!(chi(&body), 2); // genus-1 (chi 0) + genus-0 (chi 2)
    let fused2 = body.kfmrh(seed.face, inner2).unwrap();
    assert_eq!(fused2.killed_shell, Some(shells2[1]));
    assert_eq!(chi(&body), 0, "genera add: 1 + 0 = 1");
    assert_eq!(validate(&body), Ok(()));
    assert_eq!(validate_closed(&body), Ok(()));
}

/// TARGET 2: the null-scaffold fail-loud audit, driven as a consumer.
/// Every public door that would need real geometry must refuse a body
/// carrying a null edge with a TYPED error - nothing may launder the
/// scaffold into an EdgeCurve or silently skip it.
#[test]
fn null_scaffold_fail_loud_audit() {
    let mut cube = geometric_cube::<f64>();
    describe_as_intersections(&mut cube.body);
    assert_eq!(validate_geometric(&cube.body, Tol::witness()), Ok(()));
    let he = cube
        .body
        .get_vertex(cube.mevs[0].vertex)
        .unwrap()
        .emanating
        .unwrap();
    let created = cube
        .body
        .mev_null(MevSite::Fan { he1: he, he2: he }, NewVertexSide::Above)
        .unwrap();
    // The arena entry is the typed scaffold; there is no certified view.
    let geom = cube.body.get_curve_geom(created.curve).unwrap();
    assert!(geom.certified().is_none());
    assert!(geom.null_scaffold().is_some());
    // Door 1: tier 2 refuses at rest, by name.
    let errs = validate_closed(&cube.body).unwrap_err();
    assert!(
        errs.iter().any(
            |e| matches!(e, ValidationError::NullEdgeAtRest { edge } if *edge == created.edge)
        )
    );
    // Door 2: mass properties refuse, typed.
    let err = topo::mass_properties(&cube.body, Tol::witness()).unwrap_err();
    assert!(
        matches!(err, topo::MassPropsError::NullScaffoldEdge { edge } if edge == created.edge),
        "mass props must not skip or launder a null edge: {err:?}"
    );
    // Door 3: tier 3 refuses (volume is uncomputable, reported typed).
    let errs = validate_geometric(&cube.body, Tol::witness()).unwrap_err();
    assert!(!errs.is_empty());
    // Door 4: split_edge on the scaffold refuses, typed and atomic.
    let before = dump(&cube.body);
    let err = cube
        .body
        .split_edge(created.edge, 0.5, Tol::witness())
        .unwrap_err();
    assert!(matches!(err, EulerOpError::NullScaffoldCurve { curve } if curve == created.curve));
    assert_eq!(dump(&cube.body), before, "failed split mutated the body");
    // Door 5: merge_coplanar_faces refuses non-tier-2 input whole.
    let err = cube.body.merge_coplanar_faces(Tol::witness()).unwrap_err();
    let topo::MergeCoplanarError::InputNotClosed { errors } = err else {
        panic!("expected InputNotClosed, got {err:?}");
    };
    assert!(
        errors
            .iter()
            .any(|e| matches!(e, ValidationError::NullEdgeAtRest { .. }))
    );
    assert_eq!(dump(&cube.body), before);
    // Door 6: revert carries the scaffold through UNCHANGED (still
    // typed scaffolding on the other side, still refused at rest).
    let reverted = cube.body.revert().unwrap();
    assert!(
        reverted
            .get_curve_geom(created.curve)
            .unwrap()
            .null_scaffold()
            .is_some()
    );
    assert!(validate_closed(&reverted).is_err());
    // Cleanup door: kev consumes the scaffold; every gate reopens.
    cube.body.kev(created.he_plus).unwrap();
    assert_eq!(validate_geometric(&cube.body, Tol::witness()), Ok(()));
    assert!(topo::mass_properties(&cube.body, Tol::witness()).is_ok());
}

/// TARGET 3: split_edge honesty on line carriers - a double split
/// (split, then split a child) with tier 3 preserved at every step,
/// child intervals abutting exactly, and volume unchanged bitwise.
#[test]
fn split_edge_double_split_preserves_tier3_and_volume() {
    let mut cube = geometric_cube::<f64>();
    describe_as_intersections(&mut cube.body);
    let vol0 = topo::mass_properties(&cube.body, Tol::witness())
        .unwrap()
        .volume;
    let edge = cube.mevs[0].edge; // A->B, unit line, params [0, 1]
    let first = cube.body.split_edge(edge, 0.25, Tol::witness()).unwrap();
    assert_eq!(validate_geometric(&cube.body, Tol::witness()), Ok(()));
    // Parent kept [t0, t]; child took [t, t1].
    let c1 = cube
        .body
        .get_curve_geom(cube.body.get_edge(edge).unwrap().curve)
        .unwrap()
        .certified()
        .unwrap();
    assert_eq!(c1.params(), (0.0, 0.25));
    let c2 = cube
        .body
        .get_curve_geom(cube.body.get_edge(first.new_edge).unwrap().curve)
        .unwrap()
        .certified()
        .unwrap();
    assert_eq!(c2.params(), (0.25, 1.0));
    let p = cube.body.get_point(first.point).unwrap();
    assert_eq!((p.x, p.y, p.z), (0.25, 0.0, 0.0));
    // Split the SECOND child (already-restricted geometry re-splits).
    let second = cube
        .body
        .split_edge(first.new_edge, 0.5, Tol::witness())
        .unwrap();
    assert_eq!(validate_geometric(&cube.body, Tol::witness()), Ok(()));
    let c3 = cube
        .body
        .get_curve_geom(cube.body.get_edge(first.new_edge).unwrap().curve)
        .unwrap()
        .certified()
        .unwrap();
    assert_eq!(c3.params(), (0.25, 0.5));
    let p2 = cube.body.get_point(second.point).unwrap();
    assert_eq!((p2.x, p2.y, p2.z), (0.5, 0.0, 0.0));
    // Splitting is geometry-neutral: exact volume identical bitwise.
    let vol1 = topo::mass_properties(&cube.body, Tol::witness())
        .unwrap()
        .volume;
    assert_eq!(vol0.to_bits(), vol1.to_bits(), "split changed the solid");
}

/// TARGET 3: Intersection-description splits re-mint each child's
/// witness BITWISE as that child's certification mid-sample
/// carrier(t_a + (t_b - t_a)/2) - derived independently here.
#[test]
fn split_edge_intersection_witness_bitwise_remint() {
    let mut cube = geometric_cube::<f64>();
    describe_as_intersections(&mut cube.body);
    assert_eq!(validate_geometric(&cube.body, Tol::witness()), Ok(()));
    let edge = cube.mevs[0].edge;
    let parent = cube
        .body
        .get_curve_geom(cube.body.get_edge(edge).unwrap().curve)
        .unwrap()
        .certified()
        .cloned()
        .unwrap();
    let (t0, t1) = parent.params();
    let t = 0.3_f64; // deliberately not dyadic
    let created = cube.body.split_edge(edge, t, Tol::witness()).unwrap();
    assert_eq!(validate_geometric(&cube.body, Tol::witness()), Ok(()));
    for (curve_key, (ta, tb)) in [
        (created.first_curve, (t0, t)),
        (created.second_curve, (t, t1)),
    ] {
        let child = cube
            .body
            .get_curve_geom(curve_key)
            .unwrap()
            .certified()
            .unwrap();
        let topo::EdgeGeometry::Intersection { witness, s1, s2 } = *child.description() else {
            panic!("child lost its Intersection description");
        };
        // Independent derivation of the mid-sample: t_a + (t_b - t_a) *
        // (4/8), the certification schedule's middle sample.
        let expected = parent.carrier().eval(ta + (tb - ta) * 0.5);
        assert_eq!(
            (
                witness.x.to_bits(),
                witness.y.to_bits(),
                witness.z.to_bits()
            ),
            (
                expected.x.to_bits(),
                expected.y.to_bits(),
                expected.z.to_bits()
            ),
            "witness is not the bitwise mid-sample"
        );
        // Children keep the parent's surface keys.
        let topo::EdgeGeometry::Intersection { s1: p1, s2: p2, .. } = *parent.description() else {
            panic!("parent description");
        };
        assert_eq!((s1, s2), (p1, p2));
    }
}

/// TARGET 3: the interiority band - at-endpoint and inside-band
/// parameters refuse with SplitParamNotInterior; far-outside refuses
/// the same way (Negative margin); every refusal is atomic. Runs at
/// whatever epsilon row the process is configured with (the suite is
/// executed across all three rows in CI).
#[test]
fn split_edge_interiority_band_edges() {
    let eps = geom_core::Tol::witness().get().eps;
    let mut cube = geometric_cube::<f64>();
    describe_as_intersections(&mut cube.body);
    let edge = cube.mevs[0].edge; // line, params [0, 1], scale 1 m
    let before = dump(&cube.body);
    for t in [
        0.0,             // exactly the start
        1.0,             // exactly the end
        eps * 0.5,       // strictly inside the band at the start
        1.0 - eps * 0.5, // strictly inside the band at the end
        -0.25,           // outside the interval entirely
        1.25,            // beyond the end
    ] {
        let err = cube.body.split_edge(edge, t, Tol::witness()).unwrap_err();
        assert!(
            matches!(err, EulerOpError::SplitParamNotInterior { edge: e } if e == edge),
            "t = {t}: expected SplitParamNotInterior, got {err:?}"
        );
        assert_eq!(dump(&cube.body), before, "refusal at t = {t} mutated");
    }
    // In the ESCALATION region (between zero-band and escalate-band):
    // typed escalation, atomic.
    let err = cube
        .body
        .split_edge(edge, 5.0 * eps, Tol::witness())
        .unwrap_err();
    assert!(matches!(err, EulerOpError::SplitParamEscalated { .. }));
    assert_eq!(dump(&cube.body), before);
    // Beyond the escalation band: definitely interior, splits.
    let t_ok = 20.0 * eps;
    cube.body.split_edge(edge, t_ok, Tol::witness()).unwrap();
    assert_eq!(validate_geometric(&cube.body, Tol::witness()), Ok(()));
}

/// TARGET 4: revert posture on a body WITH a ring and split edges (the
/// harder inventory than the shipped plain-cube pin): bitwise
/// involution, D9 determinism, tier-2 currency, tier 3 = exactly
/// NegativeVolume and nothing else, volume negated bitwise.
#[test]
fn revert_on_split_body_involution_and_posture() {
    let mut cube = geometric_cube::<f64>();
    describe_as_intersections(&mut cube.body);
    cube.body
        .split_edge(cube.mevs[0].edge, 0.5, Tol::witness())
        .unwrap();
    cube.body
        .split_edge(cube.mevs[5].edge, 0.25, Tol::witness())
        .unwrap();
    assert_eq!(validate_geometric(&cube.body, Tol::witness()), Ok(()));
    let original = dump(&cube.body);
    let vol = topo::mass_properties(&cube.body, Tol::witness())
        .unwrap()
        .volume;
    let reverted = cube.body.revert().unwrap();
    // Source untouched (functional, both-results-free).
    assert_eq!(dump(&cube.body), original);
    // Tier-2 currency; tier 3 EXACTLY NegativeVolume.
    assert_eq!(validate_closed(&reverted), Ok(()));
    assert_eq!(
        validate_geometric(&reverted, Tol::witness()),
        Err(vec![ValidationError::NegativeVolume]),
        "tier 3 on the reverted body must fail with exactly NegativeVolume"
    );
    let rvol = topo::mass_properties(&reverted, Tol::witness())
        .unwrap()
        .volume;
    assert_eq!(rvol.to_bits(), (-vol).to_bits());
    // Bitwise involution + determinism.
    assert_eq!(dump(&reverted.revert().unwrap()), original);
    assert_eq!(dump(&cube.body.revert().unwrap()), dump(&reverted));
    // Double-revert of the REVERTED body too (involution both ways).
    let twice = reverted.revert().unwrap().revert().unwrap();
    assert_eq!(dump(&twice), dump(&reverted));
}

/// TARGET 4 + 7b: ring_move (the laringmv role) on a reverted-ring
/// inventory: revert a body whose face carries a ring (the fused
/// cube+box would need planes; instead: merged-annulus below covers
/// rings + revert). Here: ring_move round-trips a ring between two
/// faces of one shell and replay is byte-stable, including the
/// documented same-face no-op.
#[test]
fn ring_move_laringmv_roundtrip_and_noop() {
    let (mut body, seed) = ops_cube_public();
    let inner = plant_detached_box(&mut body, seed.face, pt(0.3, 0.3, 1.4));
    body.movefac(seed.shell).unwrap();
    let fused = body.kfmrh(seed.face, inner).unwrap();
    let ring = fused.ring;
    assert_eq!(validate_closed(&body), Ok(()));
    let other_face = body
        .faces()
        .map(|(k, _)| k)
        .find(|&k| k != seed.face && body.get_face(k).unwrap().shell == seed.shell)
        .unwrap();
    let before = dump(&body);
    // Same-face move: documented no-op, byte-stable.
    body.ring_move(ring, seed.face).unwrap();
    assert_eq!(dump(&body), before);
    // Cross-face round trip restores the original bytes exactly.
    body.ring_move(ring, other_face).unwrap();
    assert_eq!(validate(&body), Ok(()));
    assert_eq!(body.get_face(other_face).unwrap().rings, vec![ring]);
    assert!(body.get_face(seed.face).unwrap().rings.is_empty());
    body.ring_move(ring, seed.face).unwrap();
    assert_eq!(dump(&body), before, "round trip is not byte-stable");
}

/// A face's outer-cycle vertex set (empty set for an empty loop).
fn cycle_verts_of(
    body: &Body<f64>,
    face: topo::FaceKey,
) -> std::collections::BTreeSet<topo::VertexKey> {
    let f = body.get_face(face).unwrap();
    match body.get_loop(f.outer).unwrap().boundary {
        topo::LoopBoundary::Cycle { first } => body
            .loop_cycle(first)
            .unwrap()
            .into_iter()
            .map(|he| body.get_half_edge(he).unwrap().start)
            .collect(),
        topo::LoopBoundary::Empty { .. } => std::collections::BTreeSet::new(),
    }
}

/// Subdivides the geometric cube's top face into 4 quadrants around an
/// interior square (center face), all planar and tier-3 valid. Returns
/// (cube, center_face, interior square vertex keys).
fn annulus_top_cube() -> (common::GeoCube<f64>, topo::FaceKey, [topo::VertexKey; 4]) {
    let mut cube = geometric_cube::<f64>();
    let top = cube.seed.face;
    let top_plane = plane(&[
        pt(0.0, 0.0, 1.0),
        pt(1.0, 0.0, 1.0),
        pt(1.0, 1.0, 1.0),
        pt(0.0, 1.0, 1.0),
    ]);
    let (a1, b1, c1, d1) = (
        cube.mevs[3].vertex, // A'
        cube.mevs[4].vertex, // B'
        cube.mevs[5].vertex, // C'
        cube.mevs[6].vertex, // D'
    );
    let he_at = |body: &Body<f64>, face, v| {
        let f = body.get_face(face).unwrap();
        let topo::LoopBoundary::Cycle { first } = body.get_loop(f.outer).unwrap().boundary else {
            panic!("cycle");
        };
        body.loop_cycle(first)
            .unwrap()
            .into_iter()
            .find(|&he| body.get_half_edge(he).unwrap().start == v)
            .unwrap()
    };
    let point_of =
        |body: &Body<f64>, v| *body.get_point(body.get_vertex(v).unwrap().point).unwrap();
    // Radial strut a1 -> p, then the interior chain p -> q -> r -> s.
    let (pp, pq, pr, ps) = (
        pt(0.25, 0.25, 1.0),
        pt(0.75, 0.25, 1.0),
        pt(0.75, 0.75, 1.0),
        pt(0.25, 0.75, 1.0),
    );
    let he_a1 = he_at(&cube.body, top, a1);
    let e_ap = cube
        .body
        .mev(
            MevSite::Fan {
                he1: he_a1,
                he2: he_a1,
            },
            pp,
            line(point_of(&cube.body, a1), pp),
            Tol::witness(),
        )
        .unwrap();
    let fan = |he| MevSite::Fan { he1: he, he2: he };
    let e_pq = cube
        .body
        .mev(fan(e_ap.he_minus), pq, line(pp, pq), Tol::witness())
        .unwrap();
    let e_qr = cube
        .body
        .mev(fan(e_pq.he_minus), pr, line(pq, pr), Tol::witness())
        .unwrap();
    let e_rs = cube
        .body
        .mev(fan(e_qr.he_minus), ps, line(pr, ps), Tol::witness())
        .unwrap();
    // Close the interior square: chord s -> p; the new face is the
    // side of the cycle from he2 (at p, the p->q half) around to he1 -
    // i.e. the center square.
    let closing = cube
        .body
        .mef(
            MefSite::Chords {
                he1: e_pq.he_plus,
                he2: e_rs.he_minus,
            },
            line(pp, ps),
            FaceSurface::New(top_plane.clone()),
            Tol::witness(),
        )
        .unwrap();
    // Identify the center square at runtime (4-cycle on {p,q,r,s}).
    let square: std::collections::BTreeSet<_> =
        [e_ap.vertex, e_pq.vertex, e_qr.vertex, e_rs.vertex].into();
    let center = if cycle_verts_of(&cube.body, closing.face) == square {
        closing.face
    } else {
        assert_eq!(
            cycle_verts_of(&cube.body, top),
            square,
            "no center square face"
        );
        top
    };
    assert_ne!(
        center, top,
        "construction assumes top keeps the annulus side"
    );
    // Three more radial chords: b1->q, c1->r, d1->s (each splits the
    // annulus region; all quadrants get the bit-identical top plane).
    // The host face is found at runtime: whichever face's outer cycle
    // holds both endpoints (robust to mef's side convention).
    for (corner, inner) in [(b1, e_pq.vertex), (c1, e_qr.vertex), (d1, e_rs.vertex)] {
        let host = cube
            .body
            .faces()
            .map(|(k, _)| k)
            .find(|&k| {
                let vs = cycle_verts_of(&cube.body, k);
                vs.contains(&corner) && vs.contains(&inner)
            })
            .unwrap();
        let he1 = he_at(&cube.body, host, corner);
        let he2 = he_at(&cube.body, host, inner);
        let spec = line(point_of(&cube.body, corner), point_of(&cube.body, inner));
        cube.body
            .mef(
                MefSite::Chords { he1, he2 },
                spec,
                FaceSurface::New(top_plane.clone()),
                Tol::witness(),
            )
            .unwrap();
    }
    // The center face gets a NUMERICALLY coplanar but descriptively
    // different plane (another on-plane origin) - the round-8 teeth.
    let numeric_plane = geom::Surface::Plane {
        origin: pt(0.5, 0.5, 1.0),
        normal: pt(0.0, 0.0, 1.0) - pt(0.0, 0.0, 0.0),
        u_ref: pt(1.0, 0.0, 0.0) - pt(0.0, 0.0, 0.0),
    };
    cube.body
        .set_face_surface(center, FaceSurface::New(numeric_plane))
        .unwrap();
    // Tier 2 (chord-line descriptions are not intrinsic, so tier 3's
    // TransverseNotIntrinsic applies by design; the coplanar plateau
    // cannot take the Intersection upgrade - coincident planes have no
    // intersection line).
    assert_eq!(validate_closed(&cube.body), Ok(()));
    // M4 PR 5 (N6 retirement): the quadrants' "declared" equality is
    // now a shared GeomSource — stamp every surface bit-equal to the
    // canonical top plane with ONE source (the numeric center's
    // different description stays unstamped and unmergeable).
    stamp_top_sources(&mut cube.body);
    let verts = [e_ap.vertex, e_pq.vertex, e_qr.vertex, e_rs.vertex];
    (cube, center, verts)
}

/// Stamps one shared [`topo::GeomSource`] onto every surface whose
/// plane description bit-equals the canonical top plane (f64 lane) —
/// the test-side stand-in for the recipe layer's post-op stamping.
fn stamp_top_sources(body: &mut topo::Body<f64>) {
    let canon = plane(&[
        pt(0.0, 0.0, 1.0),
        pt(1.0, 0.0, 1.0),
        pt(1.0, 1.0, 1.0),
        pt(0.0, 1.0, 1.0),
    ]);
    let geom::Surface::Plane {
        origin: co,
        normal: cn,
        u_ref: cu,
    } = canon
    else {
        panic!("canonical top plane is a plane")
    };
    let eq = |a: f64, b: f64| a.to_bits() == b.to_bits();
    let keys: Vec<_> = body
        .surfaces()
        .filter_map(|(k, s)| match *s {
            geom::Surface::Plane {
                origin: o,
                normal: n,
                u_ref: u,
            } if eq(o.x, co.x)
                && eq(o.y, co.y)
                && eq(o.z, co.z)
                && eq(n.x, cn.x)
                && eq(n.y, cn.y)
                && eq(n.z, cn.z)
                && eq(u.x, cu.x)
                && eq(u.y, cu.y)
                && eq(u.z, cu.z) =>
            {
                Some(k)
            }
            _ => None,
        })
        .collect();
    let src = topo::GeomSource::minted(1001, 0);
    for k in keys {
        body.set_surface_source(k, src.clone()).unwrap();
    }
}

/// TARGET 6: the annulus merge - four declared-equal quadrants around a
/// numerically-coplanar-but-differently-described center. The quadrants
/// merge into ONE annulus face whose hole is a genuine kemr ring on the
/// center square; the center face must survive unmerged (the round-8
/// teeth on a harder shape than the shipped diagonal). Geometry is
/// untouched: tier 3 still holds and the volume is unchanged. Replay is
/// byte-identical (D9), and the merged (all-plane, ringed) body reverts
/// with bitwise involution and the exactly-NegativeVolume posture.
#[test]
fn merge_coplanar_annulus_makes_ring_and_spares_numeric_center() {
    let build = || annulus_top_cube();
    let (mut cube, center, square) = build();
    assert_eq!(cube.body.faces().count(), 10); // 5 cube sides + 5 top
    let vol0 = topo::mass_properties(&cube.body, Tol::witness())
        .unwrap()
        .volume;
    let outcome = cube.body.merge_coplanar_faces(Tol::witness()).unwrap();
    // One group: the four quadrants; the numeric center is spared.
    assert_eq!(outcome.groups.len(), 1, "expected exactly one merged run");
    let g = &outcome.groups[0];
    assert_eq!(g.kept, cube.seed.face);
    assert_eq!(g.absorbed.len(), 3);
    assert_eq!(g.killed_edges.len(), 4, "4 radial edges must die");
    assert_eq!(g.rings_made.len(), 1, "the hole must become a kemr ring");
    assert_eq!(cube.body.faces().count(), 7); // 6 cube faces + center
    assert!(
        cube.body.get_face(center).is_some(),
        "numeric center merged!"
    );
    // The ring is the interior square's cycle, on the survivor.
    let ring = g.rings_made[0];
    assert_eq!(cube.body.get_face(g.kept).unwrap().rings, vec![ring]);
    let ring_loop = cube.body.get_loop(ring).unwrap();
    let topo::LoopBoundary::Cycle { first } = ring_loop.boundary else {
        panic!("ring must be a cycle");
    };
    let ring_verts: std::collections::BTreeSet<_> = cube
        .body
        .loop_cycle(first)
        .unwrap()
        .into_iter()
        .map(|he| cube.body.get_half_edge(he).unwrap().start)
        .collect();
    assert_eq!(
        ring_verts,
        std::collections::BTreeSet::from(square),
        "ring is not the hole boundary"
    );
    // Geometry-neutral: tier 2 holds (tier 3 is out of reach for
    // chord-line descriptions), volume unchanged (analytically the
    // same solid; allow only summation-order noise).
    assert_eq!(validate_closed(&cube.body), Ok(()));
    let vol1 = topo::mass_properties(&cube.body, Tol::witness())
        .unwrap()
        .volume;
    assert!((vol0 - vol1).abs() < 1e-12, "merge changed the volume");
    // D9 determinism across a full independent replay.
    let (mut cube2, _, _) = build();
    cube2.body.merge_coplanar_faces(Tol::witness()).unwrap();
    assert_eq!(dump(&cube.body), dump(&cube2.body));
    // Revert the merged, ring-carrying, all-plane body: involution,
    // tier-2 currency, and the negated exact volume (tier 3 is out of
    // reach for chord-line descriptions - TransverseNotIntrinsic - so
    // the volume sign carries the complement witness here).
    let original = dump(&cube.body);
    let reverted = cube.body.revert().unwrap();
    assert_eq!(validate_closed(&reverted), Ok(()));
    let rvol = topo::mass_properties(&reverted, Tol::witness())
        .unwrap()
        .volume;
    assert_eq!(rvol.to_bits(), (-vol1).to_bits());
    assert_eq!(dump(&reverted.revert().unwrap()), original);
}

/// TARGET 6/8: sharper numeric-coincidence teeth than the shipped
/// test - descriptions that agree on origin AND normal bits and differ
/// only in u_ref, and descriptions equal up to the sign of a zero
/// (-0.0 vs 0.0). Both are numerically the same plane; neither may
/// merge (and the Debug channel must distinguish -0.0 from 0.0).
#[test]
fn merge_coplanar_uref_and_signed_zero_teeth() {
    let diagonal_split = |surface: fn(&geom::Surface<f64>) -> geom::Surface<f64>| {
        let mut cube = geometric_cube::<f64>();
        let top = cube.seed.face;
        let (a1, c1) = (cube.mevs[3].vertex, cube.mevs[5].vertex);
        let f = cube.body.get_face(top).unwrap();
        let topo::LoopBoundary::Cycle { first } = cube.body.get_loop(f.outer).unwrap().boundary
        else {
            panic!("cycle");
        };
        let find = |body: &Body<f64>, v| {
            body.loop_cycle(first)
                .unwrap()
                .into_iter()
                .find(|&he| body.get_half_edge(he).unwrap().start == v)
                .unwrap()
        };
        let (he1, he2) = (find(&cube.body, a1), find(&cube.body, c1));
        let pa = *cube
            .body
            .get_point(cube.body.get_vertex(a1).unwrap().point)
            .unwrap();
        let pc = *cube
            .body
            .get_point(cube.body.get_vertex(c1).unwrap().point)
            .unwrap();
        let top_surface = cube
            .body
            .get_surface(cube.body.get_face(top).unwrap().surface)
            .unwrap()
            .clone();
        let variant = surface(&top_surface);
        cube.body
            .mef(
                MefSite::Chords { he1, he2 },
                line(pa, pc),
                FaceSurface::New(variant),
                Tol::witness(),
            )
            .unwrap();
        cube
    };
    // Tooth 1: same origin and normal bits, u_ref swapped to another
    // in-plane direction - only the FRAME differs. Must stay unmerged.
    let mut uref = diagonal_split(|s| {
        let geom::Surface::Plane {
            origin,
            normal,
            u_ref,
        } = *s
        else {
            panic!("plane");
        };
        let swapped = geom_core::Vec3::new(-u_ref.y, u_ref.x, u_ref.z);
        geom::Surface::Plane {
            origin,
            normal,
            u_ref: swapped,
        }
    });
    assert_eq!(
        uref.body
            .merge_coplanar_faces(Tol::witness())
            .unwrap()
            .groups,
        vec![]
    );
    assert_eq!(uref.body.faces().count(), 7, "u_ref-only variant merged");
    // Tooth 2: identical except normal.x = -0.0 instead of 0.0 -
    // numerically equal everywhere, one sign bit apart. Must stay
    // unmerged, proving the Debug channel is bit- (not value-) equality.
    let mut zero = diagonal_split(|s| {
        let geom::Surface::Plane {
            origin,
            normal,
            u_ref,
        } = *s
        else {
            panic!("plane");
        };
        assert_eq!(normal.x, 0.0, "test assumes a +z top normal");
        geom::Surface::Plane {
            origin,
            normal: geom_core::Vec3::new(-0.0, normal.y, normal.z),
            u_ref,
        }
    });
    assert_eq!(
        zero.body
            .merge_coplanar_faces(Tol::witness())
            .unwrap()
            .groups,
        vec![]
    );
    assert_eq!(zero.body.faces().count(), 7, "-0.0 variant merged");
}

/// TARGET 8: the f64 Debug channel's injectivity, probed directly.
/// Shortest-roundtrip Debug distinguishes -0.0/0.0, subnormals one ulp
/// apart, and near-integers - but it is NOT injective on NaN payloads:
/// every NaN prints "NaN". This pins the channel's one blind spot.
#[test]
fn f64_debug_channel_injectivity_probes() {
    // Distinguishing cases (bit-different => string-different).
    let pairs = [
        (0.0_f64, -0.0_f64),
        (f64::MIN_POSITIVE, f64::MIN_POSITIVE * 0.5), // normal vs subnormal
        (5e-324, 1e-323),                             // two subnormals
        (1.0, 1.0 + f64::EPSILON),
        (0.1, 0.1 + f64::EPSILON * 0.1),
    ];
    for (a, b) in pairs {
        assert_ne!(a.to_bits(), b.to_bits());
        assert_ne!(
            format!("{a:?}"),
            format!("{b:?}"),
            "Debug conflates {a} and {b}"
        );
    }
    // The blind spot: distinct NaN payloads/signs print identically.
    let nan1 = f64::from_bits(0x7ff8_0000_0000_0001);
    let nan2 = f64::from_bits(0xfff8_0000_0000_0002);
    assert_ne!(nan1.to_bits(), nan2.to_bits());
    assert_eq!(
        format!("{nan1:?}"),
        format!("{nan2:?}"),
        "NaNs print apart?"
    );
}

/// TARGET 8 (the exploit, FIXED): two adjacent faces carrying
/// DIFFERENT bit-patterns (distinct NaN payloads) that the old
/// Debug-string channel could not tell apart. The fix pass replaced
/// the declared-equality rung with per-component to_bits equality, so
/// these MUST stay unmerged (the empty-groups arm); a merge here is a
/// regression to the non-injective Debug channel. (Bit-IDENTICAL NaN
/// planes still compare equal by design - declared coincidence of
/// garbage; tier 3 refuses downstream.)
#[test]
fn merge_coplanar_nan_payload_debug_collision() {
    let mut cube = geometric_cube::<f64>();
    let top = cube.seed.face;
    let (a1, c1) = (cube.mevs[3].vertex, cube.mevs[5].vertex);
    let f = cube.body.get_face(top).unwrap();
    let topo::LoopBoundary::Cycle { first } = cube.body.get_loop(f.outer).unwrap().boundary else {
        panic!("cycle");
    };
    let find = |body: &Body<f64>, v| {
        body.loop_cycle(first)
            .unwrap()
            .into_iter()
            .find(|&he| body.get_half_edge(he).unwrap().start == v)
            .unwrap()
    };
    let (he1, he2) = (find(&cube.body, a1), find(&cube.body, c1));
    let pa = *cube
        .body
        .get_point(cube.body.get_vertex(a1).unwrap().point)
        .unwrap();
    let pc = *cube
        .body
        .get_point(cube.body.get_vertex(c1).unwrap().point)
        .unwrap();
    let nan_plane = |payload: u64| geom::Surface::Plane {
        origin: pt(0.5, 0.5, 1.0),
        normal: geom_core::Vec3::new(f64::from_bits(payload), 0.0, 1.0),
        u_ref: geom_core::Vec3::new(1.0, 0.0, 0.0),
    };
    let split = cube
        .body
        .mef(
            MefSite::Chords { he1, he2 },
            line(pa, pc),
            FaceSurface::New(nan_plane(0x7ff8_0000_0000_0001)),
            Tol::witness(),
        )
        .unwrap();
    cube.body
        .set_face_surface(top, FaceSurface::New(nan_plane(0x7ff8_0000_0000_0002)))
        .unwrap();
    let _ = split;
    // Tier 2 (structural) passes: the gate merge_coplanar_faces runs.
    assert_eq!(validate_closed(&cube.body), Ok(()));
    let before = dump(&cube.body);
    let outcome = cube.body.merge_coplanar_faces(Tol::witness());
    match outcome {
        Ok(o) if o.groups.is_empty() => {} // bit-different: unmerged (required)
        Ok(o) => panic!(
            "BIT-DIFFERENT NaN descriptions merged as declared-equal: {o:?} \
             (Debug channel non-injectivity laundered them)"
        ),
        Err(_) => assert_eq!(dump(&cube.body), before, "refusal must be atomic"),
    }
}

/// TARGET 6: staged-clone atomicity - an all-five merge attempt on the
/// annulus top (center given the SAME declared plane) collapses the
/// whole plateau; whatever the op decides, a refusal must leave the
/// operand byte-identical and a success must be tier-2 valid. This
/// drives the kef/kemr cascade into its hardest in-scope shape.
#[test]
fn merge_coplanar_full_plateau_atomicity() {
    let (mut cube, center, _) = annulus_top_cube();
    let top_plane = plane(&[
        pt(0.0, 0.0, 1.0),
        pt(1.0, 0.0, 1.0),
        pt(1.0, 1.0, 1.0),
        pt(0.0, 1.0, 1.0),
    ]);
    cube.body
        .set_face_surface(center, FaceSurface::New(top_plane))
        .unwrap();
    // Re-stamp: the center's replacement surface joins the shared
    // source (the M4 PR 5 form of "declared-equal center").
    stamp_top_sources(&mut cube.body);
    assert_eq!(validate_closed(&cube.body), Ok(()));
    let before = dump(&cube.body);
    match cube.body.merge_coplanar_faces(Tol::witness()) {
        Ok(o) => {
            eprintln!("full-plateau merge SUCCEEDED: {o:?}");
            // If it succeeds, the contract demands a tier-2 result and
            // a single top face with no leftover interior scaffolding.
            assert_eq!(validate_closed(&cube.body), Ok(()));
            assert_eq!(cube.body.faces().count(), 6);
        }
        Err(e) => {
            eprintln!("full-plateau merge REFUSED: {e:?}");
            assert_eq!(
                dump(&cube.body),
                before,
                "refusal mutated the operand: {e:?}"
            );
        }
    }
}

/// TARGET 8, interval lane: the Debug channel over `Interval` - probe
/// whether the interval scalar's Debug is faithful to the enclosure
/// (bounds AND
/// decoration). If two different enclosures print alike, the merge op's
/// declared-equality (and every D9 dump comparison) is blind to the
/// difference in the interval lane.
#[cfg(feature = "interval")]
#[test]
fn interval_debug_channel_faithfulness_probe() {
    use geom_core::{Interval, Real};
    let a = Interval::from_bounds(0.1, 0.2);
    let b = Interval::from_bounds(0.1, 0.2 + f64::EPSILON * 0.1);
    assert_ne!(
        format!("{a:?}"),
        format!("{b:?}"),
        "one-ulp-different interval bounds print identically: the Debug \
         bit-equality channel is NOT injective in the interval lane"
    );
    // Same bounds, different decoration: sqrt over a domain-violating
    // enclosure lands on [0, 1] with a degraded decoration, while the
    // fresh construction is Com.
    let degraded = Interval::from_bounds(-1.0, 1.0).sqrt();
    let fresh = Interval::from_bounds(0.0, 1.0);
    let (d, f) = (format!("{degraded:?}"), format!("{fresh:?}"));
    assert_ne!(
        d, f,
        "decoration difference is invisible to the Debug channel: a \
         poisoned enclosure and a clean one print identically"
    );
}

/// TARGETS 3 + 8, interval lane: split_edge through the Q1 funnel - a
/// singleton parameter splits and re-certifies; an enclosure straddling
/// the interiority band escalates typed (never silently splits); the
/// interval cube replays deterministically through the split.
#[cfg(feature = "interval")]
#[test]
fn interval_split_edge_lane() {
    use geom_core::{Interval, Real};
    let build = || geometric_cube::<Interval>();
    let mut cube = build();
    assert_eq!(validate_closed(&cube.body), Ok(()));
    let edge = cube.mevs[0].edge;
    // A straddling parameter enclosure cannot be classified interior:
    // typed escalation, atomic.
    let before = format!("{:?}", cube.body);
    let err = cube
        .body
        .split_edge(edge, Interval::from_bounds(-0.1, 0.1))
        .unwrap_err();
    assert!(
        matches!(err, EulerOpError::SplitParamEscalated { .. }),
        "straddling t must escalate, got {err:?}"
    );
    assert_eq!(format!("{:?}", cube.body), before);
    // A singleton parameter splits, and the whole thing is replayable.
    cube.body.split_edge(edge, Interval::from_f64(0.5)).unwrap();
    assert_eq!(validate(&cube.body), Ok(()));
    assert_eq!(validate_closed(&cube.body), Ok(()));
    let mut cube2 = build();
    cube2
        .body
        .split_edge(cube2.mevs[0].edge, Interval::from_f64(0.5))
        .unwrap();
    assert_eq!(format!("{:?}", cube.body), format!("{:?}", cube2.body));
}

/// TARGET 2 (the last laundering door): set_edge_curve cannot promote
/// a null edge into geometry - its endpoints are bitwise coincident,
/// so ANY certified replacement fails the forward-span gate with a
/// typed Certification error, and the body is untouched. Also pins
/// mfkrh's FaceSurface::Inherit as a same-KEY share (the spec's
/// same-key demand for section faces).
#[test]
fn null_edge_cannot_be_laundered_through_set_edge_curve() {
    let mut cube = geometric_cube::<f64>();
    let he = cube
        .body
        .get_vertex(cube.mevs[0].vertex)
        .unwrap()
        .emanating
        .unwrap();
    let created = cube
        .body
        .mev_null(MevSite::Fan { he1: he, he2: he }, NewVertexSide::Above)
        .unwrap();
    let before = dump(&cube.body);
    let p = *cube
        .body
        .get_point(cube.body.get_vertex(created.vertex).unwrap().point)
        .unwrap();
    let err = cube
        .body
        .set_edge_curve(
            created.edge,
            line(p, pt(p.x + 1.0, p.y, p.z)),
            Tol::witness(),
        )
        .unwrap_err();
    assert!(
        matches!(err, EulerOpError::Certification { .. }),
        "a null edge accepted a certified carrier: {err:?}"
    );
    assert_eq!(dump(&cube.body), before);
    cube.body.kev(created.he_plus).unwrap();
    // mfkrh Inherit = same surface key as the demoting face.
    let (mut body, seed) = ops_cube_public();
    let inner = plant_detached_box(&mut body, seed.face, pt(0.3, 0.3, 1.4));
    body.movefac(seed.shell).unwrap();
    let fused = body.kfmrh(seed.face, inner).unwrap();
    let promoted = body.mfkrh(fused.ring, FaceSurface::Inherit).unwrap();
    assert_eq!(
        body.get_face(promoted.face).unwrap().surface,
        body.get_face(seed.face).unwrap().surface,
        "Inherit did not share the demoting face's surface key"
    );
}
