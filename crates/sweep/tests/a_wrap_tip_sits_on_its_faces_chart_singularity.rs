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
//! - a valence-1 vertex on an edge that is not a wrap edge is refused at
//!   tier 2 as it was, while both ends of a lone wrap edge are tips: the
//!   one-face ball passes, and a lone strut short of both poles refuses
//!   at tier 3;
//! - a wrap tip on a cylinder, which has no singular point, refuses as
//!   such;
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

/// **A valence-1 vertex off a wrap edge is still a strut**: a spur not
/// described as a wrap edge.
#[test]
fn a_valence_one_vertex_off_a_wrap_edge_is_a_strut() {
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
}

/// The unit ball with one of its two meridians killed: one face whose
/// loop walks the other pole to pole and back, restated as its wrap edge.
fn one_face_ball() -> (Body<f64>, FaceKey, EdgeKey) {
    let mut ball = revolved_about_y(
        vec![(Point2::new(0.0, -1.0), 1.0), (Point2::new(0.0, 1.0), 0.0)],
        Revolution::Full,
        tol(),
    );
    let out = leaving(&ball, vertex_at(&ball, NORTH));
    let wraps = ball.get_half_edge(out[0]).unwrap().edge;
    let face = ball.face_of_half_edge(out[0]).unwrap();
    let surface = ball.get_face(face).unwrap().surface;
    ball.kef_describing(
        out[1],
        &[(wraps, EdgeDescriptionSpec::wrap(surface))],
        tol(),
    )
    .expect("the one-face ball");
    let face = ball.face_of_half_edge(out[0]).unwrap();
    (ball, face, wraps)
}

/// Kills the edge leaving `tip`, with `tip`.
fn kill_tip(body: &mut Body<f64>, tip: VertexKey) {
    let out = leaving(body, tip);
    let e = body
        .get_edge(body.get_half_edge(out[0]).unwrap().edge)
        .unwrap();
    let toward = if e.he_plus == out[0] {
        e.he_minus
    } else {
        e.he_plus
    };
    body.kev_describing(toward, &[], tol())
        .expect("the tip's edge dies with it");
}

/// **Both ends of a lone wrap edge are tips.** The one-face ball, whose
/// one meridian runs pole to pole, passes tiers 2 and 3 and measures the
/// ball. Cut 0.3 rad short of each pole, the lone strut still passes
/// tier 2, and tier 3 refuses each pole inside the face and each tip off
/// it.
#[test]
fn a_lone_wrap_edge_has_two_tips() {
    let (ball, _, _) = one_face_ball();
    assert_eq!(topo::validate_closed(&ball), Ok(()), "the ball: tier 2");
    assert_eq!(
        topo::validate_geometric(&ball, tol()),
        Ok(()),
        "the ball: tier 3"
    );
    let (flux, area) = measured(&ball, "the one-face ball");
    close(flux / 3.0, 4.0 * PI / 3.0, "the one-face ball: volume");
    close(area, 4.0 * PI, "the one-face ball: area");

    let (mut body, face, wraps) = one_face_ball();
    let (north, south) = (
        vertex_at(&body, NORTH),
        vertex_at(&body, Point3::new(0.0, -1.0, 0.0)),
    );
    body.split_edge(wraps, 0.3, tol())
        .expect("split 0.3 rad from the north pole");
    kill_tip(&mut body, north);
    let rest = body.get_half_edge(leaving(&body, south)[0]).unwrap().edge;
    body.split_edge(rest, PI - 0.3, tol())
        .expect("split 0.3 rad from the south pole");
    kill_tip(&mut body, south);
    assert_eq!(body.edges().count(), 1, "the strut is the body's one edge");
    let strut = body
        .edges()
        .map(|(e, _)| e)
        .find(|&e| body.both_sides_face(e) == Some(Some(face)))
        .expect("the strut lies in the sphere face");
    let ends = {
        let e = body.get_edge(strut).unwrap();
        let mut ends = [e.he_plus, e.he_minus].map(|h| body.get_half_edge(h).unwrap().start);
        ends.sort();
        ends
    };
    assert_eq!(topo::validate_closed(&body), Ok(()), "short strut: tier 2");
    let got = topo::validate_geometric(&body, tol()).expect_err("short strut refuses");
    let interior = |point| ValidationError::ChartSingularityNotAVertex {
        face,
        point,
        lies: topo::SingularityLies::Interior,
    };
    let off = |tip| ValidationError::WrapTipOffChartSingularity {
        face,
        edge: strut,
        tip,
    };
    use geom_brep::chart_singularity::Singularity::{North, South};
    assert_eq!(got.len(), 4, "short strut: {got:?}");
    for want in [interior(North), interior(South), off(ends[0]), off(ends[1])] {
        assert!(got.contains(&want), "short strut: {want:?} in {got:?}");
    }
}

