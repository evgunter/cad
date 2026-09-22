//! **The four-link chain's DOCUMENT** — an angular error at every
//! joint, authored once and read by both of its cells.
//!
//! It lives beside [`crate::plate`] and for the same reason: the
//! picture and the certified table must be about the same part.
//! [`crate::mcchain`] draws the population the advisory lane averages
//! over; [`crate::chaintol`] measures what the certified lane can say
//! about it. A second transcription of the chain would let the two
//! drift into being about two different studies.
//!
//! The split is also what makes the density cell REACHABLE: the
//! Monte-Carlo lane is pure `f64` replay and is ungated on purpose
//! (`crates/pncad/src/analysis.rs`), so the document it replays cannot
//! sit inside an `interval`-gated module.
//!
//! # The kinematics, and the door that expresses it
//!
//! Joint `k` carries a parameter `joint_k` at [`Dimension::Angle`],
//! and joint `k`'s error has to move links `k..n` — that is what
//! "down the chain" means. The document says so STRUCTURALLY: link
//! `k` is the bar profile under `k` nested [`Node::Transform`]s, the
//! innermost carrying `joint_k` and the outermost `joint_1`, so the
//! sub-chain downstream of a joint is literally downstream of that
//! joint's node. Nothing here transcribes a forward-kinematic sum.
//!
//! `Node::Transform` rotates about an axis THROUGH THE WORLD ORIGIN
//! and then translates (`eval::wire`'s `transform_map`), which is
//! exactly a joint: the map that carries a point of link `k`'s own
//! frame into link `k-1`'s frame is "rotate by `joint_k` about the
//! joint, then step one link length along the parent". So `T_k` is
//! `rotate joint_k, translate (L, 0, 0)` for `k >= 2`, `T_1` is
//! `rotate joint_1, translate 0` (joint 1 is AT the origin), and link
//! `k` is `T_1(T_2(...T_k(bar)))`.
//!
//! **Why not a [`Datum::Frame`] per link whose `u`/`v` carry
//! `Expr::cos`/`sin` of the partial sum `joint_1 + … + joint_k`.**
//! That door works, and it is the one a user reaches for when they
//! have already done the kinematics on paper — which is the objection
//! to it here twice over. It puts the forward kinematics in the
//! author's algebra rather than in the document's shape, so "joint 2
//! moves links 2, 3 and 4" becomes a property of four hand-written
//! expressions instead of a property of the graph; and on the
//! certified lane it is strictly the longer argument, because a
//! frame's `u` and `v` are ORTHONORMALISED at evaluation. On
//! intervals `cos(θ)² + sin(θ)²` is not 1, so that normalisation is a
//! `sqrt` of a bracket around 1 and two divides by it, per link, on
//! top of the `sin`/`cos` the study actually contains. The transform
//! door's rotation axis is the literal `+z`, so the ONLY widened
//! quantity in each joint's map is the `sin`/`cos` of that joint's
//! angle — which is the quantity
//! `work/sym/a-widened-rotation-angle-is-unmeasured-on-the-certified-lane`
//! names, measured here rather than padded first.
//!
//! # What the chain is made of
//!
//! Four bars of one length and one section, and a PIN at every joint
//! plus one at the tip — the shape a real chain has, and the reason
//! the sheet can read a joint's position off a built body exactly:
//! a pin's `Surface::Cylinder` carries its centre, the way the
//! plate's holes do. The pins are separate extrudes rather than
//! bores, which is [`crate::plate`]'s own convention: the study is
//! about where the pins ARE, and a boolean between a bar and its pin
//! would add a subject the study does not have.
//!
//! The TARGET is a fixed pin at the chain's nominal tip — the mating
//! hole the last link has to reach. It carries no parameter, so it is
//! the datum the tip's position is measured against, and the measure
//! is the distance between the two pins' axes: a true-position error,
//! nominally zero, first order in every joint.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use pncad::document::{
    AssertionDir, CancelToken, Datum, Dimension, Distribution, DocEdit, DocParam, DocumentId,
    EvalOptions, Evaluation, Expr, LoopProgram, MeasureExpr, MeasurePrimitive, Node, ParamName,
    ProfileDoc, ProfileProgram, RecipeNodeId, RefusingReach, SitedRef, apply, evaluate,
};
use pncad::geom_core::Tol;
use pncad::select::{EntityKind, GeomPred, NamePat, Selector, SurfaceKindSet, select_where};

