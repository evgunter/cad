//! Review probe (PR 5706): every op in both orders over a corpus of
//! flush pairs, planar and curved, nested, voided and multi-shell. Each
//! run prints a BODY line (the refusal, or the judged line, kind, census
//! and rounded vertex set) and a ROWS line (rows and discards by
//! carrier), and where one order of ∪ or ∩ takes the containment
//! fallback, a CMP line saying whether the two orders' rows agree.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::collections::BTreeSet;

use crate::common::cavity::{cut, rod};
use crate::common::differential::judged;
use geom_core::{Point2, Tol};
use sweep::test_support::{brick, finished, hemisphere_on_flat_base};
use topo::{AtRestBody, BooleanBody, BooleanResult, BooleanResultKind, FaceKey, Operand};

fn r3(x: f64) -> String {
    let r = (x * 1000.0).round() / 1000.0;
    format!("{}", if r == 0.0 { 0.0 } else { r })
}

fn rounded(s: &str) -> String {
    let mut out = String::new();
    let mut num = String::new();
    let flush = |num: &mut String, out: &mut String| {
        if let Ok(x) = num.parse::<f64>()
            && num.contains('.')
        {
            out.push_str(&r3(x));
        } else {
            out.push_str(num);
        }
        num.clear();
    };
    for c in s.chars() {
        let exponent = c == 'e' && !num.is_empty();
        let sign = c == '-' && (num.is_empty() || num.ends_with('e'));
        if c.is_ascii_digit() || c == '.' || exponent || sign {
            num.push(c);
        } else {
            flush(&mut num, &mut out);
            out.push(c);
        }
    }
    flush(&mut num, &mut out);
    out
}

fn seat(body: &topo::Body<f64>, f: FaceKey) -> String {
    rounded(&format!(
        "{:?}",
        topo::face_carrier(body, f).expect("a carrier")
    ))
}

fn root(out: &BooleanBody<f64>, o: Operand, f: FaceKey) -> FaceKey {
    match o {
        Operand::A => out.naming.face_fragments_a.root(f),
        Operand::B => out.naming.face_fragments_b.root(f),
    }
}

fn pt(body: &topo::Body<f64>, v: topo::VertexKey) -> String {
    let p = topo::readback::vertex_point(body, v).expect("a live vertex");
    format!("({},{},{})", r3(p.x), r3(p.y), r3(p.z))
}

fn live(out: &BooleanBody<f64>, rows: &[(topo::VertexKey, topo::VertexKey)]) -> Vec<String> {
    let body = &out.body;
    let settler = out.naming.settler(body);
    let ends = |e: topo::EdgeKey| {
        let d = body.get_edge(e).expect("a live edge");
        [d.he_plus, d.he_minus].map(|h| body.get_half_edge(h).expect("a live half").start)
    };
    rows.iter()
        .map(|&(u, w)| {
            let (u, w) = match (settler.settle(u), settler.settle(w)) {
                (Ok(u), Ok(w)) => (u, w),
                _ => return "UNSETTLED".to_string(),
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
                Some(e) => {
                    let mut s = ends(e).map(|v| pt(body, v));
                    s.sort();
                    format!("{} {}", s[0], s[1])
                }
                None => "NOEDGE".to_string(),
            }
        })
        .collect()
}

/// Rows: (sorted held pairs by carrier, discards by carrier with their
/// stretch ends as points).
type Rows = (BTreeSet<(String, String)>, Vec<(String, Vec<String>)>);

fn run(
    op: &str,
    x: &AtRestBody<f64>,
    y: &AtRestBody<f64>,
) -> (String, Option<BooleanResultKind>, Rows) {
    let tol = Tol::witness();
    let r = match op {
        "∪" => topo::union(x, y, tol),
        "∩" => topo::intersect(x, y, tol),
        _ => topo::subtract(x, y, tol),
    };
    let rows_of = |out: &BooleanBody<f64>| -> Rows {
        let face = |o: Operand, f| match o {
            Operand::A => seat(x, root(out, o, f)),
            Operand::B => seat(y, root(out, o, f)),
        };
        let held = out
            .naming
            .substituted
            .iter()
            .map(|s| {
                let (p, q) = (face(s.operand, s.face), face(s.operand.other(), s.by));
                if p < q { (p, q) } else { (q, p) }
            })
            .collect();
        let mut lost: Vec<(String, Vec<String>)> = out
            .naming
            .discards
            .iter()
            .map(|d| {
                let all: Vec<_> = d
                    .bordered
                    .iter()
                    .chain(&d.bordered_through)
                    .copied()
                    .collect();
                let mut ends = live(out, &all);
                ends.sort();
                (face(d.operand, d.face), ends)
            })
            .collect();
        lost.sort();
        (held, lost)
    };
    let (kind, rows, extra) = match &r {
        Ok(BooleanResult::Body(out)) => {
            let b = &out.body;
            let mut vs: Vec<String> = b.vertices().map(|(v, _)| pt(b, v)).collect();
            vs.sort();
            let census = [
                b.shells().count(),
                b.faces().count(),
                b.edges().count(),
                b.vertices().count(),
            ];
            (
                Some(out.kind),
                rows_of(out),
                format!("{:?} {census:?} {}", out.kind, vs.join(" ")),
            )
        }
        _ => (None, Default::default(), String::new()),
    };
    let line = judged(r, None, tol).line;
    (format!("{line} | {extra}"), kind, rows)
}

