//! **A node's standing: one type, one read, every door.**
//!
//! "This node has no usable value" is one fact with four shapes — the
//! run stopped before it, not a node of this document, failed,
//! poisoned through its nearest failed ancestor — and it is
//! [`NodeStanding`], read by [`Evaluation::usable`]. Each door that
//! needs a node's value refuses with the standing as its payload, under
//! its own subject; none re-spells the arms or their sentences.
//!
//! The rows:
//!
//! - the census: every reader of a node's result in shipped `src` goes
//!   through the one read, or is listed with its reason;
//! - every standing renders one way through every door that carries
//!   it, each door's subject in front;
//! - no door that runs no hit test says "hit test" for a standing;
//! - a poisoned datum reaching the distance query carries `through`;
//! - the checks registry's root refusal names the node the repair is
//!   at;
//! - `RunStatus`, the standing's persisted projection, keeps its JSON
//!   words and its key bytes.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture::{self, insert, len, minted, on_frame, step};
use editor_core::analysis::ParamBox;
use editor_core::clearance::{
    ClearanceRefusal, ClearanceVerdict, Selection, SelectionRefusal, clearance,
};
use editor_core::{
    CancelToken, ChecksConfig, ChecksError, Cmp, DocEdit, EntityKind, EvalOptions, Evaluation,
    GeomPred, HitTestError, InterrogateError, NameLookupError, NamePat, Node, NodePick,
    NodePickError, NodeStanding, ProfileDoc, RecipeNodeId, Resolution, ResolveIndeterminate,
    RoleSeg, RunCtx, RunStatus, SelectRefusal, Selector, SlotId, VerdictRow, VerdictVector,
    body_name, denotation, evaluate, find_flush_candidates, resolve, run_checks, select_where,
};
use geom_core::Tol;

const DELTA: f64 = 0.1;

fn run(doc: &ProfileDoc, cancel: &CancelToken) -> Evaluation<f64> {
    evaluate::<f64>(doc, None, cancel, &EvalOptions::default(), Tol::witness())
}

/// One document in the three standings.
struct Standings {
    /// The document with `failed`'s extrude degenerate.
    doc: ProfileDoc,
    /// The sketch the extrude reads — it evaluates in every run but
    /// the canceled one.
    profile: RecipeNodeId,
    /// The degenerate extrude.
    failed: RecipeNodeId,
    /// A transform of it, poisoned through it.
    poisoned: RecipeNodeId,
    /// A run of the document before the extrude went degenerate, in
    /// which both nodes built: the picks are built against it.
    good: Evaluation<f64>,
    /// A run of the same document after it did.
    broken: Evaluation<f64>,
    /// A run of the same document canceled before any node ran.
    canceled: Evaluation<f64>,
}

impl Standings {
    fn new() -> Self {
        let (doc, profile) = on_frame(
            ProfileDoc::empty_derived("node_standing", Tol::witness()),
            [0.0; 3],
            [1.0, 0.0, 0.0],
            [0.0, 1.0, 0.0],
            vec![vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]],
        );
        let (doc, failed) = insert(
            doc,
            Node::Extrude {
                profile,
                distance: len(1.0),
            },
        );
        let (doc, poisoned) = insert(
            doc,
            fixture::xform(failed, [2.0, 0.0, 0.0], [0.0, 0.0, 1.0], 0.0),
        );
        let good = run(&doc, &CancelToken::new());
        let (doc, _) = step(
            doc,
            DocEdit::SetParam {
                node: failed,
                slot: SlotId::Distance,
                expr: len(0.0),
            },
        );
        let broken = run(&doc, &CancelToken::new());
        let cancel = CancelToken::new();
        cancel.cancel();
        let canceled = run(&doc, &cancel);
        Self {
            doc,
            profile,
            failed,
            poisoned,
            good,
            broken,
            canceled,
        }
    }

    /// The failed and poisoned standings, in the broken run.
    fn broken_standings(&self) -> [NodeStanding; 2] {
        [
            NodeStanding::Failed { node: self.failed },
            NodeStanding::Poisoned {
                node: self.poisoned,
                through: self.failed,
            },
        ]
    }
}

