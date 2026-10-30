//! **A wrap edge's tip is admitted at tier 2, and holds at tier 3 only on
//! its face's chart singularity** (DESIGN `[topology.tiers]`, and the
//! seam clause of `[topology]`: where no edge the author wrote reaches a
//! pole or an apex, one wrap edge does, walked out and back).
//!
//! No door mints a tip yet, so each body is a revolved solid cut by
//! Euler operators: the two meridians a full revolve leaves at a pole or
//! an apex become one by `kef`, restated as the face's wrap edge.
//!
//! - a hemisphere's pole and a cone's apex, each reached by one wrap edge,
//!   pass tiers 2 and 3 and measure their closed forms through the chart
//!   image;
//! - a wrap edge that stops short of the pole passes tier 2 and is
//!   refused at tier 3, alone where the pole is a vertex of another edge,
//!   and beside the chart-singularity arm's refusal where it is not (the
//!   shape of `step-import`'s `rim-only-cap/spur.step`);
//! - a valence-1 vertex on an edge that is not a wrap edge, and both ends
//!   of a lone wrap strut, are refused at tier 2 as they were;
//! - at a tip near the pole, the mint's joint and tier 3's two readings of
//!   the pole answer by the one incidence reading: on, all three admit;
//!   in band, all three stop short of a decision, on one escalation.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::f64::consts::{FRAC_PI_2, PI, SQRT_2};
use geom_brep::{EdgeCurveSpec, EdgeDescriptionSpec};
use geom_core::{Band, Point2, Point3, Tol, Vec3};
use sweep::Revolution;
use sweep::test_support::revolved_about_y;
use topo::construction::{Construct, MevSite};
use topo::{Body, EdgeKey, FaceKey, HalfEdgeKey, ValidationError, VertexKey};

fn tol() -> Tol {
    Tol::witness()
}

const NORTH: Point3<f64> = Point3::new(0.0, 1.0, 0.0);

/// The unit hemisphere `y ≥ 0` on its base disk: two quarter-sphere lunes
/// meeting along two meridians at the pole.
fn hemisphere() -> Body<f64> {
    revolved_about_y(
        vec![
            (Point2::new(0.0, 0.0), 0.0),
            (Point2::new(1.0, 0.0), (PI / 8.0).tan()),
            (Point2::new(0.0, 1.0), 0.0),
        ],
        Revolution::Full,
        tol(),
    )
}

/// The cone of radius and height 1 on its base disk, apex at `NORTH`.
fn cone() -> Body<f64> {
    revolved_about_y(
        vec![
            (Point2::new(0.0, 0.0), 0.0),
            (Point2::new(1.0, 0.0), 0.0),
            (Point2::new(0.0, 1.0), 0.0),
        ],
        Revolution::Full,
        tol(),
    )
}

fn point(body: &Body<f64>, v: VertexKey) -> Point3<f64> {
    *body.get_point(body.get_vertex(v).unwrap().point).unwrap()
}

/// The vertex at `at`.
fn vertex_at(body: &Body<f64>, at: Point3<f64>) -> VertexKey {
    body.vertices()
        .find(|&(v, _)| (point(body, v) - at).norm() < 1e-12)
        .map(|(v, _)| v)
        .expect("a vertex there")
}

/// The half-edges leaving `v`, in half-edge-slot order.
fn leaving(body: &Body<f64>, v: VertexKey) -> Vec<HalfEdgeKey> {
    body.half_edges()
        .filter(|(_, h)| h.start == v)
        .map(|(he, _)| he)
        .collect()
}

/// Whether the joint the mint stored at `he` (into `he` from its loop's
/// predecessor) is a reset.
fn resets(body: &Body<f64>, he: HalfEdgeKey) -> bool {
    body.joint(he).is_some_and(topo::JointElement::is_reset)
}

