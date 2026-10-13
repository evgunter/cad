//! **A given pair is made from a verdict some boolean decided.** Its
//! one public mint is `GlueVerdict::given_at`, and a verdict is minted
//! only by the kernel ([`topo::BooleanBody::verdicts`]). Nothing ties a
//! verdict to the operands it is handed to
//! (`work/flush/a-glue-verdict-is-not-bound-to-the-bodies-it-judged.md`).
#![allow(clippy::panic, clippy::expect_used, clippy::unwrap_used)]

use crate::common;
use common::{brick, finished};
use geom_core::Tol;
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
    let [verdict] = decided.verdicts[..] else {
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
        given: Some(vec![verdict.given_at(verdict.a())]),
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

/// Whether `s` names one of the two gated types as a whole word.
fn names_a_gated_type(s: &str) -> bool {
    s.split(|c: char| !(c.is_alphanumeric() || c == '_'))
        .any(|w| w == "GluedPair" || w == "GlueVerdict")
}

/// **`given_at` is the one public mint of a given pair**: no other
/// public function in `topo`'s source, nor a trait function on either
/// type, returns a `GluedPair` or a `GlueVerdict` (or `Self` inside
/// their impls), neither type has a public field, and neither derives
/// `Default` or `Deserialize`.
/// A second mint, such as a public `of_rows` over `Coincidence`'s public
/// fields, would let any caller state a pair.
#[test]
fn the_one_public_mint_of_a_given_pair_is_given_at() {
    fn walk(dir: &std::path::Path, out: &mut Vec<std::path::PathBuf>) {
        for e in std::fs::read_dir(dir).unwrap() {
            let p = e.unwrap().path();
            if p.is_dir() {
                walk(&p, out);
            } else if p.extension().is_some_and(|x| x == "rs") {
                out.push(p);
            }
        }
    }
    let mut files = Vec::new();
    walk(
        &std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src"),
        &mut files,
    );
    files.sort();
    let mut mints = Vec::new();
    let mut gated_types = 0;
    for f in &files {
        let src = std::fs::read_to_string(f).unwrap();
        let lines: Vec<&str> = src.lines().collect();
        let mut in_gated_impl = false;
        for (i, line) in lines.iter().enumerate() {
            let t = line.trim_start();
            if t.starts_with("impl") {
                in_gated_impl = names_a_gated_type(t.split('{').next().unwrap());
            }
            if t.starts_with("pub struct GluedPair") || t.starts_with("pub struct GlueVerdict") {
                gated_types += 1;
                let derive = lines[..i]
                    .iter()
                    .rev()
                    .find(|d| d.trim_start().starts_with("#[derive"))
                    .unwrap();
                assert!(
                    !derive.contains("Default") && !derive.contains("Deserialize"),
                    "{t} derives a mint: {derive}"
                );
                for field in lines[i + 1..].iter().take_while(|l| !l.starts_with('}')) {
                    assert!(
                        !field.trim_start().starts_with("pub "),
                        "{t} has a public field: {field}"
                    );
                }
            }
            let public = t.starts_with("pub fn ") || t.starts_with("pub const fn ");
            let trait_fn = in_gated_impl && t.starts_with("fn ");
            if !(public || trait_fn) {
                continue;
            }
            // The whole signature, which may wrap, up to its body or `;`.
            let mut sig = String::new();
            for l in &lines[i..] {
                sig.push_str(l);
                if l.contains('{') || l.trim_end().ends_with(';') {
                    break;
                }
            }
            let sig = sig.split('{').next().unwrap();
            let ret = sig.split_once("->").map_or("", |(_, r)| r);
            if names_a_gated_type(ret) || (in_gated_impl && ret.contains("Self")) {
                let name = t.split('(').next().unwrap().rsplit(' ').next().unwrap();
                mints.push(format!("{}: {name}", f.display()));
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
