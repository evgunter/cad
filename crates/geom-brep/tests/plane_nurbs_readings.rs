//! **The plane × NURBS lane's sections, read on the 1×2 trace**: each
//! fixture's section read whole by `ImplicitSection` with the plane as
//! its partner (C3's residual × chart trace), asserted as the plane's
//! retired ℝ⁴ reading read it. Before that lane was retired, a
//! differential row read every fixture here through both lanes at
//! ε 1e-9, 1e-6 and 1e-12 and found them the same section, but for one:
//! the bent-path flat wall at 1e-12, which the ℝ⁴ lane decided (one
//! branch, `v = 0` side to `v = 1` side) and the 1×2 lane refuses
//! `TubeNotOneArc { cause: Short }`
//! (`work/ssiarith/ssi-limb-three-refuses-a-bent-chart-path-line-at-eps-1e-12.md`).
//! These rows keep that evidence: the readings the two lanes agreed on.
//!
//! The fixtures are the plane × NURBS lane's own: the certifiable and
//! the inflected wall under the tilted cutting plane, a dome's level
//! loops, the far wall the ε-rounding rows place from 20 m to 520 m off
//! the origin, and the bent-path flat wall, each at the run's ε (the
//! suite runs 1e-9, 1e-6 and 1e-12).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use geom::{NurbsSurface, Surface};
use geom_brep::ssi::BranchEnd;
use geom_brep::{ImplicitSection, SsiDomain, SsiError, SsiOutcome};
use geom_core::spline::KnotVector;
use geom_core::{Point3, Vec3};

use crate::shared::tol::{band, eps};

/// The wall of `m5_pr7_ssi`: a cubic in `u` through four columns, linear
/// in `v`, extruded `0.8` in `z`.
fn wall_from_cols(cols: [(f64, f64); 4]) -> NurbsSurface<f64> {
    let ku = KnotVector::clamped(vec![0.0, 0.0, 0.0, 0.0, 1.0, 1.0, 1.0, 1.0], 3).unwrap();
    let kv = KnotVector::clamped(vec![0.0, 0.0, 1.0, 1.0], 1).unwrap();
    let mut control = Vec::with_capacity(8);
    for (x, y) in cols {
        control.push(Point3::new(x, y, 0.0));
        control.push(Point3::new(x, y, 0.8));
    }
    NurbsSurface::new(ku, kv, control, vec![1.0; 8]).unwrap()
}

const CERTIFIABLE: [(f64, f64); 4] = [(0.0, 0.0), (0.35, 0.14), (0.70, 0.24), (1.05, 0.30)];
const INFLECTED: [(f64, f64); 4] = [(0.0, 0.0), (0.35, 0.18), (0.70, -0.12), (1.05, 0.04)];

/// The tilted plane at mid height, shifted `x` along the first axis.
fn cutting_plane(x: f64) -> Surface<f64> {
    let n = Vec3::new(0.0, 0.25, 1.0);
    let n = n / n.norm();
    let u = Vec3::new(1.0, 0.0, 0.0);
    let u = (u - n * u.dot(n)) / (u - n * u.dot(n)).norm();
    Surface::Plane {
        origin: Point3::new(x, 0.0, 0.4),
        normal: n,
        u_ref: u,
    }
}

fn wall_domain() -> SsiDomain {
    SsiDomain {
        center: Point3::new(0.5, 0.0, 0.4),
        half_extent: 2.0,
        extent: 1.5,
        floor_scale: 1.0,
    }
}

/// A biquadratic dome whose middle control point stands at `y = −1`.
fn dome() -> NurbsSurface<f64> {
    let k = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
    let control = (0..9)
        .map(|n| {
            let (i, j) = (f64::from(n / 3), f64::from(n % 3));
            let y = if n == 4 { -1.0 } else { 0.0 };
            Point3::new(i / 2.0, y, j / 2.0)
        })
        .collect();
    NurbsSurface::new(k.clone(), k, control, vec![1.0; 9]).unwrap()
}