/// **The two meridians at the pole made one wrap edge**: the second is
/// killed, merging the two lunes into the face it leaves, and the first is
/// restated as that face's wrap edge. Where `join`, the join then takes
/// the rim's two arcs back into one closed edge at the vertex the kill
/// left with nothing else at it; where not, that vertex is left for a
/// spur to stand on. Returns the face, the wrap edge and its tip.
fn one_wrap_edge(mut body: Body<f64>, join: bool) -> (Body<f64>, FaceKey, EdgeKey, VertexKey) {
    let tip = vertex_at(&body, NORTH);
    let out = leaving(&body, tip);
    assert_eq!(out.len(), 2, "two meridians leave the pole");
    let wraps = body.get_half_edge(out[0]).unwrap().edge;
    let face = body.face_of_half_edge(out[0]).unwrap();
    let surface = body.get_face(face).unwrap().surface;
    body.kef_describing(
        out[1],
        &[(wraps, EdgeDescriptionSpec::wrap(surface))],
        tol(),
    )
    .expect("the survivor restates as the face's wrap edge");
    if join {
        body.join_edges(Band::linear(tol()).unwrap(), tol())
            .expect("the rim joins");
    }
    let face = body.face_of_half_edge(out[0]).unwrap();
    (body, face, wraps, tip)
}

/// **A meridian spur of the unit sphere** from the equator vertex at
/// `(x, 0, 0)` (`x = ±1`) up to the latitude `top`, spliced into `face`'s
/// loop there and described by `description`. Returns its edge and tip.
fn spur(
    body: &mut Body<f64>,
    face: FaceKey,
    x: f64,
    top: f64,
    description: EdgeDescriptionSpec<f64>,
) -> (EdgeKey, VertexKey) {
    let from = vertex_at(body, Point3::new(x, 0.0, 0.0));
    let he = leaving(body, from)
        .into_iter()
        .find(|&he| body.face_of_half_edge(he) == Some(face))
        .expect("the face's loop leaves that vertex");
    let carrier = geom::Curve3::Circle {
        center: Point3::origin(),
        axis: Vec3::new(0.0, 0.0, x),
        radius: 1.0,
        u_ref: Vec3::new(x, 0.0, 0.0),
    };
    let tip = carrier.eval(top);
    let mut spec = EdgeCurveSpec::arc_of_circle(carrier, 0.0, top).unwrap();
    spec.description = description;
    let made = body
        .mev(MevSite::Fan { he1: he, he2: he }, tip, spec, tol())
        .expect("the spur splices into the face's loop");
    (made.edge, made.vertex)
}

/// The hemisphere with its one wrap edge killed along with the pole: its
/// curved face is bounded by the equator alone, and holds the pole.
fn equator_only() -> (Body<f64>, FaceKey) {
    let (mut body, face, wraps, tip) = one_wrap_edge(hemisphere(), true);
    let e = body.get_edge(wraps).unwrap();
    let toward = if body.get_half_edge(e.he_plus).unwrap().start == tip {
        e.he_minus
    } else {
        e.he_plus
    };
    body.kev_describing(toward, &[], tol())
        .expect("the wrap edge and its tip die");
    (body, face)
}

/// Every face's reading summed: a third of the flux is the volume.
fn measured(body: &Body<f64>, what: &str) -> (f64, f64) {
    body.faces()
        .map(|(face, _)| {
            let r = topo::test_support::face_reading(body, face, tol(), 1e-6)
                .unwrap_or_else(|e| panic!("{what}: face {face:?} reads: {e}"));
            (r.flux, r.area)
        })
        .fold((0.0, 0.0), |(f, a), (g, b)| (f + g, a + b))
}

fn close(got: f64, want: f64, what: &str) {
    assert!(
        (got - want).abs() <= 1e-9 * want.abs().max(1.0),
        "{what}: {got}, closed form {want}"
    );
}

