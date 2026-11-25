//! [`super::crossings`] on Bézier and B-spline carriers against a plane,
//! a cylinder and a sphere: clear, a pass within the band refusing, one
//! crossing, three crossings (the first and third of one sense), a
//! tangency, a crossing on a knot, a meeting at a crease, an in-band
//! apex and dip there, a crossing within the band of one and one away
//! from it, an end on the surface, a rational arc, a frame off
//! orthonormal, a degenerate frame, and the divisor's pointwise factor.

#![allow(clippy::unwrap_used, clippy::panic)]

use core::f64::consts::FRAC_1_SQRT_2;

use geom::{NurbsCurve3, Surface};
use geom_core::spline::KnotVector;
use geom_core::{Band, Interval, Point3, Vec3};

use super::{CompositeRefusal, crossings};
use crate::roots::Unsettled;

const ZERO: f64 = 1e-9;

fn band() -> Band {
    Band::new(ZERO, 10.0 * ZERO).unwrap()
}

/// The plane `z = 0`.
fn ground() -> Surface<f64> {
    Surface::Plane {
        frame: geom::Frame::new(Point3::origin(), Vec3::unit_z(), Vec3::unit_x()),
    }
}

/// A non-rational carrier of `degree` on `knots`, its control points
/// `(x, 0, z)`.
fn spline(degree: usize, knots: &[f64], xz: &[(f64, f64)]) -> NurbsCurve3<f64> {
    let kv = KnotVector::clamped(knots.to_vec(), degree).unwrap();
    let control = xz.iter().map(|&(x, z)| Point3::new(x, 0.0, z)).collect();
    NurbsCurve3::new(kv, control, vec![1.0; xz.len()]).unwrap()
}

/// A cubic Bézier over `x ∈ [0, 1]`, its control heights `z`.
fn cubic(z: [f64; 4]) -> NurbsCurve3<f64> {
    let x = [0.0, 1.0 / 3.0, 2.0 / 3.0, 1.0];
    let xz: Vec<(f64, f64)> = x.into_iter().zip(z).collect();
    spline(3, &[0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0], &xz)
}

fn read(net: &NurbsCurve3<f64>, surface: &Surface<f64>) -> Result<Vec<f64>, CompositeRefusal> {
    let (lo, hi) = net.domain();
    crossings(net, (lo, hi), surface, band(), Interval::from_certified).map(|found| found.roots)
}

/// Each crossing reads ON the surface, and the residual changes sign
/// across it; the signs it changes to, in order.
fn senses(net: &NurbsCurve3<f64>, surface: &Surface<f64>, roots: &[f64]) -> Vec<bool> {
    roots
        .iter()
        .map(|&t| {
            let on = crate::implicit_residual(surface, net.eval(t));
            assert!(on.abs() <= ZERO, "t {t}: {on} m off the surface");
            let (before, after) = (
                crate::implicit_residual(surface, net.eval(t - 1e-6)),
                crate::implicit_residual(surface, net.eval(t + 1e-6)),
            );
            assert!(
                before * after < 0.0,
                "t {t}: no sign change ({before}, {after})"
            );
            after > 0.0
        })
        .collect()
}

#[test]
fn a_carrier_above_the_plane_is_clear() {
    assert_eq!(
        read(&cubic([1.0, 2.0, 2.0, 1.0]), &ground()).unwrap(),
        vec![]
    );
    // Within the band of the plane at both ends only: those are the end
    // vertices' meetings, which the caller reads.
    assert_eq!(
        read(&cubic([1e-10, 1.0, 1.0, 1e-10]), &ground()).unwrap(),
        vec![]
    );
}

