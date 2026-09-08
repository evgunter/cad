//! **Review probes for the spine-kind recourse (unit 3).** Each row
//! executes a request the sentence endorses, or one its sibling
//! sentences omit, and reads the rendered refusal or the carve against
//! the wording.
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

/// **C2 / Q5 — the coaxial clause endorses a rim the door refuses.**
/// The spool's torus wall meets its base plane in a latitude circle:
/// the torus and the plane are two surfaces of revolution about ONE
/// axis, so the pair is "a rim between two coaxial surfaces of
/// revolution" in the sentence's own words — and its rolling ball's
/// centre is confined to the meridian, its spine is a circle, its
/// band a torus. The door refuses it on KIND (`Meridian::trace` has no
/// torus row), with the pair roster as payload; the sentence appended
/// to that refusal endorses the configuration just refused and calls
/// it, by elimination, a pair that "needs the canal-surface
/// approximating blend".
#[test]
fn the_spine_kind_sentence_endorses_the_coaxial_torus_rim_the_door_refuses() {
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
        assert!(
            text.contains("a rim between two coaxial surfaces of revolution"),
            "the sentence endorses, in so many words, the coaxial rim it just refused:\n{text}"
        );
    }
}

/// **C3 / C5 / C4 — the ruled family's own door, followed as a caller
/// would.** "A straight edge between two supports sharing one ruling
/// direction" read by a caller with a rod is the cylinder–plane
/// crease, not a cube edge; it carves through `fillet_edges` at
/// `r = 0.1`, tier-3 valid. The assembly sentence's open clause names
/// only plane–plane links at trivalent corners, and the geometry
/// sentence says the surgery reads "planes (for a fillet's rim, also
/// a sphere cap)" — both narrower than the cylinder supports this
/// carve reads.
#[test]
fn the_ruled_crease_carves_and_two_sibling_sentences_do_not_name_it() {
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
    let open_clause = FILLET3_ASSEMBLY_RECOURSE.split(';').next().unwrap();
    assert!(
        !open_clause.contains("cylinder") && !open_clause.contains("ruling"),
        "the assembly sentence's open clause names no ruled chain: {open_clause}"
    );
    assert!(
        !FILLET3_GEOMETRY_RECOURSE.contains("cylinder"),
        "the geometry sentence names no cylinder support: {FILLET3_GEOMETRY_RECOURSE}"
    );
}
