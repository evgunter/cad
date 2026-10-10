//! **Interference between copies at rest** (D10, Assertions; ASSEMBLY
//! A5 *Interference.*): the at-rest gate's interference findings, and
//! the quieting rule's one home.
//!
//! An overlap of two copies' material is a finding of its own, never a
//! refusal. The census decides that two copies interfere (a vertex of
//! one strictly inside the other's material, an edge piercing a face,
//! or an in-plane crossing whose side test reads `SameSide`); the gate
//! then intersects the two copies, and each connected solid of the
//! result is one [`InterferenceFinding`], named by the faces of the two
//! copies that bound it. An intersection the kernel cannot form leaves
//! one [`Overlap::Unlocalized`] finding for the pair, which is loud and
//! which nothing quiets.
//!
//! **What quiets one** ([`quieted_by`]): a holding assertion that reads
//! a `Gap`'s output directly, over an opposed pair of faces of the two
//! copies, admitting only negative values (`≤ b` or `= b`, `b` decided
//! negative), whose two faces both bound the overlap, and between
//! whose carriers every face bounding the overlap lies. The rule reads
//! the assertion's verdict, never its measure's value, and an assertion
//! speaks for nothing but its own site: every other overlap is loud.
//!
//! Inside one copy's body an overlap stays the kernel's typed error:
//! only verdicts between solids of two different copies are findings.

use std::collections::{BTreeMap, BTreeSet};

use geom_core::k_stats::decide;
use geom_core::{Band, Decide, Margin, Sign, Tol};
use topo::{Body, CensusContact, CrossingSideVerdict, FaceKey, SolidKey, ValidationError};

use crate::VarId;
use crate::doc::Doc;
use crate::eval::{Evaluation, ValuePayload};
use crate::measure::{AssertionRelation, AssertionVerdict, MeasurePrimitive};
use crate::names::{EntityKey, EntityRef, Entry, NameTable, StableName};
use crate::node::{Node, RecipeNodeId};
use crate::product::{GatheredCopy, Product};

/// **One copy, as a finding names it**: the placement's output
/// variable, and which member of that placement's value it is (0 for a
/// placement of one body).
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct CopyRef {
    /// The placement's output variable.
    pub var: VarId,
    /// The output-body index within the placement's value.
    pub member: u32,
}

/// **A face of a copy**, as the copy's own name table spells it, which
/// is the name a selection of the copy reads.
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct FaceSite {
    /// The copy.
    pub copy: CopyRef,
    /// The face's name.
    pub face: StableName,
}

/// **Where two copies overlap.**
#[derive(Debug, Clone)]
pub enum Overlap {
    /// One connected overlap of the two copies' material, named by the
    /// faces of both copies that bound it, sorted and deduplicated.
    Bounded {
        /// The bounding faces.
        faces: Vec<FaceSite>,
    },
    /// The census decided that the copies overlap and the kernel could
    /// not bound the overlap. Loud, and nothing quiets it.
    Unlocalized(Unlocalized),
}

/// **Why an overlap has no site.**
#[derive(Debug, Clone)]
pub enum Unlocalized {
    /// The intersection of the two copies refused, in its own words.
    Refused {
        /// The refusal's `Display`.
        refusal: String,
    },
    /// A copy did not validate as a body of its own, so it could not
    /// be intersected.
    Invalid {
        /// That copy's findings.
        errors: Vec<ValidationError>,
    },
    /// The intersection is empty although the census decided an
    /// overlap: the overlap is thinner than the boolean's tolerance.
    Empty,
    /// A face of the intersection descends from no named face of
    /// either copy.
    Unnamed,
}

/// **An overlap of two copies' material at rest** (D10): reported,
/// never refused, and quiet only under the assertion [`quieted_by`]
/// finds.
#[derive(Debug, Clone)]
pub struct InterferenceFinding {
    /// The copy earlier in gather order.
    pub a: CopyRef,
    /// The other copy.
    pub b: CopyRef,
    /// The census's verdicts on the pair, in its own order: every
    /// finding of one pair carries the same list.
    pub evidence: Vec<ValidationError>,
    /// The overlap.
    pub overlap: Overlap,
    /// The assertion that quiets it, or `None` when it is loud.
    pub quiet: Option<RecipeNodeId>,
}

impl InterferenceFinding {
    /// Whether no assertion quiets it.
    #[must_use]
    pub fn is_loud(&self) -> bool {
        self.quiet.is_none()
    }
}

