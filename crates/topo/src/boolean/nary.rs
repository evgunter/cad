//! **The n-ary union and intersect** (DM4, `[operations.operators]`):
//! one operation over a list of members in the author's order.
//!
//! It judges every member pair whose boxes meet once, as a pair
//! boolean, then folds the members in list order, `((m0 ∘ m1) ∘ m2) ∘ …`.
//! Each fold step is given the judgements' verdicts on the member it
//! joins, read through the face lineage the door advances from its own
//! steps' naming ([`MemberLineage`]), and decides no carrier pair of its
//! own; it carries the accumulation's records and the joining member's.
//! Two members are one pair: their one step is the judgement.
//!
//! Where the operation stops, at a refusal or an intersect's empty
//! result, the door returns what it built up to there
//! ([`NaryRun::end`]), so a caller that reads each judgement and step in
//! turn meets the operation's events in the order they happened.
//!
//! A verdict is handed to a step only here, over member bodies the door
//! holds, so no caller states a glue no record over those bodies states.

use std::collections::{BTreeMap, BTreeSet};

use geom_core::{Bounds, Decide, Real, Tol};

use super::ops::boolean_op_given;
use super::{
    BooleanBody, BooleanError, BooleanInputs, BooleanNaming, BooleanOp, BooleanResult, GlueVerdict,
    GluedPair, Operand, OperandRecords, SweepStrategy,
};
use crate::AtRestBody;
use crate::body::Body;
use crate::entity::FaceKey;
use crate::props::AtRestPolicy;

/// The two operators that take a list of members.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum NaryOp {
    /// The material in any member.
    Union,
    /// The material in every member.
    Intersect,
}

impl NaryOp {
    /// The pair boolean each judgement and each step runs.
    pub fn op(self) -> BooleanOp {
        match self {
            Self::Union => BooleanOp::Union,
            Self::Intersect => BooleanOp::Intersect,
        }
    }
}

/// **What one member carries besides its body**: its own contact
/// records, carried into the step that joins it, and its rank, which
/// orders a pairwise judgement's two operands: the member of the lesser
/// rank is operand A, the earlier listed on a tie. With two members no
/// rank is read: their one step is `m0 ∘ m1`, in list order.
#[derive(Clone, Debug, Default)]
pub struct MemberInputs {
    /// The member's own contact records.
    pub records: OperandRecords,
    /// The member's rank.
    pub rank: u32,
}

/// One member of an n-ary operation: its body and what it carries.
#[derive(Clone, Copy, Debug)]
pub struct NaryMember<'m, T: Real> {
    /// The member's finished body.
    pub body: &'m AtRestBody<T>,
    /// Its records and rank.
    pub inputs: &'m MemberInputs,
}

/// **The member pairs certified apart** before any judgement runs:
/// each `(a, b)`, `a < b`, a pair whose two bodies are rigid images of
/// one body that its separation certificate separates
/// ([`crate::Separation::apart`]). A pair certified apart cannot meet,
/// so its judgement would find nothing and is not run; every other pair
/// whose hulls meet is judged.
#[derive(Clone, Debug, Default, PartialEq, Eq)]
pub struct Apart(BTreeSet<(usize, usize)>);

impl Apart {
    /// The pairs `pairs`, each read in ascending order.
    #[must_use]
    pub fn of(pairs: impl IntoIterator<Item = (usize, usize)>) -> Self {
        Self(
            pairs
                .into_iter()
                .map(|(a, b)| (a.min(b), a.max(b)))
                .collect(),
        )
    }

    /// Whether members `a` and `b` are certified apart.
    #[must_use]
    pub fn holds(&self, a: usize, b: usize) -> bool {
        self.0.contains(&(a.min(b), a.max(b)))
    }

    /// How many pairs are certified apart.
    #[must_use]
    pub fn len(&self) -> usize {
        self.0.len()
    }

    /// Whether no pair is.
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.0.is_empty()
    }
}

/// **A member face**: the member's place in the list and the face in
/// that member's keys.
pub type MemberFace = (usize, FaceKey);

