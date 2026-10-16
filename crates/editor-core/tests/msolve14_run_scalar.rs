//! MSOLVE-14 acceptance — **the mate solve runs at the evaluation's own
//! scalar** (`ASSEMBLY.md` A11 (5): the solve's inputs are read at the
//! evaluation's scalar and over its parameters; a pattern's count and a
//! `Part`'s index are read at the nominal).
//!
//! The rows, by the spec's acceptance:
//!
//! - **A3, the `f64` fence**: a mate corpus — authored and face frames,
//!   every primitive, a LOOSE bundle and an OVERCONSTRAINED one, a
//!   patterned bolt, a transform placer, a parametric world pin, a chain
//!   of bundles — solved and evaluated at `f64`, digested, and pinned;
//!   and the same corpus at `Dual64` reading the `f64` bits on its value
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
//! - **C5**: one document's structure — spaces, faults' presence — is
//!   the same in every lane.
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
    Alignment, AxisSense, BoxAxis, CancelToken, CapEnd, ContactClass, Dimension, DocEdit, DocRef,
    DocumentId, EvalOptions, Evaluation, Formula, FreeVar, Mate, MateFrame, MatePrimitive, MateRef,
    Node, NodeResult, ParamBox, PatternKind, Placement, ProfileDoc, ProfileLift, RecipeNodeId,
    SitedFace, StableName, Step, VarName, all_vertices, evaluate, vertex_position,
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

fn spacing() -> VarName {
    VarName::from_static("s")
}

fn gap() -> VarName {
    VarName::from_static("gap")
}

fn lift() -> VarName {
    VarName::from_static("lift")
}

fn turn() -> VarName {
    VarName::from_static("spin")
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
            profile: profile.into(),
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

fn authored(origin: [f64; 3], axis: [f64; 3]) -> MateFrame<Formula> {
    MateFrame::authored(origin, axis, [1.0, 0.0, 0.0], Tol::witness()).expect("a frame")
}

/// **A seat**: `mover`'s frame (its own origin, axis down) coincident
/// with `onto`'s, axes opposed — the mover stands ON the other part.
/// `onto_frame` is the frame on the seat's receiving side: authored at
/// a point of its top cap, or the cap face's own pose.
fn seat(mover: SitedFace, onto: SitedFace, onto_frame: MateFrame<Formula>) -> Mate<Formula> {
    fixture::mate(
        mover,
        onto,
        ContactClass::Rest,
        Alignment {
            a: authored([0.0, 0.0, 0.0], [0.0, 0.0, -1.0]),
            b: onto_frame,
            primitive: MatePrimitive::FrameCoincidence,
            sense: AxisSense::Opposed,
            clocking: None,
        },
    )
}

/// The receiving frame authored at `(x, y)` of the slab's top.
fn slab_at(x: f64, y: f64) -> MateFrame<Formula> {
    authored([x, y, SLAB_HEIGHT], [0.0, 0.0, 1.0])
}

fn declare(doc: ProfileDoc, name: VarName, v: f64, dim: Dimension) -> ProfileDoc {
    step(
        doc,
        DocEdit::DeclareVar {
            name,
            def: editor_core::VarDecl::Free(FreeVar::continuous(dim, v)),
        },
    )
    .0
}

fn set_value(doc: ProfileDoc, name: VarName, v: f64) -> ProfileDoc {
    step(
        doc,
        DocEdit::SetVarValue {
            var: name.into(),
            value: editor_core::FreeValue::Continuous(v),
        },
    )
    .0
}

/// A slab (pinned to the world at the identity) and `n` bolts whose
/// bundles are empty, each placed by whatever mates the caller adds.
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

/// Adds `mate` to `placement`'s bundle through the parts' own reach —
/// the door a face-framed side needs, since its admission reads the
/// part's face, and the door that folds the bundle it joins.
fn constrain_through(
    doc: ProfileDoc,
    placement: RecipeNodeId,
    mate: Mate<Formula>,
    opts: &EvalOptions,
) -> (ProfileDoc, MateRef) {
    let reach = editor_core::mate_reach::<f64>(opts, Tol::witness());
    match fixture::at_the_door_of(&doc, &reach, placement, mate) {
        Ok(admitted) => admitted,
        Err((at, fault)) => panic!("the constraint is admitted, got {at:?}: {fault}"),
    }
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
    /// The seat, a constraint of the bolt's bundle.
    mate: MateRef,
}

fn bolted(label: &str, slab_frame: MateFrame<Formula>) -> Bolted {
    let p = parts(label);
    let (doc, slab, bolts) = slab_and_bolts(&p, label, 1);
    let bolt = bolts[0];
    let doc = declare(doc, spacing(), SPACING, Dimension::Length);
    let (doc, pattern) = insert(
        doc,
        Node::Pattern {
            input: bolt.into(),
            count: Formula::count(3),
            kind: PatternKind::Linear {
                direction: [1.0, 0.0, 0.0].map(scl),
                spacing: Formula::named(spacing(), Dimension::Length),
            },
        },
    );
    let opts = p.opts();
    let (doc, mate) = constrain_through(
        doc,
        bolt,
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
    /// The seat, a constraint of the bolt's bundle.
    mate: MateRef,
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
                translation: [len(0.0), len(0.0), Formula::named(gap(), Dimension::Length)],
                axis: [1.0, 0.0, 0.0].map(scl),
                angle: ang(0.4),
            },
        ),
    );
    let opts = p.opts();
    let (doc, mate) = constrain_through(
        doc,
        bolt,
        seat(
            head_at(transform, p.bolt_foot(bolt)),
            head(p.slab_top(slab)),
            slab_at(2.0, 3.0),
        ),
        &opts,
    );
    Lifted {
        doc,
        opts,
        bolt,
        transform,
        mate,
    }
}

/// **A shaft in two coaxial bores**: the bolt held by two coaxial mates
/// on ONE line, so the second bore's axis point lies on the first's
/// axis and the fold meets a clocking with no radius to turn
/// (`clocking_about`'s degenerate arm); either bore is then one the
/// rest of the bundle already fixes, so the bundle is overconstrained. The clocking is then fixed
/// either by a clocked coaxial rider and a planar rest
/// ([`ShaftShape::ClockedRider`]), or by a planar rest and a third pin
/// on a parallel axis a quarter off the first ([`ShaftShape::ParallelPin`]).
/// The rest's slab frame rises with `rise`, which lifts the shaft one
/// for one; `idle` is declared and read by nothing.
struct Shaft {
    doc: ProfileDoc,
    opts: EvalOptions,
    bolt: RecipeNodeId,
    /// The bolt's bundle, in the order it was authored.
    mates: Vec<MateRef>,
}

#[derive(Clone, Copy, Debug)]
enum ShaftShape {
    ClockedRider,
    ParallelPin,
}

const RISE: f64 = 1.0;
const IDLE: f64 = 0.5;

fn rise() -> VarName {
    VarName::from_static("rise")
}

fn idle() -> VarName {
    VarName::from_static("idle")
}

/// The two bores' bolt-side and slab-side axis points, on the bolt's
/// and the slab's own axis lines.
type Bores = ([[f64; 3]; 2], [[f64; 3]; 2]);

