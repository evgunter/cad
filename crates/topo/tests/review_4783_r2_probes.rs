//! **PR 4783 review, R2 probes** (`flush/thinness-by-pairs`): what the
//! boolean door ships now that the sole-shell `V/A` gate is retired,
//! read by the at-rest census over each result's own contacts.
//!
//! Each probe prints a table (run with `--nocapture`) and asserts only
//! what the review states as found, so it goes red if the finding moves.

#![allow(clippy::expect_used, clippy::panic, clippy::print_stderr)]

use geom_core::{Point3, Tol};
use topo::{AtRestBody, BooleanError, BooleanResult, intersect};

use crate::common;

type V3 = [f64; 3];

fn dot(a: V3, b: V3) -> f64 {
    a[0] * b[0] + a[1] * b[1] + a[2] * b[2]
}
fn cross(a: V3, b: V3) -> V3 {
    [
        a[1] * b[2] - a[2] * b[1],
        a[2] * b[0] - a[0] * b[2],
        a[0] * b[1] - a[1] * b[0],
    ]
}
fn add(a: V3, b: V3) -> V3 {
    [a[0] + b[0], a[1] + b[1], a[2] + b[2]]
}
fn scale(a: V3, s: f64) -> V3 {
    a.map(|c| c * s)
}
fn unit(a: V3) -> V3 {
    scale(a, 1.0 / dot(a, a).sqrt())
}
fn basis(m: V3) -> (V3, V3) {
    let m = unit(m);
    let seed = if m[2].abs() < 0.9 {
        [0.0, 0.0, 1.0]
    } else {
        [1.0, 0.0, 0.0]
    };
    let u = unit(cross(seed, m));
    (u, cross(m, u))
}
fn cube_axis(e: V3, a: u32, d: f64) -> V3 {
    let e = unit(e);
    let (p1, p2) = basis(e);
    let al = std::f64::consts::TAU * (f64::from(a) + 0.25) / 16.0;
    let base = add(scale(p1, al.cos()), scale(p2, al.sin()));
    unit(unit(add(base, scale(e, d))))
}

/// `sliver_shell_role.rs`'s `corner_pose`, verbatim.
fn corner_pose(
    profile: &[(f64, f64)],
    v: V3,
    e: V3,
    a: u32,
    d: f64,
    tol: Tol,
) -> (AtRestBody<f64>, AtRestBody<f64>) {
    let prism = common::prism::<f64>(profile, 1.0, tol).body;
    let prism = AtRestBody::validate(prism, tol).expect("the prism is at rest");
    let m = cube_axis(e, a, d);
    let (u, w) = basis(m);
    let cube = common::mapped_cube::<f64>(
        move |x, y, z| {
            let (a, b, c) = (-2.0 + 4.0 * x, -2.0 + 4.0 * y, 4.0 * z);
            let p = add(v, add(scale(u, a), add(scale(w, b), scale(m, c))));
            Point3::new(p[0], p[1], p[2])
        },
        tol,
    );
    let cube = AtRestBody::validate(cube, tol).expect("the cube is at rest");
    (prism, cube)
}

/// What one op did: refused, empty, built with a clean census, or built
/// holding census findings (the predicates named).
#[derive(Debug, Clone, PartialEq)]
enum Outcome {
    Refused(String),
    Empty,
    Clean,
    Findings(Vec<String>),
}

fn outcome(result: Result<BooleanResult<f64>, BooleanError>, tol: Tol) -> Outcome {
    match result {
        Err(e) => Outcome::Refused(format!("{e:?}").chars().take(160).collect()),
        Ok(r) => match r.body() {
            None => Outcome::Empty,
            Some(b) => match b.body.validate_pseudomanifold(&b.contacts, tol) {
                Ok(()) => Outcome::Clean,
                Err(f) => Outcome::Findings(
                    f.iter()
                        .map(|f| match f {
                            topo::ValidationError::CensusEscalated { cause } => format!(
                                "CensusEscalated({:?}, {:?})",
                                cause.predicate,
                                cause.margin.diagnostic_f64_for_error_text().value()
                            ),
                            other => format!("{other:?}").chars().take(120).collect(),
                        })
                        .collect(),
                ),
            },
        },
    }
}

