//! **The carry rows**: a spline wall's section against a plane and a
//! sphere, read whole and carried. A branch between two rim ends is
//! carried as the reading fitted it, its pad in metres; a loop on the
//! chart carries none, and a classification that reaches is not the
//! listing the carry takes. Each row names the mutant that turns it red,
//! and runs at the run's ε.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::{NurbsSurface, Surface};
use geom_core::spline::KnotVector;
use geom_core::{Band, Point3, Tol, Vec3};

use crate::pair_section::{
    CarriedComponent, CarryError, CarryRefusal, ChartReach, ComponentShape, NoCarrier, PairSection,
    SectionRead, SsiLane, pair_section,
};

fn band() -> Band {
    Band::linear(Tol::witness()).expect("the run's tolerance forms a band")
}

fn pt(x: f64, y: f64, z: f64) -> Point3<f64> {
    Point3::new(x, y, z)
}

/// **A lofted wall, scaled by `s`**: linear in `u` from `x = −1` to
/// `x = 1`, quadratic in `v` through the heights `z = 0, 1, 2` with the
/// middle row bulged out to `y = 1.3` and the end rows at `y = 1`, every
/// coordinate times `s`. Its height is `z = 2v` exactly, so the plane
/// `z = s` cuts it along the iso-line `v = ½`, from the side `u = 0` to
/// the side `u = 1`.
fn wall(s: f64) -> Surface<f64> {
    let ku = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
    let control = [-1.0, 1.0]
        .into_iter()
        .flat_map(|x| [(1.0, 0.0), (1.3, 1.0), (1.0, 2.0)].map(|(y, z)| pt(s * x, s * y, s * z)))
        .collect();
    Surface::Nurbs(std::sync::Arc::new(
        NurbsSurface::new(ku, kv, control, vec![1.0; 6]).unwrap(),
    ))
}

/// The plane `z = s`.
fn level(s: f64) -> Surface<f64> {
    Surface::Plane {
        origin: pt(0.0, 0.0, s),
        normal: Vec3::unit_z(),
        u_ref: Vec3::unit_x(),
    }
}

/// The sphere about `s·c` of radius `s·r`.
fn ball(s: f64, c: (f64, f64, f64), r: f64) -> Surface<f64> {
    Surface::Sphere {
        center: pt(s * c.0, s * c.1, s * c.2),
        radius: s * r,
        axis: Vec3::unit_z(),
        u_ref: Vec3::unit_x(),
    }
}

/// `a` against `b` classified through the march, then read whole: the
/// listing the carry takes.
fn listed(a: &Surface<f64>, b: &Surface<f64>) -> PairSection<f64> {
    let lane = Some(SsiLane::marched());
    match pair_section(a, b, None, lane, band()).expect("the pair classifies") {
        SectionRead::Built(s) => s
            .read_whole(lane, band())
            .unwrap_or_else(|e| panic!("{:?} × {:?} reads whole: {e:?}", a.kind(), b.kind())),
        SectionRead::Elsewhere(_) => panic!("a spline × implicit pair is built here"),
    }
}

/// The section's one carried component.
fn one(section: &PairSection<f64>) -> CarriedComponent<f64> {
    let carried = section
        .carry(Some(SsiLane::marched()), band())
        .expect("the section carries");
    let [c] = carried.components() else {
        panic!("one component: {:?}", carried.components().len());
    };
    c.clone()
}

/// The wall's point at `(u, v)`.
fn on_wall(w: &Surface<f64>, u: f64, v: f64) -> Point3<f64> {
    let Surface::Nurbs(net) = w else {
        unreachable!("the fixture's wall is a spline")
    };
    net.eval(u, v)
}

/// **The root in `v` of `f` on `[lo, hi]`**, bisected: `f(lo)` and
/// `f(hi)` hold opposite signs.
fn root(f: impl Fn(f64) -> f64, (mut lo, mut hi): (f64, f64), what: &str) -> f64 {
    let below = f(lo) < 0.0;
    assert!(
        below != (f(hi) < 0.0),
        "{what}: the iso-line crosses the section on [{lo}, {hi}]"
    );
    for _ in 0..80 {
        let mid = 0.5 * (lo + hi);
        if (f(mid) < 0.0) == below {
            lo = mid;
        } else {
            hi = mid;
        }
    }
    0.5 * (lo + hi)
}

