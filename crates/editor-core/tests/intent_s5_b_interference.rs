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

/// A `Gap` over faces of two world copies, read where each copy holds
/// it (`face.in_copy(placement)`), bounded by `relation` and `bound`.
fn gap_at_copies(
    doc: ProfileDoc,
    (p, outer): (RecipeNodeId, StableName),
    (q, inner): (RecipeNodeId, StableName),
    relation: AssertionRelation,
    bound: f64,
) -> (ProfileDoc, RecipeNodeId) {
    asserted_gap(
        doc,
        [
            SitedRef::new(p, outer.in_copy(p)),
            SitedRef::new(q, inner.in_copy(q)),
        ],
        relation,
        bound,
    )
}

/// The one finding of `doc`'s assembly.
fn only_finding(doc: &ProfileDoc) -> InterferenceFinding {
    let assembly = assembled(doc);
    let [finding] = assembly.interference.as_slice() else {
        panic!("one finding: {:?}", assembly.interference)
    };
    finding.clone()
}

/// **(B, test 6) Quiet by a one-sided negative bound, and only by
/// one.** The slab overlap's `Gap(a's +x wall, b's −x wall)` over the
/// two copies is −0.05. Under `≤ −0.01` and `= −0.05` the finding is
/// listed and quiet, naming the assertion; under `≤ 0` (the bound is
/// not negative), `≥ −0.1` (the relation admits clearance) and
/// `≤ −0.1` (`Violated`) it stays loud.
///
/// Red if the rule reads the measure's value instead of the verdict
/// (the violated row quiets), admits a straddling bound (`≤ 0` quiets),
/// or ignores the relation (`≥` quiets).
#[test]
fn a_holding_negative_gap_over_the_copies_quiets_its_overlap_and_nothing_else_does() {
    let (doc, a, b, p, q) = slab_overlap();
    let (outer, inner) = (plus_x(&doc, a), minus_x(&doc, b));
    for (relation, bound, quiet) in [
        (AssertionRelation::AtMost, -0.01, true),
        (AssertionRelation::Equal, -0.05, true),
        (AssertionRelation::AtMost, 0.0, false),
        (AssertionRelation::AtLeast, -0.1, false),
        (AssertionRelation::AtMost, -0.1, false),
    ] {
        let (doc, assertion) = gap_at_copies(
            doc.clone(),
            (p, outer.clone()),
            (q, inner.clone()),
            relation,
            bound,
        );
        let finding = only_finding(&doc);
        assert_eq!(
            finding.quiet,
            quiet.then_some(assertion),
            "gap −0.05 under {} {bound}",
            relation.symbol()
        );
        assert!(
            matches!(finding.overlap, Overlap::Bounded { .. }),
            "a quiet finding is still listed with its site"
        );
    }
}

/// **(B, test 7) A second overlap of the same pair stays loud.** A
/// U-shaped copy whose two lugs both overlap one block, their +x walls
/// two faces on one carrier: two findings, and the assertion over the
/// first lug's wall quiets the first alone (the clevis's second lug).
///
/// Red if the site is the copy pair or the carrier pair (both quiet).
#[test]
fn an_assertion_on_one_lug_leaves_the_second_lug_loud() {
    let doc = ProfileDoc::empty_derived("intent-s5-b-clevis", Tol::witness());
    let (doc, profile) = on_frame(
        doc,
        [0.0, 0.0, 0.0],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![
            (-0.5, -0.5),
            (0.5, -0.5),
            (0.5, -0.2),
            (0.0, -0.2),
            (0.0, 0.2),
            (0.5, 0.2),
            (0.5, 0.5),
            (-0.5, 0.5),
        ]],
    );
    let (doc, clevis) = insert(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: len(1.0),
            side: ExtrudeSide::Along,
        },
    );
    let (doc, pin) = block(doc, (0.75, 0.0, 0.25), 0.3, 0.5);
    let (doc, p) = place(doc, clevis);
    let (doc, q) = place(doc, pin);
    let (first, second) = (
        fname(clevis, wall(&doc, clevis, 1)),
        fname(clevis, wall(&doc, clevis, 5)),
    );
    let (doc, assertion) = gap_at_copies(
        doc.clone(),
        (p, first.clone()),
        (q, minus_x(&doc, pin)),
        AssertionRelation::AtMost,
        -0.01,
    );
    let assembly = assembled(&doc);
    assert_eq!(assembly.interference.len(), 2, "one finding per lug");
    for finding in &assembly.interference {
        let sites = sites(finding, &doc);
        let expected = if sites.contains(&(p, first.clone())) {
            Some(assertion)
        } else {
            assert!(
                sites.contains(&(p, second.clone())),
                "the other is the second lug's: {sites:?}"
            );
            None
        };
        assert_eq!(
            finding.quiet, expected,
            "the assertion speaks for its own lug only"
        );
    }
}

