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
//! Each step publishes its [`Holdings`] in member faces: exactly the
//! member faces whose area each face of the accumulation holds, and each
//! discarded face lost (`lift`). The lineage only routes the verdicts,
//! where a superset is harmless; what a face holds is read from the
//! records and, where a piece's source face holds several member faces,
//! from the piece's own region ([`super::overlap`](fn@super::overlap)).
//!
//! **What depends on member order** (`[operations.operators]`): the
//! holdings do not, since each reading is the geometric truth of a
//! piece lying inside its source face. Which pieces a fold makes does,
//! and so where an in-band reading of a piece refuses: one member order
//! can refuse a face's holdings where another reads them. So does which
//! member's carrier description a glued face keeps.
//!
//! Where the operation stops, at a refusal or an intersect's empty
//! result, the door returns what it built up to there
//! ([`NaryRun::end`]), so a caller that reads each judgement and step in
//! turn meets the operation's events in the order they happened.
//!
//! A verdict is handed to a step only here, over member bodies the door
//! holds, so no caller states a glue no record over those bodies states.

use std::collections::{BTreeMap, BTreeSet};

use geom_core::{Band, Bounds, Decide, Real, Tol};

use super::holdings::{Held, HeldRefusal, Holdings};
use super::ops::boolean_op_given;
use super::overlap::overlap;
use super::{
    BooleanBody, BooleanError, BooleanInputs, BooleanNaming, BooleanOp, BooleanResult,
    GivenDecision, GlueVerdict, GluedPair, Operand, OperandRecords, SweepStrategy,
};
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
pub struct NaryMember<'m, T: crate::props::AtRestPolicy> {
    /// The member's finished body.
    pub body: &'m T::Finished,
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
///
/// It routes the judgements' verdicts to the steps, where a superset is
/// harmless; it lists member faces a piece holds no area of, and grows
/// along the fold. What a face holds is its [`Holdings`].
#[derive(Clone, Debug, PartialEq, Eq)]
pub(crate) struct MemberLineage {
    faces: BTreeMap<FaceKey, BTreeSet<MemberFace>>,
}

