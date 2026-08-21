//! **The fillet surgery's birth records** (M6-5 PR-1, kernel side) —
//! and the spec's F-e measurement, kept executed.
//!
//! F-e asked whether the composed die needs ONE fillet node or two
//! chained ones: does a single `fillet_edges` call accept the pipped
//! cube's 12 open box-edge chains together with its closed pip rim?
//! It does (`fe_single_call_...` below), so the recipe layer needs no
//! chain-handling growth and the corpus document stands on one node.
//!
//! The rest of the file is the naming contract the emitter rests on:
//! every entity of the surgery's OUTPUT is either a recorded mint or
//! a survivor of the source, with no third case and no overlap.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, dead_code)]

use geom::Surface;
use geom_core::{Affine3, Band, Point2, Point3, Tolerance, Vec2, Vec3};
use profile::RawLoop;
use profile::{Profile, ProfileLoop, ProfileVertex, SketchPlane};
use sweep::fillet::build::fillet_edges;
use sweep::test_support::cube;
use sweep::{Revolution, RevolveAxis, revolve};
use topo::boolean::{BooleanOp, SweepStrategy, boolean_op_with};
use topo::{Body, BooleanDeclarations, EdgeKey};
use geom_core::Tol;

const DIE_L: f64 = 1.0;
const PIP_R: f64 = 0.09;
const PIP_H: f64 = 0.05;
const R: f64 = 0.05;

fn band() -> Band {
    let tol = Tol::witness().get();
    Band::new(tol.eps, tol.k * tol.eps).unwrap()
}

fn ball_at(r: f64, c: Vec3<f64>) -> Body<f64> {
    let lp = ProfileLoop::new(vec![
        ProfileVertex::new(Point2::new(0.0, -r), 1.0),
        ProfileVertex::new(Point2::new(0.0, r), 0.0),
    ]);
    let vp = Profile::new(SketchPlane::xy(), vec![lp])
        .validate(Tol::witness())
        .unwrap();
    let axis = RevolveAxis {
        origin: Point2::new(0.0, 0.0),
        dir: Vec2::new(0.0, 1.0),
    };
    let ball = revolve(&vp, axis, Revolution::Full).unwrap().body;
    topo::transform_rigid(&ball, &Affine3::translation(c)).unwrap()
}

/// The die_composed pip: a ball poled along +Z, centred at `c`.
fn ball_poled_z(r: f64, c: Vec3<f64>) -> Body<f64> {
    let ball = ball_at(r, Vec3::new(0.0, 0.0, 0.0));
    let placed = topo::transform_rigid(
        &ball,
        &Affine3::rotation_about_axis(
            Point3::new(0.0, 0.0, 0.0),
            Vec3::new(1.0, 0.0, 0.0),
            core::f64::consts::FRAC_PI_2,
        ),
    )
    .unwrap();
    topo::transform_rigid(&placed, &Affine3::translation(c)).unwrap()
}

fn rim_edges(body: &Body<f64>) -> Vec<EdgeKey> {
    let kind_of = |f: topo::FaceKey| -> Option<u8> {
        match body.get_surface(body.get_face(f)?.surface)? {
            Surface::Plane { .. } => Some(0),
            Surface::Sphere { .. } => Some(1),
            _ => Some(2),
        }
    };
    let face_of = |he: topo::HalfEdgeKey| -> Option<topo::FaceKey> {
        Some(body.get_loop(body.get_half_edge(he)?.parent_loop)?.face)
    };
    let mut out = Vec::new();
    for (k, e) in body.edges() {
        let (Some(fa), Some(fb)) = (face_of(e.he_plus), face_of(e.he_minus)) else {
            continue;
        };
        if matches!(
            (kind_of(fa), kind_of(fb)),
            (Some(0), Some(1)) | (Some(1), Some(0))
        ) {
            out.push(k);
        }
    }
    out
}

/// The pipped cube of `corpus/die_composed.rs`, its 12 surviving box
/// edges and its pip rim's two arcs.
fn pipped_die() -> (Body<f64>, Vec<EdgeKey>, Vec<EdgeKey>) {
    let cube0 = cube(DIE_L);
    let box_keys: Vec<_> = cube0.edges().map(|(k, _)| k).collect();
    let pip = ball_poled_z(PIP_R, Vec3::new(0.5, 0.5, DIE_L + (PIP_R - PIP_H)));
    let pipped = boolean_op_with(
        BooleanOp::Subtract,
        &cube0,
        &pip,
        &BooleanDeclarations::none(),
        SweepStrategy::Realized,
    )
    .unwrap()
    .body()
    .expect("a body")
    .body
    .clone();
    let box_edges: Vec<_> = box_keys
        .into_iter()
        .filter(|k| pipped.get_edge(*k).is_some())
        .collect();
    let rims = rim_edges(&pipped);
    (pipped, box_edges, rims)
}