/// `z = (1 − 2t)² + 10⁻¹⁰`: above the plane everywhere, but within the
/// band of it about `t = ½`, inside the span. Not definitely apart there,
/// so not clear: it refuses rather than read a near pass as a miss.
#[test]
fn an_in_band_pass_inside_the_span_refuses() {
    let lift = 1e-10;
    let net = spline(
        2,
        &[0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
        &[(0.0, 1.0 + lift), (0.5, -1.0 + lift), (1.0, 1.0 + lift)],
    );
    let got = read(&net, &ground());
    assert!(
        matches!(got, Err(CompositeRefusal::Escalated(_))),
        "{got:?}"
    );
}

#[test]
fn a_carrier_rising_through_the_plane_crosses_once() {
    let net = cubic([-1.0, -0.5, 0.5, 1.0]);
    let roots = read(&net, &ground()).unwrap();
    assert_eq!(roots.len(), 1, "{roots:?}");
    assert!((roots[0] - 0.5).abs() < 1e-12, "{roots:?}");
    assert_eq!(senses(&net, &ground(), &roots), vec![true]);
}

/// Up, down and up again: three crossings, ascending, the first and the
/// third of one sense.
#[test]
fn three_crossings_come_in_order_two_of_one_sense() {
    let net = cubic([-1.0, 3.0, -3.0, 1.0]);
    let roots = read(&net, &ground()).unwrap();
    assert_eq!(roots.len(), 3, "{roots:?}");
    assert!(
        roots.windows(2).all(|w| w[0] < w[1]),
        "ascending: {roots:?}"
    );
    assert_eq!(senses(&net, &ground(), &roots), vec![true, false, true]);
}

/// `z = (1 − 2t)²` touches the plane at `t = ½` without crossing it.
#[test]
fn a_tangency_refuses() {
    let net = spline(
        2,
        &[0.0, 0.0, 0.0, 1.0, 1.0, 1.0],
        &[(0.0, 1.0), (0.5, -1.0), (1.0, 1.0)],
    );
    let got = read(&net, &ground());
    assert!(
        matches!(got, Err(CompositeRefusal::Escalated(_))),
        "{got:?}"
    );
}

/// A cubic B-spline whose controls are antisymmetric about its one
/// interior knot crosses the plane on that knot, where both pieces' rows
/// hold the zero at their shared end.
#[test]
fn a_crossing_on_a_knot_is_the_knots() {
    let net = spline(
        3,
        &[0.0, 0.0, 0.0, 0.0, 0.5, 1.0, 1.0, 1.0, 1.0],
        &[
            (0.0, -1.0),
            (0.25, -1.0),
            (0.5, 0.0),
            (0.75, 1.0),
            (1.0, 1.0),
        ],
    );
    let roots = read(&net, &ground()).unwrap();
    assert_eq!(roots, vec![0.5]);
    assert_eq!(senses(&net, &ground(), &roots), vec![true]);
}

/// A quadratic with a double interior knot: a crease at `t = ½`, where
/// the carrier turns from along `x` to along `y` and passes through
/// `z = 0` on the turn. A meeting there refuses; the plane `z = ½`, met
/// halfway along the second piece, is an ordinary crossing.
#[test]
fn a_meeting_at_a_crease_refuses_and_one_off_it_crosses() {
    let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.5, 0.5, 1.0, 1.0, 1.0], 2).unwrap();
    let control = vec![
        Point3::new(0.0, 0.0, -1.0),
        Point3::new(0.5, 0.0, -0.5),
        Point3::new(1.0, 0.0, 0.0),
        Point3::new(1.0, 0.5, 0.5),
        Point3::new(1.0, 1.0, 1.0),
    ];
    let net = NurbsCurve3::new(kv, control, vec![1.0; 5]).unwrap();
    let got = read(&net, &ground());
    assert!(
        matches!(got, Err(CompositeRefusal::Unsettled(Unsettled::Crease))),
        "{got:?}"
    );
    let half = Surface::Plane {
        frame: geom::Frame::new(Point3::new(0.0, 0.0, 0.5), Vec3::unit_z(), Vec3::unit_x()),
    };
    let roots = read(&net, &half).unwrap();
    assert_eq!(roots.len(), 1, "{roots:?}");
    assert!((roots[0] - 0.75).abs() < 1e-12, "{roots:?}");
}

/// A carrier leaving the plane from its start: the end is the end
/// vertex's meeting, not a crossing; dipping under first, it crosses once
/// more, inside.
#[test]
fn an_end_on_the_plane_is_not_a_crossing() {
    assert_eq!(
        read(&cubic([0.0, 1.0, 1.0, 1.0]), &ground()).unwrap(),
        vec![]
    );
    let net = cubic([0.0, -1.0, 1.0, 1.0]);
    let roots = read(&net, &ground()).unwrap();
    assert_eq!(roots.len(), 1, "{roots:?}");
    assert!(roots[0] > 0.1, "{roots:?}");
    assert_eq!(senses(&net, &ground(), &roots), vec![true]);
}