impl MemberLineage {
    /// The lineage of member `member`'s own body: each face its own.
    pub(crate) fn of_member<T: Real>(member: usize, body: &Body<T>) -> Self {
        Self {
            faces: body
                .faces()
                .map(|(f, _)| (f, BTreeSet::from([(member, f)])))
                .collect(),
        }
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
    pub(crate) fn operand_faces(
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
pub struct NaryJudgement<T: crate::props::AtRestPolicy> {
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
pub struct NaryStep<T: crate::props::AtRestPolicy> {
    /// The member the step joins.
    pub member: usize,
    /// The pair boolean's result: the accumulation after the step.
    pub body: BooleanBody<T>,
    /// Each row of the step's coincidences that restates a given pair's
    /// decision (a tangency a face–face record cites), with the rows of
    /// the judgements whose verdicts it stands for, in judgement order:
    /// by the row's place, ascending.
    pub restated: Vec<(usize, Vec<JudgedRow>)>,
    /// Per face of [`Self::body`], and per discard row of its naming,
    /// exactly the member faces whose area it holds, or why that could
    /// not be read (module docs).
    pub holdings: Holdings,
}

impl<T: crate::props::AtRestPolicy> NaryStep<T> {
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
pub struct NaryRun<T: crate::props::AtRestPolicy> {
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

impl<T: crate::props::AtRestPolicy> NaryRun<T> {
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

impl NaryRefusal {
    fn write(
        &self,
        f: &mut core::fmt::Formatter<'_>,
        at: Option<geom_brep::recourse::ReadAt>,
    ) -> core::fmt::Result {
        use geom_brep::recourse::read_at;
        match self {
            Self::Arity { members } => write!(
                f,
                "an n-ary boolean takes two or more members, and was given {members}"
            ),
            Self::Hull { member, error } => {
                write!(f, "member {member}'s box: {}", read_at(error.as_ref(), at))
            }
            Self::Judgement { a, b, error } => write!(
                f,
                "the judgement of members {a} and {b}: {}",
                read_at(error.as_ref(), at)
            ),
            Self::Step { member, error } => write!(
                f,
                "the fold step joining member {member}: {}",
                read_at(error.as_ref(), at)
            ),
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

geom_brep::ended_by_write!(NaryRefusal);

impl core::error::Error for NaryRefusal {}

/// **One carrier pair a step is given**: the two members and a face of
/// each, `a`'s then `b`'s, the decision, and the judgement row that
/// recorded it, none for a face–face record's meaning
/// ([`tool_parts`]).
struct PairVerdict<T: Real> {
    a: usize,
    b: usize,
    faces: (FaceKey, FaceKey),
    decision: GivenDecision<T>,
    row: Option<JudgedRow>,
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
/// (`GluedPair::given_as` returns one) and the pair boolean that takes
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
    run.end = match run_members(op, members, apart, None, sweep, tol, &mut run) {
        Ok(()) => NaryEnd::Built,
        Err(end) => end,
    };
    run
}

/// [`nary_boolean`]'s body, filling `run`'s judgements and steps as it
/// runs, pairwise as `run` says, and ending early with how it stopped;
/// or, where `given` holds the verdicts (a cut's tool read as its
/// material, [`cut_by_material`]), the fold alone, every step given them.
fn run_members<T: Decide + Bounds + AtRestPolicy>(
    op: NaryOp,
    members: &[NaryMember<'_, T>],
    apart: &Apart,
    given: Option<&[PairVerdict<T>]>,
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
    let pairwise = *pairwise && given.is_none();
    if members.len() < 2 {
        return Err(NaryEnd::Refused(NaryRefusal::Arity {
            members: members.len(),
        }));
    }
    let refused = NaryEnd::Refused;
    let empty = |what: &str| match op {
        NaryOp::Intersect => Err(NaryEnd::Empty),
        NaryOp::Union => unreachable!(
            "a union's {what} returned empty, though both operands are finished non-empty \
             bodies and their union holds each"
        ),
    };
    // Each member's box, read wherever the run judges its own pairs, its
    // two members' one step included: given verdicts judge none.
    let hulls = match given {
        Some(_) => Vec::new(),
        None => members
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
            .collect::<Result<Vec<_>, _>>()?,
    };
    let mut judged = Vec::new();
    if pairwise {
        let ranks: Vec<u32> = members.iter().map(|m| m.inputs.rank).collect();
        for (a, b) in judged_pairs(&hulls, &ranks).filter(|&(a, b)| !apart.holds(a, b)) {
            let judgement = boolean_op_given(
                op.op(),
                members[a].body,
                members[b].body,
                &BooleanInputs::default(),
                (None, None),
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
            let BooleanResult::Body(body) = judgement else {
                return empty("pairwise judgement");
            };
            let place = judgements.len();
            judged.extend(body.verdicts.iter().map(|verdict| PairVerdict {
                a,
                b,
                faces: (verdict.a(), verdict.b()),
                decision: verdict.decision(),
                row: Some(JudgedRow {
                    judgement: place,
                    row: verdict_row(&body.coincidences, verdict, (a, b)),
                }),
            }));
            judgements.push(NaryJudgement { a, b, body });
        }
    }
    // Each step is given the verdicts on the member it joins, read
    // through the lineage the fold advances; two members judged by their
    // one step decide its own.
    let verdicts: Option<&[PairVerdict<T>]> = given.or(pairwise.then_some(judged.as_slice()));
    let mut lineage = MemberLineage::of_member(0, members[0].body);
    let mut holdings = Holdings::of_member(0, members[0].body);
    let mut records_a = members[0].inputs.records.clone();
    for member in 1..members.len() {
        let acc = steps.last().map_or(members[0].body, |s| &s.body.body);
        let given = match verdicts {
            Some(verdicts) => Some(given_pairs(verdicts, member, &lineage, acc).map_err(refused)?),
            None => None,
        };
        let inputs = BooleanInputs {
            records_a,
            records_b: members[member].inputs.records.clone(),
        };
        let clones = super::CloneSlot::default();
        let stepped = boolean_op_given(
            op.op(),
            acc,
            members[member].body,
            &inputs,
            (given.as_deref(), Some(clones.clone())),
            sweep,
            tol,
        )
        .map_err(|error| {
            let error = Box::new(error);
            refused(if verdicts.is_some() {
                NaryRefusal::Step { member, error }
            } else {
                NaryRefusal::Judgement { a: 0, b: 1, error }
            })
        })?;
        let BooleanResult::Body(body) = stepped else {
            return empty("fold step");
        };
        let restated = match (&given, verdicts) {
            (Some(given), Some(verdicts)) => {
                restated_rows(&body.coincidences, given, verdicts, member, &lineage)
            }
            _ => Vec::new(),
        };
        lineage = lineage.step(member, &body.naming, &body.body);
        let clone_a = clones
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)[0]
            .take();
        let band = Band::linear(tol).unwrap_or_else(|e| {
            unreachable!(
                "the fold step joining member {member} built at a band it cannot read: {e}"
            )
        });
        holdings = lift(&holdings, member, &body, clone_a.as_ref(), members, band);
        records_a = body.contacts.carried();
        steps.push(NaryStep {
            member,
            body,
            restated,
            holdings: holdings.clone(),
        });
    }
    Ok(())
}

/// **The holdings of the step joining member `member`**, in member faces:
/// `step`'s own, in its operand faces ([`BooleanBody::holdings`]), read
/// through `prev`, the accumulation's. A face of the joining member holds
/// that face; an accumulation face holds what `prev` says it does where
/// that is one member face, and otherwise those of them the region in
/// question meets in area ([`overlap`]): the finished face's own, or a
/// discarded face's in `clone_a`, the step's A clone before its discards.
/// A refusal in `prev`, or in the step's own holdings, carries to every
/// face read through it.
///
/// # Panics
///
/// Where an accumulation face the step's holdings name has no holdings
/// (the pair door checks each is a face of the accumulation where its
/// result leaves the door), a discard of A's names a face `clone_a` does
/// not hold, or a
/// finished face holds no member face: the step's holdings are read from
/// its final records, the pair door keeps its A clone wherever it
/// discards A's faces, and every finished face is a piece of a member's.
fn lift<T: Decide + crate::AtRestPolicy>(
    prev: &Holdings,
    member: usize,
    step: &BooleanBody<T>,
    clone_a: Option<&Body<T>>,
    members: &[NaryMember<'_, T>],
    band: Band,
) -> Holdings {
    // The step's operand faces, as member faces: each exact, or a
    // candidate the region must be read against; or why not.
    let candidates = |operand_faces: &BTreeSet<MemberFace>| {
        let mut exact = BTreeSet::new();
        let mut maybe = BTreeSet::new();
        for &(place, f) in operand_faces {
            if place == 1 {
                exact.insert((member, f));
                continue;
            }
            match prev.of(f) {
                Some(Held::Faces(h)) if h.len() == 1 => exact.extend(h.iter().copied()),
                Some(Held::Faces(h)) => maybe.extend(h.iter().copied()),
                Some(refused @ Held::Refused(_)) => return Err(refused.clone()),
                None => unreachable!(
                    "the fold step joining member {member} names accumulation face {f:?}, which \
                     has no holdings, though its holdings are read from its final rows and the \
                     door reads them for every face of every accumulation it builds"
                ),
            }
        }
        Ok((exact, maybe))
    };
    // What `region` holds of `maybe`, besides `exact`.
    let read = |(mut exact, maybe): (BTreeSet<MemberFace>, BTreeSet<MemberFace>),
                region: (&Body<T>, FaceKey)| {
        for m in maybe {
            if exact.contains(&m) {
                continue;
            }
            match overlap(region, (members[m.0].body, m.1), band) {
                Ok(true) => {
                    exact.insert(m);
                }
                Ok(false) => {}
                Err(refusal) => {
                    return Held::Refused(HeldRefusal::Overlap {
                        member_face: m,
                        refusal,
                    });
                }
            }
        }
        Held::Faces(exact)
    };
    // A refusal of the step's own, against operand face `(place, f)`.
    let restate = |r: &HeldRefusal| match r {
        HeldRefusal::Overlap {
            member_face: (1, f),
            refusal,
        } => HeldRefusal::Overlap {
            member_face: (member, *f),
            refusal: refusal.clone(),
        },
        HeldRefusal::Overlap {
            member_face: (_, f),
            refusal,
        } => HeldRefusal::Accumulation {
            face: *f,
            refusal: refusal.clone(),
        },
        HeldRefusal::Recharted { face: (1, f) } => HeldRefusal::Recharted { face: (member, *f) },
        HeldRefusal::Recharted { face: (_, f) } => HeldRefusal::Accumulation {
            face: *f,
            refusal: super::overlap::OverlapRefusal::Unread(
                super::overlap::OverlapUnread::RechartedRegionLost,
            ),
        },
        HeldRefusal::NotUnder {
            member_face: (1, f),
        } => HeldRefusal::NotUnder {
            member_face: (member, *f),
        },
        HeldRefusal::CarrierUnread {
            member_face: (1, f),
        } => HeldRefusal::CarrierUnread {
            member_face: (member, *f),
        },
        HeldRefusal::NotUnder {
            member_face: (_, f),
        } => HeldRefusal::Accumulation {
            face: *f,
            refusal: super::overlap::OverlapRefusal::Unread(
                super::overlap::OverlapUnread::HeldElsewhere,
            ),
        },
        HeldRefusal::CarrierUnread {
            member_face: (_, f),
        } => HeldRefusal::Accumulation {
            face: *f,
            refusal: super::overlap::OverlapRefusal::Unread(
                super::overlap::OverlapUnread::HeldCarrierUnread,
            ),
        },
        HeldRefusal::Accumulation { .. } | HeldRefusal::UncutRemainder { .. } => r.clone(),
    };
    let result: &Body<T> = &step.body;
    let faces = step
        .holdings
        .faces()
        .map(|(f, held)| {
            let held = match held {
                Held::Refused(r) => Held::Refused(restate(r)),
                Held::Faces(s) => match candidates(s) {
                    Ok(c) => read(c, (result, f)),
                    Err(refused) => refused,
                },
            };
            if held.faces().is_some_and(BTreeSet::is_empty) {
                unreachable!(
                    "finished face {f:?} of the fold step joining member {member} holds no \
                     member face, though every finished face is a piece of a member's"
                );
            }
            (f, held)
        })
        .collect();
    let lost = step
        .naming
        .discards
        .iter()
        .zip(step.holdings.lost_rows())
        .map(|(row, held)| {
            let Held::Faces(s) = held else {
                return held.clone();
            };
            match candidates(s) {
                Err(refused) => refused,
                Ok((exact, maybe)) if maybe.is_empty() => Held::Faces(exact),
                Ok(c) => {
                    let region = clone_a.filter(|a| a.get_face(row.face).is_some());
                    let Some(region) = region else {
                        unreachable!(
                            "the fold step joining member {member} discards A's face {:?}, which \
                             the A clone it kept does not hold, though the pair door keeps its A \
                             clone wherever it discards A's faces",
                            row.face
                        );
                    };
                    read(c, (region, row.face))
                }
            }
        })
        .collect();
    let uncut = step
        .naming
        .uncut
        .iter()
        .zip(step.holdings.uncut_rows())
        .map(|(row, held)| {
            let Held::Faces(s) = held else {
                return held.clone();
            };
            match candidates(s) {
                Err(refused) => refused,
                Ok((exact, maybe)) if maybe.is_empty() => Held::Faces(exact),
                Ok(_) => Held::Refused(HeldRefusal::UncutRemainder { face: row.face }),
            }
        })
        .collect();
    Holdings::from_parts(faces, lost, uncut)
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
fn sides<T: Real>(v: &PairVerdict<T>, member: usize) -> Option<(MemberFace, FaceKey)> {
    if v.b == member {
        Some(((v.a, v.faces.0), v.faces.1))
    } else if v.a == member {
        Some(((v.b, v.faces.1), v.faces.0))
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
    let mut given: BTreeMap<(FaceKey, FaceKey), GivenDecision<T>> = BTreeMap::new();
    for (f, _) in acc.faces() {
        let parents = lineage.of(f);
        for v in verdicts {
            let Some((at, joining)) = sides(v, member) else {
                continue;
            };
            if !parents.contains(&at) {
                continue;
            }
            let prior = given.entry((f, joining)).or_insert(v.decision);
            if prior.class() != v.decision.class() {
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
        .map(|((f, joining), decision)| GluedPair::given_as(f, joining, decision))
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
            let Some((at, other)) = sides(v, member) else {
                continue;
            };
            let Some(judged_row) = v.row else {
                continue;
            };
            if other == joining
                && v.decision.class().relation() == row.relation
                && parents.contains(&at)
                && !judged.contains(&judged_row)
            {
                judged.push(judged_row);
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

/// **A cut's tool read as its material** (`crates/topo/README.md`,
/// `[topo.cut-material]`): the tool's solids, each carved out of it
/// with its keys, so a member face is the tool's face, and the fold that
/// united them, each step given the meaning of the face–face records
/// between the solids it joins. The last step's body is the material.
pub(super) struct ToolMaterial<T: AtRestPolicy> {
    /// The members, the tool's solids in its solid order, each in the
    /// tool's keys.
    pub(super) members: Vec<T::Finished>,
    /// The fold's steps, in order: step `k` joins member `k + 1`.
    pub(super) steps: Vec<NaryStep<T>>,
}

impl<T: AtRestPolicy> ToolMaterial<T> {
    /// The material: the last step's body.
    pub(super) fn body(&self) -> &BooleanBody<T> {
        &self
            .steps
            .last()
            .unwrap_or_else(|| unreachable!("a tool of two or more solids folds at least once"))
            .body
    }
}

/// **The parts of the tool `tool` of a cut, read as its material**,
/// where its solids rest on each other: `None` where no face–face record
/// of `records` joins two of its solids, so the cut reads it as it stands.
///
/// Every solid of the tool is a member, in the tool's solid order, and
/// the fold ([`cut_by_material`]) unites them, each step given, on every
/// pair of a face of the accumulation whose lineage holds one face of
/// such a record and the joining member's other face, the record's
/// meaning: two planar faces a record holds apart lie on one plane with
/// the materials on its two sides ([`crate::contact::area_record`]), one
/// carrier with opposed senses, a [`GivenDecision::Rest`]. No step
/// decides a carrier pair of its own. A record within one solid rides
/// with its member.
///
/// # Errors
///
/// [`BooleanError::ToolSeatUnread`] for a record between two solids that
/// is not one of two planar faces: of curved faces, which may be an area
/// or a line tangency, or of lower cells.
/// [`BooleanError::ResultInvalid`] where a solid carved out of the
/// finished tool does not finish, a kernel defect.
fn tool_parts<T: Decide + AtRestPolicy>(
    tool: &T::Finished,
    records: &OperandRecords,
    tol: Tol,
) -> Result<Option<ToolParts<T>>, BooleanError> {
    use super::Cell::{Edge, Face, Vertex};
    let OperandRecords {
        vv,
        vf,
        ve,
        ee,
        ef,
        ff,
    } = records;
    let solids: Vec<crate::SolidKey> = tool.solids().map(|(s, _)| s).collect();
    let place = |cell: super::Cell| {
        crate::census_cell_solid(tool, cell).and_then(|s| solids.iter().position(|&k| k == s))
    };
    let unread =
        |record: u32, cells: [super::Cell; 2]| BooleanError::ToolSeatUnread { record, cells };
    // The face–face records: a planar one between two solids is a verdict,
    // a curved one between two refuses, and a planar one inside one solid
    // is a solid touching itself over an area, which no finished body is.
    let mut verdicts: Vec<PairVerdict<T>> = Vec::new();
    for carried in ff {
        let (x, y) = (carried.contact.a, carried.contact.b);
        let cells = [Face(x), Face(y)];
        let (Some(i), Some(j)) = (place(cells[0]), place(cells[1])) else {
            continue;
        };
        let area = crate::contact::area_record(tool, x, y);
        match (i == j, area) {
            (true, true) => {
                return Err(BooleanError::TouchesItself {
                    operand: Operand::B,
                    record: carried.record,
                    faces: [x, y],
                });
            }
            (true, false) => {}
            (false, false) => return Err(unread(carried.record, cells)),
            (false, true) => verdicts.push(PairVerdict {
                a: i,
                b: j,
                faces: (x, y),
                decision: GivenDecision::Rest,
                row: None,
            }),
        }
    }
    if verdicts.is_empty() {
        return Ok(None);
    }
    // The solids the fold fuses into one: those the verdicts join, each
    // group folded on its own. A solid no verdict joins stands as it is.
    let n = solids.len();
    let mut group: Vec<usize> = (0..n).collect();
    let root = |group: &[usize], mut k: usize| {
        while group[k] != k {
            k = group[k];
        }
        k
    };
    for v in &verdicts {
        let (ra, rb) = (root(&group, v.a), root(&group, v.b));
        group[ra.max(rb)] = ra.min(rb);
    }
    let roots: Vec<usize> = (0..n).map(|k| root(&group, k)).collect();
    let fused = |k: usize| roots.iter().filter(|&&r| r == roots[k]).count() > 1;
    // Each solid's group, by the group's first solid, and its place there.
    let mut groups: Vec<Vec<usize>> = Vec::new();
    let mut at: Vec<Option<(usize, usize)>> = vec![None; n];
    for k in (0..n).filter(|&k| fused(k)) {
        let g = match groups.iter().position(|g| roots[g[0]] == roots[k]) {
            Some(g) => g,
            None => {
                groups.push(Vec::new());
                groups.len() - 1
            }
        };
        at[k] = Some((g, groups[g].len()));
        groups[g].push(k);
    }
    // Each record within a fused solid rides with its member, one within a
    // standing solid with that solid. A record between two solids that
    // stay apart rides on the material, as a record of the `⊔` it stays.
    // A lower-cell record between two solids one group fuses is a meeting
    // inside the one solid they become, which no verdict states, and
    // refuses.
    let mut own: Vec<Vec<OperandRecords>> = groups
        .iter()
        .map(|g| vec![OperandRecords::default(); g.len()])
        .collect();
    let mut standing_records = OperandRecords::default();
    let mut riding = OperandRecords::default();
    let split = |cells: [super::Cell; 2], record: u32| match (place(cells[0]), place(cells[1])) {
        (Some(i), Some(j)) if i == j => Ok(match at[i] {
            Some((g, m)) => Ride::Member(g, m),
            None => Ride::Standing,
        }),
        (Some(i), Some(j)) if fused(i) && roots[i] == roots[j] => Err(unread(record, cells)),
        _ => Ok(Ride::Material),
    };
    macro_rules! split_list {
        ($list:ident, |$c:ident| $cells:expr) => {
            for carried in $list {
                let $c = &carried.contact;
                match split($cells, carried.record)? {
                    Ride::Member(g, m) => own[g][m].$list.push(*carried),
                    Ride::Standing => standing_records.$list.push(*carried),
                    Ride::Material => riding.$list.push(*carried),
                }
            }
        };
    }
    split_list!(vv, |c| [Vertex(c.a), Vertex(c.b)]);
    split_list!(vf, |c| [Vertex(c.vertex), Face(c.face)]);
    split_list!(ve, |c| [Vertex(c.vertex), Edge(c.edge)]);
    split_list!(ee, |c| [Edge(c.a), Edge(c.b)]);
    split_list!(ef, |c| [Edge(c.edge), Face(c.face)]);
    for carried in ff {
        let cells = [Face(carried.contact.a), Face(carried.contact.b)];
        if let (Some(i), Some(j)) = (place(cells[0]), place(cells[1]))
            && i == j
        {
            match at[i] {
                Some((g, m)) => own[g][m].ff.push(*carried),
                None => standing_records.ff.push(*carried),
            }
        }
    }
    let carved = |keep: crate::SolidKey| {
        let body = crate::carve::solids_alone(tool, &[keep]).map_err(|_| {
            BooleanError::ClassificationInvariant {
                what: "a finished tool's solid does not carve out of it",
            }
        })?;
        T::finish(body, tol).map_err(|findings| BooleanError::ResultInvalid { findings })
    };
    let mut parts = Vec::with_capacity(groups.len());
    for (g, own) in groups.iter().zip(own) {
        let members = g
            .iter()
            .map(|&k| carved(solids[k]))
            .collect::<Result<Vec<_>, _>>()?;
        let local = |k: usize| at[k].map_or(0, |(_, m)| m);
        let verdicts = verdicts
            .iter()
            .filter(|v| roots[v.a] == roots[g[0]])
            .map(|v| PairVerdict {
                a: local(v.a),
                b: local(v.b),
                faces: v.faces,
                decision: v.decision,
                row: None,
            })
            .collect();
        parts.push(GroupParts {
            members,
            inputs: own
                .into_iter()
                .map(|records| MemberInputs { records, rank: 0 })
                .collect(),
            verdicts,
        });
    }
    Ok(Some(ToolParts {
        groups: parts,
        standing: (0..n).filter(|&k| !fused(k)).map(|k| solids[k]).collect(),
        standing_records,
        riding,
    }))
}

/// Where a record of a cut's tool goes ([`tool_parts`]): with the member
/// whose solid holds both its cells, or on the material.
enum Ride {
    /// With member `.1` of fused group `.0`.
    Member(usize, usize),
    /// With a solid that stands as it is.
    Standing,
    /// On the material, between two solids that stay apart.
    Material,
}

/// **What a cut's tool is united from** ([`tool_parts`]): each group of
/// its solids the verdicts join, the solids that stand as they are, and
/// the records between solids that stay apart.
struct ToolParts<T: AtRestPolicy> {
    groups: Vec<GroupParts<T>>,
    /// The solids no verdict joins, in the tool's solid order.
    standing: Vec<crate::SolidKey>,
    /// The records within those solids, in the tool's keys.
    standing_records: OperandRecords,
    /// The records between two solids that stay apart, in the tool's
    /// keys: records of the material, which the cut carries.
    riding: OperandRecords,
}

/// **One group of a cut's tool's solids the verdicts join**: its solids,
/// each in the tool's keys, their own records, and the verdicts between
/// them, by the members' places in the group.
struct GroupParts<T: AtRestPolicy> {
    members: Vec<T::Finished>,
    inputs: Vec<MemberInputs>,
    verdicts: Vec<PairVerdict<T>>,
}

impl<T: AtRestPolicy> GroupParts<T> {
    /// The members as the fold reads them.
    fn views(&self) -> Vec<NaryMember<'_, T>> {
        self.members
            .iter()
            .zip(&self.inputs)
            .map(|(body, inputs)| NaryMember { body, inputs })
            .collect()
    }
}

/// **`a − b`, `b` read as its material**, where `b`'s solids rest on each
/// other (`crates/topo/README.md`, `[topo.cut-material]`): the tool's
/// parts ([`tool_parts`]) united by the fold, every step given the
/// verdicts its records mean, then the ordinary cut by that material,
/// stated in the tool's own cells ([`super::material::Through`]). `None`
/// where `b`'s solids do not rest on each other, and the cut reads `b`
/// as it stands.
///
/// # Errors
///
/// [`tool_parts`]'s; a fold step's refusal, the union's own for a seat
/// it has no glue arm for; the cut's; the read-back's.
pub(super) fn cut_by_material<T: Decide + Bounds + AtRestPolicy>(
    a: &T::Finished,
    b: &T::Finished,
    inputs: &BooleanInputs,
    sweep: SweepStrategy,
    tol: Tol,
) -> Result<Option<BooleanResult<T>>, BooleanError> {
    super::validate_carried(b, &inputs.records_b, Operand::B)?;
    let Some(parts) = tool_parts::<T>(b, &inputs.records_b, tol)? else {
        return Ok(None);
    };
    let ToolParts {
        groups,
        standing,
        standing_records,
        riding,
    } = parts;
    let mut folded = Vec::with_capacity(groups.len());
    for group in groups {
        let mut run = NaryRun {
            judgements: Vec::new(),
            steps: Vec::new(),
            pairwise: false,
            end: NaryEnd::Built,
        };
        let ended = run_members(
            NaryOp::Union,
            &group.views(),
            &Apart::default(),
            Some(&group.verdicts),
            sweep,
            tol,
            &mut run,
        );
        let material = match ended {
            Ok(()) => ToolMaterial {
                members: group.members,
                steps: run.steps,
            },
            Err(NaryEnd::Refused(NaryRefusal::Step { error, .. })) => {
                return Err(BooleanError::ToolFuseRefused {
                    why: Box::new(super::ToolFuseRefusal::Step(*error)),
                });
            }
            Err(_) => {
                return Err(BooleanError::ClassificationInvariant {
                    what: "a cut's tool's fold of its solids ends other than built or refused at \
                           a step",
                });
            }
        };
        let through = super::material::Through::of(&material)?;
        folded.push((material, through));
    }
    let (fused, records_b, through) =
        super::material::assembled(b, &standing, &standing_records, folded, &riding, tol)?;
    let cut = boolean_op_given(
        BooleanOp::Subtract,
        a,
        &fused,
        &BooleanInputs {
            records_a: inputs.records_a.clone(),
            records_b,
        },
        (None, None),
        sweep,
        tol,
    )?;
    let BooleanResult::Body(mut out) = cut else {
        return Ok(Some(cut));
    };
    through.restate(&mut out, b)?;
    Ok(Some(BooleanResult::Body(out)))
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

    use std::collections::BTreeMap;

    use geom_core::Tol;

    use super::{
        Apart, NaryEnd, NaryOp, NaryRun, SweepStrategy, judged_pairs, run_members, tool_parts,
    };
    use crate::test_support::meeting::{posed_boxes, poses};
    use crate::{ContactRecords, SolidKey};

    /// **A cut's tool is fused on its seat's record, decided nowhere**:
    /// the bolt on the plate, one body of two solids, its seat recorded
    /// as the census at a `⊔`'s door records it, read as material is one
    /// solid of 11 faces (the plate's top keeps the foot's outline), no
    /// pair of its solids is judged, and no step of the fold records a
    /// carrier pair's row, at rest and turned: the glue is the record's
    /// meaning. Red where the fold decides its own glue (it records the
    /// plane ladder's row of the foot and the top), where it judges the
    /// pair before the fold, where it is handed no pair for the record (a
    /// foot corner on the top escalates) or the pair aligned (a face of
    /// the result runs the wrong way round).
    #[test]
    fn a_tools_seat_is_taken_and_decided_nowhere() {
        let tol = Tol::witness();
        for pose in poses()
            .into_iter()
            .filter(|p| ["at rest", "turned"].contains(&p.label))
        {
            let body = posed_boxes(
                "a bolt on a plate",
                &[
                    [(-0.2, 0.2), (-0.2, 0.2), (0.2, 1.2)],
                    [(-1.0, 1.0), (-1.0, 1.0), (0.0, 0.2)],
                ],
                &pose,
                tol,
            );
            let groups: BTreeMap<SolidKey, u32> = body.solids().map(|(s, _)| s).zip(0..).collect();
            let recorded = crate::record_at_rest(
                &body,
                &ContactRecords::default(),
                |records| {
                    crate::validate_pseudomanifold(&body, records, tol)
                        .err()
                        .unwrap_or_default()
                },
                &|s| groups.get(&s).copied(),
                0,
            );
            assert_eq!(
                recorded.records.face_face().count(),
                1,
                "{}: the premise, the seat is one face–face record",
                pose.label
            );
            let parts = tool_parts::<f64>(&body, &recorded.records.carried(), tol)
                .unwrap_or_else(|e| panic!("{}: the tool's parts: {e:?}", pose.label))
                .unwrap_or_else(|| panic!("{}: its solids rest on each other", pose.label));
            let mut run = NaryRun {
                judgements: Vec::new(),
                steps: Vec::new(),
                pairwise: false,
                end: NaryEnd::Built,
            };
            assert!(
                parts.standing.is_empty() && parts.groups.len() == 1,
                "{}: the bolt and the plate are one group",
                pose.label
            );
            let group = &parts.groups[0];
            let ended = run_members(
                NaryOp::Union,
                &group.views(),
                &Apart::default(),
                Some(&group.verdicts),
                SweepStrategy::Realized,
                tol,
                &mut run,
            );
            assert!(
                ended.is_ok(),
                "{}: the tool fuses: {:?}",
                pose.label,
                run.end
            );
            assert!(
                run.judgements.is_empty() && !run.pairwise,
                "{}: no pair of the tool's solids is judged: {} judgements",
                pose.label,
                run.judgements.len()
            );
            let fused = &run.steps.last().unwrap().body.body;
            assert_eq!(fused.solids().count(), 1, "{}: one solid", pose.label);
            assert_eq!(fused.faces().count(), 11, "{}: its faces", pose.label);
            for step in &run.steps {
                assert!(
                    step.body
                        .coincidences
                        .iter()
                        .all(|row| row.face_pair().is_none()),
                    "{}: the fold decides no carrier pair: {:?}",
                    pose.label,
                    step.body.coincidences
                );
            }
        }
    }

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
