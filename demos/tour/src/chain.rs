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
//! certified lane it looks like the longer argument, because a frame's
//! `u` and `v` are ORTHONORMALISED at evaluation (`eval::wire`) and on
//! intervals `cos(θ)² + sin(θ)²` is not 1, so that normalisation is a
//! `sqrt` of a bracket around 1 and two divides by it, per link, on
//! top of the `sin`/`cos` the study actually contains. The transform
//! door's rotation axis is the literal `+z`, so the ONLY widened
//! quantity in each joint's map is the `sin`/`cos` of that joint's
//! angle — which is the quantity
//! `work/sym/a-widened-rotation-angle-is-unmeasured-on-the-certified-lane`
//! names, measured here rather than padded first.
//!
//! **The comparison is an argument from the two doors, not a
//! measurement**: the frame document was never built, so how much
//! narrower the transform door's box actually is has no number. What
//! the certified table DOES measure is that on the transform door the
//! plain interval lane refuses on exactly the column-unit check a
//! widened normalisation would run into
//! ([`crate::chaintol`]) — which is the shape of the argument showing
//! up, one door over, and not the argument being checked.
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
//!
//! # What was awkward to write, stated rather than smoothed over
//!
//! Per `memories/demo-purpose.md`, the awkwardness is the finding:
//!
//! 1. **There is no joint in the vocabulary.** A revolute joint is
//!    spelled as a rigid transform whose rotation axis the author has
//!    to remember passes through the WORLD origin, so the stack reads
//!    outermost-first and its kinematics has to be unfolded by the
//!    reader. And it does not SHARE: link `k`'s stack and link
//!    `k+1`'s agree on every joint but differ in their innermost
//!    operand, so there is no sub-expression the two have in common
//!    and a four-link chain with its pins is twenty-four transform
//!    nodes for four rigid bodies.
//! 2. **The measure cannot reach the tip end face's centre.** The
//!    closed form that gives a CENTRE exactly is cylinder × cylinder
//!    (`eval::measure`'s `distance`, the axis distance); a planar end
//!    face arrives as a plane carrier, whose point-plane distance is
//!    the chain's REACH — second order in the joint errors, so it
//!    barely disperses — and the face's own corner vertices have no
//!    exact selector, only the DECIDED `GeomPred::DatumDistance`,
//!    which would be re-decided on every one of 512 replays. Hence
//!    the pins: they make the quantity the study is about reachable
//!    through an exact door, and they are what a chain has anyway.
//! 3. **The selection door needs an evaluation mid-authoring** — the
//!    plate's note, unchanged: the document is evaluated once here,
//!    before the measure, purely to ask each pin for its cylindrical
//!    face's name.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use pncad::document::{
    AssertionDir, CancelToken, Datum, Dimension, Distribution, DocEdit, DocParam, DocumentId,
    EvalOptions, Evaluation, Expr, LoopProgram, MeasureExpr, MeasurePrimitive, Node, ParamName,
    ProfileDoc, ProfileProgram, RecipeNodeId, RefusingReach, SitedRef, apply, evaluate,
};
use pncad::geom::Surface;
use pncad::geom_core::Tol;
use pncad::select::{EntityKind, GeomPred, NamePat, Selector, SurfaceKindSet, select_where};
use pncad::topo::{Body, SurfaceKey};

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
///
/// **At the DEFAULT ε**, like every other measured number here: the
/// fraction moves with ε — `1.083e-1` at ε = 1e-6, measured. WHY it
/// moves is not established, and the cell says so: the refusal at the
/// wall is the wedge's poisoned margin, which a band does not
/// classify. So the cell asks at every ε whether this published box
/// still certifies there rather than reasoning about it.
pub const CERTIFIABLE_FRACTION: f64 = 1.110e-1;

/// **The same measurement at 1, 2, 3 and 4 links** — one number in
/// four spellings.
///
/// The tip's certified lateral half-width, `L · 3σ · f · n(n+1)/2` at
/// `n` links, is `3.998e-4` m at two, three and four links alike. What
/// that number IS, is half of [`PIN_RADIUS`] — a property of THIS
/// document's geometry, not of the tier: the ratio is
/// `0.500 / 0.500 / 0.499`, and MEASURED with the radius doubled to
/// `1.6e-3` m the fractions become `1.0000 / 0.73841 / 0.36921 /
/// 0.22192` and the half-width `7.975e-4` m — still `0.498` of the
/// radius. [`CERTIFIED_TIP_OVER_PIN_RADIUS`] pins it. The one-link row
/// is capped by the study itself rather than by the wall, and sits at
/// `0.450` of the radius.
///
/// (An earlier reading of this table said the constant thing was an
/// ANGLE — "about 1.9° of accumulated swing, however many joints it is
/// spread over". It is not: the swing doubles with the pin radius, to
/// `3.81°`. The invariance across link counts is real; the angle was
/// the shipped radius in disguise.)
///
/// MEASURED by [`crate::chaintol`] and pinned there;
/// [`CERTIFIABLE_FRACTION`] is the last row.
///
/// Read only by that cell, which is behind the `interval` feature, so
/// a default build legitimately has no consumer for it — the
/// measurement is part of this document's record either way, and the
/// sheet's own [`CERTIFIABLE_FRACTION`] is the last row of it.
#[cfg_attr(not(feature = "interval"), allow(dead_code))]
pub const CERTIFIABLE_FRACTION_BY_LINKS: [f64; LINKS] = [1.0, 3.702e-1, 1.851e-1, 1.110e-1];