/// The unit quarter circle as a rational quadratic, against the plane
/// `x = y` (at 45°) and the unit cylinder about `x = 1` (at 60°).
#[test]
fn a_rational_arc_crosses_a_plane_and_a_cylinder() {
    let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
    let control = vec![
        Point3::new(1.0, 0.0, 0.0),
        Point3::new(1.0, 1.0, 0.0),
        Point3::new(0.0, 1.0, 0.0),
    ];
    let arc = NurbsCurve3::new(kv, control, vec![1.0, FRAC_1_SQRT_2, 1.0]).unwrap();
    let diagonal = Surface::Plane {
        frame: geom::Frame::new(
            Point3::origin(),
            Vec3::new(FRAC_1_SQRT_2, -FRAC_1_SQRT_2, 0.0),
            Vec3::unit_z(),
        ),
    };
    let roots = read(&arc, &diagonal).unwrap();
    assert_eq!(roots.len(), 1, "{roots:?}");
    let p = arc.eval(roots[0]);
    assert!(
        (p.x - FRAC_1_SQRT_2).abs() < 1e-9 && (p.y - FRAC_1_SQRT_2).abs() < 1e-9,
        "{p:?}"
    );
    let wall = Surface::Cylinder {
        frame: geom::Frame::new(Point3::new(1.0, 0.0, 0.0), Vec3::unit_z(), Vec3::unit_x()),
        radius: 1.0,
    };
    let roots = read(&arc, &wall).unwrap();
    assert_eq!(roots.len(), 1, "{roots:?}");
    let p = arc.eval(roots[0]);
    assert!(
        (p.x - 0.5).abs() < 1e-9 && (p.y - 0.75f64.sqrt()).abs() < 1e-9,
        "{p:?}"
    );
    // The arc lies on the unit cylinder about the origin: the composite is
    // zero to rounding everywhere, which no cut reads a sign of.
    let own = Surface::Cylinder {
        frame: geom::Frame::new(Point3::origin(), Vec3::unit_z(), Vec3::unit_x()),
        radius: 1.0,
    };
    let got = read(&arc, &own);
    assert!(
        matches!(got, Err(CompositeRefusal::Unsettled(Unsettled::Tangency))),
        "{got:?}"
    );
}

/// A non-rational cubic from inside a sphere to outside it, read through
/// the certified lane's door, and only over the part of its domain it is
/// given: the crossing outside that part is not the span's.
#[test]
fn a_cubic_leaves_a_sphere_through_the_door() {
    let net = cubic([0.0, 0.0, 0.0, 0.0]);
    let ball = Surface::Sphere {
        frame: geom::Frame::new(Point3::origin(), Vec3::unit_z(), Vec3::unit_x()),
        radius: 0.5,
    };
    let lane = crate::NurbsLane::<f64>::certified();
    let roots = lane
        .surface_crossings(&net, (0.0, 1.0), &ball, band())
        .unwrap()
        .roots;
    assert_eq!(roots.len(), 1, "{roots:?}");
    assert!((net.eval(roots[0]).x - 0.5).abs() < 1e-9, "{roots:?}");
    let short = lane
        .surface_crossings(&net, (0.0, 0.25), &ball, band())
        .unwrap()
        .roots;
    assert_eq!(short, vec![]);
    let from_it = lane
        .surface_crossings(&net, (0.5, 1.0), &ball, band())
        .unwrap();
    assert_eq!(
        from_it.roots,
        vec![],
        "the span's start on the sphere is its end vertex's"
    );
    assert!(
        matches!(from_it.ends, [crate::curve_face::EndReading::Within { until }, crate::curve_face::EndReading::Past(_)] if 0.5 < until && until < 1.0),
        "the start reads near, and its stretch stops short of the stop: {:?}",
        from_it.ends
    );
}

