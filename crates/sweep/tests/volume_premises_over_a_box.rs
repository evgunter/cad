//! **A curved face's closed-form premises are decided once, at the
//! scalar its body lives at** (`topo::props`' `rederive_faces`,
//! `walked_closed_form`).
//!
//! Check 7 reads a volume's sign off an interval re-derivation of the
//! role walk's faces. At a scalar that rounds, the re-derivation runs
//! each closed form again at the interval scalar over the face's
//! geometry lifted. At a scalar whose own sums are enclosures it reads
//! a curved face off the walk's sums: run again, the closed form would
//! ask each premise of the value channel alone, where a rim radius
//! less its wall's is `r − r`, as wide as the box.
//!
//! No door builds a curved body over a box today (each refuses at edge
//! certification), so a fixture is built at an exact parameter and its
//! stored geometry restated (`topo::test_support::restate_geometry`):
//! a radius as a boxed parameter, or a datum written wrong.
//!
//! A cone has no row. Its walk refuses over a box at
//! `props_chart_junction_closed` before any premise is read again
//! (`work/nearpoint/a-cones-chart-junction-gap-is-wide-over-a-box-on-its-axial-position.md`),
//! and so do a cylinder bounded by rulings and a sphere of boxed
//! radius, at `props_meridian_on_surface`
//! (`work/boxfinish/a-radial-distance-and-a-rulings-azimuth-recover-the-radius-through-a-root.md`)
//! and `props_sphere_circle_fit`
//! (`work/boxfinish/a-spheres-circle-fit-recovers-the-radius-through-a-root.md`).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use core::cell::Cell;

use geom_core::{
    CertifiedBounds, Interval, ParamSymbol, Point2, Real, RigidMotion, Sym, SymBudget, Tol, Vec3,
};
use sweep::Revolution;
use sweep::test_support::{ball_poled_y, revolved_about_y_at};
use topo::test_support::{
    Stored, closed_form_reads, restate_geometry, seamless_drum, volume_role_reading,
};
use topo::{Body, CertifiedPolicy};

type Boxed = Sym<Interval>;

/// The nominal radius: dyadic, and equal to no other stored number of a
/// fixture, so a stored scalar that reads it is the radius.
const R: f64 = 0.8125;
/// Where the placed fixtures sit along their axis.
const A: f64 = 3.0;

/// One stored value restated: in `arena` (every arena where `None`), a
/// scalar reading `key` becomes `to`.
type Restated<T> = (Option<Stored>, f64, T);

fn near<T: CertifiedBounds>(x: T, key: f64) -> bool {
    (x.lo() - key).abs() <= 1e-12 && (x.hi() - key).abs() <= 1e-12
}

/// `body` with each stored scalar the table names restated, and how
/// many it restated.
fn restated<T: Real + CertifiedBounds>(
    mut body: Body<T>,
    table: &[Restated<T>],
) -> (Body<T>, usize) {
    let hits = Cell::new(0usize);
    restate_geometry(&mut body, |arena, x| {
        for &(only, key, to) in table {
            if only.is_none_or(|a| a == arena) && near(x, key) {
                hits.set(hits.get() + 1);
                return to;
            }
        }
        x
    });
    (body, hits.get())
}

