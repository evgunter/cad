//! Review probes (FLUSH coincident fate cut, R2): every op in both
//! operand orders over flush fixtures, printing one line per run and
//! checking order symmetry of ∪ and ∩ (volume, unordered substitution
//! pairs by operand root face, the set of bordered stretches).
//!
//! `cargo test -p sweep --test all flush_fate_review_r2 -- --ignored --nocapture`

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::BTreeSet;

use geom_core::{Point2, Tol, Vec3};
use sweep::test_support as st;
use topo::{
    AtRestBody, BooleanBody, BooleanResult, FaceKey, Operand, intersect, mass_properties,
    subtract, union, validate_geometric, validate_pseudomanifold,
};

type Ends = [[i64; 3]; 2];

fn tol() -> Tol {
    Tol::witness()
}

fn fin(name: &str, b: topo::Body<f64>) -> AtRestBody<f64> {
    topo::test_support::finished(name, b, tol())
}

fn brick(x: (f64, f64), y: (f64, f64), z: (f64, f64)) -> AtRestBody<f64> {
    fin("brick", st::brick(x, y, z, tol()))
}

fn body(r: Result<BooleanResult<f64>, topo::BooleanError>) -> AtRestBody<f64> {
    match r {
        Ok(BooleanResult::Body(b)) => b.body,
        other => panic!("fixture op: {:?}", other.map(|_| ())),
    }
}

fn poly(pts: &[(f64, f64)]) -> Vec<(Point2<f64>, f64)> {
    pts.iter().map(|&(x, y)| (Point2::new(x, y), 0.0)).collect()
}

fn stretches(out: &BooleanBody<f64>, rows: &[(topo::VertexKey, topo::VertexKey)]) -> Vec<Ends> {
    let body = &out.body;
    let settler = out.naming.settler(body);
    let at = |v| {
        let p = topo::readback::vertex_point(body, v).expect("live");
        [p.x, p.y, p.z].map(|c| (c * 1000.0).round() as i64)
    };
    let ends = |e: topo::EdgeKey| {
        let d = body.get_edge(e).expect("a live edge");
        [d.he_plus, d.he_minus].map(|h| body.get_half_edge(h).expect("he").start)
    };
    let mut v: Vec<Ends> = rows
        .iter()
        .map(|&(u, w)| {
            let (Ok(u), Ok(w)) = (settler.settle(u), settler.settle(w)) else {
                return [[i64::MIN; 3]; 2];
            };
            let between = match (u, w) {
                (topo::Cell::Vertex(u), topo::Cell::Vertex(w)) => {
                    body.edges().map(|(k, _)| k).find(|&k| {
                        let [s, t] = ends(k);
                        (s, t) == (u, w) || (s, t) == (w, u)
                    })
                }
                _ => None,
            };
            match between.or_else(|| out.naming.stretch_through_joins(body, (u, w))) {
                Some(edge) => {
                    let mut e = ends(edge).map(at);
                    e.sort();
                    e
                }
                None => [[i64::MAX; 3]; 2],
            }
        })
        .collect();
    v.sort();
    v
}

#[derive(Debug, PartialEq, Eq, Clone)]
struct Summary {
    outcome: String,
    volume: i64,
    pairs: BTreeSet<((u8, String), (u8, String))>,
    borders: BTreeSet<Ends>,
}

fn root(out: &BooleanBody<f64>, op: Operand, f: FaceKey) -> FaceKey {
    match op {
        Operand::A => out.naming.face_fragments_a.root(f),
        Operand::B => out.naming.face_fragments_b.root(f),
    }
}