/// `door`'s refusal renders as `prefix` followed by the standing's own
/// sentence, exactly.
fn speaks(door: &str, prefix: &str, refusal: &dyn core::fmt::Display, standing: NodeStanding) {
    assert_eq!(
        refusal.to_string(),
        format!("{prefix}{standing}"),
        "{door} renders {standing:?} as its subject and then the standing's own words"
    );
}

/// **Every standing renders one way through every door that carries
/// it**, each under that door's own subject: the hit test, the
/// pick-index build, the name lookup, the name read, the resolution
/// verdict, the distance and flush queries, the checks registry, the
/// product gather and the clearance engine's selection (`pncad`'s suite holds the export door
/// to the same shape, and `viewer`'s the duplicate and the blend tool).
/// Each door's payload IS the standing [`Evaluation::usable`] answers,
/// so no door can name a different node, lose `through`, or word the
/// state its own way.
#[test]
fn every_standing_renders_one_way_through_every_door() {
    let s = Standings::new();
    let foreign = RecipeNodeId(9999);
    let [failed, poisoned] = s.broken_standings();
    let cases = [
        (&s.broken, failed),
        (&s.broken, poisoned),
        (&s.broken, NodeStanding::NotInDocument { node: foreign }),
        (&s.canceled, NodeStanding::NotEvaluated { node: s.failed }),
    ];

    for (eval, standing) in cases {
        let node = standing.node();
        assert_eq!(
            eval.usable(node).err(),
            Some(standing),
            "the one read answers {standing:?}"
        );

        let hit = body_name(eval, node, 0).expect_err("no table to invert");
        assert_eq!(hit, HitTestError::Standing(standing));
        speaks("the hit test", "hit test: ", &hit, standing);

        let pick =
            NodePick::build(eval, node, 0, DELTA, Tol::witness()).expect_err("no body to index");
        assert_eq!(pick, NodePickError::Standing(standing));
        speaks("the pick-index build", "pick: ", &pick, standing);

        let name = minted(EntityKind::Body, node, RoleSeg::OutputBody);
        let read = denotation(eval, node, &name).expect_err("no table to read");
        assert_eq!(read, InterrogateError::Standing(standing));
        speaks("the name read", "", &read, standing);

        // The distance query reads its datum only for a queried node
        // that has a value, which the canceled run has none of.
        if eval.usable(s.profile).is_ok() {
            let query = select_where(
                eval,
                s.profile,
                &Selector::of(NamePat::of_kind(EntityKind::Face)),
                &[GeomPred::DatumDistance {
                    datum: node,
                    cmp: Cmp::Approx,
                    value: len(0.0),
                }],
                &s.doc.param_env::<f64>(),
                Tol::witness(),
            )
            .expect_err("a datum with no value");
            assert!(
                matches!(query, SelectRefusal::DatumHasNoValue(carried) if carried == standing),
                "{query:?}"
            );
            speaks(
                "the distance query",
                "select: the distance query's datum has no value: ",
                &query,
                standing,
            );
        }

        let flush = find_flush_candidates(eval, node, s.profile, Tol::witness())
            .expect_err("a node with no value");
        assert!(
            matches!(flush, SelectRefusal::NodeHasNoValue(carried) if carried == standing),
            "{flush:?}"
        );
        speaks(
            "the flush query",
            "select: the flush query's node has no value: ",
            &flush,
            standing,
        );
    }

    // The checks registry reads the document's roots, and a root is
    // never an ancestor of another: the poisoned transform is the one
    // root here, in the broken run and the canceled one, and the failed
    // extrude is the root of a document of its own (below).
    let (rooted, _) = step(
        s.doc.clone(),
        DocEdit::SetRoots {
            roots: vec![s.poisoned],
        },
    );
    for (eval, standing) in [
        (&s.broken, poisoned),
        (&s.canceled, NodeStanding::NotEvaluated { node: s.poisoned }),
    ] {
        let checks = run_checks(&rooted, eval, &ChecksConfig::default(), Tol::witness())
            .expect_err("a root with no value refuses the registry");
        assert_eq!(checks, ChecksError::Root(standing));
        speaks("the checks registry", "checks: root ", &checks, standing);
        let gather = editor_core::product(&rooted, eval, Tol::witness())
            .expect_err("a root with no value refuses the gather");
        speaks("the product gather", "product: root ", &gather, standing);
    }

    // The clearance engine replays the document itself, so its
    // standings are the uncanceled run's.
    let leaf = ParamBox::from_axes(std::collections::BTreeMap::new());
    for standing in [
        failed,
        poisoned,
        NodeStanding::NotInDocument { node: foreign },
    ] {
        let sel = Selection::body_of(standing.node());
        let report = clearance(&s.doc, &leaf, &sel, &sel, 0.1, Tol::witness());
        let ClearanceVerdict::Refused(ClearanceRefusal::Selection(refusal)) = report.verdict()
        else {
            panic!(
                "{standing:?}: a selection refusal, got {:?}",
                report.verdict()
            );
        };
        assert_eq!(refusal, &SelectionRefusal::NodeDidNotBuild(standing));
        speaks(
            "the clearance selection",
            "the selection has no faces to measure a clearance between in this leaf's replay: ",
            refusal,
            standing,
        );
    }

    // The resolution verdict, over a name each standing's node mints:
    // a foreign id's name is `NodeGone` rather than indeterminate, so
    // the unevaluated standing is the canceled run's.
    let unevaluated = NodeStanding::NotEvaluated { node: s.failed };
    for (eval, standing) in [
        (&s.broken, s.broken_standings()[0]),
        (&s.broken, s.broken_standings()[1]),
        (&s.canceled, unevaluated),
    ] {
        let name = minted(EntityKind::Body, standing.node(), RoleSeg::OutputBody);
        let verdict = resolve(RunCtx { doc: &s.doc, eval }, &name);
        let Resolution::Indeterminate(cause) = verdict else {
            panic!("{standing:?}: the reference is indeterminate, got {verdict:?}");
        };
        assert_eq!(cause, ResolveIndeterminate { standing });
        speaks(
            "the resolution verdict",
            "the reference is indeterminate until its minting node evaluates: ",
            &cause,
            standing,
        );
    }

    // The name lookup, over picks built while both nodes had a value:
    // the later runs of the same document are admitted by the pairing
    // and refused by the standing.
    for (node, broken_standing) in [s.failed, s.poisoned].into_iter().zip(s.broken_standings()) {
        let pick = NodePick::build(&s.good, node, 0, DELTA, Tol::witness()).expect("it built");
        for (eval, standing) in [
            (&s.broken, broken_standing),
            (&s.canceled, NodeStanding::NotEvaluated { node }),
        ] {
            for refusal in [
                pick.patch_names(eval).expect_err("no table"),
                pick.boundary_names(eval).expect_err("no table"),
            ] {
                assert_eq!(refusal, NameLookupError::Standing(standing));
                speaks("the name lookup", "name lookup: ", &refusal, standing);
            }
        }
    }
}

