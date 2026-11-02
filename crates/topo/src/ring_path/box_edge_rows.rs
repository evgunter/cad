//! **A torus circle the plane×torus lane mints at the edge of the
//! session box reads as it reads at the origin**: the lane mints from
//! stored directions and radii, never from a difference of far points.

use super::cone_islands::{Frame, band, frames};
use super::torus_islands::{IslandEdge, RADII, meridian, parallel, torus};
use super::*;
use core::f64::consts::{FRAC_PI_2, PI};
use geom_core::Tol;
use geom_core::tolerance::{EPS_MIN, SESSION_BOX_EXTENT};

/// The kind [`Quadric::torus_circle_arc`] reads a circle as: a
/// meridian, a parallel on its side of `R`, a crest on its side of the
/// equator plane, `None` or an escalation.
fn circle_kind(read: Result<Option<LoopArc<f64>>, Indeterminate>) -> String {
    match read {
        Err(_) => "escalated".into(),
        Ok(None) => "none".into(),
        Ok(Some(LoopArc::Crest { own, .. })) => format!("crest {}", own.positive),
        Ok(Some(LoopArc::Conic(ConicArc { own: Some(own), .. }))) => match own.split {
            LoopSplit::Side { .. } => "meridian".into(),
            LoopSplit::Radius { .. } => format!("parallel {}", own.positive),
            LoopSplit::Bisector { .. } => "bisector".into(),
        },
        Ok(Some(_)) => "another arc".into(),
    }
}

/// The section of `torus(f, mirror)` by the plane of `edge`, stored at
/// its centre moved `shift` local units along the plane, through
/// [`geom_brep::plane_torus_section`] at `band`: the section's kind and,
/// for the circle of it nearest `edge`'s carrier, the kind the ring lane
/// reads.
fn minted_kind(
    f: Frame,
    mirror: bool,
    edge: &IslandEdge,
    shift: f64,
    band: Band,
) -> (&'static str, String) {
    use geom_brep::PlaneTorusSection as S;
    let surface = torus(f, mirror);
    let quadric = Quadric::of(&surface).unwrap();
    let (centre, normal) = edge.plane;
    let u_ref = normal.cross(f.x + f.y * 0.5).normalize();
    let plane = geom::Surface::Plane {
        origin: centre + u_ref * (shift * f.s),
        normal,
        u_ref,
    };
    let extent = geom_core::test_support::fixture_lever(2.0 * (RADII.0 + RADII.1) * f.s);
    let (section, circles) = match geom_brep::plane_torus_section(&plane, &surface, extent, band) {
        Ok(S::MeridianCircles { c1, c2 }) => ("meridians", vec![c1, c2]),
        Ok(S::ConcentricCircles { c1, c2 }) => ("concentric", vec![c1, c2]),
        Ok(S::TangentCircle(c)) => ("tangent", vec![c]),
        Ok(S::SpiricOvals { .. }) => ("ovals", vec![]),
        Ok(S::Empty) => ("empty", vec![]),
        Err(_) => ("refused", vec![]),
    };
    let geom::Curve3::Circle(geom::Circle {
        frame: geom::Frame { center: k, .. },
        radius: rho,
    }) = edge.carrier
    else {
        unreachable!("a meridian or a parallel")
    };
    let off = |c: &geom::Curve3<f64>| match c {
        geom::Curve3::Circle(geom::Circle {
            frame: geom::Frame { center, .. },
            radius,
        }) => (*center - k).norm() + (radius - rho).abs(),
        _ => f64::INFINITY,
    };
    let kind = circles
        .into_iter()
        .min_by(|a, b| off(a).total_cmp(&off(b)))
        .map_or_else(String::new, |c| {
            circle_kind(quadric.torus_circle_arc(&c, edge.params, band))
        });
    (section, kind)
}

/// How far, in local units, [`minted_kind`] stores a plane's origin from
/// its circle's centre when it does not store it there.
const SHIFT: f64 = 3.7;

