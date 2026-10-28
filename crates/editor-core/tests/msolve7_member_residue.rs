//! **The solve's one environment, and the constraint wire's denial of
//! a stray key.**
//!
//! A placement folds its bundle where it evaluates, so every number
//! the fold reads — a value's slide or spin, a standoff, a world read's
//! moves — is read in the environment of the evaluation's own lane,
//! which it already holds. The solve builds no environment of its own:
//! `solve_document` IS an evaluation. The build count is pinned here by
//! the source rather than by a probe — no counter sees a `var_env`
//! build — and the rows over parameters pin what that environment IS
//! for the nominal solve: the document's nominal, bit for bit,
//! following an edit of the parameter and ignoring its distribution.
//!
//! A constraint's objects on the wire — the mate and the value —
//! refuse a field this build lacks through the load door, in the typed
//! arm the format uses for every stray field (`PersistError::Unreadable`,
//! naming the field); the same bundle without the key loads.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;
use crate::wire;
use editor_core::pose::{PoseDef, PoseRole, RoleRead};
use editor_core::{
    CapEnd, ContentPin, Coord, Dimension, Distribution, DocEdit, DocRef, DocumentId, EntityKind,
    EvalOptions, Formula, FreeValue, FreeVar, MateRef, Node, Operand, PersistError, ProfileDoc,
    RecipeNodeId, RoleSeg, SitedFace, StableName, TargetAxis, VarName, apply, load, save,
};
use fixture::resolver::{PartStore, in_part, with_resolver};
use fixture::{ang, head_at, insert, len, on_frame, solve, step};
use geom_core::linalg::Affine3;
use geom_core::{Point3, Tol, Vec3};
use test_utils::source;

// ---- A1: the solve builds no environment ----

const SOLVE: &str = include_str!("../src/mate/solve.rs");
const VALUE: &str = include_str!("../src/mate/value.rs");
const PIN: &str = include_str!("../src/mate/pin.rs");
const CORNER: &str = include_str!("../src/mate/corner.rs");
const COSET: &str = include_str!("../src/mate/coset.rs");
const WIRE: &str = include_str!("../src/eval/wire.rs");
const EVAL: &str = include_str!("../src/eval/mod.rs");

/// A file's shipped code: comments and literals blanked, cut at its
/// `#[cfg(test)]` module if it has one — a unit row there hands the
/// reader an environment the way the evaluation does, and building it
/// is the row's business. `must_hold` are heads the prefix has to keep,
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

/// The body of the item `head` declares in `code`.
fn body_of<'a>(code: &'a str, head: &str) -> &'a str {
    let at = code
        .find(head)
        .unwrap_or_else(|| panic!("`{head}` is declared"));
    let source::ItemBody::Body(body) = source::item_body(code, at) else {
        panic!("`{head}` has a body");
    };
    &code[body]
}

