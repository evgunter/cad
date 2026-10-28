//! **A degree-1 loft is one wall per segment per slab**
//! (`work/tess/loft-of-three-or-more-sections-at-degree-one-is-unmeshable.md`):
//! through `k ≥ 3` sections at v-degree 1, `sweep::loft_body` builds
//! `k − 1` two-section lofts glued at the shared sections, so every wall
//! is a single-span ruled patch, each interior section is a ring of
//! edges on the placed sketch segments and each strut a straight edge
//! per slab. Red on `main` before the split: the creased wall refused at
//! the C¹ gate (`UnsupportedNurbsFace`, "degree 1 with a sharp crease"),
//! and behind it the kinked strut at the chord pass ("a C⁰ polyline").

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::{PI, TAU};

use geom::{Curve3, Surface};
use geom_core::{Arc2, Point2, Tol};
use profile::test_support::bulge_loop;
use profile::{ProfileLoop, RawLoop, Segment};
use sweep::test_support::{brick, finished, stacked_at};
use sweep::{Lofted, Section, loft_body};
use topo::{Body, ContactRecords};

fn tol() -> Tol {
    Tol::witness()
}

/// The profiles the row measured, at scale `r`.
#[derive(Clone, Copy, Debug)]
enum Shape {
    /// A square of half-side `r`.
    Square,
    /// A circle of radius `r` as two half-turn arcs, each two
    /// rational-quadratic sub-arcs in the skin.
    Circle,
    /// A circle of radius `r` as one full-turn segment (D1).
    FullTurn,
}

impl Shape {
    fn section(self, r: f64) -> Section {
        let p = Point2::new;
        let lp: ProfileLoop<f64> = match self {
            Self::Square => RawLoop::polygon([p(-r, -r), p(r, -r), p(r, r), p(-r, r)]),
            Self::Circle => bulge_loop(vec![(p(r, 0.0), 1.0), (p(-r, 0.0), 1.0)]),
            Self::FullTurn => RawLoop::new([(
                p(r, 0.0),
                Segment::Arc(Arc2 {
                    centre: p(0.0, 0.0),
                    radius: r,
                    sweep: TAU,
                }),
            )]),
        };
        vec![lp]
    }

    /// The section's area at scale `r`.
    fn area(self, r: f64) -> f64 {
        match self {
            Self::Square => 4.0 * r * r,
            Self::Circle | Self::FullTurn => PI * r * r,
        }
    }

    /// Segments per section.
    fn segments(self) -> usize {
        match self {
            Self::Square => 4,
            Self::Circle => 2,
            Self::FullTurn => 1,
        }
    }
}

/// The section heights of `n` sections over `[0, 2]`, by spacing.
fn heights(n: usize, spacing: &str) -> Vec<f64> {
    #[allow(clippy::cast_precision_loss)]
    let t = |i: usize| i as f64 / (n - 1) as f64;
    (0..n)
        .map(|i| {
            let t = t(i);
            match spacing {
                "uniform" => 2.0 * t,
                "0.6t+0.4t²" => 2.0 * (0.6 * t + 0.4 * t * t),
                // A fixed jitter of the interior sections, inside a
                // third of a step so the order holds.
                #[allow(clippy::cast_precision_loss)]
                "jittered" if i > 0 && i + 1 < n => {
                    2.0 * (t + (i as f64 * 2.399).sin() / (3.0 * (n - 1) as f64))
                }
                _ => 2.0 * t,
            }
        })
        .collect()
}

/// A degree-1 loft of `shape` through sections at `zs`, section `i` at
/// scale `scale(i)`.
fn loft(shape: Shape, zs: &[f64], scale: impl Fn(usize) -> f64) -> Lofted<f64> {
    let sections: Vec<Section> = (0..zs.len()).map(|i| shape.section(scale(i))).collect();
    loft_body::<f64>(&sections, &stacked_at(zs), 1, tol())
        .unwrap_or_else(|e| panic!("{shape:?} at {zs:?}: the degree-1 loft builds: {e}"))
}

/// The closed form: each slab of a degree-1 loft between homothetic
/// sections about one axis is a frustum, `h (A₀ + √(A₀A₁) + A₁) / 3`.
fn frusta(shape: Shape, zs: &[f64], scale: impl Fn(usize) -> f64) -> f64 {
    zs.windows(2)
        .enumerate()
        .map(|(s, w)| {
            let (a0, a1) = (shape.area(scale(s)), shape.area(scale(s + 1)));
            (w[1] - w[0]) * (a0 + (a0 * a1).sqrt() + a1) / 3.0
        })
        .sum()
}