/// **The checks registry's root refusal names the node the repair is
/// at.** A root with no value refuses the registry; a poisoned root's
/// repair is upstream, at the failure that poisoned it, and a failed
/// root's is its own. The refusal carries the standing, so it says
/// which — where it used to tell the author to "fix or remove the
/// failing root" about a root that had not failed.
#[test]
fn the_checks_root_refusal_names_the_node_the_repair_is_at() {
    let s = Standings::new();
    let (rooted, _) = step(
        s.doc.clone(),
        DocEdit::SetRoots {
            roots: vec![s.poisoned],
        },
    );
    let refusal = run_checks(&rooted, &s.broken, &ChecksConfig::default(), Tol::witness())
        .expect_err("a poisoned root refuses the registry");
    assert_eq!(
        refusal,
        ChecksError::Root(NodeStanding::Poisoned {
            node: s.poisoned,
            through: s.failed
        })
    );
    assert!(
        refusal
            .to_string()
            .contains(&format!("the repair is upstream, at node {}", s.failed.0)),
        "{refusal}"
    );

    // A failed root, the leaf of a document of its own.
    let (doc, profile) = on_frame(
        ProfileDoc::empty_derived("node_standing_checks", Tol::witness()),
        [0.0; 3],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]],
    );
    let (doc, failed) = insert(
        doc,
        Node::Extrude {
            profile,
            distance: len(0.0),
        },
    );
    let (doc, _) = step(
        doc,
        DocEdit::SetRoots {
            roots: vec![failed],
        },
    );
    let ev = run(&doc, &CancelToken::new());
    let standing = NodeStanding::Failed { node: failed };
    let refusal = run_checks(&doc, &ev, &ChecksConfig::default(), Tol::witness())
        .expect_err("a failed root refuses the registry");
    assert_eq!(refusal, ChecksError::Root(standing));
    speaks("the checks registry", "checks: root ", &refusal, standing);
}

