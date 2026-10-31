//! **The regularity sweep's rows**: the probe's pairs it proves
//! loop-free, which the guard clears with no whole reading, and the
//! poses it declines, which fall to C3's whole reading. Every row names
//! the mutant that turns it red; each runs at the run's ε (the suite
//! runs 1e-9, 1e-6 and 1e-12).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::{NurbsSurface, Surface};
use geom_core::spline::KnotVector;
use geom_core::{Band, Point3, Tol, Vec3};

use super::{Declined, Sweep};
use crate::pair_section::{ChartReach, Components, SectionRead, SsiLane, pair_section};
use crate::ssi::{ChartAxis, ChartEnd, ChartSide};

fn band() -> Band {
    Band::linear(Tol::witness()).expect("the run's tolerance forms a band")
}

fn pt(x: f64, y: f64, z: f64) -> Point3<f64> {
    Point3::new(x, y, z)
}

/// The plane through `origin` square to `normal`.
fn plane(origin: Point3<f64>, normal: Vec3<f64>) -> Surface<f64> {
    let n = normal * (1.0 / normal.norm());
    let pick = if n.x.abs() < 0.9 {
        Vec3::unit_x()
    } else {
        Vec3::unit_y()
    };
    let u = pick - n * pick.dot(n);
    Surface::Plane {
        origin,
        normal: n,
        u_ref: u * (1.0 / u.norm()),
    }
}

fn level(h: f64) -> Surface<f64> {
    plane(pt(0.5, 0.5, h), Vec3::unit_z())
}

fn wall(n: NurbsSurface<f64>) -> Surface<f64> {
    Surface::Nurbs(std::sync::Arc::new(n))
}

/// A wall over `[0, 1]²`, of degree one less than its rows in `u` and
/// ruled in `v` from the `bottom` row (`v = 0`) to the `top`.
fn ruled(bottom: &[Point3<f64>], top: &[Point3<f64>]) -> Surface<f64> {
    let p = bottom.len() - 1;
    let mut ku = vec![0.0; p + 1];
    ku.extend(vec![1.0; p + 1]);
    let ku = KnotVector::clamped(ku, p).unwrap();
    let kv = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    let control: Vec<_> = bottom.iter().zip(top).flat_map(|(b, t)| [*b, *t]).collect();
    let count = control.len();
    wall(NurbsSurface::new(ku, kv, control, vec![1.0; count]).unwrap())
}

/// The planar wall in `y = 0` whose bottom side is the quadratic with
/// heights `z` at its three controls, its top side at `z = 1`.
fn bowed(z: [f64; 3]) -> Surface<f64> {
    ruled(
        &[pt(0.0, 0.0, z[0]), pt(0.5, 0.0, z[1]), pt(1.0, 0.0, z[2])],
        &[pt(0.0, 0.0, 1.0), pt(0.5, 0.0, 1.0), pt(1.0, 0.0, 1.0)],
    )
}

/// The S-shaped wall of the probe, `s` metres across about `o`: a cubic
/// side inflected twice, ruled upward; `twisted` mirrors its top row's
/// S and tilts its rows.
fn inflected(o: Point3<f64>, s: f64, twisted: bool) -> Surface<f64> {
    let at = |x: f64, y: f64, z: f64| pt(o.x + s * x, o.y + s * y, o.z + s * z);
    let (lift, flip) = if twisted { (0.1, -1.0) } else { (0.0, 1.0) };
    ruled(
        &[
            at(0.0, 0.0, 0.0),
            at(0.33, 0.6, lift),
            at(0.66, -0.6, -lift),
            at(1.0, 0.0, 0.0),
        ],
        &[
            at(0.0, 0.0, 1.0),
            at(0.33, 0.6 * flip, 1.0 - lift),
            at(0.66, -0.6 * flip, 1.0 + lift),
            at(1.0, 0.0, 1.0),
        ],
    )
}

/// A biquadratic over `[0, 1]²` whose height is `z(i, j)` at control
/// point `(i, j)`.
fn net(z: &dyn Fn(usize, usize) -> f64) -> Surface<f64> {
    let k = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
    let control = (0..9)
        .map(|n| {
            let (i, j) = (n / 3, n % 3);
            #[allow(clippy::cast_precision_loss)]
            pt(0.5 * i as f64, 0.5 * j as f64, z(i, j))
        })
        .collect();
    wall(NurbsSurface::new(k.clone(), k, control, vec![1.0; 9]).unwrap())
}