/// **(B) An overlap reaching past the asserted carriers stays loud.**
/// `b` is an L whose foot sinks 0.2 into `a` beside the 0.05 slab, one
/// connected overlap bounded by the asserted pair: the assertion's
/// faces bound it, but the foot lies beyond `b`'s −x wall's carrier.
///
/// Red if the rule only asks that the assertion's faces bound the
/// overlap (the sunk flange quiets).
#[test]
fn an_overlap_with_a_sunk_flange_stays_loud() {
    let doc = ProfileDoc::empty_derived("intent-s5-b-flange", Tol::witness());
    let (doc, a) = block(doc, (0.0, 0.0, 0.0), 0.5, 1.0);
    let (doc, profile) = on_frame(
        doc,
        [0.0, 0.0, 0.25],
        [1.0, 0.0, 0.0],
        [0.0, 1.0, 0.0],
        vec![vec![
            (0.45, -0.3),
            (1.05, -0.3),
            (1.05, 0.3),
            (0.3, 0.3),
            (0.3, 0.1),
            (0.45, 0.1),
        ]],
    );
    let (doc, b) = insert(
        doc,
        Node::Extrude {
            profile: profile.into(),
            distance: len(0.5),
            side: ExtrudeSide::Along,
        },
    );
    let (doc, p) = place(doc, a);
    let (doc, q) = place(doc, b);
    let (doc, _) = gap_at_copies(
        doc.clone(),
        (p, plus_x(&doc, a)),
        (q, fname(b, wall(&doc, b, 5))),
        AssertionRelation::AtMost,
        -0.01,
    );
    let finding = only_finding(&doc);
    assert!(
        finding.is_loud(),
        "the foot is beyond the asserted carriers: {finding:?}"
    );
}

/// **(B, test 10) A failed requirement gates nothing.** The quieting
/// assertion's measure names a face its copy does not hold, so the
/// measure refuses and the assertion has no verdict: the gate still
/// answers `Ok`, the finding is loud, and the assertion reports its
/// own failure.
#[test]
fn a_gap_whose_face_vanished_quiets_nothing_and_gates_nothing() {
    let (doc, a, b, p, q) = slab_overlap();
    let vanished = fname(a, RoleSeg::Cap(editor_core::CapEnd::End)).in_copy(q);
    let (doc, assertion) = asserted_gap(
        doc.clone(),
        [
            SitedRef::new(p, plus_x(&doc, a).in_copy(p)),
            SitedRef::new(q, vanished),
        ],
        AssertionRelation::AtMost,
        -0.01,
    );
    let _ = b;
    let run = corpus::eval::<f64>(&doc);
    assert!(
        !matches!(
            run.value(assertion).map(|v| &v.payload),
            Some(editor_core::ValuePayload::Assertion(_))
        ),
        "the assertion has no verdict"
    );
    let assembly = assemble(&doc, &run, Tol::witness()).expect("a failed assertion gates nothing");
    let [finding] = assembly.interference.as_slice() else {
        panic!("one finding: {:?}", assembly.interference)
    };
    assert!(finding.is_loud(), "and quiets nothing");
}

/// A plate with a 0.5 bore and a pin of radius `pin_r` through it,
/// each placed once: `(doc, plate, pin, copy of plate, copy of pin,
/// the bore's wall, the pin's wall)`.
fn bore_and_pin(
    pin_r: f64,
) -> (
    ProfileDoc,
    RecipeNodeId,
    RecipeNodeId,
    RecipeNodeId,
    StableName,
    StableName,
) {
    bore_pin_and_holes(pin_r, &[])
}

/// [`bore_and_pin`] with further holes `(cx, cy, r)` through the plate.
fn bore_pin_and_holes(
    pin_r: f64,
    holes: &[(f64, f64, f64)],
) -> (
    ProfileDoc,
    RecipeNodeId,
    RecipeNodeId,
    RecipeNodeId,
    StableName,
    StableName,
) {
    use crate::fixture::{frame, piece};
    use editor_core::{LoopProgram, ProfileProgram};
    let circle = |r| LoopProgram::<Formula>::circle(0.0, 0.0, r).expect("a literal circle");
    let prism = |doc: ProfileDoc, z, loops, h| {
        let (doc, plane) = insert(doc, frame([0.0, 0.0, z], [1.0, 0.0, 0.0], [0.0, 1.0, 0.0]));
        let (doc, profile) = insert(
            doc,
            Node::Profile(ProfileProgram {
                frame: plane.into(),
                loops,
                ids: Vec::new(),
            }),
        );
        insert(
            doc,
            Node::Extrude {
                profile: profile.into(),
                distance: len(h),
                side: ExtrudeSide::Along,
            },
        )
    };
    let doc = ProfileDoc::empty_derived("intent-s5-b-press", Tol::witness());
    let outline = crate::fixture::desc(RecipeNodeId::new(0, 0), vec![square(0.0, 0.0, 1.0)])
        .loops
        .remove(0);
    let mut loops = vec![outline, circle(0.5)];
    loops.extend(
        holes.iter().map(|&(cx, cy, r)| {
            LoopProgram::<Formula>::circle(cx, cy, r).expect("a literal circle")
        }),
    );
    let (doc, plate) = prism(doc, 0.0, loops, 1.0);
    let (doc, pin) = prism(doc, -0.5, vec![circle(pin_r)], 2.0);
    let (doc, p) = place(doc, plate);
    let (doc, q) = place(doc, pin);
    let lateral =
        |doc: &ProfileDoc, node, l| fname(node, RoleSeg::Lateral(piece(doc, node, l, 0).into()));
    let (bore, wall) = (lateral(&doc, plate, 1), lateral(&doc, pin, 0));
    (doc, p, q, plate, bore, wall)
}