/// **Which member faces each face of an accumulation descends from**,
/// read off each step's naming, never off a name: a piece of a face
/// descends from what the face does, a merged face from what each face
/// it absorbed does, and a face of the joining member from that face.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct MemberLineage {
    faces: BTreeMap<FaceKey, BTreeSet<MemberFace>>,
}

impl MemberLineage {
    /// The lineage of member `member`'s own body: each face its own.
    pub fn of_member<T: Real>(member: usize, body: &Body<T>) -> Self {
        Self {
            faces: body
                .faces()
                .map(|(f, _)| (f, BTreeSet::from([(member, f)])))
                .collect(),
        }
    }

    /// The member faces face `f` descends from; `None` for a face the
    /// accumulation does not hold.
    pub fn member_faces(&self, f: FaceKey) -> Option<&BTreeSet<MemberFace>> {
        self.faces.get(&f)
    }

    /// The member faces `f` descends from.
    ///
    /// # Panics
    ///
    /// Where `f` has no lineage: every face of every accumulation the
    /// door builds has one.
    #[track_caller]
    fn of(&self, f: FaceKey) -> &BTreeSet<MemberFace> {
        self.faces.get(&f).unwrap_or_else(|| {
            unreachable!(
                "accumulation face {f:?} has no member-face lineage, though the n-ary door \
                 records one for every face of every accumulation it builds"
            )
        })
    }

    /// **The member faces an operand face of the step joining `member`
    /// to this accumulation descends from**: an accumulation face's
    /// lineage for operand A, the member's own face for operand B. `f`
    /// is a root face in that operand's keys ([`FaceDescent`]).
    ///
    /// # Panics
    ///
    /// Where an operand-A face has no lineage here.
    #[track_caller]
    pub fn operand_faces(
        &self,
        member: usize,
        operand: Operand,
        f: FaceKey,
    ) -> BTreeSet<MemberFace> {
        match operand {
            Operand::A => self.of(f).clone(),
            Operand::B => BTreeSet::from([(member, f)]),
        }
    }

    /// The lineage of `result`, the step that joined member `member` to
    /// this accumulation with the naming `naming`.
    fn step<T: Real>(&self, member: usize, naming: &BooleanNaming, result: &Body<T>) -> Self {
        let descent = FaceDescent::of(naming);
        let from = |(operand, f): (Operand, FaceKey)| self.operand_faces(member, operand, f);
        let merged = descent.merged();
        let faces = result
            .faces()
            .map(|(f, _)| {
                let mut set = BTreeSet::new();
                for &g in merged.get(&f).map_or(&[f][..], Vec::as_slice) {
                    set.extend(from(descent.result_face(g)));
                }
                (f, set)
            })
            .collect();
        Self { faces }
    }
}

/// **Which operand face each face of a boolean result descends from**,
/// by entity: the operand and the root face in that operand's space,
/// read through the graft rows and the fragment rows.
pub struct FaceDescent<'n> {
    naming: &'n BooleanNaming,
    inv_faces: BTreeMap<FaceKey, FaceKey>,
}

impl<'n> FaceDescent<'n> {
    /// The descent `naming` records.
    pub fn of(naming: &'n BooleanNaming) -> Self {
        Self {
            naming,
            inv_faces: naming.graft_faces.iter().map(|&(s, d)| (d, s)).collect(),
        }
    }

    /// The operand face result face `f` descends from. A merged face
    /// descends from each face it absorbed as well: see
    /// [`FaceDescent::merged`].
    ///
    /// # Panics
    ///
    /// As [`BooleanNaming::operand_of`].
    #[track_caller]
    pub fn result_face(&self, f: FaceKey) -> (Operand, FaceKey) {
        let (operand, k, _) = self.naming.operand_of(&self.inv_faces, f);
        (operand, self.clone_face(operand, k))
    }

    /// The operand face a face in `operand`'s CLONE keys is a fragment
    /// of (a discarded face's key is one).
    pub fn clone_face(&self, operand: Operand, f: FaceKey) -> FaceKey {
        match operand {
            Operand::A => self.naming.face_fragments_a.root(f),
            Operand::B => self.naming.face_fragments_b.root(f),
        }
    }

