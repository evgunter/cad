//! **The two-hole plate's DOCUMENT** — ERROR-DESIGN's worked example,
//! authored once and read by two cells.
//!
//! It lives here rather than inside either cell because the two are
//! about the same part and must stay about the same part:
//! [`crate::tolerance`] runs the certified and advisory lanes over it
//! and narrates the numbers, and [`crate::mcplate`] draws the
//! population the advisory lane averages over. A second transcription
//! of the plate would let the picture and the report drift into being
//! about two different studies, which is the one failure a density
//! picture cannot survive.
//!
//! **The study's document does not cut its holes**: the web is read
//! off the hole extrudes' own walls, so its product is the blank. A
//! user would write the holes one of two natural ways, and both stop
//! short of the study:
//!
//! - **cut**: the blank minus the two hole extrudes, [`cut_plate`],
//!   authored beside the study and attempted every run as two walls —
//!   the certified drive certifies no box of it, and it has no product
//!   root;
//! - **sketched**: one extrude of a profile with the two circles as
//!   inner loops. No boolean, so no tie: it certifies whole boxes up to
//!   `1e-2` of the study, and 0 of 512 leaves over the real study
//!   (`work/paths/inner-loop-circles-bound-the-plate-study-at-arc-span.md`).
//!   Its measure would read the part's own walls, so it has no product
//!   root either.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use pncad::document::ExtrudeSide;
use pncad::document::{
    AssertionDir, BooleanOp, CancelToken, Dimension, Distribution, DocEdit, DocParam, DocumentId,
    EvalOptions, Evaluation, Expr, LoopProgram, MeasureExpr, MeasurePrimitive, Node, ParamName,
    ProfileDoc, ProfileProgram, RecipeNodeId, RefusingReach, SitedRef, apply, evaluate,
};
use pncad::geom_core::Tol;
use pncad::prelude::PlaneRelation;
use pncad::select::{
    BooleanCoincidence, EntityKind, GeomPred, NamePat, SegPat, SegTag, Selector, SurfaceKindSet,
    declare_node, find_flush_candidates, select_where,
};

/// The nominal hole spacing, in metres (3.1 mm).
pub const SPACING: f64 = 3.1e-3;
/// The nominal hole radius, in metres (1.25 mm).
pub const RADIUS: f64 = 1.25e-3;
/// The nominal web: `SPACING − 2·RADIUS` = 0.6 mm.
pub const WEB: f64 = SPACING - 2.0 * RADIUS;

/// The study the machinist writes down: ±0.05 mm on the hole spacing…
pub const SPACING_HALF_WIDTH: f64 = 5.0e-5;
/// …σ = 0.01 mm on each radius…
pub const RADIUS_SIGMA: f64 = 1.0e-5;
/// …and the web asserted at least 0.1 mm under its nominal.
pub const WEB_BOUND: f64 = WEB - 1.0e-4;

/// **The widest box of this plate that certifies whole, as a fraction
/// of the real study.** `7.81e2 · ε`, which at the default ε is this.
///
/// MEASURED by [`crate::tolerance`], not chosen: its module header
/// carries the measurement and what bounds it (an arc rim's endpoint
/// pinning, whose normal form no shipped rule reaches). Named here
/// because [`crate::mcplate`] draws it to scale, and a number a
/// picture is built around should not be a literal buried in the
/// drawing code.
pub const CERTIFIABLE_FRACTION: f64 = 7.81e-7;

pub fn len(v: f64) -> Expr {
    Expr::literal(v, Dimension::Length).expect("finite length")
}

fn scl(v: f64) -> Expr {
    Expr::literal(v, Dimension::Scalar).expect("finite scalar")
}

fn param(n: &'static str) -> Expr {
    Expr::param(ParamName::from_static(n), Dimension::Length)
}

fn insert(doc: &mut ProfileDoc, node: Node<ProfileProgram>, tol: Tol) -> RecipeNodeId {
    let applied = apply(
        doc,
        &DocEdit::InsertNode {
            node: Box::new(node),
        },
        tol,
        &RefusingReach,
    )
    .expect("the insert applies");
    *doc = applied.doc;
    applied.record.minted.expect("an insert mints an id")
}

fn declare(
    doc: &mut ProfileDoc,
    n: &'static str,
    value: f64,
    distribution: Distribution,
    tol: Tol,
) {
    let applied = apply(
        doc,
        &DocEdit::SetDocParam {
            name: ParamName::from_static(n),
            value: DocParam::continuous_with(Dimension::Length, value, distribution),
        },
        tol,
        &RefusingReach,
    )
    .expect("the parameter applies");
    *doc = applied.doc;
}

