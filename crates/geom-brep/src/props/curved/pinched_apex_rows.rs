//! **A cone loop pinched at its apex is measured lobe by lobe**
//! ([`super::require_apex_lobes`]): the 30° cone, apex at the origin and
//! its nappe below, its loops passing the apex twice.

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

fn line(from: Point3<f64>, to: Point3<f64>) -> LoopEdge<f64> {
    let dir = to - from;
    LoopEdge::hand_built(
        Curve3::line(from, dir.normalize()),
        0.0,
        dir.norm(),
        true,
        0,
        0,
    )
}

fn arc(depth: f64, span: (f64, f64), forward: bool) -> LoopEdge<f64> {
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
        0,
        0,
    )
}

/// The lateral area of the nappe down to `depth`, over the azimuth `span`.
fn lateral(depth: f64, span: f64) -> f64 {
    let alpha = 30f64.to_radians();
    span / 2.0 * (depth * t()) * (depth / alpha.cos())
}

/// The sector lobe `apex → (depth, θ₀) → arc to θ₁ → apex`.
fn sector(depth: f64, from: f64, to: f64) -> [LoopEdge<f64>; 3] {
    let apex = Point3::origin();
    [
        line(apex, at(depth, from)),
        arc(depth, (from, to), true),
        line(at(depth, to), apex),
    ]
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

/// **The whole turn a wrap seam closes, less a notch at the apex**: the
/// seam at azimuth `0` down to the base at depth `1`, the base round,
/// the seam back up, and the notch's lobe between azimuths `π/2` and `π`
/// above depth `0.45`, wound against the turn. Read from every start, it
/// measures the nappe less the notch.
#[test]
fn a_whole_turn_less_a_notch_measures_from_every_start() {
    let apex = Point3::origin();
    let (d1, d2) = (0.45, 1.0);
    let edges = [
        arc(d1, (FRAC_PI_2, PI), true),
        line(at(d1, PI), apex),
        line(apex, at(d2, 0.0)),
        arc(d2, (0.0, TAU), false),
        line(at(d2, 0.0), apex),
        line(apex, at(d1, FRAC_PI_2)),
    ];
    let want = lateral(d2, TAU) - lateral(d1, FRAC_PI_2);
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

/// **Two sectors side by side at the apex** measure their sum; two whose
/// wedges overlap there cover it twice and refuse
/// (`props_cone_apex_lobes`).
#[test]
fn sectors_at_the_apex_measure_apart_and_refuse_overlapping() {
    let apart: Vec<_> = sector(1.0, 0.0, FRAC_PI_2)
        .into_iter()
        .chain(sector(0.5, PI, 1.5 * PI))
        .collect();
    let want = lateral(1.0, FRAC_PI_2) + lateral(0.5, FRAC_PI_2);
    let area = measured(&apart).unwrap();
    assert!((area - want).abs() < 1e-12, "{area} against {want}");
    let overlapping: Vec<_> = sector(1.0, 0.0, PI)
        .into_iter()
        .chain(sector(0.5, FRAC_PI_2, 1.5 * PI))
        .collect();
    assert_eq!(
        measured(&overlapping),
        Err(PropsError::ConeLoop {
            what: "props_cone_apex_lobes"
        })
    );
}
