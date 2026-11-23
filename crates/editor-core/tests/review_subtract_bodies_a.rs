//! **Review rows for a subtract's tool list** (DESIGN `[repetition]`,
//! REFERENCES DM4: the cut is the fold `a − t₁ − … − tₙ`, order-free in
//! the tools, fusing none of them with another). Each row states one
//! claim of the tool list's lowering and goes red where it fails.
#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::blocks::{block, block_on, census_of};
use crate::fixture::{self, insert, step};
use editor_core::{
    Bodies, BodyRead, BooleanValue, CitedInput, DocEdit, Evaluation, Formula, NamedCell, Node,
    NodeErrorKind, NodeResult, Operand, ProfileDoc, RecipeNodeId, TargetAxis, ValuePayload,
};
use geom_core::Tol;

fn run(doc: &ProfileDoc) -> Evaluation<f64> {
    fixture::run(doc, &Default::default())
}

fn plain(id: RecipeNodeId) -> BodyRead<Formula> {
    BodyRead::plain(Operand::Node(id))
}

/// The plate, 3 × 1 × 0.2, on its own xy plane.
fn plate() -> (ProfileDoc, RecipeNodeId) {
    let doc = ProfileDoc::empty_derived("review-subtract-bodies", Tol::witness());
    block(doc, (0.0, 3.0), (0.0, 1.0), 0.2)
}

/// A block in the plate's space over `xs` × `ys`, from `z0` up `dz`.
fn tool(
    doc: ProfileDoc,
    plate: RecipeNodeId,
    xs: (f64, f64),
    ys: (f64, f64),
    (z0, dz): (f64, f64),
) -> (ProfileDoc, RecipeNodeId) {
    block_on(doc, plate, xs, ys, z0, dz)
}

