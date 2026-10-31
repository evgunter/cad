//! **The boolean's turn order and the split's walk order are one order
//! along a section conic.** The boolean ranks a germ's partners by
//! [`nearer_along`] — the half-turn ([`germ_arm`]), then the turn from
//! the incumbent ([`super::travel`]); the split pairs a curved face's
//! crossings in walk order ([`crate::splitting::join::conic_pairs_of_sites`],
//! the eccentric anomaly through `sort_along_runs`). Each row feeds one
//! set of sites on one conic to both, walking it either way, and asks
//! for one cyclic order.
//!
//! The sites, listed counter-clockwise from the germ: one mid-arc; a
//! pair thirty bands apart on the flank; a pair straddling the conic's
//! own seam (`θ = ±π`, where the split's walk starts); a pair straddling
//! the germ's half-turn (one `Ahead`, one `Behind`); and two behind.
//! Each straddling pair sits fifteen bands either side, clear of the
//! band's escalation edge (ten) and of the turn's off-conic slack. The
//! lanes take them in a scrambled insertion order, so neither inherits
//! the answer from its input.
//!
//! The circle on a sphere is also the coaxial cylinder × sphere section
//! the table carries ([`geom_brep::pair_section`]), so there the carried
//! lane's foot parameter ([`along_facing`]) is held too, with a pair
//! straddling the carried circle's own seam. No pair the table carries
//! is an ellipse: the ellipse rows hold two orders, not three.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use super::super::section_table::SectionTable;
use super::super::{Branch, HalfGerm, Locus};
use super::{Faced, GermArm, Reach, Turn, along_facing, germ_arm, nearer_along};
use crate::HalfEdgeKey;
use geom::{Curve3, Surface};
use geom_core::{Band, Point3, Tol, Vec3};

fn band() -> Band {
    Band::linear(Tol::witness()).unwrap()
}

/// The conic's `(centre, axis)`, the frame its turn is read about.
fn frame(conic: &Curve3<f64>) -> (Point3<f64>, Vec3<f64>) {
    match *conic {
        Curve3::Ellipse { center, axis, .. } | Curve3::Circle { center, axis, .. } => {
            (center, axis)
        }
        _ => panic!("not a conic: {conic:?}"),
    }
}

