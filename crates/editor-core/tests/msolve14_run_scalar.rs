//! MSOLVE-14 acceptance — **the mate solve runs at the evaluation's own
//! scalar** (`ASSEMBLY.md` A11 (5): the solve's inputs are read at the
//! evaluation's scalar and over its parameters; a pattern's count and a
//! `Part`'s index are read at the nominal).
//!
//! The rows, by the spec's acceptance:
//!
//! - **A3, the `f64` fence**: a mate corpus — authored and face frames,
//!   every primitive, a refusing UNDER and CONTRADICTORY pair, a
//!   patterned bolt, a transform placer, a parametric gauge chain with
//!   a checked offset, a closing loop — solved and evaluated at `f64`,
//!   digested, and pinned to the number the pre-generic tree gives; and
//!   the same corpus at `Dual64` reading the `f64` bits on its value
//!   channel.
//! - **A1**: a face-framed mate on a `Dual64` seed run carries its pose's
//!   tangent, and on an `Interval` box run an enclosure of the pose at
//!   every sampled box corner.
//! - **A2**: a mate read through a parametric placer moves its mated
//!   part in the run that binds the placer — the patterned bolt
//!   (∂B/∂s = −2, ∂copy2/∂s = 0) and a rotating transform.
//! - **A4**: a seeded pass threaded an unseeded `Dual64` prior reuses no
//!   zero-tangent pose (the memo key carries the pose's two channels).
//! - **A5**: `stackup::sensitivities` and certified `clearance` over the
//!   face-framed bolt.
//! - **C5**: one document's structure — roles, roots, faults' presence —
//!   is the same in every lane.
//!
//! Every analysis-lane answer is held to an INDEPENDENT computation:
//! `f64` evaluations at sampled parameter values, and for the bolt the
//! closed form.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::fixture;
use editor_core::ExtrudeSide;

use std::collections::BTreeMap;
use std::sync::Arc;

use editor_core::{
    Alignment, AxisSense, BoxAxis, CancelToken, CapEnd, ContactClass, Dimension, DocEdit, DocParam,
    DocRef, DocumentId, EvalOptions, Evaluation, Expr, MateFrame, MatePrimitive, Node,
    NodeResult, ParamBox, ParamName, PatternKind, Placement, ProfileDoc, ProfileLift,
    ProfileProgram, RecipeNodeId, SitedFace, StableName, Step, ValuePayload, all_vertices,
    evaluate, vertex_position,
};
use fixture::resolver::{PartStore, in_part, with_resolver};
use fixture::{ang, head, head_at, in_copy, insert, len, on_frame, scl, solve, step};
use geom_core::{Dual64, Interval, Tol};

// ---- Substrate ----

/// The slab's side and height, and the bolt's: a `9 x 9 x 1` slab wide
/// enough for three bolts on it, and a `1 x 1 x 3` bolt.
const SLAB_WIDTH: f64 = 9.0;
const SLAB_HEIGHT: f64 = 1.0;
const BOLT_WIDTH: f64 = 1.0;
const BOLT_HEIGHT: f64 = 3.0;

/// The bolt pattern's spacing parameter and its nominal.
const SPACING: f64 = 2.0;
/// The transform placer's lift parameter and its nominal.
const GAP: f64 = 0.5;

fn spacing() -> ParamName {
    ParamName::from_static("s")
}

fn gap() -> ParamName {
    ParamName::from_static("gap")
}

fn lift() -> ParamName {
    ParamName::from_static("lift")
}

fn turn() -> ParamName {
    ParamName::from_static("turn")
}

/// A `w x w x h` block from the origin, as a whole part document, and
/// its body.
fn block(label: &str, w: f64, h: f64) -> (ProfileDoc, RecipeNodeId) {
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
            profile,
            distance: len(h),
            side: ExtrudeSide::Along,
        },
    )
}

/// The two parts every document here instantiates, in one store.
struct Parts {
    store: PartStore,
    slab: DocRef,
    slab_body: RecipeNodeId,
    bolt: DocRef,
    bolt_body: RecipeNodeId,
}