/// **An assertion that can quiet an interference**: it holds, it reads
/// a `Gap`'s output directly over faces of two different copies, those
/// faces are opposed, and it admits only negative values. Whether it
/// quiets a given overlap is [`quieted_by`]'s.
#[derive(Debug, Clone)]
pub struct GapAssertion {
    /// The assertion.
    pub assertion: RecipeNodeId,
    /// The `Gap`'s outer face: its copy's index in
    /// [`Product::copies`], and the face in the aggregate.
    outer: (usize, FaceKey),
    /// The `Gap`'s inner face, likewise.
    inner: (usize, FaceKey),
}

/// The funnel site name of the quieting rule's decision that a bound
/// is negative.
pub(crate) const AT_REST_QUIET_BOUND: &str = "at_rest_quiet_bound";

/// The funnel site name of the quieting rule's decision that two
/// planar faces' outward normals oppose.
pub(crate) const AT_REST_QUIET_OPPOSED: &str = "at_rest_quiet_opposed";

/// **Every assertion in `doc` that can quiet an interference between
/// two of `copies`** ([`GapAssertion`]), in document order.
///
/// It reads the assertion's verdict in `evaluation` and never its
/// measure's value, and evaluates nothing: an assertion that did not
/// evaluate, or does not hold, is simply not a candidate.
pub(crate) fn gap_assertions<P, T: Decide>(
    doc: &Doc<P>,
    evaluation: &Evaluation<T>,
    aggregate: &Body<T>,
    names: &NameTable,
    copies: &[GatheredCopy<T>],
    tol: Tol,
) -> Vec<GapAssertion> {
    let Ok(band) = Band::linear(tol) else {
        return Vec::new();
    };
    let face_of = |var: VarId| -> Option<(usize, FaceKey)> {
        let select = doc.selection(var)?;
        let [name] = select.names.as_slice() else {
            return None;
        };
        let Some(Entry::Unique(EntityRef {
            key: EntityKey::Face(face),
            ..
        })) = names.lookup(name)
        else {
            return None;
        };
        let solid = aggregate.solid_of_face(*face)?;
        let copy = copies
            .iter()
            .position(|c| c.var == select.body && c.keys.solids().contains(&solid))?;
        Some((copy, *face))
    };
    let mut out = Vec::new();
    for id in doc.ids() {
        let Some(Node::Assertion {
            value, relation, ..
        }) = doc.node(id)
        else {
            continue;
        };
        if !matches!(
            relation,
            AssertionRelation::AtMost | AssertionRelation::Equal
        ) {
            continue;
        }
        let Some((measure, 0)) = doc.var(*value).and_then(|v| v.def().output()) else {
            continue;
        };
        let Some(Node::Measure {
            primitive: MeasurePrimitive::Gap { outer, inner },
        }) = doc.node(measure)
        else {
            continue;
        };
        let Some(ValuePayload::Assertion(AssertionVerdict::Holds { bound, .. })) =
            evaluation.value(id).map(|v| &v.payload)
        else {
            continue;
        };
        if decide(AT_REST_QUIET_BOUND, Margin::of(*bound), band) != Ok(Sign::Negative) {
            continue;
        }
        let (Some(outer), Some(inner)) = (face_of(*outer), face_of(*inner)) else {
            continue;
        };
        if outer.0 == inner.0 || !opposed(aggregate, outer.1, inner.1, band) {
            continue;
        }
        out.push(GapAssertion {
            assertion: id,
            outer,
            inner,
        });
    }
    out
}

/// **Whether a `Gap`'s two faces are opposed**: each faces the other's
/// material, which is the mating configuration C5's gap is written
/// for. Two planes oppose when their outward normals do; a bore and a
/// pin, or a socket and a ball, when the outer face is concave (its
/// material outside its carrier) and the inner one convex. The `Gap`
/// that holds over them already decided their carriers parallel,
/// coaxial or concentric.
fn opposed<T: Decide>(body: &Body<T>, outer: FaceKey, inner: FaceKey, band: Band) -> bool {
    use crate::eval::measure::{Carrier, carrier_of};
    let at = |key| {
        carrier_of(
            body,
            EntityRef {
                body: 0,
                key: EntityKey::Face(key),
            },
        )
    };
    let sense = |key| body.get_face(key).map(|f| f.sense);
    match (at(outer), at(inner)) {
        (Carrier::Plane { outward: wo, .. }, Carrier::Plane { outward: wi, .. }) => {
            decide(AT_REST_QUIET_OPPOSED, Margin::of(wo.dot(wi)), band) == Ok(Sign::Negative)
        }
        (Carrier::Cylinder { .. }, Carrier::Cylinder { .. })
        | (Carrier::Sphere { .. }, Carrier::Sphere { .. }) => {
            sense(outer) == Some(false) && sense(inner) == Some(true)
        }
        _ => false,
    }
}

