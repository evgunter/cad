#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
use editor_core::test_support::{ang, len};
use editor_core::{
    BodyRead, CancelToken, CapEnd, DocEdit, EntityKind, EvalOptions, Evaluation, Formula,
    LoopProgram, Node, OnFace, Operand, ProfileDoc, ProfileProgram, RecipeNodeId, RoleSeg,
    StableName, evaluate,
};
use geom_core::Tol;

fn insert(doc: ProfileDoc, node: Node<ProfileProgram<Formula>, Formula>) -> (ProfileDoc, RecipeNodeId) {
    let applied = doc
        .apply(&DocEdit::InsertNode { node: Box::new(node), fresh: Vec::new() }, Tol::witness())
        .unwrap();
    (applied.doc, applied.record.minted.unwrap())
}
fn eval(doc: &ProfileDoc) -> Evaluation<f64> {
    evaluate::<f64>(doc, None, &CancelToken::new(), &EvalOptions::default(), Tol::witness())
}
fn circle(r: f64) -> LoopProgram<Formula> {
    LoopProgram::Circle { centre: [len(0.0), len(0.0)], radius: len(r) }
}
fn report(ev: &Evaluation<f64>, what: &str, n: RecipeNodeId) {
    match ev.node_error(n) {
        None => println!("PROBE {what}: builds (value present: {})", ev.value(n).is_some()),
        Some(e) => println!("PROBE {what}: REFUSES: {}", e.kind),
    }
}
fn run(angle: f64, axis_x: f64, end: CapEnd, heading: f64) {
    println!("PROBE ---- angle {angle} axis_x {axis_x} end {end:?} heading {heading}");
    let doc = ProfileDoc::empty_derived("probe_elbow", Tol::witness());
    let (doc, ring) = insert(
        doc,
        Node::Profile(ProfileProgram { loops: vec![circle(1.0), circle(0.6)], ids: Vec::new(), on: None }),
    );
    let (doc, pipe) = insert(doc, Node::Extrude { profile: ring.into(), distance: len(2.0) });
    let cap = StableName { kind: EntityKind::Face, node: pipe, path: vec![RoleSeg::Cap(end)] };
    let drawn = editor_core::draw_on_face(
        &doc,
        BodyRead::plain(Operand::output(pipe, 0)),
        cap,
        vec![circle(1.0), circle(0.6)],
        OnFace::nearest(),
        &EvalOptions::default(),
        &CancelToken::new(),
        Tol::witness(),
    );
    let drawn = match drawn {
        Ok(d) => d,
        Err(e) => {
            println!("PROBE draw_on_face REFUSES: {e}");
            return;
        }
    };
    let (doc, elbow) = insert(
        drawn.doc,
        Node::Revolve {
            profile: drawn.profile.into(),
            axis_through: [len(axis_x), len(0.0)],
            axis_heading: ang(heading),
            angle: ang(angle),
        },
    );
    let (doc, whole) = insert(
        doc,
        Node::Union { members: editor_core::Bodies::Spelled(vec![pipe.into(), BodyRead::plain(Operand::output(elbow, 0))]) },
    );
    let ev = eval(&doc);
    report(&ev, "pipe", pipe);
    report(&ev, "profile on cap", drawn.profile);
    report(&ev, "elbow (revolve)", elbow);
    report(&ev, "union pipe+elbow", whole);
    if let Some(v) = ev.value(whole) {
        println!("PROBE union coincidences: {}", v.coincidences.len());
        let rows = &v.coincidences;
        let proofs = editor_core::coincide::prove(
            &doc, &[(whole, &rows[..])], None, Tol::witness(), &editor_core::Stop::default(),
        );
        match proofs {
            Ok(p) => {
                for (row, proof) in rows.iter().zip(&p[0]) {
                    println!("PROBE   {:?} @ {:?} -> {:?}", row.relation, row.site, proof);
                }
            }
            Err(e) => println!("PROBE prove failed: {e:?}"),
        }
    }
}

#[test]
fn probe() {
    let q = std::f64::consts::FRAC_PI_2;
    for end in [CapEnd::End, CapEnd::Start] {
        for (axis_x, heading) in [(3.0, q), (3.0, -q), (-3.0, q), (-3.0, -q)] {
            for angle in [q, -q] {
                run(angle, axis_x, end, heading);
            }
        }
    }
    panic!("PROBE DONE: this test fails on purpose so the runner prints the PROBE lines above");
}
