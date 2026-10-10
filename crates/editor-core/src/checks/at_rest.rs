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
    /// An assertion's faces bound the overlap, and the kernel could not
    /// say whether the overlap lies between their carriers.
    Containment {
        /// Why, in the refusing door's words.
        refusal: String,
    },
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
    // What the census left undecided between two copies it decided
    // interfere — a crossing standing on the pair, curved faces within
    // reach of each other — is the same overlap, which the intersection
    // below localizes. A pair it decided nothing about stays refused as
    // it left it.
    let undecided_pair = |error: &ValidationError| match error {
        ValidationError::CensusUndecidable { a, b, .. } => {
            let (x, y) = (
                *copy_of.get(&solid_of(&product.body, *a)?)?,
                *copy_of.get(&solid_of(&product.body, *b)?)?,
            );
            (x != y).then_some((x.min(y), x.max(y)))
        }
        _ => None,
    };
    let mut kept = Vec::new();
    for error in rest {
        match undecided_pair(&error).and_then(|pair| pairs.iter_mut().find(|(p, _)| *p == pair)) {
            Some((_, evidence)) => evidence.push(error),
            None => kept.push(error),
        }
    }
    let findings = pairs
        .into_iter()
        .flat_map(|((a, b), evidence)| localize(product, a, b, evidence, tol))
        .collect();
    (findings, kept)
}

/// The solid a census entity belongs to: itself, or a face's.
fn solid_of<T: Decide>(body: &Body<T>, entity: topo::EntityId) -> Option<SolidKey> {
    match entity {
        topo::EntityId::Solid(s) => Some(s),
        topo::EntityId::Face(f) => body.solid_of_face(f),
        _ => None,
    }
}

/// **Two copies intersected**: the overlap of their material, `None`
/// when it is empty.
///
/// # Errors
///
/// Why it could not be formed.
fn intersection<T: crate::EvalScalar>(
    product: &Product<T>,
    a: usize,
    b: usize,
    tol: Tol,
) -> Result<Option<topo::BooleanBody<T>>, Unlocalized> {
    let operand = |i: usize| T::gate_at_rest_kept((*product.copies[i].body).clone(), tol);
    let (body_a, body_b) = match (operand(a), operand(b)) {
        (Ok(x), Ok(y)) => (x, y),
        (Err(errors), _) | (_, Err(errors)) => return Err(Unlocalized::Invalid { errors }),
    };
    match topo::intersect(&body_a, &body_b, tol) {
        Ok(topo::BooleanResult::Body(result)) => Ok(Some(result)),
        Ok(topo::BooleanResult::Empty) => Ok(None),
        Err(refusal) => Err(Unlocalized::Refused {
            refusal: refusal.to_string(),
        }),
    }
}

/// One connected overlap: the faces bounding it, as aggregate faces of
/// the two copies and in the intersection's own arena.
struct Component {
    /// Its solid in the intersection's arena.
    solid: SolidKey,
    /// The aggregate faces it descends from, each with its copy's index.
    sources: BTreeSet<(usize, FaceKey)>,
}

/// **The findings of one interfering pair**: one per connected solid of
/// the two copies' intersection, each quieted by [`quiet_verdicts`].
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
    let result = match intersection(product, a, b, tol) {
        Ok(Some(result)) => result,
        Ok(None) => return unlocalized(Unlocalized::Empty),
        Err(why) => return unlocalized(why),
    };
    let Some(components) = components(product, a, b, &result) else {
        return unlocalized(Unlocalized::Unnamed);
    };
    let verdicts = quiet_verdicts(product, a, b, &components, &result, tol);
    // A quiet finding must not carry a verdict nothing checked: every
    // undecided verdict on the pair names faces of the component it
    // rides, which its containment check covered.
    let covered = |component: &Component| {
        evidence.iter().all(|error| match error {
            ValidationError::CensusUndecidable {
                a: topo::EntityId::Face(x),
                b: topo::EntityId::Face(y),
                ..
            } => [x, y]
                .into_iter()
                .all(|f| component.sources.iter().any(|(_, s)| s == f)),
            ValidationError::CensusUndecidable { .. } => false,
            _ => true,
        })
    };
    let mut out = Vec::with_capacity(components.len());
    for (component, verdict) in components.iter().zip(verdicts) {
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
        out.push(match verdict {
            Quiet::Loud => finding(Overlap::Bounded { faces }, None),
            Quiet::By(assertion) if covered(component) => {
                finding(Overlap::Bounded { faces }, Some(assertion))
            }
            Quiet::By(_) => finding(Overlap::Bounded { faces }, None),
            Quiet::Refused(refusal) => finding(
                Overlap::Unlocalized(Unlocalized::Containment { refusal }),
                None,
            ),
        });
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
        out.push(Component { solid, sources });
    }
    Some(out)
}