/// **No door that runs no hit test says it ran one.** The pick-index
/// build (`NodePick::build`, `build_all`) and the name doors
/// (`patch_names`, `boundary_names`) read a node's value and a table;
/// a standing refusal out of them names the standing under their own
/// subject. The viewer forwards these sentences whole into its pick
/// banner, where "hit test:" would report an event that did not
/// happen.
#[test]
fn no_standing_refusal_says_hit_test_outside_the_hit_test() {
    let s = Standings::new();
    let mut said: Vec<String> = Vec::new();
    for standing in s.broken_standings() {
        let node = standing.node();
        said.push(
            NodePick::build(&s.broken, node, 0, DELTA, Tol::witness())
                .expect_err("no body")
                .to_string(),
        );
        said.push(
            NodePick::build_all(&s.broken, node, DELTA, Tol::witness())
                .expect_err("no body")
                .to_string(),
        );
        let pick = NodePick::build(&s.good, node, 0, DELTA, Tol::witness()).expect("it built");
        said.push(
            pick.patch_names(&s.broken)
                .expect_err("no table")
                .to_string(),
        );
        said.push(
            pick.boundary_names(&s.broken)
                .expect_err("no table")
                .to_string(),
        );
    }
    assert_eq!(said.len(), 8, "four doors, two standings");
    for text in &said {
        assert!(
            !text.contains("hit test"),
            "a door that ran no hit test: {text}"
        );
    }
}

/// **A poisoned datum carries `through`.** The distance query's datum
/// is poisoned by the failure upstream of it; the refusal says so with
/// the node the repair is at, where it used to fold "a poisoned node"
/// into a word and drop which failure did it.
#[test]
fn a_poisoned_datum_carries_through_to_the_select_refusal() {
    let s = Standings::new();
    let refusal = select_where(
        &s.broken,
        s.profile,
        &Selector::of(NamePat::of_kind(EntityKind::Face)),
        &[GeomPred::DatumDistance {
            datum: s.poisoned,
            cmp: Cmp::Approx,
            value: len(0.0),
        }],
        &s.doc.param_env::<f64>(),
        Tol::witness(),
    )
    .expect_err("a poisoned datum");
    let SelectRefusal::DatumHasNoValue(standing) = refusal else {
        panic!("a datum with no value is its own refusal: {refusal:?}");
    };
    assert_eq!(
        standing.through(),
        Some(s.failed),
        "the repair's node rides the refusal"
    );
    assert!(
        refusal
            .to_string()
            .contains(&format!("at node {}", s.failed.0)),
        "{refusal}"
    );
}