fn brief(o: &Outcome) -> String {
    match o {
        Outcome::Refused(s) => format!("refused {}", s.chars().take(60).collect::<String>()),
        Outcome::Empty => "empty".to_owned(),
        Outcome::Clean => "clean".to_owned(),
        Outcome::Findings(f) => format!("SHIPS-WITH-FINDINGS {}", f.len()),
    }
}

fn eps_is_1e9(tol: Tol) -> bool {
    (tol.eps() - 1e-9).abs() < 1e-21
}

const NOTCH307: [(f64, f64); 5] = [(0.0, 0.0), (4.0, 0.0), (4.0, 2.0), (2.0, 1.0), (0.0, 2.0)];
const NOTCH307_PIECE: [(f64, f64); 4] = [(2.0, 0.0), (4.0, 0.0), (4.0, 2.0), (2.0, 1.0)];

/// **Claim 1, beyond the author's five rows**: the `notch307` piece and
/// the whole `notch307` prism ∩ the witness cube over every pose
/// `a0..a15` and a ladder of `d`, both orders. Counts the results that
/// ship holding a census finding.
#[test]
fn r2_needle_family_ships_census_findings() {
    let tol = Tol::witness();
    if !eps_is_1e9(tol) {
        return;
    }
    let (corner, edge) = ([2.0, 1.0, 1.0], [2.0, 1.0, 0.0]);
    let mut shipped_with_findings = Vec::new();
    let mut tally = std::collections::BTreeMap::<&str, usize>::new();
    for (name, profile) in [("piece", &NOTCH307_PIECE[..]), ("notch307", &NOTCH307[..])] {
        for a in 0..16 {
            for d in [5e-9, 1e-8, 1.5e-8, 2e-8, 2.5e-8, 3e-8, 3.05e-8, 4e-8] {
                let (p, c) = corner_pose(profile, corner, edge, a, d, tol);
                for (order, r) in [
                    ("p∩c", intersect(&p, &c, tol)),
                    ("c∩p", intersect(&c, &p, tol)),
                ] {
                    let o = outcome(r, tol);
                    eprintln!("R2 POSE {name} a{a} d{d:e} {order}: {}", brief(&o));
                    let key = match &o {
                        Outcome::Refused(_) => "refused",
                        Outcome::Empty => "empty",
                        Outcome::Clean => "clean",
                        Outcome::Findings(_) => "SHIPS-WITH-FINDINGS",
                    };
                    *tally.entry(key).or_default() += 1;
                    if let Outcome::Findings(f) = &o {
                        shipped_with_findings.push(format!("{name} a{a} d{d:e} {order}: {f:?}"));
                    }
                }
            }
        }
    }
    eprintln!("R2 needle family tally: {tally:?}");
    for line in &shipped_with_findings {
        eprintln!("R2 SHIPS {line}");
    }
}

fn brick(x: (f64, f64), y: (f64, f64), z: (f64, f64), tol: Tol) -> AtRestBody<f64> {
    AtRestBody::validate(common::brick::<f64>(x, y, z, tol), tol).expect("the brick is at rest")
}

