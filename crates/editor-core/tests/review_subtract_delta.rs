//! **Review rows for a subtract's n-ary fold** (PR 7426, the fix pass
//! that folds a subtract's tools through the union's n-ary core): a
//! subtract of two or more tools builds the body and the contact
//! records the chain of pair cuts `((a − t₁) − t₂) − …` builds, in
//! every order of its tools, where the tools meet the kept body's faces
//! or each other's flush, opposed, tangent or one inside another.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::blocks::{block, block_on, census_of};
use crate::fixture::{self, insert, len, slid};
use editor_core::{
    Bodies, BodyRead, BooleanValue, Evaluation, Formula, LoopProgram, NamedCell, Node, NodeResult,
    Operand, ProfileDoc, ProfileProgram, RecipeNodeId, ValuePayload,
};
use geom_core::Tol;
use std::collections::BTreeMap;

fn run(doc: &ProfileDoc) -> Evaluation<f64> {
    fixture::run(doc, &Default::default())
}

fn plain(id: RecipeNodeId) -> BodyRead<Formula> {
    BodyRead::plain(Operand::Node(id))
}

/// The plate, 3 × 1 × 0.2, on its own xy plane.
fn plate() -> (ProfileDoc, RecipeNodeId) {
    let doc = ProfileDoc::empty_derived("review-subtract-delta", Tol::witness());
    block(doc, (0.0, 3.0), (0.0, 1.0), 0.2)
}

fn tool(
    doc: ProfileDoc,
    plate: RecipeNodeId,
    xs: (f64, f64),
    ys: (f64, f64),
    (z0, dz): (f64, f64),
) -> (ProfileDoc, RecipeNodeId) {
    block_on(doc, plate, xs, ys, z0, dz)
}

/// A disc of radius `r` about `(cx, cy)`, from `z0` up `dz`, placed in
/// the plate's space.
fn disc(
    doc: ProfileDoc,
    plate: RecipeNodeId,
    (cx, cy): (f64, f64),
    r: f64,
    (z0, dz): (f64, f64),
) -> (ProfileDoc, RecipeNodeId) {
    let (doc, p) = insert(
        doc,
        Node::Profile(ProfileProgram {
            loops: vec![LoopProgram::circle(cx, cy, r).expect("a finite circle")],
            ids: Vec::new(),
            on: None,
        }),
    );
    let (doc, d) = insert(
        doc,
        Node::Extrude {
            profile: p.into(),
            distance: len(dz),
        },
    );
    slid(doc, plate, d, [0.0, 0.0, z0])
}

fn permutations(items: &[RecipeNodeId]) -> Vec<Vec<RecipeNodeId>> {
    if items.len() <= 1 {
        return vec![items.to_vec()];
    }
    let mut out = Vec::new();
    for i in 0..items.len() {
        let mut rest = items.to_vec();
        let head = rest.remove(i);
        for mut tail in permutations(&rest) {
            tail.insert(0, head);
            out.push(tail);
        }
    }
    out
}

fn the_body(ev: &Evaluation<f64>, id: RecipeNodeId) -> &topo::Body<f64> {
    match ev.value(id).map(|v| &v.payload) {
        Some(ValuePayload::Boolean(BooleanValue::Body { body, .. }) | ValuePayload::Body(body)) => {
            body.body()
        }
        _ => panic!("{id:?} is not one body: {:?}", ev.result(id)),
    }
}

fn q(x: f64) -> i64 {
    (x * 1e7).round() as i64
}

/// A body and its records, said by geometry alone, so two nodes over
/// different operand reads compare.
#[derive(Debug, PartialEq)]
struct Shape {
    volume: i64,
    solids: usize,
    faces: usize,
    edges: usize,
    vertices: Vec<[i64; 3]>,
    /// Each record as its two cells, a vertex by its point, an edge by
    /// its ends, a face by its kind alone, sorted.
    records: Vec<String>,
    /// The tier-3′ census over the body and its records: its findings'
    /// kinds.
    census: Result<(), Vec<String>>,
}

