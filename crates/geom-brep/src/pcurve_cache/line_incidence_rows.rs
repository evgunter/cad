//! **A line's incidence read is its distance's supremum over the span**
//! (`line_incidence`), searched for a counterexample: a parameter of the
//! span where the line is farther from the chart than the read says.
//! Lines are drawn about each chart's singular points (a cone's apex, a
//! spindle's apexes, a sphere's or torus's centre) and its wall, at
//! offsets from far inside the zero band to far past it, and the
//! distance is hunted at an independent schedule: a dense grid with
//! each local maximum refined by golden section, and multi-scale probes
//! about the line's nearest approach to each singular point and to the
//! axis. Each hunted distance is the `Interval` enclosure's lower end at
//! that point, so it is the exact distance or less. The verdicts of the
//! incidence door ([`read_incidence`]) are then held to the hunt at
//! both scalars: a line the hunt shows past the band is refused
//! `CarrierOffChart`, and a line read off is not inside the band.
//!
//! Built with `PCERT_RADIAL_PAIR_MUTANT=sampled-line-incidence` the
//! search finds its counterexamples at once.

test_utils::gated_to![
    "crates/geom-brep/src/pcurve_cache.rs",
    "crates/geom-brep/src/pcurve_cache/line_incidence.rs",
    "crates/geom-brep/src/implicit.rs",
];

use geom::{Curve3, Surface};
use geom_core::{Bounds, Interval, Point3, Real, Vec3};
use test_utils::fuzz;

use super::harmonic_sup_rows::{band, zero};
use super::{PcurveCertifyError, chart_distance, line_incidence, read_incidence};