fn boxed<Out>(read: impl FnOnce(&dyn Fn(&str, f64, f64) -> Boxed) -> Out) -> Out {
    let budget = SymBudget {
        max_terms: 64,
        max_degree: 8,
    };
    geom_core::sym::with_session(budget, || {
        read(&|name, lo, hi| {
            if lo == hi {
                return Boxed::from_f64(lo);
            }
            Sym::param_over(
                ParamSymbol::new(test_utils::symbol_id(name)),
                Interval::from_bounds(lo, hi),
                lo,
                hi,
            )
        })
    })
    .0
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum Wall {
    Cylinder,
    Torus,
    Sphere,
}

/// The wall's body at the nominal, and its table at parameter `p`.
fn fixture<T: CertifiedPolicy + CertifiedBounds>(wall: Wall, p: T, tol: Tol) -> (Body<T>, usize) {
    let c = T::from_f64;
    match wall {
        Wall::Cylinder => restated(
            seamless_drum::<T>(R, 1.0, tol),
            &[(None, R, p), (None, -R, -p)],
        ),
        Wall::Torus => restated(
            revolved_about_y_at::<T>(
                vec![
                    (Point2::new(c(2.0 - R), c(0.0)), c(1.0)),
                    (Point2::new(c(2.0 + R), c(0.0)), c(1.0)),
                ],
                Revolution::Full,
                tol,
            ),
            &[
                (None, R, p),
                (None, -R, -p),
                (None, 2.0 + R, c(2.0) + p),
                (None, 2.0 - R, c(2.0) - p),
                (None, -2.0 - R, c(-2.0) - p),
                (None, -2.0 + R, c(-2.0) + p),
            ],
        ),
        Wall::Sphere => restated(
            ball_poled_y::<T>(c(R), Vec3::new(c(0.0), c(A), c(0.0)), tol),
            &[
                (None, A, p),
                (None, A + R, p + c(R)),
                (None, A - R, p - c(R)),
            ],
        ),
    }
}

fn nominal(wall: Wall) -> f64 {
    match wall {
        Wall::Cylinder | Wall::Torus => R,
        Wall::Sphere => A,
    }
}

const WALLS: [Wall; 3] = [Wall::Cylinder, Wall::Torus, Wall::Sphere];

/// The premise the closed form run again loses over a box on `wall`.
fn lost(wall: Wall) -> &'static str {
    match wall {
        Wall::Cylinder | Wall::Torus => "props_rim_fit",
        Wall::Sphere => "props_sphere_circle_on",
    }
}

/// A box on the parameter, a tenth wide.
const HALF: f64 = 0.05;

/// **A curved wall over a box wider than the band finishes its volume
/// role walk at the symbolic box scalar, and the volume it reads holds
/// every member's**: a seamless cylinder and a torus of boxed radius, a
/// sphere of boxed position, each against the same body at `f64` at the
/// box's two ends and its middle.
///
/// Each fixture is one the closed form run again refuses, which the row
/// reads first (`closed_form_reads`), so it cannot pass on a body the
/// change does not touch.
///
/// Red where `walked_closed_form` answers `None` at every scalar (the
/// closed form run again over the lifted geometry): the walk refuses
/// `VolumeUncomputable` at `props_rim_fit` `[-0.1, 0.1]` on the
/// cylinder.
#[test]
fn a_boxed_curved_wall_reads_a_volume_that_holds_every_members() {
    let tol = Tol::witness();
    for wall in WALLS {
        let n = nominal(wall);
        let members: Vec<(f64, f64)> = [n - HALF, n, n + HALF]
            .into_iter()
            .map(|p| {
                let (body, hits) = fixture::<f64>(wall, p, tol);
                assert!(hits > 0, "{wall:?}: the member's table restated nothing");
                let (role, volume) = volume_role_reading(&body, tol)
                    .unwrap_or_else(|e| panic!("{wall:?} at f64 {p}: {e:?}"));
                assert_eq!(role, Some(topo::ShellRole::Outer), "{wall:?} at f64 {p}");
                volume
            })
            .collect();
        boxed(|param| {
            let (body, hits) = fixture::<Boxed>(wall, param("boxed", n - HALF, n + HALF), tol);
            assert!(hits > 0, "{wall:?}: the box's table restated nothing");
            let reads = closed_form_reads(&body, tol).unwrap();
            assert!(
                reads.iter().any(|read| matches!(
                    &read.lifted,
                    Err(refusal) if format!("{refusal:?}").contains(lost(wall))
                )),
                "{wall:?}: the closed form run again refuses no face at {}: {reads:?}",
                lost(wall)
            );
            let (role, (lo, hi)) = volume_role_reading(&body, tol)
                .unwrap_or_else(|e| panic!("{wall:?} over the box: the walk refuses: {e:?}"));
            assert_eq!(role, Some(topo::ShellRole::Outer), "{wall:?} over the box");
            for (member, &(m_lo, m_hi)) in members.iter().enumerate() {
                assert!(
                    lo <= m_lo && m_hi <= hi,
                    "{wall:?}: the box's volume [{lo}, {hi}] misses member {member}'s \
                     [{m_lo}, {m_hi}]"
                );
            }
        });
    }
}

