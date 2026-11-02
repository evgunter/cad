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
//! # The kinematics, and the door that expresses it
//!
//! Joint `k` carries a parameter `joint_k` at [`Dimension::Angle`],
//! and joint `k`'s error has to move links `k..n` — that is what
//! "down the chain" means. The document says so STRUCTURALLY: link 1
//! is a copy of the bar placed in the world, its spin `joint_1`; link
//! `k` is a copy placed against link `k-1`'s copy, its start cap mated
//! to the parent's, slid one link length along it and spun by
//! `joint_k`. So link `k`'s placement reads link `k-1`'s, the
//! sub-chain downstream of a joint is literally downstream of that
//! joint's placement, and nothing here transcribes a forward-kinematic
//! sum.
//!
//! A value is charted on the two bodies' own frames: a spin turns the
//! copy about its own origin, which is the bar's joint end, and a slide
//! moves that origin along the parent's own axes. That is exactly a
//! joint: "rotate by `joint_k` about the joint, then step one link
//! length along the parent".
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
//! 1. **There is no joint in the vocabulary.** A revolute joint is a
//!    copy placed against its parent's copy plus three values, and the
//!    spin's sense is the chart's: about the mated caps' outward
//!    normal `−z`, so a turn of `joint_k` about `+z` is written
//!    `−joint_k` — except at the base, where the "place part" gesture
//!    flips the start cap onto the world's `+z` and `joint_1` is written
//!    as it is.
//! 2. **The measure cannot reach the tip end face's centre.** The
//!    closed form that gives a CENTRE exactly is cylinder × cylinder
//!    (`eval::measure`'s `distance`, the axis distance); a planar end
//!    face arrives as a plane carrier, whose point-plane distance is
//!    the chain's REACH — second order in the joint errors, so it
//!    barely disperses — and the face's own corner vertices have no
//!    exact selector, only the DECIDED `GeomPred::PlaneDistance`,
//!    which would be re-decided on every one of 512 replays. Hence
//!    the pins: they make the quantity the study is about reachable
//!    through an exact door, and they are what a chain has anyway.
//! 3. **The selection door needs an evaluation mid-authoring** — the
//!    plate's note, unchanged: the document is evaluated once here,
//!    before the measure, purely to ask each pin for its cylindrical
//!    face's name.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use pncad::document::{
    AssertionRelation, CancelToken, Coord, Dimension, Distribution, DocEdit, DocumentId,
    EvalOptions, Evaluation, Formula, FreeVar, LoopProgram, MeasurePrimitive, Node, ProfileDoc,
    ProfileProgram, RecipeNodeId, SitedRef, TargetAxis, VarName, apply, evaluate,
};
use pncad::geom::Surface;
use pncad::geom_core::Tol;
use pncad::prelude::AuthoredNode;
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
/// fraction moves with ε — `1.083e-1` at ε = 1e-6, measured — for the
/// reason `chaintol`'s header gives ("What sets the wall"). So the
/// cell asks at every ε whether this published box still certifies
/// there rather than reasoning about it.
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
/// Read only by that cell; the sheet's own [`CERTIFIABLE_FRACTION`] is
/// the last row of it.
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
/// Read only by that cell.
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
/// measurement carries. Along the chain they are microns: the sheet
/// draws each side under `mcchain`'s pixel floor AT that floor,
/// centred on the pin, says in its legend which sides are floored, and
/// prints the true half-widths in its table.
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
pub fn joint_name(k: usize) -> VarName {
    VarName::new(format!("joint_{k}")).expect("joint_<k> is one identifier")
}

fn len(v: f64) -> Formula {
    Formula::literal(v, Dimension::Length).expect("finite length")
}

fn insert(doc: &mut ProfileDoc, node: AuthoredNode, tol: Tol) -> RecipeNodeId {
    let applied = apply(
        doc,
        &DocEdit::InsertNode {
            node: Box::new(node),
            fresh: Vec::new(),
        },
        tol,
    )
    .expect("the insert applies");
    *doc = applied.doc;
    applied.record.minted.expect("an insert mints an id")
}

