//! **INTENT stage 5 B — interference between copies is a finding**
//! (`docs/INTENT-STAGE5-SPEC.md` §3, §6 rows 5–10; D10, Assertions).
//!
//! At rest, an overlap of two copies' material is an
//! [`editor_core::InterferenceFinding`] on the assembly, one per
//! connected overlap, never a refusal. A holding assertion quiets one
//! only when it reads a `Gap` directly over an opposed pair of the two
//! copies' faces, admits only negative values, its two faces bound the
//! overlap, and every face bounding it lies between their carriers.

#![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

use crate::corpus;
use crate::fixture::{fname, insert, len, on_frame, place, square, wall};
use editor_core::{
    Assembly, AssertionRelation, ExtrudeSide, Formula, InterferenceFinding, MeasurePrimitive, Node,
    Overlap, ProfileDoc, RecipeNodeId, RoleSeg, SitedRef, StableName, assemble,
};
use geom_core::Tol;
use topo::{CensusContact, ValidationError};

/// A block extruded `height` from a square of half-side `h` at
/// `(cx, cy)`, on a frame `z` up the world's z axis: the extrude.
fn block(
    doc: ProfileDoc,
    (cx, cy, z): (f64, f64, f64),
    h: f64,
    height: f64,
) -> (ProfileDoc, RecipeNodeId) {
    let (doc, profile) = on_frame(
        doc,
        [0.0, 0.0, z],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![square(cx, cy, h)],
    );
    insert(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: len(height),
            side: ExtrudeSide::Along,
        },
    )
}

/// The `square` wall facing +x (segment 1) and the one facing −x
/// (segment 3), as `block`'s extrude names them.
fn plus_x(doc: &ProfileDoc, b: RecipeNodeId) -> StableName {
    fname(b, wall(doc, b, 1))
}

fn minus_x(doc: &ProfileDoc, b: RecipeNodeId) -> StableName {
    fname(b, wall(doc, b, 3))
}

/// **Two copies overlapping in one slab**: a unit block `a`, and a
/// smaller block `b` whose −x wall sits 0.05 inside `a`'s +x wall,
/// strictly inside `a` across y and z, so every vertex of `b` on that
/// wall is inside `a`'s material and nothing is coplanar. Each block
/// placed once at the identity: `(doc, a, b, copy of a, copy of b)`.
fn slab_overlap() -> (
    ProfileDoc,
    RecipeNodeId,
    RecipeNodeId,
    RecipeNodeId,
    RecipeNodeId,
) {
    let doc = ProfileDoc::empty_derived("intent-s5-b-slab", Tol::witness());
    let (doc, a) = block(doc, (0.0, 0.0, 0.0), 0.5, 1.0);
    let (doc, b) = block(doc, (0.75, 0.0, 0.25), 0.3, 0.5);
    let (doc, p) = place(doc, a);
    let (doc, q) = place(doc, b);
    (doc, a, b, p, q)
}

fn assembled(doc: &ProfileDoc) -> Assembly<f64> {
    let run = corpus::eval::<f64>(doc);
    assemble(doc, &run, Tol::witness()).unwrap_or_else(|e| panic!("the gate refused: {e}"))
}

/// The faces a bounded finding names, as `(placement, the body's own
/// name)`.
fn sites(finding: &InterferenceFinding, doc: &ProfileDoc) -> Vec<(RecipeNodeId, StableName)> {
    let Overlap::Bounded { faces } = &finding.overlap else {
        panic!("the overlap is bounded: {:?}", finding.overlap)
    };
    faces
        .iter()
        .map(|site| {
            let placement = doc
                .operation_of(site.copy.var)
                .expect("a copy is a placement's output");
            let [RoleSeg::Placed { of }] = &site.face.path[..] else {
                panic!("a copy's face name wraps the body's: {:?}", site.face)
            };
            (placement, of.name().clone())
        })
        .collect()
}

/// **(B, test 5) An overlap is a finding, not a refusal.** The gate
/// returns `Ok` with one loud finding, whose evidence is the census's
/// decided `InstanceInterference` (what the gate refused before) and
/// whose site names `a`'s +x wall and `b`'s −x wall among the faces
/// bounding it.
#[test]
fn two_overlapping_copies_gate_ok_with_one_loud_finding() {
    let (doc, a, b, p, q) = slab_overlap();
    let assembly = assembled(&doc);
    let [finding] = assembly.interference.as_slice() else {
        panic!("one overlap, one finding: {:?}", assembly.interference)
    };
    assert!(finding.is_loud(), "no assertion is written, so it is loud");
    assert_eq!(
        (finding.a.var, finding.b.var),
        (doc.output(p, 0).unwrap(), doc.output(q, 0).unwrap()),
        "the finding names the two copies in gather order"
    );
    assert!(
        finding
            .evidence
            .iter()
            .any(|e| matches!(e, ValidationError::InstanceInterference { .. })),
        "the census decided it as a vertex inside the other's material: {:?}",
        finding.evidence
    );
    let sites = sites(finding, &doc);
    for (copy, face) in [(p, plus_x(&doc, a)), (q, minus_x(&doc, b))] {
        assert!(
            sites.contains(&(copy, face.clone())),
            "{face:?} of copy {copy:?} bounds the overlap: {sites:?}"
        );
    }
    assert!(
        !sites.contains(&(p, minus_x(&doc, a))),
        "a's far wall does not bound it: {sites:?}"
    );
}

