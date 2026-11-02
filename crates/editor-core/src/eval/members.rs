//! **Per-member evaluation** (D10 `[repetition]`): an operation whose
//! reads reach an index is evaluated once per value of the indices it
//! reaches, and its value is the family of those members.
//!
//! Each member is the operation evaluated as any other node is
//! ([`super::eval_node`]), over the environment with its indices bound
//! to the member's values and every definition reaching them bound again
//! ([`crate::Doc::bind_member`]), and over the results with each
//! operation it reads as a member replaced by that member
//! ([`overlay`]). A seat that reads a family whole — a member seat of
//! a `union`, `intersect` or `assemble`, or an indexed read
//! ([`crate::repetition::takes_whole`]) — and a selection naming its
//! members read the family the member stands for
//! ([`super::NodeValue::whole`]), so one operation may read a family
//! plainly at one seat and whole at another. A member that refuses
//! refuses the family, naming the member.
//!
//! A family of bodies holds its members' bodies as an `Instances`
//! payload, one per member in index order, named `Member { at, of }`
//! over each member's own names ([`crate::names::name_family`]); any
//! other family's payload is [`ValuePayload::Family`]. Either keeps each
//! member's whole value ([`Members`]), which is what a reader of one
//! member reads, and what the memo serves a member from.

use std::collections::BTreeMap;
use std::sync::Arc;

use geom_core::Decide;

use super::memo::KeyHasher;
use super::{
    ContentKey, NamingKey, NodeError, NodeErrorKind, NodeResult, NodeStep, NodeValue, ValuePayload,
    tag,
};
use crate::doc::Doc;
use crate::node::RecipeNodeId;
use crate::program::ProfileProgram;
use crate::repetition::MemberCountFault;
use crate::var::VarId;

/// **A family's members**, in index order, outermost index first.
#[derive(Debug, Clone)]
pub struct Members<T: Decide> {
    /// The indices the family reaches, outermost first.
    pub over: Arc<[VarId]>,
    /// Each member: its index values, one per index of `over`, and its
    /// value, in ascending order of the values, so a member is found by
    /// them in logarithmic time.
    pub members: Vec<(Vec<u32>, NodeValue<T>)>,
}

impl<T: Decide> Members<T> {
    /// The member at `at`, the values of `over` in order.
    #[must_use]
    pub fn at(&self, at: &[u32]) -> Option<&NodeValue<T>> {
        self.position(at).map(|i| &self.members[i].1)
    }

    /// The member standing at the values `bound` gives the indices of
    /// `over`, where it gives every one.
    fn bound_at(&self, bound: &BTreeMap<VarId, u32>) -> Option<&NodeValue<T>> {
        let at: Option<Vec<u32>> = self
            .over
            .iter()
            .map(|index| bound.get(index).copied())
            .collect();
        self.at(&at?)
    }

    /// Each member's position in the family's flat body list, with its
    /// index values: the order an `Instances` payload holds them in.
    pub(crate) fn position(&self, at: &[u32]) -> Option<usize> {
        self.members
            .binary_search_by(|(values, _)| values.as_slice().cmp(at))
            .ok()
    }

    /// Each member's index values, paired with the indices they are of:
    /// what its names' `Member` segment holds.
    pub(crate) fn member_at(&self, values: &[u32]) -> crate::names::MemberAt {
        self.over
            .iter()
            .copied()
            .zip(values.iter().copied())
            .collect()
    }
}

/// **Each member of `chain`**, outermost index first and one at a time:
/// its index values, and `base` at it — each index bound to its value
/// by `bind`, outer before inner, so an inner index's count is evaluated
/// over `counts` of the environment with every outer index bound. The
/// one place a family's members are listed: the evaluator's lanes and
/// [`Doc::member_envs`] both read them here.
///
/// Lazy: it holds one environment per index of `chain`, never one per
/// member, so a reader that stops at a member binds none past it.
///
/// # Errors
///
/// An item is [`MemberCountFault::Count`] where an index's count does
/// not evaluate, and [`MemberCountFault::OutOfRange`] where it
/// evaluates outside [`MemberCountFault::ADMITS`]; nothing follows it.
/// A count of zero gives no member.
pub(crate) fn members<'a, P: crate::ProfilePayload, E: Clone + 'a, C: Decide + 'a>(
    doc: &'a Doc<P>,
    base: E,
    chain: &'a [VarId],
    counts: impl Fn(&E) -> &crate::expr::VarEnv<C> + 'a,
    bind: impl Fn(&mut E, &[(VarId, i64)]) + 'a,
) -> impl Iterator<Item = Result<(Vec<i64>, E), MemberCountFault>> + 'a {
    listing(doc, base, chain, counts, bind).filter_map(|step| match step {
        Ok(Listed::Member(values, at)) => Some(Ok((values, at))),
        Ok(Listed::Within) => None,
        Err(fault) => Some(Err(fault)),
    })
}

