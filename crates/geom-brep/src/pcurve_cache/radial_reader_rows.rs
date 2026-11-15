//! **A point built on a cylinder or a sphere at a parameter radius
//! reads its chart with no sign asked of the radius.** A point at
//! `c + r·x`, `x` a unit direction, is on the chart of radius `r` at
//! every `r`; over a box on `r` its distance from the chart
//! ([`chart_distance`]) is the zero form, and a ruling through it is
//! placed at `x`'s own angle ([`chart_pcurve`]). The same readers at
//! `f64` and `Interval` are held to the direct spellings, `|‖w‖ − R|`
//! and the angle of the offset at its length, on a table that runs
//! from the axis to far past the wall.
//!
//! Each row names in its own doc the mutant it is red under.

test_utils::gated_to!["crates/geom-brep/src/pcurve_cache.rs"];

use geom::{Curve3, Surface};
use geom_core::k_stats::decide_reported;
use geom_core::sym::with_session;
use geom_core::{
    Bounds, Decided, Interval, Margin, ParamSymbol, Point3, Real, Sign, Sym, SymBudget, Vec3,
};

use super::harmonic_sup_rows::band;
use super::{Azimuth, chart_distance, chart_pcurve, ruling_direction};

type S = Sym<Interval>;

/// The box on the radius every row here reads over, and a box on the
/// angle a direction is built at.
const RADIUS: (f64, f64) = (0.2, 0.4);
const ANGLE: (f64, f64) = (0.3, 0.9);

fn budget() -> SymBudget {
    SymBudget {
        max_terms: 4096,
        max_degree: 64,
    }
}

fn boxed(name: &str, (lo, hi): (f64, f64)) -> S {
    Sym::param_over(
        ParamSymbol::new(test_utils::symbol_id(name)),
        Interval::from_bounds(lo, hi),
        lo,
        hi,
    )
}

fn lit(x: f64) -> S {
    S::from_f64(x)
}

fn lit3(v: Vec3<f64>) -> Vec3<S> {
    Vec3::new(lit(v.x), lit(v.y), lit(v.z))
}

/// The chart's frame: an axis, its reference direction and their
/// cross, each a literal unit vector, and the chart's centre.
fn frame() -> (Vec3<f64>, Vec3<f64>, Vec3<f64>, Point3<f64>) {
    let axis = Vec3::new(0.0, 0.0, 1.0);
    let u_ref = Vec3::new(0.0, 1.0, 0.0);
    (axis, u_ref, axis.cross(u_ref), Point3::new(0.5, -0.25, 2.0))
}

/// The directions a point is built along, square to the axis: the
/// frame's own four, and one at a boxed angle, unit by `cos² + sin²`.
fn radial_directions() -> Vec<(&'static str, Vec3<S>)> {
    let (_, u_ref, cv, _) = frame();
    let (s, c) = boxed("radial_reader_angle", ANGLE).sin_cos();
    vec![
        ("u_ref", lit3(u_ref)),
        ("cv", lit3(cv)),
        ("-u_ref", lit3(-u_ref)),
        ("-cv", lit3(-cv)),
        ("at a boxed angle", lit3(u_ref) * c + lit3(cv) * s),
    ]
}

fn point(c: Point3<f64>) -> Point3<S> {
    Point3::new(lit(c.x), lit(c.y), lit(c.z))
}

/// How a length reads over the box: its sign, and whether the tier
/// proved it.
fn reading(d: S) -> (Sign, bool) {
    let Decided { sign, margin } = decide_reported("radial_reader_row", Margin::of(d), band())
        .unwrap_or_else(|e| panic!("a length that does not decide: {e:?} ({:?})", d.value));
    (sign, margin.proven())
}

const PROVEN_ZERO: (Sign, bool) = (Sign::Zero, true);

/// **A point built on a cylinder is on it as a theorem, over a box on
/// the radius.** The point `c + r·x + h·axis`, `x` a unit direction
/// square to the axis, against the cylinder of radius `r`: the distance
/// is the zero form, with nothing known of `r`'s sign. Against the
/// radius spelled `|r|` it is the zero form too (`|r|² = r²` in the
/// tier), as it was read as `√(r²) − |r|`.
///
/// Red under `PCERT_RADIAL_PAIR_MUTANT=distance-raw-norm` at the radius
/// `r`: the distance is `|√(r²) − r|`, as wide as the box.
#[test]
fn a_point_built_on_a_cylinder_is_on_it_over_a_box_on_the_radius() {
    let (axis, u_ref, _, c) = frame();
    with_session(budget(), || {
        let r = boxed("radial_reader_radius", RADIUS);
        let h = boxed("radial_reader_height", (-1.0, 2.0));
        for (name, x) in radial_directions() {
            for (spelling, radius) in [("r", r), ("|r|", r.abs())] {
                let cylinder = Surface::Cylinder {
                    origin: point(c),
                    axis: lit3(axis),
                    radius,
                    u_ref: lit3(u_ref),
                };
                let p = point(c) + x * r + lit3(axis) * h;
                assert_eq!(
                    reading(chart_distance(&cylinder, p)),
                    PROVEN_ZERO,
                    "along {name}, radius {spelling}"
                );
            }
        }
    });
}