/// **The tip's certified lateral half-width, over the pin radius** —
/// the same at every link count whose box the WALL sets, and the
/// number [`CERTIFIABLE_FRACTION_BY_LINKS`] is four spellings of.
///
/// MEASURED by [`crate::chaintol`] at 2, 3 and 4 links and pinned
/// there with a paste-ready re-baseline; it is not derived from the
/// two constants beside it, because what it asserts is that those two
/// stand in this ratio AT EVERY LINK COUNT, which neither of them
/// says. The one-link chain is excluded on purpose: its box is the
/// study, not the wall.
///
/// Read only by that cell, which is behind the `interval` feature.
#[cfg_attr(not(feature = "interval"), allow(dead_code))]
pub const CERTIFIED_TIP_OVER_PIN_RADIUS: f64 = 4.995e-1;

/// **The certified enclosure of each joint pin's centre at that box**
/// — `(half-width along the chain, half-width across it)`, in metres,
/// pin 1 (the base) first.
///
/// MEASURED by [`crate::chaintol`] over the box
/// [`CERTIFIABLE_FRACTION`] names, and pinned there. This is the
/// certified half of the picture Ev asked for: an enclosure per link,
/// growing down the chain, drawn by [`crate::mcchain`] beside the
/// advisory cloud. It is named here for the same reason
/// `CERTIFIABLE_FRACTION` is — a number a picture is built around
/// should not be a literal buried in the drawing code — and it is
/// ungated for the same reason too: the sheet is drawn in a default
/// build.
///
/// **The across-the-chain half-widths run 1 : 3 : 6 : 10**, which is
/// the lever sum at pin `k`, `Σ_{j=1..k−1} (k−j)` = `k(k−1)/2` — the
/// WORST-CASE one, every joint at its own extreme at once — while the
/// advisory σ at the same pins runs
/// `1 : 2.24 : 3.74 : 5.48`, the quadrature sum. That gap between a
/// linear sum and a root-sum-square is E11's subject, and on this
/// document it is visible on the sheet rather than only in a report.
/// The enclosures are TIGHT, not padded: `3.996e-5` m is exactly
/// `L · 3σ_c · 1` at the certified box's own σ, to every digit the
/// measurement carries.
pub const CERTIFIED_PIN_BOX: [(f64, f64); LINKS + 1] = [
    (0.0, 0.0),
    (6.653231802815351e-8, 3.995961969247516e-5),
    (3.3266085237848575e-7, 1.198788590774255e-4),
    (9.314487635844748e-7, 2.3975816125464358e-4),
    (1.995959949908921e-6, 3.9959841242371446e-4),
];

/// **The pin's axis, read off the body the kernel built** — ONE rule,
/// used by every cell that reads a pin back.
///
/// The rule is not "the first cylindrical face" or "the last" or "and
/// they had better agree numerically": a pin extrude has exactly one
/// cylindrical SURFACE, and the seam may split it across more than one
/// face. So the faces are gathered by their surface KEY, the keys are
/// required to be one, and that surface's stored origin is the answer.
/// That is exact at every scalar — it compares arena keys, never
/// coordinates — which is what lets the density sheet (`f64`) and the
/// certified cell (`Sym<Interval>`) share it. Three spellings of this
/// used to sit in three modules, two of them with rules that would
/// have disagreed on a seam-split cylinder.
///
/// The face NAME the measure's reference is sited at is a different
/// question, answered by the selection door in [`chain`]: any face on
/// that cylinder names the same carrier, which is exactly what this
/// function's one-surface assertion establishes.
pub fn pin_axis<T: pncad::geom_core::Real>(body: &Body<T>) -> (T, T) {
    let mut found: Option<(SurfaceKey, (T, T))> = None;
    for (_, face) in body.faces() {
        let Some(Surface::Cylinder { origin, .. }) = body.get_surface(face.surface) else {
            continue;
        };
        match found {
            Some((key, _)) => assert_eq!(
                key, face.surface,
                "a pin extrude's cylindrical faces lie on ONE surface; two surface keys here \
                 means this body is not a pin"
            ),
            None => found = Some((face.surface, (origin.x, origin.y))),
        }
    }
    found.expect("a pin extrude has a cylindrical wall").1
}

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
                LoopProgram::polygon([(0.0, -h), (LINK_LENGTH, -h), (LINK_LENGTH, h), (0.0, h)])
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
    // `k`, `k-1`, … 1, innermost first. Joint `j`'s node therefore
    // sits ABOVE every joint below it in the chain and BELOW every
    // joint above it, which is the kinematics: joint `j` turns links
    // `j..n` and leaves links `1..j` where they were.
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
                    rotation_angle: Expr::param(ParamName::new(joint_name(j)), Dimension::Angle),
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
        // WHICH face does not matter: they lie on one cylinder, which
        // is [`pin_axis`]'s asserted rule, and a measure's carrier is
        // the surface and not the face. Sorted first so the choice is
        // the same on every run.
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