/// **`RunStatus` keeps its JSON words and its key bytes.** It is the
/// standing's persisted projection — `Ok`, or the standing's kind with
/// `NotEvaluated` spelled `Absent` — carried by the ε audit's
/// cross-process summary and by every certified leaf's verdict-vector
/// key. Both are pinned as literals, and a run's own rows are read
/// through the one door, so a standing mapped to the wrong status, a
/// word that moved, or a key byte that moved reds here.
#[test]
fn run_status_round_trips_its_json_and_keeps_its_key_bytes() {
    for (status, word) in [
        (RunStatus::Ok, "\"Ok\""),
        (RunStatus::Failed, "\"Failed\""),
        (RunStatus::Poisoned, "\"Poisoned\""),
        (RunStatus::Absent, "\"Absent\""),
    ] {
        let json = serde_json::to_string(&status).expect("serializes");
        assert_eq!(json, word, "{status:?}'s word");
        let back: RunStatus = serde_json::from_str(&json).expect("deserializes");
        assert_eq!(back, status, "{status:?} round-trips");
    }

    let s = Standings::new();
    let outcomes: Vec<(RecipeNodeId, RunStatus)> = VerdictVector::of(&s.broken)
        .rows
        .iter()
        .map(|row| (row.node, row.outcome))
        .collect();
    assert!(
        outcomes.contains(&(s.profile, RunStatus::Ok)),
        "{outcomes:?}"
    );
    assert!(
        outcomes.contains(&(s.failed, RunStatus::Failed)),
        "{outcomes:?}"
    );
    assert!(
        outcomes.contains(&(s.poisoned, RunStatus::Poisoned)),
        "{outcomes:?}"
    );
    assert!(
        VerdictVector::of(&s.canceled)
            .rows
            .iter()
            .all(|row| row.outcome == RunStatus::Absent),
        "a canceled run's rows are absent"
    );

    let key = VerdictVector {
        rows: [
            RunStatus::Ok,
            RunStatus::Failed,
            RunStatus::Poisoned,
            RunStatus::Absent,
        ]
        .into_iter()
        .zip(1..)
        .map(|(outcome, id)| VerdictRow {
            node: RecipeNodeId(id),
            outcome,
            verdicts: Vec::new(),
        })
        .collect(),
    }
    .key();
    assert_eq!(
        key.0, 70_071_079_698_853_325_673_526_678_003_706_389_063,
        "the four outcomes' key bytes"
    );
}

/// Hits of one reader of a node's result in `code`: the type's name as
/// a whole word — every path to it names it, an alias included, since
/// `use … NodeResult as R` spells it — plus the poisoned arm's own
/// accessor and [`Evaluation::result`] called with an id.
fn reads(code: &str) -> usize {
    let name = "NodeResult";
    let named = code
        .match_indices(name)
        .filter(|&(at, _)| test_utils::source::word_at(code, at, name))
        .count();
    let through = code.matches(".poisoned_through(").count();
    let result = code.matches(".result(").count() - code.matches(".result()").count();
    named + through + result
}

