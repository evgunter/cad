//! **A cone loop that passes its apex more than once is measured lobe by
//! lobe, or refused by name** ([`super::require_apex_lobes`]): the 30°
//! cone, apex at the origin and its nappe below. Each point is a vertex
//! with its own tag, the apex's `0`.

use core::f64::consts::{FRAC_PI_2, PI, TAU};

use geom::Curve3;
use geom_core::{Band, Point3, Tol, Vec3};

use super::cone_face_closed_form;
use crate::props::{LoopEdge, PropsError};

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

fn t() -> f64 {
    30f64.to_radians().tan()
}

/// The point of the nappe at `depth` below the apex, azimuth `theta`.
fn at(depth: f64, theta: f64) -> Point3<f64> {
    let (s, c) = theta.sin_cos();
    Point3::new(depth * t() * c, depth * t() * s, -depth)
}

/// The apex, or the point `at(depth, theta)`, with its vertex tag.
#[derive(Clone, Copy)]
struct V(Point3<f64>, u32);

const APEX: V = V(Point3::new(0.0, 0.0, 0.0), 0);

fn line(from: V, to: V) -> LoopEdge<f64> {
    let dir = to.0 - from.0;
    LoopEdge::hand_built(
        Curve3::line(from.0, dir.normalize()),
        0.0,
        dir.norm(),
        true,
        from.1,
        to.1,
    )
}

/// The rim at `depth` over `span`, traversed forward or back, between
/// the vertices tagged `tags` in traversal order.
fn arc(depth: f64, span: (f64, f64), forward: bool, tags: (u32, u32)) -> LoopEdge<f64> {
    LoopEdge::hand_built(
        Curve3::Circle(geom::Circle::new(
            geom::Frame::new(
                Point3::new(0.0, 0.0, -depth),
                Vec3::new(0.0, 0.0, 1.0),
                Vec3::new(1.0, 0.0, 0.0),
            ),
            depth * t(),
        )),
        span.0,
        span.1,
        forward,
        tags.0,
        tags.1,
    )
}

/// The lateral area of the nappe down to `depth`, over the azimuth `span`.
fn lateral(depth: f64, span: f64) -> f64 {
    let alpha = 30f64.to_radians();
    span / 2.0 * (depth * t()) * (depth / alpha.cos())
}

/// The sector lobe from the apex down the ruling at `from`, round the
/// rim at `depth` to `to` (backward where `to < from`), and up, its two
/// rim vertices tagged `tags`.
fn sector(depth: f64, from: f64, to: f64, tags: (u32, u32)) -> [LoopEdge<f64>; 3] {
    let (a, b) = (V(at(depth, from), tags.0), V(at(depth, to), tags.1));
    let rim = if from < to {
        arc(depth, (from, to), true, tags)
    } else {
        arc(depth, (to, from), false, tags)
    };
    [line(APEX, a), rim, line(b, APEX)]
}

fn measured(edges: &[LoopEdge<f64>]) -> Result<f64, PropsError> {
    let apex = Point3::origin();
    let axis = Vec3::new(0.0, 0.0, 1.0);
    let cone = geom::Surface::Cone {
        frame: geom::Frame::new(apex, axis, Vec3::unit_x()),
        half_angle: 30f64.to_radians(),
    };
    cone_face_closed_form(&cone, apex, axis, 30f64.to_radians(), &[edges], band()).map(|c| c.area)
}

fn refused(what: &'static str) -> Result<f64, PropsError> {
    Err(PropsError::ConeLoop { what })
}

fn chain<const A: usize, const B: usize>(
    a: [LoopEdge<f64>; A],
    b: [LoopEdge<f64>; B],
) -> Vec<LoopEdge<f64>> {
    a.into_iter().chain(b).collect()
}

/// The whole turn a wrap seam at azimuth `0` closes down to depth `1`,
/// less the notch between azimuths `π/2` and `π` above depth `0.45`,
/// wound against the turn; the second pass at the apex tagged
/// `second_apex`.
fn turn_less_notch(second_apex: u32) -> [LoopEdge<f64>; 6] {
    let (d1, d2) = (0.45, 1.0);
    let (v4, v5, v2) = (V(at(d1, FRAC_PI_2), 1), V(at(d1, PI), 2), V(at(d2, 0.0), 3));
    let other = V(APEX.0, second_apex);
    [
        arc(d1, (FRAC_PI_2, PI), true, (1, 2)),
        line(v5, other),
        line(other, v2),
        arc(d2, (0.0, TAU), false, (3, 3)),
        line(v2, APEX),
        line(APEX, v4),
    ]
}

/// **The whole turn less a notch at the apex measures from every start**:
/// the nappe less the notch.
#[test]
fn a_whole_turn_less_a_notch_measures_from_every_start() {
    let edges = turn_less_notch(0);
    let want = lateral(1.0, TAU) - lateral(0.45, FRAC_PI_2);
    for start in 0..edges.len() {
        let turned: Vec<_> = edges[start..]
            .iter()
            .chain(&edges[..start])
            .cloned()
            .collect();
        let area = measured(&turned).unwrap_or_else(|e| panic!("start {start}: {e:?}"));
        assert!(
            (area - want).abs() < 1e-12,
            "start {start}: {area} against {want}"
        );
    }
}