/// **How far `p` stands from the component's carrier**, read
/// independently of the carry: the nearest of 4001 carrier points over
/// its span, then a golden-section search about it.
fn from_carrier(c: &CarriedComponent<f64>, p: Point3<f64>) -> f64 {
    let (a, b) = c.span;
    let d = |t: f64| (c.carrier.eval(t) - p).norm();
    let n: u32 = 4000;
    let at = |k: u32| a + (b - a) * f64::from(k) / f64::from(n);
    let k = (0..=n)
        .min_by(|&i, &j| d(at(i)).total_cmp(&d(at(j))))
        .unwrap();
    let (mut lo, mut hi) = (at(k.saturating_sub(1)), at((k + 1).min(n)));
    let g = 0.5 * (5f64.sqrt() - 1.0);
    for _ in 0..100 {
        let (x1, x2) = (hi - g * (hi - lo), lo + g * (hi - lo));
        if d(x1) < d(x2) {
            hi = x2;
        } else {
            lo = x1;
        }
    }
    d(0.5 * (lo + hi))
}

/// **A branch between two rim ends is carried as the reading fitted it,
/// its pad in metres**, at `s = 0.01, 1, 10`, against a plane (the
/// iso-line `v = ½`), a sphere (an arc from the bottom side back to it)
/// and a cylinder along the wall (an arc from side to side across its
/// upper half). The component is open on the wall's rim and holds the
/// reading's certificate; its pad holds the certificate's limb-2 bound
/// and every point of the section, each bisected along an iso-line of
/// the wall and its distance from the carrier read independently
/// (`from_carrier`); and a point twice the pad off the section reads
/// on no component.
///
/// What the code guarantees is the pad's lower side: every point of the
/// section within it. How near the rung it stands is a measurement, held
/// here as a tripwire: on these walls the pad is 1.0 to 1.2 rungs and the
/// reach at most about half the wall across (one window holds the whole
/// arc, `pad_of`).
///
/// Red against: a carry that refuses every spline × implicit component
/// (the listing's `Unminted`); a pad left in chart units (the tube's
/// `pad_u`, `pad_v`), far past the rung at `s = 0.01` and inside it at
/// `s = 10`; a window read whole, not cut into cells (the pad an eighth
/// of the arc's length, past two rungs). Larger walls are not read: at
/// `ε = 1e-12` the plane's whole reading at `s = 100` cannot decide its
/// limb 2, and at the default `ε` the sphere's at `s = 1000` settles no
/// foot
/// (`work/sectarms/a-sphere-against-a-spline-wall-a-kilometre-across-settles-no-foot.md`),
/// each before any carry.
#[test]
fn a_rim_arc_is_carried_as_its_reading_fitted_it_with_its_pad_in_metres() {
    for s in [1e-2, 1.0, 10.0] {
        let w = wall(s);
        // The sphere about the bottom side's middle, pushed out past the
        // wall's rim line: an arc from the side `v = 0` back to it,
        // inside the ball at the rim and outside at `v = ½`.
        let sphere_point = |u: f64| {
            let c = pt(0.0, 1.1 * s, 0.0);
            let v = root(
                |v| (on_wall(&w, u, v) - c).norm() - 0.4 * s,
                (0.0, 0.5),
                "the sphere",
            );
            on_wall(&w, u, v)
        };
        // The cylinder along `x` about `(y, z) = (1.15·s, 1.6·s)` of
        // radius `0.5·s`: outside it at `v = 0.3`, inside at `v = 0.8`,
        // and its upper crossing past the wall's top.
        let rod_point = |u: f64| {
            let f = |v: f64| {
                let p = on_wall(&w, u, v);
                (p.y - 1.15 * s).hypot(p.z - 1.6 * s) - 0.5 * s
            };
            on_wall(&w, u, root(f, (0.3, 0.8), "the cylinder"))
        };
        let tube = Surface::Cylinder {
            origin: pt(0.0, 1.15 * s, 1.6 * s),
            axis: Vec3::unit_x(),
            radius: 0.5 * s,
            u_ref: Vec3::unit_z(),
        };
        let cases = [
            (
                "the plane",
                one(&listed(&level(s), &w)),
                (0..=20)
                    .map(|i| on_wall(&w, f64::from(i) / 20.0, 0.5))
                    .collect::<Vec<_>>(),
            ),
            (
                "the sphere",
                one(&listed(&w, &ball(s, (0.0, 1.1, 0.0), 0.4))),
                (0..=10)
                    .map(|i| sphere_point(0.32 + 0.36 * f64::from(i) / 10.0))
                    .collect(),
            ),
            (
                "the cylinder",
                one(&listed(&w, &tube)),
                (0..=20).map(|i| rod_point(f64::from(i) / 20.0)).collect(),
            ),
        ];
        for (what, comp, points) in &cases {
            let ComponentShape::Open { ends } = comp.shape else {
                panic!("s = {s}, {what}: a rim arc is open: {:?}", comp.shape);
            };
            assert_ne!(
                ends.rims,
                [ChartReach::default(); 2],
                "s = {s}, {what}: the arc reaches the wall's rim"
            );
            let cert = comp
                .certificate
                .as_ref()
                .expect("the reading's certificate is carried");
            assert!(
                comp.pad.is_finite() && comp.pad > 0.0 && comp.pad >= cert.hull_sup,
                "s = {s}, {what}: the pad {} holds the limb-2 bound {}",
                comp.pad,
                cert.hull_sup
            );
            let crate::ssi::SsiTube::Chart { rung, .. } = cert.tube else {
                panic!("s = {s}, {what}: a chart tube certifies the arc");
            };
            assert!(
                0.5 * rung <= comp.pad && comp.pad <= 2.0 * rung,
                "s = {s}, {what}: the pad {} stands within two rungs ({rung}) on these walls",
                comp.pad
            );
            assert!(
                comp.reach.is_finite() && comp.reach <= 1.1 * s,
                "s = {s}, {what}: the witness's reach {} reads, within about half the wall",
                comp.reach
            );
            let nearest = points
                .iter()
                .map(|&p| (p - comp.witness).norm())
                .fold(f64::INFINITY, f64::min);
            assert!(
                nearest <= comp.reach,
                "s = {s}, {what}: the witness stands {nearest} from the section's nearest \
                 point read, past its reach {}",
                comp.reach
            );
            for &p in points {
                let off = from_carrier(comp, p);
                assert!(
                    off <= comp.pad,
                    "s = {s}, {what}: the section's point {p:?} stands {off} from the carrier, \
                     past the pad {}",
                    comp.pad
                );
                assert!(
                    comp.reads(p, band())
                        .expect("a section point reads")
                        .is_some(),
                    "s = {s}, {what}: the section's point {p:?} reads on its component \
                     within the pad {}",
                    comp.pad
                );
            }
            // Twice the pad off the section, square to the carrier's
            // tangent at its middle (along `x`) and level: off the wall.
            let p = points[points.len() / 2];
            let (_, tangent) = comp.carrier.ders1(geom::mid_param(comp.span.0, comp.span.1));
            let across = Vec3::unit_z().cross(tangent);
            let q = p + across * (2.0 * comp.pad / across.norm());
            assert!(
                from_carrier(comp, q) > comp.pad + band().escalate(),
                "s = {s}, {what}: the probe stands past the pad"
            );
            assert!(
                comp.reads(q, band())
                    .expect("a point clear of the pad reads")
                    .is_none(),
                "s = {s}, {what}: a point twice the pad {} off the section reads on no \
                 component",
                comp.pad
            );
        }
    }
}

