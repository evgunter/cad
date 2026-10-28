//! MEASUREMENT (§8 Q1): a ring of N planar wedges about z, each wedge
//! the trapezoid `(2, 0), (3, 0), 3·(cos θ, sin θ), 2·(cos θ, sin θ)`
//! extruded 1, `θ = turn/N`, its members touching their neighbours
//! side to side; which of the census's rows the coincidence door
//! proves.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture::{self, insert, len};
use editor_core::{
    Bodies, Coord, Formula, LoopProgram, Node, Operand, PatternKind, ProfileDoc, ProfileProgram,
    ProgramStep, ProgramTarget, RecipeNodeId, TargetAxis,
};
use geom_core::Tol;

/// The wedge on the declared radii `ri < ro`: its near side from
/// `(ri, 0)` to `(ro, 0)`, its far side the same two radii at `θ =
/// turn/N`, written through `cos θ` and `sin θ` (or, `literal`, at a
/// quarter turn, `(0, r)`), so the far side is the near side turned by
/// `θ` for every value of the radii.
fn wedge(doc: ProfileDoc, n: u64, literal: bool) -> (ProfileDoc, RecipeNodeId) {
    use editor_core::{Dimension, DocEdit, FreeVar, VarDecl, VarName};
    let mut doc = doc;
    let mut params = std::collections::BTreeMap::new();
    for (name, value) in [("ri", 2.0), ("ro", 3.0)] {
        let name = VarName::new(name).unwrap();
        doc = fixture::step(
            doc,
            DocEdit::DeclareVar {
                name: name.clone(),
                def: VarDecl::Free(FreeVar::continuous(Dimension::Length, value)),
            },
        )
        .0;
        params.insert(name, Dimension::Length);
    }
    let f = |text: &str| editor_core::parse_formula(text, &params).unwrap();
    let zero = || Formula::zero(Dimension::Length);
    let at = |r: &str| {
        if literal {
            return [zero(), f(r)];
        }
        [f(&format!("{r} * cos(turn / {n})")), f(&format!("{r} * sin(turn / {n})"))]
    };
    let (doc, profile) = insert(
        doc,
        Node::Profile(ProfileProgram {
            loops: vec![LoopProgram::Chain(vec![
                ProgramStep::At([f("ri"), zero()]),
                ProgramStep::LineTo(ProgramTarget::Point([f("ro"), zero()])),
                ProgramStep::LineTo(ProgramTarget::Point(at("ro"))),
                ProgramStep::LineTo(ProgramTarget::Point(at("ri"))),
                ProgramStep::LineTo(ProgramTarget::Start),
            ])],
            ids: Vec::new(),
        }),
    );
    insert(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: len(1.0),
        },
    )
}

/// N world placements of the wedge, member `k`'s mateless bundle
/// spinning it by the written value `k · turn/N` about z. Each value is
/// a free variable of its own.
fn ring_placed(n: u64, literal: bool) -> (ProfileDoc, Vec<RecipeNodeId>) {
    let doc = ProfileDoc::empty_derived("g-ring-placed", Tol::witness());
    let (mut doc, wedge) = wedge(doc, n, literal);
    let body = doc.output(wedge, 0).expect("the body");
    let mut members = Vec::new();
    for k in 0..n {
        let (d, p) = if k == 0 {
            fixture::place(doc, wedge)
        } else {
            let spin = Formula::mul(
                Formula::ratio(i64::try_from(k).unwrap(), n).unwrap(),
                Formula::turn(),
            )
            .unwrap();
            let placed =
                editor_core::Placed::of(&doc, Vec::new(), vec![(Coord::Spin(Some(TargetAxis::Z)), spin)]);
            let node = fixture::place_node(
                &doc,
                editor_core::BodyRead::plain(Operand::Var(body)),
                placed.bundle(),
            );
            let (d, p) = insert(doc, node);
            (fixture::assemble_world_copies(d), p)
        };
        doc = d;
        members.push(p);
    }
    (doc, members)
}

/// Today's pattern of the wedge: one `Circular` rule of step `turn/N`
/// about z (a slot constant, no variable), its members kept apart by an
/// `Assemble`, that `⊔` placed as the product.
fn ring_pattern(n: u64, literal: bool) -> (ProfileDoc, RecipeNodeId) {
    let doc = ProfileDoc::empty_derived("g-ring-pattern", Tol::witness());
    let (doc, wedge) = wedge(doc, n, literal);
    let body = Operand::Var(doc.output(wedge, 0).expect("the body"));
    let axis = z_axis(&doc, wedge, body.clone(), n);
    let (doc, pattern) = insert(
        doc,
        Node::Pattern {
            input: body,
            count: Formula::count(i64::try_from(n).unwrap()),
            kind: PatternKind::Circular {
                axis,
                step: Formula::turn_over(n).unwrap(),
            },
        },
    );
    let (doc, assemble) = insert(
        doc,
        Node::Assemble {
            members: Bodies::Family(pattern.into()),
        },
    );
    let (doc, _) = fixture::place(doc, assemble);
    (doc, assemble)
}