/// The edges `a` and `b` share.
fn shared_edges(body: &Body<f64>, a: topo::FaceKey, b: topo::FaceKey) -> Vec<topo::EdgeKey> {
    let face_of = |he| {
        body.get_loop(body.get_half_edge(he).unwrap().parent_loop)
            .unwrap()
            .face
    };
    let mut out: Vec<topo::EdgeKey> = body
        .half_edges()
        .filter(|&(he, _)| face_of(he) == a && face_of(body.mate(he).unwrap()) == b)
        .map(|(_, h)| h.edge)
        .collect();
    out.sort_unstable();
    out.dedup();
    out
}

/// Tier 3 and tier 3′, by name.
fn at_rest(body: &Body<f64>, what: &str) {
    assert_eq!(topo::validate_closed(body), Ok(()), "{what}: tier 2");
    assert_eq!(
        topo::validate_geometric(body, tol()),
        Ok(()),
        "{what}: tier 3"
    );
    assert_eq!(
        topo::validate_pseudomanifold(body, &ContactRecords::default(), tol()),
        Ok(()),
        "{what}: tier 3′"
    );
}

/// Meshes `body` at `delta`, checks the mesh and returns its volume.
fn meshed_volume(body: &Body<f64>, delta: f64, what: &str) -> f64 {
    let mesh = mesh::tessellate(body, delta, tol())
        .unwrap_or_else(|e| panic!("{what}, δ = {delta:e}: the loft tessellates: {e:?}"));
    mesh::validate::check_mesh(&mesh)
        .unwrap_or_else(|e| panic!("{what}, δ = {delta:e}: the mesh is closed: {e:?}"));
    mesh::validate::signed_volume(&mesh)
}

/// The integrated volume of `body` and its certified half-width.
fn volume(body: &Body<f64>) -> (f64, f64) {
    let m = topo::mass_properties(
        &finished("the measured loft", Body::clone(body), tol()),
        tol(),
        topo::test_support::pinned_resolution(tol()),
    )
    .unwrap();
    (m.enclosure.volume, m.enclosure.volume_pad)
}

/// The row's grid for one profile: `n = 3..9` sections, three spacings,
/// pure translation, each body meshed at both δ, every mesh closed and
/// its volume the closed form's. The tiers, which cost seconds a body
/// on the circle's rational walls, run where `at_rest(n)` asks.
fn the_row_grid(shape: Shape, at_rest_at: impl Fn(usize) -> bool) {
    for spacing in ["uniform", "0.6t+0.4t²", "jittered"] {
        for n in 3..=9 {
            let zs = heights(n, spacing);
            let what = format!("{shape:?}, {n} sections, {spacing}");
            let lofted = loft(shape, &zs, |_| 1.0);
            if at_rest_at(n) {
                at_rest(&lofted.body, &what);
            }
            let want = frusta(shape, &zs, |_| 1.0);
            for delta in [1e-2, 2e-3] {
                let v = meshed_volume(&lofted.body, delta, &what);
                // A square's mesh is exact; a circle's chords cut inside
                // it by at most δ.
                let slack = match shape {
                    Shape::Square => 1e-9,
                    Shape::Circle | Shape::FullTurn => 3.0 * delta,
                };
                assert!(
                    ((v - want) / want).abs() < slack,
                    "{what}, δ = {delta:e}: mesh volume {v} vs closed form {want}"
                );
            }
        }
    }
}

/// The square's grid, at rest at the fewest and the most sections.
#[test]
fn every_square_loft_the_row_measured_meshes() {
    the_row_grid(Shape::Square, |n| n == 3 || n == 9);
}

/// The two-arc circle's grid (each wall two rational quarter arcs in
/// `u`), at rest at three sections.
#[test]
fn every_circle_loft_the_row_measured_meshes() {
    the_row_grid(Shape::Circle, |n| n == 3);
}

