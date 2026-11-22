//! **A corner pinned throughout the span its wall covers builds** (class
//! 1 of the loft door's pinned corners). The corner is one vertex with no
//! strut, each wall beside it has a collapsed side there (a chart
//! singularity read exactly off its net, DESIGN `[extensibility]`), and a
//! segment pinned at both ends that is the same curve in both sections
//! has no wall: the walls beside it, or the two caps, share its edge.
//! Each witness is built through `loft_body`, is a finished body (tiers
//! 1–3 and the at-rest census), and its volume is checked against one
//! computed off the sections alone: a closed form, the divergence
//! theorem over the bilinear walls a v-degree-1 loft of polygons has, or
//! the triangles of the body's mesh.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_brep::chart_singularity::{Singularity, singular_points};
use geom_core::{Affine3, Point2, Point3, Tol, Vec3};
use profile::test_support::bulge_loop;
use profile::{ProfileLoop, RawLoop, SketchPlane};
use sweep::{Lofted, loft_body};

/// The 2×2 square, corner 0 at (−1, −1), counterclockwise.
const SQUARE: [(f64, f64); 4] = [(-1.0, -1.0), (1.0, -1.0), (1.0, 1.0), (-1.0, 1.0)];

fn square() -> Vec<ProfileLoop<f64>> {
    vec![ProfileLoop::polygon(
        SQUARE.iter().map(|&(x, y)| Point2::new(x, y)),
    )]
}

/// A half disc on the chord from (−1, 0) to (1, 0): the chord, then
/// the upper semicircle back.
fn dee() -> Vec<ProfileLoop<f64>> {
    vec![bulge_loop(vec![
        (Point2::new(-1.0, 0.0), 0.0),
        (Point2::new(1.0, 0.0), 1.0),
    ])]
}

fn places(ms: &[Affine3<f64>], tol: Tol) -> Vec<SketchPlane<f64>> {
    ms.iter()
        .map(|m| SketchPlane::from_frame(geom_core::test_support::rigid(m, tol)))
        .collect()
}

fn turn(at: [f64; 3], about: [f64; 3], angle: f64) -> Affine3<f64> {
    Affine3::rotation_about_axis(
        Point3::new(at[0], at[1], at[2]),
        Vec3::new(about[0], about[1], about[2]),
        angle,
    )
}

/// The square turned about its edge x = −1 (segment 3).
fn hinge(angle: f64) -> Affine3<f64> {
    turn([-1.0, 0.0, 0.0], [0.0, -1.0, 0.0], angle)
}

/// The square turned about the diagonal through corner 0, which no other
/// corner is on.
fn about_corner(angle: f64) -> Affine3<f64> {
    turn([-1.0, -1.0, 0.0], [1.0, -1.0, 0.0], angle)
}

fn build(
    what: &str,
    sections: &[Vec<ProfileLoop<f64>>],
    ms: &[Affine3<f64>],
    v_degree: usize,
) -> Lofted<f64> {
    let tol = Tol::witness();
    loft_body::<f64>(sections, &places(ms, tol), v_degree, tol)
        .unwrap_or_else(|e| panic!("{what}: the loft builds, got {e}"))
}

/// `(V, E, F)` of the body.
fn census(l: &Lofted<f64>) -> (usize, usize, usize) {
    (
        l.body.vertices().count(),
        l.body.edges().count(),
        l.body.faces().count(),
    )
}

/// The body's own measured volume, and the half-width its quadrature
/// certifies it to.
fn measured(l: &Lofted<f64>) -> (f64, f64) {
    let tol = Tol::witness();
    let m = topo::mass_properties(&l.body, tol, topo::test_support::pinned_resolution(tol))
        .unwrap()
        .enclosure;
    (m.volume, m.volume_pad)
}