/// **(B, test 8) A pierce is interference.** A bar through a plate's
/// two side walls, every vertex of each outside the other's material:
/// the census sees only edges piercing faces, and the gate reports one
/// finding for the pair rather than refusing an undeclared contact.
#[test]
fn a_bar_through_a_plate_with_no_vertex_inside_is_one_finding() {
    let doc = ProfileDoc::empty_derived("intent-s5-b-pierce", Tol::witness());
    let (doc, plate) = block(doc, (0.0, 0.0, 0.0), 0.5, 1.0);
    // The bar: x ∈ [−1, 1], y ∈ [−0.1, 0.1], z ∈ [0.4, 0.6].
    let (doc, profile) = on_frame(
        doc,
        [0.0, 0.0, 0.4],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![(-1.0, -0.1), (1.0, -0.1), (1.0, 0.1), (-1.0, 0.1)]],
    );
    let (doc, bar) = insert(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: len(0.2),
            side: ExtrudeSide::Along,
        },
    );
    let (doc, p) = place(doc, plate);
    let (doc, q) = place(doc, bar);
    let assembly = assembled(&doc);
    let [finding] = assembly.interference.as_slice() else {
        panic!("one overlap, one finding: {:?}", assembly.interference)
    };
    // No vertex is inside: the evidence is the pierces, and the
    // containment arm's undecided pair the standing pierces left it.
    assert!(
        finding.evidence.iter().any(|e| matches!(
            e,
            ValidationError::UndeclaredContact {
                contact: CensusContact::EdgeFacePierce { .. },
                ..
            }
        )) && finding.evidence.iter().all(|e| matches!(
            e,
            ValidationError::UndeclaredContact {
                contact: CensusContact::EdgeFacePierce { .. },
                ..
            } | ValidationError::CensusUndecidable { .. }
        )),
        "the census's evidence is pierces: {:?}",
        finding.evidence
    );
    let sites = sites(finding, &doc);
    for (copy, face) in [(p, plus_x(&doc, plate)), (p, minus_x(&doc, plate))] {
        assert!(
            sites.contains(&(copy, face.clone())),
            "the plate's wall {face:?} bounds the overlap: {sites:?}"
        );
    }
    assert!(
        sites.iter().any(|(copy, _)| *copy == q),
        "the bar's faces bound it too: {sites:?}"
    );
}

/// A `Gap` over two faces read at `sites`, and an assertion bounding it
/// by `relation` and `bound` metres: the document after them, and the
/// assertion.
fn asserted_gap(
    doc: ProfileDoc,
    sites: [SitedRef; 2],
    relation: AssertionRelation,
    bound: f64,
) -> (ProfileDoc, RecipeNodeId) {
    let (doc, measure) = crate::fixture::measure_node(
        &doc,
        MeasurePrimitive::Gap { outer: 0, inner: 1 },
        sites.to_vec(),
    );
    let value: Formula = crate::fixture::value_of(&doc, measure);
    insert(
        doc,
        Node::Assertion {
            value,
            bound: len(bound),
            relation,
        },
    )
}

/// **(B, test 6, the unplaced row) A gap over the unplaced bodies'
/// faces quiets nothing**, though it holds: it is a statement about
/// the bodies in their construction frames, not about the copies.
#[test]
fn a_holding_gap_over_the_unplaced_bodies_quiets_nothing() {
    let (doc, a, b, _, _) = slab_overlap();
    let (doc, assertion) = asserted_gap(
        doc.clone(),
        [
            SitedRef::new(a, plus_x(&doc, a)),
            SitedRef::new(b, minus_x(&doc, b)),
        ],
        AssertionRelation::AtMost,
        -0.01,
    );
    let run = corpus::eval::<f64>(&doc);
    assert!(
        matches!(
            run.value(assertion).map(|v| &v.payload),
            Some(editor_core::ValuePayload::Assertion(
                editor_core::AssertionVerdict::Holds { .. }
            ))
        ),
        "the assertion holds over the bodies (gap −0.05 ≤ −0.01)"
    );
    let assembly = assemble(&doc, &run, Tol::witness()).expect("the gate reports");
    let [finding] = assembly.interference.as_slice() else {
        panic!("one finding: {:?}", assembly.interference)
    };
    assert!(finding.is_loud(), "the bodies are not the copies");
}
