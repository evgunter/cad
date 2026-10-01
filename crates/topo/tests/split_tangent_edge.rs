//! **A plane tangent to a solid along an edge classifies the contact
//! with its material.**
//!
//! The plane y + z = 2 touches a block y, z ∈ (0, 1) along its top/far
//! edge y = z = 1. That edge is convex and all the material at it lies
//! below, so it stays an ordinary edge of the below half and adds
//! nothing to the section. Alone, the block lands below whole; with a
//! slab through it at x = 1.2..1.3, the split cuts the slab only.
//!
//! One orientation of the slab case still refuses, for a reason that is
//! not the tangency:
//! `work/cleave/split-strands-a-lone-below-bisector-as-loose-ends.md`.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_core::{Point3, Tol, Vec3};
use topo::test_support::brick;
use topo::{
    Body, SectionError, SplitError, SplitJoinError, SplitPart, SplitPlane, SplitResult,
    mass_properties, plane_section, split, union, validate_closed,
};

/// The plane y + z = 2, normal (0, s·h, s·h).
fn tangent_plane(s: f64) -> SplitPlane<f64> {
    let h = std::f64::consts::FRAC_1_SQRT_2;
    SplitPlane {
        origin: Point3::new(0.0, 1.0, 1.0),
        normal: Vec3::new(0.0, s * h, s * h),
    }
}

fn unite(a: &Body<f64>, b: &Body<f64>) -> Body<f64> {
    union(a, b, Tol::witness())
        .unwrap()
        .body()
        .expect("the two bricks overlap")
        .body
        .clone()
}

fn block() -> Body<f64> {
    brick::<f64>((0.0, 1.5), (0.0, 1.0), (0.0, 1.0), Tol::witness())
}

/// A closed half's volume; `None` for an empty one.
fn volume(label: &str, part: &SplitPart<f64>) -> Option<f64> {
    part.body().map(|b| {
        assert_eq!(validate_closed(b), Ok(()), "{label}");
        mass_properties(b, Tol::witness()).unwrap().volume
    })
}

/// (the material side's half, the other half), for a plane whose
/// normal points away from the material when `s > 0`.
fn by_material(r: &SplitResult<f64>, s: f64) -> (&SplitPart<f64>, &SplitPart<f64>) {
    if s > 0.0 {
        (&r.below, &r.above)
    } else {
        (&r.above, &r.below)
    }
}

fn near(got: Option<f64>, want: f64) -> bool {
    got.is_some_and(|v| (v - want).abs() <= 1e-12 * want)
}

/// The edges of `body` lying along y = z = 1, as sorted (x₀, x₁) spans.
fn rim_spans(body: &Body<f64>) -> Vec<(f64, f64)> {
    let point = |he| {
        let v = body.get_half_edge(he).unwrap().start;
        *body.get_point(body.get_vertex(v).unwrap().point).unwrap()
    };
    let mut spans: Vec<(f64, f64)> = body
        .edges()
        .filter_map(|(_, e)| {
            let (p, q) = (point(e.he_plus), point(e.he_minus));
            (p.y == 1.0 && p.z == 1.0 && q.y == 1.0 && q.z == 1.0)
                .then(|| (p.x.min(q.x), p.x.max(q.x)))
        })
        .collect();
    spans.sort_by(|a, b| a.partial_cmp(b).unwrap());
    spans
}

/// The block alone, in both orientations: the whole block on its
/// material side, `Empty` on the other, the rim one ordinary edge, and
/// no section polygon.
#[test]
fn a_tangent_contact_standing_alone_lands_whole() {
    let body = block();
    for s in [1.0, -1.0] {
        let r = split(&body, &tangent_plane(s), Tol::witness()).unwrap();
        let (full, empty) = by_material(&r, s);
        assert!(matches!(empty, SplitPart::Empty), "s = {s}");
        assert!(near(volume("block", full), 1.5), "s = {s}");
        assert_eq!(rim_spans(full.body().unwrap()), vec![(0.0, 1.5)], "s = {s}");
        let section = plane_section(&body, &tangent_plane(s), Tol::witness()).unwrap();
        assert!(section.regions.is_empty(), "s = {s}");
    }
}

/// With the slab the contact reaches, in both operand orders, and the
/// normal (0, h, h): the section is the slab's alone. The slab's part
/// above the plane is 0.1 × 4.375; the rest of the 2.2 is below, where
/// the rim survives as two ordinary edges, one each side of the slab.
///
/// With (0, −h, −h) the direct run strands the two vertices where the
/// rim meets the slab and refuses `UnpairedLooseEnds`, and `split`'s
/// mirror rerun does not take that refusal. That is the row named in
/// the module docs, and this assertion flips when it is fixed.
#[test]
fn a_tangent_contact_meeting_a_real_section_cuts_only_the_slab() {
    let slab = brick::<f64>((1.2, 1.3), (-1.0, 2.0), (0.5, 3.0), Tol::witness());
    for (label, body) in [
        ("block ∪ slab", unite(&block(), &slab)),
        ("slab ∪ block", unite(&slab, &block())),
    ] {
        let r = split(&body, &tangent_plane(1.0), Tol::witness()).unwrap();
        let above = volume(label, &r.above);
        let below = volume(label, &r.below);
        assert!(near(above, 0.4375), "{label}: above {above:?}");
        assert!(near(below, 1.7625), "{label}: below {below:?}");
        assert_eq!(
            rim_spans(r.below.body().unwrap()),
            vec![(0.0, 1.2), (1.3, 1.5)],
            "{label}: the rim either side of the slab"
        );
        assert!(rim_spans(r.above.body().unwrap()).is_empty(), "{label}");
        let section = plane_section(&body, &tangent_plane(1.0), Tol::witness()).unwrap();
        assert_eq!(section.regions.len(), 1, "{label}: the slab's section");

        for r in [
            split(&body, &tangent_plane(-1.0), Tol::witness()).map(|_| ()),
            plane_section(&body, &tangent_plane(-1.0), Tol::witness())
                .map(|_| ())
                .map_err(|e| match e {
                    SectionError::Split(e) => e,
                    e => panic!("{label}, −n: {e:?}"),
                }),
        ] {
            assert!(
                matches!(
                    r,
                    Err(SplitError::Join(SplitJoinError::UnpairedLooseEnds { .. }))
                ),
                "{label}, −n: {r:?}"
            );
        }
    }
}