/// **The solve builds no environment; the fold reads the lane's**:
/// the mate modules hold no `var_env` build; `solve_document` answers
/// off an evaluation (its body hands off to `solve_document_at`, which
/// calls `evaluate`); the fold, `fold_with` in `eval/wire.rs`, reads the
/// parameters of the lane it runs in (`env.lane.params`) and builds
/// none; and the evaluation never calls back into `solve_document`,
/// which would evaluate the document a second time.
///
/// What it cannot see: an environment reached through another door
/// (`VarEnv { .. }` written by hand, `var_env_over`, `seed_env`), and a
/// lane environment built correctly but not the one the fold is handed
/// — the rows over parameters below are what pin that the environment
/// the nominal solve reads is the document's nominal.
///
/// *Breaks if* a mate module builds an environment of its own, the
/// solve stops being an evaluation, or the fold stops reading its
/// lane's parameters.
#[test]
fn a1_the_solve_builds_no_environment_and_the_fold_reads_its_lanes() {
    const NEEDLE: &str = "var_env";
    for (file, text, heads) in [
        (
            "mate/solve.rs",
            SOLVE,
            &["pub fn solve_document", "fn solve_bundle"][..],
        ),
        ("mate/value.rs", VALUE, &["fn settle"][..]),
        ("mate/pin.rs", PIN, &["pub fn of<P>"][..]),
        ("mate/corner.rs", CORNER, &["pub fn corner_frame"][..]),
        ("mate/coset.rs", COSET, &["pub fn intersect"][..]),
    ] {
        let code = shipped(text, heads);
        assert_eq!(
            code.matches(NEEDLE).count(),
            0,
            "{file} builds no environment"
        );
    }
    let solve = shipped(SOLVE, &["pub fn solve_document"]);
    assert!(
        body_of(&solve, "pub fn solve_document(").contains("solve_document_at("),
        "`solve_document` is `solve_document_at` at `f64`"
    );
    // `solve_document_at` lives at the evaluation's scalar seam
    // (`eval/mod.rs`), re-exported from the solve.
    let at = shipped(EVAL, &["pub fn solve_document_at"]);
    let at = body_of(&at, "pub fn solve_document_at");
    assert!(at.contains("evaluate::<T>("), "the solve is the evaluation");
    assert_eq!(at.matches(NEEDLE).count(), 0, "the solve builds none");
    let wire = shipped(WIRE, &["fn fold_with"]);
    let fold = body_of(&wire, "fn fold_with");
    assert!(
        fold.contains("env.lane.params"),
        "the fold reads the parameters of the lane it runs in"
    );
    assert_eq!(fold.matches(NEEDLE).count(), 0, "the fold builds none");
    let eval = shipped(EVAL, &["pub fn evaluate"]);
    assert!(
        !eval.contains("solve_document("),
        "the evaluation never asks the entry that evaluates again"
    );
}

// ---- A1: the environment is the document's nominal ----

/// A slab `w × w × h`, as a whole part document, and its body.
fn slab(label: &str, w: f64, h: f64) -> (ProfileDoc, RecipeNodeId) {
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (doc, profile) = on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(0.0, 0.0), (w, 0.0), (w, w), (0.0, w)]],
    );
    insert(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: len(h),
        },
    )
}
const BASE_HEIGHT: f64 = 1.0;
const BASE_WIDTH: f64 = 3.0;
const TOP_HEIGHT: f64 = 3.0;

/// A base pinned to the world and a top seated on it: the top's
/// bottom cap resting on the base's top cap, its origin over `(x, 1)`
/// and turned `turn` about the cap's normal — the rest and its three
/// values, `x` and `turn` as written.
#[derive(Clone)]
struct Scene {
    doc: ProfileDoc,
    opts: EvalOptions,
    top: RecipeNodeId,
    rest: MateRef,
}