/// **A torus circle the plane×torus lane mints anywhere in the session
/// box reads the section and the kind it reads at the origin**: the
/// meridians, the parallels either side of `R` and the crests of a grid
/// round the torus, each cut by its plane stored at the circle's centre
/// or off it, in the three frames' axes moved out to the box's edge at
/// scales 1 and `1e2`, at the run's ε and at [`EPS_MIN`]. The lane reads
/// its gaps along the plane's normal on the difference of the plane's
/// stored origin and the torus's centre, so the far frame's rounding
/// enters them as a few ulps of the box, under [`EPS_MIN`]. It mints a
/// circle's axis as the plane's normal or the torus's axis and its radius
/// as `r`, `R` or `R ± w` with `w` decided off zero, none of them a
/// difference of far points, so the four readings of
/// [`Quadric::torus_circle_arc`] see what they see at the origin.
#[test]
fn a_torus_circle_the_section_lane_mints_in_the_box_reads_its_kind_at_the_origin() {
    let us: Vec<f64> = (0..12)
        .map(|k| -PI + (f64::from(k) + 0.31) * PI / 6.0)
        .collect();
    let vs: Vec<f64> = (0..16)
        .map(|k| -PI + (f64::from(k) + 0.43) * PI / 8.0)
        .chain([FRAC_PI_2, -FRAC_PI_2, 0.0, PI])
        .collect();
    let bands = [
        band(),
        Band::linear_at(Tol::witness(), EPS_MIN).expect("the floor is an admissible ε"),
    ];
    let mut compared = std::collections::BTreeMap::<String, usize>::new();
    let mut other_section = Vec::new();
    let mut wrong = Vec::new();
    for band in bands {
        for axes in frames() {
            for s in [1.0, 1e2] {
                // Every stored point lies within the torus's outer radius
                // and the plane's shift of the frame's origin.
                let edge_of_box = SESSION_BOX_EXTENT - (RADII.0 + RADII.1 + SHIFT) * s;
                let origins: Vec<_> = [1e2, 3e2, edge_of_box]
                    .into_iter()
                    .flat_map(|m| {
                        [
                            (1.0, 0.0, 0.0),
                            (0.8, -0.3, 0.5),
                            (-0.2, 0.9, 0.4),
                            (0.0, 0.0, -1.0),
                        ]
                        .map(|(x, y, z)| Point3::origin() + Vec3::new(x, y, z).normalize() * m)
                    })
                    .collect();
                for mirror in [false, true] {
                    let home = Frame {
                        o: Point3::origin(),
                        s,
                        ..axes
                    };
                    let edges = us
                        .iter()
                        .map(|&u| -> Box<dyn Fn(Frame) -> IslandEdge> {
                            Box::new(move |f| meridian(f, u, (0.1, 0.5)))
                        })
                        .chain(vs.iter().map(|&v| -> Box<dyn Fn(Frame) -> IslandEdge> {
                            Box::new(move |f| parallel(f, v, (0.1, 0.5)))
                        }));
                    for edge in edges {
                        for shift in [0.0, SHIFT] {
                            let want = minted_kind(home, mirror, &edge(home), shift, band);
                            for &o in &origins {
                                let f = Frame { o, ..home };
                                let got = minted_kind(f, mirror, &edge(f), shift, band);
                                let at = format!("at {o:?}, scale {s}, {band:?}");
                                if got.0 != want.0 {
                                    other_section.push(format!("{} → {} {at}", want.0, got.0));
                                } else if got.1 != want.1 {
                                    wrong.push(format!("{want:?} reads {got:?} {at}"));
                                } else {
                                    *compared.entry(got.1).or_default() += 1;
                                }
                            }
                        }
                    }
                }
            }
        }
    }
    let mut by_class = std::collections::BTreeMap::<&str, usize>::new();
    for o in &other_section {
        *by_class
            .entry(o.split(" at ").next().unwrap_or(o))
            .or_default() += 1;
    }
    assert!(
        other_section.is_empty(),
        "{} sections in the box read another section ({by_class:?}), the first {:?}",
        other_section.len(),
        &other_section[..other_section.len().min(3)]
    );
    assert!(
        wrong.is_empty(),
        "{} circles in the box read another kind, the first {:?}",
        wrong.len(),
        &wrong[..wrong.len().min(3)]
    );
    let kinds = [
        "meridian",
        "parallel true",
        "parallel false",
        "crest true",
        "crest false",
    ];
    assert!(
        kinds
            .iter()
            .all(|k| compared.get(*k).is_some_and(|&n| n >= 100))
            && compared.len() == kinds.len(),
        "each kind read on a hundred circles in the box or more: {compared:?}"
    );
}