/// The bores on one line exactly: `(0.5, 0.5)` through the bolt,
/// `(4, 4)` through the slab.
const BORES: Bores = (
    [[0.5, 0.5, 0.0], [0.5, 0.5, 2.0]],
    [[4.0, 4.0, SLAB_HEIGHT], [4.0, 4.0, -1.0]],
);

fn shaft(label: &str, shape: ShaftShape, bores: Bores) -> Shaft {
    try_shaft(label, shape, bores)
        .unwrap_or_else(|fault| panic!("{label}: the door admits the shaft's bundle: {fault}"))
}

/// [`shaft`], or the fault the constraint door refused its bundle with:
/// the door folds the bundle a constraint joins, so a bundle whose fold
/// cannot settle refuses there.
fn try_shaft(
    label: &str,
    shape: ShaftShape,
    (bolt_at, slab_at): Bores,
) -> Result<Shaft, editor_core::MateFault> {
    let p = parts(label);
    let (doc, slab, bolts) = slab_and_bolts(&p, label, 1);
    let bolt = bolts[0];
    let doc = declare(doc, rise(), RISE, Dimension::Length);
    let doc = declare(doc, idle(), IDLE, Dimension::Length);
    let bore = |a: [f64; 3], b: [f64; 3], clocking: Option<f64>| {
        fixture::mate(
            head(p.bolt_foot(bolt)),
            head(p.slab_top(slab)),
            ContactClass::Rest,
            Alignment {
                a: authored(a, [0.0, 0.0, -1.0]),
                b: authored(b, [0.0, 0.0, 1.0]),
                primitive: MatePrimitive::Coaxial,
                sense: AxisSense::Opposed,
                clocking,
            },
        )
    };
    let rest = fixture::mate(
        head(p.bolt_foot(bolt)),
        head(p.slab_top(slab)),
        ContactClass::Rest,
        Alignment {
            a: authored([0.0, 0.0, 0.0], [0.0, 0.0, -1.0]),
            b: MateFrame::on_part(Step::Rigid {
                translation: [
                    len(0.0),
                    len(0.0),
                    Formula::named(rise(), Dimension::Length),
                ],
                axis: [0.0, 0.0, 1.0].map(scl),
                angle: ang(0.0),
            }),
            primitive: MatePrimitive::PlanarRest { offset: 0.25 },
            sense: AxisSense::Opposed,
            clocking: None,
        },
    );
    let quarter = |x: [f64; 3]| [x[0] + 0.25, x[1], x[2]];
    let then = match shape {
        ShaftShape::ClockedRider => [bore(bolt_at[0], slab_at[0], Some(0.3)), rest],
        ShaftShape::ParallelPin => [rest, bore(quarter(bolt_at[0]), quarter(slab_at[0]), None)],
    };
    let opts = p.opts();
    let mut doc = doc;
    let mut mates = Vec::new();
    for mate in [
        bore(bolt_at[0], slab_at[0], None),
        bore(bolt_at[1], slab_at[1], None),
    ]
    .into_iter()
    .chain(then)
    {
        let reach = editor_core::mate_reach::<f64>(&opts, Tol::witness());
        let (next, mate) =
            fixture::at_the_door_of(&doc, &reach, bolt, mate).map_err(|(_, fault)| fault)?;
        doc = next;
        mates.push(mate);
    }
    Ok(Shaft {
        doc,
        opts,
        bolt,
        mates,
    })
}

// ---- The mate corpus ----

