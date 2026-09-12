//! **A loft's end profiles are the `f64` canonical form, lifted.**
//!
//! A section is `f64` data and the walls are skinned from the `f64`
//! canonical loops `loft_geometry` validates. These rows pin that the
//! body's seam struts — `seam_edges[l][j]`, the strut at CANONICAL
//! vertex `j` — join exactly that vertex's two world points, bit for
//! bit at `f64` and as point enclosures at `Interval`. So the caps are
//! the walls' own sections rather than a second canonicalization of
//! the same data made in the evaluation scalar's arithmetic: an
//! end-profile lift that moved a bit, rotated a loop's canonical start
//! or reversed its traversal fails here.
//!
//! The strut rows' expectation is built through `Profile::validate` on
//! the section at its own placement. That IS the call `loft_geometry`
//! makes at the door, so the row is not independent of the decision
//! under test; what it still pins independently is everything the
//! assembly does with that decision — which canonical vertex each
//! strut is wired to, and that `lift_onto` carries the scalars onto
//! the lane's placement without touching a bit.
//!
//! The last row is the separating one: a section whose `f64`
//! validation decides and whose `Interval` re-validation escalates
//! lofts at `Interval`, because the assembly reads the decided form
//! instead of deciding again.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Affine3, Point2, Point3, Real, Tol, Vec3};
use profile::{Profile, ProfileLoop, ProfileVertex, RawLoop, SketchPlane};
use sweep::{Section, loft_body};
use topo::{Body, EdgeKey};

/// A circle as a two-vertex bulge loop — an ARC-bearing section, so
/// the canonical form carries real decisions (segment classification,
/// traversal sense, the lex-min start) and the lift has arc carriers
/// to rebuild.
fn circle(r: f64) -> ProfileLoop<f64> {
    ProfileLoop::new(vec![
        ProfileVertex::new(Point2::new(-r, 0.0), 1.0),
        ProfileVertex::new(Point2::new(r, 0.0), 1.0),
    ])
}

fn cone_sections() -> (Vec<Section>, Vec<Affine3<f64>>) {
    (
        vec![vec![circle(1.0)], vec![circle(0.625)]],
        vec![
            Affine3::identity(),
            Affine3::translation(Vec3::new(0.0, 0.0, 1.5)),
        ],
    )
}

type Bits = (u64, u64, u64);

/// The world points of one section's CANONICAL loop 0, in canonical
/// vertex order, taken straight from the profile door at `f64`.
fn canonical_world_points(section: &Section, place: &Affine3<f64>) -> Vec<Point3<f64>> {
    let validated = Profile::new(SketchPlane::new(*place), section.clone())
        .validate(Tol::witness())
        .expect("the section validates at f64");
    validated.loops()[0]
        .vertices()
        .iter()
        .map(|v| place.transform_point(Point3::new(v.pos().x, v.pos().y, 0.0)))
        .collect()
}

/// Strut `j`'s expected endpoints, as an unordered pair of coordinate
/// bit triples: canonical vertex `j` on the bottom section and on the
/// top one.
fn expected_struts() -> Vec<[Bits; 2]> {
    let (sections, places) = cone_sections();
    let bottom = canonical_world_points(&sections[0], &places[0]);
    let top = canonical_world_points(&sections[1], &places[1]);
    assert_eq!(bottom.len(), top.len());
    bottom
        .iter()
        .zip(&top)
        .map(|(b, t)| {
            let mut pair = [bits(*b), bits(*t)];
            pair.sort_unstable();
            pair
        })
        .collect()
}

fn bits(p: Point3<f64>) -> Bits {
    (p.x.to_bits(), p.y.to_bits(), p.z.to_bits())
}

/// An edge's two endpoint positions: `he_plus`'s start and
/// `he_minus`'s, which are the edge's two ends.
fn edge_points<T: Real>(body: &Body<T>, e: EdgeKey) -> [Point3<T>; 2] {
    let ed = body.get_edge(e).unwrap();
    [ed.he_plus, ed.he_minus].map(|h| {
        let v = body.get_half_edge(h).unwrap().start;
        *body.get_point(body.get_vertex(v).unwrap().point).unwrap()
    })
}

#[test]
fn each_seam_strut_joins_its_canonical_vertexs_two_world_points() {
    let (sections, places) = cone_sections();
    let lofted = loft_body::<f64>(&sections, &places, 1, Tol::witness()).expect("the loft builds");
    let want = expected_struts();
    assert_eq!(lofted.seam_edges.len(), 1, "one loop");
    assert_eq!(lofted.seam_edges[0].len(), want.len(), "one strut a vertex");
    for (j, &e) in lofted.seam_edges[0].iter().enumerate() {
        let mut got = edge_points(&lofted.body, e).map(bits);
        got.sort_unstable();
        assert_eq!(got, want[j], "strut {j}");
    }
}

/// The interval twin: the same struts at `Interval` carry the same
/// points as POINT enclosures. Nothing widens, because the end
/// profiles are lifted rather than re-derived at the scalar.
#[cfg(feature = "interval")]
mod interval {
    use super::{Bits, cone_sections, edge_points, expected_struts};
    use geom_core::{Affine3, Bounds, Interval, Point2, Point3, Real, Tol, Vec3};
    use profile::{Profile, ProfileLoop, ProfileVertex, RawLoop, SketchPlane};
    use sweep::{LoftError, Section, SkinError, loft_body};

    fn bits(p: Point3<Interval>) -> Bits {
        for c in [p.x, p.y, p.z] {
            assert!(
                c.lo().to_bits() == c.hi().to_bits(),
                "a lifted end point is a point enclosure, got [{}, {}]",
                c.lo(),
                c.hi()
            );
        }
        (p.x.lo().to_bits(), p.y.lo().to_bits(), p.z.lo().to_bits())
    }