    /// Merged result face → every face it holds, itself first, in
    /// result keys before the merge.
    pub fn merged(&self) -> BTreeMap<FaceKey, Vec<FaceKey>> {
        self.naming
            .merge_groups
            .iter()
            .map(|(kept, absorbed)| {
                (
                    *kept,
                    core::iter::once(*kept)
                        .chain(absorbed.iter().copied())
                        .collect(),
                )
            })
            .collect()
    }
}

/// **One pairwise judgement**: members `a` (operand A) and `b`
/// (operand B), run as their own pair boolean.
#[derive(Debug)]
pub struct NaryJudgement<T: Real> {
    /// Operand A's member.
    pub a: usize,
    /// Operand B's member.
    pub b: usize,
    /// The pair boolean's result. It is run with no records carried in,
    /// so its contacts are the touches it decided between the two
    /// members, complete relative to no member records: the members'
    /// own records reach the result through the fold steps.
    pub body: BooleanBody<T>,
}

/// **A judged row**: a row of one judgement's coincidences, by the
/// judgement's place in [`NaryRun::judgements`] and the row's in its
/// [`BooleanBody::coincidences`].
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct JudgedRow {
    /// The judgement's place.
    pub judgement: usize,
    /// The row's place.
    pub row: usize,
}

/// **One fold step**: member `member` joined to the accumulation of
/// every member before it (operand A) as operand B.
#[derive(Debug)]
pub struct NaryStep<T: Real> {
    /// The member the step joins.
    pub member: usize,
    /// The pair boolean's result: the accumulation after the step.
    pub body: BooleanBody<T>,
    /// Each row of the step's coincidences that restates a given pair's
    /// decision (a tangency a face–face record cites), with the rows of
    /// the judgements whose verdicts it stands for, in judgement order:
    /// by the row's place, ascending.
    pub restated: Vec<(usize, Vec<JudgedRow>)>,
    /// The member faces each face of [`Self::body`] descends from.
    pub lineage: MemberLineage,
}

impl<T: Real> NaryStep<T> {
    /// The judged rows the step's row `row` restates; `None` for a row
    /// the step decided.
    pub fn restates(&self, row: usize) -> Option<&[JudgedRow]> {
        self.restated
            .binary_search_by_key(&row, |(k, _)| *k)
            .ok()
            .map(|i| self.restated[i].1.as_slice())
    }
}

/// **What an n-ary operation ran**: its judgements, then its fold
/// steps, each in the order run, and how it ended. Where it built, the
/// last step's body is the result; where it stopped, these are what it
/// ran before the stop. With two members there is no judgement apart
/// from the one step, which is the pair's.
#[derive(Debug)]
pub struct NaryRun<T: Real> {
    /// Each judgement, in the order judged: every member pair whose
    /// boxes meet, in list order.
    pub judgements: Vec<NaryJudgement<T>>,
    /// Each fold step, in fold order: step `k` joins member `k + 1`.
    pub steps: Vec<NaryStep<T>>,
    /// Whether the members were judged pairwise before the fold: three
    /// or more were; two are one pair, judged by their one step.
    pub pairwise: bool,
    /// How the operation ended.
    pub end: NaryEnd,
}

/// **How an n-ary operation ended**, after the judgements and steps its
/// [`NaryRun`] holds.
#[derive(Debug)]
pub enum NaryEnd {
    /// The last step's body is the result.
    Built,
    /// An intersect whose next judgement or step is empty: the typed
    /// empty result.
    Empty,
    /// The next judgement or step refused.
    Refused(NaryRefusal),
}

impl<T: Real> NaryRun<T> {
    /// The result, where the operation built: the last step's body.
    pub fn result(&self) -> Option<&BooleanBody<T>> {
        match self.end {
            NaryEnd::Built => self.steps.last().map(|s| &s.body),
            NaryEnd::Empty | NaryEnd::Refused(_) => None,
        }
    }
}

