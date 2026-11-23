//! **The boolean's foot order and the split's walk order are one order
//! along a section conic.** The boolean ranks a germ's partners on a
//! pair the section table carries by their foot parameters' travel along
//! the carried component ([`along_facing`], through [`along_sign`]); the
//! split pairs a curved face's crossings in walk order
//! ([`crate::splitting::join::conic_pairs_of_sites`], the eccentric
//! anomaly through `sort_along_runs`). Each row feeds one set of sites on
//! one conic to both, walking it either way, and asks for one cyclic
//! order. The conics are the section value's own: a circle on a sphere
//! (plane × sphere) and tilted ellipses on a cylinder (plane × cylinder),
//! carried ([`geom_brep::pair_section`]).
//!
//! The sites, listed counter-clockwise from the germ: one mid-arc; a
//! pair thirty bands apart on the flank; a pair straddling the conic's
//! own seam (`θ = ±π`, where the split's walk starts); a pair straddling
//! the germ's half-turn; a pair straddling the carried component's seam;
//! and two behind. Each straddling pair sits fifteen bands either side,
//! clear of the band's escalation edge ([`Band::escalate`]). Every lane
//! takes them in one scrambled insertion order, so none inherits the
//! answer from its input.
//!
//! Each set is read twice: on the conic, then with every site moved
//! within the band off it, across it in the section plane and to
//! alternate sides, so neighbours sit up to two band widths apart across
//! the conic.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use super::super::section_table::SectionTable;
use super::super::{BooleanError, HalfGerm, Locus};
use super::{Faced, PartnerDistance, along_facing, along_sign, germ_arm, nearest_of};
use crate::HalfEdgeKey;
use crate::chord_join::JoinError;
use geom::{Curve3, Surface};
use geom_brep::SectionPick;
use geom_core::{Band, Point3, Tol, Vec3};

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

/// A site: its point and its unit tangent along the walk.
type Site = (Point3<f64>, Vec3<f64>);

/// The conic's axis line, through its centre, which sites move across the
/// conic by.
fn frame(conic: &Curve3<f64>) -> geom::Line<f64> {
    match conic {
        Curve3::Ellipse(_) | Curve3::Circle(_) => conic.frame().map(|f| f.axis_line()),
        _ => None,
    }
    .unwrap_or_else(|| panic!("not a conic: {conic:?}"))
}

/// The point at eccentric anomaly `t` and the unit tangent `sense`
/// (`±1`) of counter-clockwise there.
fn at(conic: &Curve3<f64>, t: f64, sense: f64) -> Site {
    let (p, d) = conic.ders1(t);
    (p, d * (sense / d.norm()))
}

/// `t` moved `bands` bands of arc along the conic.
fn on(conic: &Curve3<f64>, t: f64, bands: f64) -> f64 {
    t + bands * band().zero() / conic.ders1(t).1.norm()
}

/// The sites' parameters, counter-clockwise from the germ at `t0`
/// (module docs), with `extra` pairs straddling further parameters.
fn sites(conic: &Curve3<f64>, t0: f64, extra: &[f64]) -> Vec<f64> {
    use core::f64::consts::{PI, TAU};
    let flank = t0 + 1.6;
    let half = t0 + PI;
    let mut ts = vec![t0 + 0.8, flank, on(conic, flank, 30.0)];
    for &mid in [PI, half].iter().chain(extra) {
        ts.extend([on(conic, mid, -15.0), on(conic, mid, 15.0)]);
    }
    ts.extend([t0 + 4.6, t0 + 5.5]);
    let ccw = |t: f64| (t - t0).rem_euclid(TAU);
    ts.sort_by(|a, b| ccw(*a).total_cmp(&ccw(*b)));
    for w in ts.windows(2) {
        let arc = (ccw(w[1]) - ccw(w[0])) * conic.ders1(w[0]).1.norm() / band().zero();
        assert!(arc > 25.0, "fixture: sites {w:?} only {arc} bands apart");
    }
    ts.insert(0, t0);
    ts
}

/// Insertion slot `j` holds the site `k`-th in walk order, `k = 7j + 3`
/// mod `n`: a permutation for every `n` prime to 7.
fn scrambled(walk: &[usize]) -> Vec<usize> {
    let n = walk.len();
    assert_ne!(n % 7, 0, "fixture: 7 must be prime to the site count");
    (0..n).map(|j| walk[(7 * j + 3) % n]).collect()
}

