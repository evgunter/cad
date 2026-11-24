//! **A point built on a cylinder or a sphere at a parameter radius,
//! along the chart's own frame, reads its chart with no sign asked of
//! the radius.** A point at `c + r·x`, `x` a unit direction of an
//! exactly orthonormal frame, is on the chart of radius `r` at every
//! `r`; over a box on `r` its distance from the chart
//! ([`chart_distance`]) is the zero form, and a ruling through it is
//! placed at `x`'s own angle ([`chart_pcurve`]). On a frame whose axes
//! are unit only to rounding (normalized `f64` directions) the distance
//! is not the zero form; that is pinned as it stands. The same readers
//! at `f64` and `Interval` are held to the direct spellings,
//! `|‖w‖ − R|` and the angle of the offset at its length, on a table
//! that runs from the axis to far past the wall.
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
                    frame: geom::Frame::new(point(c), lit3(axis), lit3(u_ref)),
                    radius,
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
                    frame: geom::Frame::new(point(c), lit3(axis), lit3(u_ref)),
                    radius,
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
                frame: geom::Frame::new(point(c), lit3(axis), lit3(u_ref)),
                radius: r,
            };
            let line = geom::Line {
                origin: point(c) + lit3(x) * r,
                dir: geom_core::UnitVec3::squared(lit3(axis)),
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
                            (cylinder.eval(q.x, q.y) - (line.origin + line.dir.get() * lit(t)))
                                .norm()
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
/// the `Interval` read meets it. Against the direct spelling on the
/// same operands (the offset about the frame's own axis) the `Interval`
/// read is as narrow within a part in `1e4` of the wall, where
/// incidence is decided (measured: `0.77` to `1.01` of its width); away
/// from the wall its quotient carries the offset's rounding twice, and
/// it is up to five times the direct one's width, a rounding of the
/// offset still.
///
/// Red with [`super::wall_gap`] handing the gap the radius's
/// square for the radius (run by hand: no site in `mutants`).
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
                                    frame: geom::Frame::new(c, axis, u_ref),
                                    radius: radius_f,
                                },
                                Surface::Sphere {
                                    frame: geom::Frame::new(ivp(c), iv3(axis), iv3(u_ref)),
                                    radius: Interval::from_f64(radius_f),
                                },
                            )
                        } else {
                            (
                                Surface::Cylinder {
                                    frame: geom::Frame::new(c, axis, u_ref),
                                    radius: radius_f,
                                },
                                Surface::Cylinder {
                                    frame: geom::Frame::new(ivp(c), iv3(axis), iv3(u_ref)),
                                    radius: Interval::from_f64(radius_f),
                                },
                            )
                        };
                        // The direct spelling on the reader's own operands:
                        // the frame's axis as the frame holds it.
                        let w = if sphere {
                            ivp(p) - ivp(c)
                        } else {
                            let held = geom::Frame::new(ivp(c), iv3(axis), iv3(u_ref)).axis();
                            let q = ivp(p) - ivp(c);
                            q - held * q.dot(held)
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
    let (_, u_ref, cv, _) = tilted();
    let tau = core::f64::consts::TAU;
    let mut read = 0_u32;
    for radius in RADII {
        for reach in REACHES {
            for theta in table_angles() {
                let (s, cs) = theta.sin_cos();
                let w_r = (u_ref * cs + cv * s) * (radius * reach);
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

/// **Against a radius that is not positive no point reads on.** A
/// radius is positive wherever its surface is at rest (check 1's
/// `radius > 0`, `Surface`'s representability margin); this is what the
/// reader does if one that is not reaches it all the same. Against a
/// cylinder or a sphere of radius `−R` every point of the table off the
/// mirror wall reads `‖w‖ + R`, as the direct spelling does: the gap's
/// `abs` is taken of the whole quotient, whose denominator `‖w‖ − R`
/// changes sign across that wall. On the mirror wall the gap's one
/// precondition fails: the read is poison, which a decision refuses, or
/// a finite quotient of two roundings, which decides off; it never
/// decides on.
///
/// Red with [`super::wall_gap`] taking the `abs` of the numerator
/// alone (run by hand: no site in `mutants`): a point inside the mirror
/// wall reads a negative distance, which decides as on the chart.
#[test]
fn a_radius_that_is_not_positive_reads_no_point_on_it() {
    let (axis, u_ref, cv, c) = tilted();
    let mut read = 0_u32;
    for radius in RADII {
        for reach in REACHES {
            for theta in table_angles().step_by(7) {
                let (s, cs) = theta.sin_cos();
                let x = u_ref * cs + cv * s;
                let frame = geom::Frame::new(c, axis, u_ref);
                let cases = [
                    (
                        "cylinder",
                        Surface::Cylinder {
                            frame,
                            radius: -radius,
                        },
                        c + x * (radius * reach) + axis * 0.37,
                    ),
                    (
                        "sphere",
                        Surface::Sphere {
                            frame,
                            radius: -radius,
                        },
                        c + (x * 0.8 + axis * 0.6) * (radius * reach),
                    ),
                ];
                for (name, surface, p) in cases {
                    let got = chart_distance(&surface, p);
                    let want = radius * (reach + 1.0);
                    let label = format!("{name} R = -{radius}, reach {reach}, at {theta}");
                    // On the mirror wall the gap's precondition fails
                    // (`‖w‖ + radius = 0`): the quotient is its own
                    // rounding, poison or a finite value of no meaning,
                    // and the one thing that holds is that it is never
                    // read as on.
                    if reach == 1.0 {
                        let on = decide_reported("radial_reader_row", Margin::of(got), band())
                            .is_ok_and(|d| d.sign != Sign::Positive);
                        assert!(!on, "{label}: {got} on the mirror wall reads on");
                    } else {
                        assert!(
                            (got - want).abs() <= 1.0e-6 * want,
                            "{label}: {got} against {want}"
                        );
                    }
                    read += 1;
                }
            }
        }
    }
    assert_eq!(read, 4 * 11 * 10 * 2, "every row of the table read");
}

/// **On a tilted frame whose axes are exactly orthonormal the point is
/// on the wall as a theorem too.** The frame `(0.6, 0, 0.8)`,
/// `(0, 1, 0)`: the chart's own `eval` at a literal angle, on the
/// cylinder and the sphere, and `c + r·u_ref + h·axis` on the frame's
/// own axes, each against the radius `r` over a box on it.
///
/// Red under `PCERT_RADIAL_PAIR_MUTANT=distance-raw-norm`.
#[test]
fn a_point_built_on_an_exactly_orthonormal_tilt_is_on_it_over_a_box() {
    let c = Point3::new(0.5, -0.25, 2.0);
    with_session(budget(), || {
        let r = boxed("radial_reader_radius", RADIUS);
        let frame = geom::Frame::new(
            point(c),
            lit3(Vec3::new(0.6, 0.0, 0.8)),
            lit3(Vec3::new(0.0, 1.0, 0.0)),
        );
        let cylinder = Surface::Cylinder { frame, radius: r };
        let sphere = Surface::Sphere { frame, radius: r };
        let built = frame.origin() + frame.u_ref() * r + frame.axis() * lit(0.9);
        for (name, d) in [
            (
                "the cylinder's eval",
                chart_distance(&cylinder, cylinder.eval(lit(0.7), lit(0.4))),
            ),
            (
                "the sphere's eval",
                chart_distance(&sphere, sphere.eval(lit(0.7), lit(0.3))),
            ),
            ("c + r·u_ref + h·axis", chart_distance(&cylinder, built)),
        ] {
            assert_eq!(reading(d), PROVEN_ZERO, "{name}");
        }
    });
}

/// **On a frame made of normalized `f64` directions the point is NOT
/// proven on the wall: today's truth, pinned**
/// (`work/boxfinish/a-point-built-on-a-frame-of-normalized-directions-does-not-fold-onto-its-wall`).
/// Such axes are unit only to rounding, so `w·w − r²` keeps a residue
/// of the radius's box and the reading is its enclosure. Over a box of
/// half-width `0.1` it is undecided, `[0, 0.32]` where the direct
/// spelling's is `[0, 0.21]`; from a half-width of `1e-3` down the two
/// spellings' enclosures agree to four digits (`2.06e-6` at `1e-6`),
/// and at a quarter of the band's zero (`2.5e-10` at the default ε) it
/// reads Zero, unproven, as the direct spelling does.
#[test]
fn a_point_built_on_a_normalized_tilt_is_not_proven_on_it_over_a_box() {
    let (axis, u_ref, _, c) = tilted();
    let at = |half: f64| {
        with_session(budget(), || {
            let r = boxed("radial_reader_radius", (0.3 - half, 0.3 + half));
            let frame = geom::Frame::new(point(c), lit3(axis), lit3(u_ref));
            let cylinder = Surface::Cylinder { frame, radius: r };
            let d = chart_distance(&cylinder, cylinder.eval(lit(0.7), lit(0.4)));
            let read = decide_reported("radial_reader_row", Margin::of(d), band())
                .ok()
                .map(|g| (g.sign, g.margin.proven()));
            (read, d.value.hi())
        })
        .0
    };
    let (wide, hi) = at(0.1);
    assert!(
        wide.is_none() && (0.30..0.33).contains(&hi),
        "a wide box: undecided at [0, 0.32], read {wide:?} at {hi}"
    );
    let (mid, hi) = at(1.0e-6);
    assert!(
        mid.is_none() && (2.0e-6..2.1e-6).contains(&hi),
        "a box of half-width 1e-6: undecided at the box's own width, read {mid:?} at {hi}"
    );
    let (narrow, hi) = at(0.25 * band().zero());
    assert_eq!(
        narrow,
        Some((Sign::Zero, false)),
        "a box inside the band: Zero, and not a theorem ({hi})"
    );
}

/// **At `Dual` the gap's tangent is the difference's**: along a
/// direction that moves the offset and the radius together, on the
/// wall, inside it, outside it and a part in `1e9` off it, the
/// quotient's derivative is `d‖w‖ − dR` to a part in `1e6`.
///
/// Red with [`geom_core::radius_gap`]'s denominator spelled
/// `norm − radius` (run by hand).
#[test]
fn the_gaps_tangent_is_the_differences() {
    use geom_core::Dual;
    for (radius, reach, dr, dw) in [
        (0.7, 1.0, 0.3, 1.1),
        (0.7, 0.5, -0.2, 0.4),
        (1.0e3, 1.0 + 1.0e-9, 1.0, -2.0),
        (1.0e-3, 2.0, 0.0, 1.0),
    ] {
        let r = Dual::new(radius, dr);
        let x = Vec3::new(0.6, 0.8, 0.0);
        let w = Vec3::new(
            Dual::new(x.x * radius * reach, x.x * dw),
            Dual::new(x.y * radius * reach, x.y * dw),
            Dual::new(0.0, 0.0),
        );
        let (gap, direct) = (w.radius_gap(r), w.norm() - r);
        assert!(
            (gap.deriv - direct.deriv).abs() <= 1.0e-6 * (1.0 + direct.deriv.abs()),
            "R = {radius}, reach {reach}: the gap's tangent {} against {}",
            gap.deriv,
            direct.deriv
        );
    }
}

/// **The wall an extruded circle leaves: its seam's ruling and its rims
/// are placed at the zero form, with the radius stored as a magnitude.**
/// The wall's `u_ref` is the start vertex's own radial over its length
/// (`r·x/√(r²)`) and its radius is `|r|`; the seam is the ruling through
/// the vertex `c + r·x`, and a rim is the circle of radius `|r|` on the
/// same frame. The ruling's direction is its offset over the radius,
/// `r²/(√(r²)·|r|)` along `u_ref`, which is `1` in the tier; the rim's
/// is its axis vector at its length, `r²/√(r²)` along `u_ref`,
/// non-negative by its syntax. Either way the phase is the zero form.
///
/// This pins the shape the plate certifies in. With the radius a
/// magnitude the readers' direct spellings fold too, so it is green
/// under both env mutants; it is red with [`super::ruling_direction`]
/// dividing by the radius's negation (run by hand), which places the
/// seam half a turn off.
#[test]
fn an_extruded_circles_wall_places_its_seam_and_rims_at_zero() {
    let (axis, _, _, c) = frame();
    let x = Vec3::new(0.0, 1.0, 0.0);
    with_session(budget(), || {
        let r = boxed("radial_reader_radius", RADIUS);
        let start = point(c) + lit3(x) * r;
        let wall = geom::Frame::new(point(c), lit3(axis), start - point(c));
        let cylinder = Surface::Cylinder {
            frame: wall,
            radius: r.abs(),
        };
        let seam = Curve3::Line(geom::Line {
            origin: start,
            dir: geom_core::UnitVec3::squared(lit3(axis)),
        });
        let rim = Curve3::Circle(geom::Circle::new(wall, r.abs()));
        for (name, carrier) in [("the seam", seam), ("a rim", rim)] {
            let image = chart_pcurve(&carrier, &cylinder, band())
                .unwrap_or_else(|e| panic!("{name}: no image: {e:?}"));
            assert_eq!(
                reading(image.eval(S::zero()).x),
                PROVEN_ZERO,
                "{name}: placed at the zero form"
            );
        }
    });
}