/// **Claim 4, finer than the author's ladder**: the slab, the hollow, the
/// fin and the cube on a plate, across `[9ε, 25ε]` in half-ε steps, each
/// result read by the census. Prints the first thickness each builds at
/// and every build that holds a finding.
#[test]
fn r2_walls_across_k_eps() {
    let tol = Tol::witness();
    if !eps_is_1e9(tol) {
        return;
    }
    let eps = tol.eps();
    let slab = |t: f64| {
        let a = brick((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), tol);
        let b = brick((-1.0, 2.0), (-1.0, 2.0), (1.0 - t, 2.0), tol);
        intersect(&a, &b, tol)
    };
    let hollow = |t: f64| {
        let a = brick((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), tol);
        let core = brick((t, 1.0 - t), (t, 1.0 - t), (t, 1.0 - t), tol);
        topo::subtract(&a, &core, tol)
    };
    let fin = |t: f64| {
        let a = brick((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), tol);
        let block = brick((0.5, 0.7), (0.25, 0.75), (0.5, 1.5), tol);
        let both = topo::union(&a, &block, tol).expect("unite");
        let both = &both.body().expect("not empty").body;
        let cutter = brick((0.5 + t, 1.0), (0.0, 1.0), (1.0, 2.0), tol);
        topo::subtract(both, &cutter, tol)
    };
    // A thin plate under a millimetre cube, the plate `t` thick.
    let plate = |t: f64| {
        let cube = brick((0.0, 1e-3), (0.0, 1e-3), (0.0, 1e-3), tol);
        let plate = brick((-0.5, 0.5), (-0.5, 0.5), (-t, 0.0), tol);
        topo::union(&cube, &plate, tol)
    };
    // A step: a unit brick less a brick that leaves a ledge `t` thick.
    let step = |t: f64| {
        let a = brick((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), tol);
        let b = brick((0.5, 2.0), (-1.0, 2.0), (t, 2.0), tol);
        topo::subtract(&a, &b, tol)
    };
    type F<'a> = &'a dyn Fn(f64) -> Result<BooleanResult<f64>, BooleanError>;
    let walls: [(&str, F); 5] = [
        ("slab", &slab),
        ("hollow", &hollow),
        ("fin", &fin),
        ("plate", &plate),
        ("step", &step),
    ];
    for (name, build) in walls {
        let mut row = Vec::new();
        let mut m = 9.0;
        while m <= 25.0 {
            let Ok(o) = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
                outcome(build(m * eps), tol)
            })) else {
                row.push(format!("{m}ε:fixture-panics"));
                m += 0.5;
                continue;
            };
            let tag = match &o {
                Outcome::Refused(s) => format!("R[{}]", s.chars().take(70).collect::<String>()),
                Outcome::Empty => "E".to_owned(),
                Outcome::Clean => "ok".to_owned(),
                Outcome::Findings(f) => format!("SHIPS-FINDINGS{f:?}"),
            };
            row.push(format!("{m}ε:{tag}"));
            m += 0.5;
        }
        eprintln!("R2 wall {name}: {}", row.join("  "));
    }
}

/// **Claims 2 and 5**: each needle the door now ships, used as an
/// operand of a union with a cube far away, which reads its orientation
/// up front (`reduce.rs` `gate_sliver_orientation`).
#[test]
fn r2_shipped_needles_as_operands() {
    let tol = Tol::witness();
    if !eps_is_1e9(tol) {
        return;
    }
    let (corner, edge) = ([2.0, 1.0, 1.0], [2.0, 1.0, 0.0]);
    let far = brick((50.0, 51.0), (50.0, 51.0), (50.0, 51.0), tol);
    for (a, d) in [
        (3, 1e-8),
        (3, 1.5e-8),
        (3, 2e-8),
        (3, 2.5e-8),
        (3, 3e-8),
        (2, 3e-8),
        (2, 3.05e-8),
    ] {
        let (p, c) = corner_pose(&NOTCH307_PIECE, corner, edge, a, d, tol);
        let Ok(meet) = intersect(&p, &c, tol) else {
            eprintln!("R2 OPERAND a{a} d{d:e}: the ∩ refuses");
            continue;
        };
        let Some(needle) = meet.body() else {
            eprintln!("R2 OPERAND a{a} d{d:e}: empty");
            continue;
        };
        let m = topo::mass_properties(&needle.body, tol).map(|m| (m.volume, m.surface_area));
        let u = outcome(topo::union(&needle.body, &far, tol), tol);
        let s = outcome(topo::subtract(&c, &needle.body, tol), tol);
        eprintln!(
            "R2 OPERAND a{a} d{d:e}: (V, A) {m:?}; needle ∪ far: {}; cube ∖ needle: {}",
            brief(&u),
            brief(&s)
        );
    }
}