/// `order` (sites by insertion slot) as a cycle read from site 0.
fn from_germ(order: &[usize]) -> Vec<usize> {
    let mut cycle = order.to_vec();
    let g = cycle
        .iter()
        .position(|&i| i == 0)
        .expect("the germ is a site");
    cycle.rotate_left(g);
    cycle
}

/// The candidates ranked by the join's own pick: the nearest
/// ([`nearest_of`] under [`along_sign`], free of nothing), as
/// `loose_partners` takes a germ's partner; then again over the rest.
/// Read from the germ, so the germ leads.
fn ranked(mut left: Vec<(usize, PartnerDistance<f64>)>) -> Result<Vec<usize>, BooleanError> {
    let mut out = vec![0];
    while !left.is_empty() {
        let cands = left
            .iter()
            .enumerate()
            .map(|(k, &(_, reach))| (reach, k))
            .collect();
        let sign = |c, b| along_sign(c, b, band());
        let best = nearest_of(cands, sign)?;
        let (_, k) = best.expect("a candidate is left");
        out.push(left.remove(k).0);
    }
    Ok(out)
}

/// One reading of a conic's sites by both lanes, each as a cycle of
/// site indices from the germ: walked `sense` (`±1` of counter-clockwise),
/// each site moved `off` band widths across the conic, to alternate
/// sides. `walk` is the sites' own order that way.
struct Lanes {
    walk: Vec<usize>,
    split: Result<Vec<usize>, JoinError>,
    foot: Result<Vec<usize>, BooleanError>,
}

fn lanes(
    (wall, section): (&Surface<f64>, &geom_brep::CarriedSection<f64>),
    conic: &Curve3<f64>,
    ts: &[f64],
    sense: f64,
    off: f64,
) -> Lanes {
    let n = ts.len();
    let walk: Vec<usize> = if sense > 0.0 {
        (0..n).collect()
    } else {
        core::iter::once(0).chain((1..n).rev()).collect()
    };
    let mut down = vec![false; n];
    for (k, &i) in walk.iter().enumerate() {
        down[i] = k % 2 == 0;
    }
    let slots = scrambled(&walk);
    let site = |i: usize| {
        let (p, ahead) = at(conic, ts[i], sense);
        let side = if i.is_multiple_of(2) { off } else { -off };
        let across = frame(conic).dir.get().cross(ahead).normalize();
        (p + across * (side * band().zero()), ahead)
    };

    let input: Vec<_> = slots
        .iter()
        .map(|&i| {
            let (p, ahead) = site(i);
            (p, down[i], if down[i] { ahead } else { -ahead })
        })
        .collect();
    let split_conic = crate::chord_join::section_conic(conic.clone()).unwrap();
    let split = crate::splitting::join::conic_pairs_of_sites(wall, &split_conic, &input, band())
        .map(|pairs| {
            let walked: Vec<usize> = pairs
                .iter()
                .flat_map(|&(a, b)| [slots[a], slots[b]])
                .collect();
            from_germ(&walked)
        });

    let (g, dir) = site(0);
    let partners = slots
        .iter()
        .filter(|&&i| i != 0)
        .map(|&i| (i, site(i)))
        .collect::<Vec<_>>();
    Lanes {
        walk,
        split,
        foot: feet(section, (g, dir), &partners).and_then(ranked),
    }
}

/// One conic, walked each way, on it and off it: the split's walk and
/// the carried lane's foot parameter, against the sites' own order.
fn hold(
    name: &str,
    (wall, section): (&Surface<f64>, &geom_brep::CarriedSection<f64>),
    conic: &Curve3<f64>,
    ts: &[f64],
) {
    let walks = [(1.0, "ccw"), (-1.0, "cw")];
    for ((sense, way), (off, place)) in walks
        .into_iter()
        .flat_map(|w| [(w, (0.0, "on")), (w, (0.9, "off"))])
    {
        let way = &format!("{way} {place} the conic");
        let read = lanes((wall, section), conic, ts, sense, off);
        let split = read
            .split
            .unwrap_or_else(|e| panic!("{name} {way}: the split's walk refused: {e:?}"));
        let foot = read
            .foot
            .unwrap_or_else(|e| panic!("{name} {way}: the foot order refused: {e:?}"));
        assert_eq!(
            foot, split,
            "{name} {way}: the carried lane's foot order and the split's walk order differ"
        );
        assert_eq!(
            split, read.walk,
            "{name} {way}: both lanes agree on an order that is not the sites' own"
        );
    }
}

