//! **Predicate 2 reads each face three ways**: clear of a band's
//! material, wholly inside it, or crossing it (C8). A face wholly inside
//! goes with that material, as a feature wholly inside a Boolean's tool
//! goes with the tool; that carve is not built, so today the reading
//! refuses as `WhollyInside`, typed apart from the crossing
//! (`FaceClearance`, a measurement) and from a face that cannot be
//! certified either way (`FaceClearance`, a bound).
//!
//! The convex witness is the block `[0, 4]³` with a sealed box void
//! running along its edge `y = z = 0`, whose band (radius or setback
//! [`R`]) removes the material `y, z ≥ 0` outside the ball's section
//! about `(R, R)` (the fillet) or below the chord `y + z = R` (the
//! chamfer). The void's section is `[lo, hi]²` in `(y, z)`, over
//! `x ∈ [1.9, 2.1]`. The concave witness is a sealed cavity with a small
//! island standing in the corner its floor edge's band fills.
//!
//! The cut-off's own end-face reading is beside the bores it reads:
//! `band_ruled_cap_ring`, `review_band_ruled_ring_probes` and
//! `band_planar_oblique_fillet`.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Band, Bounds, Decide, ErrorTextReading, Interval, Point3, Sign, Tol};
use sweep::blend::{
    BlendError, BlendRequest, ClassifiedMargin, Convexity, chamfer_edges, fillet_edges,
};
use sweep::test_support::band_reach;
use topo::{AtRestBody, AtRestPolicy, Body, EdgeKey, EntityId, mass_properties, query};

use crate::common::cavity::{brick, cut, edges_with_corners};
use crate::common::interval::iv;

fn tol() -> Tol {
    Tol::witness()
}

fn band() -> Band {
    Band::linear(tol()).expect("the witness band")
}

/// The convex band's radius and setback.
const R: f64 = 0.5;

/// The block `[0, 4]³` less the box void `[1.9, 2.1] × [lo, hi]²`.
fn voided<T: Decide + Bounds + AtRestPolicy>(lo: f64, hi: f64) -> AtRestBody<T> {
    let t = tol();
    let block = sweep::test_support::finished(
        "block",
        sweep::test_support::brick::<T>((0.0, 4.0), (0.0, 4.0), (0.0, 4.0), t),
        t,
    );
    let void = sweep::test_support::finished(
        "void",
        sweep::test_support::brick::<T>((1.9, 2.1), (lo, hi), (lo, hi), t),
        t,
    );
    topo::subtract(&block, &void, t)
        .expect("the cut succeeds")
        .body()
        .expect("the cut leaves material")
        .body
        .clone()
}

/// The block's edge `y = z = 0`, found by position at any scalar.
fn the_edge<T: Bounds>(body: &Body<T>) -> EdgeKey {
    let near = |p: &Point3<T>, q: [f64; 3]| {
        [p.x, p.y, p.z]
            .iter()
            .zip(q)
            .all(|(c, w)| c.lo() <= w + 1e-12 && w - 1e-12 <= c.hi())
    };
    let (a, b) = ([0.0, 0.0, 0.0], [4.0, 0.0, 0.0]);
    query::all_edges(body)
        .into_iter()
        .find(|&e| {
            let he = body.get_edge(e).unwrap().he_plus;
            let p = |v| *body.get_point(body.get_vertex(v).unwrap().point).unwrap();
            let (s, t) = (
                p(body.get_half_edge(he).unwrap().start),
                p(body.half_edge_end(he).unwrap()),
            );
            (near(&s, a) && near(&t, b)) || (near(&s, b) && near(&t, a))
        })
        .expect("the block's edge y = z = 0")
}

/// The two verbs at one size.
#[derive(Clone, Copy, Debug)]
enum Verb {
    Fillet,
    Chamfer,
}

impl Verb {
    fn run(self, body: &AtRestBody<f64>, edges: &[EdgeKey], size: f64) -> Result<f64, BlendError> {
        let out = match self {
            Self::Fillet => fillet_edges(body, edges, size, tol()).map(|f| f.body),
            Self::Chamfer => chamfer_edges(body, edges, size, tol()).map(|c| c.body),
        }
        .map_err(|e| e.error)?;
        topo::validate_geometric(&out, tol()).unwrap_or_else(|e| panic!("{self:?}: tier 3, {e:?}"));
        Ok(mass_properties(&out, tol()).expect("a volume").volume)
    }

    /// The section the band removes from a right dihedral.
    fn section(self, size: f64) -> f64 {
        match self {
            Self::Fillet => size * size * (1.0 - std::f64::consts::FRAC_PI_4),
            Self::Chamfer => size * size / 2.0,
        }
    }
}

/// A refusal's `WhollyInside` payload, or a panic naming `what`.
fn wholly_inside(
    what: &str,
    e: Result<impl Sized, BlendError>,
) -> (EntityId, Convexity, ClassifiedMargin) {
    match e {
        Err(BlendError::WhollyInside { at, chain, margin }) => (at, chain, margin),
        Err(other) => panic!("{what}: expected WhollyInside, got {other:?}"),
        Ok(_) => panic!("{what}: expected WhollyInside, the blend built"),
    }
}