/// The document parameters declared first, then base, top and the
/// seat.
fn scene(label: &str, params: &[(&'static str, FreeVar)], x: Formula, turn: Formula) -> Scene {
    let mut store = PartStore::default();
    let (base_ref, base_body) = store.insert_part(
        slab(&format!("{label}-base"), BASE_WIDTH, BASE_HEIGHT),
        Tol::witness(),
    );
    let (top_ref, top_body) = store.insert_part(
        slab(&format!("{label}-top"), 1.0, TOP_HEIGHT),
        Tol::witness(),
    );
    let opts = with_resolver(store);
    let mut doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    for (name, value) in params {
        doc = step(
            doc,
            DocEdit::DeclareVar {
                name: VarName::from_static(name),
                def: editor_core::VarDecl::Free(value.clone()),
            },
        )
        .0;
    }
    let (doc, base) = fixture::insert_pinned(doc, crate::fixture::mated_instance(base_ref), &opts);
    let (doc, top) = insert(doc, fixture::mated_instance(top_ref));
    let (doc, rest) = fixture::mated(
        doc,
        top,
        fixture::rest(
            head_at(top, in_part(top, top_body, CapEnd::Start)),
            head_at(base, in_part(base, base_body, CapEnd::End)),
        ),
        &[
            (Coord::Slide(TargetAxis::X), x),
            (Coord::Slide(TargetAxis::Y), len(1.0)),
            (Coord::SPIN, turn),
        ],
    );
    Scene {
        doc,
        opts,
        top,
        rest,
    }
}

fn set_value(doc: ProfileDoc, name: &'static str, value: FreeValue) -> ProfileDoc {
    step(
        doc,
        DocEdit::SetVarValue {
            var: VarName::from_static(name).into(),
            value,
        },
    )
    .0
}

fn named(name: &'static str, dim: Dimension) -> Formula {
    Formula::named(VarName::from_static(name), dim)
}

/// The solved pose of the top, faults asserted absent.
fn top_pose(s: &Scene, what: &str) -> editor_core::Frame {
    let poses = solve(&s.doc, &s.opts, Tol::witness());
    assert!(
        poses.fault(s.rest.placement).is_none(),
        "{what}: {:?}",
        poses.fault(s.rest.placement)
    );
    poses.relative(s.top).expect("the top solves")
}

/// The bits of a frame, for a bit-for-bit comparison.
fn bits(f: &editor_core::Frame) -> Vec<u64> {
    f.columns
        .iter()
        .flatten()
        .chain(&f.translation)
        .map(|x| x.to_bits())
        .collect()
}

/// **A slide value read off a parameter is the document's nominal
/// parameter, bit for bit.** The slide is a parameter carrying a
/// distribution, so an environment drawn or seeded from it would place
/// the copy somewhere else; the top seated over `s` stands where the
/// same seat written with the literal the document's own nominal
/// environment gives `s` stands, bit for bit, and follows
/// `SetVarValue` — the environment is the document's, not a run's.
///
/// *Breaks if* the fold reads a value in any environment other than
/// the evaluation's nominal one.
#[test]
fn a1_a_slide_value_is_the_documents_nominal_parameter_bit_for_bit() {
    let params = [(
        "s",
        FreeVar::continuous_with(Dimension::Length, 5.0, Distribution::Normal { sigma: 1.0 }),
    )];
    let test = scene(
        "msolve7-a1-slide",
        &params,
        named("s", Dimension::Length),
        Formula::zero(Dimension::Angle),
    );
    // The number through the public expression door against the
    // document's own nominal environment.
    let nominal = editor_core::eval::<f64>(
        &editor_core::Expr::var(
            test.doc.var_named("s").expect("s is declared"),
            Dimension::Length,
        ),
        &test.doc.var_env::<f64>(),
    )
    .unwrap();
    assert_eq!(nominal.to_bits(), 5.0_f64.to_bits());
    let control = scene(
        "msolve7-a1-slide-control",
        &params,
        len(nominal),
        Formula::zero(Dimension::Angle),
    );
    let (t, c) = (top_pose(&test, "s = 5"), top_pose(&control, "5 m"));
    assert_eq!(bits(&t), bits(&c), "the slide is the nominal, bit for bit");
    // The environment is the DOCUMENT's: a new nominal, a new pose.
    let edited = Scene {
        doc: set_value(test.doc.clone(), "s", FreeValue::Continuous(7.5)),
        ..test.clone()
    };
    let t2 = top_pose(&edited, "s = 7.5");
    let control2 = scene(
        "msolve7-a1-slide-control2",
        &params,
        len(7.5),
        Formula::zero(Dimension::Angle),
    );
    assert_eq!(
        bits(&t2),
        bits(&top_pose(&control2, "7.5 m")),
        "the pose follows the edit"
    );
    assert!(
        (t2.translation[0] - t.translation[0] - 2.5).abs() <= 1e-12,
        "the edit moved the top 2.5 m along the slide: {t:?} vs {t2:?}"
    );
    // One document, two solves: bit-identical.
    assert_eq!(bits(&top_pose(&edited, "again")), bits(&t2));
}

/// **A spin value follows the document's angle parameter**: the
/// solved pose is the control turned `θ` about the vertical through
/// the top's origin, at the nominal `θ`, and follows an edit of `θ`.
///
/// *Breaks if* a spin value is read anywhere but the document's
/// nominal environment, or turns about another line.
#[test]
fn a1_a_spin_value_follows_the_documents_angle_parameter() {
    let params = [("th", FreeVar::continuous(Dimension::Angle, 0.7))];
    let control = scene(
        "msolve7-a1-spin-control",
        &params,
        len(1.0),
        Formula::zero(Dimension::Angle),
    );
    let c = top_pose(&control, "control").affine::<f64>();
    let s = scene(
        "msolve7-a1-spin",
        &params,
        len(1.0),
        named("th", Dimension::Angle),
    );
    let check = |doc: ProfileDoc, th: f64, what: &str| -> Affine3<f64> {
        let sc = Scene { doc, ..s.clone() };
        let t = top_pose(&sc, what).affine::<f64>();
        let expected = Affine3::rotation_about_axis(
            Point3::origin() + c.translation,
            Vec3::new(0.0, 0.0, 1.0),
            th,
        ) * c;
        let gap = fixture::seat::map_gap(&t, &expected);
        assert!(gap <= 1e-12, "{what}: gap to R(θ)·control is {gap:e}");
        assert!(
            fixture::seat::map_gap(&t, &c) > 0.1,
            "{what}: the value turned the pose"
        );
        t
    };
    let t1 = check(s.doc.clone(), 0.7, "θ=0.7");
    let t2 = check(
        set_value(s.doc.clone(), "th", FreeValue::Continuous(1.3)),
        1.3,
        "θ=1.3",
    );
    assert!(
        fixture::seat::map_gap(&t1, &t2) > 0.1,
        "the edit moved the pose"
    );
}

// ---- A4: the constraint wire's denial ----

/// **`face`'s plane slid `h` along its own outward normal and
/// flipped**: a target a face rests against at a standoff of `h`.
fn standoff(face: SitedFace, h: f64) -> Operand {
    let plane = fixture::plane(face);
    fixture::flip(Operand::Pose(Box::new(PoseDef::Slide {
        pose: RoleRead::plain(plane.clone()),
        along: RoleRead::through(plane, PoseRole::Normal),
        by: len(h),
    })))
}

/// A document with two instances, the second resting on the first at
/// a standoff and turned by a spin value, saved: the text a stray key
/// is injected into.
fn saved_with_a_bundle(label: &str) -> (ProfileDoc, String) {
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
                node: Box::new(fixture::mated_instance(doc_ref)),
                fresh: Vec::new(),
            },
            Tol::witness(),
        )
        .expect("an instance inserts");
        ids.push(applied.record.minted.expect("a minted id"));
        doc = applied.doc;
    }
    // A part's cap, read at the instance whose copy carries it.
    let name = |node, end| {
        fixture::head_at(
            node,
            StableName {
                kind: EntityKind::Face,
                node: RecipeNodeId::new(0, 1),
                path: vec![RoleSeg::Cap(end)],
            },
        )
    };
    let (doc, _) = fixture::mated(
        doc,
        ids[1],
        fixture::mate(
            fixture::plane(name(ids[1], CapEnd::Start)),
            standoff(name(ids[0], CapEnd::End), 0.5),
        ),
        &[(Coord::SPIN, ang(0.25))],
    );
    let text = save(&doc, &[], Tol::witness()).expect("saves");
    (doc, text)
}