/// A V at a crease whose apex stands above the plane by less than the
/// zero band: not definitely apart, so not clear, and the meeting is the
/// crease's.
#[test]
fn an_in_band_apex_at_a_crease_is_not_read_clear() {
    let got = read(&crease_v(1e-12), &ground());
    assert!(
        matches!(got, Err(CompositeRefusal::Unsettled(Unsettled::Crease))),
        "{got:?}"
    );
}

/// The same V, its apex just below the plane: two crossings, each
/// within the band of the crease.
#[test]
fn an_in_band_dip_at_a_crease_refuses_crease() {
    let got = read(&crease_v(-1e-12), &ground());
    assert!(
        matches!(got, Err(CompositeRefusal::Unsettled(Unsettled::Crease))),
        "{got:?}"
    );
}

/// A crossing within the band of a crease, not on it: the crease row's
/// plane lifted by `10⁻¹¹`, so the crossing sits on the second piece a
/// hair past the turn. It refuses as the crease's, read in metres from
/// the crease's point, not by the knot's value.
#[test]
fn a_crossing_within_band_of_a_crease_refuses() {
    let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.5, 0.5, 1.0, 1.0, 1.0], 2).unwrap();
    let control = vec![
        Point3::new(0.0, 0.0, -1.0),
        Point3::new(0.5, 0.0, -0.5),
        Point3::new(1.0, 0.0, 0.0),
        Point3::new(1.0, 0.5, 0.5),
        Point3::new(1.0, 1.0, 1.0),
    ];
    let net = NurbsCurve3::new(kv, control, vec![1.0; 5]).unwrap();
    let lifted = Surface::Plane {
        frame: geom::Frame::new(Point3::new(0.0, 0.0, 1e-11), Vec3::unit_z(), Vec3::unit_x()),
    };
    let got = read(&net, &lifted);
    assert!(
        matches!(got, Err(CompositeRefusal::Unsettled(Unsettled::Crease))),
        "{got:?}"
    );
}

/// A quadratic V with a crease at `t = ½`, its apex at `(1, 0, apex)`.
fn crease_v(apex: f64) -> NurbsCurve3<f64> {
    let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.5, 0.5, 1.0, 1.0, 1.0], 2).unwrap();
    let control = vec![
        Point3::new(0.0, 0.0, 1.0),
        Point3::new(0.5, 0.0, 0.5),
        Point3::new(1.0, 0.0, apex),
        Point3::new(1.5, 0.0, 0.5),
        Point3::new(2.0, 0.0, 1.0),
    ];
    NurbsCurve3::new(kv, control, vec![1.0; 5]).unwrap()
}

/// **The chart frame is formed from the axis, not read as stored.** The
/// radial terms take the frame as orthonormal. A reference direction off
/// square to a cylinder's axis, or a sphere's axis off unit length, by
/// far more than rounding so the effect stands above the band: a run
/// clear of each surface reads clear, where the stored frame would have
/// tilted or stretched it through the surface.
#[test]
fn a_frame_off_orthonormal_reads_the_surface_it_names() {
    let line = |from: Point3<f64>, to: Point3<f64>| {
        let kv = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
        NurbsCurve3::new(kv, vec![from, to], vec![1.0; 2]).unwrap()
    };
    // Outside the unit cylinder at radius 1.05, along its axis.
    let wall = Surface::Cylinder {
        frame: geom::Frame::new(Point3::origin(), Vec3::unit_z(), Vec3::new(1.0, 0.0, 0.1)),
        radius: 1.0,
    };
    let along = line(Point3::new(1.05, 0.0, -1.0), Point3::new(1.05, 0.0, 1.0));
    assert_eq!(read(&along, &wall).unwrap(), vec![], "outside the wall");
    // Inside the unit sphere, up its axis to 0.95.
    let ball = Surface::Sphere {
        frame: geom::Frame::new(Point3::origin(), Vec3::new(0.0, 0.0, 1.1), Vec3::unit_x()),
        radius: 1.0,
    };
    let up = line(Point3::new(0.0, 0.0, 0.5), Point3::new(0.0, 0.0, 0.95));
    assert_eq!(read(&up, &ball).unwrap(), vec![], "inside the ball");
}