fn fin(what: &str, body: topo::Body<f64>) -> AtRestBody<f64> {
    finished(what, body, Tol::witness())
}

fn b(x: (f64, f64), y: (f64, f64), z: (f64, f64)) -> topo::Body<f64> {
    brick(x, y, z, Tol::witness())
}

pub(crate) fn circle(cx: f64, cy: f64, r: f64, az: f64) -> profile::ProfileLoop<f64> {
    profile::RawLoop::new([(
        Point2::new(cx + r * az.cos(), cy + r * az.sin()),
        profile::Segment::Arc(geom_core::Arc2 {
            centre: Point2::new(cx, cy),
            radius: r,
            sweep: core::f64::consts::TAU,
        }),
    )])
}

pub(crate) fn extruded(loops: Vec<profile::ProfileLoop<f64>>, z0: f64, h: f64) -> topo::Body<f64> {
    let tol = Tol::witness();
    let profile = profile::Profile::new(profile::SketchPlane::<f64>::xy(), loops)
        .validate(tol)
        .unwrap();
    let depth = sweep::Extrusion::Distance {
        depth: h,
        side: sweep::ExtrudeSide::Along,
    };
    let body = sweep::extrude(&profile, depth, tol).unwrap().body;
    let up = geom_core::Affine3::translation(geom_core::Vec3::new(0.0, 0.0, z0));
    topo::transform_rigid(&body, &geom_core::test_support::rigid(&up, tol), tol).unwrap()
}

pub(crate) fn square(h: f64) -> profile::ProfileLoop<f64> {
    profile::RawLoop::polygon([
        Point2::new(-h, -h),
        Point2::new(h, -h),
        Point2::new(h, h),
        Point2::new(-h, h),
    ])
}