/// The nominal link length, in metres (12 mm) — one joint pin to the
/// next.
pub const LINK_LENGTH: f64 = 12.0e-3;
/// The bar's section height, in metres (3 mm).
pub const LINK_HEIGHT: f64 = 3.0e-3;
/// The bar's thickness, in metres (2 mm) — the extrude distance, and
/// the one dimension the sheet does not draw.
pub const LINK_THICKNESS: f64 = 2.0e-3;
/// The joint pin's radius, in metres (0.8 mm).
pub const PIN_RADIUS: f64 = 0.8e-3;

/// How many links the demo's chain has. Ev's request says "4ish"; the
/// certified table runs the same document at 1, 2, 3 and 4.
pub const LINKS: usize = 4;

/// **The study: σ = 0.01 rad (0.57°) on every joint, independently.**
///
/// ONE law at every joint, because the joints are one design of joint:
/// a study with four different σ would be about a chain someone built
/// out of four different parts, and the thing being shown — that the
/// SAME error, four times, disperses more and more — would be
/// confounded with it.
///
/// Normal rather than the machinist's uniform ±, which is what
/// [`crate::plate`]'s spacing carries. Two reasons, both about this
/// study rather than about taste. A joint's angular error is the sum
/// of many small independent ones (the pin's fit in each of the two
/// bores, the bores' own positions, the faces they were cut against),
/// so the normal is the honest shape and its accumulation law is the
/// one the picture is about: the lateral spread at pin `k` is
/// `L·σ·sqrt(Σ_{j<k} (k−j)²)`, which grows as `1 : 2.24 : 3.74 :
/// 5.48` over four links and is a prediction the sheet's own numbers
/// check. And the analyzed box of a normal is ±3σ, so a tail stays
/// OUTSIDE the box — the trade E11 names, which a uniform's box (the
/// whole support) would have quietly retired.
pub const JOINT_SIGMA: f64 = 1.0e-2;

/// **The requirement: the tip pin within 1 mm of the target.** A
/// true-position tolerance, the shape a real assembly drawing states
/// it in. Sized so the advisory lane says something: at [`JOINT_SIGMA`]
/// the tip's lateral σ is `L·σ·sqrt(30)` = 0.657 mm, so a 1 mm band is
/// neither always met nor never met.
pub const POSITION_BOUND: f64 = 1.0e-3;

/// **The widest box of this chain that certifies whole, as a fraction
/// of the real study.**
///
/// MEASURED by [`crate::chaintol`], not chosen: its module header
/// carries the measurement and what bounds it. Named here because
/// [`crate::mcchain`] draws it to scale, and a number a picture is
/// built around should not be a literal buried in the drawing code.
pub const CERTIFIABLE_FRACTION: f64 = 0.0;

/// The parameter name of joint `k` (`k` is 1-based, joint 1 at the
/// base). One spelling, read by the document, the sheet and the
/// certified table alike.
pub fn joint_name(k: usize) -> String {
    format!("joint_{k}")
}

fn len(v: f64) -> Expr {
    Expr::literal(v, Dimension::Length).expect("finite length")
}

fn scl(v: f64) -> Expr {
    Expr::literal(v, Dimension::Scalar).expect("finite scalar")
}

fn insert(doc: &mut ProfileDoc, node: Node<ProfileProgram>, tol: Tol) -> RecipeNodeId {
    let applied =
        apply(doc, &DocEdit::InsertNode { node }, tol, &RefusingReach).expect("the insert applies");
    *doc = applied.doc;
    applied.record.minted.expect("an insert mints an id")
}