/// **Where both read, the walk's sums are the closed form run again, to
/// the bit**: every curved face's flux and area, over a box an eighth of
/// `ε` wide (inside the band, so the lifted run decides its premises),
/// at the interval scalar and at the symbolic box scalar. The walk's sums
/// are that arithmetic, so reading them is no second enclosure.
///
/// Red where `walked_closed_form` pads a sum by anything but its run's
/// own pad, or lifts the area in the flux's place.
#[test]
fn the_walks_sums_are_the_closed_form_run_again_where_both_read() {
    let tol = Tol::witness();
    let half = 0.125 * tol.eps();
    let same = |what: &str, reads: Vec<topo::test_support::ClosedFormRead>| {
        assert!(!reads.is_empty(), "{what}: no curved closed-form face");
        for read in reads {
            let lifted = read
                .lifted
                .unwrap_or_else(|e| panic!("{what}: the lifted run refuses in band: {e:?}"));
            let walked = read.walked.expect("an exact-witness scalar reads the walk");
            for (walked, lifted) in walked.iter().zip(lifted) {
                assert!(
                    walked.0.to_bits() == lifted.0.to_bits()
                        && walked.1.to_bits() == lifted.1.to_bits(),
                    "{what} {:?}: walked {walked:?}, lifted {lifted:?}",
                    read.face
                );
            }
        }
    };
    for wall in WALLS {
        let n = nominal(wall);
        let (body, _) = fixture::<Interval>(wall, Interval::from_bounds(n - half, n + half), tol);
        same(
            &format!("{wall:?} at the interval scalar"),
            closed_form_reads(&body, tol).unwrap(),
        );
        boxed(|param| {
            let (body, _) = fixture::<Boxed>(wall, param("narrow", n - half, n + half), tol);
            same(
                &format!("{wall:?} at the symbolic scalar"),
                closed_form_reads(&body, tol).unwrap(),
            );
        });
        let (body, _) = fixture::<f64>(wall, n, tol);
        assert!(
            closed_form_reads(&body, tol)
                .unwrap()
                .iter()
                .all(|read| read.walked.is_none() && read.lifted.is_ok()),
            "{wall:?}: `f64` rounds, so its faces are read by the closed form run again"
        );
    }
}

/// How far a false premise's datum is written off: a thousandth, far
/// past the band at every `ε` row.
const OFF: f64 = 1.0e-3;

/// The seamless drum placed off the origin, so its axis has coordinates
/// no other datum shares.
fn placed_drum<T: CertifiedPolicy + CertifiedBounds>(tol: Tol) -> Body<T> {
    let c = T::from_f64;
    topo::transform_rigid(
        &seamless_drum::<T>(R, 1.0, tol),
        &RigidMotion::translation(Vec3::new(c(7.0), c(11.0), c(13.0))).unwrap(),
    )
    .unwrap()
}

/// A cylinder through the revolve door: its wall is two faces bounded
/// by rulings, one at `x = −R`.
fn ruled_cylinder<T: CertifiedPolicy + CertifiedBounds>(tol: Tol) -> Body<T> {
    let c = T::from_f64;
    let at = |x: f64, y: f64| (Point2::new(c(x), c(y)), c(0.0));
    revolved_about_y_at::<T>(
        vec![at(0.0, 0.0), at(R, 0.0), at(R, 1.0), at(0.0, 1.0)],
        Revolution::Full,
        tol,
    )
}

/// A body whose premise `named` is false by [`OFF`], with the radius
/// `r` where its scalar boxes one.
fn false_premise<T: CertifiedPolicy + CertifiedBounds>(
    named: &str,
    r: T,
    tol: Tol,
) -> (Body<T>, usize) {
    let c = T::from_f64;
    match named {
        // The wall's radius is not its rims'.
        "props_rim_fit" => restated(
            seamless_drum::<T>(R, 1.0, tol),
            &[
                (Some(Stored::Surface), R, r + c(OFF)),
                (None, R, r),
                (None, -R, -r),
            ],
        ),
        // Both rims' centres stand off the wall's axis.
        "props_cylinder_circle_off_axis" => restated(
            placed_drum::<T>(tol),
            &[
                (Some(Stored::Curve), 7.0, c(7.0 + OFF)),
                (None, R, r),
                (None, 7.0 + R, c(7.0) + r),
            ],
        ),
        // One ruling stands outside the wall.
        "props_meridian_on_surface" => restated(
            ruled_cylinder::<T>(tol),
            &[(Some(Stored::Curve), -R, c(-R - OFF))],
        ),
        other => panic!("no fixture for {other}"),
    }
}

