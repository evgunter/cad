//! **A corner pinned throughout the span its wall covers builds** (class
//! 1 of the loft door's pinned corners). The corner is one vertex with no
//! strut, each wall beside it has a collapsed side there (a chart
//! singularity read exactly off its net, DESIGN `[extensibility]`), and a
//! segment pinned at both ends that is the same curve in both sections
//! has no wall: the walls beside it, or the two caps, share its edge.
//! Each witness is built through `loft_body`, is a finished body (tiers
//! 1–3 and the at-rest census), and its volume is checked against one
//! the kernel does not measure: a closed form, the divergence theorem
//! over the bilinear walls a v-degree-1 loft of polygons has (off the
//! sections alone), or the divergence theorem over the body's own faces
//! (off its stored surfaces, by quadrature written here).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom_brep::chart_singularity::{Singularity, singular_points};
use geom_core::{Affine3, Point2, Point3, Tol, Vec3};
use profile::test_support::bulge_loop;
use profile::{ProfileLoop, SketchPlane};
use sweep::{Lofted, loft_body};

use sweep::test_support::PINNED_SQUARE as SQUARE;

fn square() -> Vec<ProfileLoop<f64>> {
    sweep::test_support::pinned_square()
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
    sweep::test_support::rigid_places(ms, tol)
}