/// What the quieting rule says of one component.
enum Quiet {
    /// No assertion quiets it.
    Loud,
    /// This assertion quiets it.
    By(RecipeNodeId),
    /// An assertion whose faces bound it could not be checked against
    /// it, and none quiets it: the kernel could not bound the overlap
    /// against the carriers.
    Refused(String),
}

/// **The quieting rule** (D10), per component of one pair's
/// intersection: the first assertion, in document order, among the
/// product's [`GapAssertion`]s over the pair whose two faces both bound
/// the component and between whose carriers the component lies.
///
/// "Between the carriers" is decided by the kernel's own boolean: for
/// each of the assertion's two faces, the region on the far side of
/// its carrier from its copy's material is built as a solid
/// ([`beyond`]) and intersected with the pair's components, and a
/// component lies between when neither intersection keeps any of it.
/// A component touching a carrier is between it; one that crosses it
/// (a sunk flange, a pin bottoming in a blind bore) is not.
fn quiet_verdicts<T: crate::EvalScalar>(
    product: &Product<T>,
    a: usize,
    b: usize,
    components: &[Component],
    result: &topo::BooleanBody<T>,
    tol: Tol,
) -> Vec<Quiet> {
    let mut verdicts: Vec<Quiet> = components.iter().map(|_| Quiet::Loud).collect();
    for g in &product.gap_assertions {
        if (g.outer.0.min(g.inner.0), g.outer.0.max(g.inner.0)) != (a, b) {
            continue;
        }
        let bounded: Vec<usize> = components
            .iter()
            .enumerate()
            .filter(|(_, c)| c.sources.contains(&g.outer) && c.sources.contains(&g.inner))
            .filter(|(i, _)| !matches!(verdicts[*i], Quiet::By(_)))
            .map(|(i, _)| i)
            .collect();
        if bounded.is_empty() {
            continue;
        }
        let outside = [g.outer.1, g.inner.1]
            .into_iter()
            .map(|face| crossing(&product.body, face, components, result, tol))
            .collect::<Result<Vec<_>, String>>()
            .map(|sets| sets.into_iter().flatten().collect::<BTreeSet<usize>>());
        for i in bounded {
            match &outside {
                Ok(crossed) if !crossed.contains(&i) => verdicts[i] = Quiet::By(g.assertion),
                Ok(_) => {}
                Err(refusal) => {
                    if matches!(verdicts[i], Quiet::Loud) {
                        verdicts[i] = Quiet::Refused(refusal.clone());
                    }
                }
            }
        }
    }
    verdicts
}

/// The components of `result` that reach past the carrier of the
/// aggregate face `face`, away from its copy's material.
///
/// # Errors
///
/// Why the kernel could not answer: a carrier or a component face with
/// no lane here, or the boolean's own refusal.
fn crossing<T: crate::EvalScalar>(
    aggregate: &Body<T>,
    face: FaceKey,
    components: &[Component],
    result: &topo::BooleanBody<T>,
    tol: Tol,
) -> Result<BTreeSet<usize>, String> {
    let beyond = beyond(aggregate, face, &result.body, tol)?;
    let kept = match topo::intersect(&result.body, &beyond, tol) {
        Ok(topo::BooleanResult::Empty) => return Ok(BTreeSet::new()),
        Ok(topo::BooleanResult::Body(kept)) => kept,
        Err(refusal) => return Err(refusal.to_string()),
    };
    let descent = crate::names::FaceDescent::of(&kept.naming);
    let of_solid: BTreeMap<SolidKey, usize> = components
        .iter()
        .enumerate()
        .map(|(i, c)| (c.solid, i))
        .collect();
    let mut out = BTreeSet::new();
    for (solid, _) in kept.body.solids() {
        let mut named = false;
        for f in kept.body.faces_of_solid(solid).unwrap_or_default() {
            if let Ok((topo::Operand::A, root)) = descent.result_face(f)
                && let Some(&i) = result
                    .body
                    .solid_of_face(root)
                    .and_then(|s| of_solid.get(&s))
            {
                out.insert(i);
                named = true;
            }
        }
        // A kept piece bounded by the beyond solid's faces alone lies
        // inside some component and names none: every one is crossed.
        if !named {
            out.extend(0..components.len());
        }
    }
    Ok(out)
}

/// The funnel site name of the frame the beyond region is built on.
pub(crate) const AT_REST_BEYOND_FRAME: &str = "at_rest_beyond_frame";