/// Slabs that change scale: each is a frustum, so a wall on the wrong
/// slab's rows, or a ring at the wrong section, moves the volume. The
/// one-segment circle rides here too, its turn swept top slab first
/// through every slab below, and so does a square with a one-segment
/// hole and a square hole, swept slab by slab inside the outer loop.
#[test]
fn a_degree_one_loft_that_changes_scale_is_its_frusta() {
    let scale = |i: usize| [1.0, 0.6, 1.3, 0.8][i];
    let zs = heights(4, "0.6t+0.4t²");
    for shape in [Shape::Square, Shape::FullTurn] {
        let what = format!("{shape:?}, scaled");
        let lofted = loft(shape, &zs, scale);
        at_rest(&lofted.body, &what);
        let want = frusta(shape, &zs, scale);
        let (got, pad) = volume(&lofted.body);
        assert!(
            (got - want).abs() <= pad + 1e-9 * want,
            "{what}: volume {got} ± {pad}, closed form {want}"
        );
        let v = meshed_volume(&lofted.body, 2e-3, &what);
        assert!(
            ((v - want) / want).abs() < 6e-3,
            "{what}: mesh volume {v} vs closed form {want}"
        );
    }
    // Three sections of a 4 × 4 square holed by a circle of radius 0.5
    // about (−1, 0) and a 0.8 square about (1, 0), each scaled about its
    // own centre, so every slab of every loop is a frustum.
    let p = Point2::new;
    let holed = |s: f64| -> Section {
        let square = |cx: f64, h: f64| -> ProfileLoop<f64> {
            RawLoop::polygon([p(cx - h, -h), p(cx - h, h), p(cx + h, h), p(cx + h, -h)])
        };
        let outer: ProfileLoop<f64> =
            RawLoop::polygon([p(-2.0, -2.0), p(2.0, -2.0), p(2.0, 2.0), p(-2.0, 2.0)]);
        let round: ProfileLoop<f64> = RawLoop::new([(
            p(-1.0 + 0.5 * s, 0.0),
            Segment::Arc(Arc2 {
                centre: p(-1.0, 0.0),
                radius: 0.5 * s,
                sweep: -TAU,
            }),
        )]);
        vec![outer, round, square(1.0, 0.4 * s)]
    };
    let hs = [1.0, 0.7, 1.2];
    let zs = [0.0, 0.8, 2.0];
    let sections: Vec<Section> = hs.iter().map(|&s| holed(s)).collect();
    let lofted = loft_body::<f64>(&sections, &stacked_at(&zs), 1, tol())
        .unwrap_or_else(|e| panic!("the holed square lofts: {e}"));
    assert!(
        lofted.side_faces.iter().flatten().all(|w| w.len() == 2),
        "every loop's walls are two slabs"
    );
    at_rest(&lofted.body, "holed, scaled");
    let area = |s: f64| (16.0, PI * 0.25 * s * s, 0.64 * s * s);
    let want: f64 = zs
        .windows(2)
        .zip(hs.windows(2))
        .map(|(z, s)| {
            let ((o0, r0, q0), (o1, r1, q1)) = (area(s[0]), area(s[1]));
            let frustum = |a0: f64, a1: f64| (z[1] - z[0]) * (a0 + (a0 * a1).sqrt() + a1) / 3.0;
            frustum(o0, o1) - frustum(r0, r1) - frustum(q0, q1)
        })
        .sum();
    let (got, pad) = volume(&lofted.body);
    assert!(
        (got - want).abs() <= pad + 1e-9 * want,
        "holed: volume {got} ± {pad}, closed form {want}"
    );
    let v = meshed_volume(&lofted.body, 2e-3, "holed");
    assert!(
        ((v - want) / want).abs() < 1e-3,
        "holed: mesh volume {v} vs closed form {want}"
    );
}