/// A chart searched: its name, the surface, and its singular points.
type Chart = (&'static str, Surface<f64>, Vec<Point3<f64>>);

/// The charts searched.
fn charts() -> Vec<Chart> {
    let axis = Vec3::new(0.1, -0.2, 1.0).normalize();
    let u_ref = Vec3::new(1.0, 0.0, -0.1).normalize();
    let c = Point3::new(0.4, 0.1, -0.3);
    let spindle = |big: f64, r: f64| {
        let tip = (r * r - big * big).sqrt();
        (
            Surface::Spindle {
                center: c,
                axis,
                major_radius: big,
                minor_radius: r,
                u_ref,
            },
            vec![c + axis * tip, c - axis * tip, c],
        )
    };
    let (apple, apple_tips) = spindle(0.3, 0.5);
    let (lemon, lemon_tips) = spindle(-0.3, 0.5);
    vec![
        (
            "plane",
            Surface::Plane {
                origin: c,
                normal: axis,
                u_ref,
            },
            vec![c],
        ),
        (
            "cylinder",
            Surface::Cylinder {
                origin: c,
                axis,
                radius: 0.7,
                u_ref,
            },
            vec![c],
        ),
        (
            "sphere",
            Surface::Sphere {
                center: c,
                radius: 0.7,
                axis,
                u_ref,
            },
            vec![c],
        ),
        (
            "cone",
            Surface::Cone {
                apex: c,
                axis,
                half_angle: 0.5,
                u_ref,
            },
            vec![c],
        ),
        (
            "narrow cone",
            Surface::Cone {
                apex: c,
                axis,
                half_angle: 0.05,
                u_ref,
            },
            vec![c],
        ),
        (
            "torus",
            Surface::Torus {
                center: c,
                axis,
                major_radius: 1.2,
                minor_radius: 0.4,
                u_ref,
            },
            vec![c],
        ),
        ("apple", apple, apple_tips),
        ("lemon", lemon, lemon_tips),
        ("apple on z", on_z(0.3).0, on_z(0.3).1),
        ("lemon on z", on_z(-0.3).0, on_z(-0.3).1),
    ]
}

/// A spindle about the `z` axis through the origin, whose axis lines
/// have no part off the axis to round, with its apexes and centre.
fn on_z(big: f64) -> (Surface<f64>, Vec<Point3<f64>>) {
    let tip = (0.25 - big * big).sqrt();
    let (c, z) = (Point3::origin(), Vec3::unit_z());
    (
        Surface::Spindle {
            center: c,
            axis: z,
            major_radius: big,
            minor_radius: 0.5,
            u_ref: Vec3::unit_x(),
        },
        vec![c + z * tip, c - z * tip, c],
    )
}

/// The distance's exact value or less at `t`: the enclosure's lower end.
fn distance(surface: &Surface<Interval>, line: &Curve3<Interval>, t: f64) -> f64 {
    chart_distance(surface, line.eval(Interval::from_f64(t))).lo()
}

/// The hunt's largest distance over `[t0, t1]` (module docs).
fn hunt(
    surface: &Surface<f64>,
    singular: &[Point3<f64>],
    line: &geom::Line<f64>,
    (t0, t1): (f64, f64),
) -> f64 {
    let f = Interval::from_f64;
    let surface_i = surface.map_scalar(f);
    let line_i = Curve3::line(
        Point3::new(f(line.origin.x), f(line.origin.y), f(line.origin.z)),
        Vec3::new(f(line.dir.x), f(line.dir.y), f(line.dir.z)),
    );
    let at = |t: f64| distance(&surface_i, &line_i, t.clamp(t0, t1));
    let n: u32 = 2000;
    let grid: Vec<f64> = (0..=n)
        .map(|i| t0 + (t1 - t0) * f64::from(i) / f64::from(n))
        .collect();
    let values: Vec<f64> = grid.iter().map(|t| at(*t)).collect();
    let mut best = values.iter().copied().fold(0.0, f64::max);
    // The eight highest local maxima of the grid, each refined: a
    // rounding-flat stretch has a maximum at every point.
    let mut peaks: Vec<usize> = (1..n as usize)
        .filter(|&i| values[i] >= values[i - 1] && values[i] >= values[i + 1])
        .collect();
    peaks.sort_by(|a, b| values[*b].total_cmp(&values[*a]));
    for &i in peaks.iter().take(8) {
        best = best.max(golden(&at, grid[i - 1], grid[i + 1]));
    }
    let uu = line.dir.dot(line.dir);
    let mut near: Vec<f64> = singular
        .iter()
        .map(|s| (*s - line.origin).dot(line.dir) / uu)
        .collect();
    if let Some(geom::Line { origin: o, dir: a }) = surface.axis_line().map(geom::Line::unit) {
        let (q, u) = (line.origin - o, line.dir);
        let (qo, uo) = (q - a * q.dot(a), u - a * u.dot(a));
        near.push(-qo.dot(uo) / uo.dot(uo));
    }
    for c in near.into_iter().filter(|c| c.is_finite()) {
        best = best.max(at(c));
        for k in 0..60 {
            let step = 1e-15 * 2f64.powi(k) * (t1 - t0);
            best = best.max(at(c + step)).max(at(c - step));
        }
    }
    best
}

/// The largest value of `f` on `[a, b]` golden section reaches.
fn golden(f: &impl Fn(f64) -> f64, mut a: f64, mut b: f64) -> f64 {
    let g = 0.5 * (5f64.sqrt() - 1.0);
    let mut best = f(a).max(f(b));
    for _ in 0..80 {
        let (x, y) = (b - g * (b - a), a + g * (b - a));
        let (fx, fy) = (f(x), f(y));
        best = best.max(fx).max(fy);
        if fx > fy {
            b = y;
        } else {
            a = x;
        }
    }
    best
}

/// The module's read in `f64`: the distance at the span's ends and at
/// its critical fractions.
fn read(surface: &Surface<f64>, line: &geom::Line<f64>, (t0, t1): (f64, f64)) -> f64 {
    let carrier = Curve3::Line(*line);
    [0.0, 1.0]
        .into_iter()
        .chain(line_incidence::critical_fractions(line, surface, (t0, t1)))
        .map(|l| chart_distance(surface, carrier.eval(t0 + (t1 - t0) * l)))
        .fold(0.0, f64::max)
}

fn unit(rng: &mut fuzz::Rng) -> Vec3<f64> {
    loop {
        let v = Vec3::new(
            rng.range(-1.0, 1.0),
            rng.range(-1.0, 1.0),
            rng.range(-1.0, 1.0),
        );
        if v.norm() > 0.1 && v.norm() <= 1.0 {
            return v.normalize();
        }
    }
}

/// A line drawn about `surface`: through one of its singular points, a
/// point of its wall or a point of its axis, displaced by a log-uniform
/// offset or by none, along a direction of the wall there, a direction
/// toward the singular point, the axis, or any direction, tilted or not.
fn draw(rng: &mut fuzz::Rng, surface: &Surface<f64>, singular: &[Point3<f64>]) -> geom::Line<f64> {
    let (u, v) = (rng.range(-3.0, 3.0), rng.range(-1.2, 1.2));
    let wall = surface.eval(u, v);
    let along = match rng.below(3) {
        0 => surface.deriv_u(u, v).normalize(),
        1 => surface.deriv_v(u, v).normalize(),
        _ => {
            let (a, b) = (rng.range(-1.0, 1.0), rng.range(-1.0, 1.0));
            (surface.deriv_u(u, v).normalize() * a + surface.deriv_v(u, v).normalize() * b)
                .normalize()
        }
    };
    let mut scaled = |lo: f64, hi: f64| {
        if rng.below(4) == 0 {
            0.0
        } else {
            zero() * rng.log_uniform(lo, hi)
        }
    };
    let (offset_by, tilt_by) = (scaled(1e-3, 1e3), scaled(1e-4, 1e2));
    let offset = unit(rng) * offset_by;
    let tilt = unit(rng) * tilt_by;
    let (point, dir) = match rng.below(5) {
        4 => match surface.axis_line() {
            Some(axis) => (axis.at(rng.range(-1.0, 1.0)), axis.dir),
            None => (wall, unit(rng)),
        },
        0 | 1 => {
            let s = singular[rng.below(singular.len())];
            let toward = wall - s;
            let dir = if toward.norm() > 1e-6 && rng.below(2) == 0 {
                toward.normalize()
            } else {
                unit(rng)
            };
            (s, dir)
        }
        2 => (wall, along),
        _ => (wall, unit(rng)),
    };
    let dir = if dir.norm().is_finite() {
        dir
    } else {
        unit(rng)
    };
    geom::Line {
        origin: point + offset - dir * rng.range(-1.0, 1.0),
        dir: (dir + tilt) * rng.log_uniform(0.25, 4.0),
    }
}

/// **No parameter of the span is farther from the chart than the read**,
/// and the incidence door's verdict agrees with the hunt at both scalars
/// (module docs).
#[test]
fn no_parameter_of_the_span_is_farther_than_the_read() {
    let mut rng = fuzz::start("line incidence supremum");
    let (zero, escalate) = (zero(), band().escalate());
    let f = Interval::from_f64;
    let (mut off, mut on) = (0, 0);
    for (name, surface, singular) in charts() {
        let surface_i = surface.map_scalar(f);
        for _ in 0..fuzz::scaled(30) {
            let line = draw(&mut rng, &surface, &singular);
            let span = (-rng.range(0.05, 1.5), rng.range(0.05, 1.5));
            let (read, hunted) = (
                read(&surface, &line, span),
                hunt(&surface, &singular, &line, span),
            );
            let reach = (line.origin - Point3::origin()).norm() + line.dir.norm() * 2.0 + 3.0;
            let what = format!("{name}: {line:?} over {span:?}; {}", fuzz::replay());
            assert!(
                read >= hunted - (1e-4 * zero + 256.0 * f64::EPSILON * reach),
                "{what}: the read {read:e} is under a hunted distance {hunted:e}"
            );
            let carrier = Curve3::Line(line);
            let carrier_i = carrier.map_scalar(f);
            let verdicts = [
                read_incidence(&carrier, span.0, span.1, &surface, band()),
                read_incidence(&carrier_i, f(span.0), f(span.1), &surface_i, band()),
            ];
            for verdict in &verdicts {
                let shown_off = matches!(verdict, Err(PcurveCertifyError::CarrierOffChart { .. }));
                if hunted > 1.2 * escalate {
                    assert!(
                        shown_off,
                        "{what}: hunted {hunted:e} past the band, read {verdict:?}"
                    );
                }
                if shown_off {
                    assert!(
                        hunted > 0.8 * escalate,
                        "{what}: read off, hunted only {hunted:e}: {verdict:?}"
                    );
                }
                if verdict.is_ok() {
                    assert!(hunted < 1.2 * zero, "{what}: read on, hunted {hunted:e}");
                }
            }
            off += usize::from(hunted > 1.2 * escalate);
            on += usize::from(hunted < 0.8 * zero);
        }
    }
    println!("[line incidence] past the band {off}, inside the zero band {on}");
}

/// **A line on the chart, or within the zero band of it, is not read
/// off**: each cone's generators through the apex on both nappes, read
/// over spans across the apex and to one side; the same shifted along
/// the rim by `0.5·zero` (`0.44·zero` off at the apex); a cylinder's
/// rulings; and a plane's lines. The incidence door passes each at both
/// scalars, where a read whose rounding the charge did not cover, or a
/// candidate placed off the span, would refuse or escalate.
#[test]
#[allow(
    clippy::approx_constant,
    reason = "3.1415 is an azimuth just short of the seam, not π"
)]
fn a_line_on_the_chart_or_inside_the_zero_band_is_not_read_off() {
    let f = Interval::from_f64;
    let mut read = 0;
    for (name, surface, _) in charts() {
        let mut lines = Vec::new();
        for u in [0.0, 1.0, -2.5, 3.1415] {
            match surface {
                Surface::Cone { apex, .. } => {
                    for v in [0.3, -0.3] {
                        let wall = surface.eval(u, v);
                        let g = wall - apex;
                        let side = surface.deriv_u(u, v).normalize();
                        for shift in [0.0, 0.5 * zero()] {
                            lines.push(geom::Line {
                                origin: wall + side * shift,
                                dir: g,
                            });
                        }
                    }
                }
                Surface::Cylinder { axis, .. } => lines.push(geom::Line {
                    origin: surface.eval(u, 0.4),
                    dir: axis * 0.7,
                }),
                Surface::Plane { .. } => lines.push(geom::Line {
                    origin: surface.eval(u, 0.4),
                    dir: surface.deriv_u(u, 0.4) + surface.deriv_v(u, 0.4) * u,
                }),
                _ => {}
            }
        }
        let surface_i = surface.map_scalar(f);
        for line in lines {
            for span in [(-1.5, 0.5), (-0.4, 1.0), (0.1, 0.6)] {
                let carrier = Curve3::Line(line);
                let carrier_i = carrier.map_scalar(f);
                let verdicts = (
                    read_incidence(&carrier, span.0, span.1, &surface, band()),
                    read_incidence(&carrier_i, f(span.0), f(span.1), &surface_i, band()),
                );
                assert!(
                    verdicts.0.is_ok() && verdicts.1.is_ok(),
                    "{name}: {line:?} over {span:?} is not read off: {:?} / {:?}",
                    verdicts.0,
                    verdicts.1
                );
                read += 1;
            }
        }
    }
    assert_eq!(
        read,
        2 * 4 * 2 * 2 * 3 + 2 * 4 * 3,
        "the table's lines are all read"
    );
}