fn shape(ev: &Evaluation<f64>, id: RecipeNodeId) -> Result<Shape, String> {
    match ev.result(id) {
        Some(NodeResult::Failed(e)) => return Err(format!("{:?}", e.kind)),
        Some(NodeResult::Poisoned { through }) => {
            return Err(format!(
                "poisoned through {through:?}: {:?}",
                ev.result(*through)
            ));
        }
        _ => {}
    }
    let b = the_body(ev, id);
    let census = census_of(ev, id).map_err(|e| {
        e.iter()
            .map(|f| format!("{f:?}").split('{').next().unwrap_or("").to_string())
            .collect::<Vec<_>>()
    });
    let measured = topo::test_support::finished("the measured body", b.clone(), Tol::witness());
    let volume =
        topo::test_support::pinned_volume("the measured body", &measured, Tol::witness()).volume;
    let points: BTreeMap<_, _> = b.vertex_points().collect();
    let at = |v| {
        let p = points[&v];
        [q(p.x), q(p.y), q(p.z)]
    };
    let cell = |c: topo::Cell| match c {
        topo::Cell::Vertex(v) => format!("V{:?}", at(v)),
        topo::Cell::Edge(e) => {
            let (u, v) = b.edge_vertices(e).expect("a live edge");
            let mut ends = [at(u), at(v)];
            ends.sort_unstable();
            format!("E{ends:?}")
        }
        topo::Cell::Face(_) => "F".to_string(),
    };
    let value = ev.value(id).unwrap();
    let mut records: Vec<String> = value
        .carry
        .contacts
        .cell_pairs()
        .map(|(x, y)| {
            let mut two = [cell(x), cell(y)];
            two.sort();
            format!("{} ~ {}", two[0], two[1])
        })
        .collect();
    records.sort();
    let mut vertices: Vec<[i64; 3]> = points.values().map(|p| [q(p.x), q(p.y), q(p.z)]).collect();
    vertices.sort_unstable();
    Ok(Shape {
        volume: q(volume),
        solids: b.solids().count(),
        faces: b.faces().count(),
        edges: b.edges().count(),
        vertices,
        records,
        census,
    })
}