/// **Which census verdict is interference evidence**, and the two
/// solids it is about: a vertex inside another solid's material, an
/// edge piercing a face, or an in-plane crossing read `SameSide`.
fn interference_solids<T: Decide>(
    body: &Body<T>,
    error: &ValidationError,
) -> Option<(SolidKey, SolidKey)> {
    let of_edge = |edge| {
        let he = body.get_edge(edge)?.he_plus;
        body.solid_of_face(body.face_of_half_edge(he)?)
    };
    match error {
        ValidationError::InstanceInterference { outer, inner, .. } => Some((*outer, *inner)),
        ValidationError::UndeclaredContact {
            contact: CensusContact::EdgeFacePierce { edge, face },
            ..
        } => Some((of_edge(*edge)?, body.solid_of_face(*face)?)),
        ValidationError::UndeclaredContact {
            contact:
                CensusContact::EdgeEdgeCross {
                    a,
                    b,
                    side: Some(CrossingSideVerdict::SameSide),
                },
            ..
        } => Some((of_edge(*a)?, of_edge(*b)?)),
        _ => None,
    }
}

/// **The at-rest census's verdicts, partitioned**: the interference
/// findings between copies, in the census's order of first mention of
/// each pair, and every other verdict, which the gate attributes and
/// refuses as before.
pub(crate) fn partition<T: crate::EvalScalar>(
    product: &Product<T>,
    errors: Vec<ValidationError>,
    tol: Tol,
) -> (Vec<InterferenceFinding>, Vec<ValidationError>) {
    let copy_of: BTreeMap<SolidKey, usize> = product
        .copies
        .iter()
        .enumerate()
        .flat_map(|(i, c)| c.keys.solids().iter().map(move |&s| (s, i)))
        .collect();
    let mut pairs: Vec<((usize, usize), Vec<ValidationError>)> = Vec::new();
    let mut rest = Vec::new();
    for error in errors {
        let pair = interference_solids(&product.body, &error)
            .and_then(|(x, y)| Some((*copy_of.get(&x)?, *copy_of.get(&y)?)))
            .filter(|(x, y)| x != y)
            .map(|(x, y)| (x.min(y), x.max(y)));
        match pair {
            Some(pair) => match pairs.iter_mut().find(|(p, _)| *p == pair) {
                Some((_, evidence)) => evidence.push(error),
                None => pairs.push((pair, vec![error])),
            },
            None => rest.push(error),
        }
    }
    // The containment arm leaves a pair it could not clear undecided
    // when a crossing stands on it; once the crossing is decided
    // interference, that undecided pair is the same overlap.
    let (subsumed, rest): (Vec<_>, Vec<_>) = rest.into_iter().partition(|error| {
        let ValidationError::CensusUndecidable {
            a: topo::EntityId::Solid(x),
            b: topo::EntityId::Solid(y),
            ..
        } = error
        else {
            return false;
        };
        let (Some(&x), Some(&y)) = (copy_of.get(x), copy_of.get(y)) else {
            return false;
        };
        pairs.iter().any(|(p, _)| *p == (x.min(y), x.max(y)))
    });
    for error in subsumed {
        let ValidationError::CensusUndecidable {
            a: topo::EntityId::Solid(x),
            b: topo::EntityId::Solid(y),
            ..
        } = &error
        else {
            unreachable!("partitioned on that shape above")
        };
        let pair = (copy_of[x].min(copy_of[y]), copy_of[x].max(copy_of[y]));
        if let Some((_, evidence)) = pairs.iter_mut().find(|(p, _)| *p == pair) {
            evidence.push(error);
        }
    }
    let findings = pairs
        .into_iter()
        .flat_map(|((a, b), evidence)| localize(product, a, b, evidence, tol))
        .collect();
    (findings, rest)
}

/// One connected overlap: the faces bounding it, as aggregate faces of
/// the two copies and in the intersection's own arena.
struct Component {
    /// The aggregate faces it descends from, each with its copy's index.
    sources: BTreeSet<(usize, FaceKey)>,
    /// Its faces in the intersection's arena.
    faces: Vec<FaceKey>,
}