/// **The body's volume by the divergence theorem over its own faces**,
/// read off each face's surface and loops and nothing the kernel
/// measures: `⅓∫ X·N dA`, a plane face's as the fan of its loop's
/// vertices (its edges are lines), a spline wall's by composite Gauss over
/// each knot span of its chart, the whole chart its face (a loft wall's
/// boundary is its chart's), the normal `S_u × S_v` turned by the face's
/// sense.
fn divergence_volume(l: &Lofted<f64>) -> f64 {
    let body = &l.body;
    let mut volume = 0.0;
    let (g, w) = (
        [
            0.5 - 0.5 * (0.6_f64).sqrt(),
            0.5,
            0.5 + 0.5 * (0.6_f64).sqrt(),
        ],
        [5.0 / 18.0, 8.0 / 18.0, 5.0 / 18.0],
    );
    for (fk, face) in body.faces() {
        let sign = if face.sense { 1.0 } else { -1.0 };
        match body.get_surface(face.surface).unwrap() {
            geom::Surface::Plane { .. } => {
                let ps: Vec<Vec3<f64>> = body
                    .face(fk)
                    .unwrap()
                    .half_edges()
                    .into_iter()
                    .map(|he| {
                        let v = body.get_half_edge(he).unwrap().start;
                        let p = body.get_vertex(v).unwrap().point;
                        *body.get_point(p).unwrap() - Point3::origin()
                    })
                    .collect();
                for k in 1..ps.len() - 1 {
                    volume += ps[0].dot(ps[k].cross(ps[k + 1])) / 6.0;
                }
            }
            geom::Surface::Nurbs(net) => {
                let spans = |b: Vec<f64>| -> Vec<(f64, f64)> {
                    b.windows(2)
                        .flat_map(|ab| {
                            let pieces = 16;
                            (0..pieces).map(move |i| {
                                let h = (ab[1] - ab[0]) / f64::from(pieces);
                                (ab[0] + h * f64::from(i), ab[0] + h * f64::from(i + 1))
                            })
                        })
                        .collect()
                };
                let su = spans(net.knot_values_u().breakpoints().collect());
                let sv = spans(net.knot_values_v().breakpoints().collect());
                for &(u0, u1) in &su {
                    for &(v0, v1) in &sv {
                        for (gu, wu) in g.iter().zip(w) {
                            for (gv, wv) in g.iter().zip(w) {
                                let (u, v) = (u0 + (u1 - u0) * gu, v0 + (v1 - v0) * gv);
                                let jet = net.ders(u, v);
                                let x = jet.point - Point3::origin();
                                volume += sign
                                    * wu
                                    * wv
                                    * (u1 - u0)
                                    * (v1 - v0)
                                    * x.dot(jet.du.cross(jet.dv))
                                    / 3.0;
                            }
                        }
                    }
                }
            }
            other => panic!("a loft face on {other:?}"),
        }
    }
    volume
}

/// **The volume of a v-degree-1 polygon loft off its sections alone**:
/// the divergence theorem over each slab's bilinear walls (3-point
/// Gauss in each parameter, exact for their degree-2 integrand) and the
/// two caps' fans. Section `i`'s corners are `ms[i]` applied to the
/// square's.
fn ruled_volume(ms: &[Affine3<f64>]) -> f64 {
    let corner = |m: &Affine3<f64>, j: usize| {
        let (x, y) = SQUARE[j % 4];
        m.transform_point(Point3::new(x, y, 0.0)) - Point3::origin()
    };
    let fan = |m: &Affine3<f64>| {
        (1..3)
            .map(|k| corner(m, 0).dot(corner(m, k).cross(corner(m, k + 1))) / 6.0)
            .sum::<f64>()
    };
    let (g, w) = (
        [
            0.5 - 0.5 * (0.6_f64).sqrt(),
            0.5,
            0.5 + 0.5 * (0.6_f64).sqrt(),
        ],
        [5.0 / 18.0, 8.0 / 18.0, 5.0 / 18.0],
    );
    let mut volume = fan(&ms[ms.len() - 1]) - fan(&ms[0]);
    for slab in ms.windows(2) {
        for j in 0..4 {
            let (a0, a1, b0, b1) = (
                corner(&slab[0], j),
                corner(&slab[0], j + 1),
                corner(&slab[1], j),
                corner(&slab[1], j + 1),
            );
            for (u, wu) in g.iter().zip(w) {
                for (t, wt) in g.iter().zip(w) {
                    let s =
                        (a0 * (1.0 - u) + a1 * *u) * (1.0 - t) + (b0 * (1.0 - u) + b1 * *u) * *t;
                    let su = (a1 - a0) * (1.0 - t) + (b1 - b0) * *t;
                    let st = (b0 * (1.0 - u) + b1 * *u) - (a0 * (1.0 - u) + a1 * *u);
                    volume += wu * wt * s.dot(su.cross(st)) / 3.0;
                }
            }
        }
    }
    volume
}

/// The body's measured volume holds `want` within its certified
/// half-width, and a relative `slack` of it.
fn holds(what: &str, (got, pad): (f64, f64), want: f64, slack: f64) {
    assert!(
        (got - want).abs() <= pad + slack * want.abs(),
        "{what}: volume {got} ± {pad}, independent {want}"
    );
}

fn close(what: &str, got: f64, want: f64, slack: f64) {
    holds(what, (got, 0.0), want, slack);
}

/// The collapsed sides the walls carry, over every wall.
fn collapsed_sides(l: &Lofted<f64>) -> usize {
    l.body
        .faces()
        .filter_map(|(_, f)| l.body.get_surface(f.surface))
        .map(|s| {
            singular_points(s)
                .iter()
                .filter(|(w, _)| matches!(w, Singularity::Side(_)))
                .count()
        })
        .sum()
}

/// **The two-section wedge**: the square turned half a radian about its
/// edge x = −1. That edge is the hinge both caps hold, so its segment has
/// no wall, the two walls beside it are triangles with a collapsed side
/// at the hinge's ends, and the body is a prism on the triangle of side
/// 2, 2 at the angle: `4 sin(0.5)`.
#[test]
fn the_two_section_wedge_builds_as_its_prism() {
    let l = build(
        "wedge",
        &vec![square(); 2],
        &[Affine3::identity(), hinge(0.5)],
        1,
    );
    assert_eq!(census(&l), (6, 9, 5), "wedge: a triangular prism's census");
    assert_eq!(
        l.side_faces[0][3],
        vec![None],
        "wedge: no wall on the hinge"
    );
    assert_eq!(
        (l.seam_edges[0][0][0], l.seam_edges[0][3][0]),
        (None, None),
        "wedge: no strut at the hinge's ends"
    );
    assert_eq!(
        collapsed_sides(&l),
        2,
        "wedge: one collapsed side per triangle wall"
    );
    let want = 4.0 * 0.5_f64.sin();
    holds("wedge, closed form", measured(&l), want, 1e-9);
    close("wedge, its faces", divergence_volume(&l), want, 1e-12);
    close(
        "wedge, ruled",
        ruled_volume(&[Affine3::identity(), hinge(0.5)]),
        want,
        1e-12,
    );
}