fn declare(doc: &mut ProfileDoc, n: &str, value: f64, distribution: Distribution, tol: Tol) {
    let applied = apply(
        doc,
        &DocEdit::SetDocParam {
            name: ParamName::new(n),
            value: DocParam::continuous_with(Dimension::Angle, value, distribution),
        },
        tol,
        &RefusingReach,
    )
    .expect("the parameter applies");
    *doc = applied.doc;
}

/// The chain, its pins, the target, the position measure and the
/// assertion — authored the way a user would.
pub struct Chain {
    /// The document itself.
    pub doc: ProfileDoc,
    /// The tip-position `Measure` node — `distance(tip pin, target)`.
    pub measure: RecipeNodeId,
    /// The `Assertion` over it. Read by [`crate::chaintol`], which is
    /// behind the `interval` feature, so a default build legitimately
    /// has no consumer for it — the field is part of the document
    /// either way and a cell that dropped it would be describing a
    /// different one.
    #[cfg_attr(not(feature = "interval"), allow(dead_code))]
    pub assertion: RecipeNodeId,
    /// The placed bars, base first. Carried because a cell that DRAWS
    /// the study needs the built bodies and not only the summary over
    /// them.
    pub bars: Vec<RecipeNodeId>,
    /// The placed joint pins, base first: `links + 1` of them, pin `k`
    /// at the base of link `k` and the last at the tip.
    pub pins: Vec<RecipeNodeId>,
}

