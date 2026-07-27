//! M4 PR 6 spec D6.1 — the save/load/replay-identity CI row.
//!
//! Corpus documents round-trip BIT-identically (snapshot, edit log,
//! canonical bytes) AND replay to bit-identical evaluations — name
//! tables, bodies (arena floats via Debug's shortest-round-trip
//! encoding, which is bit-faithful for the finite values documents
//! carry), verdict logs, content keys. The CI matrix runs this file
//! at ε ∈ {1e-6, 1e-9, 1e-12} and under the interval feature
//! (`m4_pr6_roundtrip_interval` wraps the same corpus).
//!
//! D2 lives in `m4_pr6_floats.rs` (the bit-level float property row).

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

mod fixture;

use editor_core::{
    Attr, AttrKind, BooleanOp, CancelToken, Datum, DocEdit, DocParam, EntityKind, EvalOptions,
    Expr, MetaValue, Node, ParamName, PatternKind, ProfileDesc, ProfileDoc, RecipeNodeId, Rgba8,
    RoleSeg, StableName, WitnessDatum, apply, evaluate, load, save,
};
use fixture::{ang, declare_x_offset_flush, desc, die, len, scl};

/// Applies-and-records: the corpus's edit-log author (snapshot =
/// empty, edits = everything — load must replay the whole document).
struct Recorder {
    doc: ProfileDoc,
    edits: Vec<DocEdit<ProfileDesc>>,
}

impl Recorder {
    fn new() -> Self {
        Self {
            doc: ProfileDoc::empty(),
            edits: Vec::new(),
        }
    }
    fn push(&mut self, edit: DocEdit<ProfileDesc>) -> Option<RecipeNodeId> {
        let applied = apply(&self.doc, &edit).expect("corpus edit must apply");
        self.doc = applied.doc;
        self.edits.push(edit);
        applied.record.minted
    }
    fn insert(&mut self, node: Node<ProfileDesc>) -> RecipeNodeId {
        self.push(DocEdit::InsertNode { node }).expect("minted id")
    }
}