/// Each partner as the carried lane reaches it from the germ: its foot
/// parameter's travel along the component ([`along_facing`]).
fn feet(
    section: &geom_brep::CarriedSection<f64>,
    (g, dir): Site,
    partners: &[(usize, Site)],
) -> Result<Vec<(usize, PartnerDistance<f64>)>, BooleanError> {
    let pillow = crate::fixtures::pillow(Tol::witness());
    let table = SectionTable::default();
    let mut halves = slotmap::SlotMap::<HalfEdgeKey, ()>::with_key();
    let geom_brep::SiteReading::OnArc(foot) =
        section.site(g, band()).map_err(super::site_refusal)?
    else {
        panic!("a site off every node")
    };
    let component = foot.component;
    let mut germ = |dir| HalfGerm {
        he: halves.insert(()),
        a_face: pillow.face_a,
        b_face: pillow.face_b,
        a_locus: Locus::InFace(pillow.face_a),
        b_locus: Locus::InFace(pillow.face_b),
        dir,
        branch: SectionPick::Component(component),
    };
    let from = germ(dir);
    partners
        .iter()
        .map(|&(i, (p, ahead))| {
            let to = germ(-ahead);
            let bodies = (&table, &pillow.body, &pillow.body);
            let along = match along_facing(bodies, section, (&from, g), (&to, p), band())? {
                Faced::Along(along) => along,
                _ => panic!("site {i}: a partner facing back does not face the germ"),
            };
            let reach = PartnerDistance {
                arm: germ_arm::<f64>(None, band())?,
                chord: (p - g).norm(),
                turn: None,
                along: Some(along),
            };
            Ok((i, reach))
        })
        .collect()
}

/// **The pair's section, carried, and its one loop**: `a` and `b` read
/// on the section value over `region`, its one component's carrier the
/// conic the sites lie on.
fn carried(
    a: &Surface<f64>,
    b: &Surface<f64>,
    region: geom_brep::ExtentBall<f64>,
) -> (geom_brep::CarriedSection<f64>, Curve3<f64>) {
    let section =
        match geom_brep::pair_section(a, b, Some(region), None::<geom_core::NoLane>, band())
            .unwrap()
        {
            geom_brep::SectionRead::Built(s) => s.carry(None::<geom_core::NoLane>, band()).unwrap(),
            geom_brep::SectionRead::Elsewhere(route) => panic!("{route:?}"),
        };
    let [component] = section.components() else {
        panic!("one loop: {:?}", section.components().len())
    };
    let conic = component.carrier.clone();
    (section, conic)
}

/// The sites on `conic` from the germ at `t0`, with a pair straddling
/// the carried component's own seam.
fn sites_about_the_seam(
    section: &geom_brep::CarriedSection<f64>,
    conic: &Curve3<f64>,
    t0: f64,
) -> Vec<f64> {
    let seam = section.components()[0].carrier.eval(0.0);
    let split_conic = crate::chord_join::section_conic(conic.clone()).unwrap();
    sites(conic, t0, &[split_conic.param(seam)])
}

/// A tilted plane and the unit wall about `z` through `origin`, the
/// plane's normal `tilt` off the axis, and the region about them.
fn tilted_cut(tilt: f64, reach: f64) -> (Surface<f64>, Surface<f64>, geom_brep::ExtentBall<f64>) {
    let origin = Point3::new(0.3, -0.2, 0.0);
    let wall = Surface::Cylinder {
        frame: geom::Frame::new(origin, Vec3::unit_z(), Vec3::unit_x()),
        radius: 1.0,
    };
    let normal = Vec3::new(
        tilt.sin() * 0.7f64.cos(),
        tilt.sin() * 0.7f64.sin(),
        tilt.cos(),
    );
    let plane = Surface::Plane {
        frame: geom::Frame::new(
            origin + Vec3::unit_z() * 0.5,
            normal,
            Vec3::unit_z().cross(normal).normalize(),
        ),
    };
    let region = geom_brep::ExtentBall::fixture(origin, reach);
    (plane, wall, region)
}

/// A tilted ellipse on a cylinder (plane × cylinder): aspect 2, so the
/// flank is live.
#[test]
fn a_tilted_ellipse_on_a_cylinder_walks_one_order_in_both_lanes() {
    let (plane, wall, region) = tilted_cut(60f64.to_radians(), 4.0);
    let (section, conic) = carried(&plane, &wall, region);
    assert!(matches!(conic, Curve3::Ellipse(_)), "{conic:?}");
    let ts = sites_about_the_seam(&section, &conic, 0.4);
    hold("ellipse", (&wall, &section), &conic, &ts);
}