fn declare(doc: &mut ProfileDoc, name: VarName, value: f64, distribution: Distribution, tol: Tol) {
    let applied = apply(
        doc,
        &DocEdit::DeclareVar {
            name,
            def: pncad::document::VarDecl::Free(FreeVar::continuous_with(
                Dimension::Angle,
                value,
                distribution,
            )),
        },
        tol,
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
    /// The `Assertion` over it. Read by [`crate::chaintol`].
    pub assertion: RecipeNodeId,
    /// The placed bars, base first. Carried because a cell that DRAWS
    /// the study needs the built bodies and not only the summary over
    /// them.
    pub bars: Vec<RecipeNodeId>,
    /// The placed joint pins, base first: `links + 1` of them, pin `k`
    /// at the base of link `k` and the last at the tip.
    pub pins: Vec<RecipeNodeId>,
}

/// **The study** at `links` links: [`JOINT_SIGMA`] on every joint and
/// [`POSITION_BOUND`] on the tip. At [`LINKS`] it is the chain
/// [`crate::mcchain`] replays and the gallery writes;
/// [`crate::chaintol`]'s table walks it from one link up.
pub fn study(links: usize, tol: Tol) -> Chain {
    chain(links, JOINT_SIGMA, POSITION_BOUND, tol)
}

/// The study's document at [`LINKS`] links, as the GUI opens it.
pub fn gallery_document(tol: Tol) -> ProfileDoc {
    study(LINKS, tol).doc
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
            joint_name(k),
            0.0,
            Distribution::Normal { sigma: joint_sigma },
            tol,
        );
    }

    // The three shapes the chain is built from, each authored ONCE at
    // the origin of a link's own frame and then placed. Literal
    // rectangles and circles: the study is about the JOINTS, and a
    // parameter nothing measures would be noise in the per-parameter
    // table.
    let h = LINK_HEIGHT / 2.0;
    let bar_profile = insert(
        &mut doc,
        Node::Profile(ProfileProgram {
            loops: vec![
                LoopProgram::polygon([(0.0, -h), (LINK_LENGTH, -h), (LINK_LENGTH, h), (0.0, h)])
                    .expect("finite bar corners"),
            ],
            ids: Vec::new(),
            on: None,
        }),
        tol,
    );
    let bar = insert(
        &mut doc,
        Node::Extrude {
            profile: bar_profile.into(),
            distance: len(LINK_THICKNESS),
        },
        tol,
    );
    let pin_along = |doc: &mut ProfileDoc, x: Formula, tol| {
        let profile = insert(
            doc,
            Node::Profile(ProfileProgram {
                loops: vec![LoopProgram::Circle {
                    centre: [x, Formula::zero(Dimension::Length)],
                    radius: len(PIN_RADIUS),
                }],
                ids: Vec::new(),
                on: None,
            }),
            tol,
        );
        insert(
            doc,
            Node::Extrude {
                profile: profile.into(),
                distance: len(LINK_THICKNESS),
            },
            tol,
        )
    };
    let base_pin = pin_along(&mut doc, Formula::zero(Dimension::Length), tol);
    let tip_pin = pin_along(&mut doc, len(LINK_LENGTH), tol);

    // **The joints.** Link `k` stands in the world turned about the
    // world's `z` by `θ_k = joint_1 + … + joint_k`, its own origin at
    // link `k-1`'s tip: `Σ_{i<k} LINK_LENGTH·(cos θ_i, sin θ_i)`. Its
    // values are on the world's axes, each reading the joint variables,
    // so joint `j` turns links `j..n` and leaves links `1..j` where they
    // were.
    let theta = |k: usize| {
        (1..=k)
            .map(|j| Formula::named(joint_name(j), Dimension::Angle))
            .reduce(|a, b| Formula::add(a, b).expect("angles add"))
            .expect("a link has a joint")
    };
    let along = |k: usize, trig: fn(Formula) -> Result<Formula, _>| {
        (1..k).fold(Formula::zero(Dimension::Length), |sum, i| {
            let step = Formula::mul(len(LINK_LENGTH), trig(theta(i)).expect("an angle's ratio"))
                .expect("a length times a ratio");
            Formula::add(sum, step).expect("lengths add")
        })
    };
    let link_values = |k: usize| {
        vec![
            (Coord::Spin(Some(TargetAxis::Z)), theta(k)),
            (Coord::Slide(TargetAxis::X), along(k, Formula::cos)),
            (Coord::Slide(TargetAxis::Y), along(k, Formula::sin)),
        ]
    };
    let mut bars: Vec<RecipeNodeId> = Vec::with_capacity(links);
    for k in 1..=links {
        let placed = crate::world::in_world_by(&doc, bar, link_values(k), tol);
        doc = placed.doc;
        bars.push(placed.placement);
    }
    // Pin `k` sits at the base of link `k`, so it stands where link `k`
    // does, by the same values; the last pin is link `links`'s TIP pin,
    // one shape over.
    let mut pins: Vec<RecipeNodeId> = Vec::with_capacity(links + 1);
    for k in 1..=links {
        let shapes: &[RecipeNodeId] = if k == links {
            &[base_pin, tip_pin]
        } else {
            &[base_pin]
        };
        for &shape in shapes {
            let placed = crate::world::in_world_by(&doc, shape, link_values(k), tol);
            doc = placed.doc;
            pins.push(placed.placement);
        }
    }

    // The target: the mating pin at the chain's nominal tip, fixed in
    // the world where it is drawn.
    let target = pin_along(&mut doc, len(links as f64 * LINK_LENGTH), tol);
    let placed = crate::world::place_in_world(&doc, target, tol);
    let target = placed.placement;
    // What the document makes: the links, their pins and the target.
    let made: Vec<pncad::document::Operand> = bars
        .iter()
        .chain(&pins)
        .chain([&target])
        .map(|&copy| pncad::document::Operand::Node(copy))
        .collect();
    doc = crate::world::set_product(&placed.doc, &made, tol).doc;

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
                pncad::prelude::SurfaceKind::Cylinder,
            ))],
            &doc.var_env::<f64>(),
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
    // The two references are read BEFORE the insert borrows the
    // document mutably — the borrow checker's way of saying that a
    // measure's references are resolved against a document that
    // already exists.
    let position = MeasurePrimitive::Distance {
        a: wall(*pins.last().expect("a chain has a tip pin")),
        b: wall(target),
    };
    let measure = insert(&mut doc, Node::measure(&position), tol);
    let position = doc.output(measure, 0).expect("a measure defines its value");
    let assertion = insert(
        &mut doc,
        Node::Assertion { claim: pncad::document::Claim::Value {
            value: Formula::var(position, Dimension::Length),
            bound: len(bound),
            relation: AssertionRelation::AtMost,
        } },
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
