//! Reviewer-3 probes (local only, not for commit).
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic, missing_docs)]

use crate::common;

use common::meeting::{Hole, MEET, PLATE, corners_disjoint, cycles_of, ell, notch, wedge};
use common::{FaceGeometry, describe_as_intersections, finished, prism_ops};
use geom_core::{Point3, Tol};
use topo::{
    AtRestBody, Body, BooleanResult, intersect, subtract, union, validate_geometric,
    validate_pseudomanifold,
};

fn t() -> Tol {
    Tol::witness()
}

struct Pose {
    label: &'static str,
    r: [[f64; 3]; 3],
    t: [f64; 3],
}

impl Pose {
    fn turn(label: &'static str, axis: [f64; 3], angle: f64, t: [f64; 3]) -> Self {
        let l = axis.iter().map(|a| a * a).sum::<f64>().sqrt();
        let [x, y, z] = axis.map(|a| a / l);
        let (s, c) = angle.sin_cos();
        let d = 1.0 - c;
        let r = [
            [c + x * x * d, x * y * d - z * s, x * z * d + y * s],
            [y * x * d + z * s, c + y * y * d, y * z * d - x * s],
            [z * x * d - y * s, z * y * d + x * s, c + z * z * d],
        ];
        Self { label, r, t }
    }
    fn at(&self, p: [f64; 3]) -> Point3<f64> {
        let q = [0, 1, 2].map(|i| (0..3).map(|k| self.r[i][k] * p[k]).sum::<f64>() + self.t[i]);
        Point3::new(q[0], q[1], q[2])
    }
}

fn poses() -> Vec<Pose> {
    let mut v = vec![
        Pose::turn("rest", [0.0, 0.0, 1.0], 0.0, [0.0; 3]),
        Pose::turn("turned", [1.0, 2.0, 3.0], 0.7, [0.3, -0.2, 0.5]),
        Pose::turn("flipped", [1.0, 0.0, 0.0], std::f64::consts::PI, [0.0; 3]),
    ];
    if std::env::var("RV3_ALL_POSES").is_ok() {
        v.push(Pose::turn("z-turn", [0.0, 0.0, 1.0], 0.65, [0.0; 3]));
        v.push(Pose::turn("tilt2", [-2.0, 1.0, 1.0], 2.3, [-0.4, 0.1, 0.3]));
    }
    v
}

fn posed_box(b: [(f64, f64); 3], pose: &Pose) -> AtRestBody<f64> {
    let [(x0, x1), (y0, y1), z] = b;
    let mut body = Body::<f64>::new();
    prism_ops(
        &mut body,
        &[(x0, y0), (x1, y0), (x1, y1), (x0, y1)],
        z,
        |x, y, z| pose.at([x, y, z]),
        FaceGeometry::Certified,
        t(),
    );
    describe_as_intersections(&mut body, t());
    finished("box", body, t())
}

fn posed_prism(h: &Hole, pose: &Pose) -> AtRestBody<f64> {
    let [o, u, v, n] = h.frame();
    let mut body = Body::<f64>::new();
    prism_ops(
        &mut body,
        &h.profile(),
        (0.0, h.length),
        |x, y, z| pose.at([0, 1, 2].map(|i| o[i] + x * u[i] + y * v[i] + z * n[i])),
        FaceGeometry::Certified,
        t(),
    );
    describe_as_intersections(&mut body, t());
    finished("prism", body, t())
}

fn vol(b: &Body<f64>) -> f64 {
    topo::mass_properties(b, t()).unwrap().volume
}

/// Max visits of one loop / one face at the vertex nearest `m`.
fn visits(b: &Body<f64>, m: Point3<f64>) -> (usize, usize, usize) {
    let mut ml = 0;
    let mut mf = 0;
    let near = |p: Point3<f64>| (p - m).norm() < 1e-6;
    for (_, f) in b.faces() {
        let mut fsum = 0;
        for c in cycles_of(b, f) {
            let n = c
                .iter()
                .filter(|&&he| near(b.half_edge_start_point(he).unwrap()))
                .count();
            ml = ml.max(n);
            fsum += n;
        }
        mf = mf.max(fsum);
    }
    let nv = b
        .vertices()
        .filter(|&(k, _)| near(topo::readback::vertex_point(b, k).unwrap()))
        .count();
    (ml, mf, nv)
}