fn parts(label: &str) -> Parts {
    let mut store = PartStore::new();
    let (slab, slab_body) = store.insert_part(
        block(&format!("{label}-slab"), SLAB_WIDTH, SLAB_HEIGHT),
        Tol::witness(),
    );
    let (bolt, bolt_body) = store.insert_part(
        block(&format!("{label}-bolt"), BOLT_WIDTH, BOLT_HEIGHT),
        Tol::witness(),
    );
    Parts {
        store,
        slab,
        slab_body,
        bolt,
        bolt_body,
    }
}

impl Parts {
    fn opts(&self) -> EvalOptions {
        with_resolver(self.store.clone())
    }
    /// The slab's top cap, as `slab` places it.
    fn slab_top(&self, slab: RecipeNodeId) -> StableName {
        in_part(slab, self.slab_body, CapEnd::End)
    }
    /// The bolt's bottom cap, as `bolt` places it.
    fn bolt_foot(&self, bolt: RecipeNodeId) -> StableName {
        in_part(bolt, self.bolt_body, CapEnd::Start)
    }
    /// The bolt's top cap, as `bolt` places it.
    fn bolt_head(&self, bolt: RecipeNodeId) -> StableName {
        in_part(bolt, self.bolt_body, CapEnd::End)
    }
}

fn authored(origin: [f64; 3], axis: [f64; 3]) -> MateFrame {
    MateFrame::authored(origin, axis, [1.0, 0.0, 0.0])
}

/// **A seat**: `mover`'s frame (its own origin, axis down) coincident
/// with `onto`'s, axes opposed — the mover stands ON the other part.
/// `onto_frame` is the frame on the seat's receiving side: authored at
/// a point of its top cap, or the cap face's own pose.
fn seat(mover: SitedFace, onto: SitedFace, onto_frame: MateFrame) -> Node<ProfileProgram> {
    Node::Mate {
        a: mover,
        b: onto,
        class: ContactClass::Rest,
        alignment: Alignment {
            a: authored([0.0, 0.0, 0.0], [0.0, 0.0, -1.0]),
            b: onto_frame,
            primitive: MatePrimitive::FrameCoincidence,
            sense: AxisSense::Opposed,
            clocking: None,
        },
    }
}

/// The receiving frame authored at `(x, y)` of the slab's top.
fn slab_at(x: f64, y: f64) -> MateFrame {
    authored([x, y, SLAB_HEIGHT], [0.0, 0.0, 1.0])
}

fn declare(doc: ProfileDoc, name: ParamName, v: f64, dim: Dimension) -> ProfileDoc {
    step(
        doc,
        DocEdit::SetDocParam {
            name,
            value: DocParam::continuous(dim, v),
        },
    )
    .0
}

fn set_value(doc: ProfileDoc, name: ParamName, v: f64) -> ProfileDoc {
    step(
        doc,
        DocEdit::SetDocParamValue {
            name,
            value: editor_core::DocParamValue::Continuous(v),
        },
    )
    .0
}

fn set_offset(doc: ProfileDoc, instance: RecipeNodeId, offset: Option<Placement>) -> ProfileDoc {
    step(doc, DocEdit::SetOffset { instance, offset }).0
}

fn set_gauge(doc: ProfileDoc, node: RecipeNodeId, gauge: Option<RecipeNodeId>) -> ProfileDoc {
    step(doc, DocEdit::SetGauge { node, gauge }).0
}

/// A slab (the root, at its default offset) and `n` bolts carrying no
/// offset, each placed by whatever mates the caller adds.
fn slab_and_bolts(p: &Parts, label: &str, n: usize) -> (ProfileDoc, RecipeNodeId, Vec<RecipeNodeId>) {
    let doc = ProfileDoc::empty(DocumentId::derive(label), Tol::witness());
    let (mut doc, slab) = insert(doc, Node::instantiate_part(p.slab));
    let mut bolts = Vec::new();
    for _ in 0..n {
        let (next, bolt) = insert(doc, fixture::mated_instance(p.bolt));
        doc = next;
        bolts.push(bolt);
    }
    (doc, slab, bolts)
}

/// Inserts `node` through the parts' own reach — the door a face-framed
/// side's insert needs, since its admission reads the part's face.
fn insert_through(doc: ProfileDoc, node: Node<ProfileProgram>, opts: &EvalOptions) -> (ProfileDoc, RecipeNodeId) {
    let reach = editor_core::mate_reach::<f64>(opts, Tol::witness());
    let (doc, minted) = fixture::step_with(
        doc,
        DocEdit::InsertNode {
            node: Box::new(node),
        },
        &reach,
    );
    (doc, minted.expect("the insert mints"))
}

