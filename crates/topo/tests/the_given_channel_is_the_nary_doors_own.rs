//! **A given pair is the n-ary door's own.** A fold step of
//! [`topo::nary_boolean`] is given the judgements' verdicts on the
//! member it joins, read through the lineage the door advances; no
//! public input states a glue, so a glue no record over the bodies it
//! joins states cannot be written
//! (`work/flush/a-glue-verdict-is-not-bound-to-the-bodies-it-judged.md`).
#![allow(clippy::panic, clippy::expect_used, clippy::unwrap_used)]

use crate::common;
use common::{brick, finished};
use geom_core::Tol;
use test_utils::source;
use topo::{Body, FaceKey, MemberInputs, NaryMember, NaryOp, NaryResult, Relation, Surface};

fn tol() -> Tol {
    Tol::witness()
}

fn face_at_z(body: &Body<f64>, z: f64) -> FaceKey {
    body.faces()
        .find(|(_, f)| {
            matches!(
                body.get_surface(f.surface),
                Some(Surface::Plane { origin, normal, .. })
                    if normal.z.abs() == 1.0 && origin.z == z
            )
        })
        .map(|(k, _)| k)
        .expect("a face on z")
}

/// A brick resting on a cube's top, beside a block far from both: the
/// one judgement's one verdict is the touch it decided, and the step
/// that joins the brick glues that pair as given and decides no face
/// pair of its own.
#[test]
fn a_rest_touch_is_the_unions_one_verdict_and_glues_as_given() {
    let a = finished("a", brick((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), tol()), tol());
    let b = finished(
        "b",
        brick((0.25, 0.75), (0.25, 0.75), (1.0, 2.0), tol()),
        tol(),
    );
    let far = finished(
        "far",
        brick((5.0, 6.0), (0.0, 1.0), (0.0, 1.0), tol()),
        tol(),
    );
    let inputs: Vec<MemberInputs> = (0..3)
        .map(|rank| MemberInputs {
            rank,
            ..MemberInputs::default()
        })
        .collect();
    let members: Vec<NaryMember<'_, f64>> = [&a, &b, &far]
        .into_iter()
        .zip(&inputs)
        .map(|(body, inputs)| NaryMember { body, inputs })
        .collect();
    let NaryResult::Body(built) = topo::nary_boolean(
        NaryOp::Union,
        &members,
        topo::SweepStrategy::Realized,
        tol(),
    )
    .unwrap() else {
        panic!("a union of three bodies is not empty");
    };
    let [judged] = &built.judgements[..] else {
        panic!(
            "one judgement, the cube and the brick, whose boxes alone meet: {:?}",
            built
                .judgements
                .iter()
                .map(|j| (j.a, j.b))
                .collect::<Vec<_>>()
        );
    };
    let [verdict] = &judged.body.verdicts[..] else {
        panic!("one verdict, the touch: {:?}", judged.body.verdicts);
    };
    assert_eq!(
        (
            judged.a,
            judged.b,
            verdict.a(),
            verdict.b(),
            verdict.relation()
        ),
        (
            0,
            1,
            face_at_z(&a, 1.0),
            face_at_z(&b, 1.0),
            Relation::SameOpposite
        ),
        "the verdict is the cube's top against the brick's bottom, opposed"
    );
    let step = &built.steps[0];
    // The step still decides the brick's four bottom corners on the
    // cube's top, by value.
    let top = topo::RowCell::Input {
        input: topo::Operand::A,
        cell: topo::Cell::Face(face_at_z(&a, 1.0)),
    };
    assert!(
        step.member == 1
            && step.body.verdicts.is_empty()
            && step.restated.is_empty()
            && step.body.coincidences.len() == 4
            && step.body.coincidences.iter().all(|c| c.site == topo::DecisionSite::VertexFusion
                && matches!(
                    c.cells,
                    [topo::RowCell::Input { input: topo::Operand::B, cell: topo::Cell::Vertex(_) }, t]
                        if t == top
                )),
        "a given step decides no pair of its own, only the corners on the top: {:?}",
        step.body.coincidences
    );
    let volume =
        |out: &topo::BooleanBody<f64>| topo::mass_properties(&out.body, tol()).unwrap().volume;
    assert_eq!(
        (volume(&step.body), step.body.body.faces().count()),
        (volume(&judged.body), judged.body.body.faces().count()),
        "the given glue builds the judged body"
    );
}

/// Whether the item head whose `fn` or `struct` is at `at` is `pub`
/// with no restriction: `pub` (and `const`) before it on its line.
fn plainly_public(code: &str, at: usize) -> bool {
    let mut before = code[source::line_start(code, at)..at].split_whitespace();
    before.clone().any(|w| w == "pub") && before.all(|w| w == "pub" || w == "const")
}

/// Every whole-word offset of `word` in `code`.
fn words<'a>(code: &'a str, word: &'a str) -> impl Iterator<Item = usize> + 'a {
    code.match_indices(word)
        .map(|(at, _)| at)
        .filter(move |&at| source::word_at(code, at, word))
}

/// **No public door states a given pair**: `GluedPair` is not a public
/// type, `GlueVerdict::given_at` is not a public function, and
/// `BooleanInputs` carries no `given` field, so the only code that
/// hands a boolean a glue it did not measure is the n-ary door, over
/// member bodies it holds.
#[test]
fn no_public_door_states_a_given_pair() {
    let src = source::crate_dir(env!("CARGO_MANIFEST_DIR")).join("src");
    let mut seen = [0usize; 3];
    for file in source::rust_sources(&src) {
        let code = source::code_only(&std::fs::read_to_string(&file).unwrap());
        for at in words(&code, "struct") {
            let name_at = source::skip_ws(&code, at + "struct".len());
            if source::word_at(&code, name_at, "GluedPair") {
                seen[0] += 1;
                assert!(
                    !plainly_public(&code, at),
                    "{}: GluedPair is a public type",
                    file.display()
                );
            }
            if source::word_at(&code, name_at, "BooleanInputs") {
                seen[1] += 1;
                let source::ItemBody::Body(body) = source::item_body(&code, at) else {
                    panic!("BooleanInputs has fields");
                };
                assert!(
                    words(&code[body], "given").next().is_none(),
                    "{}: BooleanInputs carries a given field",
                    file.display()
                );
            }
        }
        for at in words(&code, "fn") {
            if source::word_at(&code, source::skip_ws(&code, at + "fn".len()), "given_at") {
                seen[2] += 1;
                assert!(
                    !plainly_public(&code, at),
                    "{}: given_at is a public function",
                    file.display()
                );
            }
        }
    }
    assert_eq!(
        seen,
        [1, 1, 1],
        "the census sees GluedPair, BooleanInputs and given_at once each"
    );
}