/// **One corner pinned in a two-section loft**, turned about the
/// diagonal through it: two triangle walls beside it, two warped quads.
#[test]
fn one_corner_pinned_in_a_wedge_builds() {
    let ms = [Affine3::identity(), about_corner(0.5)];
    let l = build("one corner", &vec![square(); 2], &ms, 1);
    assert_eq!(census(&l), (7, 11, 6), "one corner: one strut fewer");
    assert_eq!(
        collapsed_sides(&l),
        2,
        "one corner: the two walls beside it"
    );
    let want = ruled_volume(&ms);
    holds("one corner", measured(&l), want, 1e-9);
    close("one corner, its faces", divergence_volume(&l), want, 1e-12);
}

/// **A corner pinned through three sections**: at v-degree 1 each slab's
/// walls are their own; at v-degree 2 one wall spans every section,
/// collapsed at the corner over the whole span.
#[test]
fn a_corner_pinned_through_three_sections_builds() {
    let ms = [Affine3::identity(), about_corner(0.3), about_corner(0.6)];
    let one = build("three sections, v-degree 1", &vec![square(); 3], &ms, 1);
    let want = ruled_volume(&ms);
    holds("three sections, v-degree 1", measured(&one), want, 1e-9);
    let two = build("three sections, v-degree 2", &vec![square(); 3], &ms, 2);
    assert_eq!(
        collapsed_sides(&two),
        2,
        "v-degree 2: the two walls beside it"
    );
    holds(
        "three sections, v-degree 2",
        measured(&two),
        divergence_volume(&two),
        1e-9,
    );
}

/// **A hinge through three sections at v-degree 1**: section 1 is the
/// square turned about its edge y = +1, section 2 the square lifted, so
/// the hinge is pinned in slab 0 alone; its rim in section 1 is the far
/// rim of the slab-0 walls beside it and the base of slab 1's wall.
#[test]
fn a_hinge_in_one_slab_builds() {
    let ms = [
        Affine3::identity(),
        turn([0.0, 1.0, 0.0], [1.0, 0.0, 0.0], -0.3),
        Affine3::translation(Vec3::new(0.0, 0.0, 2.0)),
    ];
    let l = build("hinge in slab 0", &vec![square(); 3], &ms, 1);
    assert_eq!(
        l.side_faces[0][2][0], None,
        "hinge: no wall on it in slab 0"
    );
    assert!(
        l.side_faces[0][2][1].is_some(),
        "hinge: a wall on it in slab 1"
    );
    holds("hinge in slab 0", measured(&l), ruled_volume(&ms), 1e-9);
}

/// **A half disc turned about its chord**: the chord is a hinge both
/// caps hold, and the arc's wall a lune with a collapsed side at each
/// end.
#[test]
fn a_half_disc_turned_about_its_chord_builds_a_lune() {
    let ms = [Affine3::identity(), turn([0.0; 3], [1.0, 0.0, 0.0], 0.8)];
    let l = build("half disc", &vec![dee(); 2], &ms, 1);
    assert_eq!(census(&l), (2, 3, 3), "half disc: two caps and a lune");
    assert_eq!(collapsed_sides(&l), 2, "half disc: the lune's two sides");
    holds("half disc", measured(&l), divergence_volume(&l), 1e-9);
}

/// **A side one ulp wide is no collapsed side** (DESIGN
/// `[extensibility]`: no band decides a collapse), so a face that holds
/// an edge along it holds an edge one ulp long, which the attach gate
/// refuses. The other shape, a face that walks such a side with no edge
/// on it, refuses at its joint's chart gap (`topo`'s
/// `collapsed_side_rows`). The loft never meets either: its door writes
/// a corner it decides pinned as one point.
#[test]
fn an_edge_along_a_side_one_ulp_wide_refuses_at_the_attach_gate() {
    use topo::construction::{Construct, MevSite};
    let tol = Tol::witness();
    let p = Point3::new(1.0, 1.0, 0.0);
    let q = Point3::new(f64::from_bits(1.0_f64.to_bits() + 1), 1.0, 0.0);
    let mut body = topo::Body::<f64>::new();
    let seed = body.mvfs(p, true).unwrap();
    let out = body.mev(
        MevSite::Lone {
            r#loop: seed.r#loop,
        },
        q,
        geom_brep::EdgeCurveSpec::line_between(p, q),
        tol,
    );
    assert!(out.is_err(), "an edge one ulp long is refused, got {out:?}");
}