pub(crate) fn corpus() -> Vec<(&'static str, AtRestBody<f64>, AtRestBody<f64>)> {
    let tol = Tol::witness();
    let plate = || b((-1.0, 1.0), (-1.0, 1.0), (-0.5, 0.0));
    let a = || b((0.0, 2.0), (0.0, 1.0), (0.0, 1.0));
    let block = || b((0.0, 3.0), (0.0, 3.0), (0.0, 2.0));
    let voided = || {
        let outer = sweep::test_support::brick((0.0, 4.0), (0.0, 4.0), (0.0, 4.0), tol);
        let inner = sweep::test_support::brick((1.0, 3.0), (1.0, 3.0), (1.0, 3.0), tol);
        let at = fin("outer", outer);
        let it = fin("inner", inner);
        // The void: the outer block less nothing yet; build by a hollow:
        // outer ∖ inner where inner is strictly inside yields a void.
        topo::subtract(&at, &it, tol)
            .expect("hollow")
            .body()
            .expect("material")
            .body
            .clone()
            .into_body()
    };
    let two = || {
        let l = fin("l", b((0.0, 1.0), (0.0, 1.0), (0.0, 1.0)));
        let r = fin("r", b((2.0, 3.0), (0.0, 1.0), (0.0, 1.0)));
        topo::union(&l, &r, tol)
            .expect("pair")
            .body()
            .expect("material")
            .body
            .clone()
            .into_body()
    };
    let hemi = || hemisphere_on_flat_base(1.0, tol);
    vec![
        (
            "a/inside",
            fin("a", a()),
            fin("in", b((0.5, 1.5), (0.0, 1.0), (0.0, 1.0))),
        ),
        (
            "a/poking",
            fin("a", a()),
            fin("p", b((0.5, 1.5), (0.0, 1.0), (0.0, 2.0))),
        ),
        (
            "a/lid",
            fin("a", a()),
            fin("lid", b((-1.0, 3.0), (-1.0, 2.0), (1.0, 2.0))),
        ),
        (
            "a/big",
            fin("a", a()),
            fin("big", b((-1.0, 3.0), (-1.0, 2.0), (-1.0, 1.0))),
        ),
        (
            "a/bigtwo",
            fin("a", a()),
            fin("bt", b((-1.0, 3.0), (0.0, 2.0), (-1.0, 1.0))),
        ),
        ("a/same", fin("a", a()), fin("a2", a())),
        (
            "a/half",
            fin("a", a()),
            fin("h", b((0.0, 1.0), (0.0, 1.0), (0.0, 1.0))),
        ),
        (
            "a/corner",
            fin("a", a()),
            fin("c", b((0.0, 0.5), (0.0, 0.5), (0.0, 0.5))),
        ),
        (
            "a/mid",
            fin("a", a()),
            fin("m", b((0.5, 1.5), (0.25, 0.75), (0.25, 1.0))),
        ),
        (
            "block/box",
            fin("bl", block()),
            fin("bx", b((1.0, 2.0), (1.0, 2.0), (1.0, 2.0))),
        ),
        (
            "block/cornerbox",
            fin("bl", block()),
            fin("cb", b((0.0, 1.0), (0.0, 1.0), (1.0, 2.0))),
        ),
        (
            "void/fill",
            fin("v", voided()),
            fin("f", b((1.0, 3.0), (1.0, 3.0), (1.0, 3.0))),
        ),
        (
            "void/floor",
            fin("v", voided()),
            fin("fl", b((1.0, 3.0), (1.0, 3.0), (1.0, 2.0))),
        ),
        (
            "void/small",
            fin("v", voided()),
            fin("s", b((1.5, 2.5), (1.5, 2.5), (1.0, 2.0))),
        ),
        (
            "void/outerflush",
            fin("v", voided()),
            fin("o", b((0.0, 4.0), (0.0, 4.0), (0.0, 0.5))),
        ),
        (
            "void/hugger",
            fin("v", voided()),
            fin("hg", b((-1.0, 5.0), (-1.0, 5.0), (-1.0, 4.0))),
        ),
        (
            "two/inleft",
            fin("two", two()),
            fin("il", b((0.0, 0.5), (0.0, 1.0), (0.0, 1.0))),
        ),
        (
            "two/lidleft",
            fin("two", two()),
            fin("ll", b((-0.5, 1.5), (-0.5, 1.5), (1.0, 2.0))),
        ),
        (
            "two/bigleft",
            fin("two", two()),
            fin("bl", b((-0.5, 1.5), (-0.5, 1.5), (-0.5, 1.0))),
        ),
        (
            "two/spanbig",
            fin("two", two()),
            fin("sb", b((-0.5, 3.5), (-0.5, 1.5), (-0.5, 1.0))),
        ),
        (
            "hemi/brickin",
            fin("hemi", hemi()),
            fin("bi", b((-0.3, 0.3), (0.0, 0.5), (-0.3, 0.3))),
        ),
        (
            "hemi/slab",
            fin("hemi", hemi()),
            fin("sl", b((-2.0, 2.0), (-1.0, 0.0), (-2.0, 2.0))),
        ),
        (
            "hemi/bigslab",
            fin("hemi", hemi()),
            fin("bs", b((-2.0, 2.0), (-1.0, 2.0), (-2.0, 2.0))),
        ),
        (
            "hemi/pad",
            fin("hemi", hemi()),
            fin("pd", b((-0.3, 0.3), (-0.5, 0.0), (-0.3, 0.3))),
        ),
        (
            "rod/box",
            fin("rod", rod(Point2::new(0.0, 0.0), 0.5, 0.0, 1.0)),
            fin("bx", b((-1.0, 1.0), (-1.0, 1.0), (0.0, 1.0))),
        ),
        (
            "rod/lowbox",
            fin("rod", rod(Point2::new(0.0, 0.0), 0.5, 0.0, 1.0)),
            fin("lb", b((-1.0, 1.0), (-1.0, 1.0), (0.0, 2.0))),
        ),
        (
            "rod/inrod",
            fin("rod", rod(Point2::new(0.0, 0.0), 1.0, 0.0, 1.0)),
            fin("ir", rod(Point2::new(0.0, 0.0), 0.5, 0.0, 1.0)),
        ),
        (
            "rod/offrod",
            fin("rod", rod(Point2::new(0.0, 0.0), 1.0, 0.0, 1.0)),
            fin("or", rod(Point2::new(0.3, 0.0), 0.5, 0.0, 0.5)),
        ),
        (
            "box/rodin",
            fin("bx", b((-1.0, 1.0), (-1.0, 1.0), (0.0, 1.0))),
            fin("ri", rod(Point2::new(0.0, 0.0), 0.5, 0.0, 0.5)),
        ),
        (
            "boss/plate",
            fin("boss", extruded(vec![circle(0.0, 0.0, 0.3, 0.0)], 0.0, 1.0)),
            fin("pl", plate()),
        ),
        (
            "boss/plate45",
            fin("boss", extruded(vec![circle(0.0, 0.0, 0.3, 0.7)], 0.0, 1.0)),
            fin("pl", plate()),
        ),
        (
            "boss/holed",
            fin("boss", extruded(vec![circle(0.0, 0.0, 0.3, 0.0)], 0.0, 1.0)),
            fin(
                "hp",
                extruded(vec![square(1.0), circle(0.0, 0.0, 0.1, 0.0)], -0.5, 0.5),
            ),
        ),
        (
            "boss/holedlow",
            fin(
                "boss",
                extruded(vec![circle(0.0, 0.0, 0.3, 0.0)], -0.5, 0.5),
            ),
            fin(
                "hp",
                extruded(vec![square(1.0), circle(0.0, 0.0, 0.1, 0.0)], -0.5, 0.5),
            ),
        ),
        (
            "boss/padin",
            fin("boss", extruded(vec![circle(0.0, 0.0, 0.3, 0.0)], 0.0, 1.0)),
            fin("pad", extruded(vec![circle(0.0, 0.0, 0.2, 1.0)], -0.5, 0.5)),
        ),
        (
            "boss/padin0",
            fin("boss", extruded(vec![circle(0.0, 0.0, 0.3, 0.0)], 0.0, 1.0)),
            fin("pad", extruded(vec![circle(0.0, 0.0, 0.2, 0.0)], 0.0, 0.5)),
        ),
        (
            "boss/squarein",
            fin("boss", extruded(vec![circle(0.0, 0.0, 0.3, 0.0)], 0.0, 1.0)),
            fin("sq", extruded(vec![square(0.1)], 0.0, 0.5)),
        ),
        (
            "boss/hang",
            fin(
                "boss",
                extruded(vec![circle(0.2, 0.1, 0.3, 0.0)], -1.5, 1.0),
            ),
            fin("pl", plate()),
        ),
        (
            "cavity/plug",
            fin(
                "cv",
                cut(
                    "c",
                    &b((0.0, 3.0), (0.0, 3.0), (0.0, 3.0)),
                    &b((1.0, 2.0), (1.0, 2.0), (2.0, 4.0)),
                ),
            ),
            fin("pl", b((1.0, 2.0), (1.0, 2.0), (2.0, 3.0))),
        ),
        (
            "cavity/half",
            fin(
                "cv",
                cut(
                    "c",
                    &b((0.0, 3.0), (0.0, 3.0), (0.0, 3.0)),
                    &b((1.0, 2.0), (1.0, 2.0), (2.0, 4.0)),
                ),
            ),
            fin("hp", b((1.0, 2.0), (1.0, 2.0), (2.0, 2.5))),
        ),
    ]
}