/// The cylinder along `x` about the wall's middle line
/// `(y, z) = (1.15·s, s)`, of radius `r·s`: it cuts the wall in two arcs
/// from side to side, about `2·r·s` apart.
fn rod(s: f64, r: f64) -> Surface<f64> {
    Surface::Cylinder {
        origin: pt(0.0, 1.15 * s, s),
        axis: Vec3::unit_x(),
        radius: r * s,
        u_ref: Vec3::unit_z(),
    }
}

/// The two arcs [`rod`] cuts on the wall, below the middle line then
/// above it: each the wall's point on 21 iso-lines `u`, bisected.
fn rod_arcs(w: &Surface<f64>, s: f64, r: f64) -> [Vec<Point3<f64>>; 2] {
    let f = |u: f64| {
        move |v: f64| {
            let p = on_wall(w, u, v);
            (p.y - 1.15 * s).hypot(p.z - s) - r * s
        }
    };
    let arc = |range: (f64, f64)| {
        (0..=20)
            .map(|i| {
                let u = f64::from(i) / 20.0;
                on_wall(w, u, root(f(u), range, "the rod"))
            })
            .collect()
    };
    [arc((0.0, 0.5)), arc((0.5, 1.0))]
}

/// **Each point of two close rim arcs reads on its own arc alone, or the
/// carry refuses by name**, at `s = 0.01, 1, 10`: a rod along the wall
/// cuts it in two arcs `2·r` apart, `r = 0.07, 0.10, 0.15`. Carried, each
/// arc's witness and every point of each arc (bisected along the wall's
/// iso-lines) reads on that arc and on no other (`CarriedSection::site`);
/// refused, the refusal is [`CarryRefusal::PadsMeet`]. A site read on two
/// arcs that share no node is a desync, so neither may be met.
///
/// Red against: a pad an eighth of the arc's length (the window read
/// whole), which reaches across to the other arc: a witness on two
/// components.
#[test]
fn each_point_of_two_close_rim_arcs_reads_on_its_own_arc_or_the_carry_refuses() {
    for s in [1e-2, 1.0, 10.0] {
        for r in [0.07, 0.10, 0.15] {
            let w = wall(s);
            let carried = match listed(&w, &rod(s, r)).carry(Some(SsiLane::marched()), band()) {
                Ok(carried) => carried,
                Err(CarryError::Refused(CarryRefusal::PadsMeet { .. })) => continue,
                Err(e) => panic!("s = {s}, r = {r}: carried, or refused by name: {e:?}"),
            };
            let [lower, upper] = carried.components() else {
                panic!(
                    "s = {s}, r = {r}: two arcs: {}",
                    carried.components().len()
                );
            };
            // The listing's order is the march's: name each arc by its
            // witness's height.
            let (below, above) = if lower.witness.z < upper.witness.z {
                (0, 1)
            } else {
                (1, 0)
            };
            let [low_points, high_points] = rod_arcs(&w, s, r);
            let witnesses = carried.components().iter().map(|c| c.witness);
            for (k, points) in [
                (below, low_points),
                (above, high_points),
            ] {
                for p in points.into_iter().chain(witnesses.clone().skip(k).take(1)) {
                    match carried.site(p, band()) {
                        Ok(crate::pair_section::SiteReading::OnArc(foot)) => assert_eq!(
                            foot.component, k,
                            "s = {s}, r = {r}: {p:?} reads on its own arc"
                        ),
                        other => panic!("s = {s}, r = {r}: {p:?} reads on one arc: {other:?}"),
                    }
                }
            }
        }
    }
}