/// **The F-e measurement, executed** (M6-5 spec §0): twelve open
/// plane–plane chains and one closed plane–sphere rim, in ONE
/// `fillet_edges` call — the form the composed-die document is
/// authored in. Supported by contract, never before exercised.
#[test]
fn fe_single_call_twelve_open_chains_plus_one_closed_rim() {
    let (pipped, box_edges, rims) = pipped_die();
    assert_eq!(box_edges.len(), 12, "the twelve box edges survive");
    assert_eq!(rims.len(), 2, "the pip rim is two arcs");
    let mut all = box_edges;
    all.extend(rims);
    let out = fillet_edges(&pipped, &all, R, band())
        .expect("one call takes the open chains and the closed rim together");
    assert_eq!(out.blend_faces.len(), 12, "one blend per box edge");
    assert_eq!(out.corner_faces.len(), 8, "one octant per box corner");
    assert_eq!(out.band_faces.len(), 1, "one torus band for the pip rim");
}

/// **The birth records partition the result**: every face, edge and
/// vertex of the surgery's output is EITHER a recorded mint OR a key
/// that survived from the source — never both, never neither. This is
/// what lets the emitter name a survivor without matching geometry.
#[test]
fn every_output_entity_is_a_recorded_mint_or_a_survivor() {
    let (pipped, box_edges, rims) = pipped_die();
    let mut all = box_edges;
    all.extend(rims);
    let out = fillet_edges(&pipped, &all, R, band()).expect("the surgery");
    let rec = out.naming.as_ref().expect("the surgery keeps its records");

    let mut minted_f: Vec<_> = rec
        .blends
        .iter()
        .map(|(f, _)| *f)
        .chain(rec.corners.iter().map(|(f, _)| *f))
        .chain(rec.bands.iter().map(|(f, _)| *f))
        .collect();
    let mut minted_e: Vec<_> = rec
        .trims
        .iter()
        .map(|(e, _, _)| *e)
        .chain(rec.arcs.iter().map(|(e, _, _)| *e))
        .chain(rec.rim_trims.iter().map(|(e, _, _)| *e))
        .chain(rec.meridian_remnants.iter().map(|(e, _)| *e))
        .chain(rec.slits.iter().map(|(e, _)| *e))
        .collect();
    let mut minted_v: Vec<_> = rec
        .feet
        .iter()
        .map(|(v, _, _)| *v)
        .chain(rec.rim_feet.iter().map(|(v, _)| *v))
        .chain(rec.meridian_splits.iter().map(|(v, _)| *v))
        .collect();
    fn dedup<K: Ord + Copy>(v: &mut Vec<K>) {
        let n = v.len();
        v.sort_unstable();
        v.dedup();
        assert_eq!(n, v.len(), "a mint was recorded twice");
    }
    dedup(&mut minted_f);
    dedup(&mut minted_e);
    dedup(&mut minted_v);

    // No mint is a survivor: a minted key never existed in the source.
    for f in &minted_f {
        assert!(pipped.get_face(*f).is_none(), "a minted face reused a key");
    }
    // Split FRAGMENTS are the one construction that can carry a source
    // key: `split_edge` hands the parent key to one of its two
    // children. Both children are new entities semantically (each
    // spans a strictly shorter arc), so both are recorded as mints —
    // and the key they carry is their own parent's, never an
    // unrelated survivor's.
    let fragments: Vec<_> = rec
        .meridian_remnants
        .iter()
        .chain(rec.slits.iter())
        .collect();
    for e in &minted_e {
        let frag = fragments.iter().find(|(k, _)| k == e);
        match frag {
            Some((_, parent)) => assert!(
                e == parent || pipped.get_edge(*e).is_none(),
                "a split fragment carries a key that is neither its parent's nor fresh"
            ),
            None => assert!(pipped.get_edge(*e).is_none(), "a minted edge reused a key"),
        }
    }
    for v in &minted_v {
        assert!(
            pipped.get_vertex(*v).is_none(),
            "a minted vertex reused a key"
        );
    }

    // Every output entity is covered.
    for (f, _) in out.body.faces() {
        assert!(
            minted_f.contains(&f) || pipped.get_face(f).is_some(),
            "an output face is neither minted nor a survivor"
        );
    }
    for (e, _) in out.body.edges() {
        assert!(
            minted_e.contains(&e) || pipped.get_edge(e).is_some(),
            "an output edge is neither minted nor a survivor"
        );
    }
    for (v, _) in out.body.vertices() {
        assert!(
            minted_v.contains(&v) || pipped.get_vertex(v).is_some(),
            "an output vertex is neither minted nor a survivor"
        );
    }

    // ---- The identity, BOTH directions (M6-5 PR-2, review F-2).
    //
    // PR-1 executed only `output ⊆ source ⊎ minted`. That direction
    // alone cannot see a source entity destroyed WITHOUT a record: it
    // leaves no output key to ask about, so nothing downstream ever
    // gets a chance to refuse. The converse — `(source − retired) ⊆
    // output` — is what catches it, and it is the only claim in this
    // file that `Retired` is needed for.
    for e in &rec.dead.edges {
        assert!(out.body.get_edge(*e).is_none(), "a retired edge survived");
    }
    for v in &rec.dead.vertices {
        assert!(
            out.body.get_vertex(*v).is_none(),
            "a retired vertex survived"
        );
    }
    for (e, _) in pipped.edges() {
        assert!(
            rec.dead.edges.contains(&e) || out.body.get_edge(e).is_some(),
            "a source edge vanished without a retirement record"
        );
    }
    for (v, _) in pipped.vertices() {
        assert!(
            rec.dead.vertices.contains(&v) || out.body.get_vertex(v).is_some(),
            "a source vertex vanished without a retirement record"
        );
    }
    // Faces are never retired — a support shrinks, it does not die —
    // so the claim there is total.
    for (f, _) in pipped.faces() {
        assert!(
            out.body.get_face(f).is_some(),
            "a source face vanished; supports shrink, they do not die"
        );
    }
}