/// **The mate corpus the fence is taken over** — one document per shape
/// the solve has a branch for, each with its resolver. Built through
/// the ordinary doors, so the documents are what an author makes.
fn corpus() -> Vec<(&'static str, ProfileDoc, EvalOptions)> {
    let mut out = Vec::new();

    // An authored opposed seat, and a face-framed one on the slab's cap.
    let p = parts("msolve14-c-seat");
    let opts = p.opts();
    let (doc, slab, bolts) = slab_and_bolts(&p, "msolve14-c-seat", 2);
    let (doc, _) = constrain_through(
        doc,
        bolts[0],
        seat(
            head(p.bolt_foot(bolts[0])),
            head(p.slab_top(slab)),
            slab_at(1.0, 1.0),
        ),
        &opts,
    );
    let (doc, _) = constrain_through(
        doc,
        bolts[1],
        seat(
            head(p.bolt_foot(bolts[1])),
            head(p.slab_top(slab)),
            MateFrame::from_face(),
        ),
        &opts,
    );
    out.push(("seats", doc, opts));

    // Coaxial with a clocking rider, cut to DETERMINED by a planar rest:
    // the prismatic-planar row and the translation stage's cosine.
    let p = parts("msolve14-c-coax");
    let opts = p.opts();
    let (doc, slab, bolts) = slab_and_bolts(&p, "msolve14-c-coax", 1);
    let coax = fixture::mate(
        head(p.bolt_foot(bolts[0])),
        head(p.slab_top(slab)),
        ContactClass::Rest,
        Alignment {
            a: authored([0.5, 0.5, 0.0], [0.0, 0.0, -1.0]),
            b: authored([4.0, 4.0, SLAB_HEIGHT], [0.0, 0.0, 1.0]),
            primitive: MatePrimitive::Coaxial,
            sense: AxisSense::Opposed,
            clocking: Some(0.3),
        },
    );
    let rest_alignment = Alignment {
        a: authored([0.0, 0.0, 0.0], [0.0, 0.0, -1.0]),
        b: authored([0.0, 0.0, SLAB_HEIGHT], [0.0, 0.0, 1.0]),
        primitive: MatePrimitive::PlanarRest { offset: 0.25 },
        sense: AxisSense::Opposed,
        clocking: None,
    };
    let rest = |p: &Parts, bolt: RecipeNodeId, slab: RecipeNodeId| {
        fixture::mate(
            head(p.bolt_foot(bolt)),
            head(p.slab_top(slab)),
            ContactClass::Rest,
            rest_alignment.clone(),
        )
    };
    let (doc, _) = constrain_through(doc, bolts[0], coax, &opts);
    let (doc, _) = constrain_through(doc, bolts[0], rest(&p, bolts[0], slab), &opts);
    out.push(("coaxial-clocked-rest", doc, opts));

    // A lone planar rest: the bolt is LOOSE, its own space's root.
    let p = parts("msolve14-c-loose");
    let opts = p.opts();
    let (doc, slab, bolts) = slab_and_bolts(&p, "msolve14-c-loose", 1);
    let (doc, _) = constrain_through(doc, bolts[0], rest(&p, bolts[0], slab), &opts);
    out.push(("loose", doc, opts));

    // A planar rest, then a coincidence the rest is redundant beside:
    // the door admits the coincidence (the bundle before it pinned
    // less), and the solve refuses the bundle OVERCONSTRAINED, naming
    // the rest.
    let p = parts("msolve14-c-over");
    let opts = p.opts();
    let (doc, slab, bolts) = slab_and_bolts(&p, "msolve14-c-over", 1);
    let (doc, _) = constrain_through(doc, bolts[0], rest(&p, bolts[0], slab), &opts);
    let (doc, _) = constrain_through(
        doc,
        bolts[0],
        seat(
            head(p.bolt_foot(bolts[0])),
            head(p.slab_top(slab)),
            slab_at(2.0, 1.0),
        ),
        &opts,
    );
    out.push(("overconstrained", doc, opts));

    // The patterned bolt, authored and face-framed.
    let b = bolted("msolve14-c-bolt", slab_at(6.0, 4.0));
    out.push(("patterned-bolt", b.doc, b.opts));
    let b = bolted("msolve14-c-bolt-face", MateFrame::from_face());
    out.push(("patterned-bolt-face", b.doc, b.opts));

    // The rotating transform placer.
    let l = lifted("msolve14-c-lift");
    out.push(("transform-placer", l.doc, l.opts));

    // A parametric world pin of three steps (two of them read
    // parameters), a transform on a seat's side, and a seat on a copy
    // the seat before it placed: a chain of bundles back to the world.
    let p = parts("msolve14-c-pin");
    let opts = p.opts();
    let doc = ProfileDoc::empty(DocumentId::derive("msolve14-c-pin"), Tol::witness());
    let doc = declare(doc, lift(), 2.0, Dimension::Length);
    let doc = declare(doc, turn(), 0.25, Dimension::Angle);
    let (doc, slab) = insert(doc, fixture::mated_instance(p.slab));
    let (doc, _) = fixture::constrain_at(
        doc,
        slab,
        Mate::world_pin(Placement {
            steps: vec![
                Step::Rigid {
                    translation: [5.0, 0.0, 0.0].map(len),
                    axis: [1.0, 0.0, 0.0].map(scl),
                    angle: ang(0.3),
                },
                Step::Rigid {
                    translation: [
                        len(0.0),
                        len(0.0),
                        Formula::named(lift(), Dimension::Length),
                    ],
                    axis: [0.0, 0.0, 1.0].map(scl),
                    angle: Formula::named(turn(), Dimension::Angle),
                },
                Step::Rigid {
                    translation: [1.0, 2.0, 0.0].map(len),
                    axis: [0.0, 1.0, 0.0].map(scl),
                    angle: ang(0.2),
                },
            ],
        }),
    );
    let (doc, top) = insert(doc, fixture::mated_instance(p.bolt));
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
    let (doc, _) = constrain_through(
        doc,
        top,
        seat(
            head_at(t, p.bolt_foot(top)),
            head(p.slab_top(slab)),
            slab_at(1.0, 1.0),
        ),
        &opts,
    );
    let (doc, third) = insert(doc, fixture::mated_instance(p.bolt));
    let (doc, _) = constrain_through(
        doc,
        third,
        seat(
            head(p.bolt_foot(third)),
            head(p.bolt_head(top)),
            authored([0.5, 0.5, BOLT_HEIGHT], [0.0, 0.0, 1.0]),
        ),
        &opts,
    );
    out.push(("pinned-chain", doc, opts));

    // Two bolts on the slab and a third seated on the second: a chain
    // of bundles, each pinned to the copy before it.
    let p = parts("msolve14-c-chain");
    let opts = p.opts();
    let (doc, slab, bolts) = slab_and_bolts(&p, "msolve14-c-chain", 3);
    let (doc, _) = constrain_through(
        doc,
        bolts[0],
        seat(
            head(p.bolt_foot(bolts[0])),
            head(p.slab_top(slab)),
            slab_at(1.0, 1.0),
        ),
        &opts,
    );
    let (doc, _) = constrain_through(
        doc,
        bolts[1],
        seat(
            head(p.bolt_foot(bolts[1])),
            head(p.slab_top(slab)),
            slab_at(4.0, 1.0),
        ),
        &opts,
    );
    let (doc, _) = constrain_through(
        doc,
        bolts[2],
        seat(
            head(p.bolt_foot(bolts[2])),
            head(p.bolt_head(bolts[1])),
            authored([0.5, 0.5, BOLT_HEIGHT], [0.0, 0.0, 1.0]),
        ),
        &opts,
    );
    out.push(("chain", doc, opts));

    // Two pins on parallel axes and a rest: the fold meets two
    // rotation constraints about parallel axes and SOLVES the free
    // clocking between them (`clocking_about`'s angle), then the rest
    // cuts the prismatic residual to DETERMINED.
    let p = parts("msolve14-c-two-pin");
    let opts = p.opts();
    let (doc, slab, bolts) = slab_and_bolts(&p, "msolve14-c-two-pin", 1);
    let pin = |a: [f64; 3], b: [f64; 3]| {
        fixture::mate(
            head(p.bolt_foot(bolts[0])),
            head(p.slab_top(slab)),
            ContactClass::Rest,
            Alignment {
                a: MateFrame::authored(a, [0.0, 0.0, -1.0], [0.6, 0.8, 0.0], Tol::witness())
                    .expect("a frame"),
                b: MateFrame::authored(b, [0.0, 0.0, 1.0], [1.0, 0.0, 0.0], Tol::witness())
                    .expect("a frame"),
                primitive: MatePrimitive::Coaxial,
                sense: AxisSense::Opposed,
                clocking: None,
            },
        )
    };
    let flat_rest = |p: &Parts, bolt: RecipeNodeId, slab: RecipeNodeId| {
        fixture::mate(
            head(p.bolt_foot(bolt)),
            head(p.slab_top(slab)),
            ContactClass::Rest,
            Alignment {
                a: authored([0.0, 0.0, 0.0], [0.0, 0.0, -1.0]),
                b: authored([0.0, 0.0, SLAB_HEIGHT], [0.0, 0.0, 1.0]),
                primitive: MatePrimitive::PlanarRest { offset: 0.0 },
                sense: AxisSense::Opposed,
                clocking: None,
            },
        )
    };
    let (doc, _) = constrain_through(
        doc,
        bolts[0],
        pin([0.25, 0.25, 0.0], [3.0, 2.0, SLAB_HEIGHT]),
        &opts,
    );
    let (doc, _) = constrain_through(
        doc,
        bolts[0],
        pin([0.75, 0.5, 0.0], [3.5, 2.25, SLAB_HEIGHT]),
        &opts,
    );
    let (doc, _) = constrain_through(doc, bolts[0], flat_rest(&p, bolts[0], slab), &opts);
    out.push(("two-pin", doc, opts));

    // A pin across a rest's normal: two rotation constraints about
    // DIFFERENT axes, met through the reachability predicate and its
    // angle (`candidate_rotation`'s two-axis arm); the prismatic
    // residual along the pin leaves the bolt LOOSE.
    let p = parts("msolve14-c-cross-pin");
    let opts = p.opts();
    let (doc, slab, bolts) = slab_and_bolts(&p, "msolve14-c-cross-pin", 1);
    let (doc, _) = constrain_through(doc, bolts[0], flat_rest(&p, bolts[0], slab), &opts);
    let (doc, _) = constrain_through(
        doc,
        bolts[0],
        fixture::mate(
            head(p.bolt_foot(bolts[0])),
            head(p.slab_top(slab)),
            ContactClass::Rest,
            Alignment {
                a: MateFrame::authored(
                    [0.5, 0.0, 0.5],
                    [0.3, 1.0, 0.0],
                    [0.0, 0.0, 1.0],
                    Tol::witness(),
                )
                .expect("a frame"),
                b: MateFrame::authored(
                    [2.0, 3.0, 1.5],
                    [1.0, 0.2, 0.0],
                    [0.0, 0.0, 1.0],
                    Tol::witness(),
                )
                .expect("a frame"),
                primitive: MatePrimitive::Coaxial,
                sense: AxisSense::Aligned,
                clocking: None,
            },
        ),
        &opts,
    );
    out.push(("cross-pin", doc, opts));

    // A shaft in two coaxial bores on one line, clocked by a rider or by
    // a third pin: the fold's clocking with no radius to turn.
    for (label, shape) in [
        ("shaft-clocked-rider", ShaftShape::ClockedRider),
        ("shaft-parallel-pin", ShaftShape::ParallelPin),
    ] {
        let s = shaft(&format!("msolve14-c-{label}"), shape, BORES);
        out.push((label, s.doc, s.opts));
    }
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
/// document order with its fault and space, and every instance's
/// relative pose and world placement by bits (or the
/// refusal, by its `Debug`, whose `f64`s are shortest round-trips) —
/// then the document's evaluation, through the shared evaluation
/// digest, and every node's error.
fn solve_digest(h: &mut Fnv, doc: &ProfileDoc, opts: &EvalOptions) {
    let poses = solve(doc, opts, Tol::witness());
    for id in doc.ids() {
        h.feed(format!("#{id:?}").as_bytes());
        h.feed(format!("fault={:?}", poses.fault(id)).as_bytes());
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
    for id in doc.ids() {
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

/// **The corpus digest, per ε row.** It was measured on the
/// pre-generic tree from this same file's corpus and digest — first at
/// `a8f56a79f`, before any solve code moved, again at `2d29576fb` once
/// PLACE's mate-frame offset (#3961) changed how an authored side is
/// held, and at `11d9c7a7f` once the corpus took the shaft in two
/// coaxial bores — which is what showed the generic solve at `f64` to be
/// the nominal solve, bit for bit. The numbers below are no longer that
/// tree's: they were re-taken on this tree when the variable table
/// renumbered the corpus's ids (the paragraph below), so they pin the
/// generic solve against itself, and the bit-for-bit claim against the
/// pre-generic tree rests on that measurement and on the id-free rows.
///
/// One number per row because the documents are not ε-free: a part's
/// content pin hashes its recorded ε, and an instance's id hashes the
/// pin, so every id the digest feeds moves with the row. The rows are
/// the three the hosted matrix runs; any other ε has no measurement and
/// the row fails rather than pass on nothing.
///
/// Re-taken when an intrinsic edge description's surfaces became a
/// set (`geom_brep::SurfacePair`): the digest feeds the evaluated
/// bodies' `Debug`, which spells the pair differently, and no pose moved
/// (the id-free rows held).
///
/// The digest feeds node ids, so it moves whenever ids do — a declare
/// extends the mint chain, and an insert's preimage holds its readers by
/// variable id — with no pose moving: this file's id-free rows (each
/// pose against the `f64` solve at the box's corners, each tangent
/// against its central difference) are what say so.
///
/// Re-taken once more for INTENT-LITERALS PR C: every slot holds a
/// variable's id, so every node is minted from other bytes, and the
/// id-free rows held.
///
/// And again when an id became its mint ordinal and its digest (all
/// three rows): the digest spells every id it feeds. The same digest
/// with each id replaced by its document position measured equal on
/// main and on the branch at all three ε, so no pose, role, fault or
/// placement moved.
///
/// And again for INTENT stage 2 C (all three rows): each part places
/// its body in its world, so every part's pin — and every instance id
/// hashing it — moved, and the part's names reach an instance under its
/// placement's copy. The id-free rows held.
///
/// And again for INTENT stage 4 E (all three rows): the bodies this
/// digest feeds carry no provenance side tables, and a node error is
/// the value-decided refusal or the glued build. The id-free rows held.
///
/// And again for INTENT stage 2 F (all three rows): a mate's sides are
/// selections, minted at its insert, so every id after a mate moved,
/// and a mate's content key reads its bodies' keys. Each instance's
/// solved and world pose, each mate's role and whether it faulted,
/// keyed by document position, measured equal on main and on the
/// branch at all three ε. Taken over the surface pair's set spelling
/// above, which landed beside it.
///
/// And again for INTENT stage 3 B (all three rows): a mate is a
/// constraint of the placement it places, so the corpus's documents
/// changed shape — the lone rest is a loose copy rather than a refusal,
/// the contradictory pair is an overconstrained bundle, the gauge chain
/// is a three-step world pin, the closing loop is a chain of bundles,
/// and both shafts' bundles are overconstrained — and the digest feeds
/// each node's space and fault where it fed a mate's role and a
/// group's root.
const MAIN_CORPUS_DIGEST: [(f64, u64); 3] = [
    (1e-9, 0xeca8_2432_1b41_a72c),
    (1e-6, 0x842a_00b7_5945_84ae),
    (1e-12, 0x340f_1cf9_18ae_208f),
];

/// **A3, the `f64` fence**: the corpus's solved poses, spaces, faults and
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
    param: VarName,
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

/// The variable `doc` names `param`.
fn var_of(doc: &ProfileDoc, param: &VarName) -> editor_core::VarId {
    doc.var_named(param.as_str())
        .expect("the fixture declares it")
}

fn seeded(doc: &ProfileDoc, opts: &EvalOptions, param: VarName) -> EvalOptions {
    EvalOptions {
        seed: Some(var_of(doc, &param)),
        profile_lift: ProfileLift::Guided,
        ..opts.clone()
    }
}

fn boxed(doc: &ProfileDoc, opts: &EvalOptions, param: VarName, lo: f64, hi: f64) -> EvalOptions {
    EvalOptions {
        param_box: Some(Arc::new(ParamBox::from_axes(BTreeMap::from([(
            var_of(doc, &param),
            BoxAxis::Varying { lo, hi },
        )])))),
        profile_lift: ProfileLift::Guided,
        ..opts.clone()
    }
}

/// **A seed run's tangent at every vertex of `node` is the central
/// difference of two `f64` builds** at `nominal ± h` — exact up to
/// rounding where the motion is affine in the parameter (a translation,
/// or a rotation the parameter does not turn); [`assert_turn_tangents_match`]
/// takes a finer step for one the parameter turns. The value channel is
/// the `f64` nominal build's bits. Returns each vertex's tangent.
fn assert_tangents_match(
    doc: &ProfileDoc,
    opts: &EvalOptions,
    ev: &Evaluation<Dual64>,
    node: RecipeNodeId,
    param: VarName,
    nominal: f64,
    what: &str,
) -> Vec<(StableName, [f64; 3])> {
    tangents_match(doc, opts, ev, node, (param, nominal), (1e-3, 1e-9), what)
}

/// [`assert_tangents_match`] for a parameter that TURNS the vertices:
/// the central difference's truncation is `h²/6` of the third
/// derivative, so the step is `1e-5` and the slack `1e-8`.
fn assert_turn_tangents_match(
    doc: &ProfileDoc,
    opts: &EvalOptions,
    ev: &Evaluation<Dual64>,
    node: RecipeNodeId,
    param: VarName,
    nominal: f64,
    what: &str,
) -> Vec<(StableName, [f64; 3])> {
    tangents_match(doc, opts, ev, node, (param, nominal), (1e-5, 1e-8), what)
}

fn tangents_match(
    doc: &ProfileDoc,
    opts: &EvalOptions,
    ev: &Evaluation<Dual64>,
    node: RecipeNodeId,
    (param, nominal): (VarName, f64),
    (h, slack): (f64, f64),
    what: &str,
) -> Vec<(StableName, [f64; 3])> {
    let at = f64_vertices_at(doc, opts, node, param.clone(), nominal);
    let up = f64_vertices_at(doc, opts, node, param.clone(), nominal + h);
    let down = f64_vertices_at(doc, opts, node, param, nominal - h);
    let mut out = Vec::new();
    for (name, p) in vertices(ev, node) {
        let value = [p.x.value, p.y.value, p.z.value];
        let tangent = [p.x.deriv, p.y.deriv, p.z.deriv];
        assert!(
            tangent.iter().all(|t| t.is_finite()),
            "{what}: the tangent of {name:?} is finite: {tangent:?}"
        );
        assert_eq!(
            value.map(f64::to_bits),
            at[&name].map(f64::to_bits),
            "{what}: the value channel of {name:?} is the f64 build's"
        );
        for k in 0..3 {
            let fd = (up[&name][k] - down[&name][k]) / (2.0 * h);
            assert!(
                (tangent[k] - fd).abs() <= slack,
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
/// nominal** — so where the vertex moves across the box, the enclosure
/// spans that motion.
fn assert_encloses_corners(
    doc: &ProfileDoc,
    opts: &EvalOptions,
    ev: &Evaluation<Interval>,
    node: RecipeNodeId,
    (param, nominal): (VarName, f64),
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
/// (`work/topotail/a-boxed-rotation-refuses-not-rigid-at-every-placer.md`,
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
    param: VarName,
    nominal: f64,
    (lo, hi): (f64, f64),
    what: &str,
) -> f64 {
    use geom_core::Bounds;
    let pbox = ParamBox::from_axes(BTreeMap::from([(
        var_of(doc, &param),
        BoxAxis::Varying { lo, hi },
    )]));
    let env = editor_core::var_env_over::<Interval, _>(doc, &pbox).expect("the box binds");
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
    let ev = run_at::<Dual64>(&b.doc, &seeded(&b.doc, &b.opts, spacing()), None);
    assert!(
        ev.node_error(b.mate.placement).is_none(),
        "{:?}",
        ev.node_error(b.mate.placement)
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
    let ev = run_at::<Interval>(&b.doc, &boxed(&b.doc, &b.opts, spacing(), lo, hi), None);
    assert!(
        ev.node_error(b.mate.placement).is_none(),
        "{:?}",
        ev.node_error(b.mate.placement)
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
/// same way): the bolt's own refusal is the rigid transform's
/// certification, and the pattern, which reads the bolt, is poisoned
/// through it.
#[test]
fn a2_over_a_wide_box_the_bolts_refusal_is_the_placement_doors() {
    let b = bolted("msolve14-a2-wide", slab_at(6.0, 4.0));
    let ev = run_at::<Interval>(
        &b.doc,
        &boxed(&b.doc, &b.opts, spacing(), -0.25, 0.25),
        None,
    );
    assert!(
        matches!(
            ev.result(b.pattern),
            Some(NodeResult::Poisoned { through }) if *through == b.bolt
        ),
        "{:?}",
        ev.result(b.pattern)
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
    let ev = run_at::<Dual64>(&l.doc, &seeded(&l.doc, &l.opts, gap()), None);
    assert!(
        ev.node_error(l.mate.placement).is_none(),
        "{:?}",
        ev.node_error(l.mate.placement)
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
    let ev = run_at::<Interval>(&l.doc, &boxed(&l.doc, &l.opts, gap(), lo, hi), None);
    assert!(
        ev.node_error(l.mate.placement).is_none(),
        "{:?}",
        ev.node_error(l.mate.placement)
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

// ---- A2's third set of names: a mate frame's own offset ----

/// **A bolt seated on a slab frame whose offset two parameters drive**
/// (PLACE's mate-frame offset): the slab side is the part base composed
/// with a rigid step that slides along `x` by `slide` and turns about
/// the frame's axis by `spin`. The mate seats the bolt there, so the
/// bolt moves with both.
struct Slid {
    doc: ProfileDoc,
    opts: EvalOptions,
    bolt: RecipeNodeId,
    /// The seat, a constraint of the bolt's bundle.
    mate: MateRef,
}

const SLIDE: f64 = 2.0;
const SPIN: f64 = 0.3;

fn slide() -> VarName {
    VarName::from_static("slide")
}

fn spin() -> VarName {
    VarName::from_static("spin")
}

fn slid(label: &str) -> Slid {
    let p = parts(label);
    let (doc, slab, bolts) = slab_and_bolts(&p, label, 1);
    let doc = declare(doc, slide(), SLIDE, Dimension::Length);
    let doc = declare(doc, spin(), SPIN, Dimension::Angle);
    let frame = MateFrame::on_part(Step::Rigid {
        translation: [
            Formula::named(slide(), Dimension::Length),
            len(3.0),
            len(SLAB_HEIGHT),
        ],
        axis: [0.0, 0.0, 1.0].map(scl),
        angle: Formula::named(spin(), Dimension::Angle),
    });
    let opts = p.opts();
    let (doc, mate) = constrain_through(
        doc,
        bolts[0],
        seat(head(p.bolt_foot(bolts[0])), head(p.slab_top(slab)), frame),
        &opts,
    );
    Slid {
        doc,
        opts,
        bolt: bolts[0],
        mate,
    }
}

/// **A seed on a mate frame's offset moves the mated part in that
/// lane**: seeded on the slide, the bolt moves by `(1, 0, 0)`; seeded on
/// the turn, each vertex by its `f64` builds' central difference — the
/// frame's own parameters, a third set of names a run binds beside a
/// placer's and a world pin's.
#[test]
fn a2_a_seed_on_a_mate_frames_offset_moves_the_mated_part() {
    let s = slid("msolve14-a2-offset-seed");
    let ev = run_at::<Dual64>(&s.doc, &seeded(&s.doc, &s.opts, slide()), None);
    assert!(
        ev.node_error(s.mate.placement).is_none(),
        "{:?}",
        ev.node_error(s.mate.placement)
    );
    for (name, t) in assert_tangents_match(
        &s.doc,
        &s.opts,
        &ev,
        s.bolt,
        slide(),
        SLIDE,
        "the slid bolt",
    ) {
        assert!(
            (t[0] - 1.0).abs() <= 1e-12 && t[1].abs() <= 1e-12 && t[2].abs() <= 1e-12,
            "∂B/∂slide at {name:?} is {t:?}"
        );
    }
    let ev = run_at::<Dual64>(&s.doc, &seeded(&s.doc, &s.opts, spin()), None);
    assert!(
        ev.node_error(s.mate.placement).is_none(),
        "{:?}",
        ev.node_error(s.mate.placement)
    );
    let turned = assert_turn_tangents_match(
        &s.doc,
        &s.opts,
        &ev,
        s.bolt,
        spin(),
        SPIN,
        "the turned bolt",
    );
    assert!(
        turned.iter().any(|(_, t)| t[0].abs() + t[1].abs() > 0.1),
        "the turn moves the bolt in the plane"
    );
}

/// **A box on a mate frame's offset encloses the mated part**: over a
/// wide box on either parameter, the solve's pose of the bolt encloses
/// its `f64` pose at both corners and the nominal; over a narrow box on
/// the slide, so does every vertex of the evaluated bolt.
#[test]
fn a2_a_box_on_a_mate_frames_offset_encloses_the_mated_part() {
    let s = slid("msolve14-a2-offset-box");
    let width = assert_pose_encloses_corners(
        &s.doc,
        &s.opts,
        s.bolt,
        slide(),
        SLIDE,
        (-0.25, 0.25),
        "the slid box, at the solve",
    );
    assert!(width >= 0.5, "the slide widens the pose: {width}");
    assert_pose_encloses_corners(
        &s.doc,
        &s.opts,
        s.bolt,
        spin(),
        SPIN,
        (-0.1, 0.1),
        "the turned box, at the solve",
    );
    let (lo, hi) = narrow(0.05);
    let ev = run_at::<Interval>(&s.doc, &boxed(&s.doc, &s.opts, slide(), lo, hi), None);
    assert!(
        ev.node_error(s.mate.placement).is_none(),
        "{:?}",
        ev.node_error(s.mate.placement)
    );
    assert_encloses_corners(
        &s.doc,
        &s.opts,
        &ev,
        s.bolt,
        (slide(), SLIDE),
        (lo, hi),
        "the slid box, evaluated",
    );
}

// ---- C2: a clocking with no radius to turn ----

/// **`placement` is refused as overconstrained, naming both of the
/// shaft's bores, or escalates**: the bundle pins the bolt with the two
/// coaxial bores on one line, either of which the rest already fix, so
/// the solve refuses it naming each ([`MateFault::Overconstrained`]);
/// where a decision on the way could not settle, the fault is
/// `Indeterminate` over a margin of `kind` (a point at `f64` and
/// `Dual64`, an enclosure at `Interval` — never a poisoned one), on the
/// placement's own log. Answers whether it escalated.
///
/// [`MateFault::Overconstrained`]: editor_core::MateFault::Overconstrained
fn overconstrained_or_escalates<T: geom_core::Decide>(
    ev: &Evaluation<T>,
    s: &Shaft,
    kind: geom_core::MarginKind,
    what: &str,
) -> bool {
    let error = ev
        .node_error(s.bolt)
        .unwrap_or_else(|| panic!("{what}: the shaft's bundle is refused"));
    if let Some(diag) = escalation(ev, s.bolt) {
        assert_eq!(diag.margin.kind(), kind, "{what}: {diag:?}");
        assert!(
            !error.escalations.is_empty(),
            "{what}: the escalation is on the placement's log"
        );
        return true;
    }
    assert!(
        matches!(
            &error.kind,
            editor_core::NodeErrorKind::Mate(fault)
                if matches!(
                    &**fault,
                    editor_core::MateFault::Overconstrained { placement, constraints, .. }
                        if *placement == s.bolt
                            && constraints.contains(&s.mates[0].id)
                            && constraints.contains(&s.mates[1].id)
                )
        ),
        "{what}: the bundle is refused overconstrained, naming both bores: {error:?}"
    );
    false
}

/// **A shaft in two coaxial bores is overconstrained in every lane**
/// ([`shaft`]), clocked either way: the fold meets the clocking with no
/// radius to turn, and the bundle then pins the bolt with two bores
/// either of which the rest already fix, so the solve refuses it naming
/// both — at `f64`, at `Dual64` seeded on `rise` and on `idle`, and at
/// `Interval` over a box on `rise` a fraction of ε wide, save where the
/// corpus's identically-zero membership margins escalate unboxed
/// ([`INTERVAL_ESCALATIONS`]), where the bolt's fault is that
/// escalation instead.
#[test]
fn c2_a_shaft_in_two_coaxial_bores_is_overconstrained_in_every_lane() {
    use geom_core::MarginKind;
    let eps = Tol::witness().eps();
    for (label, shape) in [
        ("shaft-clocked-rider", ShaftShape::ClockedRider),
        ("shaft-parallel-pin", ShaftShape::ParallelPin),
    ] {
        let s = shaft(&format!("msolve14-c2-{label}"), shape, BORES);
        let f = run_at::<f64>(&s.doc, &s.opts, None);
        assert!(
            !overconstrained_or_escalates(&f, &s, MarginKind::Value, &format!("{label}, f64")),
            "{label}: f64 decides"
        );
        for param in [rise(), idle()] {
            let d = run_at::<Dual64>(&s.doc, &seeded(&s.doc, &s.opts, param.clone()), None);
            assert!(
                !overconstrained_or_escalates(
                    &d,
                    &s,
                    MarginKind::Value,
                    &format!("{label}, Dual64 seeded on {param:?}")
                ),
                "{label}: Dual64 decides"
            );
        }
        let escalates = INTERVAL_ESCALATIONS
            .iter()
            .any(|(e, l, _)| e.to_bits() == eps.to_bits() && *l == label);
        let (lo, hi) = narrow(0.05);
        let i = run_at::<Interval>(&s.doc, &boxed(&s.doc, &s.opts, rise(), lo, hi), None);
        assert_eq!(
            overconstrained_or_escalates(
                &i,
                &s,
                MarginKind::Enclosure,
                &format!("{label}, Interval")
            ),
            escalates,
            "{label}: Interval escalates exactly where the corpus's escalations say"
        );
    }
}

/// **A near-degenerate shaft decides or escalates, and never reads a
/// NaN**: its bores a rounding apart (non-dyadic points on one line), or
/// three ε apart, inside the band. The in-band pair escalates at the
/// constraint door, which folds the bundle a constraint joins: the
/// fault is `Indeterminate` over a point margin. The rounding pair is
/// admitted, and in every lane — `f64`, `Dual64` unseeded and seeded on
/// `rise`, `Interval` unboxed — the bolt's bundle is refused
/// overconstrained or escalates on its own log, over a point margin or
/// an enclosure and never a poisoned one, the same way at `f64` and
/// `Dual64`.
#[test]
fn c2_a_near_degenerate_shaft_decides_or_escalates_and_never_reads_nan() {
    use geom_core::MarginKind;
    let eps = Tol::witness().eps();
    let rounding: Bores = (
        [[0.1, 0.7, 0.0], [0.1, 0.7, 2.3]],
        [[4.3, 4.1, SLAB_HEIGHT], [4.3, 4.1, -1.3]],
    );
    let (bolt_at, [first, second]) = BORES;
    let in_band: Bores = (
        bolt_at,
        [first, [second[0] + 3.0 * eps, second[1], second[2]]],
    );
    for (variant, bores) in [("rounding", rounding), ("in-band", in_band)] {
        for shape in [ShaftShape::ClockedRider, ShaftShape::ParallelPin] {
            let label = format!("{variant} {shape:?}");
            let s = match try_shaft(
                &format!("msolve14-c2-near-{variant}-{shape:?}"),
                shape,
                bores,
            ) {
                Ok(s) => s,
                Err(fault) => {
                    assert_eq!(variant, "in-band", "{label}: the door refused: {fault:?}");
                    let editor_core::MateFault::Indeterminate { diag, .. } = &fault else {
                        panic!("{label}: the door escalates: {fault:?}");
                    };
                    assert_eq!(diag.margin.kind(), MarginKind::Value, "{label}: {diag:?}");
                    continue;
                }
            };
            let f = run_at::<f64>(&s.doc, &s.opts, None);
            let escalated =
                overconstrained_or_escalates(&f, &s, MarginKind::Value, &format!("{label}, f64"));
            for opts in [s.opts.clone(), seeded(&s.doc, &s.opts, rise())] {
                let d = run_at::<Dual64>(&s.doc, &opts, None);
                assert_eq!(
                    overconstrained_or_escalates(
                        &d,
                        &s,
                        MarginKind::Value,
                        &format!("{label}, Dual64 {:?}", opts.seed),
                    ),
                    escalated,
                    "{label}: Dual64 decides where f64 does"
                );
            }
            let i = run_at::<Interval>(&s.doc, &s.opts, None);
            overconstrained_or_escalates(
                &i,
                &s,
                MarginKind::Enclosure,
                &format!("{label}, Interval"),
            );
        }
    }
}

// ---- A1: a face frame resolves on every lane ----

/// **A face-framed mate on a `Dual64` seed run resolves and carries its
/// pose's tangent**: the bolt seated through copy #2 on the slab cap's
/// OWN pose moves by `−2` per unit of spacing, as the authored seat
/// does, held to the `f64` builds' central difference.
#[test]
fn a1_a_face_frame_on_a_seed_run_carries_the_poses_tangent() {
    let b = bolted("msolve14-a1-seed", MateFrame::from_face());
    let ev = run_at::<Dual64>(&b.doc, &seeded(&b.doc, &b.opts, spacing()), None);
    assert!(
        ev.node_error(b.mate.placement).is_none(),
        "the face frame resolves at Dual64: {:?}",
        ev.node_error(b.mate.placement)
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
        unseeded.node_error(b.mate.placement).is_none(),
        "{:?}",
        unseeded.node_error(b.mate.placement)
    );
}

/// **A face-framed mate on an `Interval` box run resolves to an
/// enclosure of the pose at every box corner**: at the solve over a wide
/// box, and through the evaluation over a narrow one, where the slab —
/// whose face the frame reads — holds still.
#[test]
fn a1_a_face_frame_on_a_box_run_encloses_the_pose_at_every_corner() {
    let b = bolted("msolve14-a1-box", MateFrame::from_face());
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
    let ev = run_at::<Interval>(&b.doc, &boxed(&b.doc, &b.opts, spacing(), lo, hi), None);
    assert!(
        ev.node_error(b.mate.placement).is_none(),
        "the face frame resolves at Interval: {:?}",
        ev.node_error(b.mate.placement)
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
    let b = bolted("msolve14-a4", MateFrame::from_face());
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
    let pass = run_at::<Dual64>(&b.doc, &seeded(&b.doc, &b.opts, spacing()), Some(&base));
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

/// What a lane's evaluation says about the structure: each placement's
/// value kind and space, or the class of its fault ([`fault_class`]).
fn structure<T: editor_core::EvalScalar>(
    doc: &ProfileDoc,
    ev: &Evaluation<T>,
) -> Vec<(RecipeNodeId, String)> {
    doc.placements()
        .into_iter()
        .map(|id| {
            let said = match ev.result(id) {
                Some(NodeResult::Ok(v)) => {
                    format!("{} in {:?}", v.payload.kind_name(), ev.space(id))
                }
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
    let variant = |debug: String| {
        debug
            .split(|c: char| !c.is_alphanumeric())
            .next()
            .unwrap_or_default()
            .to_string()
    };
    match r {
        NodeResult::Failed(e) => match &e.kind {
            editor_core::NodeErrorKind::Mate(fault) => {
                format!("Mate::{}", variant(format!("{fault:?}")))
            }
            other => variant(format!("{other:?}")),
        },
        NodeResult::Poisoned { .. } => "Poisoned".to_string(),
        NodeResult::Ok(_) => unreachable!("an Ok result has a value, not a fault class"),
    }
}

/// **Every escalation an unboxed `Interval` run of the corpus makes**:
/// the ε row, the document, and the predicate whose enclosure straddled
/// the band. An identically-zero membership margin (the constructed
/// candidate's own residual) encloses zero only as tightly as the
/// lane's rounding, so at a fine enough ε it escalates with no box at
/// all (the spec's §5 (b),
/// `work/msolve/an-identically-zero-margin-escalates-at-a-fine-eps.md`):
/// at ε = 1e-12, five of the thirteen documents (the clocked coaxial
/// pair, both pin documents and both shafts), and none at the coarser
/// rows.
const INTERVAL_ESCALATIONS: [(f64, &str, &str); 5] = [
    (
        1e-12,
        "coaxial-clocked-rest",
        "mate_member_rotation_identity",
    ),
    (1e-12, "two-pin", "mate_member_axis_fixed"),
    (1e-12, "cross-pin", "mate_member_axis_fixed"),
    (
        1e-12,
        "shaft-clocked-rider",
        "mate_member_rotation_identity",
    ),
    (1e-12, "shaft-parallel-pin", "mate_member_axis_fixed"),
];

/// The escalation `id` fails with in `ev`, if its fault is one: a mate
/// fault `Indeterminate`.
fn escalation<T: geom_core::Decide>(
    ev: &Evaluation<T>,
    id: RecipeNodeId,
) -> Option<geom_core::predicate::Indeterminate> {
    use editor_core::{MateFault, NodeErrorKind};
    match &ev.node_error(id)?.kind {
        NodeErrorKind::Mate(fault) => match &**fault {
            MateFault::Indeterminate { diag, .. } => Some(**diag),
            _ => None,
        },
        _ => None,
    }
}

/// **An interval lane's structure is the `f64` one's, or one of the
/// corpus's own escalations** ([`INTERVAL_ESCALATIONS`]): where an
/// interval decision could not settle, the placement faults
/// `Indeterminate` over an enclosure (never a poisoned margin), under a
/// predicate the table names for this document at this ε; the
/// escalation is on a placement's own log; everything else reads as at
/// `f64`. A document the table names escalates.
fn assert_interval_structure(
    doc: &ProfileDoc,
    f: &Evaluation<f64>,
    i: &Evaluation<Interval>,
    label: &str,
) {
    use geom_core::MarginKind;
    let eps = Tol::witness().eps();
    let allowed: Vec<&str> = INTERVAL_ESCALATIONS
        .iter()
        .filter(|(e, l, _)| e.to_bits() == eps.to_bits() && *l == label)
        .map(|(_, _, predicate)| *predicate)
        .collect();
    let log_escalated = fixture::constrained(doc)
        .into_iter()
        .any(|id| match i.result(id) {
            Some(NodeResult::Ok(v)) => !v.escalations.is_empty(),
            _ => i.node_error(id).is_some_and(|e| !e.escalations.is_empty()),
        });
    let want = structure(doc, f);
    let got = structure(doc, i);
    let mut moved = Vec::new();
    for ((id, w), (_, g)) in want.iter().zip(&got) {
        if w == g {
            continue;
        }
        let diag = escalation(i, *id).unwrap_or_else(|| {
            panic!(
                "{label}: Interval reads {g} at {id:?} where f64 reads {w}, and its fault is no \
                 escalation"
            )
        });
        assert_eq!(
            diag.margin.kind(),
            MarginKind::Enclosure,
            "{label}: the escalation at {id:?} is over an enclosure, not a poisoned margin: {diag:?}"
        );
        assert!(
            diag.predicate.is_some_and(|p| allowed.contains(&p)),
            "{label}: Interval escalates at {id:?} on {:?} at ε = {eps:e}, which the corpus's \
             escalations do not list ({allowed:?})",
            diag.predicate
        );
        moved.push(*id);
    }
    assert_eq!(
        moved.is_empty(),
        allowed.is_empty(),
        "{label}: at ε = {eps:e} the corpus's escalations list {allowed:?}, and Interval \
         escalated at {moved:?}"
    );
    if !moved.is_empty() {
        assert!(
            log_escalated,
            "{label}: Interval escalated at {moved:?} with no escalation on a placement's log"
        );
    }
}

/// **C5 and A3's dual half**: over the whole corpus, the solve's
/// structure — every placement's space, and which placements fail —
/// is the `f64` build's at `Dual64` (unseeded and under every seed) and
/// at `Interval` (unboxed), save where an interval decision ESCALATES
/// ([`assert_interval_structure`]); and at `Dual64` every instance's
/// every vertex reads the `f64` build's bits on its value channel and a
/// finite tangent.
#[test]
fn c5_one_documents_structure_is_the_same_in_every_lane_and_the_dual_value_is_f64s() {
    for (label, doc, opts) in corpus() {
        let f = run_at::<f64>(&doc, &opts, None);
        let d = run_at::<Dual64>(&doc, &opts, None);
        let i = run_at::<Interval>(&doc, &opts, None);
        let want = structure(&doc, &f);
        assert_eq!(structure(&doc, &d), want, "{label}: Dual64's structure");
        assert_interval_structure(&doc, &f, &i, label);
        // Every free continuous variable is a seed; a defined one takes
        // none (its derivative is its inputs').
        let params: Vec<editor_core::VarId> = doc
            .free_vars()
            .filter(|(_, free)| matches!(free, editor_core::FreeVar::Continuous { .. }))
            .map(|(id, _)| id)
            .collect();
        let seeds = std::iter::once(None).chain(params.into_iter().map(Some));
        for seed in seeds {
            let o = EvalOptions {
                seed,
                profile_lift: ProfileLift::Guided,
                ..opts.clone()
            };
            let d = run_at::<Dual64>(&doc, &o, None);
            assert_eq!(structure(&doc, &d), want, "{label}: seeded {seed:?}");
            for id in doc.ids() {
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
                    assert!(
                        [p.x.deriv, p.y.deriv, p.z.deriv]
                            .iter()
                            .all(|t| t.is_finite()),
                        "{label}, seed {seed:?}: the tangent at {name:?} is finite: {:?}",
                        [p.x.deriv, p.y.deriv, p.z.deriv]
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
    use editor_core::stackup::{SensitivityOutcome, SensitivityRefusal, sensitivities};
    use editor_core::{MeasurePrimitive, SitedRef};
    let b = bolted("msolve14-a5-stackup", MateFrame::from_face());
    let ev = run_at::<f64>(&b.doc, &b.opts, None);
    let foot = all_vertices(&ev, b.bolt)[0].clone();
    let corner = all_vertices(&ev, b.slab)[0].clone();
    let (doc, m) = crate::fixture::measure_node(
        &b.doc,
        MeasurePrimitive::Distance { a: 0, b: 1 },
        vec![
            SitedRef::new(b.bolt, foot.clone()),
            SitedRef::new(b.slab, corner.clone()),
        ],
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
    // The spacing toleranced, so the driver varies it (VR8: an
    // untoleranced variable is a constant of the analysis).
    let spacing = doc.var_named("s").expect("the spacing parameter");
    let (doc, _) = crate::fixture::step(
        doc,
        editor_core::DocEdit::SetVarDistribution {
            var: spacing.into(),
            distribution: Some(editor_core::Distribution::Normal { sigma: 1e-4 }),
        },
    );
    let entries = sensitivities(
        &doc,
        crate::fixture::output(&doc, m),
        None,
        None,
        false,
        Some(&resolver),
        Tol::witness(),
    )
    .expect("the driver runs");
    assert_eq!(entries.len(), 1, "one entry, the toleranced spacing's");
    let entry = entries
        .iter()
        .find(|e| e.param == spacing)
        .expect("an entry for the spacing");
    match &entry.outcome {
        SensitivityOutcome::Derivative { value, .. } => assert!(
            (value - fd).abs() <= 1e-12,
            "∂m/∂s is {value} where the closed form gives {fd}"
        ),
        other => panic!("a derivative across the face frame, not {other:?}"),
    }
    // A record build over no resolver is not this build: its instances
    // refuse where the anchor's resolve, and the pairing gate says so.
    let unresolved = run_at::<f64>(&doc, &EvalOptions::default(), None);
    assert!(
        matches!(
            sensitivities(
                &doc,
                crate::fixture::output(&doc, m),
                Some(&unresolved),
                None,
                false,
                Some(&resolver),
                Tol::witness()
            ),
            Err(SensitivityRefusal::Pairing(_))
        ),
        "a record build over another resolver fails the pairing gate"
    );
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
    let b = bolted("msolve14-a5-clearance", MateFrame::from_face());
    let resolver = b.opts.resolver.clone().expect("the store resolves");
    let (lo, hi) = narrow(0.05);
    let leaf = ParamBox::from_axes(BTreeMap::from([(
        var_of(&b.doc, &spacing()),
        BoxAxis::Varying { lo, hi },
    )]));
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
    // The verdicts alone cannot tell a solve at the leaf's scalar from
    // one held at the nominal: the leaf is a fraction of ε (the
    // placement door's limit) and they are decided against ε. The
    // leaf's own evaluation can: the bolt moves by −2 per unit of `s`,
    // so its enclosure holds the f64 builds' bolt at both corners, which
    // a bolt held at its nominal pose would not.
    let ev = run_at::<Interval>(&b.doc, &boxed(&b.doc, &b.opts, spacing(), lo, hi), None);
    assert_encloses_corners(
        &b.doc,
        &b.opts,
        &ev,
        b.bolt,
        (spacing(), SPACING),
        (lo, hi),
        "the clearance leaf",
    );
}