/// **Two vertices at the apex point are no pinch of one face**
/// (`props_cone_apex_vertices`): the same loop with its second pass at
/// the apex at a vertex of its own, as two cones of material meeting
/// there leave it, refuses from every start.
#[test]
fn two_vertices_at_the_apex_refuse_from_every_start() {
    let edges = turn_less_notch(9);
    for start in 0..edges.len() {
        let turned: Vec<_> = edges[start..]
            .iter()
            .chain(&edges[..start])
            .cloned()
            .collect();
        assert_eq!(
            measured(&turned),
            refused("props_cone_apex_vertices"),
            "start {start}"
        );
    }
}

/// **Sectors side by side at the apex measure their sum**, wound either
/// way: every arc of the turn is covered once or not at all, by one sign.
#[test]
fn sectors_side_by_side_measure_their_sum_either_way_round() {
    let want = lateral(1.0, FRAC_PI_2) + lateral(0.5, FRAC_PI_2);
    for (what, edges) in [
        (
            "counter-clockwise",
            chain(
                sector(1.0, 0.0, FRAC_PI_2, (1, 2)),
                sector(0.5, PI, 1.5 * PI, (3, 4)),
            ),
        ),
        (
            "clockwise",
            chain(
                sector(1.0, FRAC_PI_2, 0.0, (1, 2)),
                sector(0.5, 1.5 * PI, PI, (3, 4)),
            ),
        ),
    ] {
        let area = measured(&edges).unwrap_or_else(|e| panic!("{what}: {e:?}"));
        assert!((area - want).abs() < 1e-12, "{what}: {area} against {want}");
    }
}

fn covers_other_than_once(edges: &[LoopEdge<f64>]) {
    assert_eq!(measured(edges), refused("props_cone_apex_lobes"));
}

/// **Sectors whose wedges overlap cover the apex twice, and refuse**
/// (`props_cone_apex_lobes`).
#[test]
fn overlapping_sectors_refuse() {
    covers_other_than_once(&chain(
        sector(1.0, 0.0, PI, (1, 2)),
        sector(0.5, FRAC_PI_2, 1.5 * PI, (3, 4)),
    ));
}

/// **A sector beside a notch wound against it refuses**
/// (`props_cone_apex_lobes`): it covers the apex once on one side and
/// once against the face's side on the other.
#[test]
fn a_sector_beside_a_notch_wound_against_it_refuses() {
    covers_other_than_once(&chain(
        sector(1.0, 0.0, FRAC_PI_2, (1, 2)),
        sector(0.5, 1.5 * PI, PI, (3, 4)),
    ));
}

/// **Two whole turns wound against each other cover the apex nowhere,
/// and refuse** (`props_cone_apex_lobes`): the band between their rims
/// is a region, but its loop's rulings to the apex bound no face there.
#[test]
fn two_whole_turns_wound_against_each_other_refuse() {
    let seam = |depth: f64, forward: bool, tag: u32| {
        let v = V(at(depth, if forward { 0.0 } else { PI }), tag);
        let span = if forward { (0.0, TAU) } else { (PI, PI + TAU) };
        [
            line(APEX, v),
            arc(depth, span, forward, (tag, tag)),
            line(v, APEX),
        ]
    };
    covers_other_than_once(&chain(seam(1.0, true, 1), seam(0.5, false, 2)));
}

/// **A lobe sweeping past a turn refuses** (`props_cone_apex_loop_sweep`)
/// beside a lobe that does not: a turn and a quarter, down the ruling at
/// `0` round the rim at depth `1` to `3π/2`, along that ruling to depth
/// `0.7` and round on to `5π/2`.
#[test]
fn a_lobe_sweeping_past_a_turn_refuses() {
    let (a, b, c, d) = (
        V(at(1.0, 0.0), 1),
        V(at(1.0, 1.5 * PI), 2),
        V(at(0.7, 1.5 * PI), 3),
        V(at(0.7, 2.5 * PI), 4),
    );
    let past = [
        line(APEX, a),
        arc(1.0, (0.0, 1.5 * PI), true, (1, 2)),
        line(b, c),
        arc(0.7, (1.5 * PI, 2.5 * PI), true, (3, 4)),
        line(d, APEX),
    ];
    let edges = chain(past, sector(0.4, PI, 1.25 * PI, (5, 6)));
    assert_eq!(measured(&edges), refused("props_cone_apex_loop_sweep"));
}

/// **Two lobes meeting off the apex refuse**
/// (`props_cone_apex_lobe_vertex`): sectors side by side sharing their
/// ruling's rim vertex, a second meeting point of the loop.
#[test]
fn lobes_meeting_off_the_apex_refuse() {
    let edges = chain(
        sector(1.0, 0.0, FRAC_PI_2, (1, 2)),
        sector(1.0, FRAC_PI_2, PI, (2, 3)),
    );
    assert_eq!(measured(&edges), refused("props_cone_apex_lobe_vertex"));
}