/// The plate, its two holes, the web measure and the assertion — the
/// worked example, as one of [`plate`] (the study's document, holes
/// left as extrudes) or [`cut_plate`] (the holes cut).
///
/// The two tolerances are passed separately rather than scaled from
/// one number: their RATIO is what decides whether the RSS and the
/// certified worst case disagree, so it is a modelling choice and not a
/// scale.
pub struct Plate {
    /// The document itself.
    pub doc: ProfileDoc,
    /// The web `Measure` node — `distance(wall_a, wall_b) − r_a − r_b`.
    pub measure: RecipeNodeId,
    /// The `Assertion` over it. Read by [`crate::tolerance`].
    pub assertion: RecipeNodeId,
    /// The two hole extrudes, in the order their centres run along
    /// `−x` then `+x`. Carried because a cell that DRAWS the study
    /// needs the built bodies and not only the summary over them.
    pub holes: [RecipeNodeId; 2],
}

/// **The real study**: the plate the machinist's numbers describe —
/// the one [`crate::tolerance`]'s stop 1 analyzes, [`crate::mcplate`]
/// draws, and the gallery writes.
pub fn real_study(tol: Tol) -> Plate {
    plate(SPACING_HALF_WIDTH, RADIUS_SIGMA, WEB_BOUND, tol)
}

/// The real study's document, as the GUI opens it.
pub fn gallery_document(tol: Tol) -> ProfileDoc {
    real_study(tol).doc
}

/// The study's document: the blank and the two hole extrudes, the web
/// read off the holes' own walls.
pub fn plate(spacing_half_width: f64, radius_sigma: f64, bound: f64, tol: Tol) -> Plate {
    author(spacing_half_width, radius_sigma, bound, false, tol)
}

/// **The plate with its holes cut**, one of its two natural spellings
/// (the other, inner-loop circles in one extrude, is the module doc's):
/// the holes subtracted from the blank by two `Boolean(Subtract)`s, and
/// the web read off the cut part's bore walls. Not the study's
/// document, because two doors refuse it:
/// the certified drive certifies no box of it
/// (`work/reach/a-hole-wholly-inside-its-target-ties-the-subtract-volume-bound.md`,
/// pinned in [`crate::tolerance`]), and its one root is the assertion,
/// so it has no product to draw
/// (`work/recipe/a-measured-part-is-not-a-product-root.md`, pinned in
/// [`crate::gallery`]).
pub fn cut_plate(spacing_half_width: f64, radius_sigma: f64, bound: f64, tol: Tol) -> Plate {
    author(spacing_half_width, radius_sigma, bound, true, tol)
}