/// The z axis the wedge's two sides meet in: its far side's plane, then
/// its near side's (`y = 0`, facing −y), so the meet runs along +z.
fn z_axis(
    doc: &ProfileDoc,
    wedge: RecipeNodeId,
    body: Operand,
    n: u64,
) -> editor_core::pose::RoleRead<Operand> {
    use editor_core::pose::{PoseDef, RoleRead};
    use editor_core::{EntityKey, Entry};
    let ev = fixture::run(doc, &Default::default());
    let theta = std::f64::consts::TAU / n as f64;
    let out = geom_core::Vec3::new(-theta.sin(), theta.cos(), 0.0);
    let (var, index) = ev.body_read_of(wedge, 0).expect("the wedge evaluated");
    let solid = match &ev.value(wedge).unwrap().payload {
        editor_core::ValuePayload::Body(b) => fixture::in_space(b),
        other => panic!("a body, got {other:?}"),
    };
    let far = ev
        .table(var)
        .unwrap()
        .iter()
        .find_map(|(name, entry)| match entry {
            Entry::Unique(r) if r.body == index => match r.key {
                EntityKey::Face(f) => match topo::face_carrier(&solid, f)? {
                    topo::CarrierDesc::Plane { normal, .. } => {
                        ((normal - out).norm() < 1e-9).then(|| name.clone())
                    }
                    _ => None,
                },
                _ => None,
            },
            _ => None,
        })
        .expect("the far side");
    let far = RoleRead::plain(Operand::Pose(Box::new(PoseDef::Plane {
        face: Operand::select(body.clone(), vec![far]),
    })));
    let near = fixture::plane_on(doc, body, [0.0; 3], [0.0, -1.0, 0.0]);
    RoleRead::plain(Operand::Pose(Box::new(PoseDef::Meet { a: far, b: near })))
}

fn verdict(proof: &editor_core::Proof) -> String {
    match proof {
        editor_core::Proof::Structural(rung) => format!("{rung:?}"),
        editor_core::Proof::Unproven {
            residual, recourse, ..
        } => format!("UNPROVEN {:?} {recourse:?}", residual.replay),
    }
}

#[test]
#[ignore = "measurement: cargo nextest run -p editor-core --run-ignored all g_ring_closure_measure --no-capture"]
fn measure_ring_closure_placed() {
    for (n, literal) in [(4, true), (4, false), (6, false), (8, false), (12, false)] {
        let (doc, members) = ring_placed(n, literal);
        println!("free vars: {:?}", doc.free_vars().map(|(id, p)| format!("{} {p:?}", doc.spoken_var(id))).collect::<Vec<_>>());
        let ev = fixture::run(&doc, &Default::default());
        let product = editor_core::product_recorded(&doc, &ev, Tol::witness()).expect("gathers");
        let at = |var| members.iter().position(|&m| doc.output(m, 0) == Some(var));
        let mut tally = std::collections::BTreeMap::<String, usize>::new();
        for r in &product.coincidences {
            let (a, b) = &r.copies;
            let node = doc.operation_of(a.var).expect("a copy's placement");
            let rows = [r.row.clone()];
            let proof = editor_core::coincide::prove(
                &doc,
                &[(node, &rows[..])],
                None,
                Tol::witness(),
                &editor_core::CancelToken::new(),
            )
            .expect("door");
            let v = verdict(&proof[0][0]);
            println!("  {:?}/{:?} {:?}: {v}", at(a.var), at(b.var), r.row.relation);
            *tally.entry(format!("{:?} {v}", r.row.relation)).or_default() += 1;
        }
        println!("PLACED N = {n} literal {literal}: census {} {tally:?}", product.census.len());
    }
}

#[test]
#[ignore = "measurement: cargo nextest run -p editor-core --run-ignored all g_ring_closure_measure --no-capture"]
fn measure_ring_closure_pattern() {
    for (n, literal) in [(4, true), (4, false), (6, false), (8, false), (12, false)] {
        let (doc, assemble) = ring_pattern(n, literal);
        let ev = fixture::run(&doc, &Default::default());
        let v = ev.value(assemble).expect("the assemble builds");
        let census = v.census.as_ref().expect("an assemble keeps its census");
        let product = census.verdict(&doc, Tol::witness()).expect("the census");
        let mut tally = std::collections::BTreeMap::<String, usize>::new();
        for r in &product.coincidences {
            let rows = [r.row.clone()];
            let proof = editor_core::coincide::prove(
                &doc,
                &[(assemble, &rows[..])],
                None,
                Tol::witness(),
                &editor_core::CancelToken::new(),
            )
            .expect("door");
            let v = verdict(&proof[0][0]);
            let cells: Vec<String> = r.row.cells.iter().map(|c| format!("{c:?}")).collect();
            println!("  {:?}: {v}\n     {}", r.row.relation, cells.join("\n     "));
            *tally.entry(format!("{:?} {v}", r.row.relation)).or_default() += 1;
        }
        println!(
            "PATTERN N = {n} literal {literal}: census errors {} {:?} rows {tally:?}",
            product.census.len(),
            product.census.first().map(|e| format!("{e:?}").chars().take(300).collect::<String>())
        );
    }
}