// ---- The documents the rows share ----

/// **The patterned bolt** (the designers' worked example): a linear
/// pattern of the bolt along `+x` at spacing `s`, three copies, and
/// copy #2 seated on the slab at `(6, 4)` — through `slab_frame`, so
/// the face-framed variant reads the slab's cap pose. Copy #2 stays on
/// the slab for every `s`, so the pattern's master instance sits at
/// `6 − 2s` and moves by `−2` per unit of `s`.
struct Bolted {
    doc: ProfileDoc,
    opts: EvalOptions,
    slab: RecipeNodeId,
    bolt: RecipeNodeId,
    pattern: RecipeNodeId,
    mate: RecipeNodeId,
}

fn bolted(label: &str, slab_frame: MateFrame) -> Bolted {
    let p = parts(label);
    let (doc, slab, bolts) = slab_and_bolts(&p, label, 1);
    let bolt = bolts[0];
    let doc = declare(doc, spacing(), SPACING, Dimension::Length);
    let (doc, pattern) = insert(
        doc,
        Node::Pattern {
            input: bolt,
            count: Expr::count(3),
            kind: PatternKind::Linear {
                direction: [1.0, 0.0, 0.0].map(scl),
                spacing: Expr::param(spacing(), Dimension::Length),
            },
        },
    );
    let opts = p.opts();
    let (doc, mate) = insert_through(
        doc,
        seat(
            head_at(pattern, in_copy(pattern, 2, p.bolt_foot(bolt))),
            head(p.slab_top(slab)),
            slab_frame,
        ),
        &opts,
    );
    Bolted {
        doc,
        opts,
        slab,
        bolt,
        pattern,
        mate,
    }
}

/// The bolt seated through a rotating TRANSFORM placer whose lift is a
/// parameter: the transform lifts the bolt by `gap` and turns it 0.4
/// rad about `x`, and the mate seats the TRANSFORMED bolt on the slab —
/// so the bolt instance itself sits at `T(gap)⁻¹ ∘ seat`.
struct Lifted {
    doc: ProfileDoc,
    opts: EvalOptions,
    bolt: RecipeNodeId,
    transform: RecipeNodeId,
    mate: RecipeNodeId,
}

fn lifted(label: &str) -> Lifted {
    let p = parts(label);
    let (doc, slab, bolts) = slab_and_bolts(&p, label, 1);
    let bolt = bolts[0];
    let doc = declare(doc, gap(), GAP, Dimension::Length);
    let (doc, transform) = insert(
        doc,
        Node::transform(
            bolt,
            Step::Rigid {
                translation: [len(0.0), len(0.0), Expr::param(gap(), Dimension::Length)],
                axis: [1.0, 0.0, 0.0].map(scl),
                angle: ang(0.4),
            },
        ),
    );
    let (doc, mate) = insert(
        doc,
        seat(
            head_at(transform, p.bolt_foot(bolt)),
            head(p.slab_top(slab)),
            slab_at(2.0, 3.0),
        ),
    );
    Lifted {
        doc,
        opts: p.opts(),
        bolt,
        transform,
        mate,
    }
}

// ---- The mate corpus ----