fn author(spacing_half_width: f64, radius_sigma: f64, bound: f64, cut: bool, tol: Tol) -> Plate {
    let mut doc = ProfileDoc::empty(DocumentId::derive("pncad-demo-tolerance"), tol);
    // The hole spacing: a UNIFORM tolerance, the machinist's ±.
    declare(
        &mut doc,
        "half_spacing",
        SPACING / 2.0,
        Distribution::Uniform {
            lo: -spacing_half_width,
            hi: spacing_half_width,
        },
        tol,
    );
    // The two radii: INDEPENDENT normals. Independent because they are
    // two names (PL6), which is what makes the RSS's root-sum-square
    // differ from the worst case's linear sum — the whole subject of
    // stop 2.
    for n in ["hole_a_r", "hole_b_r"] {
        declare(
            &mut doc,
            n,
            RADIUS,
            Distribution::Normal {
                sigma: radius_sigma,
            },
            tol,
        );
    }

    let plane = insert(
        &mut doc,
        Node::Datum(pncad::document::Datum::Frame {
            origin: [len(0.0), len(0.0), len(0.0)],
            u: [scl(1.0), scl(0.0), scl(0.0)],
            v: [scl(0.0), scl(1.0), scl(0.0)],
        }),
        tol,
    );
    // The plate itself is a literal rectangle: the study is about the
    // holes, and a parameter nothing measures would be noise in the
    // stackup's per-parameter table.
    let plate_profile = insert(
        &mut doc,
        Node::Profile(ProfileProgram {
            plane,
            loops: vec![
                LoopProgram::polygon([
                    (-4.0e-3, -2.0e-3),
                    (4.0e-3, -2.0e-3),
                    (4.0e-3, 2.0e-3),
                    (-4.0e-3, 2.0e-3),
                ])
                .expect("finite plate corners"),
            ],
            ids: Vec::new(),
        }),
        tol,
    );
    let blank = insert(
        &mut doc,
        Node::Extrude {
            profile: plate_profile,
            distance: len(1.0e-3),
            side: ExtrudeSide::Along,
        },
        tol,
    );

    let hole = |doc: &mut ProfileDoc, centre: Expr, radius: &'static str, tol| {
        let profile = insert(
            doc,
            Node::Profile(ProfileProgram {
                plane,
                loops: vec![LoopProgram::Circle {
                    centre: [centre, len(0.0)],
                    radius: param(radius),
                }],
                ids: Vec::new(),
            }),
            tol,
        );
        insert(
            doc,
            Node::Extrude {
                profile,
                distance: len(1.0e-3),
                side: ExtrudeSide::Along,
            },
            tol,
        )
    };
    let hole_a = hole(
        &mut doc,
        Expr::sub(len(0.0), param("half_spacing")).expect("a length"),
        "hole_a_r",
        tol,
    );
    let hole_b = hole(&mut doc, param("half_spacing"), "hole_b_r", tol);

    // The holes run the plate's full depth, so each hole's caps lie
    // flush on the blank's: a cut declares those continuations, as the
    // coincidence discipline asks of any flush pair.
    let eval_here = |doc: &ProfileDoc| -> Evaluation<f64> {
        evaluate(doc, None, &CancelToken::new(), &EvalOptions::default(), tol)
    };
    let subtract = |doc: &mut ProfileDoc, a: RecipeNodeId, b: RecipeNodeId| {
        let found = find_flush_candidates(&eval_here(doc), a, b, tol)
            .expect("the hole's caps are definite flush pairs");
        // The inspection: the hole's two caps, each continuing the
        // blank's cap it lies in.
        assert_eq!(found.len(), 2, "one continuation per cap: {found:#?}");
        assert!(
            found
                .iter()
                .all(|f| f.class == BooleanCoincidence::Continuation
                    && f.evidence.relation == PlaneRelation::SameOriented),
            "each hole cap continues the blank's: {found:#?}"
        );
        let declare = insert(doc, declare_node(&found).expect("nonempty findings"), tol);
        insert(
            doc,
            Node::Boolean {
                op: BooleanOp::Subtract,
                a,
                b,
                declare: Some(declare),
            },
            tol,
        )
    };
    // Where each bore wall is read, and the name that picks it there.
    // On the cut part a name records its lineage operand by operand,
    // so "the faces hole a cut" is spelled through both cuts: the
    // second's A side, then the first's B side.
    let face = || NamePat::of_kind(EntityKind::Face);
    let from = |side: SegTag, inner: NamePat| face().seg(SegPat::tag(side).of([inner]));
    let sites = if cut {
        let drilled = subtract(&mut doc, blank, hole_a);
        let part = subtract(&mut doc, drilled, hole_b);
        [
            (
                part,
                from(SegTag::FromA, from(SegTag::FromB, face().node(hole_a))),
            ),
            (part, from(SegTag::FromB, face().node(hole_b))),
        ]
    } else {
        [(hole_a, face()), (hole_b, face())]
    };

    // The wall names come from the SELECTION door, the way a user gets
    // them: evaluate what is built so far, then ask for each hole's
    // cylindrical face.
    let ev = eval_here(&doc);
    let wall = |(at, lineage): (RecipeNodeId, NamePat)| {
        let mut faces = select_where(
            &ev,
            at,
            &Selector::of(lineage),
            &[GeomPred::SurfaceKind(SurfaceKindSet::just(
                pncad::prelude::SurfaceKind::Cylinder,
            ))],
            &doc.param_env::<f64>(),
            tol,
        )
        .expect("the surface-kind atom is exact");
        faces.sort();
        assert!(!faces.is_empty(), "each hole has a cylindrical wall");
        SitedRef::new(at, faces.remove(0))
    };

    // web = distance(wall_a, wall_b) − r_a − r_b. The distance between
    // two parallel cylinder faces is their AXIS distance (the closed
    // form's own contract), so the subtraction of the radii is the
    // author's arithmetic and not a hidden convention.
    let radius_of = |n: &'static str| MeasureExpr::value(param(n));
    let web = MeasureExpr::sub(
        MeasureExpr::primitive(MeasurePrimitive::Distance { a: 0, b: 1 }),
        MeasureExpr::add(radius_of("hole_a_r"), radius_of("hole_b_r")).expect("Length + Length"),
    )
    .expect("Length − Length");
    // The two references are read BEFORE the insert borrows the
    // document mutably — the borrow checker's way of saying that a
    // measure's references are resolved against a document that
    // already exists, which is exactly the E3 contract.
    let [site_a, site_b] = sites;
    let refs = vec![wall(site_a), wall(site_b)];
    let measure = insert(
        &mut doc,
        Node::measure(web, refs).expect("both indices in range"),
        tol,
    );
    let assertion = insert(
        &mut doc,
        Node::Assertion {
            measure,
            bound: len(bound),
            dir: AssertionDir::AtLeast,
        },
        tol,
    );
    Plate {
        doc,
        measure,
        assertion,
        holes: [hole_a, hole_b],
    }
}