/// **A point built on a sphere is on it as a theorem, over a box on the
/// radius**: `c + r·x` for a unit `x`, on the frame's directions and at
/// a boxed longitude and latitude, against the radius `r` and against
/// `|r|`.
///
/// Red under `PCERT_RADIAL_PAIR_MUTANT=distance-raw-norm` at the radius
/// `r`.
#[test]
fn a_point_built_on_a_sphere_is_on_it_over_a_box_on_the_radius() {
    let (axis, u_ref, _, c) = frame();
    with_session(budget(), || {
        let r = boxed("radial_reader_radius", RADIUS);
        let (s, cs) = boxed("radial_reader_latitude", (-0.4, 1.1)).sin_cos();
        let mut directions = radial_directions();
        directions.push(("the pole", lit3(axis)));
        let (_, parallel) = directions[4];
        directions.push(("at a boxed latitude", parallel * cs + lit3(axis) * s));
        for (name, x) in directions {
            for (spelling, radius) in [("r", r), ("|r|", r.abs())] {
                let sphere = Surface::Sphere {
                    center: point(c),
                    radius,
                    axis: lit3(axis),
                    u_ref: lit3(u_ref),
                };
                assert_eq!(
                    reading(chart_distance(&sphere, point(c) + x * r)),
                    PROVEN_ZERO,
                    "along {name}, radius {spelling}"
                );
            }
        }
    });
}

/// **A ruling built on a cylinder is placed at the angle it was built
/// along, over a box on the radius.** The line through `c + r·x` along
/// the axis, on the cylinder of radius `r`: its image is derived, its
/// azimuth less `x`'s angle is the zero form where the tier folds that
/// angle (`x = ±u_ref`: `0` and `π`), and there the image maps back
/// onto the line as a theorem at every parameter read. Along `±cv` the
/// angle is `atan2(±1, 0)`, no form the tier folds, and reads as narrow
/// as its literal.
///
/// Red under `PCERT_RADIAL_PAIR_MUTANT=ruling-azimuth-unscaled` along
/// `±u_ref`: the azimuth is `atan2(0, ±r)`, an atom, and the image
/// misses the line by the box's width. Red under `distance-raw-norm`
/// too, where the line is not shown on the chart.
#[test]
fn a_ruling_built_on_a_cylinder_is_placed_at_its_own_angle_over_a_box() {
    let (axis, u_ref, cv, c) = frame();
    let quarter = core::f64::consts::FRAC_PI_2;
    let directions = [
        ("u_ref", u_ref, 0.0, true),
        ("cv", cv, quarter, false),
        ("-u_ref", -u_ref, 2.0 * quarter, true),
        ("-cv", -cv, 3.0 * quarter, false),
    ];
    with_session(budget(), || {
        let r = boxed("radial_reader_radius", RADIUS);
        for (name, x, angle, folds) in directions {
            let cylinder = Surface::Cylinder {
                origin: point(c),
                axis: lit3(axis),
                radius: r,
                u_ref: lit3(u_ref),
            };
            let line = geom::Line {
                origin: point(c) + lit3(x) * r,
                dir: lit3(axis),
            };
            let image = chart_pcurve(&Curve3::Line(line), &cylinder, band())
                .unwrap_or_else(|e| panic!("along {name}: no image: {e:?}"));
            for t in [0.0, 0.5, 1.0] {
                let q = image.eval(lit(t));
                if folds {
                    let turns = if angle == 0.0 { S::zero() } else { S::pi() };
                    assert_eq!(
                        reading(q.x - turns),
                        PROVEN_ZERO,
                        "along {name} at {t}: the azimuth is the angle built along"
                    );
                    assert_eq!(
                        reading(
                            (cylinder.eval(q.x, q.y) - (line.origin + line.dir * lit(t))).norm()
                        ),
                        PROVEN_ZERO,
                        "along {name} at {t}: the image maps onto the line"
                    );
                } else {
                    let (lo, hi) = (q.x.value.lo(), q.x.value.hi());
                    assert!(
                        lo <= angle && angle <= hi && hi - lo < 1e-14,
                        "along {name} at {t}: [{lo}, {hi}] is the literal {angle}"
                    );
                }
            }
        }
    });
}