/// The first object under `json` holding every key in `keys`.
fn object_with<'a>(
    json: &'a mut serde_json::Value,
    keys: &[&str],
) -> Option<&'a mut serde_json::Map<String, serde_json::Value>> {
    let holds = match &*json {
        serde_json::Value::Object(map) => keys.iter().all(|k| map.contains_key(*k)),
        _ => false,
    };
    if holds {
        return json.as_object_mut();
    }
    match json {
        serde_json::Value::Object(map) => map.values_mut().find_map(|v| object_with(v, keys)),
        serde_json::Value::Array(items) => items.iter_mut().find_map(|v| object_with(v, keys)),
        _ => None,
    }
}

/// `text` with `key: 2.0` written into the first object holding every
/// key in `keys`.
fn with_a_key_in(text: &str, keys: &[&str], key: &str) -> String {
    wire::doctored(text, |json| {
        object_with(json, keys)
            .unwrap_or_else(|| panic!("the saved document holds an object with {keys:?}"))
            .insert(key.to_owned(), serde_json::json!(2.0));
    })
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

/// The keys of a mate's object and of a value's on the wire.
const MATE_KEYS: [&str; 2] = ["on", "to"];
const VALUE_KEYS: [&str; 2] = ["coord", "by"];

/// **A stray key on a constraint's objects refuses at the load door**,
/// in the format's own arm for a field this build lacks, naming the
/// field: on the mate, on the value, and spelled with a word another
/// object of the wire owns (`coord` is the value's, not the mate's;
/// `on` is the mate's, not the value's), and `class` on the mate.
///
/// *Breaks if* either object drops a field it does not know.
#[test]
fn a4_a_stray_key_on_a_constraint_refuses_at_the_load_door() {
    let (_, text) = saved_with_a_bundle("msolve7-a4-stray");
    for (keys, key) in [
        (&MATE_KEYS[..], "stray"),
        (&VALUE_KEYS[..], "stray"),
        (&MATE_KEYS[..], "coord"),
        (&VALUE_KEYS[..], "on"),
        (&MATE_KEYS[..], "class"),
    ] {
        assert_eq!(
            refused_field(&with_a_key_in(&text, keys, key)).as_deref(),
            Ok(key),
            "`{key}` beside {keys:?}"
        );
    }
}

/// **Every checked-in document still loads under the attribute** —
/// the stop clause's measurement, made rather than assumed. Since
/// INTENT stage 3 D every placement is a bundle of mates and values (a
/// construction stands in the world by a mate, and `die_tool.pncad`
/// places one copy against another), so the corpus exercises the
/// attribute: what it proves is that the bundles checked in load.
///
/// The documents are authored at the default ε, and at another ε row
/// the load door refuses each at its last door (`ToleranceConflict`),
/// after the attribute has read every field. Nothing loads there to
/// count, so that row says it skips the count.
///
/// *Breaks if* a tracked document stops loading, or the corpus stops
/// carrying a bundle for the attribute to reach.
#[test]
fn a4_every_checked_in_document_loads_and_its_bundles_reach_the_attribute() {
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
    let (mut loaded_here, mut bundles) = (0, 0);
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
        loaded_here += 1;
        bundles += fixture::constrained(&loaded.doc).len();
    }
    if loaded_here == 0 {
        println!(
            "SKIPPED at ε {}: no tracked document is authored at this ε, so the load door \
             refuses each at its last door and there is no bundle to count; the default ε \
             row counts them",
            Tol::witness().eps()
        );
        return;
    }
    assert!(
        bundles > 0,
        "the corpus carries bundles the attribute reaches"
    );
}

/// **The same bundle without the key loads**, bit for bit: the
/// attribute denies what is not there and nothing that is.
///
/// *Breaks if* the clean wire refuses, or loads as another bundle.
#[test]
fn a4_the_same_bundle_without_the_key_loads() {
    let (doc, text) = saved_with_a_bundle("msolve7-a4-clean");
    let back = load(&text, Tol::witness()).expect("loads").doc;
    assert!(back.bit_eq(&doc), "the bundle round-trips bit for bit");
    let coords: Vec<Coord> = fixture::constrained(&back)
        .into_iter()
        .flat_map(|p| fixture::bundle(&back, p))
        .filter_map(|c| c.value().map(|v| v.coord))
        .collect();
    assert_eq!(
        coords,
        [
            Coord::SPIN,
            Coord::Slide(TargetAxis::X),
            Coord::Slide(TargetAxis::Y),
        ],
        "the rest's values come back as written: the spin given, then the zeros"
    );
}
