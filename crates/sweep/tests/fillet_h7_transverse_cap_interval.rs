//! The ruled band at the CERTIFIED scalar (feature `interval`) — the
//! interval twin of `fillet_h7_transverse_cap`'s rod row and of its
//! `fillet3_cap_transverse` trio, at the f64 sibling's depth.
//!
//! The rod with a flat is built at `Interval` through the extrude door
//! — the D-profile spelling of the same body (`rod_d_profile_at`),
//! because the boolean door's scalar bound is `Decide + Bounds`, a
//! compound `test_support` is not ratified to spell — both creases
//! carve with the census delta of two cut-off bands, every cap end is
//! decided `Zero` by `fillet3_cap_transverse` (four verdicts, read off
//! the funnel's log), the arcs are exact circles described as the
//! band×cap intersection and the trimlines lines described as the
//! band's tangent contact, naming is total, the result is tier-3 valid
//! with a closed-form inventory (`volume_pad == 0`), the source's own
//! enclosure is a claim, and the carved enclosure BRACKETS the prism
//! closed form `V₀ − 2·A_section·L` narrowly enough to be one. The
//! predicate's three arms are exercised at `Interval` too: a
//! transverse cap is Zero, an oblique one (its normal derived from the
//! tilt, as the f64 twin's) refuses typed, and an in-band one escalates
//! naming the predicate.

#![cfg(feature = "interval")]
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::Curve3;
use geom_brep::EdgeDescription;
use geom_core::k_stats::{start_verdict_log, take_verdict_log};
use geom_core::{Band, Bounds, Interval, Real, Sign, Tol, Vec3};
use sweep::blend::BlendError;
use sweep::blend::battery::cap_transverse;
use sweep::blend::build::fillet_edges;
use sweep::test_support::{
    ROD_FILLET, ROD_FLAT, ROD_L, ROD_R, assert_naming_totality, rod_creases, rod_d_profile_at,
    rod_section_cut,
};
use topo::{Body, EdgeKey, VertexKey, mass_properties, validate_geometric};

fn iv(x: f64) -> Interval {
    Interval::from_f64(x)
}

/// An enclosure must contain its truth AND be a claim.
fn assert_brackets(got: Interval, truth: f64, what: &str) {
    assert!(
        got.lo() <= truth && truth <= got.hi(),
        "{what}: the enclosure must contain the truth: {got:?} vs {truth}"
    );
    assert!(
        got.hi() - got.lo() < 1e-9,
        "{what}: the enclosure must be a claim, not a shrug: width {}",
        got.hi() - got.lo()
    );
}

fn census(body: &Body<Interval>) -> (usize, usize, usize) {
    (
        body.vertices().count(),
        body.edges().count(),
        body.faces().count(),
    )
}