/// The point at eccentric anomaly `t` and the unit tangent `sense`
/// (`±1`) of counter-clockwise there.
fn at(conic: &Curve3<f64>, t: f64, sense: f64) -> (Point3<f64>, Vec3<f64>) {
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

/// The candidates ranked as [`super::partners`] picks among them: the
/// nearest by `nearer_along` first, the incumbent keeping a tie, again
/// over the rest.
fn ranked(mut left: Vec<(usize, Reach<f64>)>) -> Vec<usize> {
    let mut out = Vec::new();
    while !left.is_empty() {
        let mut best = 0;
        for k in 1..left.len() {
            if nearer_along(left[k].1, left[best].1, band()).unwrap() {
                best = k;
            }
        }
        out.push(left.remove(best).0);
    }
    out
}

/// One conic, walked each way: the split's walk, the boolean's turn,
/// and (`carried`) the carried lane's foot parameter, each as a cycle of
/// site indices from the germ, against the sites' own order.
fn hold(
    name: &str,
    wall: &Surface<f64>,
    conic: &Curve3<f64>,
    ts: &[f64],
    carried: Option<&geom_brep::CarriedSection<f64>>,
) {
    let n = ts.len();
    let split_conic = crate::chord_join::section_conic(conic.clone()).unwrap();
    for (sense, way) in [(1.0, "ccw"), (-1.0, "cw")] {
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
        let site = |i: usize| at(conic, ts[i], sense);

        let input: Vec<_> = slots
            .iter()
            .map(|&i| {
                let (p, ahead) = site(i);
                (p, down[i], if down[i] { ahead } else { -ahead })
            })
            .collect();
        let pairs =
            crate::splitting::join::conic_pairs_of_sites(wall, &split_conic, &input, band())
                .unwrap_or_else(|e| panic!("{name} {way}: the split's walk refused: {e:?}"));
        let walked: Vec<usize> = pairs
            .iter()
            .flat_map(|&(a, b)| [slots[a], slots[b]])
            .collect();
        let split = from_germ(&walked);

        let (g, dir) = site(0);
        let partners = (1..n).map(|i| (i, site(i))).collect::<Vec<_>>();
        let turned = partners
            .iter()
            .map(|&(i, (p, ahead))| {
                let turn = Turn::facing(frame(conic), (g, dir), (p, -ahead), band())
                    .unwrap()
                    .expect("a partner facing back along the walk faces the germ");
                let reach = Reach {
                    arm: germ_arm(Some(turn), band()).unwrap(),
                    chord: (p - g).norm(),
                    turn: Some(turn),
                    along: None,
                };
                (i, reach)
            })
            .collect();
        let boolean: Vec<usize> = core::iter::once(0).chain(ranked(turned)).collect();

        assert_eq!(
            boolean, split,
            "{name} {way}: the boolean's turn order and the split's walk order differ"
        );
        assert_eq!(
            split, walk,
            "{name} {way}: both lanes agree on an order that is not the sites' own"
        );

        if let Some(section) = carried {
            let footed = feet(section, (g, dir), &partners);
            let carried: Vec<usize> = core::iter::once(0).chain(ranked(footed)).collect();
            assert_eq!(
                carried, split,
                "{name} {way}: the carried lane's foot order and the split's walk order differ"
            );
        }
    }
}

/// Each partner as the carried lane reaches it from the germ: its foot
/// parameter's travel along the component ([`along_facing`]).
fn feet(
    section: &geom_brep::CarriedSection<f64>,
    (g, dir): (Point3<f64>, Vec3<f64>),
    partners: &[(usize, (Point3<f64>, Vec3<f64>))],
) -> Vec<(usize, Reach<f64>)> {
    let pillow = crate::fixtures::pillow(Tol::witness());
    let table = SectionTable::default();
    let mut halves = slotmap::SlotMap::<HalfEdgeKey, ()>::with_key();
    let component = section.foot(g, band()).unwrap().component;
    let mut germ = |dir| HalfGerm {
        he: halves.insert(()),
        a_face: pillow.face_a,
        b_face: pillow.face_b,
        a_locus: Locus::InFace(pillow.face_a),
        b_locus: Locus::InFace(pillow.face_b),
        dir,
        branch: Branch::Component(component),
    };
    let from = germ(dir);
    partners
        .iter()
        .map(|&(i, (p, ahead))| {
            let to = germ(-ahead);
            let bodies = (&table, &pillow.body, &pillow.body);
            let along = match along_facing(bodies, section, (&from, g), (&to, p), band()) {
                Ok(Faced::Along(along)) => along,
                Ok(_) => panic!("site {i}: a partner facing back does not face the germ"),
                Err(e) => panic!("site {i}: {e:?}"),
            };
            let reach = Reach {
                arm: GermArm::Ahead,
                chord: (p - g).norm(),
                turn: None,
                along: Some(along),
            };
            (i, reach)
        })
        .collect()
}

/// A tilted ellipse on a cylinder (plane × cylinder): aspect 2, so the
/// turn's flank reading and its off-conic slack are both live.
#[test]
fn a_tilted_ellipse_on_a_cylinder_walks_one_order_in_both_lanes() {
    let origin = Point3::new(0.3, -0.2, 0.0);
    let wall = Surface::Cylinder {
        origin,
        axis: Vec3::unit_z(),
        radius: 1.0,
        u_ref: Vec3::unit_x(),
    };
    let (s, c) = (60f64.to_radians().sin(), 60f64.to_radians().cos());
    let normal = Vec3::new(s * 0.7f64.cos(), s * 0.7f64.sin(), c);
    let plane = Surface::Plane {
        origin: origin + Vec3::unit_z() * 0.5,
        normal,
        u_ref: Vec3::unit_z().cross(normal).normalize(),
    };
    let reach = geom_brep::Reach::Measured {
        at: origin,
        lever: 4.0,
    };
    let conic = match geom_brep::plane_cylinder_section(&plane, &wall, &reach, band()).unwrap() {
        geom_brep::PlaneCylinderSection::TiltedEllipse(c) => c,
        other => panic!("{other:?}"),
    };
    hold("ellipse", &wall, &conic, &sites(&conic, 0.4, &[]), None);
}

/// A circle on a sphere (plane × sphere), the coaxial cylinder × sphere
/// section's circle too, so the carried lane's foot order is held on it.
#[test]
fn a_circle_on_a_sphere_walks_one_order_in_all_three_lanes() {
    let center = Point3::new(0.1, 0.2, -0.3);
    let n = Vec3::new(1.0, -2.0, 2.0).normalize();
    let u_ref = Vec3::unit_z().cross(n).normalize();
    let sphere = Surface::Sphere {
        center,
        radius: 1.5,
        axis: Vec3::unit_z(),
        u_ref: Vec3::unit_x(),
    };
    let plane = Surface::Plane {
        origin: center + n * 0.6,
        normal: n,
        u_ref,
    };
    let conic = match geom_brep::plane_sphere_section(&plane, &sphere, band()).unwrap() {
        geom_brep::PlaneSphereSection::Circle(c) => c,
        other => panic!("{other:?}"),
    };
    let cylinder = Surface::Cylinder {
        origin: center,
        axis: n,
        radius: (1.5f64 * 1.5 - 0.6 * 0.6).sqrt(),
        u_ref,
    };
    let section = match geom_brep::pair_section(&cylinder, &sphere, band()).unwrap() {
        geom_brep::SectionRead::Built(s) => s.carry(None, band()).unwrap(),
        geom_brep::SectionRead::Elsewhere(route) => panic!("{route:?}"),
    };
    let component = section.foot(conic.eval(0.4), band()).unwrap().component;
    let carried_seam = section.components()[component].carrier.eval(0.0);
    let split_conic = crate::chord_join::section_conic(conic.clone()).unwrap();
    let ts = sites(&conic, 0.4, &[split_conic.param(carried_seam)]);
    hold("circle", &sphere, &conic, &ts, Some(&section));
}