/// One step of a listing: a member, or an outer index's value bound
/// with the indices within it still to list.
enum Listed<E> {
    Member(Vec<i64>, E),
    Within,
}

/// [`members`], one item per index value bound: a member where the
/// value is the innermost index's, [`Listed::Within`] where it is an
/// outer one's. So a reader counting listing steps counts each as it
/// happens, however many outer values bind no member.
fn listing<'a, P: crate::ProfilePayload, E: Clone + 'a, C: Decide + 'a>(
    doc: &'a Doc<P>,
    base: E,
    chain: &'a [VarId],
    counts: impl Fn(&E) -> &crate::expr::VarEnv<C> + 'a,
    bind: impl Fn(&mut E, &[(VarId, i64)]) + 'a,
) -> impl Iterator<Item = Result<Listed<E>, MemberCountFault>> + 'a {
    // `levels[d]` is the environment with the indices above `chain[d]`
    // bound, the next value of `chain[d]` and its count.
    let count_of = move |index: VarId, env: &E| -> Result<i64, MemberCountFault> {
        let Some(def) = doc.var(index).and_then(|v| v.def().index()) else {
            unreachable!("a reach chain holds indices alone")
        };
        let count = crate::expr::eval_count(&def.count, counts(env))
            .map_err(|source| MemberCountFault::Count { index, source })?;
        if MemberCountFault::ADMITS.contains(&count) {
            Ok(count)
        } else {
            Err(MemberCountFault::OutOfRange { index, count })
        }
    };
    let mut levels: Vec<(E, i64, i64)> = Vec::with_capacity(chain.len());
    let mut first = Some(base);
    let mut done = false;
    core::iter::from_fn(move || {
        if done {
            return None;
        }
        if let Some(base) = first.take() {
            let Some(&outer) = chain.first() else {
                done = true;
                return Some(Ok(Listed::Member(Vec::new(), base)));
            };
            match count_of(outer, &base) {
                Ok(count) => levels.push((base, 0, count)),
                Err(fault) => {
                    done = true;
                    return Some(Err(fault));
                }
            }
        }
        loop {
            let depth = levels.len();
            let Some((env, next, count)) = levels.last_mut() else {
                done = true;
                return None;
            };
            if *next == *count {
                levels.pop();
                continue;
            }
            let value = *next;
            *next += 1;
            let mut at = env.clone();
            bind(&mut at, &[(chain[depth - 1], value)]);
            if depth == chain.len() {
                let values = levels.iter().map(|(_, next, _)| next - 1).collect();
                return Some(Ok(Listed::Member(values, at)));
            }
            match count_of(chain[depth], &at) {
                Ok(count) => {
                    levels.push((at, 0, count));
                    return Some(Ok(Listed::Within));
                }
                Err(fault) => {
                    done = true;
                    return Some(Err(fault));
                }
            }
        }
    })
}

/// **The listing steps `chain`'s members take**: every index value
/// bound while they are listed by [`members`]' own walk, over `env` and
/// `nominal` as [`eval_family`] binds them, no member evaluated. `None` as soon as the steps pass `remaining`, so a
/// charge never lists further than the budget it is charged against. A
/// count that faults ends the listing where the family's evaluation
/// refuses it.
///
/// # Errors
///
/// [`super::Canceled`] where `cancel` is set before a step.
pub(super) fn listing_steps<T: Decide>(
    doc: &Doc<ProfileProgram>,
    env: &crate::expr::VarEnv<T>,
    nominal: &crate::expr::VarEnv<f64>,
    chain: &[VarId],
    remaining: u64,
    cancel: &super::CancelToken,
) -> Result<Option<u64>, super::Canceled> {
    let mut steps = 0u64;
    for step in listing(
        doc,
        (env.clone(), nominal.clone()),
        chain,
        |(params, _): &(crate::expr::VarEnv<T>, crate::expr::VarEnv<f64>)| params,
        // Only an outer index's binding is read, by the counts within
        // it; a member's environment is never built.
        |(params, nominal): &mut (crate::expr::VarEnv<T>, crate::expr::VarEnv<f64>), at| {
            if at.iter().all(|(index, _)| Some(index) != chain.last()) {
                doc.bind_member(params, at);
                doc.bind_member(nominal, at);
            }
        },
    ) {
        if cancel.is_canceled() {
            return Err(super::Canceled);
        }
        if step.is_err() {
            break;
        }
        steps += 1;
        if steps > remaining {
            return Ok(None);
        }
    }
    Ok(Some(steps))
}

