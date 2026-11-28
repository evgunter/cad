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

use crate::intersect::SectionError;
use crate::pair_section::{
    CarriedComponent, CarryError, CarryRefusal, ChartReach, ComponentShape, NoCarrier, PairSection,
    SectionRead, SsiLane, pair_section,
};

pub(super) fn band() -> Band {
    Band::linear(Tol::witness()).expect("the run's tolerance forms a band")
}

pub(super) fn pt(x: f64, y: f64, z: f64) -> Point3<f64> {
    Point3::new(x, y, z)
}

/// **A lofted wall, scaled by `s`**: linear in `u` from `x = −1` to
/// `x = 1`, quadratic in `v` through the heights `z = 0, 1, 2` with the
/// middle row bulged out to `y = 1.3` and the end rows at `y = 1`, every
/// coordinate times `s`. Its height is `z = 2v` exactly, so the plane
/// `z = s` cuts it along the iso-line `v = ½`, from the side `u = 0` to
/// the side `u = 1`.
pub(super) fn wall(s: f64) -> Surface<f64> {
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
        frame: geom::Frame::new(pt(0.0, 0.0, s), Vec3::unit_z(), Vec3::unit_x()),
    }
}

/// The sphere about `s·c` of radius `s·r`.
pub(super) fn ball(s: f64, c: (f64, f64, f64), r: f64) -> Surface<f64> {
    Surface::Sphere {
        frame: geom::Frame::new(
            pt(s * c.0, s * c.1, s * c.2),
            Vec3::unit_z(),
            Vec3::unit_x(),
        ),
        radius: s * r,
    }
}

/// `a` against `b` classified through the march, then read whole: the
/// listing the carry takes.
pub(super) fn listed(a: &Surface<f64>, b: &Surface<f64>) -> PairSection<f64> {
    let lane = Some(SsiLane::marched());
    match pair_section(a, b, None, lane, band()).expect("the pair classifies") {
        SectionRead::Built(s) => s
            .read_whole(lane, band())
            .unwrap_or_else(|e| panic!("{:?} × {:?} reads whole: {e:?}", a.kind(), b.kind())),
        SectionRead::Elsewhere(_) => panic!("a spline × implicit pair is built here"),
    }
}