/// **A wrap tip on a surface with no singular point** has nowhere to lie:
/// a wrap spur up a cylinder's wall refuses as such, naming no pole.
#[test]
fn a_wrap_tip_on_a_cylinder_has_no_singularity_to_lie_on() {
    let mut body = revolved_about_y(
        vec![
            (Point2::new(0.0, 0.0), 0.0),
            (Point2::new(1.0, 0.0), 0.0),
            (Point2::new(1.0, 1.0), 0.0),
            (Point2::new(0.0, 1.0), 0.0),
        ],
        Revolution::Full,
        tol(),
    );
    let (he, face) = body
        .half_edges()
        .find_map(|(he, h)| {
            let p = point(&body, h.start);
            let face = body.face_of_half_edge(he)?;
            let on_wall = matches!(
                body.get_surface(body.get_face(face)?.surface)?,
                geom::Surface::Cylinder { .. }
            );
            (on_wall && p.y.abs() < 1e-12).then_some((he, face))
        })
        .expect("the wall leaves a base vertex");
    let from = point(&body, body.get_half_edge(he).unwrap().start);
    let to = from + Vec3::new(0.0, 0.5, 0.0);
    let surface = body.get_face(face).unwrap().surface;
    let mut spec = EdgeCurveSpec::line_between(from, to);
    spec.description = EdgeDescriptionSpec::wrap(surface);
    let made = body
        .mev(MevSite::Fan { he1: he, he2: he }, to, spec, tol())
        .expect("the wrap spur splices into the wall");
    assert_eq!(
        topo::validate_closed(&body),
        Ok(()),
        "tier 2 admits the tip"
    );
    assert_eq!(
        topo::validate_geometric(&body, tol()),
        Err(vec![ValidationError::WrapTipWithoutChartSingularity {
            face,
            edge: made.edge,
            tip: made.vertex,
        }]),
        "a cylinder has no pole for the tip"
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
    let ending = got[1].to_string();
    assert!(
        !ending.contains("clear of it") && !ending.contains("tighten"),
        "in band: the tip's ending offers only the lever: {ending}"
    );
}

/// The unit sphere's cap above `y = 0.8` on its disk, its pole reached by
/// one wrap edge.
fn small_tip_cap() -> (Body<f64>, FaceKey, VertexKey) {
    let th: f64 = 0.8f64.asin();
    let cap = revolved_about_y(
        vec![
            (Point2::new(0.0, 0.8), 0.0),
            (Point2::new(0.6, 0.8), ((FRAC_PI_2 - th) / 4.0).tan()),
            (Point2::new(0.0, 1.0), 0.0),
        ],
        Revolution::Full,
        tol(),
    );
    let (body, face, _, tip) = one_wrap_edge(cap, true);
    (body, face, tip)
}

/// The cone of radius and height 0.2 on its disk, apex at `NORTH` reached
/// by one wrap edge.
fn small_tip_cone() -> (Body<f64>, FaceKey, VertexKey) {
    let cone = revolved_about_y(
        vec![
            (Point2::new(0.0, 0.8), 0.0),
            (Point2::new(0.2, 0.8), 0.0),
            (Point2::new(0.0, 1.0), 0.0),
        ],
        Revolution::Full,
        tol(),
    );
    let (body, face, _, tip) = one_wrap_edge(cone, true);
    (body, face, tip)
}

/// The side `face`'s loops encode, read by tier 3's side reader over their
/// chart images, holding the points `held`.
fn side(body: &Body<f64>, face: FaceKey, held: &[Point3<f64>]) -> geom_brep::props::MaterialSign {
    let f = body.get_face(face).unwrap();
    let image = topo::props::chart_image_edges(body, f.outer).unwrap().0;
    let surface = body.get_surface(f.surface).unwrap();
    let band = Band::linear(tol()).unwrap();
    geom_brep::props::boundary_material_sign_holding(surface, &[&image], held, band).unwrap()
}

/// **A wrap-tip cap reads its side off the tip its chart image drops.**
/// The image is the rim alone, with a pole on each side of it; held at
/// the tip, the sphere cap lies on the side holding the north pole and
/// not the south, the side its bit claims, and it passes tier 3 and
/// measures its closed form. The cone reads the same side with the tip
/// or without, its apex held by its rim's winding, and passes and
/// measures too.
#[test]
fn a_small_tip_cap_reads_its_side_off_its_tip_and_measures() {
    use geom_brep::props::MaterialSign::{Encoded, Unencoded};
    let (h, rr) = (0.2, 0.2);
    let rows = [
        (
            "the sphere cap",
            small_tip_cap(),
            false,
            PI * h * h * (3.0 - h) / 3.0,
            2.0 * PI * h + PI * 0.36,
        ),
        (
            "the cone",
            small_tip_cone(),
            true,
            PI * rr * rr * h / 3.0,
            PI * rr * (rr * rr + h * h).sqrt() + PI * rr * rr,
        ),
    ];
    for (what, (body, face, tip), reads_bare, volume, area) in rows {
        let sense = body.get_face(face).unwrap().sense;
        let claimed = Encoded(if sense {
            geom_core::Sign::Positive
        } else {
            geom_core::Sign::Negative
        });
        assert_eq!(
            side(&body, face, &[point(&body, tip)]),
            claimed,
            "{what}: held at the tip, the side the bit claims"
        );
        assert_eq!(
            side(&body, face, &[]),
            if reads_bare { claimed } else { Unencoded },
            "{what}: without it"
        );
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

/// **A flipped small wrap-tip cap refuses at tier 3 through the side
/// reader.** The cap's side, read off the tip its chart image drops,
/// disagrees with the flipped bit (`CurvedSenseInverted`); the sphere cap
/// so flipped claims the sphere's far side, and the chart-singularity arm
/// also finds the far pole inside it. The cone's side reader refuses it
/// alone.
#[test]
fn a_flipped_small_tip_cap_refuses_at_tier_3() {
    let (body, face, _) = small_tip_cap();
    let flipped = body.flipped_face_sense_for_tests(face).unwrap();
    let got = topo::validate_geometric(&flipped, tol()).expect_err("flipped refuses");
    assert!(
        got.contains(&ValidationError::CurvedSenseInverted { face }),
        "the sphere cap flipped: the side reader refuses it: {got:?}"
    );
    assert!(
        got.contains(&ValidationError::ChartSingularityNotAVertex {
            face,
            point: geom_brep::chart_singularity::Singularity::South,
            lies: topo::SingularityLies::Interior,
        }),
        "the sphere cap flipped: the south pole inside the face: {got:?}"
    );

    let (body, face, _) = small_tip_cone();
    let flipped = body.flipped_face_sense_for_tests(face).unwrap();
    assert_eq!(
        topo::validate_geometric(&flipped, tol()),
        Err(vec![ValidationError::CurvedSenseInverted { face }]),
        "the cone flipped: the side reader alone"
    );
}