/// The kitchen-sink document: every v1 node kind, expressions with
/// params/trig, a Declare-consuming boolean, witnesses, appearance
/// attrs + D7 metadata — authored as a full edit LOG.
fn kitchen_sink() -> Recorder {
    let mut r = Recorder::new();
    r.push(DocEdit::SetDocParam {
        name: ParamName::new("h"),
        value: DocParam::Continuous {
            dim: editor_core::Dimension::Length,
            value: 1.25,
        },
    });
    r.push(DocEdit::SetDocParam {
        name: ParamName::new("n"),
        value: DocParam::Count { value: 3 },
    });
    // Datums: a point (inert), an axis for revolve/circular pattern.
    r.insert(Node::Datum(Datum::Point {
        position: [len(0.5), len(-0.25), len(0.0)],
    }));
    let axis = r.insert(Node::Datum(Datum::Axis {
        origin: [len(-2.0), len(0.0), len(0.0)],
        direction: [scl(0.0), scl(0.0), scl(1.0)],
    }));
    // A profile extruded by h·sin(π/2) (param + trig coverage).
    let profile = r.insert(Node::Profile(desc(
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]],
    )));
    let h = Expr::param(ParamName::new("h"), editor_core::Dimension::Length);
    let dist = Expr::mul(h, Expr::sin(ang(std::f64::consts::FRAC_PI_2)).unwrap()).unwrap();
    let block_a = r.insert(Node::Extrude {
        profile,
        distance: dist,
    });
    // A flush neighbor + Declare + the consuming union (F5 coverage).
    let profile_b = r.insert(Node::Profile(desc(
        [1.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)]],
    )));
    let block_b = r.insert(Node::Extrude {
        profile: profile_b,
        distance: len(1.25),
    });
    let (doc2, declare) = declare_x_offset_flush(r.doc.clone(), block_a, block_b);
    // Re-record the fixture's insert as a log entry (same node data).
    let declare_node = doc2.node(declare).expect("declare inserted").clone();
    r.insert(declare_node);
    let union = r.insert(Node::Boolean {
        op: BooleanOp::Union,
        a: block_a,
        b: block_b,
        declare: Some(declare),
    });
    // Split the union with a plane tool.
    let tool = r.insert(Node::Datum(Datum::Plane {
        origin: [len(0.0), len(0.0), len(0.625)],
        normal: [scl(0.0), scl(0.0), scl(1.0)],
    }));
    r.insert(Node::Split {
        target: union,
        tool,
    });
    // A transformed copy, patterned linearly and circularly.
    let moved = r.insert(Node::Transform {
        input: union,
        translation: [len(0.0), len(4.0), len(0.0)],
        rotation_axis: [scl(0.0), scl(0.0), scl(1.0)],
        rotation_angle: ang(std::f64::consts::FRAC_PI_3),
    });
    r.insert(Node::Pattern {
        input: moved,
        count: Expr::param(ParamName::new("n"), editor_core::Dimension::Count),
        kind: PatternKind::Linear {
            direction: [scl(1.0), scl(0.0), scl(0.0)],
            spacing: len(3.0),
        },
    });
    let lone = r.insert(Node::Extrude {
        profile,
        distance: len(0.5),
    });
    r.insert(Node::Pattern {
        input: lone,
        count: Expr::count(2),
        kind: PatternKind::Circular {
            axis,
            step: ang(std::f64::consts::PI),
        },
    });
    // A revolve off the shared axis.
    let rev_profile = r.insert(Node::Profile(desc(
        [-2.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0],
        vec![vec![(1.0, 0.0), (2.0, 0.0), (2.0, 1.0), (1.0, 1.0)]],
    )));
    r.insert(Node::Revolve {
        profile: rev_profile,
        axis,
        angle: ang(std::f64::consts::FRAC_PI_2),
    });
    // #101 declared tangency (M4 PR 6 fix pass, MINOR-4): a
    // fillet-CONSTRUCTED loop (tangent joints declared by
    // construction) and a hand-declared collinear tangency — both
    // must survive save/load/replay bit-identically, joints included.
    let fillet_loop = profile::ProfileLoop::builder(geom_core::Point2::new(0.0, 0.0))
        .line_to(geom_core::Point2::new(3.0, 0.0))
        .line_to(geom_core::Point2::new(3.0, 1.0))
        .fillet(
            geom_core::Point2::new(1.0, 1.0),
            geom_core::Point2::new(1.0, 3.0),
            0.5,
        )
        .expect("fillet fits")
        .line_to(geom_core::Point2::new(1.0, 3.0))
        .line_to(geom_core::Point2::new(0.0, 3.0))
        .close();
    let fillet_profile = r.insert(Node::Profile(ProfileDesc(profile::Profile::new(
        profile::SketchPlane::xy(),
        vec![fillet_loop],
    ))));
    r.insert(Node::Extrude {
        profile: fillet_profile,
        distance: len(0.4),
    });
    let mut collinear = profile::ProfileLoop::new(
        [(0.0, 0.0), (1.0, 0.0), (2.0, 0.0), (2.0, 2.0), (0.0, 2.0)]
            .into_iter()
            .map(|(x, y)| profile::ProfileVertex {
                pos: geom_core::Point2::new(x, y),
                bulge: 0.0,
            })
            .collect(),
    );
    // The joint at vertex 1 sits between two collinear straights:
    // definite tangency, and #101 requires it DECLARED to validate.
    collinear.tangent_joints = vec![1];
    let tangent_profile = r.insert(Node::Profile(ProfileDesc(profile::Profile::new(
        profile::SketchPlane::xy(),
        vec![collinear],
    ))));
    r.insert(Node::Extrude {
        profile: tangent_profile,
        distance: len(0.3),
    });
    // Witness bytes (opaque, hex on the wire — bit-exact).
    r.push(DocEdit::ReWitness {
        node: profile,
        witness: WitnessDatum {
            schema: 1,
            bytes: vec![0x00, 0x01, 0x7f, 0x80, 0xfe, 0xff],
        },
    });
    // Appearance: attrs + D7 metadata on the union's output body.
    let body = StableName {
        kind: EntityKind::Body,
        node: union,
        path: vec![RoleSeg::OutputBody],
    };
    r.push(DocEdit::SetAppearance {
        name: body.clone(),
        attr: Attr::Color(Rgba8::opaque(200, 40, 40)),
    });
    r.push(DocEdit::SetAppearance {
        name: body.clone(),
        attr: Attr::Label("kitchen sink".into()),
    });
    r.push(DocEdit::SetAppearanceMeta {
        name: body.clone(),
        key: "tool.example/annotation".into(),
        value: meta_tree(),
    });
    r.push(DocEdit::ClearAppearance {
        name: body,
        kind: AttrKind::Label,
    });
    r
}

