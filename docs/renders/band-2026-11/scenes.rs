//! The bodies behind the band renders, authored through `pncad` the way
//! a user would: profiles, extrusions and booleans, then one blend call
//! on edges picked by where they lie. Each scene writes its operand and
//! every result that builds as binary STL; a refusal is printed and
//! recorded in `refusals.txt`, never hidden.
//!
//! `render.sh` copies this file into a throwaway crate that depends on a
//! checkout's `crates/pncad` and runs it with one group name: `mixed`
//! (the mixed-corner step 1), `knife` or `chain` (the rounded outline).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use std::fmt::Write as _;

use pncad::authoring::{p2, polygon, validated};
use pncad::geom_core::{Point3, Tol};
use pncad::prelude::{Open, Start};
use pncad::profile::{ConstructedLoop, SketchPlane};
use pncad::sweep::blend::{chamfer_edges, fillet_edges};
use pncad::sweep::{ExtrudeSide, Extrusion, extrude};
use pncad::topo::{AtRestBody, Body, BooleanResult, EdgeKey, query};

const DELTA: f64 = 1e-3;

fn tol() -> Tol {
    Tol::witness()
}

/// `loops` on the plane `z = z0`, extruded up `h`.
fn slab(loops: Vec<ConstructedLoop<f64>>, z0: f64, h: f64) -> AtRestBody<f64> {
    let plane = SketchPlane::xy_at(Point3::new(0.0, 0.0, z0));
    extrude(
        &validated(plane, loops, tol()).expect("the profile validates"),
        Extrusion::Distance {
            depth: h,
            side: ExtrudeSide::Along,
        },
        tol(),
    )
    .expect("the profile extrudes")
    .body
}

fn poly(points: &[(f64, f64)]) -> ConstructedLoop<f64> {
    polygon(points, tol()).expect("the outline")
}

fn circle(cx: f64, cy: f64, r: f64) -> ConstructedLoop<f64> {
    pncad::profile::circle(p2(cx, cy), r, tol())
        .expect("a positive radius")
        .into()
}

fn body_of(what: &str, r: Result<BooleanResult<f64>, pncad::topo::BooleanError>) -> AtRestBody<f64> {
    match r {
        Ok(BooleanResult::Body(b)) => b.body,
        other => panic!("{what}: {other:?}"),
    }
}

fn union(a: &AtRestBody<f64>, b: &AtRestBody<f64>) -> AtRestBody<f64> {
    body_of("union", pncad::topo::union(a, b, tol()))
}

fn point(body: &Body<f64>, v: pncad::topo::VertexKey) -> Point3<f64> {
    *body.get_point(body.get_vertex(v).unwrap().point).unwrap()
}

fn near(p: &Point3<f64>, q: [f64; 3]) -> bool {
    (p.x - q[0]).abs() + (p.y - q[1]).abs() + (p.z - q[2]).abs() < 1e-9
}

/// The edge from `a` to `b`, either way round.
fn edge(body: &Body<f64>, a: [f64; 3], b: [f64; 3]) -> EdgeKey {
    let found: Vec<EdgeKey> = query::all_edges(body)
        .into_iter()
        .filter(|&e| {
            let (v0, v1) = body.edge_vertices(e).unwrap();
            let (p, q) = (point(body, v0), point(body, v1));
            (near(&p, a) && near(&q, b)) || (near(&p, b) && near(&q, a))
        })
        .collect();
    let [e] = found[..] else {
        panic!("one edge {a:?}–{b:?}, found {found:?}");
    };
    e
}

/// The edges whose two ends satisfy `at`.
fn edges_where(body: &Body<f64>, at: impl Fn(&Point3<f64>) -> bool) -> Vec<EdgeKey> {
    query::all_edges(body)
        .into_iter()
        .filter(|&e| {
            let (v0, v1) = body.edge_vertices(e).unwrap();
            at(&point(body, v0)) && at(&point(body, v1))
        })
        .collect()
}

struct Out {
    dir: std::path::PathBuf,
    log: String,
}

impl Out {
    fn stl(&mut self, name: &str, body: &Body<f64>) {
        let mesh = pncad::mesh::tessellate(body, DELTA, tol())
            .unwrap_or_else(|e| panic!("{name}: tessellate refused: {e:?}"));
        let options = pncad::stl::BinaryOptions {
            header: pncad::stl::BinaryHeader::new(name.to_string()).expect("an STL header"),
        };
        let mut buf = Vec::new();
        pncad::stl::write_binary(&mesh, &options, &mut buf).expect("the STL writes");
        std::fs::write(self.dir.join(format!("{name}.stl")), buf).expect("write the STL");
        println!("wrote {name}.stl");
    }