/// **A pole or an apex reached by one wrap edge** passes tiers 2 and 3,
/// its joint at the tip a reset, and the body measures its closed form.
#[test]
fn a_tip_on_the_pole_or_apex_passes_and_measures() {
    let rows = [
        ("the hemisphere", hemisphere(), 2.0 * PI / 3.0, 3.0 * PI),
        ("the cone", cone(), PI / 3.0, PI * SQRT_2 + PI),
    ];
    for (what, body, volume, area) in rows {
        let (body, face, wraps, tip) = one_wrap_edge(body, true);
        let back = leaving(&body, tip);
        assert_eq!(back.len(), 1, "{what}: the tip has valence 1");
        assert_eq!(
            body.get_half_edge(back[0]).unwrap().edge,
            wraps,
            "{what}: on the wrap edge"
        );
        assert_eq!(
            body.face_of_half_edge(back[0]),
            Some(face),
            "{what}: walked by the merged face"
        );
        assert!(
            resets(&body, back[0]),
            "{what}: the joint at the tip resets: {:?}",
            body.joint(back[0])
        );
        assert_eq!(topo::validate_closed(&body), Ok(()), "{what}: tier 2");
        assert_eq!(
            topo::validate_geometric(&body, tol()),
            Ok(()),
            "{what}: tier 3"
        );
        let (flux, got) = measured(&body, what);
        close(flux / 3.0, volume, &format!("{what}: volume"));
        close(got, area, &format!("{what}: area"));
    }
}

/// **A wrap edge whose tip stops short of the pole** passes tier 2 (its
/// tip is a wrap edge's) and is refused at tier 3. Where the pole is the
/// tip of another wrap edge, the short tip is the one finding; where the
/// face's only boundary is the equator, the pole lies inside the face too,
/// and the chart-singularity arm refuses that beside it: `spur.step`'s
/// shape, a rim wearing a meridian spur.
#[test]
fn a_tip_short_of_the_pole_refuses_at_tier_3() {
    let short = FRAC_PI_2 - 0.5;
    let (mut body, face, _, _) = one_wrap_edge(hemisphere(), false);
    let surface = body.get_face(face).unwrap().surface;
    let (edge, tip) = spur(
        &mut body,
        face,
        -1.0,
        short,
        EdgeDescriptionSpec::wrap(surface),
    );
    assert_eq!(
        topo::validate_closed(&body),
        Ok(()),
        "tier 2 admits both tips"
    );
    assert_eq!(
        topo::validate_geometric(&body, tol()),
        Err(vec![ValidationError::WrapTipOffChartSingularity {
            face,
            edge,
            tip
        }]),
        "the pole a vertex: the short tip alone"
    );

    let (mut body, face) = equator_only();
    let surface = body.get_face(face).unwrap().surface;
    let (edge, tip) = spur(
        &mut body,
        face,
        1.0,
        short,
        EdgeDescriptionSpec::wrap(surface),
    );
    assert_eq!(topo::validate_closed(&body), Ok(()), "spur.step: tier 2");
    assert_eq!(
        topo::validate_geometric(&body, tol()),
        Err(vec![
            ValidationError::ChartSingularityNotAVertex {
                face,
                point: geom_brep::chart_singularity::Singularity::North,
                lies: topo::SingularityLies::Interior,
            },
            ValidationError::WrapTipOffChartSingularity { face, edge, tip },
        ]),
        "spur.step: the pole inside the face, and the tip short of it"
    );
}