/// The radii and the offsets, as multiples of the radius, the `f64`
/// tables read: on the axis, near it, inside, on the wall to a part in
/// `1e9` either side, outside, and far past it.
const RADII: [f64; 4] = [0.7, 1.0e-3, 1.0e3, 0.123_456_789];
const REACHES: [f64; 11] = [
    0.0,
    1.0e-12,
    1.0e-6,
    0.5,
    1.0 - 1.0e-9,
    1.0,
    1.0 + 1.0e-9,
    1.0 + 1.0e-4,
    2.0,
    1.0e3,
    1.0e6,
];

/// A tilted frame for the `f64` tables, and the table's directions
/// about its axis: an even grid offset off the angle window's cut.
fn tilted() -> (Vec3<f64>, Vec3<f64>, Vec3<f64>, Point3<f64>) {
    let axis = Vec3::new(0.1, -0.2, 1.0).normalize();
    let raw = Vec3::new(1.0, 0.0, -0.1);
    let u_ref = (raw - axis * raw.dot(axis)).normalize();
    (axis, u_ref, axis.cross(u_ref), Point3::new(0.4, 0.1, -0.3))
}

fn table_angles() -> impl Iterator<Item = f64> {
    let step = core::f64::consts::TAU / 64.0;
    (0..64).map(move |k| super::ANGLE_WINDOW_START + (f64::from(k) + 0.5) * step)
}

fn iv3(v: Vec3<f64>) -> Vec3<Interval> {
    Vec3::new(
        Interval::from_f64(v.x),
        Interval::from_f64(v.y),
        Interval::from_f64(v.z),
    )
}

fn ivp(p: Point3<f64>) -> Point3<Interval> {
    Point3::new(
        Interval::from_f64(p.x),
        Interval::from_f64(p.y),
        Interval::from_f64(p.z),
    )
}

/// **At `f64` and `Interval` the distance is `|‖w‖ − R|`, to its
/// rounding.** A cylinder's and a sphere's distance at every point of
/// the table against the direct spelling's `Interval` enclosure, which
/// holds the exact distance of the point as given: the `f64` read is
/// within four ulps of the offset's and the radius's sum of it, and
/// the `Interval` read meets it. Within a part in `1e4` of the wall,
/// where incidence is decided, the `Interval` read is as narrow as the
/// direct one (measured: `0.85` to `1.01` of its width); away from the
/// wall its quotient carries the offset's rounding twice, and it is up
/// to five times the direct one's width, a rounding of the offset
/// still.
///
/// Red with [`super::radius_gap`] handing `norm_gap` the radius for its
/// square (run by hand: no site in `mutants`).
#[test]
fn the_distance_is_the_measured_norm_less_the_radius_to_its_rounding() {
    let (axis, u_ref, cv, c) = tilted();
    let mut read = 0_u32;
    for radius in RADII {
        for reach in REACHES {
            for theta in table_angles().step_by(7) {
                let (s, cs) = theta.sin_cos();
                let x = u_ref * cs + cv * s;
                let on_cylinder = c + x * (radius * reach) + axis * 0.37;
                let on_sphere = c + (x * 0.8 + axis * 0.6) * (radius * reach);
                for (name, p, sphere) in [
                    ("cylinder", on_cylinder, false),
                    ("sphere", on_sphere, true),
                ] {
                    let at = |radius_f: f64| -> (f64, Interval, Interval) {
                        let (surface, surface_iv) = if sphere {
                            (
                                Surface::Sphere {
                                    center: c,
                                    radius: radius_f,
                                    axis,
                                    u_ref,
                                },
                                Surface::Sphere {
                                    center: ivp(c),
                                    radius: Interval::from_f64(radius_f),
                                    axis: iv3(axis),
                                    u_ref: iv3(u_ref),
                                },
                            )
                        } else {
                            (
                                Surface::Cylinder {
                                    origin: c,
                                    axis,
                                    radius: radius_f,
                                    u_ref,
                                },
                                Surface::Cylinder {
                                    origin: ivp(c),
                                    axis: iv3(axis),
                                    radius: Interval::from_f64(radius_f),
                                    u_ref: iv3(u_ref),
                                },
                            )
                        };
                        let w = if sphere {
                            ivp(p) - ivp(c)
                        } else {
                            let q = ivp(p) - ivp(c);
                            q - iv3(axis) * q.dot(iv3(axis))
                        };
                        let direct = (w.norm() - Interval::from_f64(radius_f)).abs();
                        (
                            chart_distance(&surface, p),
                            chart_distance(&surface_iv, ivp(p)),
                            direct,
                        )
                    };
                    let (got, got_iv, direct) = at(radius);
                    let scale = radius * (1.0 + reach) + (c - Point3::origin()).norm();
                    let slack = 4.0 * f64::EPSILON * scale;
                    assert!(
                        direct.lo() - slack <= got && got <= direct.hi() + slack,
                        "{name} R = {radius}, reach {reach}, at {theta}: {got} against \
                         [{}, {}]",
                        direct.lo(),
                        direct.hi()
                    );
                    assert!(
                        got_iv.lo() <= direct.hi() && direct.lo() <= got_iv.hi(),
                        "{name} R = {radius}, reach {reach}, at {theta}: the enclosures \
                         [{}, {}] and [{}, {}] hold one distance",
                        got_iv.lo(),
                        got_iv.hi(),
                        direct.lo(),
                        direct.hi()
                    );
                    let (width, direct_width) =
                        (got_iv.hi() - got_iv.lo(), direct.hi() - direct.lo());
                    let allowed = if (reach - 1.0).abs() <= 1.0e-4 {
                        1.05 * direct_width + f64::EPSILON * radius
                    } else {
                        6.0 * direct_width + slack
                    };
                    assert!(
                        width <= allowed,
                        "{name} R = {radius}, reach {reach}, at {theta}: an enclosure {width} \
                         wide against the direct one's {direct_width}"
                    );
                    read += 1;
                }
            }
        }
    }
    assert_eq!(read, 4 * 11 * 10 * 2, "every row of the table read");
}