fn turn(at: [f64; 3], about: [f64; 3], angle: f64) -> Affine3<f64> {
    sweep::test_support::turn_about(at, about, angle)
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

/// Three-point Gauss–Legendre on `[0, 1]`: its nodes and weights, exact
/// for polynomials of degree five.
fn gauss3() -> ([f64; 3], [f64; 3]) {
    let h = 0.5 * 0.6_f64.sqrt();
    (
        [0.5 - h, 0.5, 0.5 + h],
        [5.0 / 18.0, 8.0 / 18.0, 5.0 / 18.0],
    )
}

/// **The body's volume by the divergence theorem over its own faces**,
/// read off each face's surface and loops and nothing the kernel
/// measures: `⅓∫ X·N dA`, a plane face's as the fan of its loop's
/// vertices, which is exact where its edges are lines or its plane holds
/// the origin (`X·N` is then zero on it), a spline wall's by composite
/// Gauss over
/// each knot span of its chart, the whole chart its face (a loft wall's
/// boundary is its chart's), the normal `S_u × S_v` turned by the face's
/// sense.
fn divergence_volume(l: &Lofted<f64>) -> f64 {
    let body = &l.body;
    let mut volume = 0.0;
    let (g, w) = gauss3();
    for (fk, face) in body.faces() {
        let sign = if face.sense { 1.0 } else { -1.0 };
        match body.face_surface(fk).unwrap() {
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
    let (g, w) = gauss3();
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
        .filter_map(|(fk, _)| l.body.face_surface(fk).ok())
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
        vec![vec![None]],
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

/// The square turned about its edge y = +1 (segment 2) by −0.3 rad.
fn about_its_far_edge() -> Affine3<f64> {
    turn([0.0, 1.0, 0.0], [1.0, 0.0, 0.0], -0.3)
}

fn lift(z: f64) -> Affine3<f64> {
    Affine3::translation(Vec3::new(0.0, 0.0, z))
}

/// **A hinge in one slab, first, middle or last, at v-degree 1**: the
/// slab that turns the square about its edge y = +1 holds that segment
/// flat, so its walls beside it are triangles and it has none on the
/// segment. Each case loses the two corners' struts there, one rim, and
/// one wall, against the census of the same stack with no hinge: through
/// three sections (12, 20, 10) becomes (10, 17, 9), through four (16, 28,
/// 14) becomes (14, 25, 13). In the last slab the hinge's rim is the top
/// cap's; in the first, the bottom cap's; in the middle, an interior
/// section's rim, the far rim of the slab below.
#[test]
fn a_hinge_in_its_first_middle_or_last_slab_builds() {
    let top = lift(2.0) * about_its_far_edge();
    for (what, ms, flat, census_want) in [
        (
            "first of two slabs",
            vec![Affine3::identity(), about_its_far_edge(), lift(2.0)],
            0,
            (10, 17, 9),
        ),
        (
            "last of two slabs",
            vec![lift(-2.0), Affine3::identity(), about_its_far_edge()],
            1,
            (10, 17, 9),
        ),
        (
            "last of three slabs",
            vec![
                lift(-4.0),
                lift(-2.0),
                Affine3::identity(),
                about_its_far_edge(),
            ],
            2,
            (14, 25, 13),
        ),
        (
            "middle of three slabs",
            vec![lift(-2.0), Affine3::identity(), about_its_far_edge(), top],
            1,
            (14, 25, 13),
        ),
    ] {
        let l = build(what, &vec![square(); ms.len()], &ms, 1);
        assert_eq!(census(&l), census_want, "{what}: census");
        let walls = &l.side_faces[0][2][0];
        assert!(
            walls
                .iter()
                .enumerate()
                .all(|(s, w)| w.is_none() == (s == flat)),
            "{what}: segment 2 has no wall in slab {flat} alone, read {walls:?}"
        );
        holds(what, measured(&l), ruled_volume(&ms), 1e-9);
        close(what, divergence_volume(&l), ruled_volume(&ms), 1e-12);
    }
}

/// The half disc's turn about its chord, and the lune's volume
/// `(2/3)·sin 0.8`.
fn turned_dee() -> (Affine3<f64>, f64) {
    (
        turn([0.0; 3], [1.0, 0.0, 0.0], 0.8),
        2.0 / 3.0 * 0.8_f64.sin(),
    )
}

/// **A half disc turned about its chord**: the chord is a hinge both
/// caps hold, and the arc's wall a lune cut at its sub-arc joint into two
/// quarter walls, each with a collapsed side at its end of the chord,
/// whose volume is `(2/3)·sin 0.8`.
#[test]
fn a_half_disc_turned_about_its_chord_builds_a_lune() {
    let (turned, lune) = turned_dee();
    let l = build(
        "half disc",
        &vec![dee(); 2],
        &[Affine3::identity(), turned],
        1,
    );
    assert_eq!(
        census(&l),
        (4, 6, 4),
        "half disc: two caps and a lune of two quarter walls"
    );
    assert_eq!(collapsed_sides(&l), 2, "half disc: the lune's two sides");
    holds("half disc", measured(&l), lune, 1e-9);
    close("half disc, its faces", divergence_volume(&l), lune, 1e-9);
}

/// **The lune in the last slab**: stacked under a lifted half disc, the
/// hinge is the last slab's and the lune (two quarter walls) its only
/// walls there.
#[test]
fn a_half_disc_turned_in_the_last_slab_builds_a_lune_on_a_straight_slab() {
    let (turned, lune) = turned_dee();
    let stacked = build(
        "half disc in the last slab",
        &vec![dee(); 3],
        &[lift(-2.0), Affine3::identity(), turned],
        1,
    );
    assert_eq!(
        census(&stacked),
        (7, 12, 7),
        "half disc in the last slab: a straight slab under the lune"
    );
    let cylinder = core::f64::consts::FRAC_PI_2 * 2.0;
    holds(
        "half disc in the last slab",
        measured(&stacked),
        cylinder + lune,
        1e-9,
    );
}

/// **A side one ulp wide is no collapsed side** (DESIGN
/// `[extensibility]`: no band decides a collapse), so a face whose loop
/// runs along it with an edge there holds an edge one ulp long, which
/// the attach gate refuses: the edge's span meter collapses, the spline
/// carrier's reading of a stated interval that is not forward (a line's
/// is `IntervalNotForward`). The edge is the side's own iso, on a bilinear net whose `u = 0` column runs
/// from (1, 1, 0) to the point an ulp along `x`. The other shape, a face
/// that walks such a side with no edge on it, refuses at its joint's
/// chart gap (`topo`'s `collapsed_side_rows`). The loft never meets
/// either: its door writes a corner it decides pinned as one point.
#[test]
fn an_edge_along_a_side_one_ulp_wide_refuses_at_the_attach_gate() {
    use geom_core::spline::{KnotVector, Knots};
    use topo::construction::{Construct, FaceSurface, MevSite};
    let tol = Tol::witness();
    let p = Point3::new(1.0, 1.0, 0.0);
    let q = Point3::new(f64::from_bits(1.0_f64.to_bits() + 1), 1.0, 0.0);
    let kv = || Knots::from(KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap());
    let net = geom::NurbsSurface::new(
        kv(),
        kv(),
        vec![p, q, Point3::new(3.0, 0.0, 0.0), Point3::new(3.0, 2.0, 0.0)],
        vec![1.0; 4],
    )
    .unwrap();
    assert!(
        singular_points(&geom::Surface::Nurbs(std::sync::Arc::new(net.clone()))).is_empty(),
        "one ulp wide: no collapsed side"
    );
    let mut body = topo::Body::<f64>::new();
    let seed = body.mvfs(p, true).unwrap();
    let chart = body
        .set_face_surface(
            seed.face,
            FaceSurface::New {
                surface: geom::Surface::Nurbs(std::sync::Arc::new(net.clone())),
                sense: true,
            },
        )
        .unwrap();
    let side = geom_brep::boundary_iso_u(&net, false).unwrap();
    let out = body.mev(
        MevSite::Lone {
            r#loop: seed.r#loop,
        },
        q,
        geom_brep::EdgeCurveSpec {
            description: geom_brep::EdgeDescriptionSpec::iso(chart, 0.0, 0.0, 1.0, 0.0, 1.0),
            carrier: geom::Curve3::Nurbs(std::sync::Arc::new(side)),
            param_start: 0.0,
            param_end: 1.0,
        },
        tol,
    );
    let read = format!("{out:?}");
    assert!(
        read.contains("SpanMeterCollapsed"),
        "an edge along the side one ulp long refuses its span, read {read}"
    );
}

/// **Tier 3 reads a spline face's collapsed side as it reads a pole**: a
/// side's point in the face's closure is a vertex of it. The wedge's
/// wall 0, a triangle collapsed at corner 0, has its collapsed column
/// moved to the midpoint of its bottom rim, inside that edge: tier 3
/// refuses the face `ChartSingularityNotAVertex` inside the rim, where
/// the wedge as built passes.
#[test]
fn a_collapsed_side_inside_an_edge_is_refused_at_rest() {
    use topo::construction::{Construct, FaceSurface};
    let tol = Tol::witness();
    let ms = [Affine3::identity(), hinge(0.5)];
    let mut lofted = sweep::test_support::loft_body_construction::<f64>(
        &vec![square(); 2],
        &places(&ms, tol),
        1,
        tol,
    )
    .unwrap();
    assert_eq!(
        topo::validate_geometric(&lofted.body, tol),
        Ok(()),
        "the wedge as built"
    );
    let wall = lofted.side_faces[0][0][0][0].unwrap();
    let Ok(geom::Surface::Nurbs(net)) = lofted.body.face_surface(wall).cloned() else {
        panic!("wall 0 is a spline wall")
    };
    let (_, nv) = net.control_counts();
    let (p0, a) = (net.control()[0], net.control()[nv]);
    let mid = p0 + (a - p0) * 0.5;
    let moved = net.map_points_indexed(|i, p| if i < nv { mid } else { p });
    lofted
        .body
        .set_face_surface_unvouched_for_tests(
            wall,
            FaceSurface::New {
                surface: geom::Surface::Nurbs(std::sync::Arc::new(moved)),
                sense: true,
            },
        )
        .unwrap();
    let findings = topo::validate_geometric(&lofted.body, tol).unwrap_err();
    assert!(
        findings.iter().any(|e| matches!(
            e,
            topo::ValidationError::ChartSingularityNotAVertex {
                face,
                point: Singularity::Side(_),
                lies: topo::SingularityLies::InsideEdge { .. },
            } if *face == wall
        )),
        "the side's point inside the rim is refused, read {findings:?}"
    );
}

/// **A pinned corner stays one point through the skin's solve**: a
/// corner pinned through four sections at v-degree 3, the stack set off
/// at (0.1, 0.7, 1/3) so its coordinates are no short binary fractions,
/// is one stored point in every row of the wall's net only because the
/// skin passes a constant row through unsolved; the interpolation's
/// solve rounds it apart. Both walls beside it read their collapsed side,
/// and the body builds.
#[test]
fn a_corner_pinned_through_a_cubic_solve_stays_one_point() {
    let at = Affine3::translation(Vec3::new(0.1, 0.7, 1.0 / 3.0));
    let ms: Vec<Affine3<f64>> = [0.0, 0.2, 0.45, 0.7]
        .into_iter()
        .map(|a| at * about_corner(a))
        .collect();
    let l = build("cubic", &vec![square(); 4], &ms, 3);
    assert_eq!(
        collapsed_sides(&l),
        2,
        "cubic: the two walls beside the corner"
    );
    holds("cubic", measured(&l), divergence_volume(&l), 1e-9);
}

/// **The volume of a two-section v-degree-1 loft of a curved profile,
/// off its sections alone**: the divergence theorem over the ruled wall
/// joining `p(u)` placed by `m0` to `p(u)` placed by `m1` (the loft pairs
/// the two sections' curves at one parameter, and both are one profile),
/// by 3-point Gauss on `pieces` spans of `u ∈ [0, 1)` and in the ruling,
/// and the two caps, each its plane's offset from the origin times the
/// profile's `area`. `p` and `dp` are the profile and its derivative,
/// counterclockwise.
pub(crate) fn ruled_curve_volume(
    p: impl Fn(f64) -> (f64, f64),
    dp: impl Fn(f64) -> (f64, f64),
    area: f64,
    (m0, m1): (&Affine3<f64>, &Affine3<f64>),
    pieces: u32,
) -> f64 {
    let at = |m: &Affine3<f64>, (x, y): (f64, f64)| {
        m.transform_point(Point3::new(x, y, 0.0)) - Point3::origin()
    };
    let along = |m: &Affine3<f64>, (x, y): (f64, f64)| m.transform_vec(Vec3::new(x, y, 0.0));
    let normal = |m: &Affine3<f64>| m.transform_vec(Vec3::new(0.0, 0.0, 1.0));
    let mut volume =
        area * (normal(m1).dot(at(m1, (0.0, 0.0))) - normal(m0).dot(at(m0, (0.0, 0.0))));
    let (g, w) = gauss3();
    for i in 0..pieces {
        let h = 1.0 / f64::from(pieces);
        for (gu, wu) in g.iter().zip(w) {
            let u = h * (f64::from(i) + gu);
            let (a, b) = (at(m0, p(u)), at(m1, p(u)));
            let (da, db) = (along(m0, dp(u)), along(m1, dp(u)));
            for (t, wt) in g.iter().zip(w) {
                let x = a * (1.0 - t) + b * *t;
                let xu = da * (1.0 - t) + db * *t;
                volume += h * wu * wt * x.dot(xu.cross(b - a));
            }
        }
    }
    volume / 3.0
}

/// **A sub-arc joint pinned while its segment moves builds.** The half
/// disc's semicircle is two quarter walls joined at its apex `(0, 1)`;
/// turned about the line through the apex along `x`, the apex is one
/// world point in both sections while the chord and the rest of the arc
/// move. The stacking door pins the joint as it pins a corner, so it is
/// one vertex with no strut, and the two quarter walls each have a
/// collapsed side there. The same for a full circle turned about the
/// tangent at its 90° joint. Each is a finished body (tiers 1–3 and the
/// at-rest census), and its volume holds the ruled solid's, read off the
/// sections alone.
#[test]
fn a_pinned_sub_arc_joint_builds_as_a_pinned_corner() {
    use core::f64::consts::PI;
    let circle = || {
        vec![<ProfileLoop<f64> as profile::RawLoop<f64>>::new([(
            Point2::new(1.0, 0.0),
            profile::Segment::Arc(geom_core::Arc2 {
                centre: Point2::new(0.0, 0.0),
                radius: 1.0,
                sweep: core::f64::consts::TAU,
            }),
        )])]
    };
    // The half disc: the chord on u ∈ [0, ½), the semicircle after.
    let dee_at = |u: f64| {
        if u < 0.5 {
            (-1.0 + 4.0 * u, 0.0)
        } else {
            let a = PI * (2.0 * u - 1.0);
            (a.cos(), a.sin())
        }
    };
    let dee_d = |u: f64| {
        if u < 0.5 {
            (4.0, 0.0)
        } else {
            let a = PI * (2.0 * u - 1.0);
            (-2.0 * PI * a.sin(), 2.0 * PI * a.cos())
        }
    };
    let circle_at = |u: f64| ((2.0 * PI * u).cos(), (2.0 * PI * u).sin());
    let circle_d = |u: f64| {
        let a = 2.0 * PI * u;
        (-2.0 * PI * a.sin(), 2.0 * PI * a.cos())
    };
    let apex = |angle: f64| turn([0.0, 1.0, 0.0], [1.0, 0.0, 0.0], angle);
    let (m0, dee_m1, circle_m1) = (Affine3::identity(), apex(-0.4), apex(-0.6));
    let cases = [
        (
            "half disc",
            vec![dee(); 2],
            dee_m1,
            ruled_curve_volume(dee_at, dee_d, PI / 2.0, (&m0, &dee_m1), 256),
            (5, 8, 5),
        ),
        (
            "circle",
            vec![circle(); 2],
            circle_m1,
            ruled_curve_volume(circle_at, circle_d, PI, (&m0, &circle_m1), 256),
            (7, 11, 6),
        ),
    ];
    for (what, sections, m1, want, census_want) in cases {
        let l = build(what, &sections, &[m0, m1], 1);
        assert_eq!(census(&l), census_want, "{what}: census");
        assert_eq!(
            collapsed_sides(&l),
            2,
            "{what}: the two quarter walls' collapsed sides at the joint"
        );
        let joints: Vec<Option<topo::EdgeKey>> = l.joint_edges[0]
            .iter()
            .flatten()
            .flatten()
            .copied()
            .collect();
        assert_eq!(
            joints.iter().filter(|e| e.is_none()).count(),
            1,
            "{what}: no strut at the pinned joint, one at every other: {joints:?}"
        );
        holds(what, measured(&l), want, 1e-9);
    }
}

/// **A foreign sub-arc joint on a hinge builds.** The square turned about
/// its edge y = +1 holds that segment flat in the first slab, and a third
/// section bulges the same segment into a 106° arc, two sub-arcs. The
/// skin carries that arc's joint into every section's row, so the hinge
/// line is cut there too: in the first slab the joint is one world point
/// in both sections, pinned with the hinge (no strut, no wall on either
/// piece), and in the second it moves (a strut between two walls). A
/// finished body (tiers 1–3 and the at-rest census).
#[test]
fn a_foreign_joint_on_a_hinge_builds_pinned_with_it() {
    let bulged = vec![bulge_loop(vec![
        (Point2::new(-1.0, -1.0), 0.0),
        (Point2::new(1.0, -1.0), 0.0),
        (Point2::new(1.0, 1.0), 0.5),
        (Point2::new(-1.0, 1.0), 0.0),
    ])];
    let l = build(
        "a hinge under a bulged segment",
        &[square(), square(), bulged],
        &[Affine3::identity(), about_its_far_edge(), lift(2.0)],
        1,
    );
    let pieces = &l.side_faces[0][2];
    assert_eq!(pieces.len(), 2, "the hinge segment is cut at the joint");
    for walls in pieces {
        assert_eq!(
            (walls[0], walls[1].is_some()),
            (None, true),
            "no wall in the hinge's slab, a wall above: {pieces:?}"
        );
    }
    assert_eq!(
        l.joint_edges[0][2],
        [vec![None, l.joint_edges[0][2][0][1]]],
        "the joint has no strut in the hinge's slab"
    );
    assert!(l.joint_edges[0][2][0][1].is_some(), "and a strut above it");
    // The hinge stack's (10, 17, 9), and the joint: its vertex in the
    // hinge's slab and in the bulged section, its strut above, the hinge
    // and the top rim each cut in two, and the wall above in two.
    assert_eq!(census(&l), (12, 20, 10), "census");
}
