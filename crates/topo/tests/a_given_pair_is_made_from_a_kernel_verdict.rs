//! **A given pair is made from a verdict some boolean decided.** Its
//! one public mint is `GlueVerdict::given_at`, and a verdict is minted
//! only by the kernel ([`topo::BooleanBody::verdicts`]). Nothing ties a
//! verdict to the operands it is handed to
//! (`work/flush/a-glue-verdict-is-not-bound-to-the-bodies-it-judged.md`).
#![allow(clippy::panic, clippy::expect_used, clippy::unwrap_used)]

use crate::common;
use common::{brick, finished};
use geom_core::Tol;
use test_utils::source;
use topo::{Body, BooleanInputs, BooleanOp, BooleanResult, FaceKey, Relation, Surface};

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

/// A brick resting on a cube's top: the union's one verdict is the
/// touch it decided, and handed back to the same two operands it glues
/// that pair and decides nothing else.
#[test]
fn a_rest_touch_is_the_unions_one_verdict_and_glues_as_given() {
    let a = finished("a", brick((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), tol()), tol());
    let b = finished(
        "b",
        brick((0.25, 0.75), (0.25, 0.75), (1.0, 2.0), tol()),
        tol(),
    );
    let run = |inputs: &BooleanInputs| match topo::boolean_op_with(
        BooleanOp::Union,
        &a,
        &b,
        inputs,
        topo::SweepStrategy::Realized,
        tol(),
    )
    .unwrap()
    {
        BooleanResult::Body(out) => out,
        BooleanResult::Empty => panic!("a union of two bodies is not empty"),
    };
    let decided = run(&BooleanInputs::default());
    let [verdict] = &decided.verdicts[..] else {
        panic!("one verdict, the touch: {:?}", decided.verdicts);
    };
    assert_eq!(
        (verdict.a(), verdict.b(), verdict.relation()),
        (
            face_at_z(&a, 1.0),
            face_at_z(&b, 1.0),
            Relation::SameOpposite
        ),
        "the verdict is the cube's top against the brick's bottom, opposed"
    );
    let given = run(&BooleanInputs {
        given: Some(vec![verdict.given_at(topo::Operand::A, verdict.a())]),
        ..BooleanInputs::default()
    });
    assert!(
        given.coincidences.is_empty() && given.verdicts.is_empty(),
        "a given step decides no pair of its own: {:?}",
        given.coincidences
    );
    let volume =
        |out: &topo::BooleanBody<f64>| topo::mass_properties(&out.body, tol()).unwrap().volume;
    assert_eq!(
        (volume(&given), given.body.faces().count()),
        (volume(&decided), decided.body.faces().count()),
        "the given glue builds the decided body"
    );
}

/// The two types a given pair is made of.
const GATED: [&str; 2] = ["GluedPair", "GlueVerdict"];

/// Whether `code` names a gated type as a whole word.
fn names_a_gated_type(code: &str) -> bool {
    GATED.iter().any(|ty| {
        code.match_indices(ty)
            .any(|(at, _)| source::word_at(code, at, ty))
    })
}

/// Every whole-word offset of `word` in `code`.
fn words<'a>(code: &'a str, word: &'a str) -> impl Iterator<Item = usize> + 'a {
    code.match_indices(word)
        .map(|(at, _)| at)
        .filter(move |&at| source::word_at(code, at, word))
}

/// `code`'s item head starting at `at`, up to its body or `;`.
fn head(code: &str, at: usize) -> &str {
    match source::item_body(code, at) {
        source::ItemBody::Body(body) => &code[at..body.start],
        source::ItemBody::Declaration(end) => &code[at..end],
        source::ItemBody::Unterminated => panic!("an unterminated item at {at}"),
    }
}

/// Whether the item head whose `fn` or `struct` is at `at` is `pub`
/// with no restriction: `pub` (and `const`) before it on its line.
fn plainly_public(code: &str, at: usize) -> bool {
    let mut before = code[source::line_start(code, at)..at].split_whitespace();
    before.clone().any(|w| w == "pub") && before.all(|w| w == "pub" || w == "const")
}

/// **`given_at` is the one public mint of a given pair**: no other
/// public function in `topo`'s source, nor a trait function on either
/// type, returns a `GluedPair` or a `GlueVerdict` (or `Self` inside
/// their impls), neither type has a public field, and neither derives
/// `Default` or `Deserialize`. A second mint, such as a public
/// `of_rows` over `Coincidence`'s public fields, would let any caller
/// state a pair.
#[test]
fn the_one_public_mint_of_a_given_pair_is_given_at() {
    let src = source::crate_dir(env!("CARGO_MANIFEST_DIR")).join("src");
    let mut mints = Vec::new();
    let mut gated_types = 0;
    for file in source::rust_sources(&src) {
        let code = source::code_only(&std::fs::read_to_string(&file).unwrap());
        // Each impl of a gated type, its body's span and whether it
        // implements a trait.
        let gated_impls: Vec<(std::ops::Range<usize>, bool)> = words(&code, "impl")
            .filter_map(|at| {
                let source::ItemBody::Body(body) = source::item_body(&code, at) else {
                    return None;
                };
                let head = source::impl_head(&code, at, body.start)?;
                GATED
                    .contains(&source::type_base(&head.self_type))
                    .then_some((body, head.trait_path.is_some()))
            })
            .collect();
        for at in words(&code, "fn") {
            let in_impl = gated_impls.iter().find(|(body, _)| body.contains(&at));
            let trait_fn = in_impl.is_some_and(|&(_, is_trait)| is_trait);
            if !(plainly_public(&code, at) || trait_fn) {
                continue;
            }
            let ret = head(&code, at).split_once("->").map_or("", |(_, r)| r);
            if names_a_gated_type(ret) || (in_impl.is_some() && words(ret, "Self").next().is_some())
            {
                let name = source::ident(&code, source::skip_ws(&code, at + "fn".len()));
                mints.push(format!("{}: {name}", file.display()));
            }
        }
        for ty in GATED {
            for at in words(&code, "struct").filter(|&at| {
                source::word_at(&code, source::skip_ws(&code, at + "struct".len()), ty)
            }) {
                gated_types += 1;
                assert!(plainly_public(&code, at), "{ty} is a public type");
                let derive = code[..at].rfind("#[derive(").expect("a derive on {ty}");
                let derive = &code[derive..code[derive..].find(']').unwrap() + derive];
                assert!(
                    !derive.contains("Default") && !derive.contains("Deserialize"),
                    "{ty} derives a mint: {derive}"
                );
                let source::ItemBody::Body(body) = source::item_body(&code, at) else {
                    panic!("{ty} has fields");
                };
                for (field, _) in code[body.clone()].match_indices("pub") {
                    let rest = &code[body.start + field..];
                    assert!(
                        !source::word_at(&code, body.start + field, "pub")
                            || rest.starts_with("pub("),
                        "{ty} has a public field: {}",
                        rest.lines().next().unwrap()
                    );
                }
            }
        }
    }
    assert_eq!(gated_types, 2, "the census sees both gated types");
    assert_eq!(
        mints
            .iter()
            .map(|m| m.rsplit(' ').next().unwrap())
            .collect::<Vec<_>>(),
        ["given_at"],
        "a public mint of a given pair besides given_at: {mints:?}"
    );
}