/// **(B, test 6, curved) A press fit quiets under its bore's gap.** A
/// pin of radius 0.505, and one 2000 ε over, through a plate's 0.5 bore. The census decides
/// the pair interferes (the bore's rim vertices inside the pin, the
/// pin's seams piercing the plate) and leaves its curved face pairs
/// undecided, which are the same overlap: one finding, the annulus
/// between the two cylinders, loud unasserted and quiet under
/// `Gap(bore, pin) ≤ −0.001` over the two copies. The overlap's faces
/// lie on the asserted carriers themselves, which is where the
/// containment check's coincident faces glue.
#[test]
fn a_pin_pressed_into_a_bore_is_quiet_under_its_gap() {
    // 5 mm of interference, and 2000 ε (2 µm at the default row): the
    // second is a real press fit, its containment check's coincident
    // faces nearest the boolean's band and still well above it.
    let fine = 2000.0 * Tol::witness().eps();
    for (pin_r, bound) in [(0.505, -0.001), (0.5 + fine, -fine / 2.0)] {
        let (doc, p, q, _, bore, wall) = bore_and_pin(pin_r);
        let loud = only_finding(&doc);
        assert!(
            loud.is_loud(),
            "unasserted, the press fit is loud (pin {pin_r})"
        );
        assert!(
            loud.evidence
                .iter()
                .any(|e| matches!(e, ValidationError::InstanceInterference { .. })),
            "the census decided the overlap (pin {pin_r}): {:?}",
            loud.evidence
        );
        let (doc, assertion) = gap_at_copies(
            doc.clone(),
            (p, bore),
            (q, wall),
            AssertionRelation::AtMost,
            bound,
        );
        let finding = only_finding(&doc);
        assert_eq!(
            finding.quiet,
            Some(assertion),
            "quiet under Gap <= {bound} (pin {pin_r}): {finding:?}"
        );
    }
}

/// **(B) A clearance fit the census cannot decide still refuses.** A
/// pin of radius 0.49 in the same bore: the census decides no overlap
/// and leaves the curved pairs undecided, and the gate refuses them as
/// it always did.
///
/// Red if undecided pairs between copies are reported as interference
/// without a decided overlap.
#[test]
fn a_clearance_fit_the_census_cannot_decide_stays_refused() {
    let (doc, ..) = bore_and_pin(0.49);
    let run = corpus::eval::<f64>(&doc);
    match assemble(&doc, &run, Tol::witness()) {
        Err(editor_core::AssemblyError::AtRest { findings }) => assert!(
            findings
                .iter()
                .all(|f| matches!(f.error, ValidationError::CensusUndecidable { .. })),
            "the census's undecided pairs: {findings:?}"
        ),
        other => panic!("the undecided pair refuses: {other:?}"),
    }
}

/// **(B) A quiet finding carries no verdict nothing checked.** The
/// press fit again, its plate drilled with a small hole beside the pin
/// (0.586 from its axis at the nearest, within its faces' reach):
/// the census leaves the pin's wall and that hole's wall undecided, a
/// face pair outside the overlap the assertion's containment check
/// covers, so the finding stays loud and carries the verdict.
///
/// Red if an undecided verdict anywhere on the pair rides a quiet
/// finding.
#[test]
fn an_undecided_pair_beside_the_press_fit_keeps_it_loud() {
    let (doc, p, q, _, bore, wall) = bore_pin_and_holes(0.505, &[(0.45, 0.45, 0.05)]);
    let (doc, _) = gap_at_copies(
        doc.clone(),
        (p, bore),
        (q, wall),
        AssertionRelation::AtMost,
        -0.001,
    );
    let assembly = assembled(&doc);
    let [finding] = assembly.interference.as_slice() else {
        panic!("one overlap: {:?}", assembly.interference)
    };
    let Overlap::Bounded { faces } = &finding.overlap else {
        panic!("bounded: {:?}", finding.overlap)
    };
    let beside = finding.evidence.iter().filter(|e| {
        matches!(
            e,
            ValidationError::CensusUndecidable {
                a: topo::EntityId::Face(_),
                b: topo::EntityId::Face(_),
                ..
            }
        )
    });
    assert!(
        beside.count() > 0,
        "the census left curved pairs undecided: {:?}",
        finding.evidence
    );
    assert!(
        finding.is_loud(),
        "an undecided pair outside the overlap's {} faces keeps it loud",
        faces.len()
    );
}