/// **The mate corpus the fence is taken over** — one document per shape
/// the solve has a branch for, each with its resolver. Built through
/// the ordinary doors, so the documents are what an author makes.
fn corpus() -> Vec<(&'static str, ProfileDoc, EvalOptions)> {
    let mut out = Vec::new();

    // An authored opposed seat, and a face-framed one on the slab's cap.
    let p = parts("msolve14-c-seat");
    let (doc, slab, bolts) = slab_and_bolts(&p, "msolve14-c-seat", 2);
    let (doc, _) = insert(
        doc,
        seat(head(p.bolt_foot(bolts[0])), head(p.slab_top(slab)), slab_at(1.0, 1.0)),
    );
    let (doc, _) = insert_through(
        doc,
        seat(
            head(p.bolt_foot(bolts[1])),
            head(p.slab_top(slab)),
            MateFrame::FromFace,
        ),
        &p.opts(),
    );
    out.push(("seats", doc, p.opts()));

    // Coaxial with a clocking rider, cut to DETERMINED by a planar rest:
    // the prismatic-planar row and the translation stage's cosine.
    let p = parts("msolve14-c-coax");
    let (doc, slab, bolts) = slab_and_bolts(&p, "msolve14-c-coax", 1);
    let coax = Node::Mate {
        a: head(p.bolt_foot(bolts[0])),
        b: head(p.slab_top(slab)),
        class: ContactClass::Rest,
        alignment: Alignment {
            a: authored([0.5, 0.5, 0.0], [0.0, 0.0, -1.0]),
            b: authored([4.0, 4.0, SLAB_HEIGHT], [0.0, 0.0, 1.0]),
            primitive: MatePrimitive::Coaxial,
            sense: AxisSense::Opposed,
            clocking: Some(0.3),
        },
    };
    let rest = Node::Mate {
        a: head(p.bolt_foot(bolts[0])),
        b: head(p.slab_top(slab)),
        class: ContactClass::Rest,
        alignment: Alignment {
            a: authored([0.0, 0.0, 0.0], [0.0, 0.0, -1.0]),
            b: authored([0.0, 0.0, SLAB_HEIGHT], [0.0, 0.0, 1.0]),
            primitive: MatePrimitive::PlanarRest { offset: 0.25 },
            sense: AxisSense::Opposed,
            clocking: None,
        },
    };
    let (doc, _) = insert(doc, coax);
    let (doc, _) = insert(doc, rest.clone());
    out.push(("coaxial-clocked-rest", doc, p.opts()));

    // A lone planar rest: UNDER, naming its planar residual.
    let p = parts("msolve14-c-under");
    let (doc, _slab, _bolts) = slab_and_bolts(&p, "msolve14-c-under", 1);
    let rest = match rest {
        Node::Mate {
            class, alignment, ..
        } => Node::Mate {
            a: head(p.bolt_foot(_bolts[0])),
            b: head(p.slab_top(_slab)),
            class,
            alignment,
        },
        _ => unreachable!(),
    };
    let (doc, _) = insert(doc, rest);
    out.push(("under", doc, p.opts()));

    // Two coincidences on one pair at different points: CONTRADICTORY.
    let p = parts("msolve14-c-contra");
    let (doc, slab, bolts) = slab_and_bolts(&p, "msolve14-c-contra", 1);
    let (doc, _) = insert(
        doc,
        seat(head(p.bolt_foot(bolts[0])), head(p.slab_top(slab)), slab_at(1.0, 1.0)),
    );
    let (doc, _) = insert(
        doc,
        seat(head(p.bolt_foot(bolts[0])), head(p.slab_top(slab)), slab_at(2.0, 1.0)),
    );
    out.push(("contradictory", doc, p.opts()));

    // The patterned bolt, authored and face-framed.
    let b = bolted("msolve14-c-bolt", slab_at(6.0, 4.0));
    out.push(("patterned-bolt", b.doc, b.opts));
    let b = bolted("msolve14-c-bolt-face", MateFrame::FromFace);
    out.push(("patterned-bolt-face", b.doc, b.opts));

    // The rotating transform placer.
    let l = lifted("msolve14-c-lift");
    out.push(("transform-placer", l.doc, l.opts));

    // A parametric two-gauge chain, a turned root offset, a transform
    // on the tree mate's chain, and a checked offset on a third member.
    let p = parts("msolve14-c-gauge");
    let doc = ProfileDoc::empty(DocumentId::derive("msolve14-c-gauge"), Tol::witness());
    let doc = declare(doc, lift(), 2.0, Dimension::Length);
    let doc = declare(doc, turn(), 0.25, Dimension::Angle);
    let (doc, g0) = insert(
        doc,
        Node::gauge(
            None,
            Step::Rigid {
                translation: [5.0, 0.0, 0.0].map(len),
                axis: [1.0, 0.0, 0.0].map(scl),
                angle: ang(0.3),
            },
        ),
    );
    let (doc, g1) = insert(
        doc,
        Node::gauge(
            Some(g0),
            Step::Rigid {
                translation: [len(0.0), len(0.0), Expr::param(lift(), Dimension::Length)],
                axis: [0.0, 0.0, 1.0].map(scl),
                angle: Expr::param(turn(), Dimension::Angle),
            },
        ),
    );
    let (doc, slab) = insert(doc, Node::instantiate_part(p.slab));
    let doc = set_gauge(doc, slab, Some(g1));
    let doc = set_offset(
        doc,
        slab,
        Some(Placement::from(Step::Rigid {
            translation: [1.0, 2.0, 0.0].map(len),
            axis: [0.0, 1.0, 0.0].map(scl),
            angle: ang(0.2),
        })),
    );
    let (doc, top) = insert(doc, fixture::mated_instance(p.bolt));
    let doc = set_gauge(doc, top, Some(g1));
    let (doc, t) = insert(
        doc,
        Node::transform(
            top,
            Step::Rigid {
                translation: [0.0, 0.0, 10.0].map(len),
                axis: [1.0, 0.0, 0.0].map(scl),
                angle: ang(0.4),
            },
        ),
    );
    let (doc, _) = insert(
        doc,
        seat(head_at(t, p.bolt_foot(top)), head(p.slab_top(slab)), slab_at(1.0, 1.0)),
    );
    let (doc, third) = insert(doc, fixture::mated_instance(p.bolt));
    let doc = set_gauge(doc, third, Some(g1));
    let (doc, _) = insert(
        doc,
        seat(head(p.bolt_foot(third)), head(p.bolt_head(top)), authored([0.5, 0.5, BOLT_HEIGHT], [0.0, 0.0, 1.0])),
    );
    // The third member states where it sits — a TRUE statement, read
    // off the solve: `G⁻¹ ∘ W`, the gauge frame `G` recovered from the
    // root's world pose and its own offset. A fourth states the
    // identity, which the solve refutes. Both are checked.
    let opts = p.opts();
    let poses = solve(&doc, &opts, Tol::witness());
    let world = |id| poses.placement(&doc, id).expect("posed").affine::<f64>();
    let root_offset = editor_core::Placement::from(Step::Rigid {
        translation: [1.0, 2.0, 0.0].map(len),
        axis: [0.0, 1.0, 0.0].map(scl),
        angle: ang(0.2),
    })
    .eval(&doc.param_env::<f64>(), fixture::band())
    .expect("a literal offset evaluates");
    let stated = root_offset * world(slab).inverse() * world(third);
    let doc = set_offset(
        doc,
        third,
        Some(Placement::literal(&editor_core::Frame::from_affine(stated))),
    );
    let (doc, fourth) = insert(doc, fixture::mated_instance(p.bolt));
    let doc = set_gauge(doc, fourth, Some(g1));
    let (doc, _) = insert(
        doc,
        seat(head(p.bolt_foot(fourth)), head(p.slab_top(slab)), slab_at(7.0, 7.0)),
    );
    let doc = set_offset(
        doc,
        fourth,
        Some(Placement::literal(&editor_core::Frame::IDENTITY)),
    );
    out.push(("gauge-chain", doc, opts));

    // Three bolts in a chain and a CLOSING mate: declaring.
    let p = parts("msolve14-c-loop");
    let (doc, slab, bolts) = slab_and_bolts(&p, "msolve14-c-loop", 2);
    let (doc, _) = insert(
        doc,
        seat(head(p.bolt_foot(bolts[0])), head(p.slab_top(slab)), slab_at(1.0, 1.0)),
    );
    let (doc, _) = insert(
        doc,
        seat(head(p.bolt_foot(bolts[1])), head(p.slab_top(slab)), slab_at(4.0, 1.0)),
    );
    let (doc, _) = insert(
        doc,
        Node::Mate {
            a: head(p.bolt_foot(bolts[1])),
            b: head(p.bolt_foot(bolts[0])),
            class: ContactClass::Rest,
            alignment: Alignment {
                a: authored([0.0, 0.0, 0.0], [0.0, 0.0, -1.0]),
                b: authored([3.0, 0.0, 0.0], [0.0, 0.0, -1.0]),
                primitive: MatePrimitive::PlanarRest { offset: 0.0 },
                sense: AxisSense::Aligned,
                clocking: None,
            },
        },
    );
    out.push(("closing-loop", doc, p.opts()));
    out
}