    /// Writes the operand, then each verb's result at `size` or its refusal.
    fn blend(&mut self, name: &str, body: &AtRestBody<f64>, edges: &[EdgeKey], size: f64, verbs: &[&str]) {
        self.stl(&format!("{name}-before"), body);
        for &verb in verbs {
            let got = match verb {
                "fillet" => fillet_edges(body, edges, size, tol()).map(|b| b.body),
                "chamfer" => chamfer_edges(body, edges, size, tol()).map(|b| b.body),
                _ => unreachable!(),
            };
            match got {
                Ok(after) => self.stl(&format!("{name}-{verb}"), &after),
                Err(e) => {
                    let line = format!(
                        "{name}-{verb} at {size}: refused {:?}\n  {}\n",
                        e.error,
                        e.render(pncad::geom_brep::recourse::Reading::Build)
                    );
                    print!("{line}");
                    self.log.push_str(&line);
                }
            }
        }
    }
}

// ---------------------------------------------------------------------
// The mixed corner, step 1.
// ---------------------------------------------------------------------

/// An L section, legs 3 and 2 long and 0.5 thick, extruded 1 up.
fn l_bar() -> AtRestBody<f64> {
    slab(
        vec![poly(&[
            (0.0, 0.0),
            (3.0, 0.0),
            (3.0, 0.5),
            (0.5, 0.5),
            (0.5, 2.0),
            (0.0, 2.0),
        ])],
        0.0,
        1.0,
    )
}

/// A 6 × 4 × 0.5 plate and a 0.5 × 3 boss standing on it to z = 2, its
/// low side at `x = inset`.
fn plate_boss(inset: f64) -> AtRestBody<f64> {
    let plate = slab(vec![poly(&[(0.0, 0.0), (6.0, 0.0), (6.0, 4.0), (0.0, 4.0)])], 0.0, 0.5);
    let boss = slab(
        vec![poly(&[(inset, 0.5), (inset + 0.5, 0.5), (inset + 0.5, 3.5), (inset, 3.5)])],
        0.25,
        1.75,
    );
    union(&plate, &boss)
}

fn mixed(out: &mut Out, size: f64) {
    let both = ["fillet", "chamfer"];
    let bar = l_bar();
    let inside = edge(&bar, [0.5, 0.5, 0.0], [0.5, 0.5, 1.0]);
    out.blend("lbar-inside", &bar, &[inside], size, &both);

    let inset = plate_boss(1.0);
    let up = edge(&inset, [1.0, 0.5, 0.5], [1.0, 0.5, 2.0]);
    out.blend("boss-vertical", &inset, &[up], size, &both);

    let flush = plate_boss(0.0);
    let rim = edge(&flush, [0.0, 0.0, 0.5], [0.0, 0.5, 0.5]);
    let up = edge(&flush, [0.0, 0.5, 0.5], [0.0, 0.5, 2.0]);
    out.blend("flush-pair", &flush, &[rim, up], size, &both);

    let front = edge(&inset, [1.0, 0.5, 0.5], [1.5, 0.5, 0.5]);
    let side = edge(&inset, [1.0, 0.5, 0.5], [1.0, 3.5, 0.5]);
    out.blend("inset-pair", &inset, &[front, side], size, &both);
    let back = edge(&inset, [1.0, 3.5, 0.5], [1.5, 3.5, 0.5]);
    let far = edge(&inset, [1.5, 0.5, 0.5], [1.5, 3.5, 0.5]);
    out.blend("inset-rim", &inset, &[front, side, back, far], size, &both);
}

// ---------------------------------------------------------------------
// The knife edge.
// ---------------------------------------------------------------------

fn on_the_z_axis(p: &Point3<f64>) -> bool {
    p.x.abs() < 1e-9 && p.y.abs() < 1e-9
}

/// The lune between the circles about `(0, 1)` radius 1 and `(0, 2)`
/// radius 2, `x > 0`: its two arcs meet tangent at the origin, a cusp.
fn lune() -> ConstructedLoop<f64> {
    use core::f64::consts::FRAC_PI_2;
    Open.at(p2(0.0, 4.0))
        .angle(-FRAC_PI_2, tol())
        .unwrap()
        .line(2.0, tol())
        .unwrap()
        .turn(FRAC_PI_2, tol())
        .unwrap()
        .tangent_arc_to(p2(0.0, 0.0), tol())
        .unwrap()
        .cusp()
        .tangent_arc_to(Start, tol())
        .unwrap()
        .into()
}

