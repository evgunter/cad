//! **Review probes for the spine-kind recourse (unit 3), adopted into
//! the fix.** Each row executes a real request and reads the rendered
//! refusal, or the carve, against the wording that reaches the caller.
//!
//! Both rows arrived as the review's evidence that two sentences
//! under-described their doors, and both are kept with their asserts
//! turned round the moment the wording was fixed: what they measured
//! as a defect they now hold as the contract.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::Surface;
use sweep::Revolution;
use sweep::blend::battery::arm_roster;
use sweep::blend::{
    BlendError, FILLET3_ASSEMBLY_RECOURSE, FILLET3_GEOMETRY_RECOURSE, FILLET3_SPINE_KIND_RECOURSE,
    fillet_edges,
};
use sweep::test_support::{ROD_FILLET, rod_creases, rod_with_flat, spool};
use topo::{Body, EdgeKey, query, validate_geometric};

fn tol() -> geom_core::Tol {
    geom_core::Tol::witness()
}

fn surfaces_of(body: &Body<f64>, e: EdgeKey) -> (Surface<f64>, Surface<f64>) {
    let ed = body.get_edge(e).unwrap();
    let surface = |he| {
        let h = body.get_half_edge(he).unwrap();
        let f = body.get_loop(h.parent_loop).unwrap().face;
        body.get_surface(body.get_face(f).unwrap().surface)
            .cloned()
            .unwrap()
    };
    (surface(ed.he_plus), surface(ed.he_minus))
}

/// **C2 / Q5 — the coaxial torus rim is refused on KIND, and the
/// sentence says so.** The spool's torus wall meets its base plane in
/// a latitude circle: the torus and the plane are two surfaces of
/// revolution about ONE axis, and the rolling ball's spine there IS a
/// circle. So the symmetry hypothesis holds and the door still
/// refuses — on kind, because `Meridian::trace` has no torus row and
/// `coaxial_arm` no torus pair — with the arm roster as payload.
///
/// That is why the sentence leads with its four KINDS and only then
/// names the two families: a wording that led with "a rim between two
/// coaxial surfaces of revolution" endorsed, in so many words, the
/// configuration this row just had refused. This row holds the fixed
/// contract from the caller's side — the kind clause admits no torus,
/// so the refusal and the recourse agree.
#[test]
fn the_spine_kind_sentence_refuses_the_coaxial_torus_rim_on_kind() {
    let s = spool(Revolution::Full, tol());
    let rims: Vec<EdgeKey> = query::all_edges(&s)
        .into_iter()
        .filter(|&e| {
            let (a, b) = surfaces_of(&s, e);
            let tp = |x: &Surface<f64>, y: &Surface<f64>| {
                matches!(x, Surface::Torus { .. }) && matches!(y, Surface::Plane { .. })
            };
            tp(&a, &b) || tp(&b, &a)
        })
        .collect();
    assert!(!rims.is_empty(), "the spool has torus–plane rims");
    for e in rims {
        let (a, b) = surfaces_of(&s, e);
        let (t_axis, p_normal) = match (&a, &b) {
            (Surface::Torus { axis, .. }, Surface::Plane { normal, .. })
            | (Surface::Plane { normal, .. }, Surface::Torus { axis, .. }) => (*axis, *normal),
            _ => unreachable!(),
        };
        assert!(
            t_axis.cross(p_normal).norm() < 1e-12,
            "the torus axis and the plane normal are one axis: coaxial surfaces of revolution"
        );
        let err = fillet_edges(&s, &[e], 0.05, tol())
            .map(|_| ())
            .expect_err("the torus–plane rim refuses");
        let BlendError::SpineUnsupported { supports, .. } = err.error else {
            panic!("refused as spine-unsupported, got {:?}", err.error)
        };
        assert_eq!(
            supports,
            arm_roster(),
            "refused on KIND — the pair roster, not the coaxiality hypothesis"
        );
        let text = err.to_string();
        assert!(text.contains(FILLET3_SPINE_KIND_RECOURSE));
        // The sentence a caller reads here must not name TORUS among
        // the support kinds it admits, or it would endorse the pair
        // this row just watched refuse. The kind clause is the
        // sentence's first `;`-delimited segment.
        let kind_clause = FILLET3_SPINE_KIND_RECOURSE
            .split(';')
            .next()
            .expect("the sentence's kind clause is its first ;-delimited segment");
        assert!(
            !kind_clause.contains("torus"),
            "the refusal is on kind, so the recourse must not admit a torus support:\n{text}"
        );
        assert!(
            kind_clause.contains("plane")
                && kind_clause.contains("sphere")
                && kind_clause.contains("cylinder")
                && kind_clause.contains("cone"),
            "and it must name the four kinds the arm table does trace:\n{text}"
        );
    }
}

/// **C3 / C5 / C4 — the ruled family's own door, followed as a caller
/// would, and named by all three sentences that describe it.** "A
/// ruling shared by two supports that are each a plane or a cylinder"
/// read by a caller with a rod is the cylinder–plane crease, not a
/// cube edge; it carves through `fillet_edges`, tier-3 valid.
///
/// The same carve is described by two SIBLING sentences, and both
/// under-described it: the assembly sentence's open clause named only
/// plane–plane links at trivalent corners, and the geometry sentence
/// said the surgery reads "planes (for a fillet's rim, also a sphere
/// cap)" — narrower than the cylinder supports and the transverse-cap
/// termination this one carve exercises. Both now name it, and this
/// row is what holds them to it.
#[test]
fn the_ruled_crease_carves_and_all_three_sentences_name_it() {
    let rod = rod_with_flat(tol());
    let creases = rod_creases(&rod);
    assert_eq!(creases.len(), 2, "two cylinder–plane creases");
    for &e in &creases {
        let (a, b) = surfaces_of(&rod, e);
        assert!(
            matches!(
                (&a, &b),
                (Surface::Cylinder { .. }, Surface::Plane { .. })
                    | (Surface::Plane { .. }, Surface::Cylinder { .. })
            ),
            "a cylinder–plane crease"
        );
    }
    let out = fillet_edges(&rod, &creases, ROD_FILLET, tol())
        .unwrap_or_else(|e| panic!("the ruled crease carves, got {e}"));
    validate_geometric(&out.body, tol()).expect("tier 3");
    // The assembly sentence's OPEN clause — its first `;`-delimited
    // segment — must name this termination, not only the plane–plane
    // link at a trivalent corner.
    let open_clause = FILLET3_ASSEMBLY_RECOURSE.split(';').next().unwrap();
    for want in ["cylinder", "ruling", "TRANSVERSE CAPS"] {
        assert!(
            open_clause.contains(want),
            "the assembly sentence's open clause must name the ruled chain ({want:?} \
             missing): {open_clause}"
        );
    }
    // And the geometry sentence must name the support kinds the
    // surgery actually reads, cylinder among them.
    for want in ["cylinder", "cone", "sphere", "plane"] {
        assert!(
            FILLET3_GEOMETRY_RECOURSE.contains(want),
            "the geometry sentence must name the {want} support the surgery reads: \
             {FILLET3_GEOMETRY_RECOURSE}"
        );
    }
}