/// **Why an n-ary operation refused**, and where.
#[derive(Debug)]
pub enum NaryRefusal {
    /// Fewer than two members: one member's union is that body, and
    /// none's is not an operation's.
    Arity {
        /// How many were given.
        members: usize,
    },
    /// A member's box could not be read.
    Hull {
        /// The member.
        member: usize,
        /// The box door's refusal.
        error: Box<BooleanError>,
    },
    /// A pair's judgement refused: with three or more members, a pairwise
    /// judgement, operand A the lesser rank, so the same pair refuses
    /// alike in every member order; with two members, their one step,
    /// `a` 0 and `b` 1 in list order, no rank read.
    Judgement {
        /// Operand A's member.
        a: usize,
        /// Operand B's member.
        b: usize,
        /// The pair boolean's refusal.
        error: Box<BooleanError>,
    },
    /// The fold step joining `member` refused.
    Step {
        /// The member the step joins.
        member: usize,
        /// The pair boolean's refusal.
        error: Box<BooleanError>,
    },
    /// Two judgements' verdicts glue one accumulation face to one face
    /// of the member a step joins under two relations: two decisions
    /// from values, of two member pairs, that a merge across their
    /// faces brought onto one pair.
    VerdictsDisagree {
        /// The member the step joins.
        member: usize,
        /// The accumulation face.
        face: FaceKey,
        /// The member's face.
        joining: FaceKey,
    },
}

impl NaryRefusal {
    /// The pair boolean's or the box door's refusal, wherever it was
    /// raised; `None` for an arity refusal and for verdicts that
    /// disagree, which no boolean raised.
    pub fn error(&self) -> Option<&BooleanError> {
        match self {
            Self::Arity { .. } | Self::VerdictsDisagree { .. } => None,
            Self::Hull { error, .. } | Self::Judgement { error, .. } | Self::Step { error, .. } => {
                Some(error)
            }
        }
    }
}

impl core::fmt::Display for NaryRefusal {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::Arity { members } => write!(
                f,
                "an n-ary boolean takes two or more members, and was given {members}"
            ),
            Self::Hull { member, error } => write!(f, "member {member}'s box: {error}"),
            Self::Judgement { a, b, error } => {
                write!(f, "the judgement of members {a} and {b}: {error}")
            }
            Self::Step { member, error } => {
                write!(f, "the fold step joining member {member}: {error}")
            }
            Self::VerdictsDisagree {
                member,
                face,
                joining,
            } => write!(
                f,
                "two judgements glue accumulation face {face:?} to member {member}'s face \
                 {joining:?} under two relations"
            ),
        }
    }
}

impl core::error::Error for NaryRefusal {}

/// **One carrier pair a judgement glued**: the kernel's verdict, the
/// two members it judged, and the judgement row recording it.
struct PairVerdict<T: Real> {
    a: usize,
    b: usize,
    verdict: GlueVerdict<T>,
    row: JudgedRow,
}