/// **At `f64` and `Interval` a ruling's direction reads the angle of
/// its offset.** [`ruling_direction`] against the offset's own
/// components at every point of the table, each read in the angle
/// window: on the axis both read `0`; a point from half the radius out
/// reads the same angle to four ulps; and one nearer the axis, where
/// the window's cut side is decided at another scale, reads it modulo a
/// turn, the copy placement's marks then take.
///
/// Red with [`ruling_direction`] dividing one component alone by the
/// radius (run by hand: no site in `mutants`).
#[test]
fn a_rulings_direction_reads_the_angle_of_its_offset_to_its_rounding() {
    let (axis, u_ref, cv, _) = tilted();
    let tau = core::f64::consts::TAU;
    let mut read = 0_u32;
    for radius in RADII {
        for reach in REACHES {
            for theta in table_angles() {
                let (s, cs) = theta.sin_cos();
                let w_r = (u_ref * cs + cv * s) * (radius * reach) + axis * 0.0;
                let got = ruling_direction(w_r, radius, cv, u_ref).principal(band());
                let direct = Azimuth::of(w_r.dot(cv), w_r.dot(u_ref)).principal(band());
                let got_iv =
                    ruling_direction(iv3(w_r), Interval::from_f64(radius), iv3(cv), iv3(u_ref))
                        .principal(band());
                let direct_iv =
                    Azimuth::of(iv3(w_r).dot(iv3(cv)), iv3(w_r).dot(iv3(u_ref))).principal(band());
                let slack = 4.0 * f64::EPSILON * (1.0 + theta.abs());
                let label = format!("R = {radius}, reach {reach}, at {theta}");
                if reach == 0.0 {
                    assert_eq!((got, direct), (0.0, 0.0), "{label}: on the axis");
                    assert_eq!(
                        (got_iv.lo(), got_iv.hi(), direct_iv.lo(), direct_iv.hi()),
                        (0.0, 0.0, 0.0, 0.0),
                        "{label}: on the axis, at Interval"
                    );
                } else {
                    let turns = ((got - direct) / tau).round();
                    assert!(
                        (got - direct - turns * tau).abs() <= slack,
                        "{label}: {got} against {direct}"
                    );
                    let turns_iv = ((got_iv.lo() - direct_iv.lo()) / tau).round();
                    assert!(
                        got_iv.lo() - turns_iv * tau <= direct_iv.hi() + slack
                            && direct_iv.lo() <= got_iv.hi() - turns_iv * tau + slack,
                        "{label}: the enclosures [{}, {}] and [{}, {}] hold one angle",
                        got_iv.lo(),
                        got_iv.hi(),
                        direct_iv.lo(),
                        direct_iv.hi()
                    );
                    if reach >= 0.5 {
                        assert_eq!((turns, turns_iv), (0.0, 0.0), "{label}: one copy");
                    }
                }
                read += 1;
            }
        }
    }
    assert_eq!(read, 4 * 11 * 64, "every row of the table read");
}