/// **The findings of one interfering pair**: the two copies
/// intersected, one finding per connected solid of the result, each
/// quieted by [`quieted_by`].
fn localize<T: crate::EvalScalar>(
    product: &Product<T>,
    a: usize,
    b: usize,
    evidence: Vec<ValidationError>,
    tol: Tol,
) -> Vec<InterferenceFinding> {
    let copy_ref = |i: usize| CopyRef {
        var: product.copies[i].var,
        member: product.copies[i].output,
    };
    let finding = |overlap, quiet| InterferenceFinding {
        a: copy_ref(a),
        b: copy_ref(b),
        evidence: evidence.clone(),
        overlap,
        quiet,
    };
    let unlocalized = |why| vec![finding(Overlap::Unlocalized(why), None)];
    let operand = |i: usize| T::gate_at_rest_kept((*product.copies[i].body).clone(), tol);
    let (body_a, body_b) = match (operand(a), operand(b)) {
        (Ok(x), Ok(y)) => (x, y),
        (Err(errors), _) | (_, Err(errors)) => return unlocalized(Unlocalized::Invalid { errors }),
    };
    let result = match topo::intersect(&body_a, &body_b, tol) {
        Ok(topo::BooleanResult::Body(result)) => result,
        Ok(topo::BooleanResult::Empty) => return unlocalized(Unlocalized::Empty),
        Err(refusal) => {
            return unlocalized(Unlocalized::Refused {
                refusal: refusal.to_string(),
            });
        }
    };
    let Some(components) = components(product, a, b, &result) else {
        return unlocalized(Unlocalized::Unnamed);
    };
    let mut out = Vec::with_capacity(components.len());
    for component in components {
        let mut faces = Vec::with_capacity(component.sources.len());
        for &(copy, face) in &component.sources {
            let Some(name) = product.names.name_of(&EntityRef {
                body: 0,
                key: EntityKey::Face(face),
            }) else {
                return unlocalized(Unlocalized::Unnamed);
            };
            faces.push(FaceSite {
                copy: copy_ref(copy),
                face: name.clone(),
            });
        }
        faces.sort();
        faces.dedup();
        let quiet = quieted_by(product, a, b, &component, &result.body, tol);
        out.push(finding(Overlap::Bounded { faces }, quiet));
    }
    out
}

/// The connected solids of `result`, each with the aggregate faces its
/// faces descend from; `None` when a face's lineage does not reach a
/// face of either copy.
fn components<T: Decide>(
    product: &Product<T>,
    a: usize,
    b: usize,
    result: &topo::BooleanBody<T>,
) -> Option<Vec<Component>> {
    let descent = crate::names::FaceDescent::of(&result.naming);
    let merged = descent.merged();
    let mut out = Vec::new();
    for (solid, _) in result.body.solids() {
        let faces = result.body.faces_of_solid(solid)?;
        let mut sources = BTreeSet::new();
        for &face in &faces {
            let alone = [face];
            let held = merged.get(&face).map_or(&alone[..], Vec::as_slice);
            for &piece in held {
                let (operand, root) = descent.result_face(piece).ok()?;
                let copy = match operand {
                    topo::Operand::A => a,
                    topo::Operand::B => b,
                };
                sources.insert((copy, product.copies[copy].keys.face(root)?));
            }
        }
        out.push(Component { sources, faces });
    }
    Some(out)
}

/// **The quieting rule** (D10): the first assertion, in document
/// order, among the product's [`GapAssertion`]s over the pair whose two
/// faces both bound `component`, and between whose carriers every face
/// bounding it lies. `None` is a loud finding.
fn quieted_by<T: Decide>(
    product: &Product<T>,
    a: usize,
    b: usize,
    component: &Component,
    result: &Body<T>,
    tol: Tol,
) -> Option<RecipeNodeId> {
    let band = Band::linear(tol).ok()?;
    product
        .gap_assertions
        .iter()
        .filter(|g| {
            let copies = (g.outer.0.min(g.inner.0), g.outer.0.max(g.inner.0));
            copies == (a, b)
                && component.sources.contains(&g.outer)
                && component.sources.contains(&g.inner)
        })
        .find(|g| {
            component
                .faces
                .iter()
                .all(|&face| between(&product.body, g, result, face, band))
        })
        .map(|g| g.assertion)
}

/// **Whether `face` of the intersection lies between the carriers of
/// `g`'s two faces.**
fn between<T: Decide>(
    _aggregate: &Body<T>,
    _g: &GapAssertion,
    _result: &Body<T>,
    _face: FaceKey,
    _band: Band,
) -> bool {
    false
}