fn knife(out: &mut Out, size: f64) {
    let plate = slab(vec![poly(&[(-1.8, -1.5), (1.8, -1.5), (1.8, 0.0), (-1.8, 0.0)])], 0.0, 1.0);
    let left = slab(vec![circle(-1.0, 0.0, 1.0)], 0.0, 1.0);
    let right = slab(vec![circle(1.0, 0.0, 1.0)], 0.0, 1.0);
    let valley = union(&union(&plate, &left), &right);
    let kiss = edges_where(&valley, on_the_z_axis);
    assert_eq!(kiss.len(), 1, "one knife on the kiss, found {kiss:?}");
    out.blend("valley", &valley, &kiss, size, &["fillet"]);

    let cusp = slab(vec![lune()], 0.0, 1.0);
    let tip = edges_where(&cusp, on_the_z_axis);
    assert_eq!(tip.len(), 1, "one knife at the lune's tip, found {tip:?}");
    out.blend("lune-cusp", &cusp, &tip, size, &["fillet"]);
}

// ---------------------------------------------------------------------
// The rounded outline.
// ---------------------------------------------------------------------

/// `[−1, 1] × [−0.6, 0.6]` about `(x0, y0)`, its corners rounded at 0.3.
fn rounded_rect(x0: f64, y0: f64) -> ConstructedLoop<f64> {
    let (hx, hy, r) = (1.0, 0.6, 0.3);
    let p = |x: f64, y: f64| p2(x0 + x, y0 + y);
    Open.at(p(-(hx - r), -hy))
        .line_to(p(hx - r, -hy), tol())
        .unwrap()
        .tangent()
        .tangent_arc_to(p(hx, -(hy - r)), tol())
        .unwrap()
        .line_to(p(hx, hy - r), tol())
        .unwrap()
        .tangent()
        .tangent_arc_to(p(hx - r, hy), tol())
        .unwrap()
        .line_to(p(-(hx - r), hy), tol())
        .unwrap()
        .tangent()
        .tangent_arc_to(p(-hx, hy - r), tol())
        .unwrap()
        .line_to(p(-hx, -(hy - r)), tol())
        .unwrap()
        .tangent()
        .tangent_arc_to(Start, tol())
        .unwrap()
        .into()
}

fn chain(out: &mut Out, size: f64) {
    let plate = slab(vec![rounded_rect(0.0, 0.0)], 0.0, 0.5);
    let all = query::all_edges(&plate);
    out.blend("rounded-plate", &plate, &all, size, &["fillet", "chamfer"]);

    let block = slab(vec![poly(&[(0.0, 0.0), (3.0, 0.0), (3.0, 2.0), (0.0, 2.0)])], 0.0, 1.0);
    let cut = slab(vec![rounded_rect(1.5, 1.0)], 0.5, 1.0);
    let pocket = body_of("the pocket", pncad::topo::subtract(&block, &cut, tol()));
    let floor = edges_where(&pocket, |p| (p.z - 0.5).abs() < 1e-9);
    assert_eq!(floor.len(), 8, "the floor's rim: four lines and four arcs");
    out.blend("pocket-floor", &pocket, &floor, size, &["fillet"]);
    let mouth = edges_where(&pocket, |p| {
        (p.z - 1.0).abs() < 1e-9 && (p.x - 1.5).abs() <= 1.0 + 1e-9 && (p.y - 1.0).abs() <= 0.6 + 1e-9
    });
    assert_eq!(mouth.len(), 8, "the mouth: four lines and four arcs");
    out.blend("pocket-mouth", &pocket, &mouth, size, &["fillet"]);
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let [_, group, size, dir] = &args[..] else {
        panic!("usage: scenes <mixed|knife|chain> <size> <out-dir>");
    };
    let size: f64 = size.parse().expect("a size");
    let mut out = Out {
        dir: dir.into(),
        log: String::new(),
    };
    std::fs::create_dir_all(&out.dir).expect("the out dir");
    match group.as_str() {
        "mixed" => mixed(&mut out, size),
        "knife" => knife(&mut out, size),
        "chain" => chain(&mut out, size),
        _ => panic!("unknown group {group}"),
    }
    let mut log = String::new();
    writeln!(log, "{group} at size {size}").unwrap();
    log.push_str(&out.log);
    std::fs::write(out.dir.join(format!("refusals-{group}.txt")), log).expect("write the log");
}