/// **A valence-1 vertex not at a wrap edge's tip is still a strut**: a
/// spur not described as a wrap edge, and both ends of the one-face ball's
/// lone meridian, a wrap edge both of whose ends have valence 1.
#[test]
fn a_valence_one_vertex_off_a_wrap_tip_is_a_strut() {
    let (mut body, face, _, _) = one_wrap_edge(hemisphere(), false);
    let surface = body.get_face(face).unwrap().surface;
    let (_, tip) = spur(
        &mut body,
        face,
        -1.0,
        0.5,
        EdgeDescriptionSpec::chart(surface),
    );
    assert_eq!(
        topo::validate_closed(&body),
        Err(vec![ValidationError::ScaffoldingStrutVertex {
            vertex: tip
        }]),
        "a spur that is not a wrap edge"
    );

    let mut ball = revolved_about_y(
        vec![(Point2::new(0.0, -1.0), 1.0), (Point2::new(0.0, 1.0), 0.0)],
        Revolution::Full,
        tol(),
    );
    let north = vertex_at(&ball, NORTH);
    let south = vertex_at(&ball, Point3::new(0.0, -1.0, 0.0));
    let out = leaving(&ball, north);
    let wraps = ball.get_half_edge(out[0]).unwrap().edge;
    let face = ball.face_of_half_edge(out[0]).unwrap();
    let surface = ball.get_face(face).unwrap().surface;
    ball.kef_describing(
        out[1],
        &[(wraps, EdgeDescriptionSpec::wrap(surface))],
        tol(),
    )
    .expect("the one-face ball");
    let mut ends = [north, south];
    ends.sort();
    assert_eq!(
        topo::validate_closed(&ball),
        Err(ends
            .iter()
            .map(|&vertex| ValidationError::ScaffoldingStrutVertex { vertex })
            .collect()),
        "a lone wrap strut has no tip"
    );
}

/// **The mint and tier 3 read a tip's incidence on the pole once.** A
/// wrap spur from the equator stops `d` short of the pole, the face's
/// only edge near it. Decided on the pole (`d` inside the band's zero),
/// the mint's joint at the tip resets, the chart-singularity arm places
/// the pole at the tip, and the tip rule admits it. In band, the joint
/// does not reset, and both of tier 3's readings refuse as undecided on
/// the one escalation.
#[test]
fn the_mint_and_tier_3_read_one_incidence_at_the_tip() {
    let band = Band::linear(tol()).unwrap();
    let short_by = |d: f64| {
        let (mut body, face) = equator_only();
        let surface = body.get_face(face).unwrap().surface;
        let top = FRAC_PI_2 - 2.0 * (d / 2.0).asin();
        let (edge, tip) = spur(
            &mut body,
            face,
            1.0,
            top,
            EdgeDescriptionSpec::wrap(surface),
        );
        let back = leaving(&body, tip)[0];
        (body, face, edge, tip, back)
    };

    let (body, _, _, tip, back) = short_by(band.zero() / 4.0);
    assert!(
        (point(&body, tip) - NORTH).norm() <= band.zero(),
        "on: the tip within the band's zero"
    );
    assert!(
        resets(&body, back),
        "on: the mint resets at the tip: {:?}",
        body.joint(back)
    );
    assert_eq!(
        topo::validate_geometric(&body, tol()),
        Ok(()),
        "on: the pole is at the tip, and the tip on the pole"
    );

    let (body, face, edge, tip, back) = short_by((band.zero() * band.escalate()).sqrt());
    let d = (point(&body, tip) - NORTH).norm();
    assert!(
        band.zero() < d && d < band.escalate(),
        "in band: the tip's distance {d}"
    );
    assert!(
        !resets(&body, back),
        "in band: the mint does not reset at the tip: {:?}",
        body.joint(back)
    );
    let got = topo::validate_geometric(&body, tol()).expect_err("in band refuses");
    let [
        ValidationError::ChartSingularityUndecided {
            face: f6,
            reading: geom_brep::chart_singularity::Undecided::Boundary(at_vertex),
            ..
        },
        ValidationError::WrapTipUndecided {
            face: f8,
            edge: e8,
            tip: t8,
            cause,
        },
    ] = got.as_slice()
    else {
        panic!("in band: both readings undecided, nothing else: {got:?}")
    };
    assert_eq!(
        (*f6, *f8, *e8, *t8),
        (face, face, edge, tip),
        "in band: the subjects"
    );
    assert_eq!(at_vertex, cause, "in band: one escalation");
}
