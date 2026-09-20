//! **The member walk's residue**: one nominal environment per solve,
//! and the mate wire's one field-bearing type that dropped a stray
//! key.
//!
//! The solve reads every number it needs — a pattern's count, a
//! `Part`'s index, the slots a derived offset composes — at the
//! document's own parameter bindings. That environment is built ONCE
//! per `solve_document`, or handed in by the evaluation that already
//! holds it (`solve_with_env`), and passed down as a parameter. The
//! build count is pinned here by the source rather than by a probe —
//! no counter sees a `param_env` build — and the rows over parameters
//! pin what the environment IS: the document's nominal, bit for bit,
//! following an edit of the parameter and ignoring its distribution.
//!
//! `MatePrimitive` refuses a field this build lacks through the load
//! door, in the typed arm the format uses for every stray field
//! (`PersistError::Unreadable`, naming the field) — the persist module
//! docs' rule, proved for this type. The same alignment with the key
//! removed loads.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;
use editor_core::{
    Alignment, AxisSense, CapEnd, ContactClass, ContentPin, Datum, Dimension, Distribution,
    DocEdit, DocParam, DocParamValue, DocRef, DocumentId, EntityKind, EvalOptions, Expr, MateFault,
    MateFrame, MatePrimitive, Node, ParamName, PatternKind, PersistError, ProfileDoc, RecipeNodeId,
    RoleSeg, SitedFace, StableName, apply, load, save,
};
use fixture::resolver::{PartStore, in_part, with_resolver};
use fixture::{head, head_at, in_copy, insert, len, on_frame, scl, solve, step};
use geom_core::Tol;
use geom_core::linalg::Affine3;
use test_utils::source;

// ---- A1: one environment per solve ----

const MEMBER: &str = include_str!("../src/mate/member.rs");
const SOLVE: &str = include_str!("../src/mate/solve.rs");
const EVAL: &str = include_str!("../src/eval/mod.rs");

/// A file's shipped code: comments and literals blanked, cut at its
/// `#[cfg(test)]` module if it has one — a unit row there hands the
/// reader an environment the way the solve does, and building it is
/// the row's business. `must_hold` are heads the prefix has to keep,
/// so a marker that moved up the file cannot shrink the guarded text
/// to nothing.
fn shipped(text: &str, must_hold: &[&str]) -> String {
    let code = source::code_only(text);
    let end = code.find("#[cfg(test)]").unwrap_or(code.len());
    let code = code[..end].to_owned();
    for head in must_hold {
        assert!(
            code.contains(head),
            "the guarded prefix still holds `{head}` — a `#[cfg(test)]` above it would leave \
             this row reading nothing"
        );
    }
    code
}

/// **The nominal environment is built at one site of the solve**:
/// `solve_document`'s body holds the one `param_env` build under
/// `mate/`, `solve_with_env` — the entry the evaluation uses — holds
/// none, and `member.rs`, every reader of that environment, holds
/// none. A reader that rebuilt its own would put a second build in
/// one of these files, which is what this row counts.
///
/// What it cannot see: an environment reached through another door
/// (`ParamEnv { .. }` written by hand, `param_env_over`, `seed_env`),
/// and a build sited correctly but fed to nothing — the rows over
/// parameters below are what pin that the environment the solve
/// reads is the document's nominal.
#[test]
fn a1_the_solve_builds_its_nominal_environment_exactly_once() {
    const NEEDLE: &str = "param_env";
    let member = shipped(MEMBER, &["fn check_reference", "fn derived_offset"]);
    assert_eq!(
        member.matches(NEEDLE).count(),
        0,
        "member.rs takes the environment as a parameter and builds none"
    );
    let solve = shipped(SOLVE, &["pub fn solve_document", "fn solve_with_env"]);
    let builds: Vec<usize> = solve.match_indices(NEEDLE).map(|(at, _)| at).collect();
    assert_eq!(
        builds.len(),
        1,
        "solve.rs builds the environment once, at lines {:?}",
        builds
            .iter()
            .map(|&at| source::line(&solve, at))
            .collect::<Vec<_>>()
    );
    let body_of = |head: &str| {
        let at = solve
            .find(head)
            .unwrap_or_else(|| panic!("`{head}` is declared"));
        let source::ItemBody::Body(body) = source::item_body(&solve, at) else {
            panic!("`{head}` has a body");
        };
        body
    };
    assert!(
        body_of("pub fn solve_document").contains(&builds[0]),
        "the one build stands in `solve_document`'s body, not in a helper it calls"
    );
    let with_env = body_of("fn solve_with_env");
    assert!(
        !with_env.contains(&builds[0]),
        "`solve_with_env` reads the environment it is handed"
    );
    // The entry the evaluation takes is the one that takes an
    // environment, fed the nominal it already built.
    let eval = shipped(EVAL, &["pub fn evaluate"]);
    assert!(
        eval.contains("solve_with_env(doc, &nominal_env,"),
        "the evaluation hands its own nominal environment to the solve"
    );
    assert!(
        !eval.contains("solve_document("),
        "the evaluation does not take the entry that builds a second one"
    );
}