/// [`listed`], or the refusal the pair's reading stops on before any
/// carry, where the row's scale is past what the run's ε reads.
fn listed_at(a: &Surface<f64>, b: &Surface<f64>) -> Result<PairSection<f64>, SectionError> {
    let lane = Some(SsiLane::marched());
    match pair_section(a, b, None, lane, band())? {
        SectionRead::Built(s) => s.read_whole(lane, band()),
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
/// here as a tripwire: on these walls the pad is about a rung and the
/// reach at most about half the wall across.
///
/// Red against: a carry that refuses every spline × implicit component
/// (the listing's `Unminted`); a pad left in chart units (the tube's
/// `pad_u`, `pad_v`), far past the rung at `s = 0.01` and inside it at
/// `s = 10`. Larger walls are not read: at
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
            frame: geom::Frame::new(pt(0.0, 1.15 * s, 1.6 * s), Vec3::unit_x(), Vec3::unit_z()),
            radius: 0.5 * s,
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
            let (_, tangent) = comp
                .carrier
                .ders1(geom::mid_param(comp.span.0, comp.span.1));
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
pub(super) fn rod(s: f64, r: f64) -> Surface<f64> {
    Surface::Cylinder {
        frame: geom::Frame::new(pt(0.0, 1.15 * s, s), Vec3::unit_x(), Vec3::unit_z()),
        radius: r * s,
    }
}

/// The two arcs [`rod`] cuts on the wall, below the middle line then
/// above it: each the wall's point on 21 iso-lines `u`, bisected.
pub(super) fn rod_arcs(w: &Surface<f64>, s: f64, r: f64) -> [Vec<Point3<f64>>; 2] {
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

/// **Each point of two close rim arcs reads on its own arc alone**, at
/// `s = 0.01, 1, 100`: a rod along the wall cuts it in two arcs `2·r`
/// apart, `r = 0.045, 0.09, 0.095, 0.15`. The certifying rung follows
/// the gap, so the arcs stand two to three rungs apart (2.03 at
/// `r = 0.045` and `0.09`). Each is carried. Each arc's witness and
/// every point of each arc (bisected along the wall's iso-lines) reads
/// on that arc and on no other (`CarriedSection::site`), and so does the
/// point moved half the band's zero toward the other arc. The wall's
/// point on the rod's axis line, between the arcs, lies in no cell of
/// either and reads on no component.
///
/// At `ε = 1e-12` the wall at `s = 100` is not read: the whole reading
/// cannot decide its limb 2 there, before any carry (asserted, with the
/// scale it is allowed at).
///
/// Red against: arcs held apart by their pads about their carriers'
/// hulls, each pad a rung and more, where the arcs two rungs apart
/// refuse `PadsMeet` (`r = 0.045, 0.09, 0.095`); a refinement that
/// drops a half holding a point of the arc (the point reads on none).
#[test]
fn each_point_of_two_close_rim_arcs_reads_on_its_own_arc_alone() {
    use crate::pair_section::{SiteReading, SiteRefusal};
    for s in [1e-2, 1.0, 100.0] {
        for r in [0.045, 0.09, 0.095, 0.15] {
            let w = wall(s);
            let section = match listed_at(&w, &rod(s, r)) {
                Ok(section) => section,
                Err(unread) => {
                    assert!(
                        s == 100.0 && Tol::witness().eps() == 1e-12,
                        "s = {s}, r = {r}: the pair reads whole: {unread:?}"
                    );
                    println!("s = {s}, r = {r}: not read at this ε, before any carry: {unread:?}");
                    // The scale, not the rod, is what the ε does not read.
                    break;
                }
            };
            let carried = section
                .carry(Some(SsiLane::marched()), band())
                .unwrap_or_else(|e| panic!("s = {s}, r = {r}: two arcs are carried: {e:?}"));
            let [lower, upper] = carried.components() else {
                panic!("s = {s}, r = {r}: two arcs: {}", carried.components().len());
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
            for (k, toward, points) in [(below, 1.0, low_points), (above, -1.0, high_points)] {
                let nudge = Vec3::unit_z() * (toward * 0.5 * band().zero());
                let moved = points.iter().map(|&p| p + nudge).collect::<Vec<_>>();
                let own = witnesses.clone().skip(k).take(1);
                for p in points.into_iter().chain(moved).chain(own) {
                    match carried.site(p, band()) {
                        Ok(SiteReading::OnArc(foot)) => assert_eq!(
                            foot.component, k,
                            "s = {s}, r = {r}: {p:?} reads on its own arc"
                        ),
                        other => panic!("s = {s}, r = {r}: {p:?} reads on one arc: {other:?}"),
                    }
                }
            }
            let between = carried.site(on_wall(&w, 0.5, 0.5), band());
            assert!(
                matches!(
                    between,
                    Err(SiteRefusal::Desync(
                        "a site lies on no component of its section"
                    ))
                ),
                "s = {s}, r = {r}: the point between the arcs lies in no cell of either: \
                 {between:?}"
            );
        }
    }
}

/// **A wall whose chart quickens**, scaled by `s`: linear in `u` from
/// `x = −1` to `x = 1`, quadratic in `v` over two spans that meet at
/// `v = ½`, its heights' control rows at `z = 0, 0.05, 0.2, 2`. Over the
/// lower span it rises at `0.2` to `0.3` a unit of `v`, over the upper
/// at up to `7.2`, so a chart pad sized by the wall's fastest speed
/// reaches a few hundredths of a rung in metres on the lower span.
fn quickening(s: f64) -> Surface<f64> {
    let ku = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    let kv = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.5, 1.0, 1.0, 1.0], 2).unwrap();
    let control = [-1.0, 1.0]
        .into_iter()
        .flat_map(|x| {
            [(1.0, 0.0), (1.01, 0.05), (1.0, 0.2), (1.0, 2.0)].map(|(y, z)| pt(s * x, s * y, s * z))
        })
        .collect();
    Surface::Nurbs(std::sync::Arc::new(
        NurbsSurface::new(ku, kv, control, vec![1.0; 8]).unwrap(),
    ))
}

/// **Two arcs a ninth of the tube's rung apart are carried, each point
/// on its own arc alone**, at `s = 0.01, 1, 100`: on the wall whose
/// chart quickens, a rod of radius `0.02·s` about the line `z = 0.06·s`
/// cuts the slow span in two arcs `0.038·s` apart. The rung, `0.35·s`,
/// is the tube's at the wall's fastest speed; on the slow span a window
/// reaches a few hundredths of it in metres, so the arcs stand closer
/// than a quarter of the rung (asserted, the row's premise) while their
/// windows do not overlap. Each arc's pad, `0.06·s`, is wider than the
/// gap: it is the arcs' thin cells that stand apart, not their pads.
/// Each point of each arc, and each arc's witness, reads on its own
/// arc.
///
/// Two scales are past what a run's ε reads, each refused before any
/// carry and asserted with the ε it is allowed at: at `ε = 1e-6` the
/// wall at `s = 0.01`, whose arcs end `3.8e-4` apart on its side, pinches
/// at the side; at `ε = 1e-12` the wall at `s = 100`, where the march
/// settles no state finer than the tolerance.
///
/// Red against: arcs held apart by their pads about their carriers'
/// hulls, some `0.07·s` each, where the two arcs refuse `PadsMeet`.
#[test]
fn two_arcs_a_ninth_of_a_rung_apart_on_a_quickening_wall_are_carried() {
    use crate::pair_section::SiteReading;
    for s in [1e-2, 1.0, 100.0] {
        let w = quickening(s);
        let (z0, r) = (0.06 * s, 0.02 * s);
        let partner = Surface::Cylinder {
            frame: geom::Frame::new(pt(0.0, s, z0), Vec3::unit_x(), Vec3::unit_z()),
            radius: r,
        };
        let section = match listed_at(&w, &partner) {
            Ok(section) => section,
            Err(unread) => {
                let eps = Tol::witness().eps();
                assert!(
                    (s == 1e-2 && eps == 1e-6) || (s == 100.0 && eps == 1e-12),
                    "s = {s}: the pair reads whole: {unread:?}"
                );
                println!("s = {s}: not read at this ε, before any carry: {unread:?}");
                continue;
            }
        };
        let carried = section
            .carry(Some(SsiLane::marched()), band())
            .unwrap_or_else(|e| panic!("s = {s}: two arcs are carried: {e:?}"));
        let [a, b] = carried.components() else {
            panic!("s = {s}: two arcs: {}", carried.components().len());
        };
        let (below, above) = if a.witness.z < b.witness.z {
            (0, 1)
        } else {
            (1, 0)
        };
        let gap = (a.witness - b.witness).norm();
        for c in [a, b] {
            let Some(crate::ssi::SsiTube::Chart { rung, .. }) =
                c.certificate.as_ref().map(|cert| cert.tube)
            else {
                panic!("s = {s}: a chart tube certifies the arc");
            };
            assert!(
                gap < 0.25 * rung,
                "s = {s}: the arcs stand {gap} apart, within a quarter of the rung {rung}"
            );
        }
        // Each arc's point on an iso-line `u`: the wall's point where it
        // meets the rod, below its axis line then above it.
        let f = |u: f64| {
            let w = &w;
            move |v: f64| {
                let p = on_wall(w, u, v);
                (p.y - s).hypot(p.z - z0) - r
            }
        };
        let axis = root(|v| on_wall(&w, 0.5, v).z - z0, (0.0, 0.5), "the axis line");
        for (k, range) in [(below, (0.0, axis)), (above, (axis, 0.5))] {
            let points = (0..=20).map(|i| {
                let u = f64::from(i) / 20.0;
                on_wall(&w, u, root(f(u), range, "the rod"))
            });
            let own = carried.components()[k].witness;
            for p in points.chain([own]) {
                match carried.site(p, band()) {
                    Ok(SiteReading::OnArc(foot)) => {
                        assert_eq!(foot.component, k, "s = {s}: {p:?} reads on its own arc");
                    }
                    other => panic!("s = {s}: {p:?} reads on one arc: {other:?}"),
                }
            }
        }
    }
}

/// **Two arcs the carry cannot part are listed refusing `PadsMeet`, and
/// the section carries none**: the rod's reading at the run's band,
/// listed at a band whose escalation is `0.1`, where its two arcs,
/// `0.18` apart, stand within the band of each other. Each arc's
/// component holds the refusal, and the carry returns it.
///
/// Red against: a listing that drops the refusal and carries each arc on
/// the cells it stopped at.
#[test]
fn two_arcs_the_carry_cannot_part_are_listed_refusing_and_carry_none() {
    use crate::pair_section::{ClassCarrier, Components, PadsStop};
    let (w, p) = (wall(1.0), rod(1.0, 0.09));
    let pose = super::SplineImplicitPose::of(&w, &p).expect("a spline wall and a cylinder");
    let reading = crate::ImplicitSection::of(&p, pose.net(), pose.domain(), band())
        .expect("the pair is read");
    let form = reading.form().clone();
    let out = reading.all().expect("the pair reads whole");
    let wide = Band::linear_at(Tol::witness(), 0.1 / Tol::witness().k()).expect("a band");
    let partner = super::Bracketed {
        surface: &p,
        form: &form,
    };
    let section = super::listed(pose.clone(), partner, out, None, wide).expect("the arcs list");
    let Components::Counted(components) = section.components() else {
        panic!("a whole reading lists its components");
    };
    assert_eq!(components.len(), 2, "the rod's two arcs");
    for c in components {
        assert!(
            matches!(
                &c.carrier,
                ClassCarrier::Fitted(f) if matches!(
                    f.as_ref(),
                    Err(CarryError::Refused(CarryRefusal::PadsMeet {
                        stop: PadsStop::Stall,
                        ..
                    }))
                )
            ),
            "each arc's component holds the refusal: {:?}",
            c.carrier
        );
    }
    let refused = section.carry(Some(SsiLane::marched()), wide);
    assert!(
        matches!(
            refused,
            Err(CarryError::Refused(CarryRefusal::PadsMeet { .. }))
        ),
        "the section carries none: {refused:?}"
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