#[test]
#[ignore = "review probe; run with --ignored --nocapture"]
fn fallback_rows_review() {
    for (label, x, y) in corpus() {
        let mut keep = Vec::new();
        for (op, p, q, ord) in [
            ("∪", &x, &y, "xy"),
            ("∪", &y, &x, "yx"),
            ("∩", &x, &y, "xy"),
            ("∩", &y, &x, "yx"),
            ("∖", &x, &y, "xy"),
            ("∖", &y, &x, "yx"),
        ] {
            let got = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| run(op, p, q)));
            match got {
                Ok((line, kind, rows)) => {
                    println!("BODY {label} {op} {ord}: {line}");
                    println!("ROWS {label} {op} {ord}: {:?} {:?}", rows.0, rows.1);
                    keep.push((op, ord, kind, rows));
                }
                Err(_) => println!("BODY {label} {op} {ord}: PANIC"),
            }
        }
        for op in ["∪", "∩"] {
            let xy = keep.iter().find(|k| k.0 == op && k.1 == "xy");
            let yx = keep.iter().find(|k| k.0 == op && k.1 == "yx");
            if let (Some(p), Some(q)) = (xy, yx) {
                let fb = |k: Option<BooleanResultKind>| {
                    matches!(
                        k,
                        Some(BooleanResultKind::OperandA | BooleanResultKind::OperandB)
                    )
                };
                if fb(p.2) != fb(q.2) || (fb(p.2) && fb(q.2)) {
                    let same = p.3 == q.3;
                    println!(
                        "CMP {label} {op}: {:?}/{:?} rows {}",
                        p.2,
                        q.2,
                        if same { "ALIKE" } else { "DIFFER" }
                    );
                    if !same {
                        println!("  xy {:?}\n  yx {:?}", p.3, q.3);
                    }
                }
            }
        }
    }
}