// ---- A1: the environment is the document's nominal ----

/// A slab `w × w × h`, as a whole part document.
fn slab(label: &str, w: f64, h: f64) -> ProfileDoc {
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (doc, profile) = on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(0.0, 0.0), (w, 0.0), (w, w), (0.0, w)]],
    );
    let (doc, _) = insert(
        doc,
        Node::Extrude {
            profile,
            distance: len(h),
        },
    );
    doc
}
const BASE_HEIGHT: f64 = 1.0;
const BASE_WIDTH: f64 = 3.0;
const TOP_HEIGHT: f64 = 3.0;

/// The seat every parameter row's mate declares: `b` rests on `a`.
fn seat(a: SitedFace, b: SitedFace) -> Node<editor_core::ProfileProgram> {
    Node::Mate {
        a,
        b,
        class: ContactClass::Rest,
        alignment: Alignment {
            a: MateFrame {
                origin: [1.0, 1.0, BASE_HEIGHT],
                axis: [0.0, 0.0, 1.0],
                reference: [1.0, 0.0, 0.0],
            },
            b: MateFrame {
                origin: [0.0, 0.0, 0.0],
                axis: [0.0, 0.0, -1.0],
                reference: [1.0, 0.0, 0.0],
            },
            primitive: MatePrimitive::FrameCoincidence,
            sense: AxisSense::Opposed,
            clocking: None,
        },
    }
}

/// A base and a top, the top optionally under a pattern, the mate to
/// copy `copy` of it read at the pattern.
#[derive(Clone)]
struct Scene {
    doc: ProfileDoc,
    opts: EvalOptions,
    top: RecipeNodeId,
    pattern: Option<RecipeNodeId>,
    mate: RecipeNodeId,
}

/// A pattern rule over the axis datum the scene inserts, and its count.
type Rule = (fn(RecipeNodeId) -> PatternKind, Expr);

/// The document parameters declared first, then base, top, the
/// pattern `rule` places the top by (a linear rule, or a circular one
/// over an axis datum inserted before it), and the mate.
fn scene(label: &str, params: &[(&str, DocParam)], rule: Option<Rule>, copy: u32) -> Scene {
    let mut store = PartStore::default();
    let base_ref = store.insert(
        slab(&format!("{label}-base"), BASE_WIDTH, BASE_HEIGHT),
        Tol::witness(),
    );
    let top_ref = store.insert(
        slab(&format!("{label}-top"), 1.0, TOP_HEIGHT),
        Tol::witness(),
    );
    let opts = with_resolver(store);
    let mut doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    for (name, value) in params {
        doc = step(
            doc,
            DocEdit::SetDocParam {
                name: ParamName::new(*name),
                value: value.clone(),
            },
        )
        .0;
    }
    let (doc, base) = insert(doc, Node::instantiate_part(base_ref));
    let (doc, top) = insert(doc, Node::instantiate_part(top_ref));
    let a = head(in_part(base, CapEnd::End));
    let (doc, pattern, b) = match rule {
        None => (doc, None, head(in_part(top, CapEnd::Start))),
        Some((kind, count)) => {
            let (doc, axis) = insert(
                doc,
                Node::Datum(Datum::Axis {
                    origin: [0.0, 0.0, 0.0].map(len),
                    direction: [0.0, 0.0, 1.0].map(scl),
                }),
            );
            let (doc, pattern) = insert(
                doc,
                Node::Pattern {
                    input: top,
                    count,
                    kind: kind(axis),
                },
            );
            let b = head_at(pattern, in_copy(pattern, copy, in_part(top, CapEnd::Start)));
            (doc, Some(pattern), b)
        }
    };
    let (doc, mate) = insert(doc, seat(a, b));
    Scene {
        doc,
        opts,
        top,
        pattern,
        mate,
    }
}

fn set_value(doc: ProfileDoc, name: &str, value: DocParamValue) -> ProfileDoc {
    step(
        doc,
        DocEdit::SetDocParamValue {
            name: ParamName::new(name),
            value,
        },
    )
    .0
}