/// The dome `z = −(x − ½)² − (y − ½)²`, its top `z = 0` at the centre.
fn dome() -> Surface<f64> {
    let g = [-0.25, 0.25, -0.25];
    net(&|i, j| g[i] + g[j])
}

/// A dome over `[0, 1]²` whose top stands off the rectangle's middle,
/// near `(0.3, 0.3)`: its height `z(x) + z(y)` with `z` the quadratic
/// through `(0, −0.2)`, `(½, 0.2)`, `(1, −0.6)`.
fn skewed_dome() -> Surface<f64> {
    let g = [-0.2, 0.2, -0.6];
    net(&|i, j| g[i] + g[j])
}

/// The sweep of the pair's wall against its plane.
fn swept(p: &Surface<f64>, w: &Surface<f64>) -> Sweep {
    super::swept(p, w, band())
}

/// What a pair's classification answers, read coarsely.
#[derive(Clone, Debug, PartialEq)]
enum Got {
    /// `Reaching`, with the wall's sides.
    Reaching(ChartReach),
    /// No component.
    Apart,
    /// Refused by name.
    Pinched(&'static str),
    /// The wall decided on the plane.
    Coincident,
    /// The hull's height in band.
    Escalated,
    /// The lane cannot read the pose at this ε.
    Unreadable,
    /// Anything else.
    Other(String),
}

/// The pair's classification through the march door, read coarsely, the
/// wall's sides in either order.
fn got(p: &Surface<f64>, w: &Surface<f64>) -> Got {
    use crate::intersect::SectionError;
    match pair_section(p, w, None, Some(SsiLane::marched()), band()) {
        Ok(SectionRead::Built(s)) => match s.components() {
            Components::Reaching(reached) => {
                let [a, b] = reached.rims();
                Got::Reaching(a.or(b))
            }
            Components::Counted(c) if c.is_empty() => Got::Apart,
            other => Got::Other(format!("{other:?}")),
        },
        Err(SectionError::Pinched { predicate }) => Got::Pinched(predicate),
        Err(SectionError::CoincidentSurfaces) => Got::Coincident,
        Err(SectionError::Escalated(_)) => Got::Escalated,
        Err(SectionError::Unaccounted { cause })
            if cause.contains("can settle a state no finer") =>
        {
            Got::Unreadable
        }
        other => Got::Other(format!("{other:?}")),
    }
}

fn side(fixed: ChartAxis, end: ChartEnd) -> ChartReach {
    ChartReach::side(ChartSide { fixed, end })
}

/// What a probe pair's classification is, in the stages' order.
#[derive(Clone, Debug)]
enum Want {
    /// Proved loop-free by the sweep.
    Reaching,
    /// No component.
    Apart,
    /// A side grazing the plane, refused by the boundary pass by name.
    Graze,
    /// A side lying in the plane, the wall rising off it to the height
    /// `s` at its far side: on the plane where `s` is within the zero
    /// band, in band where it is short of the escalation, and otherwise
    /// a `Side` region the sweep proves loop-free.
    Rising(f64),
    /// A wall 1 km out: refused where the lane cannot settle a state well
    /// inside the band there ([`lane_unreadable_at_1_km`]), and otherwise
    /// as the inner answer.
    Far(Box<Want>),
}

/// The probe's synthetic pairs, named, with what each must answer.
fn probe_pairs() -> Vec<(String, Surface<f64>, Surface<f64>, Want)> {
    let eps = band().zero();
    let mut pairs: Vec<(String, Surface<f64>, Surface<f64>, Want)> = vec![
        (
            "exterior graze".into(),
            level(0.0),
            bowed([0.1, -0.1, 0.1]),
            Want::Graze,
        ),
        (
            "interior graze".into(),
            level(0.0),
            bowed([-0.1, 0.1, -0.1]),
            Want::Graze,
        ),
    ];
    for d in [100.0 * eps, -100.0 * eps, 1e-3] {
        for (what, z) in [
            ("exterior", [0.1, -0.1, 0.1]),
            ("interior", [-0.1, 0.1, -0.1]),
        ] {
            // Below the exterior wall's lowest point the plane meets
            // nothing.
            let want = if what == "exterior" && d < 0.0 {
                Want::Apart
            } else {
                Want::Reaching
            };
            pairs.push((
                format!("{what} near-graze at {d:e}"),
                level(d),
                bowed(z),
                want,
            ));
        }
    }
    // The probe's angles, its 10⁻⁶ and 10⁻⁸ rad moved to 5·10⁻⁷ and
    // 3·10⁻⁸: a 1 m wall rising 10⁻⁶ m sits on the zero band at ε 1e-6,
    // and one rising 10⁻⁸ m on the escalation at ε 1e-9 (ten bands), so
    // neither decides one way at those ε. Besides them, a wall rising a
    // tenth of the zero band (on the plane at every ε) and one rising
    // between the band's two thresholds (in band at every ε).
    let b = band();
    let within = (0.1 * b.zero()).asin();
    let between = (b.zero() * b.escalate()).sqrt().asin();
    for th in [
        std::f64::consts::FRAC_PI_2,
        0.3,
        1e-3,
        5e-7,
        3e-8,
        within,
        between,
    ] {
        let (s, c) = th.sin_cos();
        pairs.push((
            format!("a side in the plane, the wall at {th:e} rad"),
            level(0.0),
            ruled(
                &[pt(0.0, 0.0, 0.0), pt(1.0, 0.0, 0.0)],
                &[pt(0.0, c, s), pt(1.0, c, s)],
            ),
            Want::Rising(s),
        ));
    }
    pairs.push((
        "a bowed side lying in the plane".into(),
        level(0.0),
        ruled(
            &[pt(0.0, 0.0, 0.0), pt(0.5, 0.3, 0.0), pt(1.0, 0.0, 0.0)],
            &[pt(0.0, 0.0, 1.0), pt(0.5, 0.3, 1.0), pt(1.0, 0.0, 1.0)],
        ),
        Want::Reaching,
    ));
    let flat = || {
        ruled(
            &[pt(0.0, 0.0, 0.0), pt(1.0, 0.0, 0.0)],
            &[pt(0.0, 0.0, 1.0), pt(1.0, 0.0, 1.0)],
        )
    };
    // A plane through one corner only, the wall leaving it inward on
    // both sides of its height, meets nothing the pass ends a branch at.
    for (what, n, want) in [
        (
            "a plane touching one corner",
            Vec3::new(1.0, 0.0, 1.0),
            Want::Apart,
        ),
        ("the diagonal", Vec3::new(1.0, 0.0, -1.0), Want::Reaching),
        (
            "a plane near a side",
            Vec3::new(1.0, 0.0, 1e-3),
            Want::Reaching,
        ),
    ] {
        pairs.push((what.into(), plane(pt(0.0, 0.0, 0.0), n), flat(), want));
    }
    let far = pt(1000.0, 1000.0, 1000.0);
    let at = |x: f64, y: f64, z: f64| pt(far.x + 50.0 * x, far.y + 50.0 * y, far.z + 50.0 * z);
    for (what, w, n) in [
        (
            "the S wall at 1 km",
            inflected(far, 50.0, false),
            Vec3::unit_z(),
        ),
        (
            "the S wall at 1 km, the plane tilted",
            inflected(far, 50.0, false),
            Vec3::new(0.3, 0.0, 1.0),
        ),
        (
            "the twisted wall at 1 km",
            inflected(far, 50.0, true),
            Vec3::unit_z(),
        ),
        (
            "the S wall at the origin, the plane tilted",
            inflected(Point3::origin(), 1.0, false),
            Vec3::new(0.3, 0.0, 1.0),
        ),
    ] {
        let (o, want) = if what.contains("origin") {
            (pt(0.5, 0.0, 0.5), Want::Reaching)
        } else {
            (at(0.5, 0.0, 0.5), Want::Far(Box::new(Want::Reaching)))
        };
        pairs.push((what.into(), plane(o, n), w, want));
    }
    pairs.push((
        "the exterior graze at 1 km".into(),
        plane(at(0.5, 0.0, 0.0), Vec3::unit_z()),
        ruled(
            &[at(0.0, 0.0, 0.1), at(0.5, 0.0, -0.1), at(1.0, 0.0, 0.1)],
            &[at(0.0, 0.0, 1.0), at(0.5, 0.0, 1.0), at(1.0, 0.0, 1.0)],
        ),
        Want::Far(Box::new(Want::Graze)),
    ));
    pairs
}

/// **Whether the lane cannot read a wall 1 km out at the run's band**:
/// its coordinates there are `1.8·10⁻¹²` m apart, not well inside a band
/// of `10⁻¹²` m (ε 1e-12, refused) and well inside one of `10⁻⁹` m or
/// more.
fn lane_unreadable_at_1_km() -> bool {
    band().zero() < 1e-11
}

/// Whether `got` is what `want` asks, at the run's band.
fn answers(got: &Got, want: &Want) -> bool {
    let b = band();
    match want {
        Want::Reaching => matches!(got, Got::Reaching(r) if r.any()),
        Want::Apart => *got == Got::Apart,
        Want::Graze => *got == Got::Pinched("ssi_boundary_crossing"),
        Want::Rising(s) if *s <= b.zero() => *got == Got::Coincident,
        Want::Rising(s) if *s < b.escalate() => *got == Got::Escalated,
        Want::Rising(_) => {
            matches!(got, Got::Reaching(r) if *r == side(ChartAxis::V, ChartEnd::Low))
        }
        Want::Far(_) if lane_unreadable_at_1_km() => *got == Got::Unreadable,
        Want::Far(inner) => answers(got, inner),
    }
}

/// **The probe's pairs, classified in C5's order.** The hull decides a
/// wall within the zero band of the plane coincident and one short of
/// the escalation in band; the boundary pass refuses a graze of a side
/// by name, as the whole reading does; the sweep proves the rest
/// loop-free with no whole reading, in either operand order. So the
/// probe's exact grazes, which it found W5 clearing where the whole
/// reading refused, refuse here too: the pass runs before the sweep. A
/// side lying in the plane is a `Side` region on that side alone. A wall
/// 1 km out is refused where the lane cannot settle a state well inside
/// the band (at ε 1e-12), and otherwise answers as at the origin; its
/// near twin at the origin always does. The near-graze 100ε above the
/// exterior wall's lowest point reaches the bottom side alone.
///
/// Red against the sweep removed (each pose is read whole: a list), the
/// boundary pass run only behind the sweep (the grazes clear), the
/// hull's coincidence dropped and its escalation dropped (a wall rising
/// a tenth of the band, or between its thresholds, reads otherwise).
#[test]
fn the_probes_pairs_are_classified_in_c5s_order() {
    for (what, p, w, want) in probe_pairs() {
        for (order, got) in [("plane first", got(&p, &w)), ("wall first", got(&w, &p))] {
            assert!(
                answers(&got, &want),
                "{what}, {order}: {got:?}, wanted {want:?}"
            );
        }
    }
    let eps = band().zero();
    assert_eq!(
        got(&level(100.0 * eps), &bowed([0.1, -0.1, 0.1])),
        Got::Reaching(side(ChartAxis::V, ChartEnd::Low)),
        "the near-graze's short arc ends on the bottom side alone"
    );
}

/// **A loop inside the rectangle is never proved away.** The dome cut at
/// `z = −0.1` meets the plane in a circle about its top. The top's cells
/// are excluded above the plane, but no chain of cells above it joins
/// them to the rectangle's sides, so the sweep declines there, and the
/// pair is read whole: one loop.
///
/// Red against the join to the boundary dropped (a cell excluded is read
/// as harmless however it is enclosed: the sweep proves the loop away),
/// and against the sweep forced to clear.
#[test]
fn a_loop_about_an_excluded_top_declines() {
    assert!(
        matches!(
            swept(&level(-0.1), &dome()),
            Sweep::Declined(Declined::Enclosed(_))
        ),
        "{:?}",
        swept(&level(-0.1), &dome())
    );
    let got = match pair_section(
        &level(-0.1),
        &dome(),
        None,
        Some(SsiLane::marched()),
        band(),
    ) {
        Ok(SectionRead::Built(s)) => s.components().clone(),
        other => panic!("the dome is read whole: {other:?}"),
    };
    assert!(
        matches!(&got, Components::Counted(c) if c.len() == 1
            && c[0].shape == crate::pair_section::ComponentShape::Loop),
        "read whole, one loop: {got:?}"
    );
}

/// **The gradient is bounded over the whole cell, not read at a point.**
/// The skewed dome cut at `z = −0.2` meets the plane in a loop about its
/// top near `(0.3, 0.3)`; the gradient at the rectangle's middle is not
/// zero. Red against the gradient read at a cell's middle rather than
/// bounded over it (the one span cell reads regular and the loop is
/// proved away).
#[test]
fn the_gradient_is_bounded_over_the_cell() {
    assert!(
        matches!(swept(&level(-0.2), &skewed_dome()), Sweep::Declined(_)),
        "{:?}",
        swept(&level(-0.2), &skewed_dome())
    );
}

/// **A critical point within the band of the plane declines at the
/// floor.** The dome's top within the band of the plane: the cells about
/// the top are neither regular nor past the band, down to the floor,
/// where an isolated critical point is decided (a touch). The dome
/// scaled to `10⁻⁷` m deep has its top half an escalation below the
/// plane, its floor cells' boxes far narrower than that. Red against a
/// declined cell read as regular, and against the exclusion read past
/// zero rather than past the band (the shallow top's cells read below
/// the plane, joined to the sides, and the in-band touch is proved
/// away).
#[test]
fn a_critical_point_in_band_declines_at_the_floor() {
    let g = [-0.25e-7, 0.25e-7, -0.25e-7];
    let shallow = net(&|i, j| g[i] + g[j]);
    for (what, h, w) in [
        ("above", 0.5 * band().zero(), dome()),
        ("through", 0.0, dome()),
        ("below", -0.5 * band().zero(), dome()),
        ("above the shallow dome", 0.5 * band().escalate(), shallow),
    ] {
        let got = swept(&level(h), &w);
        assert!(
            matches!(got, Sweep::Declined(Declined::Floor(_))),
            "{what}, h {h:e}: {got:?}"
        );
    }
}

/// A pyramid of four bilinear patches over `[0, 1]²`, creased along
/// `x = ½` and `y = ½`, its apex `z = 1` at the centre and its rim at
/// `z = 0`.
fn pyramid() -> Surface<f64> {
    let k = KnotVector::clamped(vec![0.0, 0.0, 0.5, 1.0, 1.0], 1).unwrap();
    let control = (0..9)
        .map(|n| {
            let (i, j) = (n / 3, n % 3);
            #[allow(clippy::cast_precision_loss)]
            pt(
                0.5 * i as f64,
                0.5 * j as f64,
                if n == 4 { 1.0 } else { 0.0 },
            )
        })
        .collect();
    wall(NurbsSurface::new(k.clone(), k, control, vec![1.0; 9]).unwrap())
}

/// **A crease inside the rectangle declines.** The pyramid cut at
/// `z = ½` meets the plane in a loop about its apex, which lies on its
/// creases: each patch's gradient is off zero near the apex, from its
/// own side, and `φ` peaks where they meet. Red against the crease left
/// unread (every cell regular or joined below the plane: the loop is
/// proved away).
#[test]
fn a_crease_inside_the_rectangle_declines() {
    assert_eq!(
        swept(&level(0.5), &pyramid()),
        Sweep::Declined(Declined::Crease)
    );
}

/// **The reach is the sides the boundary pass ends the section on.** The
/// ramp `z = x` cut at `z = 0.4`: the line `x = 0.4` runs from the side
/// `v = 0` to `v = 1`, the pass's two crossings; the sides `u = 0` and
/// `u = 1` stand 0.4 and 0.6 off the plane. Red against every side read
/// as reached (the guard would ask a side the section never meets to
/// bound).
#[test]
fn the_reach_is_the_sides_the_pass_ends_the_section_on() {
    let ramp = net(&|i, _| 0.5 * f64::from(u8::try_from(i).unwrap()));
    assert_eq!(
        got(&level(0.4), &ramp),
        Got::Reaching(side(ChartAxis::V, ChartEnd::Low).or(side(ChartAxis::V, ChartEnd::High)))
    );
}

/// **The door decides a collapsed side clear over its whole row.** A
/// planar wall whose side `v = 0` collapses (its two row points half a
/// zero band apart), against the plane `x = c` placed so that the row's
/// first point stands three tenths of a band past the escalation and its
/// second two tenths short of it: the side is not decided clear of the
/// plane, so the door refuses it, named. Red against clearance read off
/// the row's first point alone (the door passes it).
#[test]
fn a_collapsed_side_is_cleared_over_its_whole_row() {
    let b = band();
    let z = b.zero();
    let wall = ruled(
        &[pt(0.5 * z, 0.0, 0.0), pt(0.0, 0.0, 0.0)],
        &[pt(1.0, 1.0, 0.0), pt(1.0, -1.0, 0.0)],
    );
    let c = 0.2 * z - b.escalate();
    let got = pair_section(
        &plane(pt(c, 0.0, 0.0), Vec3::unit_x()),
        &wall,
        None,
        Some(SsiLane::marched()),
        b,
    );
    assert!(
        matches!(&got, Err(crate::intersect::SectionError::Unaccounted { cause })
            if cause.contains("collapses to a point")),
        "the door refuses the side: {got:?}"
    );
}
