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
    turn_less_notch_at(0.45, second_apex)
}

/// [`turn_less_notch`] with the notch at depth `d1`.
fn turn_less_notch_at(d1: f64, second_apex: u32) -> [LoopEdge<f64>; 6] {
    let d2 = 1.0;
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

/// `edges` turned to start at each of its edges in turn.
fn every_start(edges: &[LoopEdge<f64>]) -> impl Iterator<Item = (usize, Vec<LoopEdge<f64>>)> + '_ {
    (0..edges.len()).map(|start| {
        (
            start,
            edges[start..]
                .iter()
                .chain(&edges[..start])
                .cloned()
                .collect(),
        )
    })
}

/// **Each pair of wedge sides is read where both its rulings reach**: a
/// thin sector of width `δ` at depth `1`, beside a sector a whole radian
/// wide whose rulings reach only depth `d` (both scaled with the band). Wound with the thin one it
/// is a face, and measures; wound against it, a notch outside it, it is
/// none, and refuses (`props_cone_apex_lobes`, or degenerate where its
/// area is none). Read at the short
/// lobe's reach, the thin sector's two sides would be one ruling and it
/// a whole turn, and the two verdicts swap.
#[test]
fn a_thin_sector_beside_a_short_lobe_reads_at_each_pairs_own_reach() {
    // Posed at the band of `1e-9`, and scaled with the run's: the widths
    // and the short reach move with it, and a reach past `1e-2` is no
    // longer short.
    let k = band().zero() / 1e-9;
    for delta in [1e-5, 1e-6, 1e-7].map(|x| x * k) {
        for d in [1e-6, 1e-5, 1e-4, 1e-3, 1e-2]
            .map(|x| x * k)
            .into_iter()
            .filter(|&d| d <= 1e-2)
        {
            let thin = || sector(1.0, 0.3, 0.3 + delta, (1, 2));
            let face = chain(thin(), sector(d, 2.0, 3.0, (3, 4)));
            let notch = chain(thin(), sector(d, 3.0, 2.0, (3, 4)));
            let want = lateral(1.0, delta) + lateral(d, 1.0);
            for (start, turned) in every_start(&face) {
                let area = measured(&turned)
                    .unwrap_or_else(|e| panic!("δ {delta}, d {d}, start {start}: {e:?}"));
                // The vector area sums terms the size of the unit
                // sector's, so its rounding is theirs.
                assert!(
                    (area - want).abs() <= 1e-14,
                    "δ {delta}, d {d}, start {start}: {area} against {want}"
                );
            }
            // Where the notch's area equals the sector's (`δ = d²`) the
            // loop's area is none, refused first as degenerate.
            let wanted = if (delta - d * d).abs() <= 1e-12 * delta {
                Err(PropsError::DegenerateFace)
            } else {
                refused("props_cone_apex_lobes")
            };
            for (start, turned) in every_start(&notch) {
                assert_eq!(
                    measured(&turned),
                    wanted,
                    "δ {delta}, d {d}, start {start}: a notch outside the sector"
                );
            }
        }
    }
}

/// **Lobes whose scales differ by a thousand and more measure**: sectors
/// side by side at depths `1` and `d`, and the whole turn less a notch at
/// depth `d`, from every start, at every `d` a hundred bands and more.
#[test]
fn lobes_of_scales_a_thousand_apart_measure() {
    let floor = 100.0 * band().zero();
    for d in [1e-3, 1e-4, 1e-5].into_iter().filter(|&d| d >= floor) {
        let sectors = chain(
            sector(1.0, 0.0, FRAC_PI_2, (1, 2)),
            sector(d, PI, 1.5 * PI, (3, 4)),
        );
        let notched = turn_less_notch_at(d, 0);
        for (what, edges, want) in [
            (
                "sectors",
                sectors,
                lateral(1.0, FRAC_PI_2) + lateral(d, FRAC_PI_2),
            ),
            (
                "a turn less a notch",
                notched.to_vec(),
                lateral(1.0, TAU) - lateral(d, FRAC_PI_2),
            ),
        ] {
            for (start, turned) in every_start(&edges) {
                let area = measured(&turned)
                    .unwrap_or_else(|e| panic!("{what}, d {d}, start {start}: {e:?}"));
                assert!(
                    (area - want).abs() <= 1e-12 * want.max(1.0),
                    "{what}, d {d}, start {start}: {area} against {want}"
                );
            }
        }
    }
}