/// **A line whose candidate closed forms degenerate is still read off**:
/// the poses where a formula of the read is `0/0` or a polynomial
/// vanishes identically, each off its chart only between its ends:
/// - a spindle's axis from apex to apex (apple and lemon, about `z` and
///   about a tilted axis), off by `r − |R|`-ish at the centre;
/// - a sphere's diameter, off by its radius at the centre;
/// - a chord of a ring torus through its core circle, off by `r` there;
/// - a chord of a cone's double nappe through the apex plane and near
///   the apex, off at the apex's kink.
///
/// Each ends `CarrierOffChart` at the incidence door and at the mint, at
/// both scalars. Red against a read that trusts a quartic, which
/// vanishes on the axis, and under `sampled-line-incidence` for the
/// narrow one (the cone chord).
#[test]
fn a_line_whose_candidate_forms_degenerate_is_read_off() {
    let f = Interval::from_f64;
    let u_ref = Vec3::unit_x();
    let mut cases: Vec<(String, Surface<f64>, geom::Line<f64>)> = Vec::new();
    for big in [0.3, -0.3] {
        for axis in [Vec3::unit_z(), Vec3::new(0.1, -0.2, 1.0).normalize()] {
            let c = Point3::new(0.0, 0.0, 0.0);
            let tip = (0.25f64 - big * big).sqrt();
            cases.push((
                format!("spindle R {big} along {axis:?}"),
                Surface::Spindle {
                    center: c,
                    axis,
                    major_radius: big,
                    minor_radius: 0.5,
                    u_ref: (u_ref - axis * u_ref.dot(axis)).normalize(),
                },
                geom::Line {
                    origin: c - axis * tip,
                    dir: axis * (2.0 * tip),
                },
            ));
        }
    }
    cases.push((
        "sphere diameter".into(),
        Surface::Sphere {
            center: Point3::origin(),
            radius: 0.5,
            axis: Vec3::unit_z(),
            u_ref,
        },
        geom::Line {
            origin: Point3::new(0.0, 0.0, -0.5),
            dir: Vec3::unit_z(),
        },
    ));
    // The torus's outer equator point to its inner one at azimuth 0,
    // through the core circle at `x = R`.
    cases.push((
        "torus chord through the core".into(),
        Surface::Torus {
            center: Point3::origin(),
            axis: Vec3::unit_z(),
            major_radius: 1.2,
            minor_radius: 0.4,
            u_ref,
        },
        geom::Line {
            origin: Point3::new(0.8, 0.0, 0.0),
            dir: Vec3::new(0.8, 0.0, 0.0),
        },
    ));
    // Two points of the cone at height ±0.3, either side of the axis,
    // shifted off the apex by `30·zero`: the chord crosses the apex plane
    // `30·zero` from the apex.
    let cone = Surface::Cone {
        apex: Point3::origin(),
        axis: Vec3::unit_z(),
        half_angle: 0.5,
        u_ref,
    };
    let r = 0.3 * 0.5f64.tan();
    let side = Vec3::new(0.0, 30.0 * zero(), 0.0);
    cases.push((
        "cone chord past the apex".into(),
        cone,
        geom::Line {
            origin: Point3::new(r, 0.0, 0.3) + side,
            dir: Point3::new(-r, 0.0, -0.3) - Point3::new(r, 0.0, 0.3),
        },
    ));
    for (what, surface, line) in cases {
        let carrier = Curve3::Line(line);
        let (carrier_i, surface_i) = (carrier.map_scalar(f), surface.map_scalar(f));
        // The second span puts the middle off the sampled schedule.
        for span in [(0.0, 1.0), (0.013, 0.9)] {
            let off = |r: Option<&PcurveCertifyError>| {
                matches!(r, Some(PcurveCertifyError::CarrierOffChart { .. }))
            };
            let door = (
                read_incidence(&carrier, span.0, span.1, &surface, band()),
                read_incidence(&carrier_i, f(span.0), f(span.1), &surface_i, band()),
            );
            let mint = (
                super::chart_pcurve_over(&carrier, span.0, span.1, &surface, band()),
                super::chart_pcurve_over(&carrier_i, f(span.0), f(span.1), &surface_i, band()),
            );
            assert!(
                off(door.0.as_ref().err()) && off(door.1.as_ref().err()),
                "{what} over {span:?}: the door refuses CarrierOffChart at both scalars: {:?} / {:?}",
                door.0,
                door.1
            );
            assert!(
                off(mint.0.as_ref().err()) && off(mint.1.as_ref().err()),
                "{what} over {span:?}: the mint refuses CarrierOffChart at both scalars: {:?} / {:?}",
                mint.0,
                mint.1
            );
        }
    }
}