fn linear_x_by_s(_axis: RecipeNodeId) -> PatternKind {
    PatternKind::Linear {
        direction: [1.0, 0.0, 0.0].map(scl),
        spacing: Expr::param(ParamName::new("s"), Dimension::Length),
    }
}

fn circular_by_th(axis: RecipeNodeId) -> PatternKind {
    PatternKind::Circular {
        axis,
        step: Expr::param(ParamName::new("th"), Dimension::Angle),
    }
}

/// The solved relative pose of the top, faults asserted absent.
fn top_pose(s: &Scene, what: &str) -> editor_core::Frame {
    let poses = solve(&s.doc, &s.opts, Tol::witness());
    assert!(
        poses.fault(s.mate).is_none() && poses.fault(s.top).is_none(),
        "{what}: {:?} / {:?}",
        poses.fault(s.mate),
        poses.fault(s.top)
    );
    poses.relative(s.top).expect("the top solves")
}

/// **A linear offset is the document's nominal parameter, bit for
/// bit.** The spacing is a parameter carrying a distribution, so an
/// environment drawn or seeded from it would place the copy somewhere
/// else; the solved offset is `1·s` exactly, `2·s` at copy 2, and
/// follows `SetDocParamValue` — the environment is the document's,
/// not a run's.
#[test]
fn a1_a_linear_offset_is_the_documents_nominal_parameter_bit_for_bit() {
    let params = [
        (
            "s",
            DocParam::continuous_with(Dimension::Length, 5.0, Distribution::Normal { sigma: 1.0 }),
        ),
        ("n", DocParam::Count { value: 3 }),
    ];
    let count = || Expr::param(ParamName::new("n"), Dimension::Count);
    let control = scene("msolve7-a1-linear-control", &params, None, 0);
    let c = top_pose(&control, "control");
    let test = scene(
        "msolve7-a1-linear",
        &params,
        Some((linear_x_by_s, count())),
        1,
    );
    let t = top_pose(&test, "s=5 copy 1");
    let diff = c.translation[0] - t.translation[0];
    assert_eq!(
        diff.to_bits(),
        5.0_f64.to_bits(),
        "the offset is 1·s exactly"
    );
    // The same number through the public expression door against the
    // document's own nominal environment.
    let via_env = editor_core::eval::<f64>(
        &Expr::param(ParamName::new("s"), Dimension::Length),
        &test.doc.param_env::<f64>(),
    )
    .unwrap();
    assert_eq!(via_env.to_bits(), diff.to_bits());
    assert_eq!(c.columns, t.columns);
    assert_eq!(c.translation[1].to_bits(), t.translation[1].to_bits());
    assert_eq!(c.translation[2].to_bits(), t.translation[2].to_bits());
    // The environment is the DOCUMENT's: a new nominal, a new offset.
    let edited = Scene {
        doc: set_value(test.doc.clone(), "s", DocParamValue::Continuous(7.5)),
        ..test.clone()
    };
    let t2 = top_pose(&edited, "s=7.5 copy 1");
    assert_eq!(
        (c.translation[0] - t2.translation[0]).to_bits(),
        7.5_f64.to_bits(),
        "the offset follows the edit"
    );
    let two = scene(
        "msolve7-a1-linear-copy2",
        &params,
        Some((linear_x_by_s, count())),
        2,
    );
    let two = Scene {
        doc: set_value(two.doc.clone(), "s", DocParamValue::Continuous(7.5)),
        ..two
    };
    let t3 = top_pose(&two, "s=7.5 copy 2");
    assert_eq!(
        (c.translation[0] - t3.translation[0]).to_bits(),
        15.0_f64.to_bits(),
        "the offset is 2·s exactly"
    );
    // One document, two solves: bit-identical.
    let again = top_pose(&edited, "again");
    assert_eq!(again.translation, t2.translation);
    assert_eq!(again.columns, t2.columns);
}

/// **The pattern's count is read at the document's own bindings**:
/// copy 2 of `n = 3` seats, and after `n → 2` the same mate refuses
/// `DanglingHead` at the pattern.
#[test]
fn a1_the_count_is_read_at_the_documents_own_bindings() {
    let params = [
        ("s", DocParam::continuous(Dimension::Length, 4.0)),
        ("n", DocParam::Count { value: 3 }),
    ];
    let count = Expr::param(ParamName::new("n"), Dimension::Count);
    let s = scene("msolve7-a1-count", &params, Some((linear_x_by_s, count)), 2);
    top_pose(&s, "n=3 copy 2");
    let shrunk = set_value(s.doc.clone(), "n", DocParamValue::Count(2));
    let poses = solve(&shrunk, &s.opts, Tol::witness());
    match poses.fault(s.mate).cloned() {
        Some(MateFault::DanglingHead { head, .. }) => assert_eq!(head, s.pattern.unwrap()),
        other => panic!("expected DanglingHead at the pattern, got {other:?}"),
    }
}