// ---- The digest ----

/// FNV-1a 64 over bytes.
struct Fnv(u64);

impl Fnv {
    fn new() -> Self {
        Self(0xcbf2_9ce4_8422_2325)
    }
    fn feed(&mut self, bytes: &[u8]) {
        for b in bytes {
            self.0 ^= u64::from(*b);
            self.0 = self.0.wrapping_mul(0x1000_0000_01b3);
        }
    }
    fn frame(&mut self, f: &editor_core::Frame) {
        for x in f.columns.iter().flatten().chain(f.translation.iter()) {
            self.feed(&x.to_bits().to_le_bytes());
        }
    }
}

/// **What the solve answers about a document, at `f64`**: every node in
/// document order with its role, fault, root and space, and every
/// instance's relative pose and world placement by bits (or the
/// refusal, by its `Debug`, whose `f64`s are shortest round-trips) —
/// then the document's evaluation, through the shared evaluation
/// digest, and every node's error.
fn solve_digest(h: &mut Fnv, doc: &ProfileDoc, opts: &EvalOptions) {
    let poses = solve(doc, opts, Tol::witness());
    for &id in doc.order() {
        h.feed(format!("#{id:?}").as_bytes());
        h.feed(format!("role={:?}", poses.role(id)).as_bytes());
        h.feed(format!("fault={:?}", poses.fault(id)).as_bytes());
        h.feed(format!("root={:?}", poses.root(id)).as_bytes());
        h.feed(format!("space={:?}", poses.space(id)).as_bytes());
        if let Some(rel) = poses.relative(id) {
            h.frame(&rel);
        }
        if matches!(doc.node(id), Some(Node::InstantiatePart { .. })) {
            match poses.placement(doc, id) {
                Ok(f) => h.frame(&f),
                Err(e) => h.feed(format!("{e:?}").as_bytes()),
            }
        }
    }
    let ev = run_at::<f64>(doc, opts, None);
    h.feed(&fixture::digest::digest(&ev).to_le_bytes());
    for &id in doc.order() {
        h.feed(format!("{:?}", ev.node_error(id).map(|e| &e.kind)).as_bytes());
    }
}