/// The chain document at `links` links, with `joint_sigma` on every
/// joint and `bound` on the tip's position.
///
/// `links` is a parameter of the AUTHOR, not of the document: the
/// certified table walks 1, 2, 3 and 4 links to watch a cost and a
/// refusal move, and a chain of `n` links is a different document
/// rather than the same one at a different value.
pub fn chain(links: usize, joint_sigma: f64, bound: f64, tol: Tol) -> Chain {
    assert!(links >= 1, "a chain has at least one link");
    let mut doc = ProfileDoc::empty(DocumentId::derive("pncad-demo-chain"), tol);
    for k in 1..=links {
        declare(
            &mut doc,
            &joint_name(k),
            0.0,
            Distribution::Normal { sigma: joint_sigma },
            tol,
        );
    }

    // The sketch plane: the chain lies in world xy and every bar is
    // extruded along +z, so the picture the sheet draws IS this plane.
    let plane = insert(
        &mut doc,
        Node::Datum(Datum::Frame {
            origin: [len(0.0), len(0.0), len(0.0)],
            u: [scl(1.0), scl(0.0), scl(0.0)],
            v: [scl(0.0), scl(1.0), scl(0.0)],
        }),
        tol,
    );

    // The three shapes the chain is built from, each authored ONCE at
    // the origin of a link's own frame and then placed. Literal
    // rectangles and circles: the study is about the JOINTS, and a
    // parameter nothing measures would be noise in the per-parameter
    // table.
    let h = LINK_HEIGHT / 2.0;
    let bar_profile = insert(
        &mut doc,
        Node::Profile(ProfileProgram {
            plane,
            loops: vec![
                LoopProgram::polygon([
                    (0.0, -h),
                    (LINK_LENGTH, -h),
                    (LINK_LENGTH, h),
                    (0.0, h),
                ])
                .expect("finite bar corners"),
            ],
        }),
        tol,
    );
    let bar = insert(
        &mut doc,
        Node::Extrude {
            profile: bar_profile,
            distance: len(LINK_THICKNESS),
        },
        tol,
    );
    let pin_at = |doc: &mut ProfileDoc, x: f64, tol| {
        let profile = insert(
            doc,
            Node::Profile(ProfileProgram {
                plane,
                loops: vec![LoopProgram::Circle {
                    centre: [len(x), len(0.0)],
                    radius: len(PIN_RADIUS),
                }],
            }),
            tol,
        );
        insert(
            doc,
            Node::Extrude {
                profile,
                distance: len(LINK_THICKNESS),
            },
            tol,
        )
    };
    let base_pin = pin_at(&mut doc, 0.0, tol);
    let tip_pin = pin_at(&mut doc, LINK_LENGTH, tol);

    // **The joint stack.** `place(k, what)` wraps `what` in joints
    // `k`, `k-1`, … 1, innermost first: joint `k`'s node is BELOW
    // joint `k-1`'s, so an error at joint `j` moves everything joints
    // `j+1..k` placed and nothing above it.
    let place = |doc: &mut ProfileDoc, k: usize, what: RecipeNodeId, tol| {
        let mut node = what;
        for j in (1..=k).rev() {
            let step = if j == 1 { 0.0 } else { LINK_LENGTH };
            node = insert(
                doc,
                Node::Transform {
                    input: node,
                    translation: [len(step), len(0.0), len(0.0)],
                    rotation_axis: [scl(0.0), scl(0.0), scl(1.0)],
                    rotation_angle: Expr::param(
                        ParamName::new(&joint_name(j)),
                        Dimension::Angle,
                    ),
                },
                tol,
            );
        }
        node
    };

    let bars: Vec<RecipeNodeId> = (1..=links).map(|k| place(&mut doc, k, bar, tol)).collect();
    // Pin `k` sits at the base of link `k`, so it rides link `k`'s own
    // stack; the last pin is link `links`'s TIP pin and rides the same
    // stack one shape over.
    let mut pins: Vec<RecipeNodeId> = (1..=links)
        .map(|k| place(&mut doc, k, base_pin, tol))
        .collect();
    pins.push(place(&mut doc, links, tip_pin, tol));

    // The target: the mating pin at the chain's nominal tip, fixed.
    let target = pin_at(&mut doc, links as f64 * LINK_LENGTH, tol);

    // The wall names come from the SELECTION door, the way a user gets
    // them: evaluate what is built so far, then ask each pin for its
    // cylindrical face.
    let ev: Evaluation<f64> = evaluate(
        &doc,
        None,
        &CancelToken::new(),
        &EvalOptions::default(),
        tol,
    );
    let wall = |node: RecipeNodeId| {
        let mut faces = select_where(
            &ev,
            node,
            &Selector::of(NamePat::of_kind(EntityKind::Face)),
            &[GeomPred::SurfaceKind(SurfaceKindSet::just(
                pncad::geom_brep::SurfaceKind::Cylinder,
            ))],
            &doc.param_env::<f64>(),
            tol,
        )
        .expect("the surface-kind atom is exact");
        faces.sort();
        assert!(!faces.is_empty(), "a pin extrude has a cylindrical wall");
        SitedRef::new(node, faces.remove(0))
    };

    // The tip's TRUE POSITION: the distance between the tip pin's axis
    // and the target's. The closed form for two cylinder faces is
    // their AXIS distance (its own contract), and both axes are `+z`,
    // so this is the tip pin's in-plane deviation from where the
    // drawing says it goes — no author's arithmetic on top of it.
    let position = MeasureExpr::primitive(MeasurePrimitive::Distance { a: 0, b: 1 });
    // The two references are read BEFORE the insert borrows the
    // document mutably — the borrow checker's way of saying that a
    // measure's references are resolved against a document that
    // already exists.
    let refs = vec![
        wall(*pins.last().expect("a chain has a tip pin")),
        wall(target),
    ];
    let measure = insert(
        &mut doc,
        Node::measure(position, refs).expect("both indices in range"),
        tol,
    );
    let assertion = insert(
        &mut doc,
        Node::Assertion {
            measure,
            bound: len(bound),
            dir: AssertionDir::AtMost,
        },
        tol,
    );
    Chain {
        doc,
        measure,
        assertion,
        bars,
        pins,
    }
}
