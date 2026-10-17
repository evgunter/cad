//! **INTENT stage 3 PR C — a mate relates two poses**
//! (`docs/INTENT-STAGE3-SPEC.md` §4 and §10 rows 8–9).
//!
//! The coset rows here build the representatives stage 3 A owed: a
//! point's (`Spherical`), a direction's (`Parallel`) and the pure
//! translation groups meet every family in the table, and the first
//! mate that folds a point or a direction folds them.

#![allow(
    clippy::unwrap_used,
    clippy::expect_used,
    clippy::panic,
    clippy::result_large_err
)]

use editor_core::mate::coset::{Arm, Coset, FoldStop, Subgroup, intersect, intersect_subgroups};
use geom_core::Tol;
use geom_core::linalg::{Affine3, Mat3, Point3, UnitVec3, Vec3};
use geom_core::predicate::Band;

fn band() -> Band {
    Band::linear(Tol::witness()).expect("a band")
}

fn arm() -> Arm {
    Arm::of(0.0, 10.0).expect("ten metres is in range")
}

fn unit(v: Vec3<f64>) -> UnitVec3<f64> {
    UnitVec3::new(v, topo::query::DATUM_UNIT_NORM, band()).expect("a direction")
}

/// A deterministic stream of numbers in `[-1, 1)`, so a row's failure
/// names the draw that broke it.
struct Draw(u64);

impl Draw {
    fn next(&mut self) -> f64 {
        self.0 ^= self.0 << 13;
        self.0 ^= self.0 >> 7;
        self.0 ^= self.0 << 17;
        #[allow(clippy::cast_precision_loss)]
        let x = (self.0 >> 11) as f64 / (1_u64 << 53) as f64;
        2.0 * x - 1.0
    }

    fn vec(&mut self) -> Vec3<f64> {
        Vec3::new(self.next(), self.next(), self.next())
    }

    fn point(&mut self) -> Point3<f64> {
        Point3::origin() + self.vec() * 2.0
    }

    fn direction(&mut self) -> UnitVec3<f64> {
        unit(self.vec() + Vec3::new(0.0, 0.0, 0.1))
    }

    fn angle(&mut self) -> f64 {
        self.next() * 3.0
    }

    fn rigid(&mut self) -> Affine3<f64> {
        let axis = self.direction().get();
        Affine3::from_parts(Mat3::rotation_about(axis, self.angle()), self.vec() * 2.0)
    }
}

/// An element of `g`, drawn: what a coset of `g` may differ from its
/// representative by.
fn element(g: Subgroup<f64>, d: &mut Draw) -> Affine3<f64> {
    let turn = |point: Point3<f64>, axis: Vec3<f64>, angle: f64| {
        Affine3::rotation_about_axis(point, axis, angle)
    };
    let shift = |v: Vec3<f64>| Affine3::from_parts(Mat3::identity(), v);
    match g {
        Subgroup::Se3 => d.rigid(),
        Subgroup::Planar { normal } => {
            shift(d.vec().reject_from(normal.get())) * turn(d.point(), normal.get(), d.angle())
        }
        Subgroup::Cylindrical { point, direction } => {
            shift(direction.get() * d.next()) * turn(point, direction.get(), d.angle())
        }
        Subgroup::Prismatic { direction } => shift(direction.get() * d.next()),
        Subgroup::Revolute { point, direction } => turn(point, direction.get(), d.angle()),
        Subgroup::Spherical { point } => turn(point, d.direction().get(), d.angle()),
        Subgroup::Parallel { direction } => {
            shift(d.vec()) * turn(d.point(), direction.get(), d.angle())
        }
        Subgroup::Translation => shift(d.vec()),
        Subgroup::PlaneTranslation { normal } => shift(d.vec().reject_from(normal.get())),
        Subgroup::Trivial | Subgroup::Empty => Affine3::identity(),
    }
}

/// The ten proper families, each at generic parameters drawn from `d`.
fn generic(d: &mut Draw) -> Vec<Subgroup<f64>> {
    vec![
        Subgroup::Se3,
        Subgroup::Planar {
            normal: d.direction(),
        },
        Subgroup::Cylindrical {
            point: d.point(),
            direction: d.direction(),
        },
        Subgroup::Prismatic {
            direction: d.direction(),
        },
        Subgroup::Revolute {
            point: d.point(),
            direction: d.direction(),
        },
        Subgroup::Spherical { point: d.point() },
        Subgroup::Parallel {
            direction: d.direction(),
        },
        Subgroup::Translation,
        Subgroup::PlaneTranslation {
            normal: d.direction(),
        },
        Subgroup::Trivial,
    ]
}