/// The structure: `[loop][segment][slab]` walls, each a single span in
/// `v` on its own surface; one straight strut per slab; each interior
/// section a ring on the placed sketch segments, between the walls of
/// the slabs it divides; `V − E + F = 2`.
#[test]
fn the_walls_struts_and_rings_are_one_per_slab() {
    let zs = heights(4, "jittered");
    for shape in [Shape::Square, Shape::Circle, Shape::FullTurn] {
        let what = format!("{shape:?}");
        let lofted = loft(shape, &zs, |_| 1.0);
        let body = &lofted.body;
        let (segs, slabs) = (shape.segments(), zs.len() - 1);
        assert_eq!(lofted.side_faces.len(), 1, "{what}: one loop");
        assert_eq!(lofted.side_faces[0].len(), segs, "{what}: segments");
        let mut keys = Vec::new();
        for (j, walls) in lofted.side_faces[0].iter().enumerate() {
            assert_eq!(walls.len(), slabs, "{what}: segment {j}'s slabs");
            for &w in walls {
                let face = body.get_face(w).unwrap();
                let Some(Surface::Nurbs(net)) = body.get_surface(face.surface) else {
                    panic!("{what}: wall {w:?} is a spline")
                };
                assert_eq!(
                    (net.knot_values_v().degree(), net.control_counts().1),
                    (1, 2),
                    "{what}: wall {w:?} is one ruled span in v"
                );
                keys.push(face.surface);
            }
        }
        keys.sort_unstable();
        keys.dedup();
        assert_eq!(keys.len(), segs * slabs, "{what}: one surface per wall");
        for (j, struts) in lofted.seam_edges[0].iter().enumerate() {
            assert_eq!(struts.len(), slabs, "{what}: vertex {j}'s struts");
            for &e in struts {
                let c = body.stored_curve(e).unwrap();
                let Curve3::Nurbs(net) = c.carrier() else {
                    panic!("{what}: strut {e:?} is a wall's boundary column")
                };
                assert_eq!(
                    (net.degree(), net.control().len()),
                    (1, 2),
                    "{what}: strut {e:?} is one straight span"
                );
            }
        }
        // The ring at each interior section: the edges between slab
        // `s − 1`'s and slab `s`'s wall of each segment.
        for s in 1..slabs {
            for j in 0..segs {
                let (below, above) = (lofted.side_faces[0][j][s - 1], lofted.side_faces[0][j][s]);
                let shared = shared_edges(body, below, above);
                let [rim] = shared[..] else {
                    panic!("{what}: section {s}, segment {j}: walls share {shared:?}")
                };
                let c = body.stored_curve(rim).unwrap();
                assert!(
                    matches!(c.carrier(), Curve3::Line { .. } | Curve3::Circle { .. }),
                    "{what}: section {s}'s rim {rim:?} is its placed sketch segment"
                );
                // Described in the row `v = 1` of the wall BELOW it. The
                // slab above holds the rim too, on its row `v = 0`, so a
                // description there would also certify; this pins the
                // choice (one-segment loops included).
                let chart = c.description().chart().unwrap_or_else(|| {
                    panic!("{what}: section {s}'s rim {rim:?} is a chart image")
                });
                assert_eq!(
                    chart.surface,
                    body.get_face(below).unwrap().surface,
                    "{what}: section {s}'s rim {rim:?} is described in the wall below it"
                );
                let (t0, t1) = c.params();
                assert_eq!(
                    chart.pcurve.eval(0.5 * (t0 + t1)).y,
                    1.0,
                    "{what}: section {s}'s rim {rim:?} lies on the row v = 1"
                );
            }
        }
        let (v, e, f) = (
            body.vertices().count(),
            body.edges().count(),
            body.faces().count(),
        );
        #[allow(clippy::cast_possible_wrap)]
        let chi = v as i64 - e as i64 + f as i64;
        assert_eq!(chi, 2, "{what}: V − E + F, ({v}, {e}, {f})");
        assert_eq!(f, 2 + segs * slabs, "{what}: two caps and the walls");
    }
}

/// v-degree ≥ 2 keeps one wall per segment across every section.
#[test]
fn a_higher_degree_loft_keeps_one_wall_per_segment() {
    let zs = heights(4, "uniform");
    let sections: Vec<Section> = (0..4).map(|_| Shape::Square.section(1.0)).collect();
    for degree in [2, 3] {
        let lofted = loft_body::<f64>(&sections, &stacked_at(&zs), degree, tol()).unwrap();
        for walls in &lofted.side_faces[0] {
            assert_eq!(walls.len(), 1, "degree {degree}: one wall per segment");
        }
        assert_eq!(lofted.body.faces().count(), 6, "degree {degree}");
    }
}

/// **A ladder gap, pinned so it goes red when the gap closes.** The
/// construction cuts a wall at each section, and the ordinary
/// maximal-faces join should then merge the slab walls whose carriers
/// coincide (Ev on PR 5913). Here they do coincide: a translated
/// square's slab walls of one segment lie on one plane. Each is its own
/// spline surface, though, and the carrier ladder cannot read a spline
/// carrier, so the merge the boolean's output stage runs
/// (`Body::merge_coplanar_faces`) glues none of them
/// (`work/coincide/the-carrier-ladder-cannot-read-a-spline-carrier-so-coincident-spline-faces-never-merge.md`).
/// That is a missing reading, not a property of the loft. When the
/// ladder reads them, these assertions fail and are rewritten to expect
/// the merge. The boolean's maximal-faces gate reads an operand the
/// same way, so it passes too; what refuses the union is the boolean's
/// spline-edge frontier, as on a two-section loft
/// (`band_subdivided_side_walls`).
#[test]
fn the_output_stage_merge_keeps_every_slab_wall() {
    let zs = heights(4, "uniform");
    let lofted = loft(Shape::Square, &zs, |_| 1.0);
    let mut merged = lofted.body.clone();
    let outcome = merged.merge_coplanar_faces(tol()).unwrap();
    assert!(
        outcome.groups.is_empty(),
        "coplanar slab walls merged: the carrier ladder now reads spline carriers \
         (work/coincide/the-carrier-ladder-cannot-read-a-spline-carrier-so-coincident-\
         spline-faces-never-merge.md); rewrite this row to expect the merge: {:?}",
        outcome.groups
    );
    assert_eq!(merged.faces().count(), lofted.body.faces().count());
    let loft = finished("the loft", lofted.body, tol());
    let cube = finished(
        "a cube",
        brick::<f64>((0.5, 1.5), (-0.5, 0.5), (0.3, 1.7), tol()),
        tol(),
    );
    let err = topo::union(&loft, &cube, tol()).map(|_| ()).unwrap_err();
    assert!(
        !matches!(err, topo::BooleanError::NonMaximalFaces { .. }),
        "the union refuses past the maximal-faces gate: {err:?}"
    );
    println!("degree-1 loft ∪ cube across two sections: {err:?}");
}