#[test]
fn the_rod_carves_at_the_certified_scalar_and_brackets_the_prism_closed_form() {
    let tol = Tol::witness();
    let r = ROD_FILLET;
    let source = rod_d_profile_at::<Interval>(tol);
    let creases = rod_creases(&source);
    assert_eq!(creases.len(), 2, "two ruling creases");
    let (v0, e0, f0) = census(&source);
    let p0 = mass_properties(&source, tol).expect("interval props");
    assert_eq!(p0.volume_pad, 0.0, "the source is closed-form");
    assert!(
        p0.volume.hi() - p0.volume.lo() < 1e-12,
        "the source's own enclosure is a claim: width {}",
        p0.volume.hi() - p0.volume.lo()
    );

    start_verdict_log();
    let out = fillet_edges(&source, &creases, iv(r), tol)
        .unwrap_or_else(|e| panic!("the ruled band carves at Interval, got {e:?}"));
    let log = take_verdict_log();
    let caps: Vec<_> = log
        .iter()
        .filter(|v| v.predicate == "fillet3_cap_transverse")
        .collect();
    assert_eq!(
        caps.len(),
        4,
        "four cap ends decided at Interval, one per crease end"
    );
    assert!(
        caps.iter().all(|v| v.sign == Sign::Zero),
        "every cap is transverse at the certified scalar: {caps:?}"
    );

    assert_eq!(out.blend_faces.len(), 2, "one band per crease");
    assert!(
        out.corner_faces.is_empty() && out.band_faces.is_empty(),
        "a transverse cap is not a corner and mints no closed band"
    );
    assert_eq!(
        census(&out.body),
        (v0 + 4, e0 + 6, f0 + 2),
        "the census delta of two cut-off bands"
    );
    validate_geometric(&out.body, tol).unwrap_or_else(|e| panic!("tier-3 valid, got {e:?}"));

    // The new edges' carriers and descriptions, as the f64 twin reads
    // them: arcs are circles of the band's radius described as the
    // band×cap intersection, trimlines are lines along the ruling
    // described as the band's tangent contact with its support.
    let rec = out.naming.as_ref().expect("birth records");
    let band_surfaces: Vec<_> = rec
        .blends
        .iter()
        .map(|(f, _)| out.body.get_face(*f).unwrap().surface)
        .collect();
    let certified = |e: EdgeKey| {
        out.body
            .get_curve_geom(out.body.get_edge(e).unwrap().curve)
            .and_then(|g| g.certified())
            .cloned()
            .expect("a certified edge")
    };
    assert_eq!(rec.arcs.len(), 4, "one cut-off arc per crease end");
    for (a, _, _) in &rec.arcs {
        let c = certified(*a);
        let Curve3::Circle { radius, .. } = *c.carrier() else {
            panic!("a cut-off arc is a circle, got {:?}", c.carrier());
        };
        assert!(
            radius.lo() <= r && r <= radius.hi() && radius.hi() - radius.lo() < 1e-15,
            "the section circle brackets the band's radius tightly: {radius:?}"
        );
        let EdgeDescription::Intersection { s1, s2, .. } = c.description() else {
            panic!(
                "the arc is a transverse intersection, got {:?}",
                c.description()
            );
        };
        assert!(
            band_surfaces.contains(s1) || band_surfaces.contains(s2),
            "the arc's description cites a band"
        );
    }
    assert_eq!(rec.trims.len(), 4, "one trimline per crease per support");
    for (t, _, support) in &rec.trims {
        let c = certified(*t);
        assert!(
            matches!(c.carrier(), Curve3::Line { .. }),
            "a trimline is a line"
        );
        let EdgeDescription::TangentIntersection { s1, s2, .. } = c.description() else {
            panic!("a trimline is a tangent contact, got {:?}", c.description());
        };
        let support_surface = out.body.get_face(*support).unwrap().surface;
        assert!(
            (*s1 == support_surface && band_surfaces.contains(s2))
                || (*s2 == support_surface && band_surfaces.contains(s1)),
            "the trimline's description cites the band and its support"
        );
    }
    assert_naming_totality(&source, &out, &creases, "the rod at Interval");

    let p1 = mass_properties(&out.body, tol).expect("interval props");
    assert_eq!(
        p1.volume_pad, 0.0,
        "closed-form inventory at the certified scalar too"
    );
    let cut = 2.0 * rod_section_cut(ROD_R, ROD_FLAT, r) * ROD_L;
    let v0 = 0.5 * (p0.volume.lo() + p0.volume.hi());
    assert_brackets(
        p1.volume,
        v0 - cut,
        "the carved body against the prism closed form",
    );
    assert!(
        p1.volume.hi() < p0.volume.lo(),
        "a convex band removes material, definitely: {:?} below {:?}",
        p1.volume,
        p0.volume
    );
}

/// **The two-tolerance trio for `fillet3_cap_transverse` at `Interval`**:
/// each arm is reachable and distinct at the certified scalar.
#[test]
fn cap_transverse_trio_at_the_certified_scalar() {
    let band = Band::linear(Tol::witness()).expect("a band");
    let v = VertexKey::default();
    let tau = Vec3::new(iv(0.0), iv(0.0), iv(1.0));
    let lever = iv(1.0);
    // Transverse: the cap normal IS the ruling.
    cap_transverse(v, Vec3::new(iv(0.0), iv(0.0), iv(-1.0)), tau, lever, band)
        .expect("a perpendicular cap is Zero");
    // Oblique: a definite departure refuses as the run-out it is. The
    // normal is the f64 twin's, derived from the tilt.
    let phi = 0.3f64;
    let oblique = cap_transverse(
        v,
        Vec3::new(iv(phi.sin()), iv(0.0), iv(phi.cos())),
        tau,
        lever,
        band,
    )
    .expect_err("an oblique cap refuses");
    assert!(
        matches!(oblique, BlendError::UnsupportedRunOut { .. }),
        "the oblique cap is a run-out, got {oblique:?}"
    );
    // In band: a departure between the band's zero and its escalate.
    let t = 0.5 * (band.zero() + band.escalate());
    let escalated = cap_transverse(v, Vec3::new(iv(t), iv(0.0), iv(1.0)), tau, lever, band)
        .expect_err("an in-band cap escalates");
    match escalated {
        BlendError::Escalated { source, .. } => {
            assert_eq!(source.predicate, Some("fillet3_cap_transverse"));
        }
        other => panic!("the in-band row must escalate, got {other:?}"),
    }
}