/// The pairs whose generic draw would miss the table's special
/// verdicts: a ball on a line or at the other ball, a direction along
/// a plane's normal or a line, a line lying in a plane.
fn aligned(d: &mut Draw) -> Vec<(Subgroup<f64>, Subgroup<f64>)> {
    let u = d.direction();
    let p = d.point();
    let across = unit(u.get().cross(d.vec()));
    vec![
        (
            Subgroup::Spherical {
                point: p + u.get() * 1.5,
            },
            Subgroup::Cylindrical {
                point: p,
                direction: u,
            },
        ),
        (
            Subgroup::Spherical { point: p },
            Subgroup::Revolute {
                point: p + u.get() * 0.5,
                direction: u,
            },
        ),
        (
            Subgroup::Spherical { point: p },
            Subgroup::Spherical { point: p },
        ),
        (
            Subgroup::Parallel { direction: u },
            Subgroup::Planar { normal: u },
        ),
        (
            Subgroup::Parallel { direction: u },
            Subgroup::Cylindrical {
                point: p,
                direction: u,
            },
        ),
        (
            Subgroup::Parallel { direction: u },
            Subgroup::Revolute {
                point: p,
                direction: u,
            },
        ),
        (
            Subgroup::Parallel { direction: u },
            Subgroup::Parallel { direction: u },
        ),
        (
            Subgroup::PlaneTranslation { normal: u },
            Subgroup::Planar { normal: u },
        ),
        (
            Subgroup::PlaneTranslation { normal: u },
            Subgroup::Cylindrical {
                point: p,
                direction: across,
            },
        ),
        (
            Subgroup::PlaneTranslation { normal: u },
            Subgroup::Prismatic { direction: across },
        ),
    ]
}

/// **Two cosets sharing a pose meet in a coset of the table's residual
/// that holds it**, in either order: each coset is `g·h⁻¹·x*` for a
/// drawn element `h` of `g`, so `x*` lies in both and the intersection
/// is not empty. The representative the fold builds must lie in both
/// (the fold's own membership check) and its coset must hold `x*`
/// (checked here against `x*`'s trivial coset).
///
/// *Breaks if* a pair with a ball, a direction or the translations has
/// no representative ([`FoldStop`] refuses), or one that leaves `x*`
/// out: a ball's turn about a line taken from the line's
/// representative rather than solved puts the ball's point elsewhere,
/// and the fold refuses a pair that meets.
#[test]
fn every_pair_of_families_sharing_a_pose_folds_to_a_coset_holding_it() {
    let mut d = Draw(0x9e37_79b9_7f4a_7c15);
    let mut pairs = Vec::new();
    for _ in 0..4 {
        let row = generic(&mut d);
        let col = generic(&mut d);
        for g1 in &row {
            for g2 in &col {
                pairs.push((*g1, *g2));
            }
        }
    }
    for _ in 0..4 {
        for (g1, g2) in aligned(&mut d) {
            pairs.push((g1, g2));
            pairs.push((g2, g1));
        }
    }
    for (i, (g1, g2)) in pairs.into_iter().enumerate() {
        let x = d.rigid();
        let h1 = element(g1, &mut d);
        let h2 = element(g2, &mut d);
        let held = Coset {
            subgroup: g1,
            representative: h1.inverse() * x,
        };
        let added = Coset {
            subgroup: g2,
            representative: h2.inverse() * x,
        };
        let at = format!("pair {i}: {} against {}", g1.name(), g2.name());
        let met = intersect(held, added, band(), arm())
            .unwrap_or_else(|stop| panic!("{at}: two cosets sharing a pose refused {stop:?}"));
        let residual = intersect_subgroups(g1, g2, band(), arm()).expect("the table decides");
        assert_eq!(met.subgroup.family(), residual.family(), "{at}");
        let pinned = Coset {
            subgroup: Subgroup::Trivial,
            representative: x,
        };
        intersect(met, pinned, band(), arm())
            .unwrap_or_else(|stop| panic!("{at}: the coset leaves the shared pose out: {stop:?}"));
    }
}

/// **A ball held at the wrong radius from a line, or two balls whose
/// chords differ, refuse with the clash measured**: the representative
/// stage turns as far as an angle can carry, and the membership check
/// names what no angle reaches.
///
/// *Breaks if* the mismatch is solved away (a turn that scales, or a
/// representative taken from one side alone that the other's check
/// never measures) or refuses without a measured clash.
#[test]
fn a_ball_off_its_radius_or_chord_refuses_with_the_clash_measured() {
    let z = unit(Vec3::new(0.0, 0.0, 1.0));
    let line = Coset {
        subgroup: Subgroup::Cylindrical {
            point: Point3::origin(),
            direction: z,
        },
        representative: Affine3::identity(),
    };
    // The ball's own point sits 1 m off the axis; the line's coset
    // carries it to a point 1 m off; the ball asks for 2 m.
    let ball = Coset {
        subgroup: Subgroup::Spherical {
            point: Point3::new(2.0, 0.0, 0.5),
        },
        representative: Affine3::from_parts(Mat3::identity(), Vec3::new(1.0, 0.0, 0.0)),
    };
    for (held, added) in [(ball, line), (line, ball)] {
        match intersect(held, added, band(), arm()) {
            Err(FoldStop::Clash { clash, .. }) => {
                assert!(clash.deviation().is_some_and(|m| m > 0.5), "{clash:?}");
            }
            other => panic!("a ball off its radius: {other:?}"),
        }
    }
    let ball_at = |point: Point3<f64>, shift: Vec3<f64>| Coset {
        subgroup: Subgroup::Spherical { point },
        representative: Affine3::from_parts(Mat3::identity(), shift),
    };
    // Copy points 1 m apart; the target points 3 m apart.
    let first = ball_at(Point3::origin(), Vec3::zero());
    let second = ball_at(Point3::new(0.0, 3.0, 0.0), Vec3::new(0.0, 2.0, 0.0));
    match intersect(first, second, band(), arm()) {
        Err(FoldStop::Clash { clash, .. }) => {
            assert!(clash.deviation().is_some_and(|m| m > 1.0), "{clash:?}");
        }
        other => panic!("two balls whose chords differ: {other:?}"),
    }
}