/// **`results` at one member**: each family a member of whose stands
/// at `bound` replaced by that member, which keeps the family for the
/// seats that read it whole ([`super::NodeValue::whole`]).
fn overlay<T: Decide>(
    results: &BTreeMap<RecipeNodeId, NodeResult<T>>,
    bound: &BTreeMap<VarId, u32>,
) -> BTreeMap<RecipeNodeId, NodeResult<T>> {
    results
        .iter()
        .map(|(&id, result)| {
            let member = match result {
                NodeResult::Ok(value) => value
                    .members
                    .as_ref()
                    .and_then(|members| members.bound_at(bound))
                    .map(|member| NodeValue {
                        family: Some(Arc::new(value.clone())),
                        ..member.clone()
                    }),
                _ => None,
            };
            // A failed operation stands as the nearest failed ancestor
            // of whatever reads it, which is all a reader asks of it.
            let result = match (member, result) {
                (Some(member), _) => NodeResult::Ok(member),
                (None, NodeResult::Ok(value)) => NodeResult::Ok(value.clone()),
                (None, NodeResult::Failed(_)) => NodeResult::Poisoned { through: id },
                (None, NodeResult::Poisoned { through }) => {
                    NodeResult::Poisoned { through: *through }
                }
            };
            (id, result)
        })
        .collect()
}

/// **The node `id`, evaluated once per member** of the indices `chain`
/// it reaches, its value the family of them; `eval_member` evaluates
/// it at one member's environment, results, prior value, lane and
/// index values.
///
/// # Errors
///
/// [`super::Canceled`] where `cancel` is set before a member: a family
/// is as large as its counts admit, so it yields to cancelation member
/// by member as the walk does node by node.
#[allow(clippy::too_many_arguments)] // eval_node's inputs, and the chain
pub(super) fn eval_family<T: Decide>(
    doc: &Doc<ProfileProgram>,
    env: &crate::expr::VarEnv<T>,
    id: RecipeNodeId,
    chain: &[VarId],
    results: &BTreeMap<RecipeNodeId, NodeResult<T>>,
    prior: Option<&NodeValue<T>>,
    cancel: &super::CancelToken,
    op_env: &super::wire::OpEnv<'_, T>,
    eval_member: impl Fn(
        &crate::expr::VarEnv<T>,
        &BTreeMap<RecipeNodeId, NodeResult<T>>,
        Option<&NodeValue<T>>,
        &super::wire::OpEnv<'_, T>,
        &[(VarId, i64)],
    ) -> NodeStep<T>,
) -> Result<NodeStep<T>, super::Canceled> {
    let fail = |kind| NodeStep {
        result: NodeResult::Failed(NodeError {
            node: id,
            kind,
            escalations: Arc::new(Vec::new()),
        }),
        reused: false,
    };
    let listed = members(
        doc,
        (env.clone(), op_env.lane.nominal.clone()),
        chain,
        |(params, _): &(crate::expr::VarEnv<T>, crate::expr::VarEnv<f64>)| params,
        |(params, nominal): &mut (crate::expr::VarEnv<T>, crate::expr::VarEnv<f64>), at| {
            doc.bind_member(params, at);
            doc.bind_member(nominal, at);
        },
    );
    let prior_members = prior.and_then(|p| p.members.as_ref());
    let mut members = Vec::new();
    let mut reused = true;
    for member in listed {
        if cancel.is_canceled() {
            return Err(super::Canceled);
        }
        let (value, (member_env, nominal)) = match member {
            Ok(member) => member,
            Err(fault) => return Ok(fail(fault.node_error())),
        };
        let at: Vec<(VarId, i64)> = chain.iter().copied().zip(value.iter().copied()).collect();
        let narrow: Vec<u32> = value
            .iter()
            .map(|&v| {
                u32::try_from(v).unwrap_or_else(|_| {
                    unreachable!("an admitted count is within u32 (`MemberCountFault::OutOfRange`)")
                })
            })
            .collect();
        let bound: BTreeMap<VarId, u32> =
            chain.iter().copied().zip(narrow.iter().copied()).collect();
        let member_op_env = super::wire::OpEnv {
            twist: op_env.twist,
            boolean_sweep: op_env.boolean_sweep,
            parts: op_env.parts,
            lane: super::wire::LaneEnv {
                params: &member_env,
                nominal: &nominal,
                // The nominal document's results hold the family, not
                // this member: a member reading a trace off them resolves
                // at the nominal lane alone.
                nominal_results: None,
                pass_two: op_env.lane.pass_two,
            },
        };
        let over = overlay(results, &bound);
        let prior_member = prior_members.and_then(|m| m.at(&narrow));
        let step = eval_member(&member_env, &over, prior_member, &member_op_env, &at);
        reused &= step.reused;
        match step.result {
            NodeResult::Ok(value) => members.push((narrow, value)),
            NodeResult::Failed(NodeError { kind, .. }) => {
                return Ok(fail(NodeErrorKind::MemberRefused {
                    at,
                    source: Box::new(kind),
                }));
            }
            poisoned @ NodeResult::Poisoned { .. } => {
                return Ok(NodeStep {
                    result: poisoned,
                    reused: false,
                });
            }
        }
    }
    let members = Members {
        over: chain.into(),
        members,
    };
    Ok(match family_value(id, members) {
        Ok(value) => NodeStep {
            result: NodeResult::Ok(value),
            reused,
        },
        Err(kind) => fail(kind),
    })
}