/// **The region past the carrier of aggregate face `face`**, away from
/// its copy's material, as a solid that holds every point of `result`
/// in that region: a box on the far side of a plane, the solid
/// cylinder inside a bore, the tube outside a pin. Its sizes come from
/// the reach of `result`'s faces, so it is built in `f64` and lifted
/// onto the scalar's frame, as the evaluator lifts a profile.
///
/// # Errors
///
/// A carrier with no arm here (a sphere, a cone), a face of `result`
/// whose reach this cannot bound, or a construction refusal.
fn beyond<T: crate::EvalScalar>(
    aggregate: &Body<T>,
    face: FaceKey,
    result: &Body<T>,
    tol: Tol,
) -> Result<topo::AtRestBody<T>, String> {
    use crate::eval::measure::{Carrier, carrier_of, reach_of};
    use geom_core::{OrthoFrame, Point2, Point3, Vec3};
    use profile::{Step, Target};
    let nominal = |x: T| 0.5 * (x.lo() + x.hi());
    let point = |p: Point3<T>| Point3::new(nominal(p.x), nominal(p.y), nominal(p.z));
    let vector = |v: Vec3<T>| Vec3::new(nominal(v.x), nominal(v.y), nominal(v.z));
    let band = Band::linear(tol).map_err(|e| e.to_string())?;
    let sense = aggregate
        .get_face(face)
        .map(|f| f.sense)
        .ok_or("an unreadable face")?;
    let carrier = carrier_of(
        aggregate,
        EntityRef {
            body: 0,
            key: EntityKey::Face(face),
        },
    );
    let origin = match &carrier {
        Carrier::Plane { origin, .. } | Carrier::Cylinder { origin, .. } => *origin,
        _ => return Err("the containment check has no lane for this carrier".to_owned()),
    };
    // Every face of the intersection a plane or a cylinder patch, whose
    // reach its own boundary bounds.
    let mut reach = 0.0_f64;
    for (f, held) in result.faces() {
        match result.get_surface(held.surface) {
            Some(topo::Surface::Plane { .. } | topo::Surface::Cylinder { .. }) => {}
            _ => return Err("a face of the overlap the containment check cannot bound".to_owned()),
        }
        let r = reach_of(result, f, origin).ok_or("a face of the overlap with no reach")?;
        reach = reach.max(r.hi());
    }
    let pad = reach.mul_add(0.5, 1e3 * tol.eps());
    let span = reach + pad;
    let rectangle = |h: f64| {
        vec![
            Step::At(Point2::new(-h, -h)),
            Step::LineTo(Target::Point(Point2::new(h, -h))),
            Step::LineTo(Target::Point(Point2::new(h, h))),
            Step::LineTo(Target::Point(Point2::new(-h, h))),
            Step::LineTo(Target::Start),
        ]
    };
    let circle = |radius: f64| {
        vec![Step::Circle {
            centre: Point2::new(0.0, 0.0),
            radius,
        }]
    };
    // The frame's base point and normal, the loops in its plane, and the
    // depth the region is swept along the normal.
    let (base, normal, loops, depth) = match carrier {
        Carrier::Plane {
            origin, outward, ..
        } => (point(origin), vector(outward), vec![rectangle(span)], span),
        Carrier::Cylinder {
            origin,
            axis,
            radius,
            ..
        } => {
            let (axis, radius) = (vector(axis), nominal(radius));
            let base = point(origin) - axis * span;
            // A concave face (a bore) has its material outside the
            // carrier, so the region past it is the solid cylinder; a
            // convex one (a pin) the tube around it.
            let loops = if sense {
                vec![circle(radius + 2.0 * span), circle(radius)]
            } else {
                vec![circle(radius)]
            };
            (base, axis, loops, 2.0 * span)
        }
        _ => unreachable!("the carrier was matched above"),
    };
    let frame = [Vec3::new(1.0, 0.0, 0.0), Vec3::new(0.0, 1.0, 0.0)]
        .into_iter()
        .find_map(|reference| {
            OrthoFrame::from_axis_and_reference(base, normal, reference, AT_REST_BEYOND_FRAME, band)
                .ok()
        })
        .ok_or("the carrier's normal spans no frame")?;
    let loops = loops
        .iter()
        .map(|steps| profile::replay(steps, tol).map_err(|e| format!("{e:?}")))
        .collect::<Result<Vec<_>, String>>()?;
    let plane = profile::SketchPlane::from_frame(frame);
    let validated = profile::ConstructedProfile::new(plane, loops)
        .validate(tol)
        .map_err(|e| e.to_string())?
        .lift_onto(plane.map(T::from_f64));
    let swept = sweep::extrude(
        &validated,
        sweep::Extrusion::Distance {
            depth: T::from_f64(depth),
            side: sweep::ExtrudeSide::Along,
        },
        tol,
    )
    .map_err(|e| e.to_string())?;
    T::gate_at_rest_kept(swept.body, tol).map_err(|errors| format!("{errors:?}"))
}