/// The reading of a classified margin, as a closed interval.
fn bracket(m: &ClassifiedMargin) -> (f64, f64) {
    match m.reading.diagnostic_f64_for_error_text() {
        ErrorTextReading::Value(v) => (v, v),
        ErrorTextReading::Enclosure { lo, hi } => (lo, hi),
        ErrorTextReading::Invalid => panic!("a poisoned reading: {m}"),
    }
}

/// `|origin · normal|` of a plane face: which plane of the fixture it
/// lies on.
fn plane_offset<T: Bounds>(body: &Body<T>, f: topo::FaceKey) -> f64 {
    let face = body.get_face(f).expect("a source face");
    match body.get_surface(face.surface) {
        Some(geom::Surface::Plane { origin, normal, .. }) => {
            let d = (*origin - Point3::origin()).dot(*normal);
            0.5 * (d.lo() + d.hi()).abs()
        }
        other => panic!("a plane face, got {other:?}"),
    }
}

/// Whether the face `at` names lies on one of the void's six planes.
fn on_the_void<T: Bounds>(body: &Body<T>, at: EntityId, lo: f64, hi: f64) -> bool {
    let EntityId::Face(f) = at else {
        return false;
    };
    let d = plane_offset(body, f);
    [1.9, 2.1, lo, hi].iter().any(|w| (d - w).abs() < 1e-9)
}

/// **A void wholly inside the material a convex band removes refuses
/// `WhollyInside`, under either verb**, naming a face of the void and
/// the convex chain. The margin is the face's depth read off its cells:
/// negative, and no deeper than the void's distance from the supports
/// (`0.03`), the bounds being 1-Lipschitz.
#[test]
fn a_void_wholly_inside_a_convex_bands_material_refuses_wholly_inside_under_both_verbs() {
    let body = voided::<f64>(0.03, 0.08);
    let edges = [the_edge(&body)];
    for verb in [Verb::Fillet, Verb::Chamfer] {
        let what = format!("{verb:?}");
        let (at, chain, margin) = wholly_inside(&what, verb.run(&body, &edges, R));
        assert_eq!(chain, Convexity::Convex, "{what}: the convex band");
        assert_eq!(margin.predicate, "fillet3_face_clearance", "{what}");
        assert_eq!(margin.sign, Sign::Negative, "{what}: definite, {margin}");
        let (lo, hi) = bracket(&margin);
        assert!(
            -0.03 - 1e-12 <= lo && hi < 0.0,
            "{what}: a depth within the void's 0.03 from the supports, {margin}"
        );
        assert!(
            on_the_void(&body, at, 0.03, 0.08),
            "{what}: the face named is the void's, {at:?}"
        );
        let text = BlendError::WhollyInside { at, chain, margin }.to_string();
        assert!(
            text.contains("lies wholly inside the material its band removes")
                && text.contains("consuming it is not built yet"),
            "{what}: the sentence says inside, and not built: {text}"
        );
    }
}

/// **A void crossing the band's boundary refuses as a measurement**, and
/// **one in the material the band keeps builds** at the closed form
/// `ΔV = −section·4`, under either verb. The crossing void's corner
/// `(0.1, 0.1)` is in the removed material and its corner `(0.3, 0.3)`
/// in the kept; the kept void `[0.4, 0.45]²` is inside the ball's
/// section and above the chord.
#[test]
fn a_void_crossing_the_band_refuses_and_one_in_the_kept_material_builds_under_both_verbs() {
    for verb in [Verb::Fillet, Verb::Chamfer] {
        let what = format!("{verb:?}");
        let body = voided::<f64>(0.1, 0.3);
        let e = verb.run(&body, &[the_edge(&body)], R);
        assert!(
            matches!(
                e,
                Err(BlendError::FaceClearance {
                    bounded: false,
                    chain: Convexity::Convex,
                    ..
                })
            ),
            "{what}: the crossing void is a measured FaceClearance, got {e:?}"
        );
        let body = voided::<f64>(0.4, 0.45);
        let v = verb
            .run(&body, &[the_edge(&body)], R)
            .unwrap_or_else(|e| panic!("{what}: the kept void builds, got {e:?}"));
        let want = 64.0 - 0.2 * 0.05 * 0.05 - 4.0 * verb.section(R);
        assert!((v - want).abs() < 1e-9, "{what}: V {v}, want {want}");
    }
}

