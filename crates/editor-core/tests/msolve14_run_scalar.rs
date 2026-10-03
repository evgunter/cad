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
    DocRef, DocumentId, EvalOptions, Evaluation, Expr, MateFrame, MatePrimitive, Node, NodeResult,
    ParamBox, ParamName, PatternKind, Placement, ProfileDoc, ProfileLift, ProfileProgram,
    RecipeNodeId, SitedFace, StableName, Step, ValuePayload, all_vertices, evaluate,
    vertex_position,
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
fn slab_and_bolts(
    p: &Parts,
    label: &str,
    n: usize,
) -> (ProfileDoc, RecipeNodeId, Vec<RecipeNodeId>) {
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
fn insert_through(
    doc: ProfileDoc,
    node: Node<ProfileProgram>,
    opts: &EvalOptions,
) -> (ProfileDoc, RecipeNodeId) {
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
        seat(
            head(p.bolt_foot(bolts[0])),
            head(p.slab_top(slab)),
            slab_at(1.0, 1.0),
        ),
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
        seat(
            head(p.bolt_foot(bolts[0])),
            head(p.slab_top(slab)),
            slab_at(1.0, 1.0),
        ),
    );
    let (doc, _) = insert(
        doc,
        seat(
            head(p.bolt_foot(bolts[0])),
            head(p.slab_top(slab)),
            slab_at(2.0, 1.0),
        ),
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
        seat(
            head_at(t, p.bolt_foot(top)),
            head(p.slab_top(slab)),
            slab_at(1.0, 1.0),
        ),
    );
    let (doc, third) = insert(doc, fixture::mated_instance(p.bolt));
    let doc = set_gauge(doc, third, Some(g1));
    let (doc, _) = insert(
        doc,
        seat(
            head(p.bolt_foot(third)),
            head(p.bolt_head(top)),
            authored([0.5, 0.5, BOLT_HEIGHT], [0.0, 0.0, 1.0]),
        ),
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
        seat(
            head(p.bolt_foot(fourth)),
            head(p.slab_top(slab)),
            slab_at(7.0, 7.0),
        ),
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
        seat(
            head(p.bolt_foot(bolts[0])),
            head(p.slab_top(slab)),
            slab_at(1.0, 1.0),
        ),
    );
    let (doc, _) = insert(
        doc,
        seat(
            head(p.bolt_foot(bolts[1])),
            head(p.slab_top(slab)),
            slab_at(4.0, 1.0),
        ),
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

    // Two pins on parallel axes and a rest: the fold meets two
    // rotation constraints about parallel axes and SOLVES the free
    // clocking between them (`clocking_about`'s angle), then the rest
    // cuts the prismatic residual to DETERMINED.
    let p = parts("msolve14-c-two-pin");
    let (doc, slab, bolts) = slab_and_bolts(&p, "msolve14-c-two-pin", 1);
    let pin = |a: [f64; 3], b: [f64; 3]| Node::Mate {
        a: head(p.bolt_foot(bolts[0])),
        b: head(p.slab_top(slab)),
        class: ContactClass::Rest,
        alignment: Alignment {
            a: MateFrame::authored(a, [0.0, 0.0, -1.0], [0.6, 0.8, 0.0]),
            b: MateFrame::authored(b, [0.0, 0.0, 1.0], [1.0, 0.0, 0.0]),
            primitive: MatePrimitive::Coaxial,
            sense: AxisSense::Opposed,
            clocking: None,
        },
    };
    let (doc, _) = insert(doc, pin([0.25, 0.25, 0.0], [3.0, 2.0, SLAB_HEIGHT]));
    let (doc, _) = insert(doc, pin([0.75, 0.5, 0.0], [3.5, 2.25, SLAB_HEIGHT]));
    let (doc, _) = insert(
        doc,
        Node::Mate {
            a: head(p.bolt_foot(bolts[0])),
            b: head(p.slab_top(slab)),
            class: ContactClass::Rest,
            alignment: Alignment {
                a: authored([0.0, 0.0, 0.0], [0.0, 0.0, -1.0]),
                b: authored([0.0, 0.0, SLAB_HEIGHT], [0.0, 0.0, 1.0]),
                primitive: MatePrimitive::PlanarRest { offset: 0.0 },
                sense: AxisSense::Opposed,
                clocking: None,
            },
        },
    );
    out.push(("two-pin", doc, p.opts()));

    // A pin across a rest's normal: two rotation constraints about
    // DIFFERENT axes, met through the reachability predicate and its
    // angle (`candidate_rotation`'s two-axis arm); UNDER, naming the
    // prismatic residual along the pin.
    let p = parts("msolve14-c-cross-pin");
    let (doc, slab, bolts) = slab_and_bolts(&p, "msolve14-c-cross-pin", 1);
    let (doc, _) = insert(
        doc,
        Node::Mate {
            a: head(p.bolt_foot(bolts[0])),
            b: head(p.slab_top(slab)),
            class: ContactClass::Rest,
            alignment: Alignment {
                a: authored([0.0, 0.0, 0.0], [0.0, 0.0, -1.0]),
                b: authored([0.0, 0.0, SLAB_HEIGHT], [0.0, 0.0, 1.0]),
                primitive: MatePrimitive::PlanarRest { offset: 0.0 },
                sense: AxisSense::Opposed,
                clocking: None,
            },
        },
    );
    let (doc, _) = insert(
        doc,
        Node::Mate {
            a: head(p.bolt_foot(bolts[0])),
            b: head(p.slab_top(slab)),
            class: ContactClass::Rest,
            alignment: Alignment {
                a: MateFrame::authored([0.5, 0.0, 0.5], [0.3, 1.0, 0.0], [0.0, 0.0, 1.0]),
                b: MateFrame::authored([2.0, 3.0, 1.5], [1.0, 0.2, 0.0], [0.0, 0.0, 1.0]),
                primitive: MatePrimitive::Coaxial,
                sense: AxisSense::Aligned,
                clocking: None,
            },
        },
    );
    out.push(("cross-pin", doc, p.opts()));
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
    (1e-9, 0x9699_0e48_61c0_f37c),
    (1e-6, 0x8f98_680c_6689_9959),
    (1e-12, 0xd74c_2d37_24c3_f3db),
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

// ---- Reading a lane's answer, and the independent one ----

/// Every vertex of `node`'s body, by name, at the lane's scalar.
fn vertices<T: geom_core::Decide>(
    ev: &Evaluation<T>,
    node: RecipeNodeId,
) -> Vec<(StableName, geom_core::Point3<T>)> {
    let names = all_vertices(ev, node);
    assert!(!names.is_empty(), "node {node:?} has vertices");
    names
        .into_iter()
        .map(|n| {
            let p = vertex_position(ev, node, &n)
                .unwrap_or_else(|e| panic!("vertex {n:?} of {node:?} reads: {e:?}"));
            (n, p)
        })
        .collect()
}

/// Every vertex of `node`, by name, in an `f64` evaluation of `doc` with
/// `param` bound at `value` — the independent computation a lane's
/// answer is held to.
fn f64_vertices_at(
    doc: &ProfileDoc,
    opts: &EvalOptions,
    node: RecipeNodeId,
    param: ParamName,
    value: f64,
) -> BTreeMap<StableName, [f64; 3]> {
    let doc = set_value(doc.clone(), param, value);
    let ev = run_at::<f64>(&doc, opts, None);
    assert!(
        ev.node_error(node).is_none(),
        "the f64 build at {value} evaluates node {node:?}: {:?}",
        ev.node_error(node)
    );
    vertices(&ev, node)
        .into_iter()
        .map(|(n, p)| (n, [p.x, p.y, p.z]))
        .collect()
}

/// The copy of a pattern a vertex of the pattern's value belongs to.
fn copy_of(name: &StableName) -> u32 {
    match name.path.first() {
        Some(editor_core::RoleSeg::Instance { i, .. }) => *i,
        other => panic!("a pattern's vertex is copy-wrapped, got {other:?}"),
    }
}

fn seeded(opts: &EvalOptions, param: ParamName) -> EvalOptions {
    EvalOptions {
        seed: Some(param),
        profile_lift: ProfileLift::Guided,
        ..opts.clone()
    }
}

fn boxed(opts: &EvalOptions, param: ParamName, lo: f64, hi: f64) -> EvalOptions {
    EvalOptions {
        param_box: Some(Arc::new(ParamBox::from_axes(BTreeMap::from([(
            param,
            BoxAxis::Varying { lo, hi },
        )])))),
        profile_lift: ProfileLift::Guided,
        ..opts.clone()
    }
}

/// **A seed run's tangent at every vertex of `node` is the central
/// difference of two `f64` builds** at `nominal ± h` — exact up to
/// rounding where the motion is affine in the parameter, which every
/// row here is (a translation, or a rotation the parameter does not
/// turn). The value channel is the `f64` nominal build's bits.
/// Returns each vertex's tangent.
fn assert_tangents_match(
    doc: &ProfileDoc,
    opts: &EvalOptions,
    ev: &Evaluation<Dual64>,
    node: RecipeNodeId,
    param: ParamName,
    nominal: f64,
    what: &str,
) -> Vec<(StableName, [f64; 3])> {
    let h = 1e-3;
    let at = f64_vertices_at(doc, opts, node, param.clone(), nominal);
    let up = f64_vertices_at(doc, opts, node, param.clone(), nominal + h);
    let down = f64_vertices_at(doc, opts, node, param, nominal - h);
    let mut out = Vec::new();
    for (name, p) in vertices(ev, node) {
        let value = [p.x.value, p.y.value, p.z.value];
        let tangent = [p.x.deriv, p.y.deriv, p.z.deriv];
        assert_eq!(
            value.map(f64::to_bits),
            at[&name].map(f64::to_bits),
            "{what}: the value channel of {name:?} is the f64 build's"
        );
        for k in 0..3 {
            let fd = (up[&name][k] - down[&name][k]) / (2.0 * h);
            assert!(
                (tangent[k] - fd).abs() <= 1e-9,
                "{what}: ∂/∂param of {name:?}[{k}] is {} where the f64 builds give {fd}",
                tangent[k]
            );
        }
        out.push((name, tangent));
    }
    out
}

/// **A box run's enclosure of every vertex of `node` contains that
/// vertex in the `f64` builds at the box's two corners and its
/// nominal**, and the enclosure is wider than rounding where the
/// vertex moves.
fn assert_encloses_corners(
    doc: &ProfileDoc,
    opts: &EvalOptions,
    ev: &Evaluation<Interval>,
    node: RecipeNodeId,
    (param, nominal): (ParamName, f64),
    (lo, hi): (f64, f64),
    what: &str,
) {
    use geom_core::Bounds;
    let samples = [nominal + lo, nominal, nominal + hi]
        .map(|v| f64_vertices_at(doc, opts, node, param.clone(), v));
    for (name, p) in vertices(ev, node) {
        for (k, x) in [p.x, p.y, p.z].into_iter().enumerate() {
            for (s, at) in samples.iter().enumerate() {
                let truth = at[&name][k];
                assert!(
                    x.lo() <= truth && truth <= x.hi(),
                    "{what}: the enclosure [{}, {}] of {name:?}[{k}] omits {truth}, its f64 \
                     position at sample {s}",
                    x.lo(),
                    x.hi()
                );
            }
        }
    }
}

/// **A box narrow enough for the placement door**: `±w·ε` about the
/// nominal. An instance's body is placed through `topo`'s rigid
/// transform, which certifies each placed edge on its placed carrier,
/// and a box-wide translation widens those two independently past the
/// band — a refusal every boxed placer meets today
/// (`work/topo/a-boxed-rotation-refuses-not-rigid-at-every-placer.md`,
/// its "Measured" section). A box a fraction of ε wide is placed, and
/// the rows that read the BODY read it over one; the rows that read
/// the SOLVE read it over a wide one ([`assert_pose_encloses_corners`]).
fn narrow(w: f64) -> (f64, f64) {
    let eps = Tol::witness().eps();
    (-w * eps, w * eps)
}

/// **The solve's own pose of `instance` over a box encloses its `f64`
/// pose at both corners and the nominal** — read at the solve, through
/// [`editor_core::mate::solve_document_at`] at `Interval` over the box's
/// own environment, and held to [`solve`] at `f64` on the document with
/// the parameter bound at each sample: the solve read where no placement
/// door stands between it and the row. Returns the enclosure's widest
/// translation component.
fn assert_pose_encloses_corners(
    doc: &ProfileDoc,
    opts: &EvalOptions,
    instance: RecipeNodeId,
    param: ParamName,
    nominal: f64,
    (lo, hi): (f64, f64),
    what: &str,
) -> f64 {
    use geom_core::Bounds;
    let pbox = ParamBox::from_axes(BTreeMap::from([(
        param.clone(),
        BoxAxis::Varying { lo, hi },
    )]));
    let env = editor_core::param_env_over::<Interval, _>(doc, &pbox).expect("the box binds");
    let reach = editor_core::mate_reach::<Interval>(opts, Tol::witness());
    let poses = editor_core::mate::solve_document_at(doc, &env, &reach, Tol::witness());
    assert!(
        poses.fault(instance).is_none(),
        "{what}: the solve over the box poses {instance:?}: {:?}",
        poses.fault(instance)
    );
    let map = poses.relative_map(instance).expect("posed over the box");
    let entries = |m: &geom_core::Affine3<Interval>| {
        [m.linear.c0, m.linear.c1, m.linear.c2, m.translation]
            .into_iter()
            .flat_map(|c| [c.x, c.y, c.z])
            .collect::<Vec<_>>()
    };
    let got = entries(&map);
    for v in [nominal + lo, nominal, nominal + hi] {
        let at = set_value(doc.clone(), param.clone(), v);
        let truth = solve(&at, opts, Tol::witness())
            .relative(instance)
            .expect("posed at f64")
            .affine::<f64>();
        let truth = [
            truth.linear.c0,
            truth.linear.c1,
            truth.linear.c2,
            truth.translation,
        ]
        .into_iter()
        .flat_map(|c| [c.x, c.y, c.z]);
        for (k, (x, t)) in got.iter().zip(truth).enumerate() {
            assert!(
                x.lo() <= t && t <= x.hi(),
                "{what}: entry {k} of the pose over the box, [{}, {}], omits {t}, its f64 \
                 pose at {v}",
                x.lo(),
                x.hi()
            );
        }
    }
    let t = map.translation;
    [t.x, t.y, t.z]
        .into_iter()
        .map(|x| x.hi() - x.lo())
        .fold(0.0, f64::max)
}

// ---- A2: a parametric placer moves its mated part in the run that binds it ----

/// **The patterned bolt, seeded on its spacing** (the designers' worked
/// example): the bolt instance — the pattern's master — moves by
/// `∂B/∂s = −2` along `x`, and copy #2, which the mate holds on the
/// slab, by `∂copy2/∂s = 0`; copy #1 by `−1`. Every vertex's tangent is
/// the central difference of two `f64` builds, and the value channel is
/// the nominal build's bits.
#[test]
fn a2_a_seed_on_the_bolts_spacing_moves_the_bolt_by_minus_two_and_holds_copy_two() {
    let b = bolted("msolve14-a2-seed", slab_at(6.0, 4.0));
    let ev = run_at::<Dual64>(&b.doc, &seeded(&b.opts, spacing()), None);
    assert!(
        ev.node_error(b.mate).is_none(),
        "{:?}",
        ev.node_error(b.mate)
    );
    let bolt = assert_tangents_match(
        &b.doc,
        &b.opts,
        &ev,
        b.bolt,
        spacing(),
        SPACING,
        "the bolt instance",
    );
    for (name, t) in &bolt {
        assert_eq!(t, &[-2.0, 0.0, 0.0], "∂B/∂s at {name:?}");
    }
    let copies = assert_tangents_match(
        &b.doc,
        &b.opts,
        &ev,
        b.pattern,
        spacing(),
        SPACING,
        "the pattern",
    );
    for (name, t) in &copies {
        let want = [-2.0, -1.0, 0.0][copy_of(name) as usize];
        assert_eq!(t, &[want, 0.0, 0.0], "∂copy/∂s at {name:?}");
    }
    assert!(
        copies.iter().any(|(n, _)| copy_of(n) == 2),
        "copy #2 is among the pattern's vertices"
    );
}

/// **The patterned bolt over a box on its spacing.** At the solve, over
/// a wide box: the bolt's pose encloses its `f64` pose at both corners
/// and the nominal, and is twice the box wide along `x` — not the
/// nominal's point. Through the evaluation, over a box narrow enough
/// for the placement door ([`narrow`]): every vertex of the bolt and of
/// the pattern is enclosed at both corners and the nominal.
#[test]
fn a2_a_box_on_the_bolts_spacing_encloses_the_bolt_at_every_corner() {
    let b = bolted("msolve14-a2-box", slab_at(6.0, 4.0));
    let (lo, hi) = (-0.25, 0.25);
    let width = assert_pose_encloses_corners(
        &b.doc,
        &b.opts,
        b.bolt,
        spacing(),
        SPACING,
        (lo, hi),
        "the bolted box, at the solve",
    );
    assert!(
        width >= 2.0 * (hi - lo),
        "the bolt's pose spans the box twice over: {width}"
    );
    let (lo, hi) = narrow(0.05);
    let ev = run_at::<Interval>(&b.doc, &boxed(&b.opts, spacing(), lo, hi), None);
    assert!(
        ev.node_error(b.mate).is_none(),
        "{:?}",
        ev.node_error(b.mate)
    );
    for node in [b.bolt, b.pattern] {
        assert_encloses_corners(
            &b.doc,
            &b.opts,
            &ev,
            node,
            (spacing(), SPACING),
            (lo, hi),
            "the bolted box, evaluated",
        );
    }
}

/// **Over a wide box the solve places the bolt, and the instance meets
/// the placement door's refusal** — the residual every boxed placer
/// meets today (a `Transform` node with a boxed translation refuses the
/// same way): the mate evaluates, and the bolt's own refusal is the
/// rigid transform's certification, not a mate fault.
#[test]
fn a2_over_a_wide_box_the_bolts_refusal_is_the_placement_doors() {
    let b = bolted("msolve14-a2-wide", slab_at(6.0, 4.0));
    let ev = run_at::<Interval>(&b.doc, &boxed(&b.opts, spacing(), -0.25, 0.25), None);
    assert!(
        ev.node_error(b.mate).is_none(),
        "{:?}",
        ev.node_error(b.mate)
    );
    let kind = &ev
        .node_error(b.bolt)
        .expect("the placement door refuses")
        .kind;
    assert!(
        matches!(kind, editor_core::NodeErrorKind::Transform(_)),
        "the bolt refuses at the rigid transform: {kind:?}"
    );
}

/// **A rotating transform placer whose lift is seeded**: the bolt
/// instance moves by `−R⁻¹ e_z` per unit of `gap` (the transform turns
/// 0.4 rad about `x` after lifting), and every vertex's tangent is the
/// `f64` builds' central difference.
#[test]
fn a2_a_seed_on_a_transform_placers_lift_moves_the_mated_part() {
    let l = lifted("msolve14-a2-lift-seed");
    let ev = run_at::<Dual64>(&l.doc, &seeded(&l.opts, gap()), None);
    assert!(
        ev.node_error(l.mate).is_none(),
        "{:?}",
        ev.node_error(l.mate)
    );
    let tangents =
        assert_tangents_match(&l.doc, &l.opts, &ev, l.bolt, gap(), GAP, "the lifted bolt");
    let (s, c) = 0.4_f64.sin_cos();
    for (name, t) in tangents {
        let want = [0.0, -s, -c];
        assert!(
            (0..3).all(|k| (t[k] - want[k]).abs() <= 1e-12),
            "∂B/∂gap at {name:?} is {t:?}, want {want:?}"
        );
    }
    // The transformed body is what the mate holds: it does not move.
    let held = assert_tangents_match(
        &l.doc,
        &l.opts,
        &ev,
        l.transform,
        gap(),
        GAP,
        "the transformed bolt",
    );
    for (name, t) in held {
        assert!(
            t.iter().all(|x| x.abs() <= 1e-12),
            "the seated body at {name:?} holds still: {t:?}"
        );
    }
}

/// **The same transform placer over a box on its lift**: the bolt's
/// pose encloses its `f64` pose at both corners at the solve over a wide
/// box, and every vertex of the bolt and the transformed body is
/// enclosed through the evaluation over a narrow one.
#[test]
fn a2_a_box_on_a_transform_placers_lift_encloses_the_mated_part() {
    let l = lifted("msolve14-a2-lift-box");
    let width = assert_pose_encloses_corners(
        &l.doc,
        &l.opts,
        l.bolt,
        gap(),
        GAP,
        (-0.2, 0.3),
        "the lifted box, at the solve",
    );
    assert!(width >= 0.5 * 0.4_f64.cos(), "the pose widens: {width}");
    let (lo, hi) = narrow(0.05);
    let ev = run_at::<Interval>(&l.doc, &boxed(&l.opts, gap(), lo, hi), None);
    assert!(
        ev.node_error(l.mate).is_none(),
        "{:?}",
        ev.node_error(l.mate)
    );
    for node in [l.bolt, l.transform] {
        assert_encloses_corners(
            &l.doc,
            &l.opts,
            &ev,
            node,
            (gap(), GAP),
            (lo, hi),
            "the lifted box, evaluated",
        );
    }
}

// ---- A1: a face frame resolves on every lane ----

/// **A face-framed mate on a `Dual64` seed run resolves and carries its
/// pose's tangent**: the bolt seated through copy #2 on the slab cap's
/// OWN pose moves by `−2` per unit of spacing, as the authored seat
/// does, held to the `f64` builds' central difference.
#[test]
fn a1_a_face_frame_on_a_seed_run_carries_the_poses_tangent() {
    let b = bolted("msolve14-a1-seed", MateFrame::FromFace);
    let ev = run_at::<Dual64>(&b.doc, &seeded(&b.opts, spacing()), None);
    assert!(
        ev.node_error(b.mate).is_none(),
        "the face frame resolves at Dual64: {:?}",
        ev.node_error(b.mate)
    );
    let tangents = assert_tangents_match(
        &b.doc,
        &b.opts,
        &ev,
        b.bolt,
        spacing(),
        SPACING,
        "the face-seated bolt",
    );
    for (name, t) in tangents {
        assert_eq!(t, [-2.0, 0.0, 0.0], "∂B/∂s at {name:?}");
    }
    let unseeded = run_at::<Dual64>(&b.doc, &b.opts, None);
    assert!(
        unseeded.node_error(b.mate).is_none(),
        "{:?}",
        unseeded.node_error(b.mate)
    );
}

/// **A face-framed mate on an `Interval` box run resolves to an
/// enclosure of the pose at every box corner**: at the solve over a wide
/// box, and through the evaluation over a narrow one, where the slab —
/// whose face the frame reads — holds still.
#[test]
fn a1_a_face_frame_on_a_box_run_encloses_the_pose_at_every_corner() {
    let b = bolted("msolve14-a1-box", MateFrame::FromFace);
    let width = assert_pose_encloses_corners(
        &b.doc,
        &b.opts,
        b.bolt,
        spacing(),
        SPACING,
        (-0.25, 0.25),
        "the face-seated box, at the solve",
    );
    assert!(width >= 1.0, "the face-seated pose widens: {width}");
    let (lo, hi) = narrow(0.05);
    let ev = run_at::<Interval>(&b.doc, &boxed(&b.opts, spacing(), lo, hi), None);
    assert!(
        ev.node_error(b.mate).is_none(),
        "the face frame resolves at Interval: {:?}",
        ev.node_error(b.mate)
    );
    for node in [b.bolt, b.pattern, b.slab] {
        assert_encloses_corners(
            &b.doc,
            &b.opts,
            &ev,
            node,
            (spacing(), SPACING),
            (lo, hi),
            "the face-seated box, evaluated",
        );
    }
}

// ---- A4: the memo key carries the pose's two channels ----

/// **A seeded pass threaded an unseeded `Dual64` prior reuses no
/// zero-tangent pose** — `stackup`'s arrangement, through the front
/// door. The bolt instance has no slot the seed reaches and no DAG
/// input, so only its solved pose distinguishes the seeded pass's
/// answer from the prior's: a key that fed the pose's value channel
/// alone would serve the prior's bolt, tangent zero, and copy #2 would
/// then move by `+2`.
#[test]
fn a4_a_seeded_pass_over_an_unseeded_prior_reuses_no_zero_tangent_pose() {
    let b = bolted("msolve14-a4", MateFrame::FromFace);
    let base_opts = EvalOptions {
        profile_lift: ProfileLift::Guided,
        ..b.opts.clone()
    };
    let base = run_at::<Dual64>(&b.doc, &base_opts, None);
    for (name, p) in vertices(&base, b.bolt) {
        assert_eq!(
            [p.x.deriv, p.y.deriv, p.z.deriv],
            [0.0; 3],
            "the unseeded base carries no tangent at {name:?}"
        );
    }
    let pass = run_at::<Dual64>(&b.doc, &seeded(&b.opts, spacing()), Some(&base));
    for (name, p) in vertices(&pass, b.bolt) {
        assert_eq!(
            [p.x.deriv, p.y.deriv, p.z.deriv],
            [-2.0, 0.0, 0.0],
            "the seeded pass's bolt at {name:?} carries ∂B/∂s, not the prior's zero"
        );
    }
    for (name, p) in vertices(&pass, b.pattern) {
        let want = [-2.0, -1.0, 0.0][copy_of(&name) as usize];
        assert_eq!(p.x.deriv, want, "∂copy/∂s at {name:?} over the prior");
    }
}

// ---- A3's other half, and C5: one structure, every lane ----

/// What a lane's evaluation says about the structure: each mate's role
/// (its value) or that it failed, and whether each instance failed.
fn structure<T: editor_core::EvalScalar>(
    doc: &ProfileDoc,
    ev: &Evaluation<T>,
) -> Vec<(RecipeNodeId, String)> {
    doc.order()
        .iter()
        .filter(|&&id| {
            matches!(
                doc.node(id),
                Some(Node::Mate { .. } | Node::InstantiatePart { .. })
            )
        })
        .map(|&id| {
            let said = match ev.result(id) {
                Some(NodeResult::Ok(v)) => match &v.payload {
                    ValuePayload::Mate(role) => format!("{role:?}"),
                    other => other.kind_name().to_string(),
                },
                Some(other) => format!("failed: {}", fault_class(other)),
                None => "absent".to_string(),
            };
            (id, said)
        })
        .collect()
}

/// A failed result's class: the error kind's variant, and for a mate
/// fault the fault's — never its numbers, which are the lane's.
fn fault_class<T: geom_core::Decide>(r: &NodeResult<T>) -> String {
    let full = format!("{r:?}");
    full.split(['{', '(']).take(4).collect::<Vec<_>>().join("|")
}

/// **An interval lane's structure is the `f64` one's, or an escalation
/// on a mate's log**: where an interval decision could not settle, the
/// mate faults `Indeterminate` — or the member whose checked offset it
/// was faults `OffsetUnchecked` with the same cause — the escalation is
/// on a mate's own log, and what the fault reaches fails with it;
/// everything else reads as at `f64`. An identically-zero membership
/// margin (the constructed candidate's own residual, a true checked
/// offset's) encloses zero only as tightly as the lane's rounding, so at
/// a fine enough ε it escalates on an unboxed run (the spec's §5 (b)):
/// the corpus's clocked coaxial pair and its true checked offset do at
/// ε = 1e-12.
fn assert_interval_structure(
    doc: &ProfileDoc,
    f: &Evaluation<f64>,
    i: &Evaluation<Interval>,
    label: &str,
) {
    use editor_core::{MateFault, NodeErrorKind, OffsetCheck};
    let escalating = |id: RecipeNodeId| {
        i.node_error(id).is_some_and(|e| match &e.kind {
            NodeErrorKind::Mate(fault) => match &**fault {
                MateFault::Indeterminate { .. } => true,
                MateFault::OffsetUnchecked { cause, .. } => {
                    matches!(**cause, OffsetCheck::Indeterminate(_))
                }
                _ => false,
            },
            _ => false,
        })
    };
    let mate_log_escalated = doc.order().iter().any(|&id| {
        matches!(doc.node(id), Some(Node::Mate { .. }))
            && match i.result(id) {
                Some(NodeResult::Ok(v)) => !v.escalations.is_empty(),
                _ => i.node_error(id).is_some_and(|e| !e.escalations.is_empty()),
            }
    });
    let want = structure(doc, f);
    let got = structure(doc, i);
    let mut moved = Vec::new();
    for ((id, w), (_, g)) in want.iter().zip(&got) {
        if w == g {
            continue;
        }
        assert!(
            escalating(*id),
            "{label}: Interval reads {g} at {id:?} where f64 reads {w}, and its fault is no \
             escalation"
        );
        moved.push(*id);
    }
    if !moved.is_empty() {
        assert!(
            mate_log_escalated,
            "{label}: Interval escalated at {moved:?} with no escalation on a mate's log"
        );
        println!(
            "{label}: Interval escalated at {moved:?} (ε = {:e})",
            Tol::witness().eps()
        );
    }
}

/// **C5 and A3's dual half**: over the whole corpus, the solve's
/// structure — every mate's role, and which mates and instances fail —
/// is the `f64` build's at `Dual64` (unseeded and under every seed) and
/// at `Interval` (unboxed), save where an interval decision ESCALATES
/// ([`assert_interval_structure`]); and at `Dual64` every instance's
/// every vertex reads the `f64` build's bits on its value channel.
#[test]
fn c5_one_documents_structure_is_the_same_in_every_lane_and_the_dual_value_is_f64s() {
    for (label, doc, opts) in corpus() {
        let f = run_at::<f64>(&doc, &opts, None);
        let d = run_at::<Dual64>(&doc, &opts, None);
        let i = run_at::<Interval>(&doc, &opts, None);
        let want = structure(&doc, &f);
        assert_eq!(structure(&doc, &d), want, "{label}: Dual64's structure");
        assert_interval_structure(&doc, &f, &i, label);
        let params: Vec<ParamName> = doc.params().keys().cloned().collect();
        let seeds = std::iter::once(None).chain(params.into_iter().map(Some));
        for seed in seeds {
            let o = EvalOptions {
                seed: seed.clone(),
                profile_lift: ProfileLift::Guided,
                ..opts.clone()
            };
            let d = run_at::<Dual64>(&doc, &o, None);
            assert_eq!(structure(&doc, &d), want, "{label}: seeded {seed:?}");
            for &id in doc.order() {
                if !matches!(doc.node(id), Some(Node::InstantiatePart { .. }))
                    || f.node_error(id).is_some()
                {
                    continue;
                }
                let at: BTreeMap<StableName, [u64; 3]> = vertices(&f, id)
                    .into_iter()
                    .map(|(n, p)| (n, [p.x, p.y, p.z].map(f64::to_bits)))
                    .collect();
                for (name, p) in vertices(&d, id) {
                    assert_eq!(
                        [p.x.value, p.y.value, p.z.value].map(f64::to_bits),
                        at[&name],
                        "{label}, seed {seed:?}: the value channel at {name:?} is f64's"
                    );
                }
            }
        }
    }
}

// ---- A5: the two analysis doors over a face-framed assembly ----

/// **`stackup::sensitivities` over the face-framed bolt**: the distance
/// from a vertex of the bolt to a vertex of the slab, differentiated in
/// the spacing, is the `f64` builds' central difference — ∂m/∂s crosses
/// the face-framed mate.
#[test]
fn a5_sensitivities_cross_a_face_framed_mate() {
    use editor_core::stackup::{SensitivityOutcome, sensitivities_resolved};
    use editor_core::{MeasureExpr, MeasurePrimitive, SitedRef};
    let b = bolted("msolve14-a5-stackup", MateFrame::FromFace);
    let ev = run_at::<f64>(&b.doc, &b.opts, None);
    let foot = all_vertices(&ev, b.bolt)[0].clone();
    let corner = all_vertices(&ev, b.slab)[0].clone();
    let (doc, m) = insert(
        b.doc.clone(),
        Node::measure(
            MeasureExpr::primitive(MeasurePrimitive::Distance { a: 0, b: 1 }),
            vec![
                SitedRef::new(b.bolt, foot.clone()),
                SitedRef::new(b.slab, corner.clone()),
            ],
        )
        .expect("both indices address a reference"),
    );
    // The closed form: the bolt moves by `−2` per unit of spacing along
    // `x` and the slab holds still, so `∂|d|/∂s = d · (−2, 0, 0) / |d|`
    // over the vertices' own `f64` positions.
    let p = vertex_position(&ev, b.bolt, &foot).expect("the bolt's vertex");
    let q = vertex_position(&ev, b.slab, &corner).expect("the slab's vertex");
    let d = [p.x - q.x, p.y - q.y, p.z - q.z];
    let fd = -2.0 * d[0] / (d[0] * d[0] + d[1] * d[1] + d[2] * d[2]).sqrt();
    assert!(fd.abs() > 0.1, "the measure moves with the spacing: {fd}");
    let resolver = b.opts.resolver.clone().expect("the store resolves");
    let entries = sensitivities_resolved(&doc, m, None, None, false, &resolver, Tol::witness())
        .expect("the driver runs");
    assert_eq!(entries.len(), 1, "one continuous parameter");
    match &entries[0].outcome {
        SensitivityOutcome::Derivative { value, .. } => assert!(
            (value - fd).abs() <= 1e-12,
            "∂m/∂s is {value} where the closed form gives {fd}"
        ),
        other => panic!("a derivative across the face frame, not {other:?}"),
    }
}

/// **Certified `clearance` over the face-framed bolt**: the separation
/// between the bolt and copy #2 is `2s − 1`, three at the nominal, so a
/// leaf around it holds a clearance of 2.5 and violates one of 3.5 —
/// each answered over the leaf, the mate solved at its scalar.
#[test]
fn a5_certified_clearance_runs_over_a_face_framed_mate() {
    use editor_core::clearance::{
        ClearanceQuery, ClearanceVerdict, FaceScope, Selection, clearance_with,
    };
    let b = bolted("msolve14-a5-clearance", MateFrame::FromFace);
    let resolver = b.opts.resolver.clone().expect("the store resolves");
    let (lo, hi) = narrow(0.05);
    let leaf = ParamBox::from_axes(BTreeMap::from([(spacing(), BoxAxis::Varying { lo, hi })]));
    let copy2 = Selection {
        at: b.pattern,
        body: 2,
        faces: FaceScope::All,
    };
    let ask = |c: f64| {
        clearance_with(
            &b.doc,
            &leaf,
            &Selection::body_of(b.bolt),
            &copy2,
            &ClearanceQuery::at_least(c, Tol::witness()).resolved_by(&resolver),
        )
        .verdict()
        .clone()
    };
    assert!(
        matches!(ask(2.5), ClearanceVerdict::Holds),
        "2s − 1 = 3 clears 2.5 over the leaf: {:?}",
        ask(2.5)
    );
    assert!(
        matches!(ask(3.5), ClearanceVerdict::Violated(_)),
        "2s − 1 = 3 violates 3.5 over the leaf: {:?}",
        ask(3.5)
    );
}