/// The counts, per the composed die's shape: 12 blends, 8 octants,
/// 1 band; 2 trimlines per box edge, 3 arcs per corner, 3 feet per
/// corner; 2 rim feet, 2 plane trims, 2 sphere trims, 2 meridian
/// splits and one slit for the single pip rim.
#[test]
fn the_records_have_the_shape_the_surgery_built() {
    let (pipped, box_edges, rims) = pipped_die();
    let mut all = box_edges;
    all.extend(rims);
    let out = fillet_edges(&pipped, &all, R, band()).expect("the surgery");
    let rec = out.naming.as_ref().expect("records");
    assert_eq!(rec.blends.len(), 12);
    assert_eq!(rec.corners.len(), 8);
    assert_eq!(rec.trims.len(), 24, "two trimlines per box edge");
    assert_eq!(rec.arcs.len(), 24, "three corner arcs per octant");
    assert_eq!(rec.feet.len(), 24, "three feet per corner");
    assert_eq!(rec.bands.len(), 1);
    assert_eq!(rec.bands[0].1.len(), 2, "the rim chain is two arcs");
    assert_eq!(rec.rim_feet.len(), 2);
    assert_eq!(rec.rim_trims.len(), 4, "two per side");
    assert_eq!(rec.meridian_splits.len(), 2);
    assert_eq!(rec.meridian_remnants.len(), 2);
    assert_eq!(rec.slits.len(), 1, "one slit per band");
}

// ------------------------------------------------------------------
// The every-edge request.
// ------------------------------------------------------------------