fn check(
    what: &str,
    r: Result<BooleanResult<f64>, topo::BooleanError>,
    want: Option<f64>,
    m: Point3<f64>,
    log: &mut Vec<String>,
    bad: &mut Vec<String>,
) -> Option<AtRestBody<f64>> {
    match r {
        Ok(BooleanResult::Body(r)) => {
            let b = r.body;
            let t3 = validate_geometric(&b, t()).is_ok();
            let t3p = validate_pseudomanifold(&b, &r.contacts, t()).is_ok();
            let cd = corners_disjoint(&b);
            let v = vol(&b);
            let vok = want.is_none_or(|w| (v - w).abs() < 1e-8);
            let (ml, mf, nv) = visits(&b, m);
            let counts = [b.faces().count(), b.edges().count(), b.vertices().count()];
            let line = format!(
                "{what}: BUILT {counts:?} t3={t3} t3'={t3p} corners={} vol_ok={vok} \
                 loopvisits={ml} facevisits={mf} vtx_at_meet={nv}",
                match &cd {
                    Ok(()) => "ok".to_string(),
                    Err(e) => e.clone(),
                }
            );
            if !(t3 && t3p && cd.is_ok() && vok) {
                bad.push(line.clone());
            }
            log.push(line);
            Some(b)
        }
        Ok(BooleanResult::Empty) => {
            log.push(format!("{what}: EMPTY"));
            None
        }
        Err(e) => {
            log.push(format!(
                "{what}: REFUSED {:?}",
                topo::BooleanError::kind(&e)
            ));
            if let topo::BooleanError::PinchOfManyHolesInOneRing { holes, .. } = e {
                log.push(format!("    holes={holes}"));
            }
            None
        }
    }
}

fn fixtures() -> Vec<(&'static str, Vec<Hole>)> {
    let n = |a, b, k| notch(a, b, k, 2.0);
    vec![
        ("NN", vec![n(80.0, 100.0, 0), n(260.0, 280.0, 1)]),
        ("NN-adj", vec![n(80.0, 100.0, 0), n(150.0, 170.0, 1)]),
        ("NW", vec![n(80.0, 100.0, 0), wedge(200.0, 250.0, 1)]),
        (
            "NNWW-adj",
            vec![
                n(80.0, 100.0, 0),
                n(130.0, 150.0, 1),
                wedge(200.0, 240.0, 2),
                wedge(290.0, 330.0, 3),
            ],
        ),
        (
            "NWWW",
            vec![
                n(80.0, 100.0, 0),
                wedge(140.0, 180.0, 1),
                wedge(220.0, 260.0, 2),
                wedge(300.0, 340.0, 3),
            ],
        ),
        (
            "NNNN",
            vec![
                n(80.0, 100.0, 0),
                n(170.0, 190.0, 1),
                n(260.0, 280.0, 2),
                n(345.0, 365.0, 3),
            ],
        ),
        (
            "NWN-narrow",
            vec![
                n(80.0, 100.0, 0),
                wedge(110.0, 130.0, 1),
                n(140.0, 160.0, 2),
            ],
        ),
        (
            "5W",
            (0..5)
                .map(|i| {
                    let a = 72.0 * i as f64;
                    wedge(a, a + 30.0, i)
                })
                .collect(),
        ),
        (
            "6W-thin",
            (0..6)
                .map(|i| {
                    let a = 60.0 * i as f64;
                    wedge(a, a + 12.0, i)
                })
                .collect(),
        ),
        (
            "3W-thin",
            vec![
                wedge(0.0, 4.0, 0),
                wedge(120.0, 124.0, 1),
                wedge(240.0, 244.0, 2),
            ],
        ),
        (
            "NNNWW",
            vec![
                n(80.0, 100.0, 0),
                n(200.0, 220.0, 1),
                n(320.0, 340.0, 2),
                wedge(140.0, 170.0, 3),
                wedge(250.0, 290.0, 4),
            ],
        ),
        (
            "6mix",
            vec![
                n(80.0, 95.0, 0),
                wedge(120.0, 140.0, 1),
                n(170.0, 185.0, 2),
                wedge(215.0, 235.0, 3),
                n(260.0, 275.0, 4),
                wedge(310.0, 330.0, 5),
            ],
        ),
        ("L+N", vec![ell(), n(200.0, 250.0, 1)]),
        (
            "N+WWadj",
            vec![
                n(80.0, 100.0, 0),
                wedge(200.0, 230.0, 1),
                wedge(250.0, 280.0, 2),
            ],
        ),
        (
            "N+WWnear",
            vec![
                n(80.0, 100.0, 0),
                wedge(105.0, 135.0, 1),
                wedge(140.0, 170.0, 2),
            ],
        ),
        (
            "Nwide+WW",
            vec![
                notch(30.0, 150.0, 0, 2.0),
                wedge(200.0, 230.0, 1),
                wedge(300.0, 330.0, 2),
            ],
        ),
        (
            "NN+WWsame",
            vec![
                n(80.0, 100.0, 0),
                n(260.0, 280.0, 1),
                wedge(120.0, 150.0, 2),
                wedge(180.0, 230.0, 3),
            ],
        ),
        (
            "L+N+W",
            vec![ell(), n(190.0, 215.0, 1), wedge(235.0, 260.0, 2)],
        ),
        ("L+2N", vec![ell(), n(190.0, 215.0, 1), n(235.0, 260.0, 2)]),
        (
            "L+3W",
            vec![
                ell(),
                wedge(185.0, 200.0, 1),
                wedge(212.0, 228.0, 2),
                wedge(240.0, 262.0, 3),
            ],
        ),
    ]
}