/// **At the band's boundary, `1e-3` either side**: the void's corner
/// `1e-3` short of the ball's section (the fillet) or of the chord (the
/// chamfer) is wholly inside, and `1e-3` past it crosses.
#[test]
fn a_void_a_hair_inside_the_band_is_wholly_inside_and_a_hair_past_it_crosses() {
    let gap = 1e-3;
    let s2 = std::f64::consts::SQRT_2;
    for (verb, inside, across) in [
        (Verb::Fillet, R - (R + gap) / s2, R - (R - gap) / s2),
        (Verb::Chamfer, 0.5 * (R - gap * s2), 0.5 * (R + gap * s2)),
    ] {
        let what = format!("{verb:?}");
        let body = voided::<f64>(0.03, inside);
        let (at, chain, margin) = wholly_inside(&what, verb.run(&body, &[the_edge(&body)], R));
        assert_eq!(chain, Convexity::Convex, "{what}");
        assert!(on_the_void(&body, at, 0.03, inside), "{what}: {at:?}");
        assert!(bracket(&margin).1 < 0.0, "{what}: {margin}");
        let body = voided::<f64>(0.03, across);
        let e = verb.run(&body, &[the_edge(&body)], R);
        assert!(
            matches!(e, Err(BlendError::FaceClearance { bounded: false, .. })),
            "{what}: a hair past the boundary crosses, got {e:?}"
        );
    }
}

/// **The reach reads alike at `Interval`**: the inside void refuses
/// `WhollyInside` with an enclosure below zero, the crossing void a
/// measured `FaceClearance`, each naming what `f64` names.
#[test]
fn the_three_way_reading_replays_at_interval() {
    let verdict = |lo: f64, hi: f64| {
        let f = voided::<f64>(lo, hi);
        let i = voided::<Interval>(lo, hi);
        let at_f64 = band_reach(
            &BlendRequest {
                body: &f,
                edges: vec![the_edge(&f)],
                size: R,
            },
            band(),
        );
        let at_interval = band_reach(
            &BlendRequest {
                body: &i,
                edges: vec![the_edge(&i)],
                size: iv(R),
            },
            band(),
        );
        (at_f64, at_interval)
    };
    let (f, i) = verdict(0.03, 0.08);
    let (_, _, mf) = wholly_inside("f64", f);
    let (_, chain, mi) = wholly_inside("Interval", i);
    assert_eq!(chain, Convexity::Convex);
    let (lo, hi) = bracket(&mi);
    assert!(lo <= hi && hi < 0.0, "an enclosure below zero: {mi}");
    assert!(
        lo <= bracket(&mf).0 + 1e-9,
        "the enclosure reaches f64's reading: {mi} vs {mf}"
    );
    let (f, i) = verdict(0.1, 0.3);
    for (what, e) in [("f64", f), ("Interval", i)] {
        assert!(
            matches!(e, Err(BlendError::FaceClearance { bounded: false, .. })),
            "{what}: the crossing void is measured, got {e:?}"
        );
    }
}

/// **A concave band's fill wholly holding an island refuses
/// `WhollyInside` on the concave chain**, under either verb: a sealed
/// cavity `[1, 3]³` in the block `[0, 4]³`, its floor edge `y = z = 1`
/// requested alone (cut off at the end walls), and an island
/// `[1.9, 2.1] × [1.01, 1.04]²` standing `0.01` off the floor and the
/// wall, inside the fill of radius or setback `0.25` (`0.297` from the
/// ball's axis, `0.17` short of the chord).
#[test]
fn an_island_wholly_inside_a_concave_bands_fill_refuses_wholly_inside_under_both_verbs() {
    let p = Point3::new;
    let block = brick(p(0.0, 0.0, 0.0), p(4.0, 4.0, 4.0));
    let cavity = brick(p(1.0, 1.0, 1.0), p(3.0, 3.0, 3.0));
    let sealed = sweep::test_support::finished("sealed", cut("cavity", &block, &cavity), tol());
    let island = sweep::test_support::finished(
        "island",
        brick(p(1.9, 1.01, 1.01), p(2.1, 1.04, 1.04)),
        tol(),
    );
    let body = topo::union(&sealed, &island, tol())
        .expect("the union succeeds")
        .body()
        .expect("material")
        .body
        .clone();
    assert_eq!(body.solids().count(), 2, "the island is a second solid");
    let edges = edges_with_corners(&body, |q| {
        (q.y - 1.0).abs() < 1e-12 && (q.z - 1.0).abs() < 1e-12 && (0.5..3.5).contains(&q.x)
    });
    assert_eq!(edges.len(), 1, "the floor edge");
    for verb in [Verb::Fillet, Verb::Chamfer] {
        let what = format!("{verb:?}");
        let (at, chain, margin) = wholly_inside(&what, verb.run(&body, &edges, 0.25));
        assert_eq!(chain, Convexity::Concave, "{what}: the concave band fills");
        assert_eq!(margin.sign, Sign::Negative, "{what}: {margin}");
        let EntityId::Face(f) = at else {
            panic!("{what}: names a face, got {at:?}")
        };
        let d = plane_offset(&body, f);
        assert!(
            [1.9, 2.1, 1.01, 1.04].iter().any(|w| (d - w).abs() < 1e-9),
            "{what}: the face named is the island's (offset {d})"
        );
        let text = BlendError::WhollyInside { at, chain, margin }.to_string();
        assert!(
            text.contains("the material its band adds"),
            "{what}: {text}"
        );
    }
}
