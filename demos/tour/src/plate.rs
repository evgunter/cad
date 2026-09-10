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
//! The split is also what makes the density cell REACHABLE. The
//! tolerance cell is behind the `interval` feature, because its whole
//! subject is the certified scalar's leaves; the Monte-Carlo lane is
//! pure `f64` replay and is ungated on purpose
//! (`crates/pncad/src/analysis.rs`, and the reason is written there),
//! so its cell must be too — and it cannot be if the only spelling of
//! the document sits inside a gated module.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use pncad::document::{
    AssertionDir, CancelToken, Dimension, Distribution, DocEdit, DocParam, DocumentId, EvalOptions,
    Evaluation, Expr, LoopProgram, MeasureExpr, MeasurePrimitive, Node, ParamName, ProfileDoc,
    ProfileProgram, RecipeNodeId, SitedRef, UnitSym, apply, evaluate,
};
use pncad::geom_core::Tol;
use pncad::select::{EntityKind, GeomPred, NamePat, Selector, SurfaceKindSet, select_where};

/// The nominal hole spacing, in metres (3.1 mm).
pub const SPACING: f64 = 3.1e-3;
/// The nominal hole radius, in metres (1.25 mm).
pub const RADIUS: f64 = 1.25e-3;
/// The nominal web: `SPACING − 2·RADIUS` = 0.6 mm.
pub const WEB: f64 = SPACING - 2.0 * RADIUS;

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

fn param(n: &str) -> Expr {
    Expr::param(ParamName::new(n), Dimension::Length)
}

fn insert(doc: &mut ProfileDoc, node: Node<ProfileProgram>, tol: Tol) -> RecipeNodeId {
    let applied = apply(doc, &DocEdit::InsertNode { node }, tol).expect("the insert applies");
    *doc = applied.doc;
    applied.record.minted.expect("an insert mints an id")
}

fn declare(doc: &mut ProfileDoc, n: &str, value: f64, distribution: Distribution, tol: Tol) {
    let applied = apply(
        doc,
        &DocEdit::SetDocParam {
            name: ParamName::new(n),
            value: DocParam::Continuous {
                dim: Dimension::Length,
                value,
                display_unit: UnitSym::canonical_for(Dimension::Length),
                distribution: Some(distribution),
            },
        },
        tol,
    )
    .expect("the parameter applies");
    *doc = applied.doc;
}

/// The plate, its two holes, the web measure and the assertion — the
/// worked example, authored the way a user would.
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
    /// The `Assertion` over it. Read by [`crate::tolerance`], which is
    /// behind the `interval` feature, so a default build legitimately
    /// has no consumer for it — the field is part of the document
    /// either way and a cell that dropped it would be describing a
    /// different one.
    #[cfg_attr(not(feature = "interval"), allow(dead_code))]
    pub assertion: RecipeNodeId,
    /// The two hole extrudes, in the order their centres run along
    /// `−x` then `+x`. Carried because a cell that DRAWS the study
    /// needs the built bodies and not only the summary over them.
    pub holes: [RecipeNodeId; 2],
}

pub fn plate(spacing_half_width: f64, radius_sigma: f64, bound: f64, tol: Tol) -> Plate {
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
        }),
        tol,
    );
    let _plate = insert(
        &mut doc,
        Node::Extrude {
            profile: plate_profile,
            distance: len(1.0e-3),
        },
        tol,
    );

    let hole = |doc: &mut ProfileDoc, centre: Expr, radius: &str, tol| {
        let profile = insert(
            doc,
            Node::Profile(ProfileProgram {
                plane,
                loops: vec![LoopProgram::Circle {
                    centre: [centre, len(0.0)],
                    radius: param(radius),
                }],
            }),
            tol,
        );
        insert(
            doc,
            Node::Extrude {
                profile,
                distance: len(1.0e-3),
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

    // The wall names come from the SELECTION door, the way a user gets
    // them: evaluate what is built so far, then ask each hole for its
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
        assert!(!faces.is_empty(), "a hole extrude has a cylindrical wall");
        SitedRef::new(node, faces.remove(0))
    };

    // web = distance(wall_a, wall_b) − r_a − r_b. The distance between
    // two parallel cylinder faces is their AXIS distance (the closed
    // form's own contract), so the subtraction of the radii is the
    // author's arithmetic and not a hidden convention.
    let radius_of = |n: &str| MeasureExpr::value(param(n));
    let web = MeasureExpr::sub(
        MeasureExpr::primitive(MeasurePrimitive::Distance { a: 0, b: 1 }),
        MeasureExpr::add(radius_of("hole_a_r"), radius_of("hole_b_r")).expect("Length + Length"),
    )
    .expect("Length − Length");
    // The two references are read BEFORE the insert borrows the
    // document mutably — the borrow checker's way of saying that a
    // measure's references are resolved against a document that
    // already exists, which is exactly the E3 contract.
    let refs = vec![wall(hole_a), wall(hole_b)];
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