/// **`from` less `tools` in the order given, under one node id**: the
/// tools inserted sorted, then re-ordered through `SetMembers`, as
/// `fixture::union_over` does for a union, so every order of one tool
/// set is one node whose names embed one id.
fn subtract_over(
    doc: ProfileDoc,
    from: RecipeNodeId,
    tools: &[RecipeNodeId],
) -> (ProfileDoc, RecipeNodeId) {
    let mut sorted = tools.to_vec();
    sorted.sort();
    let (doc, cut) = insert(
        doc,
        Node::Subtract {
            from: plain(from),
            tool: Bodies::spelled(sorted.iter().copied().map(plain).collect()),
        },
    );
    if sorted == tools {
        return (doc, cut);
    }
    let (doc, _) = step(
        doc,
        DocEdit::SetMembers {
            node: cut,
            members: Bodies::spelled(tools.iter().copied().map(plain).collect()),
        },
    );
    (doc, cut)
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

fn volume(b: &topo::Body<f64>) -> f64 {
    let measured = topo::test_support::finished("the measured body", b.clone(), Tol::witness());
    topo::test_support::pinned_volume("the measured body", &measured, Tol::witness()).volume
}

/// What one evaluation of a subtract says, each part sorted so two
/// orders compare part by part.
#[derive(Debug, PartialEq)]
struct Said {
    names: Vec<String>,
    rows: Vec<String>,
    records: Vec<String>,
    cited: Vec<String>,
}

/// The node's names, rows and records, its own id said `SELF`. A row
/// is said without its origin (a fold step's index is its place in
/// the order, which an order-free cut does not keep) and without its
/// margin's bits.
fn said(ev: &Evaluation<f64>, id: RecipeNodeId) -> Said {
    let own = format!("{id:?}");
    let s = |text: String| text.replace(&own, "SELF");
    let mut names: Vec<String> = ev
        .table_at(id)
        .unwrap()
        .iter()
        .map(|(n, _)| s(format!("{:?} {:?}", n.kind, n.path)))
        .collect();
    names.sort();
    let value = ev.value(id).expect("evaluated");
    let row = |r: &editor_core::NamedCoincidence| {
        let mut cells: Vec<String> = r
            .cells
            .iter()
            .map(|c| match c {
                NamedCell::Entity { input, name } => {
                    s(format!("{input:?}:{:?} {:?}", name.kind, name.path))
                }
                other => s(format!("{other:?}")),
            })
            .collect();
        // A symmetric relation's cells in the order the decision read
        // them, which is the fold's: said unordered.
        if !matches!(
            r.relation,
            topo::Relation::OnCarrier | topo::Relation::CommonFoot | topo::Relation::Tangent { .. }
        ) {
            cells.sort();
        }
        format!("{:?} @ {:?} [{}]", r.relation, r.site, cells.join(" | "))
    };
    // The judged rows alone: DM4 makes a fold's judged rows order-free
    // modulo the members' places, and its fold-step rows follow the
    // member order, for a union's members and a subtract's tools alike
    // (`work/formwords/fold-step-point-rows-depend-on-the-member-order.md`).
    let mut rows: Vec<String> = value
        .coincidences
        .iter()
        .filter(|r| matches!(r.origin, editor_core::RowOrigin::Judged { .. }))
        .map(row)
        .collect();
    rows.sort();
    let inputs = value.carry.cited_inputs.as_ref();
    let mut records: Vec<String> = value
        .carry
        .contacts
        .rows()
        .iter()
        .map(|r| {
            let mut cites: Vec<String> = r
                .cites
                .iter()
                .map(|b| match b {
                    topo::Backing::Decided(k) => {
                        format!("row {}", row(&value.coincidences[k as usize]))
                    }
                    topo::Backing::Carried { input, record } => format!(
                        "{:?} record {record}",
                        inputs.get(input as usize).unwrap_or_else(|| panic!(
                            "{id:?} cites input {input}, past its {} cited inputs",
                            inputs.len()
                        ))
                    ),
                })
                .collect();
            cites.sort();
            cites.join(" & ")
        })
        .collect();
    records.sort();
    let mut cited: Vec<String> = inputs.iter().map(|c| format!("{c:?}")).collect();
    cited.sort();
    Said {
        names,
        rows,
        records,
        cited,
    }
}

/// Every vertex point's bits, sorted.
fn vertex_bits(b: &topo::Body<f64>) -> Vec<[u64; 3]> {
    let mut v: Vec<[u64; 3]> = b
        .vertex_points()
        .map(|(_, p)| [p.x.to_bits(), p.y.to_bits(), p.z.to_bits()])
        .collect();
    v.sort_unstable();
    v
}

/// The scenarios the order rows run, each the plate and its tools.
fn scenarios() -> Vec<(&'static str, ProfileDoc, RecipeNodeId, Vec<RecipeNodeId>)> {
    let through = (-1.0, 3.0);
    let mut out = Vec::new();
    {
        let (doc, p) = plate();
        let (doc, a) = tool(doc, p, (0.25, 0.5), (0.25, 0.75), through);
        let (doc, b) = tool(doc, p, (1.25, 1.5), (0.25, 0.75), through);
        let (doc, c) = tool(doc, p, (2.25, 2.5), (0.25, 0.75), through);
        out.push(("disjoint holes", doc, p, vec![a, b, c]));
    }
    {
        let (doc, p) = plate();
        let (doc, a) = tool(doc, p, (0.25, 1.0), (0.25, 0.75), through);
        let (doc, b) = tool(doc, p, (0.75, 1.5), (0.3, 0.7), through);
        let (doc, c) = tool(doc, p, (0.25, 0.5), (0.25, 0.75), through);
        out.push(("overlapping holes", doc, p, vec![a, b, c]));
    }
    {
        // Two holes sharing a wall at x = 1 and their y walls' planes,
        // and a third resting on both at y = 0.5.
        let (doc, p) = plate();
        let (doc, a) = tool(doc, p, (0.5, 1.0), (0.25, 0.5), through);
        let (doc, b) = tool(doc, p, (1.0, 1.5), (0.25, 0.5), through);
        let (doc, c) = tool(doc, p, (0.75, 1.25), (0.5, 0.75), through);
        out.push(("holes sharing faces", doc, p, vec![a, b, c]));
    }
    {
        // Two slabs across the plate, each parting it, and a hole.
        let (doc, p) = plate();
        let (doc, a) = tool(doc, p, (1.0, 1.25), (-1.0, 2.0), through);
        let (doc, b) = tool(doc, p, (2.0, 2.25), (-1.0, 2.0), through);
        let (doc, c) = tool(doc, p, (0.25, 0.5), (0.25, 0.75), through);
        out.push(("slabs parting the plate", doc, p, vec![a, b, c]));
    }
    {
        // Three pockets down to one floor at z = 0.1, overlapping.
        let (doc, p) = plate();
        let floor = (0.1, 2.9);
        let (doc, a) = tool(doc, p, (0.5, 1.0), (0.25, 0.75), floor);
        let (doc, b) = tool(doc, p, (0.75, 1.25), (0.25, 0.75), floor);
        let (doc, c) = tool(doc, p, (1.0, 1.5), (0.4, 0.6), floor);
        out.push(("pockets on one floor", doc, p, vec![a, b, c]));
    }
    {
        // Two pockets that each reach the plate's edge, sharing the
        // plate's side plane.
        let (doc, p) = plate();
        let floor = (0.1, 2.9);
        let (doc, a) = tool(doc, p, (-1.0, 0.5), (0.25, 0.75), floor);
        let (doc, b) = tool(doc, p, (-1.0, 0.25), (0.0, 1.0), (0.05, 2.95));
        let (doc, c) = tool(doc, p, (0.4, 0.6), (0.5, 2.0), floor);
        out.push(("pockets through the edge", doc, p, vec![a, b, c]));
    }
    out
}

/// One evaluation per order of each scenario's tools, two and three of
/// them: what each order said, its body's vertex bits, its volume and
/// solids, or its refusal.
#[allow(clippy::type_complexity)]
fn every_order() -> Vec<(
    String,
    Vec<(
        Vec<usize>,
        Result<(Said, Vec<[u64; 3]>, f64, usize), String>,
    )>,
)> {
    let mut out = Vec::new();
    for (what, doc, p, tools) in scenarios() {
        for n in [2, 3] {
            let set = &tools[..n];
            let mut orders = Vec::new();
            for order in permutations(set) {
                let (doc, cut) = subtract_over(doc.clone(), p, &order);
                let ev = run(&doc);
                let place: Vec<usize> = order
                    .iter()
                    .map(|t| set.iter().position(|s| s == t).unwrap())
                    .collect();
                let got = match ev.result(cut) {
                    Some(NodeResult::Failed(e)) => Err(format!("{:?}", e.kind)),
                    _ => {
                        let b = the_body(&ev, cut);
                        if let Err(e) = census_of(&ev, cut) {
                            panic!("{what} {place:?}: the cut fails its census: {e:?}");
                        }
                        Ok((
                            said(&ev, cut),
                            vertex_bits(b),
                            volume(b),
                            b.solids().count(),
                        ))
                    }
                };
                orders.push((place, got));
            }
            out.push((format!("{what}, {n} tools"), orders));
        }
    }
    out
}

/// Each order's `part` of what it said, against the first order's,
/// over every scenario: the differences, said.
fn order_differences(parts: &[&str]) -> Vec<String> {
    let mut differ = Vec::new();
    for (what, orders) in every_order() {
        let (first, base) = &orders[0];
        for (order, got) in &orders[1..] {
            match (base, got) {
                (Ok((a, _, va, sa)), Ok((b, _, vb, sb))) => {
                    if (va - vb).abs() > 1e-12 || sa != sb {
                        differ.push(format!(
                            "{what} {order:?}: volume {vb} / {sb} solids against {va} / {sa} at {first:?}"
                        ));
                    }
                    for (part, x, y) in [
                        ("names", &a.names, &b.names),
                        ("rows", &a.rows, &b.rows),
                        ("records", &a.records, &b.records),
                        ("cited", &a.cited, &b.cited),
                    ] {
                        if parts.contains(&part) && x != y {
                            let only_a: Vec<_> = x.iter().filter(|e| !y.contains(e)).collect();
                            let only_b: Vec<_> = y.iter().filter(|e| !x.contains(e)).collect();
                            differ.push(format!(
                                "{what}: {part} of {order:?} differ from {first:?}'s:\n  only {first:?}: {only_a:#?}\n  only {order:?}: {only_b:#?}"
                            ));
                        }
                    }
                }
                (a, b) => {
                    let fail = |r: &Result<_, String>| r.as_ref().err().cloned();
                    if fail(a) != fail(b) {
                        differ.push(format!(
                            "{what}: {first:?} {:?} against {order:?} {:?}",
                            fail(a),
                            fail(b)
                        ));
                    }
                }
            }
        }
    }
    differ
}

/// **Every order of the tools names alike** (DM4: the fold is
/// order-free in the tools): the same volume and solid count and the
/// same table, two and three tools, disjoint, overlapping, sharing
/// faces, parting the plate and sharing a floor. Red where an order
/// names an entity differently — a crossing vertex spelled through the
/// seam of whichever tool cut first.
#[test]
fn every_order_of_the_tools_names_alike() {
    let differ = order_differences(&["names"]);
    assert!(differ.is_empty(), "{}", differ.join("\n\n"));
}

/// **Every order of the tools decides and records alike**: the same
/// rows (a symmetric relation's two cells unordered, a row's fold step
/// left out), the same records citing the same rows and inputs, and the
/// same inputs cited. Red where an order decides, records or cites
/// differently.
#[test]
fn every_order_of_the_tools_decides_and_records_alike() {
    let differ = order_differences(&["rows", "records", "cited"]);
    assert!(differ.is_empty(), "{}", differ.join("\n\n"));
}

/// **Each name of a fold names an entity by one read**: every face,
/// edge and vertex name of a cut by several tools heads `From { read }`
/// of the plate or a tool, or a name the cut minted, and no name heads
/// a fold side. Red if a fold side's read leaks into a published name.
#[test]
fn no_published_name_of_a_fold_is_keyed_by_a_fold_side() {
    for (what, orders) in every_order() {
        for (order, got) in orders {
            if let Ok((said, ..)) = got {
                for name in &said.names {
                    assert!(
                        !name.contains("FOLD") && !name.contains("Fold"),
                        "{what} {order:?}: {name}"
                    );
                }
            }
        }
    }
}

/// A through hole from below up to the plate's top, and a block above
/// the plate touching its top corner at one point there, unioned: a
/// tool whose carry holds a vertex-pair record at a corner of the hole.
fn kissing(doc: ProfileDoc, plate: RecipeNodeId, (x, y): (f64, f64)) -> (ProfileDoc, RecipeNodeId) {
    let (doc, a) = tool(doc, plate, (x, x + 0.1), (y, y + 0.1), (-1.0, 1.2));
    let (doc, b) = tool(
        doc,
        plate,
        (x + 0.1, x + 0.2),
        (y + 0.1, y + 0.2),
        (0.2, 1.0),
    );
    fixture::union_over(doc, &[a, b])
}

/// The cut's records' citations of carried records, each as the input
/// it cites.
fn carried_inputs(ev: &Evaluation<f64>, id: RecipeNodeId) -> Vec<String> {
    let value = ev
        .value(id)
        .unwrap_or_else(|| panic!("{id:?} evaluated: {:?}", ev.result(id)));
    let mut out: Vec<String> = value
        .carry
        .contacts
        .rows()
        .iter()
        .flat_map(|r| r.cites.iter().collect::<Vec<_>>())
        .filter_map(|b| match b {
            topo::Backing::Carried { input, .. } => {
                Some(format!("{:?}", value.carry.cited_inputs[input as usize]))
            }
            topo::Backing::Decided(_) => None,
        })
        .collect();
    out.sort();
    out
}

/// **A fold cites `from` as input 0 and tool `k` as input `k + 1`**: a
/// plate carrying a corner kiss of its own, less a hole and a kissing
/// pair (whose kiss the cut does not keep), in both orders and with a
/// second hole. The cut's cited inputs are the plate's read and then
/// each tool's, in list order; every carried record cites what the pair
/// cut alone cites (the plate's kiss); deleting the first tool by the
/// gesture keeps the cut by the rest, still citing so; and the document
/// before the delete, evaluated again over the evaluation after it (an
/// undo), says what it first said. Red if a record cites a tool by its
/// place before the fold, the accumulation, or the wrong input. A
/// tool's own carried record is not reached here: none of the kisses
/// tried survives the cut.
#[test]
fn a_fold_cites_its_plate_and_each_tool_by_its_read_in_every_order() {
    let (doc, p0) = plate();
    // The plate with a corner kiss of its own, outside the punches.
    let (doc, nub) = tool(doc, p0, (3.0, 3.2), (1.0, 1.2), (0.0, 0.2));
    let (doc, p) = fixture::union_over(doc, &[p0, nub]);
    let (doc, hole) = tool(doc, p0, (0.25, 0.5), (0.25, 0.75), (-1.0, 3.0));
    let (doc, hole2) = tool(doc, p0, (2.25, 2.5), (0.25, 0.75), (-1.0, 3.0));
    let (doc, punches) = kissing(doc, p0, (1.25, 0.3));
    let plate_read = format!("{:?}", CitedInput::Read(doc.read_of_node(p).unwrap()));
    let punch_read = format!("{:?}", CitedInput::Read(doc.read_of_node(punches).unwrap()));
    // Each input alone first, so the row reads what each carries.
    let (lone, lone_cut) = subtract_over(doc.clone(), p, &[punches]);
    let ev = run(&lone);
    let want = carried_inputs(&ev, lone_cut);
    eprintln!("the pair cut carries {want:?}; plate {plate_read}, punches {punch_read}");
    assert!(
        want.contains(&plate_read),
        "the pair cut carries the plate's kiss and the punches': {:?}",
        ev.value(lone_cut)
            .map(|v| (&v.carry.contacts, &v.carry.cited_inputs))
    );
    for order in [
        vec![hole, punches],
        vec![punches, hole],
        vec![hole, punches, hole2],
        vec![hole2, hole, punches],
    ] {
        let (doc, cut) = subtract_over(doc.clone(), p, &order);
        let ev = run(&doc);
        assert_eq!(
            carried_inputs(&ev, cut),
            want,
            "{order:?}: {:?}",
            ev.value(cut)
                .map(|v| (&v.carry.contacts, &v.carry.cited_inputs))
        );
        census_of(&ev, cut).unwrap_or_else(|e| panic!("{order:?}: {e:?}"));
        let listed: Vec<String> = core::iter::once(p)
            .chain(order.iter().copied())
            .map(|n| format!("{:?}", CitedInput::Read(doc.read_of_node(n).unwrap())))
            .collect();
        let held: Vec<String> = ev
            .value(cut)
            .unwrap()
            .carry
            .cited_inputs
            .iter()
            .map(|c| format!("{c:?}"))
            .collect();
        assert_eq!(
            held, listed,
            "{order:?}: from is input 0 and tool k input k + 1"
        );
        // Deleting the first tool keeps the cut by the rest.
        let after = deleted(&doc, order[0]);
        let later = fixture::run_with_prior(&after, &ev);
        let survivor = after
            .ids()
            .into_iter()
            .find(|&n| matches!(after.node(n), Some(Node::Subtract { .. })))
            .expect("the cut continues");
        let Some(Node::Subtract {
            tool: Bodies::Spelled(left),
            ..
        }) = after.node(survivor)
        else {
            panic!("a spelled tool list: {:?}", after.node(survivor));
        };
        assert_eq!(left.len(), order.len() - 1, "{order:?}: the rest are kept");
        let want_after: Vec<String> = if order[0] == punches {
            want.iter().filter(|c| **c != punch_read).cloned().collect()
        } else {
            want.clone()
        };
        assert_eq!(
            carried_inputs(&later, survivor),
            want_after,
            "{order:?} less its first: {:?}",
            later
                .value(survivor)
                .map(|v| (&v.carry.contacts, &v.carry.cited_inputs))
        );
        // The undo: the document before, over the evaluation after.
        let undone = fixture::run_with_prior(&doc, &later);
        assert_eq!(
            said(&undone, cut),
            said(&ev, cut),
            "{order:?}: the undo says what the first evaluation said"
        );
    }
}

/// `doc` with `id` deleted by the delete gesture: its readers handed
/// what it continues ([`editor_core::continuation::splice`]).
fn deleted(doc: &ProfileDoc, id: RecipeNodeId) -> ProfileDoc {
    let splice = editor_core::continuation::splice(doc, id)
        .expect("the delete splices")
        .expect("the node is live");
    let mut run = editor_core::Recording::start(doc, Tol::witness());
    splice.apply(&mut run).expect("the splice applies");
    run.finish().expect("the action finishes").doc
}

/// The tool list of the one subtract `doc` holds.
fn the_tools(doc: &ProfileDoc) -> Option<(RecipeNodeId, Vec<editor_core::VarId>)> {
    doc.ids().into_iter().find_map(|n| match doc.node(n) {
        Some(Node::Subtract {
            tool: Bodies::Spelled(reads),
            ..
        }) => Some((n, reads.iter().map(|r| r.read().read).collect())),
        _ => None,
    })
}

/// **Deleting a tool keeps the cut by the rest** (the delete
/// continuation): a plate less three holes, the middle one deleted, is
/// the plate less the other two, in list order, by two holes' volume;
/// each of the three deleted in turn from the full list leaves the
/// other two; and the plate less one hole, that hole deleted, is the
/// plate (its reader reads the plate). Red if a delete drops the cut,
/// drops a survivor, or re-orders them.
#[test]
fn deleting_a_tool_keeps_the_cut_by_the_rest_in_order() {
    let (doc, p) = plate();
    let through = (-1.0, 3.0);
    let (doc, a) = tool(doc, p, (0.25, 0.5), (0.25, 0.75), through);
    let (doc, b) = tool(doc, p, (1.25, 1.5), (0.25, 0.75), through);
    let (doc, c) = tool(doc, p, (2.25, 2.5), (0.25, 0.75), through);
    let (doc, cut) = subtract_over(doc, p, &[c, a, b]);
    let (doc, reader) = insert(
        doc,
        Node::Union {
            members: Bodies::one(plain(cut)),
        },
    );
    let hole_v = 0.25 * 0.5 * 0.2;
    for (gone, left) in [(a, vec![c, b]), (c, vec![a, b]), (b, vec![c, a])] {
        let after = deleted(&doc, gone);
        let (_, tools) = the_tools(&after).unwrap_or_else(|| panic!("{gone:?}: the cut stays"));
        let left: Vec<_> = left
            .iter()
            .map(|&n| after.read_of_node(n).unwrap())
            .collect();
        assert_eq!(tools, left, "{gone:?} deleted: the rest, in list order");
        let ev = run(&after);
        let v = volume(the_body(&ev, reader));
        assert!(
            (v - (0.6 - 2.0 * hole_v)).abs() < 1e-9,
            "{gone:?} deleted: {v}"
        );
    }
    // Down to one tool, then none.
    let one = deleted(&deleted(&doc, a), b);
    let (_, tools) = the_tools(&one).expect("the cut by one stays");
    assert_eq!(tools, vec![one.read_of_node(c).unwrap()]);
    let none = deleted(&one, c);
    assert!(the_tools(&none).is_none(), "no subtract is left");
    let ev = run(&none);
    let v = volume(the_body(&ev, reader));
    assert!((v - 0.6).abs() < 1e-9, "the reader reads the plate: {v}");
}

/// **A family with an empty member refuses at the tool seat, typed**:
/// `F[k] − F[1]` over a row of three blocks has member 1 empty, and a
/// subtract whose tool reads that family whole — spelled as one read,
/// read as the family, or beside another tool — refuses `EmptyMember`
/// naming the family and member 1. Red if the cut skips the empty
/// member, panics, or refuses untyped.
#[test]
fn a_family_with_an_empty_member_at_the_tool_seat_refuses_typed() {
    let base = std::cell::Cell::new(None);
    let (doc, _, cut) = crate::g_family_of_booleans::cut_row(|doc, b, row| {
        base.set(Some(b));
        (doc, BodyRead::indexed(row, vec![Formula::count(1)]))
    });
    let b = base.get().unwrap();
    let (doc, plate) = block_on(doc, b, (-1.0, 4.0), (-1.0, 2.0), -1.0, 3.0);
    let (doc, other) = block_on(doc, b, (5.0, 6.0), (0.0, 1.0), 0.0, 1.0);
    let (doc, spelled) = insert(
        doc,
        Node::Subtract {
            from: plain(plate),
            tool: Bodies::Spelled(vec![editor_core::Spell::All(plain(cut))]),
        },
    );
    let (doc, whole) = insert(
        doc,
        Node::Subtract {
            from: plain(plate),
            tool: Bodies::Family(plain(cut)),
        },
    );
    let (doc, beside) = insert(
        doc,
        Node::Subtract {
            from: plain(plate),
            tool: Bodies::Spelled(vec![
                editor_core::Spell::One(plain(other)),
                editor_core::Spell::All(plain(cut)),
            ]),
        },
    );
    let ev = run(&doc);
    let k = doc.var_named("k").unwrap();
    for (what, id) in [
        ("spelled as one read", spelled),
        ("read whole", whole),
        ("beside another tool", beside),
    ] {
        assert!(
            matches!(
                ev.result(id),
                Some(NodeResult::Failed(e)) if matches!(
                    &e.kind,
                    NodeErrorKind::EmptyMember { input, at } if *input == cut && at == &[(k, 1)]
                )
            ),
            "the family {what} at the tool: {:?}",
            ev.result(id)
        );
    }
}

/// **A family of one member at the tool seat is a pair cut**, and a
/// family read whole beside a spelled tool folds every member with it:
/// a plate less a row of one hole names as the plate less that hole
/// read by index does, and less `[hole, row of two]` is the plate less
/// three holes. Red if a one-member family takes the fold path and
/// names unlike the pair, or a family beside a spelled tool is cut once.
#[test]
fn a_family_of_one_at_the_tool_cuts_as_one_tool() {
    let (doc, p) = plate();
    let (doc, hole) = tool(doc, p, (0.25, 0.5), (0.25, 0.75), (-1.0, 3.0));
    let (doc, _, one) = fixture::row(doc, hole, ("k", 1), TargetAxis::X, 1.0);
    let (doc, other) = tool(doc, p, (2.5, 2.75), (0.25, 0.75), (-1.0, 3.0));
    let (doc, _, two) = fixture::row(doc, hole, ("j", 2), TargetAxis::X, 1.0);
    let (doc, whole_one) = insert(
        doc,
        Node::Subtract {
            from: plain(p),
            tool: Bodies::Spelled(vec![editor_core::Spell::All(plain(one))]),
        },
    );
    let (doc, mixed) = insert(
        doc,
        Node::Subtract {
            from: plain(p),
            tool: Bodies::Spelled(vec![
                editor_core::Spell::One(plain(other)),
                editor_core::Spell::All(plain(two)),
            ]),
        },
    );
    let ev = run(&doc);
    assert_eq!(the_body(&ev, whole_one).solids().count(), 1);
    let v = volume(the_body(&ev, whole_one));
    let hole_v = 0.25 * 0.5 * 0.2;
    assert!((v - (0.6 - hole_v)).abs() < 1e-9, "a row of one: {v}");
    let v = volume(the_body(&ev, mixed));
    assert!(
        (v - (0.6 - 3.0 * hole_v)).abs() < 1e-9,
        "a hole and a row of two: {v}"
    );
}

/// **A tool listed twice cuts once**: the plate less `[h, h]`, and less
/// a row of two read whole beside its member 1 read by index, each
/// build the plate less the holes once, in either order. Red if a
/// repeated tool refuses (a duplicate name in the collapsed table) or
/// counts twice.
#[test]
fn a_tool_listed_twice_cuts_once() {
    let (doc, p) = plate();
    let (doc, h) = tool(doc, p, (0.25, 0.5), (0.25, 0.75), (-1.0, 3.0));
    let (doc, _, row) = fixture::row(doc, h, ("k", 2), TargetAxis::X, 1.0);
    let member = BodyRead::indexed(row, vec![Formula::count(1)]);
    let hole_v = 0.25 * 0.5 * 0.2;
    let mut doc = doc;
    let mut cuts = Vec::new();
    for (what, tools, holes) in [
        (
            "h twice",
            vec![
                editor_core::Spell::One(plain(h)),
                editor_core::Spell::One(plain(h)),
            ],
            1.0,
        ),
        (
            "row then row[1]",
            vec![
                editor_core::Spell::All(plain(row)),
                editor_core::Spell::One(member.clone()),
            ],
            2.0,
        ),
        (
            "row[1] then row",
            vec![
                editor_core::Spell::One(member.clone()),
                editor_core::Spell::All(plain(row)),
            ],
            2.0,
        ),
    ] {
        let (next, cut) = insert(
            doc,
            Node::Subtract {
                from: plain(p),
                tool: Bodies::Spelled(tools),
            },
        );
        doc = next;
        cuts.push((what, cut, holes));
    }
    let ev = run(&doc);
    let mut wrong = Vec::new();
    for (what, cut, holes) in cuts {
        match ev.result(cut) {
            Some(NodeResult::Failed(e)) => wrong.push(format!("{what}: refused {:?}", e.kind)),
            _ => {
                let v = volume(the_body(&ev, cut));
                if (v - (0.6 - holes * hole_v)).abs() > 1e-9 {
                    wrong.push(format!("{what}: volume {v}"));
                }
            }
        }
    }
    assert!(wrong.is_empty(), "{wrong:#?}");
}

/// **A fold's rows keep their relations**: each scenario's fold
/// `plate − [a, b]` against what pair cuts decide too. The fold's
/// judgement of the plate against tool `t` is the pair cut `plate − t`,
/// so its rows agree with that cut's, row for row: every carrier
/// relation (any but a point: `Meet`, or `OnCarrier` at a vertex
/// fusion) and as many points. A judgement of two tools, which no pair
/// cut of the plate makes, and the fold-step rows, which DM4 lets follow
/// the member order
/// (`work/formwords/fold-step-point-rows-depend-on-the-member-order.md`),
/// are left out. Red where the fold publishes a relation the pair cut
/// decided otherwise.
#[test]
fn a_fold_steps_rows_keep_their_relations() {
    let mut differ = Vec::new();
    for (what, doc, p, tools) in scenarios() {
        let (doc, fold) = subtract_over(doc, p, &tools[..2]);
        let (doc, by_a) = subtract_over(doc, p, &tools[..1]);
        let (doc, by_b) = subtract_over(doc, p, &tools[1..2]);
        let ev = run(&doc);
        let relations = |ids: &[RecipeNodeId], judged: bool| {
            let mut out: Vec<String> = ids
                .iter()
                .flat_map(|&id| match ev.value(id) {
                    Some(v) => v
                        .coincidences
                        .iter()
                        .filter(|r| {
                            !judged
                                || matches!(
                                    r.origin,
                                    editor_core::RowOrigin::Judged { pair } if pair[0] == 0
                                )
                        })
                        .map(|r| match (r.relation, r.site) {
                            (
                                topo::Relation::Meet | topo::Relation::OnCarrier,
                                topo::DecisionSite::VertexFusion,
                            ) => "a point".to_owned(),
                            (relation, site) => format!("{relation:?} @ {site:?}"),
                        })
                        .collect::<Vec<_>>(),
                    None => vec![format!("{id:?} refused: {:?}", ev.result(id))],
                })
                .collect();
            out.sort();
            out
        };
        let (f, c) = (relations(&[fold], true), relations(&[by_a, by_b], false));
        if f != c {
            differ.push(format!("{what}:\n  fold  {f:#?}\n  pairs {c:#?}"));
        }
    }
    assert!(differ.is_empty(), "{}", differ.join("\n"));
}