/// **A circular offset follows the document's angle parameter**: the
/// solved pose is `R_z(-θ)·control` at the nominal θ, and follows an
/// edit of θ.
#[test]
fn a1_a_circular_offset_follows_the_documents_angle_parameter() {
    let params = [("th", DocParam::continuous(Dimension::Angle, 0.7))];
    let control = scene("msolve7-a1-circular-control", &params, None, 0);
    let c = top_pose(&control, "control").affine::<f64>();
    let s = scene(
        "msolve7-a1-circular",
        &params,
        Some((circular_by_th, Expr::count(4))),
        1,
    );
    let check = |doc: ProfileDoc, th: f64, what: &str| -> Affine3<f64> {
        let sc = Scene { doc, ..s.clone() };
        let t = top_pose(&sc, what).affine::<f64>();
        let expected = Affine3::rotation_about_axis(
            geom_core::Point3::origin(),
            geom_core::Vec3::new(0.0, 0.0, 1.0),
            -th,
        ) * c;
        let gap = fixture::seat::map_gap(&t, &expected);
        assert!(gap <= 1e-12, "{what}: gap to R(-θ)·control is {gap:e}");
        assert!(
            fixture::seat::map_gap(&t, &c) > 0.1,
            "{what}: the pattern moved the pose"
        );
        t
    };
    let t1 = check(s.doc.clone(), 0.7, "θ=0.7");
    let t2 = check(
        set_value(s.doc.clone(), "th", DocParamValue::Continuous(1.3)),
        1.3,
        "θ=1.3",
    );
    assert!(
        fixture::seat::map_gap(&t1, &t2) > 0.1,
        "the edit moved the pose"
    );
}

// ---- A4: the mate wire's attribute ----

/// A document with two instances and one planar-rest mate between
/// them, saved: the text a stray key is injected into.
fn saved_with_a_planar_rest(label: &str) -> (ProfileDoc, String) {
    let doc_ref = DocRef {
        id: DocumentId::derive("msolve7-part"),
        pin: ContentPin([7u8; 32]),
    };
    let mut doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let mut ids = Vec::new();
    for _ in 0..2 {
        let applied = apply(
            &doc,
            &DocEdit::InsertNode {
                node: Node::instantiate_part(doc_ref),
            },
            Tol::witness(),
            &editor_core::RefusingReach,
        )
        .expect("an instance inserts");
        ids.push(applied.record.minted.expect("a minted id"));
        doc = applied.doc;
    }
    let name = |node| StableName {
        kind: EntityKind::Face,
        node,
        path: vec![RoleSeg::InPart {
            of: StableName {
                kind: EntityKind::Face,
                node: RecipeNodeId(1),
                path: vec![RoleSeg::Cap(CapEnd::Start)],
            }
            .into(),
        }],
    };
    let f = MateFrame {
        origin: [0.0, 0.0, 0.0],
        axis: [0.0, 0.0, 1.0],
        reference: [1.0, 0.0, 0.0],
    };
    let doc = apply(
        &doc,
        &DocEdit::InsertNode {
            node: Node::Mate {
                a: crate::fixture::head(name(ids[0])),
                b: crate::fixture::head(name(ids[1])),
                class: ContactClass::Rest,
                alignment: Alignment {
                    a: f,
                    b: f,
                    primitive: MatePrimitive::PlanarRest { offset: 0.5 },
                    sense: AxisSense::Opposed,
                    clocking: None,
                },
            },
        },
        Tol::witness(),
        &editor_core::RefusingReach,
    )
    .expect("a mate inserts")
    .doc;
    let text = save(&doc, &[], Tol::witness()).expect("saves");
    (doc, text)
}

/// `text` with `injected` placed just inside the planar rest's own
/// object — the one place the wire has a named field to deny.
fn with_a_key_on_the_planar_rest(text: &str, injected: &str) -> String {
    const ANCHOR: &str = "\"planar_rest\": {";
    assert_eq!(
        text.matches(ANCHOR).count(),
        1,
        "the saved document spells one planar rest: {text}"
    );
    let open = text.find(ANCHOR).unwrap() + ANCHOR.len();
    format!("{}{injected}{}", &text[..open], &text[open..])
}