/// **The n-ary union or intersect of `members`, in their order**
/// (module docs).
///
/// Each member pair whose boxes meet ([`crate::Separation::hull`]) is
/// judged as its own pair boolean, operand A the member of the lesser
/// rank ([`MemberInputs::rank`]), in list order; a pair whose boxes do
/// not meet is not run. Whether one refuses depends on the members,
/// never on their order. Then the fold: step `k` joins member `k + 1`
/// to the accumulation, given each verdict on that member between its
/// face and every accumulation face whose lineage holds the verdict's
/// other face, carrying the accumulation's records and the member's.
/// Two members are one step, `m0 ∘ m1`, and no rank is read.
///
/// The operation stops at the first refusal, and an intersect at the
/// first empty judgement or step (the typed empty result); the run
/// holds what came before ([`NaryRun::end`]).
///
/// # The given channel is this door's own
///
/// A step's glue is handed to it only here, over the members the door
/// holds. Outside the crate no route states one. There is no public
/// given pair, and that privacy is the fence on both crate-private
/// routes that would state one: making one from a verdict
/// (`GlueVerdict::given_at` returns one) and the pair boolean that takes
/// them (`boolean_op_given`'s signature names them), so neither could be
/// named outside the crate were it public.
///
/// ```compile_fail,E0603
/// let _: Option<topo::boolean::GluedPair> = None;
/// ```
///
/// Nor has a boolean's inputs a field to hand one in.
///
/// ```compile_fail,E0560
/// let _ = topo::BooleanInputs {
///     given: None,
///     ..topo::BooleanInputs::default()
/// };
/// ```
///
/// # Panics
///
/// On a kernel bug: a union judgement or step that is empty, two
/// members being finished non-empty bodies; a step's face-pair row that
/// restates none of its given pairs (debug builds); the pair boolean's
/// own.
pub fn nary_boolean<T: Decide + Bounds + AtRestPolicy>(
    op: NaryOp,
    members: &[NaryMember<'_, T>],
    sweep: SweepStrategy,
    tol: Tol,
) -> NaryRun<T> {
    nary_boolean_apart(op, members, &Apart::default(), sweep, tol)
}

/// [`nary_boolean`] over members some pairs of which are certified
/// apart ([`Apart`]): those pairs are not judged, and the run is the
/// one a judgement of each, finding nothing, would make.
///
/// # Panics
///
/// As [`nary_boolean`].
pub fn nary_boolean_apart<T: Decide + Bounds + AtRestPolicy>(
    op: NaryOp,
    members: &[NaryMember<'_, T>],
    apart: &Apart,
    sweep: SweepStrategy,
    tol: Tol,
) -> NaryRun<T> {
    // Two members are one pair: its one step is its judgement.
    let pairwise = members.len() > 2;
    let mut run = NaryRun {
        judgements: Vec::new(),
        steps: Vec::new(),
        pairwise,
        end: NaryEnd::Built,
    };
    run.end = match run_members(op, members, apart, sweep, tol, &mut run) {
        Ok(()) => NaryEnd::Built,
        Err(end) => end,
    };
    run
}

/// [`nary_boolean`]'s body, filling `run`'s judgements and steps as it
/// runs, pairwise as `run` says, and ending early with how it stopped.
fn run_members<T: Decide + Bounds + AtRestPolicy>(
    op: NaryOp,
    members: &[NaryMember<'_, T>],
    apart: &Apart,
    sweep: SweepStrategy,
    tol: Tol,
    run: &mut NaryRun<T>,
) -> Result<(), NaryEnd> {
    let NaryRun {
        judgements,
        steps,
        pairwise,
        ..
    } = run;
    let pairwise = *pairwise;
    if members.len() < 2 {
        return Err(NaryEnd::Refused(NaryRefusal::Arity {
            members: members.len(),
        }));
    }
    let refused = NaryEnd::Refused;
    let hulls = members
        .iter()
        .enumerate()
        .map(|(member, m)| {
            crate::Separation::of(m.body, tol)
                .map(|s| s.hull())
                .map_err(|error| {
                    refused(NaryRefusal::Hull {
                        member,
                        error: Box::new(error),
                    })
                })
        })
        .collect::<Result<Vec<_>, _>>()?;
    let empty = |what: &str| match op {
        NaryOp::Intersect => Err(NaryEnd::Empty),
        NaryOp::Union => unreachable!(
            "a union's {what} returned empty, though both operands are finished non-empty \
             bodies and their union holds each"
        ),
    };
    let mut verdicts = Vec::new();
    if pairwise {
        let ranks: Vec<u32> = members.iter().map(|m| m.inputs.rank).collect();
        for (a, b) in judged_pairs(&hulls, &ranks).filter(|&(a, b)| !apart.holds(a, b)) {
            let judged = boolean_op_given(
                op.op(),
                members[a].body,
                members[b].body,
                &BooleanInputs::default(),
                None,
                sweep,
                tol,
            )
            .map_err(|error| {
                refused(NaryRefusal::Judgement {
                    a,
                    b,
                    error: Box::new(error),
                })
            })?;
            let BooleanResult::Body(body) = judged else {
                return empty("pairwise judgement");
            };
            let judgement = judgements.len();
            verdicts.extend(body.verdicts.iter().map(|&verdict| PairVerdict {
                a,
                b,
                verdict,
                row: JudgedRow {
                    judgement,
                    row: verdict_row(&body.coincidences, &verdict, (a, b)),
                },
            }));
            judgements.push(NaryJudgement { a, b, body });
        }
    }
    let mut lineage = MemberLineage::of_member(0, members[0].body);
    let mut records_a = members[0].inputs.records.clone();
    for member in 1..members.len() {
        let acc = steps.last().map_or(members[0].body, |s| &s.body.body);
        let given = if pairwise {
            Some(given_pairs(&verdicts, member, &lineage, acc).map_err(refused)?)
        } else {
            None
        };
        let inputs = BooleanInputs {
            records_a,
            records_b: members[member].inputs.records.clone(),
        };
        let stepped = boolean_op_given(
            op.op(),
            acc,
            members[member].body,
            &inputs,
            given.as_deref(),
            sweep,
            tol,
        )
        .map_err(|error| {
            let error = Box::new(error);
            refused(if pairwise {
                NaryRefusal::Step { member, error }
            } else {
                NaryRefusal::Judgement { a: 0, b: 1, error }
            })
        })?;
        let BooleanResult::Body(body) = stepped else {
            return empty("fold step");
        };
        let restated = match &given {
            Some(given) => restated_rows(&body.coincidences, given, &verdicts, member, &lineage),
            None => Vec::new(),
        };
        lineage = lineage.step(member, &body.naming, &body.body);
        records_a = body.contacts.carried();
        steps.push(NaryStep {
            member,
            body,
            restated,
            lineage: lineage.clone(),
        });
    }
    Ok(())
}

/// **The member pairs judged, as (operand A, operand B), in the order
/// judged**: each pair whose boxes `hulls` meet, in list order, its
/// operand A the member of the lesser rank, the earlier listed on a
/// tie. So one pair is one run of the pair boolean in every member
/// order.
fn judged_pairs<'h>(
    hulls: &'h [bvh::Aabb],
    ranks: &'h [u32],
) -> impl Iterator<Item = (usize, usize)> + 'h {
    (0..hulls.len()).flat_map(move |p| {
        (p + 1..hulls.len())
            .filter(move |&q| hulls[p].overlaps(&hulls[q]))
            .map(move |q| if ranks[q] < ranks[p] { (q, p) } else { (p, q) })
    })
}

/// The row of `rows`, a judgement's of members `a` and `b`, recording
/// `verdict`.
///
/// # Panics
///
/// Where no row does: the boolean records the row of every verdict it
/// glues by.
fn verdict_row<T: Real>(
    rows: &[crate::Coincidence],
    verdict: &GlueVerdict<T>,
    (a, b): (usize, usize),
) -> usize {
    rows.iter()
        .position(|row| {
            row.face_pair() == Some((verdict.a(), verdict.b()))
                && row.relation == verdict.relation()
        })
        .unwrap_or_else(|| {
            unreachable!(
                "the judgement of members {a} and {b} glued {:?}/{:?} under {:?} with no row \
                 recording it, though the boolean records the row of every verdict it glues by",
                verdict.a(),
                verdict.b(),
                verdict.relation()
            )
        })
}

/// The face of `v` on `member`'s side and the other member's face, or
/// `None` where `v` is not on `member`: which side of the verdict the
/// accumulation holds, the other member's face, and `member`'s face.
fn sides<T: Real>(v: &PairVerdict<T>, member: usize) -> Option<(Operand, MemberFace, FaceKey)> {
    if v.b == member {
        Some((Operand::A, (v.a, v.verdict.a()), v.verdict.b()))
    } else if v.a == member {
        Some((Operand::B, (v.b, v.verdict.b()), v.verdict.a()))
    } else {
        None
    }
}

/// **The verdicts given to the step joining `member`**: each verdict
/// one of whose faces is `member`'s, between that face and every
/// accumulation face whose lineage holds the verdict's other face. The
/// judgement's operand A may be either member, so either face may be
/// `member`'s. One accumulation face and one member face are one pair,
/// under the first verdict's decision.
///
/// # Errors
///
/// [`NaryRefusal::VerdictsDisagree`] where two verdicts glue one pair
/// under two relations.
fn given_pairs<T: Real>(
    verdicts: &[PairVerdict<T>],
    member: usize,
    lineage: &MemberLineage,
    acc: &Body<T>,
) -> Result<Vec<GluedPair<T>>, NaryRefusal> {
    let mut given: BTreeMap<(FaceKey, FaceKey), (Operand, GlueVerdict<T>)> = BTreeMap::new();
    for (f, _) in acc.faces() {
        let parents = lineage.of(f);
        for v in verdicts {
            let Some((held, at, joining)) = sides(v, member) else {
                continue;
            };
            if !parents.contains(&at) {
                continue;
            }
            let prior = given.entry((f, joining)).or_insert((held, v.verdict));
            if prior.1.relation() != v.verdict.relation() {
                return Err(NaryRefusal::VerdictsDisagree {
                    member,
                    face: f,
                    joining,
                });
            }
        }
    }
    Ok(given
        .into_iter()
        .map(|((f, _), (held, verdict))| verdict.given_at(held, f))
        .collect())
}

/// **The judged rows each of a step's rows restates**, where it
/// restates a given pair's decision ([`GluedPair::restated_by`], a
/// tangency a face–face record cites): the row of every verdict on the
/// pair, read through the same lineage as [`given_pairs`].
///
/// # Panics
///
/// Where a restated row's pair holds no verdict the step was handed. In
/// debug builds, where a face-pair row restates no given pair: a step
/// decides no carrier pair of its own.
fn restated_rows<T: Real>(
    rows: &[crate::Coincidence],
    given: &[GluedPair<T>],
    verdicts: &[PairVerdict<T>],
    member: usize,
    lineage: &MemberLineage,
) -> Vec<(usize, Vec<JudgedRow>)> {
    let mut restated = Vec::new();
    for (k, row) in rows.iter().enumerate() {
        let Some((f, joining)) = row.face_pair() else {
            continue;
        };
        if !given.iter().any(|g| g.restated_by(row)) {
            debug_assert!(
                false,
                "the fold step joining member {member} judged the carrier pair {f:?}/{joining:?} \
                 itself: its verdict is the pairwise judgement's, read through lineage"
            );
            continue;
        }
        let parents = lineage.of(f);
        let mut judged: Vec<JudgedRow> = Vec::new();
        for v in verdicts {
            let Some((_, at, other)) = sides(v, member) else {
                continue;
            };
            if other == joining
                && v.verdict.relation() == row.relation
                && parents.contains(&at)
                && !judged.contains(&v.row)
            {
                judged.push(v.row);
            }
        }
        if judged.is_empty() {
            unreachable!(
                "the fold step joining member {member} restates a given pair on \
                 {f:?}/{joining:?} that no verdict it was handed holds"
            );
        }
        restated.push((k, judged));
    }
    restated
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

    use super::judged_pairs;

    /// **A judged pair's operand A is its lesser rank**, whatever the
    /// list order, and the listed-first on a tie; the pairs are visited
    /// in list order, and a pair whose boxes are apart is not judged.
    #[test]
    fn a_judged_pairs_operand_a_is_its_lesser_rank() {
        let at = |x: f64| {
            bvh::Aabb::from_points([geom_core::Point3::new(x, 0.0, 0.0)]).expect("a point's box")
        };
        let ranks = [2, 0, 1, 0];
        let met = vec![at(0.0); 4];
        assert_eq!(
            judged_pairs(&met, &ranks).collect::<Vec<_>>(),
            [(1, 0), (2, 0), (3, 0), (1, 2), (1, 3), (3, 2)],
            "every pair, in list order, its lesser rank as A, the listed-first on a tie"
        );
        let apart: Vec<_> = (0..4).map(|i| at(100.0 * f64::from(i))).collect();
        assert_eq!(
            judged_pairs(&apart, &ranks).count(),
            0,
            "no pair whose boxes are apart"
        );
    }
}