/// **The metering divisor is a factor of the form at each point**, not a
/// bound along a path to the surface: `|f(p)|` is at most `d(p)` times
/// the divisor at `p` itself, `d` the distance from the form's zero set. So the net's hull, which holds every point of the span, is
/// the region the divisor is read over, and a path from a point to its
/// foot that leaves the hull charges nothing. Read on a grid of points
/// in and well past each surface's band, against the closed-form
/// distances.
#[test]
fn the_divisor_is_a_pointwise_factor_of_the_form() {
    use crate::pcurve_cache::projected::Meters;
    let (r_cyl, r_ball, (big, small), alpha) = (1.0f64, 1.5f64, (2.0f64, 0.5f64), 0.6f64);
    let (sin, cos) = alpha.sin_cos();
    // The form `f` and the distance `d` at a point `(ρ, z)` of a meridian
    // plane.
    type Reading<'a> = &'a dyn Fn(f64, f64) -> (f64, f64);
    let kinds: [(&str, Meters<f64>, Reading); 4] = [
        ("cylinder", Meters::Cylinder { radius: r_cyl }, &|rho, _| {
            (rho * rho - r_cyl * r_cyl, (rho - r_cyl).abs())
        }),
        ("sphere", Meters::Sphere { radius: r_ball }, &|rho, z| {
            let n = rho.hypot(z);
            (n * n - r_ball * r_ball, (n - r_ball).abs())
        }),
        (
            "cone",
            Meters::Cone {
                sin,
                cos,
                locus: crate::pcurve_cache::projected::Locus::Carrier,
            },
            &|rho, z| {
                let (one, other) = (rho * cos - z * sin, rho * cos + z * sin);
                (one * other, one.abs().min(other.abs()))
            },
        ),
        (
            "torus",
            Meters::Torus {
                major: big,
                minor: small,
            },
            &|rho, z| {
                let (near, far) = ((rho - big).hypot(z), (rho + big).hypot(z));
                (
                    (near * near - small * small) * (far * far - small * small),
                    (near - small).abs().min((far - small).abs()),
                )
            },
        ),
    ];
    for (name, meters, form) in kinds {
        for i in 0..=40 {
            for j in 0..=40 {
                let (rho, z) = (f64::from(i) * 0.1, f64::from(j) * 0.1 - 2.0);
                let (f, d) = form(rho, z);
                let divisor = meters.divisor(Vec3::new(rho, 0.0, z));
                assert!(
                    f.abs() <= d * divisor * (1.0 + 1e-12) + 1e-12,
                    "{name} at (ρ {rho}, z {z}): |f| {} > d {d} · divisor {divisor}",
                    f.abs()
                );
            }
        }
    }
}

/// **A face whose frame describes no chart is refused as degenerate**,
/// not as an arm that could not form: a cylinder whose reference
/// direction runs along its axis has no radial plane to read.
#[test]
fn a_reference_direction_along_the_axis_is_a_degenerate_face() {
    let along = Surface::Cylinder {
        frame: geom::Frame::new(Point3::origin(), Vec3::unit_z(), Vec3::unit_z()),
        radius: 1.0,
    };
    let got = read(&cubic([-1.0, -0.5, 0.5, 1.0]), &along);
    assert!(
        matches!(got, Err(CompositeRefusal::FaceDegenerate(_))),
        "{got:?}"
    );
}

/// The rising line `z = 2t − 1` as a cubic over `[0, 1]`, at `Interval`,
/// each control height widened by `w` on either side: a family of
/// carriers whose crossings with `z = 0` spread over `w` about `t = ½`.
fn rising_family(w: f64) -> NurbsCurve3<Interval> {
    use geom_core::Real as _;
    let net = cubic([-1.0, -1.0 / 3.0, 1.0 / 3.0, 1.0]);
    let control = net
        .control()
        .iter()
        .map(|p| {
            Point3::new(
                Interval::from_f64(p.x),
                Interval::zero(),
                Interval::from_bounds(p.z - w, p.z + w),
            )
        })
        .collect();
    let knots = net.knot_values().lift(Interval::from_f64);
    NurbsCurve3::new(knots, control, vec![Interval::one(); 4]).unwrap()
}