/// A D7 value exercising the whole MetaValue vocabulary (with the
/// required "v" version field), floats included (-0.0 is DATA).
fn meta_tree() -> MetaValue {
    let mut m = std::collections::BTreeMap::new();
    m.insert("v".into(), MetaValue::Int(1));
    m.insert("flag".into(), MetaValue::Bool(true));
    m.insert("nothing".into(), MetaValue::Null);
    m.insert("neg_zero".into(), MetaValue::Float(-0.0));
    m.insert("subnormal".into(), MetaValue::Float(f64::from_bits(1)));
    m.insert("text".into(), MetaValue::Str("π ≈ 3.14159".into()));
    m.insert("blob".into(), MetaValue::Bytes(vec![0xde, 0xad, 0x00]));
    m.insert(
        "list".into(),
        MetaValue::List(vec![MetaValue::Int(-7), MetaValue::Float(0.1)]),
    );
    MetaValue::Map(m)
}

/// The whole-evaluation bit fingerprint: Debug of every node result
/// (payload arenas, name tables, verdicts, content keys) plus the
/// appearance resolution. Debug prints floats shortest-round-trip
/// (bit-faithful for finite values, sign of zero included).
fn eval_fingerprint(doc: &ProfileDoc) -> String {
    let ev = evaluate::<f64>(doc, None, &CancelToken::new(), &EvalOptions::default());
    format!("{:?}|{:?}|{:?}", ev.order, ev.nodes, ev.appearance)
}

/// The corpus: (label, snapshot, edit log).
fn corpus() -> Vec<(&'static str, ProfileDoc, Vec<DocEdit<ProfileDesc>>)> {
    let ks = kitchen_sink();
    vec![
        ("die_snapshot", die().doc, Vec::new()),
        ("kitchen_sink_log", ProfileDoc::empty(), ks.edits.clone()),
        ("kitchen_sink_snapshot", ks.doc, Vec::new()),
    ]
}

#[test]
fn save_load_replay_identity() {
    for (label, snapshot, edits) in corpus() {
        // The expected current state: snapshot + edits.
        let mut expected = snapshot.clone();
        for edit in &edits {
            expected = apply(&expected, edit).expect("corpus edit").doc;
        }
        let text = save(&snapshot, &edits).expect("save");
        let loaded = load(&text).unwrap_or_else(|e| panic!("{label}: load refused: {e:?}"));
        assert!(
            loaded.snapshot.bit_eq(&snapshot),
            "{label}: snapshot round-trip not bit-identical"
        );
        assert_eq!(loaded.edits, edits, "{label}: edit log round-trip");
        assert!(
            loaded.doc.bit_eq(&expected),
            "{label}: replayed document not bit-identical"
        );
        // Canonical bytes: re-saving the loaded state reproduces the
        // file exactly (BTreeMap ordering + ryu canonical floats).
        let resaved = save(&loaded.snapshot, &loaded.edits).expect("re-save");
        assert_eq!(resaved, text, "{label}: save bytes not canonical");
        // Replay identity: evaluation of the loaded document is
        // bit-identical — name tables, bodies, verdicts, keys.
        assert_eq!(
            eval_fingerprint(&expected),
            eval_fingerprint(&loaded.doc),
            "{label}: evaluation fingerprint drifted across save/load"
        );
    }
}

#[test]
fn loaded_metadata_round_trips_structurally() {
    // Pass-through interop (D7): the loader reproduces the exact
    // canonical tree, bits and all, without interpreting it.
    let ks = kitchen_sink();
    let text = save(&ks.doc, &[]).expect("save");
    let loaded = load(&text).expect("load");
    let body = ks
        .doc
        .appearance()
        .keys()
        .next()
        .expect("one attributed name")
        .clone();
    let rec = loaded.doc.appearance_of(&body).expect("record survived");
    assert_eq!(rec.metadata["tool.example/annotation"], meta_tree());
}