/// One run: `x op y` where `xl`/`yl` label the operands (0 = first fixture body).
fn run(
    label: &str,
    op: &str,
    x: (&AtRestBody<f64>, u8),
    y: (&AtRestBody<f64>, u8),
) -> Summary {
    let t = tol();
    let r = match op {
        "∪" => union(x.0, y.0, t),
        "∩" => intersect(x.0, y.0, t),
        _ => subtract(x.0, y.0, t),
    };
    let lab = |o: Operand| if o == Operand::A { x.1 } else { y.1 };
    let out = match r {
        Ok(BooleanResult::Body(out)) => out,
        Ok(BooleanResult::Empty) => {
            println!("{label} [{}{op}{}]: EMPTY", x.1, y.1);
            return Summary { outcome: "empty".into(), volume: 0, pairs: BTreeSet::new(), borders: BTreeSet::new() };
        }
        Err(e) => {
            println!("{label} [{}{op}{}]: ERR {e:?}", x.1, y.1);
            return Summary { outcome: format!("err {e}"), volume: -1, pairs: BTreeSet::new(), borders: BTreeSet::new() };
        }
    };
    let vol = (mass_properties(&out.body, t).expect("mass").volume * 1e6).round() as i64;
    let t3 = validate_geometric(&out.body, t).is_ok();
    let t3p = validate_pseudomanifold(&out.body, &out.contacts, t).err();
    let mut pairs = BTreeSet::new();
    let mut rows = Vec::new();
    for s in &out.naming.substituted {
        let d = (lab(s.operand), format!("{:?}", root(&out, s.operand, s.face)));
        let k = (lab(s.operand.other()), format!("{:?}", root(&out, s.operand.other(), s.by)));
        rows.push(format!("{}:{:?}<-{}:{:?}", d.0, s.face, k.0, s.by));
        pairs.insert((d.clone().min(k.clone()), d.max(k)));
    }
    // fragments per substituted root
    let mut per_root = Vec::new();
    for s in &out.naming.substituted {
        let fr = match s.operand {
            Operand::A => &out.naming.face_fragments_a,
            Operand::B => &out.naming.face_fragments_b,
        };
        let r0 = fr.root(s.face);
        let total = 1 + fr.rows().iter().filter(|(n, _)| fr.root(*n) == r0).count();
        let subs = out.naming.substituted.iter().filter(|z| z.operand == s.operand && fr.root(z.face) == r0).count();
        let disc = out.naming.discards.iter().filter(|z| z.operand == s.operand && fr.root(z.face) == r0).count();
        per_root.push((lab(s.operand), format!("{r0:?}"), total, subs, disc));
    }
    per_root.sort();
    per_root.dedup();
    let mut borders = BTreeSet::new();
    let mut disc_lines = Vec::new();
    for d in &out.naming.discards {
        let s = stretches(&out, &d.bordered);
        if !s.is_empty() {
            disc_lines.push(format!("{}:{:?}{:?}", lab(d.operand), root(&out, d.operand, d.face), s));
        }
        borders.extend(s.into_iter().filter(|e| e[0][0] != i64::MAX));
    }
    println!(
        "{label} [{}{op}{}]: {:?} vol {vol} t3 {t3} t3' {} subs {rows:?} roots(lab,root,frags,subs,discs) {per_root:?} discards {disc_lines:?}",
        x.1, y.1, out.kind, t3p.as_ref().map_or("ok".to_string(), |e| format!("{e:?}"))
    );
    assert!(t3, "{label}: tier 3");
    Summary { outcome: "ok".into(), volume: vol, pairs, borders }
}