/// **Every reader of a node's result outside the one read, and why.**
///
/// A file here names `NodeResult` or reads [`Evaluation::result`]
/// because it needs what the standing does not carry — a failed node's
/// own error, a value's payload beside a failure's kind — or because it
/// is where results are written or re-exported. The count is
/// [`reads`]'s hits in the file's code; a hit added anywhere reds, and
/// the answer is to read through `Evaluation::usable` or to give the
/// file its line here.
const READERS: [(&str, usize, &str); 11] = [
    (
        "crates/editor-core/src/eval/mod.rs",
        24,
        "the home: the evaluator writes every result, and `usable_in` is the one ladder",
    ),
    (
        "crates/editor-core/src/eval/wire.rs",
        2,
        "the op wiring holds the result map as it is written, and reads it through `usable_in`",
    ),
    ("crates/editor-core/src/lib.rs", 1, "re-exports the type"),
    (
        "crates/pncad/src/document.rs",
        1,
        "re-exports the type on the public surface",
    ),
    (
        "crates/editor-core/src/drive.rs",
        4,
        "reads a failed node's escalation log, which the standing does not carry",
    ),
    (
        "crates/editor-core/src/stackup.rs",
        23,
        "the pairing compares two runs arm by arm — values, failure kinds, poison sources \
         — and labels each arm in its prose",
    ),
    (
        "crates/editor-core/src/mate/member.rs",
        9,
        "unit-test assertions about a node's own row",
    ),
    (
        "crates/viewer/src/bounds.rs",
        4,
        "collects the failed nodes a bounds verdict names, poisoned ones deliberately not",
    ),
    (
        "crates/viewer/src/tree.rs",
        8,
        "the tree row renders a failed node's own error and links the node its error names \
         for repair, and a poisoned row renders its source's",
    ),
    (
        "demos/tour/src/chaintol.rs",
        4,
        "prints a failed node's typed kind, as a consumer of the public result enum",
    ),
    (
        "crates/editor-core/src/eval/parts.rs",
        1,
        "moves the failure behind a failed or poisoned part root out of the part's \
         evaluation, which the standing does not carry",
    ),
];

/// **The census.** Every file of shipped `src` — each crate's and each
/// demo's — whose code reads a node's result other than through
/// [`Evaluation::usable`] is in [`READERS`] with its count and its
/// reason, and no other file does.
///
/// Blind spot: a read that never names the type nor calls
/// `Evaluation::result` — `ev.nodes.get(&id)` followed by a method on
/// the unnamed result. `Evaluation::nodes` stays `pub` (its readers in
/// every crate's tests would all move for a rule `result()` would
/// still leave open), so that shape is swept by hand in the PR.
#[test]
fn every_node_result_reader_goes_through_usable_or_is_listed() {
    let root = test_utils::source::repo_root(env!("CARGO_MANIFEST_DIR"));
    let mut found: Vec<(String, usize)> = Vec::new();
    for parent in ["crates", "demos"] {
        for entry in std::fs::read_dir(root.join(parent)).expect("listing") {
            let src = entry.expect("dir entry").path().join("src");
            if !src.is_dir() {
                continue;
            }
            for path in test_utils::source::rust_sources(&src) {
                let text = std::fs::read_to_string(&path).expect("readable source file");
                let hits = reads(&test_utils::source::code_only(&text));
                if hits == 0 {
                    continue;
                }
                let rel = path
                    .strip_prefix(&root)
                    .expect("a walked file lies under the root")
                    .to_string_lossy()
                    .replace('\\', "/");
                found.push((rel, hits));
            }
        }
    }
    found.sort();
    let mut listed: Vec<(String, usize)> = READERS
        .iter()
        .map(|&(file, count, _)| (file.to_owned(), count))
        .collect();
    listed.sort();
    assert_eq!(
        found, listed,
        "a node's result is read somewhere other than `Evaluation::usable`; read it \
         through that door, or give the file its line in `READERS` with a reason"
    );
}

/// The census's needle sees an aliased import, a module-qualified path
/// and the accessor, and not a longer identifier or the no-argument
/// `result()` of another type.
#[test]
fn the_census_needle_sees_every_path_to_the_result() {
    for (code, want) in [
        (
            "use crate::eval::NodeResult as R; match r { R::Ok(_) => 1 }",
            1,
        ),
        ("match x { Some(d::NodeResult::Failed(e)) => e }", 1),
        ("ev.result(id).and_then(|r| r.poisoned_through())", 2),
        ("r.poisoned_through()", 1),
        ("ev.result(id)", 1),
        ("probe.result()", 0),
        ("NodeResultish::Ok", 0),
        ("MyNodeResult", 0),
    ] {
        assert_eq!(reads(code), want, "{code}");
    }
}