/// A circle on a sphere (plane × sphere).
#[test]
fn a_circle_on_a_sphere_walks_one_order_in_both_lanes() {
    let center = Point3::new(0.1, 0.2, -0.3);
    let n = Vec3::new(1.0, -2.0, 2.0).normalize();
    let sphere = Surface::Sphere {
        frame: geom::Frame::new(center, Vec3::unit_z(), Vec3::unit_x()),
        radius: 1.5,
    };
    let plane = Surface::Plane {
        frame: geom::Frame::new(center + n * 0.6, n, Vec3::unit_z().cross(n).normalize()),
    };
    let (section, conic) = carried(&plane, &sphere, geom_brep::ExtentBall::fixture(center, 2.0));
    assert!(matches!(conic, Curve3::Circle(_)), "{conic:?}");
    let ts = sites_about_the_seam(&section, &conic, 0.4);
    hold("circle", (&sphere, &section), &conic, &ts);
}

/// An ellipse of aspect 30 on a cylinder (plane × cylinder), carried,
/// and the cylinder.
fn steep() -> (Surface<f64>, geom_brep::CarriedSection<f64>, Curve3<f64>) {
    let (plane, wall, region) = tilted_cut((1.0f64 / 30.0).acos(), 64.0);
    let (section, conic) = carried(&plane, &wall, region);
    (wall, section, conic)
}

/// **Off the conic on a steep flank, the lanes may refuse but never
/// order two ways.** An ellipse of aspect 30 on a cylinder, where
/// `cot ψ` at the flank (`θ = π/4`) is about 15, so a site moved within
/// the band across the conic moves its foot parameter and the split's
/// eccentric anomaly alike by many band widths. A flank pair `Δ` bands apart, moved to opposite
/// sides, between the germ and one more site: wherever a lane orders the
/// four, it is the sites' own order, and the lanes agree; each pair
/// that close may instead refuse. Far enough apart, both order.
#[test]
fn off_a_steep_flank_the_lanes_refuse_rather_than_order_two_ways() {
    use core::f64::consts::FRAC_PI_4;
    let (wall, section, conic) = steep();
    for bands in [12.0, 16.0, 24.0, 32.0, 48.0, 64.0, 96.0, 128.0] {
        let ts = [-1.0, FRAC_PI_4, on(&conic, FRAC_PI_4, bands), 2.5];
        for sense in [1.0, -1.0] {
            for off in [0.9, -0.9] {
                let at = format!("Δ {bands} bands, sense {sense:+}, off {off:+}");
                let read = lanes((&wall, &section), &conic, &ts, sense, off);
                if let Ok(split) = &read.split {
                    assert_eq!(
                        split, &read.walk,
                        "{at}: the split's walk orders the sites otherwise"
                    );
                }
                if let Ok(foot) = &read.foot {
                    assert_eq!(
                        foot, &read.walk,
                        "{at}: the carried lane's foot orders the sites otherwise"
                    );
                }
                if bands >= 128.0 {
                    assert!(
                        read.split.is_ok() && read.foot.is_ok(),
                        "{at}: far apart, both lanes order: split {:?}, foot {:?}",
                        read.split.err(),
                        read.foot.err()
                    );
                }
            }
        }
    }
}

/// **Where the split's walk cannot order a flank pair, it says so.** On
/// the steep flank, two sites twelve and sixteen bands apart, each moved
/// within the band to opposite sides of the conic, read in reverse
/// through the eccentric anomaly and decided apart; the walk then
/// refuses them as crossings that do not alternate
/// ([`crate::chord_join::ConicCrossingsCase::NotAlternating`]), a claim
/// about the face, where the cause is two sites the band cannot order.
/// Run:
/// `cargo nextest run -p topo off_a_steep_flank_the_walk_escalates --run-ignored only`.
#[test]
#[ignore = "disabled red: work/sectarms/the-splits-walk-decides-off-conic-sites-the-band-cannot-order.md"]
fn off_a_steep_flank_the_walk_escalates_where_it_cannot_order() {
    use core::f64::consts::FRAC_PI_4;
    let (wall, section, conic) = steep();
    for bands in [12.0, 16.0] {
        let ts = [-1.0, FRAC_PI_4, on(&conic, FRAC_PI_4, bands), 2.5];
        let read = lanes((&wall, &section), &conic, &ts, 1.0, 0.9);
        assert!(
            matches!(read.split, Err(JoinError::Escalated { .. })),
            "Δ {bands} bands: the walk's refusal is not an escalation: {:?}",
            read.split
        );
    }
}