fn fixtures() -> Vec<(&'static str, AtRestBody<f64>, AtRestBody<f64>)> {
    let t = tol();
    let u = (0.0, 1.0);
    let mut v = Vec::new();
    v.push(("overlap", brick((0.0, 2.0), (0.0, 2.0), u), brick((1.0, 3.0), (1.0, 3.0), u)));
    v.push(("stair", brick((0.0, 2.0), u, u), brick((1.0, 3.0), u, u)));
    v.push(("cross", brick((0.0, 3.0), (1.0, 2.0), u), brick((1.0, 2.0), (0.0, 3.0), u)));
    v.push(("bottoms", brick((0.0, 2.0), (0.0, 2.0), u), brick((1.0, 3.0), (1.0, 3.0), (0.0, 2.0))));
    let l = fin("L", st::prism(poly(&[(0.0, 0.0), (2.0, 0.0), (2.0, 1.0), (1.0, 1.0), (1.0, 2.0), (0.0, 2.0)]), 1.0, t));
    v.push(("L-overlap", l.clone(), brick((0.5, 2.5), (0.5, 1.5), u)));
    v.push(("L-reflex", l, brick((0.5, 1.5), (0.5, 1.5), u)));
    let frame = body(subtract(&brick((0.0, 3.0), (0.0, 3.0), u), &brick((1.0, 2.0), (1.0, 2.0), (-1.0, 2.0)), t));
    v.push(("ring", frame.clone(), brick((0.5, 2.5), (0.5, 2.5), u)));
    v.push(("ring-half", frame.clone(), brick((0.5, 2.5), (0.5, 1.5), u)));
    let pocketed = body(subtract(&brick((0.0, 3.0), (0.0, 3.0), u), &brick((1.0, 2.0), (1.0, 2.0), (0.5, 2.0)), t));
    v.push(("hole-kept", pocketed, brick((0.5, 2.5), (0.5, 2.5), (0.8, 1.0))));
    v.push(("sunk", brick((0.0, 3.0), (0.0, 3.0), u), brick((1.0, 2.0), (1.0, 2.0), (0.5, 1.0))));
    v.push(("sunk-edge", brick((0.0, 3.0), (0.0, 3.0), u), brick((1.0, 2.0), (0.0, 2.0), (0.5, 1.0))));
    let plate = body(subtract(&brick((0.0, 3.0), (0.0, 2.0), u), &brick((1.0, 2.0), (0.8, 1.2), (0.5, 1.5)), t));
    v.push(("pocket-floor", plate, brick((0.5, 2.5), (-1.0, 3.0), (0.0, 0.5))));
    // curved
    let c = |r, cx, cy, z0, h| fin("cyl", st::cylinder_of_arcs_at(4, r, Point2::new(cx, cy), z0, h, t));
    v.push(("cyl-coaxial", c(1.0, 0.0, 0.0, 0.0, 2.0), c(1.0, 0.0, 0.0, 1.0, 2.0)));
    v.push(("cyl-sunk", brick((-2.0, 2.0), (-2.0, 2.0), u), c(1.0, 0.0, 0.0, 0.5, 0.5)));
    v.push(("cyl-edge", brick((0.0, 2.0), (0.0, 2.0), u), c(0.5, 2.0, 1.0, 0.0, 1.0)));
    v.push(("cyl-edge-tall", brick((0.0, 2.0), (0.0, 2.0), u), c(0.5, 2.0, 1.0, 0.0, 2.0)));
    let hemi = fin("hemi", st::hemisphere_on_flat_base(1.0, t));
    v.push(("hemi-base", hemi.clone(), brick((-0.5, 2.0), (0.0, 0.5), (-2.0, 2.0))));
    v.push(("hemi-ball", hemi, fin("ball", st::ball_poled_y(1.0, Vec3::new(0.0, 0.0, 0.0), t))));
    let rev = |pts: &[(f64, f64)]| fin("rev", st::revolved_about_y(poly(pts), sweep::Revolution::Full, t));
    v.push(("cone-frustum", rev(&[(0.0, 0.0), (1.0, 0.0), (0.0, 1.0)]), rev(&[(0.0, 0.0), (1.0, 0.0), (0.5, 0.5), (0.0, 0.5)])));
    // An ellipse chord: a cylinder cut by a slanted plane, coaxial with a taller one.
    let slanted = fin(
        "slant",
        st::prism_on(
            st::sketch_from_axes(geom_core::Point3::new(0.0, -2.0, 0.0), Vec3::new(1.0, 0.0, 0.0), Vec3::new(0.0, 0.0, 1.0), t),
            poly(&[(-2.0, -1.0), (2.0, -1.0), (2.0, 1.0), (-2.0, 1.6)]),
            4.0,
            t,
        ),
    );
    let tall = c(1.0, 0.0, 0.0, -0.5, 3.0);
    match intersect(&tall, &slanted, t) {
        Ok(BooleanResult::Body(b)) => v.push(("cyl-ellipse", c(1.0, 0.0, 0.0, 0.0, 3.0), b.body)),
        other => println!("cyl-ellipse fixture: {:?}", other.map(|_| ())),
    }
    v
}

#[test]
#[ignore = "review probe"]
fn flush_fate_review_r2() {
    let mut asym = Vec::new();
    for (label, x, y) in fixtures() {
        let mut s = Vec::new();
        for op in ["∪", "∩", "−"] {
            let xy = run(label, op, (&x, 0), (&y, 1));
            let yx = run(label, op, (&y, 1), (&x, 0));
            if op != "−" && xy != yx {
                println!(
                    "ASYM {label} {op}: outcome {:?}/{:?} vol {}/{} pairs-eq {} borders only-xy {:?} only-yx {:?}",
                    xy.outcome, yx.outcome, xy.volume, yx.volume, xy.pairs == yx.pairs,
                    xy.borders.difference(&yx.borders).collect::<Vec<_>>(),
                    yx.borders.difference(&xy.borders).collect::<Vec<_>>()
                );
                asym.push(format!("{label} {op}"));
            }
            s.push((xy, yx));
        }
    }
    println!("ASYMMETRIC: {asym:?}");
}