/// `settle`'s stored crossings through the door, at a scalar the arm
/// runs at: the candidate an `f64` search proposed, read against the
/// plane at `Interval`.
fn settled(net: &NurbsCurve3<Interval>, candidate: f64) -> Result<Vec<Interval>, CompositeRefusal> {
    use geom_core::{Margin, Real as _};
    let plane = Surface::Plane {
        frame: geom::Frame::new(Point3::origin(), Vec3::unit_z(), Vec3::unit_x()),
    };
    let off = |p: Point3<Interval>| Margin::of(crate::implicit_residual(&plane, p));
    super::settle(
        net,
        (Interval::zero(), Interval::one()),
        band(),
        &[],
        vec![super::Meeting::Crossing(candidate)],
        &off,
    )
    .map(|(roots, _)| roots)
}

/// The door admits a crossing whose point reads on for the whole
/// family, as the one point the search proposed; refuses one whose
/// point reads definitely off by its row; and refuses at the door,
/// typed and named, a family whose crossing spreads past the band
/// about the candidate.
#[test]
fn a_stored_crossing_is_admitted_by_its_check_at_interval() {
    use geom_core::Bounds as _;
    let point = settled(&rising_family(0.0), 0.5);
    assert!(
        matches!(point.as_deref(), Ok([t]) if t.lo() == 0.5 && t.hi() == 0.5),
        "a point family's crossing is the candidate, one point: {point:?}"
    );
    let off = settled(&rising_family(0.0), 0.25);
    assert!(
        matches!(
            off,
            Err(CompositeRefusal::Unsettled(Unsettled::Meter {
                row: "spline_composite_root_on"
            }))
        ),
        "a candidate whose point reads definitely off the plane refuses by the door's row: {off:?}"
    );
    // Between the band's zero and its escalation: a door deciding
    // against a looser band would admit this family.
    let spread = settled(&rising_family(3.0 * ZERO), 0.5);
    assert!(
        matches!(spread, Err(CompositeRefusal::Escalated(d)) if d.predicate == Some("spline_composite_root_on")),
        "a family spread 3·zero about the candidate refuses at the door, named: {spread:?}"
    );
    let wide = settled(&rising_family(1e-6), 0.5);
    assert!(
        matches!(wide, Err(CompositeRefusal::Escalated(d)) if d.predicate == Some("spline_composite_root_on")),
        "a family whose crossing spreads past the band refuses at the door, named: {wide:?}"
    );
}

/// **The check bounds the crossing across the plane; its readers decide
/// along the carrier.** This row states the finding of
/// `work/scalar/a-crossing-parameters-check-bounds-across-the-face-and-its-readers-decide-along-the-edge`
/// as it stands, and goes red when that item changes what `settle` does
/// with it. On the shallow line `z = 10⁻³·(t − t*)`, `t* = 1 − 2·10⁻⁷`,
/// three candidates each read on the plane within the band's zero, and
/// `settle` answers each differently by where its point stands against
/// the span's end: reported, escalated, dropped as the end's.
#[test]
fn admissible_candidates_of_one_shallow_crossing_settle_three_ways() {
    use geom_core::Margin;
    let root = 1.0 - 2e-7;
    let z = |t: f64| 1e-3 * (t - root);
    let net = cubic([z(0.0), z(1.0 / 3.0), z(2.0 / 3.0), z(1.0)]);
    let plane = ground();
    let off = |p: Point3<f64>| Margin::of(crate::implicit_residual(&plane, p));
    let at = |candidate: f64| {
        assert!(z(candidate).abs() <= ZERO, "{candidate} reads on the plane");
        super::settle(
            &net,
            (0.0, 1.0),
            band(),
            &[],
            vec![super::Meeting::Crossing(candidate)],
            &off,
        )
        .map(|(roots, _)| roots)
    };
    let reported = at(root);
    assert!(
        matches!(reported.as_deref(), Ok([t]) if *t == root),
        "a candidate past the escalation from the end is reported: {reported:?}"
    );
    let escalated = at(1.0 - 5e-9);
    assert!(
        matches!(escalated, Err(CompositeRefusal::Escalated(d)) if d.predicate == Some("spline_composite_root_at_end")),
        "a candidate in the end's escalation refuses there: {escalated:?}"
    );
    let dropped = at(1.0 - 5e-10);
    assert!(
        matches!(dropped.as_deref(), Ok([])),
        "a candidate within the end's zero is the end's: {dropped:?}"
    );
}