/// The scenarios, each the plate and its tools.
fn scenarios() -> Vec<(&'static str, ProfileDoc, RecipeNodeId, Vec<RecipeNodeId>)> {
    let through = (-1.0, 3.0);
    let mut out = Vec::new();
    {
        // Two holes sharing a wall (opposed senses) and a third inside
        // the first.
        let (doc, p) = plate();
        let (doc, a) = tool(doc, p, (0.5, 1.0), (0.25, 0.75), through);
        let (doc, b) = tool(doc, p, (1.0, 1.5), (0.25, 0.75), through);
        let (doc, c) = tool(doc, p, (0.6, 0.9), (0.4, 0.6), (-0.5, 1.0));
        out.push(("shared wall, one inside", doc, p, vec![a, b, c]));
    }
    {
        // Pockets flush with the plate's top: each tool's top on the
        // plate's top plane, aligned; the pockets overlap.
        let (doc, p) = plate();
        let (doc, a) = tool(doc, p, (0.5, 1.0), (0.25, 0.75), (0.1, 0.1));
        let (doc, b) = tool(doc, p, (0.75, 1.25), (0.25, 0.75), (0.1, 0.1));
        let (doc, c) = tool(doc, p, (1.0, 1.5), (0.4, 0.6), (0.05, 0.15));
        out.push(("pockets flush with the top", doc, p, vec![a, b, c]));
    }
    {
        // A tool inside another sharing three of its walls' planes
        // (aligned senses), and a third crossing both.
        let (doc, p) = plate();
        let (doc, a) = tool(doc, p, (0.5, 1.5), (0.25, 0.75), through);
        let (doc, b) = tool(doc, p, (0.5, 1.0), (0.25, 0.75), through);
        let (doc, c) = tool(doc, p, (0.75, 1.25), (-1.0, 0.5), through);
        out.push(("one inside another, flush", doc, p, vec![a, b, c]));
    }
    {
        // Two tools touching face to face only outside the plate, and
        // a third through both.
        let (doc, p) = plate();
        let (doc, a) = tool(doc, p, (0.5, 1.0), (0.25, 0.75), (-1.0, 2.0));
        let (doc, b) = tool(doc, p, (1.0, 1.5), (0.25, 0.75), (1.0, 1.0));
        let (doc, c) = tool(doc, p, (0.75, 1.25), (0.4, 0.6), through);
        out.push(("touching outside the plate", doc, p, vec![a, b, c]));
    }
    {
        // A round hole tangent to a slot's wall, and a block crossing
        // the hole.
        let (doc, p) = plate();
        let (doc, a) = disc(doc, p, (1.0, 0.5), 0.25, through);
        let (doc, b) = tool(doc, p, (1.25, 1.75), (0.2, 0.8), through);
        let (doc, c) = tool(doc, p, (0.6, 1.1), (0.45, 0.55), through);
        out.push(("round hole tangent to a slot", doc, p, vec![a, b, c]));
    }
    {
        // A round hole and a block tangent to it only above the plate,
        // where no cut reaches: the chain never meets the tangency.
        let (doc, p) = plate();
        let (doc, a) = disc(doc, p, (1.0, 0.5), 0.25, through);
        let (doc, b) = tool(doc, p, (1.25, 1.75), (0.2, 0.8), (1.0, 1.0));
        let (doc, c) = tool(doc, p, (2.0, 2.5), (0.25, 0.75), through);
        out.push(("tangent above the plate", doc, p, vec![a, b, c]));
    }
    {
        // Two round holes tangent to each other.
        let (doc, p) = plate();
        let (doc, a) = disc(doc, p, (1.0, 0.5), 0.25, through);
        let (doc, b) = disc(doc, p, (1.5, 0.5), 0.25, through);
        let (doc, c) = tool(doc, p, (2.0, 2.5), (0.25, 0.75), through);
        out.push(("round holes tangent", doc, p, vec![a, b, c]));
    }
    out
}

/// The n-ary subtract of `tools` from `p`, and the chain of pair cuts
/// in the same order.
fn nary_and_chain(
    doc: ProfileDoc,
    p: RecipeNodeId,
    tools: &[RecipeNodeId],
) -> (ProfileDoc, RecipeNodeId, RecipeNodeId) {
    let (doc, nary) = insert(
        doc,
        Node::Subtract {
            from: plain(p),
            tool: Bodies::spelled(tools.iter().copied().map(plain).collect()),
        },
    );
    let mut doc = doc;
    let mut acc = p;
    for &t in tools {
        let (d, cut) = insert(
            doc,
            Node::Subtract {
                from: plain(acc),
                tool: Bodies::spelled(vec![plain(t)]),
            },
        );
        doc = d;
        acc = cut;
    }
    (doc, nary, acc)
}

/// What each scenario's n-ary subtract and its chain said, two and
/// three tools, in every order: a refusal is said as refused, whatever
/// wraps the pair boolean's error (the chain's later cuts are poisoned
/// through its first refused one, the n-ary door names its step).
#[allow(clippy::type_complexity)]
fn every_order() -> Vec<(
    String,
    Vec<(Vec<usize>, Result<Shape, ()>, Result<Shape, ()>)>,
)> {
    let mut out = Vec::new();
    for (what, doc, p, tools) in scenarios() {
        for n in [2, 3] {
            let set = &tools[..n];
            let mut orders = Vec::new();
            for order in permutations(set) {
                let (doc, nary, chain) = nary_and_chain(doc.clone(), p, &order);
                let ev = run(&doc);
                let place: Vec<usize> = order
                    .iter()
                    .map(|t| set.iter().position(|s| s == t).unwrap())
                    .collect();
                let said = |r: Result<Shape, String>| {
                    r.map_err(|e| eprintln!("{what} {place:?}: refused {e}"))
                };
                orders.push((
                    place.clone(),
                    said(shape(&ev, nary)),
                    said(shape(&ev, chain)),
                ));
            }
            out.push((format!("{what}, {n} tools"), orders));
        }
    }
    out
}