/// **Two arcs closer than their pads carry neither, by name**: a rod of
/// radius `0.05` cuts the wall in two arcs `0.1` apart, each read within
/// a pad of about `0.053`, so a point on the rod's axis line between them
/// would stand within both. The carry refuses
/// [`CarryRefusal::PadsMeet`], naming both pads and the gap.
///
/// Red against: a carry that holds no pair of arcs apart (`apart`): the
/// arcs carried, and the axis point a desync on two components.
#[test]
fn two_arcs_closer_than_their_pads_carry_neither_by_name() {
    let refused = listed(&wall(1.0), &rod(1.0, 0.05)).carry(Some(SsiLane::marched()), band());
    let Err(CarryError::Refused(CarryRefusal::PadsMeet { pads, gap })) = refused else {
        panic!("two arcs within their pads refuse by name: {refused:?}");
    };
    assert!(
        gap <= pads[0] + pads[1] && gap > 0.0 && gap <= 0.1,
        "the arcs stand {gap} apart, within the pads {pads:?}"
    );
}

/// **A loop on the wall's chart carries none, by name**: the reading
/// lists it and fits its carrier, and no seat opens a loop certified on a
/// chart, so the carry refuses it as [`NoCarrier::ChartLoop`] rather than
/// hand the join a component it cannot open.
///
/// Red against: a loop carried as a rim arc is.
#[test]
fn a_loop_on_the_wall_carries_none_by_name() {
    let w = wall(1.0);
    // A ball through the wall's middle, clear of every side: a loop.
    let section = listed(&w, &ball(1.0, (0.0, 1.3, 1.0), 0.3));
    let refused = section.carry(Some(SsiLane::marched()), band());
    assert!(
        matches!(
            refused,
            Err(CarryError::Refused(CarryRefusal::Unminted(
                NoCarrier::ChartLoop
            )))
        ),
        "{refused:?}"
    );
}

/// **A classification that reaches is not the listing the carry takes**:
/// it names the sides the section ends on and no component, so carrying
/// it is a desync; its whole reading is what carries.
///
/// Red against: a carry that reads a reaching classification as empty.
#[test]
fn a_reaching_classification_is_carried_from_its_whole_reading() {
    let lane = Some(SsiLane::marched());
    let SectionRead::Built(section) =
        pair_section(&level(1.0), &wall(1.0), None, lane, band()).expect("the pair classifies")
    else {
        panic!("a spline × implicit pair is built here");
    };
    assert!(
        matches!(section.carry(lane, band()), Err(CarryError::Desync(_))),
        "a reaching classification carries nothing"
    );
    assert_eq!(one(&listed(&level(1.0), &wall(1.0))).winds, [false, false]);
}