fn level(y: f64) -> Surface<f64> {
    Surface::Plane {
        origin: Point3::new(0.5, y, 0.5),
        normal: Vec3::new(0.0, 1.0, 0.0),
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    }
}

/// The flat wall `z = α·x` over `y ∈ [0, ½]` whose chart path bends
/// (`m5_pr7_ssi`'s `bent_path_flat_wall`): `x = X·[(s − ½) − κh(s − ½)² −
/// μh]`, `h = 4t(1 − t)`, biquadratic, cut by `z = 0` in the line
/// `x = z = 0`.
fn bent_path_flat_wall(alpha: f64, width: f64, kappa: f64, mu: f64) -> NurbsSurface<f64> {
    let lin = [-0.5, 0.0, 0.5];
    let square = [0.25, -0.25, 0.25];
    let hump = [0.0, 2.0, 0.0];
    let k = KnotVector::clamped(vec![0.0, 0.0, 0.0, 1.0, 1.0, 1.0], 2).unwrap();
    let mut control = Vec::with_capacity(9);
    for i in 0..3 {
        for (j, h) in hump.iter().enumerate() {
            let x = width * (lin[i] - kappa * square[i] * h - mu * h);
            control.push(Point3::new(x, 0.25 * f64::from(j as u8), alpha * x));
        }
    }
    NurbsSurface::new(k.clone(), k, control, vec![1.0; 9]).unwrap()
}

fn dome_domain() -> SsiDomain {
    SsiDomain {
        center: Point3::new(0.5, -0.2, 0.5),
        half_extent: 2.0,
        extent: 1.0,
        floor_scale: 1.0,
    }
}

/// What the section is, as the two readings must agree on it: decided or
/// refused (by the refusal's kind), and if decided, its branches' ends
/// (sorted) and its regions' count.
#[derive(Debug, PartialEq)]
enum Read {
    Decided { ends: Vec<String>, regions: usize },
    Refused(String),
}

fn read(r: Result<SsiOutcome, SsiError>) -> Read {
    match r {
        Ok(o) => {
            let mut ends: Vec<String> = o
                .branches
                .iter()
                .map(|b| match b.end {
                    BranchEnd::Closed => "loop".to_owned(),
                    BranchEnd::Rim { from, to } => {
                        let mut s = [from, to].map(|e| format!("{:?}", e.side));
                        s.sort();
                        format!("rim {} {}", s[0], s[1])
                    }
                    BranchEnd::Cut => "cut".to_owned(),
                })
                .collect();
            ends.sort();
            Read::Decided {
                ends,
                regions: o.boundary.len(),
            }
        }
        Err(e) => {
            let d = format!("{e:?}");
            let cut = d.find(['(', '{', ' ']).unwrap_or(d.len());
            Read::Refused(d[..cut].to_owned())
        }
    }
}

/// One rim branch from the wall's `side_a` to its `side_b`, as `read` writes it.
fn rim(fixed: &str) -> Read {
    Read::Decided {
        ends: vec![format!(
            "rim ChartSide {{ fixed: {fixed}, end: High }} ChartSide {{ fixed: {fixed}, end: Low }}"
        )],
        regions: 0,
    }
}

fn one_loop() -> Read {
    Read::Decided {
        ends: vec!["loop".to_owned()],
        regions: 0,
    }
}