#[test]
fn rv3_probe_mixed_configurations() {
    let filter = std::env::var("RV3_ONLY").ok();
    let mut log = Vec::new();
    let mut bad = Vec::new();
    for pose in poses() {
        let m = pose.at(MEET);
        let p = posed_box(PLATE, &pose);
        for (name, holes) in fixtures() {
            if filter.as_deref().is_some_and(|f| f != name) {
                continue;
            }
            let label = format!("{name} [{}] k={}", pose.label, holes.len());
            let prisms: Vec<_> = holes.iter().map(|h| posed_prism(h, &pose)).collect();
            let mut u = Some(prisms[0].clone());
            for q in &prisms[1..] {
                u = u.and_then(|u| {
                    check(
                        &format!("{label}: U-fold"),
                        union(&u, q, t()),
                        None,
                        m,
                        &mut log,
                        &mut bad,
                    )
                });
            }
            let Some(u) = u else {
                continue;
            };
            let inside = check(
                &format!("{label}: P∩U"),
                intersect(&p, &u, t()),
                None,
                m,
                &mut log,
                &mut bad,
            )
            .map(|b| vol(&b));
            let _ = check(
                &format!("{label}: U∩P"),
                intersect(&u, &p, t()),
                inside,
                m,
                &mut log,
                &mut bad,
            );
            let vu = vol(&u);
            let _ = check(
                &format!("{label}: U−P"),
                subtract(&u, &p, t()),
                inside.map(|i| vu - i),
                m,
                &mut log,
                &mut bad,
            );
            let _ = check(
                &format!("{label}: P−U"),
                subtract(&p, &u, t()),
                inside.map(|i| 6.0 - i),
                m,
                &mut log,
                &mut bad,
            );
            let _ = check(
                &format!("{label}: P∪U"),
                union(&p, &u, t()),
                inside.map(|i| 6.0 + vu - i),
                m,
                &mut log,
                &mut bad,
            );
            // sequential P less each
            let mut s = Some(p.clone());
            for (i, q) in prisms.iter().enumerate() {
                let w = if i + 1 == prisms.len() {
                    inside.map(|x| 6.0 - x)
                } else {
                    None
                };
                s = s.and_then(|b| {
                    check(
                        &format!("{label}: P-seq{i}"),
                        subtract(&b, q, t()),
                        w,
                        m,
                        &mut log,
                        &mut bad,
                    )
                });
            }
            // fold unions: plate first, plate last
            let mut f = Some(p.clone());
            for (i, q) in prisms.iter().enumerate() {
                let w = if i + 1 == prisms.len() {
                    inside.map(|x| 6.0 + vu - x)
                } else {
                    None
                };
                f = f.and_then(|b| {
                    check(
                        &format!("{label}: fold-plate-first{i}"),
                        union(&b, q, t()),
                        w,
                        m,
                        &mut log,
                        &mut bad,
                    )
                });
            }
            let mut g = Some(prisms[prisms.len() - 1].clone());
            for (i, q) in prisms.iter().rev().skip(1).enumerate() {
                g = g.and_then(|b| {
                    check(
                        &format!("{label}: fold-rev{i}"),
                        union(&b, q, t()),
                        None,
                        m,
                        &mut log,
                        &mut bad,
                    )
                });
            }
            let _ = g.and_then(|b| {
                check(
                    &format!("{label}: fold-plate-last"),
                    union(&b, &p, t()),
                    inside.map(|x| 6.0 + vu - x),
                    m,
                    &mut log,
                    &mut bad,
                )
            });
            // subtract-seq of plate from each prism union mid (P-U partial): P-(U of first two)
            if prisms.len() >= 3 {
                if let Ok(BooleanResult::Body(u2)) = union(&prisms[0], &prisms[1], t()) {
                    let s2 = check(
                        &format!("{label}: P−U01"),
                        subtract(&p, &u2.body, t()),
                        None,
                        m,
                        &mut log,
                        &mut bad,
                    );
                    if let Some(s2) = s2 {
                        let mut r = Some(s2);
                        for (i, q) in prisms.iter().enumerate().skip(2) {
                            r = r.and_then(|b| {
                                check(
                                    &format!("{label}: P−U01 then −{i}"),
                                    subtract(&b, q, t()),
                                    None,
                                    m,
                                    &mut log,
                                    &mut bad,
                                )
                            });
                        }
                    }
                }
            }
        }
    }
    let out = std::env::var("RV3_OUT").unwrap_or_else(|_| "/tmp/rv3_probe.log".into());
    std::fs::write(&out, log.join("\n")).unwrap();
    let _ = bad;
}