fn run_at<T: editor_core::EvalScalar>(
    doc: &ProfileDoc,
    opts: &EvalOptions,
    prior: Option<&Evaluation<T>>,
) -> Evaluation<T> {
    evaluate::<T>(doc, prior, &CancelToken::new(), opts, Tol::witness())
}

/// **The corpus digest the pre-generic tree gives, per ε row.** Measured
/// on main's tree at `a8f56a79f` (this branch's dispatch, before any
/// solve code moved) from this same file, and held since: the generic
/// solve at `f64` is the nominal solve, bit for bit.
///
/// One number per row because the documents are not ε-free: a part's
/// content pin hashes its recorded ε, and an instance's id hashes the
/// pin, so every id the digest feeds moves with the row. The rows are
/// the three the hosted matrix runs; any other ε has no measurement and
/// the row fails rather than pass on nothing.
const MAIN_CORPUS_DIGEST: [(f64, u64); 3] = [
    (1e-9, 0x10fd_6343_6e24_18c3),
    (1e-6, 0x5e76_b591_d041_e048),
    (1e-12, 0x76b0_785f_5377_979d),
];

/// **A3, the `f64` fence**: the corpus's solved poses, roles, faults and
/// evaluations are byte-identical to the pre-generic tree's.
#[test]
fn a3_the_f64_solve_is_mains_bit_for_bit_on_the_mate_corpus() {
    let eps = Tol::witness().eps();
    let Some(&(_, main)) = MAIN_CORPUS_DIGEST
        .iter()
        .find(|(e, _)| e.to_bits() == eps.to_bits())
    else {
        panic!("no main digest was measured at ε = {eps:e}; measure one on main's tree")
    };
    let mut h = Fnv::new();
    for (label, doc, opts) in corpus() {
        h.feed(label.as_bytes());
        solve_digest(&mut h, &doc, &opts);
    }
    assert_eq!(
        h.0, main,
        "the f64 solve moved on the mate corpus at ε = {eps:e}: {:#018x}",
        h.0
    );
}