/// **Each fixture reads the section the plane's ℝ⁴ lane read**, the
/// one move excepted (module docs). The far wall at 520 m refuses by its
/// scale where ε is too fine for its coordinates (1e-12), as it did; the
/// bent-path flat wall refuses its side as possibly tangent at 1e-9 and
/// 1e-6, as it did, and at 1e-12 refuses limb 3's one-arc proof short of
/// the carrier's end. Red against a plane partner whose form, exclusion,
/// side reading or march decides another section.
#[test]
fn the_plane_reads_the_section_its_lane_read() {
    let b = band();
    let fine = eps() < 1e-10;
    let bent_plane = Surface::Plane {
        origin: Point3::new(0.0, 0.0, 0.0),
        normal: Vec3::new(0.0, 0.0, 1.0),
        u_ref: Vec3::new(1.0, 0.0, 0.0),
    };
    let bent_domain = SsiDomain {
        center: Point3::new(0.0, 0.0, 0.0),
        half_extent: 1.0,
        extent: 1.0,
        floor_scale: 1.0,
    };
    let mut fixtures: Vec<(String, Surface<f64>, NurbsSurface<f64>, SsiDomain, Read)> = vec![
        (
            "certifiable wall".into(),
            cutting_plane(0.0),
            wall_from_cols(CERTIFIABLE),
            wall_domain(),
            rim("U"),
        ),
        (
            "inflected wall".into(),
            cutting_plane(0.0),
            wall_from_cols(INFLECTED),
            wall_domain(),
            rim("U"),
        ),
    ];
    for y in [-0.125, -0.05, -0.2] {
        fixtures.push((
            format!("dome level {y}"),
            level(y),
            dome(),
            dome_domain(),
            one_loop(),
        ));
    }
    for x in [20.0, 60.0, 500.0, 520.0] {
        let want = if x > 510.0 && fine {
            Read::Refused("SettlingUnresolvable".into())
        } else {
            rim("U")
        };
        fixtures.push((
            format!("far wall at {x} m"),
            cutting_plane(x),
            wall_from_cols(CERTIFIABLE.map(|(u, v)| (u + x, v))),
            wall_domain(),
            want,
        ));
    }
    fixtures.push((
        "flat wall whose chart path bends".into(),
        bent_plane,
        bent_path_flat_wall(1e-6, 0.01, 0.8, 0.2),
        bent_domain,
        Read::Refused(
            if fine {
                "TubeNotOneArc"
            } else {
                "BoundaryTangent"
            }
            .into(),
        ),
    ));
    for (what, plane, wall, domain, want) in &fixtures {
        let got = read(ImplicitSection::of(plane, wall, *domain, b).and_then(ImplicitSection::all));
        assert_eq!(&got, want, "{what} at ε {:e}", b.zero());
    }
}

/// **The plane's doors read a plane only.** `plane_nurbs_ssi` and
/// `trace_plane_nurbs_uncertified` delegate to the one 1×2 handle, which
/// reads a cylinder too. Handed an upright cylinder through the
/// certifiable wall, a partner `ImplicitSection` would trace, each door
/// still refuses with `SsiError::WrongLane` naming the plane.
#[test]
fn the_plane_doors_refuse_a_curved_partner() {
    let wall = wall_from_cols(CERTIFIABLE);
    let cylinder = Surface::Cylinder {
        origin: Point3::new(0.5, 0.2, 0.0),
        axis: Vec3::unit_z(),
        radius: 0.2,
        u_ref: Vec3::unit_x(),
    };
    let traced = ImplicitSection::of(&cylinder, &wall, wall_domain(), band());
    assert!(
        traced.is_ok(),
        "the handle reads the cylinder: {:?}",
        traced.err()
    );
    let named = |e: &SsiError| matches!(e, SsiError::WrongLane { expected } if expected.contains("a plane and"));
    let got = geom_brep::ssi::plane_nurbs_ssi(&cylinder, &wall, wall_domain(), band());
    assert!(got.as_ref().is_err_and(named), "plane_nurbs_ssi: {got:?}");
    let got = geom_brep::ssi::trace_plane_nurbs_uncertified(
        &cylinder,
        &wall,
        (0.5, 0.5),
        wall_domain(),
        eps(),
        band(),
    );
    assert!(
        got.as_ref().is_err_and(named),
        "trace_plane_nurbs_uncertified: {:?}",
        got.err()
    );
}