/// **A face whose premise is false at a member is refused at every
/// scalar, the premise named**: a wall whose radius is not its rims'
/// (`props_rim_fit`), rims whose centres stand off the axis
/// (`props_cylinder_circle_off_axis`) and a ruling off the wall
/// (`props_meridian_on_surface`), each by a thousandth, at `f64`, at the
/// interval scalar and at the symbolic box scalar. The first two are
/// read over a box on the radius; the ruling at an exact parameter,
/// since a ruled wall's true premise is not yet read over a box
/// (the module's last paragraph).
///
/// `props_rim_center_on_axis` has no row of its own: it measures the
/// rim centre's distance from the axis, the quantity
/// `props_cylinder_circle_off_axis` is asked for first, so no body is
/// false on it alone.
///
/// Red, for each premise, where `cylinder_boundary` does not ask it
/// (its `require_zero` taken out): the body reads a volume.
#[test]
fn a_false_premise_is_refused_at_every_scalar() {
    let tol = Tol::witness();
    let refused = |what: &str, named: &str, read: Result<_, topo::MassPropsError>| {
        let Err(refusal) = read else {
            panic!("{what}: a body false on {named} reads a volume: {read:?}");
        };
        assert!(
            format!("{refusal:?}").contains(named),
            "{what}: refused, but not at {named}: {refusal:?}"
        );
    };
    for named in [
        "props_rim_fit",
        "props_cylinder_circle_off_axis",
        "props_meridian_on_surface",
    ] {
        let (body, hits) = false_premise::<f64>(named, R, tol);
        assert!(hits > 0, "{named}: the table restated nothing");
        refused("f64", named, volume_role_reading(&body, tol));
        let (body, _) = false_premise::<Interval>(named, Interval::from_f64(R), tol);
        refused(
            "the interval scalar",
            named,
            volume_role_reading(&body, tol),
        );
        boxed(|param| {
            let r = if named == "props_meridian_on_surface" {
                param("exact", R, R)
            } else {
                param("boxed", R - HALF, R + HALF)
            };
            let (body, _) = false_premise::<Boxed>(named, r, tol);
            refused(
                "the symbolic scalar",
                named,
                volume_role_reading(&body, tol),
            );
        });
    }
}

/// **A rim that fits its wall only at the nominal is not accepted over a
/// box that separates them**: the rims' radius and the wall's are two
/// parameters over one box. Their difference is no identity, so the
/// symbolic scalar leaves `props_rim_fit` open, and the walk refuses
/// before anything is read off it. The same body at the nominal, where
/// the two are equal, reads its volume.
///
/// Red where an undecided premise passes at the walk's scalar
/// (`require_zero` taking an escalation as zero): the body reads a
/// volume over the box.
#[test]
fn a_rim_that_fits_only_at_the_nominal_is_not_accepted_over_a_box() {
    let tol = Tol::witness();
    let table = |rims: Boxed, wall: Boxed| {
        [
            (Some(Stored::Surface), R, wall),
            (None, R, rims),
            (None, -R, -rims),
        ]
    };
    boxed(|param| {
        let (body, hits) = restated(
            seamless_drum::<Boxed>(R, 1.0, tol),
            &table(param("exact", R, R), param("exact", R, R)),
        );
        assert!(hits > 0, "the table restated nothing");
        assert!(
            volume_role_reading(&body, tol).is_ok(),
            "equal at the nominal, the body reads its volume"
        );
        let (body, _) = restated(
            seamless_drum::<Boxed>(R, 1.0, tol),
            &table(
                param("rims", R - HALF, R + HALF),
                param("wall", R - HALF, R + HALF),
            ),
        );
        let read = volume_role_reading(&body, tol);
        let Err(refusal) = &read else {
            panic!("two radii over one box read a volume: {read:?}");
        };
        let text = format!("{refusal:?}");
        assert!(
            text.contains("Escalated") && text.contains("props_rim_fit"),
            "refused, but not as an open props_rim_fit: {text}"
        );
    });
}