#[test]
fn rv3_probe_pr_fixtures_pu() {
    use common::meeting::{inner_rows, notch_rows};
    let mut log = Vec::new();
    let mut bad = Vec::new();
    for pose in poses() {
        let m = pose.at(MEET);
        let p = posed_box(PLATE, &pose);
        for (name, holes) in inner_rows().into_iter().chain(notch_rows()) {
            let prisms: Vec<_> = holes.iter().map(|h| posed_prism(h, &pose)).collect();
            let u = prisms[1..]
                .iter()
                .fold(prisms[0].clone(), |u, q| match union(&u, q, t()) {
                    Ok(BooleanResult::Body(r)) => r.body,
                    _ => panic!(),
                });
            let _ = check(
                &format!("{name} [{}]: P−U", pose.label),
                subtract(&p, &u, t()),
                None,
                m,
                &mut log,
                &mut bad,
            );
        }
    }
    let out = std::env::var("RV3_OUT2").unwrap_or_else(|_| "/tmp/rv3_probe2.log".into());
    std::fs::write(&out, log.join("\n")).unwrap();
}

#[test]
fn rv3_probe_grid_pu() {
    let pose = Pose::turn("rest", [0.0, 0.0, 1.0], 0.0, [0.0; 3]);
    let m = pose.at(MEET);
    let p = posed_box(PLATE, &pose);
    let mut log = Vec::new();
    let mut bad = Vec::new();
    let starts: [f64; 8] = [0.0, 45.0, 90.0, 135.0, 180.0, 225.0, 270.0, 315.0];
    let k: usize = std::env::var("RV3_K")
        .ok()
        .and_then(|s| s.parse().ok())
        .unwrap_or(3);
    // choose k increasing start slots, each hole 30° wide, each N or W
    let mut combos = Vec::new();
    fn rec(i: usize, k: usize, cur: &mut Vec<usize>, out: &mut Vec<Vec<usize>>) {
        if cur.len() == k {
            out.push(cur.clone());
            return;
        }
        for s in i..8 {
            cur.push(s);
            rec(s + 1, k, cur, out);
            cur.pop();
        }
    }
    rec(0, k, &mut Vec::new(), &mut combos);
    for c in combos {
        if c[0] != 0 {
            continue;
        } // rotation-reduce
        for mask in 0..(1u32 << k) {
            let holes: Vec<Hole> = c
                .iter()
                .enumerate()
                .map(|(j, &s)| {
                    let a = starts[s] + 5.0;
                    if mask >> j & 1 == 1 {
                        notch(a, a + 30.0, j, 2.0)
                    } else {
                        wedge(a, a + 30.0, j)
                    }
                })
                .collect();
            let label = format!("slots {c:?} notchmask {mask:0k$b}");
            let prisms: Vec<_> = holes.iter().map(|h| posed_prism(h, &pose)).collect();
            let mut u = Some(prisms[0].clone());
            for q in &prisms[1..] {
                u = u.and_then(|u| match union(&u, q, t()) {
                    Ok(BooleanResult::Body(r)) => Some(r.body),
                    _ => None,
                });
            }
            let Some(u) = u else {
                log.push(format!("{label}: U fails"));
                continue;
            };
            let inside = match intersect(&p, &u, t()) {
                Ok(BooleanResult::Body(r)) => Some(vol(&r.body)),
                _ => None,
            };
            let _ = check(
                &format!("{label}: P−U"),
                subtract(&p, &u, t()),
                inside.map(|i| 6.0 - i),
                m,
                &mut log,
                &mut bad,
            );
        }
    }
    let out = std::env::var("RV3_OUT3").unwrap_or_else(|_| "/tmp/rv3_probe3.log".into());
    std::fs::write(&out, log.join("\n")).unwrap();
}