/// **Filleting EVERY edge of the cube is one more request through the
/// same door**, and its records say so: the twelve blends, eight
/// octants, trimlines, arcs and feet are recorded mints, and the six
/// support faces are survivors — no record, because the surgery
/// shrinks a support in place and it keeps its source key.
///
/// The census is the die's: `V=8, E=12, F=6` rounds to 24 / 48 / 26.
#[test]
fn the_every_edge_request_records_every_entity_it_mints() {
    let cube0 = cube(DIE_L);
    let edges: Vec<_> = cube0.edges().map(|(k, _)| k).collect();
    let out = fillet_edges(&cube0, &edges, R, band()).expect("the surgery");
    let rec = out.naming.as_ref().expect("the surgery keeps its records");

    assert_eq!(rec.blends.len(), 12, "one blend per source edge");
    assert_eq!(rec.corners.len(), 8, "one octant per source vertex");
    assert_eq!(rec.trims.len(), 24, "two trimlines per source edge");
    assert_eq!(rec.arcs.len(), 24, "three corner arcs per source vertex");
    assert_eq!(rec.feet.len(), 24, "one foot per (source vertex, support)");
    // Every chain here is open, so no rim row is written.
    assert!(rec.bands.is_empty() && rec.rim_trims.is_empty());
    assert!(rec.rim_feet.is_empty() && rec.slits.is_empty());
    assert!(rec.meridian_splits.is_empty() && rec.meridian_remnants.is_empty());

    assert_eq!(out.body.faces().count(), 26);
    assert_eq!(out.body.edges().count(), 48);
    assert_eq!(out.body.vertices().count(), 24);

    // TOTALITY: every output entity is a recorded mint or a survivor,
    // exactly one of the two, with the six shrunk supports the only
    // face survivors.
    let mut f: Vec<_> = rec
        .blends
        .iter()
        .map(|(k, _)| *k)
        .chain(rec.corners.iter().map(|(k, _)| *k))
        .collect();
    let mut e: Vec<_> = rec
        .trims
        .iter()
        .map(|(k, _, _)| *k)
        .chain(rec.arcs.iter().map(|(k, _, _)| *k))
        .collect();
    let mut v: Vec<_> = rec.feet.iter().map(|(k, _, _)| *k).collect();
    fn dedup<K: Ord + Copy>(x: &mut Vec<K>) {
        let n = x.len();
        x.sort_unstable();
        x.dedup();
        assert_eq!(n, x.len(), "a mint was recorded twice");
    }
    dedup(&mut f);
    dedup(&mut e);
    dedup(&mut v);
    let mut survivors = 0usize;
    for (k, _) in out.body.faces() {
        if !f.contains(&k) {
            assert!(
                cube0.get_face(k).is_some(),
                "an output face is neither a record nor a survivor"
            );
            survivors += 1;
        } else {
            assert!(cube0.get_face(k).is_none(), "a mint reuses a source key");
        }
    }
    assert_eq!(survivors, 6, "the six shrunk supports survive");
    for (k, _) in out.body.edges() {
        assert!(e.contains(&k), "an output edge has no record");
    }
    for (k, _) in out.body.vertices() {
        assert!(v.contains(&k), "an output vertex has no record");
    }

    // Every source edge and vertex is retired — every edge was
    // requested, so every corner is fully requested and fuses.
    assert_eq!(rec.dead.edges.len(), 12);
    assert_eq!(rec.dead.vertices.len(), 8);
    for (k, _) in cube0.edges() {
        assert!(rec.dead.edges.contains(&k), "a source edge is unaccounted");
    }
    for (k, _) in cube0.vertices() {
        assert!(
            rec.dead.vertices.contains(&k),
            "a source vertex is unaccounted"
        );
    }
}

/// **Every record names a LIVE source entity.** A row pointing at a
/// key the input never had would name something unnameable; the
/// emitter would refuse `MissingUpstream`, but the bug belongs here.
#[test]
fn every_every_edge_record_names_a_source_entity() {
    let cube0 = cube(DIE_L);
    let edges: Vec<_> = cube0.edges().map(|(k, _)| k).collect();
    let out = fillet_edges(&cube0, &edges, R, band()).expect("the surgery");
    let rec = out.naming.as_ref().expect("records");
    for (_, src) in &rec.blends {
        assert!(cube0.get_edge(*src).is_some(), "a blend names no source");
    }
    for (_, src) in &rec.corners {
        assert!(
            cube0.get_vertex(*src).is_some(),
            "an octant names no source"
        );
    }
    for (_, e, f) in &rec.trims {
        assert!(cube0.get_edge(*e).is_some() && cube0.get_face(*f).is_some());
    }
    for (_, v, f) in &rec.feet {
        assert!(cube0.get_vertex(*v).is_some() && cube0.get_face(*f).is_some());
    }
    for (_, v, e) in &rec.arcs {
        assert!(cube0.get_vertex(*v).is_some() && cube0.get_edge(*e).is_some());
    }
}

/// **The every-edge fillet is DETERMINISTIC** — same request, twice,
/// identical bodies.
///
/// Named for what it checks. It does NOT, and cannot, check
/// bit-PRESERVATION across a change: both runs are at the same
/// revision, so a geometry shift introduced by an edit moves them
/// together and goes unseen here. What pins the bytes is the
/// `step-export` byte-golden fixture and its `KERNEL_VOLUME_MM3`
/// sidecar, which are cross-revision by construction.
#[test]
fn the_every_edge_fillet_is_deterministic() {
    let cube0 = cube(DIE_L);
    let edges: Vec<_> = cube0.edges().map(|(k, _)| k).collect();
    let a = fillet_edges(&cube0, &edges, R, band()).expect("the surgery");
    let b = fillet_edges(&cube0, &edges, R, band()).expect("the surgery again");
    assert_eq!(
        format!("{:?}", a.body),
        format!("{:?}", b.body),
        "the fillet is deterministic"
    );
    assert_eq!(a.blend_faces, b.blend_faces);
    assert_eq!(a.corner_faces, b.corner_faces);
}