/// `text` with `injected` placed just after `anchor`, which the text
/// spells once.
fn inject_after(text: &str, anchor: &str, injected: &str) -> String {
    assert_eq!(
        text.matches(anchor).count(),
        1,
        "the saved document spells {anchor:?} once: {text}"
    );
    let open = text.find(anchor).unwrap() + anchor.len();
    format!("{}{injected}{}", &text[..open], &text[open..])
}

/// The load door's answer to `text`: the field the typed arm names,
/// or a description of whatever else it said.
fn refused_field(text: &str) -> Result<String, String> {
    match load(text, Tol::witness()) {
        Err(PersistError::Unreadable { detail, .. }) => detail
            .split('`')
            .nth(1)
            .filter(|_| detail.contains("unknown field"))
            .map(str::to_owned)
            .ok_or(detail),
        Err(other) => Err(format!("{other:?}")),
        Ok(_) => Err("loaded".to_owned()),
    }
}

/// **A stray key on `planar_rest` refuses at the load door**, in the
/// format's own arm for a field this build lacks, naming the field:
/// before `offset`, after it, and spelled with a word another part of
/// the wire owns (`clocking` is the alignment's rider, not the
/// primitive's field).
#[test]
fn a4_a_stray_key_on_a_planar_rest_refuses_at_the_load_door() {
    let (_, text) = saved_with_a_planar_rest("msolve7-a4-stray");
    let before = with_a_key_on_the_planar_rest(&text, "\"stray\": 2.0,");
    assert_eq!(refused_field(&before).as_deref(), Ok("stray"));
    let after = inject_after(&text, "\"offset\": 0.5", ", \"stray\": 2.0");
    assert_eq!(refused_field(&after).as_deref(), Ok("stray"));
    let borrowed = with_a_key_on_the_planar_rest(&text, "\"clocking\": 2.0,");
    assert_eq!(refused_field(&borrowed).as_deref(), Ok("clocking"));
}

/// **Every checked-in document still loads under the attribute** —
/// the stop clause's measurement, made rather than assumed. None of
/// the four carries a mate, so the corpus does not exercise the
/// attribute at all: what it proves is that nothing checked in is
/// refused, not that the attribute is reached.
#[test]
fn a4_every_checked_in_document_loads_and_none_carries_a_mate() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("../..");
    let listed = std::process::Command::new("git")
        .args(["ls-files", "-z", "--", "*.pncad"])
        .current_dir(&root)
        .output()
        .expect("git lists the tracked files");
    assert!(listed.status.success(), "{listed:?}");
    let walked: Vec<String> = String::from_utf8(listed.stdout)
        .expect("paths are utf-8")
        .split('\0')
        .filter(|p| !p.is_empty())
        .map(str::to_owned)
        .collect();
    assert_eq!(walked.len(), 4, "the corpus this row walks: {walked:?}");
    for path in &walked {
        let text = std::fs::read_to_string(root.join(path)).expect("readable");
        let loaded = match load(&text, Tol::witness()) {
            Ok(loaded) => loaded,
            // A document authored at another ε refuses at this
            // process's ε before any field is read; the other ε rows'
            // subject, not this row's.
            Err(PersistError::ToleranceConflict { .. }) => continue,
            Err(e) => panic!("{path}: loads under the attribute: {e}"),
        };
        let mates = loaded
            .doc
            .order()
            .iter()
            .filter(|&&id| matches!(loaded.doc.node(id), Some(Node::Mate { .. })))
            .count();
        assert_eq!(mates, 0, "{path}: the corpus carries no mate");
    }
}

/// **The same alignment without the key loads**, bit for bit: the
/// attribute denies what is not there and nothing that is.
#[test]
fn a4_the_same_alignment_without_the_key_loads() {
    let (doc, text) = saved_with_a_planar_rest("msolve7-a4-clean");
    let back = load(&text, Tol::witness()).expect("loads").doc;
    assert!(back.bit_eq(&doc), "the planar rest round-trips bit for bit");
    let primitives: Vec<MatePrimitive> = back
        .order()
        .iter()
        .filter_map(|&id| match back.node(id) {
            Some(Node::Mate { alignment, .. }) => Some(alignment.primitive),
            _ => None,
        })
        .collect();
    assert_eq!(primitives, [MatePrimitive::PlanarRest { offset: 0.5 }]);
}