    /// **The separating shape.** Two vertices at `(∓h, 0)`, both bulge
    /// `b` — a pair of arcs whose carrier is far longer than the chord, so
    /// `validate`'s segment-pair gates run on quantities of wildly
    /// different magnitude. Their shared-vertex contact margin is zero at
    /// `f64` and an ENCLOSURE straddling the band's zero threshold at
    /// `Interval`: the same numbers, decided by one arithmetic and refused
    /// by the other.
    fn wide_arc(h: f64, b: f64) -> ProfileLoop<f64> {
        ProfileLoop::new(vec![
            ProfileVertex::new(Point2::new(-h, 0.0), b),
            ProfileVertex::new(Point2::new(h, 0.0), b),
        ])
    }

    fn wide_arc_pair(h: f64, b: f64) -> (Vec<Section>, Vec<Affine3<f64>>) {
        (
            vec![vec![wide_arc(h, b)], vec![wide_arc(h * 0.625, b)]],
            vec![
                Affine3::identity(),
                Affine3::translation(Vec3::new(0.0, 0.0, h * 0.3)),
            ],
        )
    }

    /// **`(h, b)` where `f64` decides and `Interval` escalates, found at
    /// the run's own ε.** The class is broad — it is every configuration
    /// whose exact-at-`f64` margin is narrower than its `Interval`
    /// enclosure — but WHICH `(h, b)` lands in it depends on ε, because
    /// the enclosure's width comes from the arithmetic's scale and the
    /// band's from the tolerance. A pinned pair would gate three different
    /// questions on the gate's three ε rows, so the ladder is searched
    /// instead, and a run where NOTHING separates fails loudly rather than
    /// passing vacuously: that would mean the behaviour this row exists
    /// for is unreachable at that ε, which is a finding, not a green.
    fn separating_pair() -> (f64, f64, Vec<Section>, Vec<Affine3<f64>>) {
        let s = Tol::witness().eps() * 1e9;
        let mut tried = Vec::new();
        for h in [0.5, 5.0, 50.0, 500.0, 5000.0] {
            for b in [1e2, 1e3, 1e4, 1e5, 1e6] {
                let (sections, places) = wide_arc_pair(h * s, b);
                if sections
                    .iter()
                    .zip(&places)
                    .all(|(sec, p)| separates(sec, p))
                    && loft_body::<f64>(&sections, &places, 1, Tol::witness()).is_ok()
                {
                    return (h * s, b, sections, places);
                }
                tried.push(format!("({h}*s, {b})"));
            }
        }
        panic!(
            "no (h, b) on the ladder both separates the two arithmetics and lofts at f64, \
             at eps = {}: tried {}",
            Tol::witness().eps(),
            tried.join(", ")
        )
    }

    /// Does this section decide at `f64` and escalate at `Interval`?
    fn separates(section: &Section, place: &Affine3<f64>) -> bool {
        let raw = Profile::new(SketchPlane::new(*place), section.clone());
        if raw.validate(Tol::witness()).is_err() {
            return false;
        }
        interval_escalates(&raw)
    }

    fn interval_escalates(raw: &Profile<f64>) -> bool {
        matches!(
            raw.map_scalar(Interval::from_f64).validate(Tol::witness()),
            Err(profile::ProfileError::Escalated { .. })
        )
    }

    /// The `f64` half of the separating fixture: it validates and it
    /// lofts. The `Interval` half — the one that moved — is in the
    /// interval module below.
    #[test]
    fn the_wide_arc_section_decides_and_lofts_at_f64() {
        let (h, b, sections, places) = separating_pair();
        loft_body::<f64>(&sections, &places, 1, Tol::witness())
            .unwrap_or_else(|e| panic!("(h = {h}, b = {b}) lofts at f64: {e:?}"));
    }

    /// **The behaviour that moved, pinned.** `separating_pair` selects
    /// a section the `f64` arithmetic decides and the `Interval`
    /// arithmetic escalates on. That escalation is what an assembly
    /// deciding the canonical form at the evaluation scalar would have
    /// met — it is the refusal this loft used to return — and the
    /// assembly does not meet it any more, because the end profiles
    /// read the form the geometry door already decided.
    ///
    /// The claim is about the SECTION-PROFILE door and stops there.
    /// Whether the rest of the pipeline completes at `Interval` for a
    /// given `(h, b)` is a question about pcurve fitting and skinning
    /// at that scalar, which this row is not about and must not gate
    /// on: asserting only the absence of the profile refusal is what
    /// keeps it a statement about the change.
    #[test]
    fn a_section_whose_interval_revalidation_escalates_is_not_refused_by_the_profile_door() {
        let (h, b, sections, places) = separating_pair();
        if let Err(e) = loft_body::<Interval>(&sections, &places, 1, Tol::witness()) {
            assert!(
                !matches!(e, LoftError::Skin(SkinError::SectionProfile { .. })),
                "(h = {h}, b = {b}) at Interval: the assembly refused the section's \
                 profile, so something decided it again: {e:?}"
            );
        }
    }

    #[test]
    fn each_seam_strut_at_interval_encloses_the_f64_world_points_exactly() {
        let (sections, places) = cone_sections();
        let lofted =
            loft_body::<Interval>(&sections, &places, 1, Tol::witness()).expect("the loft builds");
        let want = expected_struts();
        assert_eq!(lofted.seam_edges[0].len(), want.len());
        for (j, &e) in lofted.seam_edges[0].iter().enumerate() {
            let mut got = edge_points(&lofted.body, e).map(bits);
            got.sort_unstable();
            assert_eq!(got, want[j], "strut {j} at Interval");
        }
    }
}