/// **A subtract of two or more tools is the chain of its pair cuts**:
/// the same body (volume, solids, faces, edges, vertex points), the
/// same contact records by geometry and the same census verdict, or a
/// refusal where the chain refuses. Red where reading a verdict or a
/// holding through a tool face with its senses reversed builds other
/// than the chain does.
#[test]
fn a_subtract_of_many_tools_is_the_chain_of_its_cuts() {
    let mut differ = Vec::new();
    for (what, orders) in every_order() {
        for (order, nary, chain) in orders {
            if nary != chain {
                differ.push(format!(
                    "{what}, {order:?}: the n-ary subtract\n{nary:#?}\nagainst the chain\n{chain:#?}"
                ));
            }
        }
    }
    assert!(differ.is_empty(), "{}", differ.join("\n\n"));
}

/// **Every order of the tools builds one body and one record list**
/// (DM4), or refuses in every order. Red where the result, its records
/// or whether it builds depends on the order the tools are listed in.
/// A scenario whose chain of pair cuts already depends on the order (the
/// round hole tangent to a slot, which the pair boolean refuses or
/// builds by which cut meets the tangency first) is the pair boolean's,
/// not the fold's, and is left out.
#[test]
fn every_order_of_the_tools_builds_alike() {
    let mut differ = Vec::new();
    for (what, orders) in every_order() {
        let (first, base, chain) = &orders[0];
        if orders[1..].iter().any(|(_, _, c)| c != chain) {
            eprintln!("{what}: the chain itself depends on the order; left out");
            continue;
        }
        for (order, nary, _) in &orders[1..] {
            if nary != base {
                differ.push(format!(
                    "{what}: {order:?} says\n{nary:#?}\nagainst {first:?}'s\n{base:#?}"
                ));
            }
        }
    }
    assert!(differ.is_empty(), "{}", differ.join("\n\n"));
}

/// **No row of a subtract relates two tools alone where no tool face
/// stands in the kept body**: two tools that touch only outside the
/// plate leave no row between them, as the chain records none. Red
/// where the tools' union judgement's rows are published as the cut's.
#[test]
fn tools_touching_outside_the_kept_body_publish_no_row_between_them() {
    let (doc, p) = plate();
    let (doc, a) = tool(doc, p, (0.5, 1.0), (0.25, 0.75), (-1.0, 2.0));
    let (doc, b) = tool(doc, p, (1.0, 1.5), (0.25, 0.75), (1.0, 1.0));
    let (doc, c) = tool(doc, p, (2.0, 2.5), (0.25, 0.75), (-1.0, 3.0));
    let (doc, cut) = insert(
        doc,
        Node::Subtract {
            from: plain(p),
            tool: Bodies::spelled(vec![plain(a), plain(b), plain(c)]),
        },
    );
    let ev = run(&doc);
    let reads: Vec<editor_core::VarId> = [a, b]
        .iter()
        .map(|&t| doc.output(t, 0).expect("a tool's read"))
        .collect();
    let value = ev.value(cut).expect("the cut evaluates");
    let between: Vec<String> = value
        .coincidences
        .iter()
        .filter(|r| {
            r.cells.iter().all(|c| match c {
                NamedCell::Entity { input, .. } => reads.contains(input),
                _ => false,
            })
        })
        .map(|r| format!("{:?} @ {:?}: {:?}", r.relation, r.site, r.cells))
        .collect();
    assert!(
        between.is_empty(),
        "the cut publishes rows between two tools that meet only outside the plate: {between:#?}"
    );
}