/// A path sweep shares the assembly: at v-degree 1 through five
/// stations along a quarter arc it is one wall per segment per slab,
/// at rest, and it meshes to the volume it integrates to.
#[test]
fn a_degree_one_path_sweep_is_one_wall_per_slab() {
    let path = geom::NurbsCurve3::new(
        geom_core::spline::KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap(),
        vec![
            geom_core::Point3::new(0.0, 0.0, 0.0),
            geom_core::Point3::new(0.0, 0.0, 5.0),
            geom_core::Point3::new(5.0, 0.0, 5.0),
        ],
        vec![1.0, core::f64::consts::FRAC_1_SQRT_2, 1.0],
    )
    .unwrap();
    let section = Shape::Square.section(0.4);
    let swept = sweep::sweep_body::<f64>(&section, profile::SketchPlane::xy(), &path, 5, 1, tol())
        .unwrap_or_else(|e| panic!("the degree-1 sweep builds: {e}"));
    assert!(
        swept.side_faces[0].iter().all(|w| w.len() == 4),
        "four slabs a segment"
    );
    at_rest(&swept.body, "path sweep");
    let (got, pad) = volume(&swept.body);
    let v = meshed_volume(&swept.body, 2e-3, "path sweep");
    assert!(
        (v - got).abs() <= pad + 1e-9 * got,
        "path sweep: mesh volume {v} vs integrated {got} ± {pad}"
    );
}

/// **An arc whose sub-arc count differs across the sections refuses
/// typed at degree 1, and lofts at degree 2.** The arc wall carries every
/// section's sub-arc structure, so the narrow interior arc reaches its
/// row refined at the wide arcs' joints, which the arc-rim class does
/// not describe
/// (`work/isotail/a-loft-whose-arc-bulge-varies-per-section-refuses-its-arc-rim.md`).
/// The refusal names the section and carries no escalation; its
/// recourse, degree 2, builds the same sections.
#[test]
fn an_interior_arc_refined_by_its_neighbours_refuses_typed() {
    let corners = [(-1.0, -1.0), (1.0, -1.0), (1.0, 1.0), (-1.0, 1.0)];
    // A square whose +x side is an arc of bulge `b` (a line at 0).
    let section = |b: f64| -> Section {
        let chain = (0..4)
            .map(|i| {
                let (x, y) = corners[i];
                (Point2::new(x, y), if i == 1 { b } else { 0.0 })
            })
            .collect();
        vec![bulge_loop(chain)]
    };
    let cases: [(&[f64], usize); 3] = [
        (&[1.0, 0.2, 1.0], 1),
        (&[1.0, 0.2, 0.0, 0.7, 1.0], 1),
        (&[0.0, 1.0, 0.3, 0.0], 2),
    ];
    for (bulges, section_at) in cases {
        let sections: Vec<Section> = bulges.iter().map(|&b| section(b)).collect();
        #[allow(clippy::cast_precision_loss)]
        let zs: Vec<f64> = (0..bulges.len()).map(|i| i as f64).collect();
        let places = stacked_at(&zs);
        match loft_body::<f64>(&sections, &places, 1, tol()) {
            Err(sweep::LoftError::SectionRim {
                section,
                source: None,
            }) => assert_eq!(section, section_at, "{bulges:?}: the refused section"),
            other => panic!(
                "{bulges:?}: degree 1 refuses at the refined arc rim, got {:?}",
                other.map(|_| ())
            ),
        }
        loft_body::<f64>(&sections, &places, 2, tol())
            .unwrap_or_else(|e| panic!("{bulges:?}: the recourse, degree 2, builds: {e}"));
    }
}