/// **A member's content key**: its operation's at the member's values,
/// then the values (D9: a node reading an index keys by its content key
/// and the index values).
pub(super) fn member_key(key: ContentKey, at: &[(VarId, i64)]) -> ContentKey {
    if at.is_empty() {
        return key;
    }
    let mut h = KeyHasher::new();
    h.write_u64(key.0 as u64);
    h.write_u64((key.0 >> 64) as u64);
    h.write_tag(tag::member::AT);
    h.write_u64(at.len() as u64);
    for &(index, value) in at {
        h.write_u64(u64::from(index.0.ordinal()));
        h.write_u64(index.0.digest());
        h.write_u64(value as u64);
    }
    h.finish()
}

/// **The family's value**: a family of bodies as their `Instances`,
/// named member by member; any other as [`ValuePayload::Family`]. Its
/// keys are its members', in order.
fn family_value<T: Decide>(
    id: RecipeNodeId,
    members: Members<T>,
) -> Result<NodeValue<T>, NodeErrorKind> {
    let bodies: Option<Vec<super::Framed<T>>> = members
        .members
        .iter()
        .map(|(_, value)| match &value.payload {
            ValuePayload::Body(body)
            | ValuePayload::Boolean(super::BooleanValue::Body { body, .. }) => Some(body.clone()),
            _ => None,
        })
        .collect();
    let mut content = KeyHasher::new();
    let mut naming = KeyHasher::new();
    content.write_tag(tag::member::FAMILY);
    naming.write_tag(tag::member::FAMILY);
    for (_, value) in &members.members {
        for (h, key) in [
            (&mut content, value.content_key.0),
            (&mut naming, value.naming_key.0),
        ] {
            h.write_u64(key as u64);
            h.write_u64((key >> 64) as u64);
        }
    }
    let (payload, outputs) = match bodies {
        Some(bodies) => {
            let tables: Vec<(crate::names::MemberAt, &crate::names::NameTable)> = members
                .members
                .iter()
                .map(|(values, value)| {
                    let table = value
                        .output(0)
                        .map(super::OutputNames::table)
                        .unwrap_or_else(|| unreachable!("a body's value names its output"));
                    (members.member_at(values), table)
                })
                .collect();
            let raw: Vec<&topo::Body<T>> = bodies.iter().map(super::Framed::body).collect();
            let table =
                crate::names::name_family(id, &tables, &raw).map_err(NodeErrorKind::Naming)?;
            (
                ValuePayload::Instances(bodies),
                Arc::from([super::OutputNames::new(table, Arc::default())]),
            )
        }
        None => (ValuePayload::Family, Arc::from([])),
    };
    // A family carries its members' carries together, each member's
    // as its own: every member is evaluated on its own, so its records
    // are keyed in its own arena, a boolean member's riding its payload.
    let carry = super::Carry::of_family(
        members
            .members
            .iter()
            .map(|(_, value)| value.carry.clone())
            .collect(),
    );
    let coincidences: Vec<crate::coincide::NamedCoincidence> = members
        .members
        .iter()
        .flat_map(|(_, value)| value.coincidences.iter().cloned())
        .collect();
    let verdicts: super::VerdictLog = members
        .members
        .iter()
        .flat_map(|(_, value)| value.verdicts.iter().cloned())
        .collect();
    let escalations: super::EscalationLog = members
        .members
        .iter()
        .flat_map(|(_, value)| value.escalations.iter().cloned())
        .collect();
    Ok(NodeValue {
        payload,
        outputs,
        carry,
        coincidences: coincidences.into(),
        motion: None,
        members: Some(Arc::new(members)),
        family: None,
        verdicts: Arc::new(verdicts),
        escalations: Arc::new(escalations),
        witness: super::WitnessSlot {},
        content_key: content.finish(),
        naming_key: NamingKey(naming.finish().0),
    })
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

    use std::cell::Cell;

    use geom_core::Tol;

    use super::*;
    use crate::eval::{CancelToken, NodeStep, parts, wire};

    /// A document declaring the index `k = index(count)`, and `k`.
    fn indexed(count: i64) -> (Doc<ProfileProgram>, VarId) {
        let tol = Tol::witness();
        let doc = Doc::<ProfileProgram>::empty_derived("members-cancel", tol);
        let name = crate::VarName::new("k").unwrap();
        let doc = doc
            .apply(
                &crate::DocEdit::DeclareVar {
                    name: name.clone(),
                    def: crate::VarDecl::Index {
                        count: crate::Formula::count(count),
                        within: None,
                    },
                },
                tol,
            )
            .unwrap()
            .doc;
        let k = doc.var_named(name.as_str()).unwrap();
        (doc, k)
    }

    /// `doc`'s family over `k`, each member a hand-built value, with
    /// `on_member` run at each member's values before it evaluates; and
    /// how many members evaluated.
    fn family(
        doc: &Doc<ProfileProgram>,
        k: VarId,
        cancel: &CancelToken,
        on_member: impl Fn(&[(VarId, i64)]),
    ) -> (Result<NodeStep<f64>, crate::eval::Canceled>, usize) {
        let tol = Tol::witness();
        let env = doc.var_env::<f64>();
        let nominal = doc.var_env::<f64>();
        let entry = parts::PartStore::<f64>::new(None, topo::SweepStrategy::Realized, tol)
            .enter_census(doc, None, cancel, tol);
        let op_env = wire::OpEnv {
            twist: None,
            boolean_sweep: topo::SweepStrategy::Realized,
            parts: &entry.reached,
            lane: wire::LaneEnv {
                params: &env,
                nominal: &nominal,
                nominal_results: None,
                pass_two: crate::eval::PassTwo::SkippedAtF64,
            },
        };
        let evaluated = Cell::new(0usize);
        let id = RecipeNodeId::new(0, test_utils::refusal::tagged(1));
        let step = eval_family(
            doc,
            &env,
            id,
            &[k],
            &BTreeMap::new(),
            None,
            cancel,
            &op_env,
            |_, _, _, _, at| {
                on_member(at);
                evaluated.set(evaluated.get() + 1);
                NodeStep {
                    result: NodeResult::Ok(NodeValue::hand_built(
                        crate::names::NameTable::new(),
                        crate::names::FragmentGroups::default(),
                        Vec::new(),
                    )),
                    reused: false,
                }
            },
        );
        (step, evaluated.get())
    }

    /// **A family yields to cancelation member by member**: the token
    /// set while member 2 of ten evaluates stops the family before
    /// member 3, typed. Red if the family checks the token only between
    /// nodes — it then evaluates all ten — or reports a value.
    #[test]
    fn a_family_canceled_by_a_member_stops_before_the_next() {
        let (doc, k) = indexed(10);
        let cancel = CancelToken::new();
        let (step, evaluated) = family(&doc, k, &cancel, |at| {
            if at == [(k, 2)] {
                cancel.cancel();
            }
        });
        assert!(step.is_err(), "a canceled family holds no result");
        assert_eq!(evaluated, 3, "members 0, 1 and 2 evaluated, and no other");
    }

    /// An uncanceled family evaluates every member, in index order.
    #[test]
    fn an_uncanceled_family_evaluates_every_member() {
        let (doc, k) = indexed(5);
        let cancel = CancelToken::new();
        let seen = Cell::new(0i64);
        let (step, evaluated) = family(&doc, k, &cancel, |at| {
            assert_eq!(at, [(k, seen.get())], "members in index order");
            seen.set(seen.get() + 1);
        });
        let Ok(NodeStep {
            result: NodeResult::Ok(value),
            ..
        }) = step
        else {
            panic!("a family of five evaluates");
        };
        assert_eq!(evaluated, 5, "one evaluation per member");
        let members = value
            .members
            .as_ref()
            .expect("a family's value holds its members");
        assert_eq!(members.members.len(), 5, "five members held");
    }
}
