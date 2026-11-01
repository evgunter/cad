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

/// **Each member of `chain`**, outermost index first: its index values,
/// and `base` at it — each index bound to its value by `bind`, outer
/// before inner, so an inner index's count is evaluated over
/// `counts(base)` with every outer index bound. The one place a
/// family's members are listed: the evaluator's lanes and
/// [`Doc::member_envs`] both read them here.
///
/// # Errors
///
/// [`MemberCountFault::Count`] where an index's count does not
/// evaluate, and [`MemberCountFault::OutOfRange`] where it evaluates
/// below zero or past `u32::MAX`. A count of zero gives no member.
pub(crate) fn members<P: crate::ProfilePayload, E: Clone, C: Decide>(
    doc: &Doc<P>,
    base: E,
    chain: &[VarId],
    counts: &impl Fn(&E) -> &crate::expr::VarEnv<C>,
    bind: &impl Fn(&mut E, &[(VarId, i64)]),
) -> Result<Vec<(Vec<i64>, E)>, MemberCountFault> {
    let Some((&index, inner)) = chain.split_first() else {
        return Ok(vec![(Vec::new(), base)]);
    };
    let Some(def) = doc.var(index).and_then(|v| v.def().index()) else {
        unreachable!("a reach chain holds indices alone")
    };
    let count = crate::expr::eval_count(&def.count, counts(&base))
        .map_err(|source| MemberCountFault::Count { index, source })?;
    if !MemberCountFault::ADMITS.contains(&count) {
        return Err(MemberCountFault::OutOfRange { index, count });
    }
    let mut out = Vec::new();
    for value in 0..count {
        let mut at = base.clone();
        bind(&mut at, &[(index, value)]);
        for (rest, env) in members(doc, at, inner, counts, bind)? {
            out.push((core::iter::once(value).chain(rest).collect(), env));
        }
    }
    Ok(out)
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
#[allow(clippy::too_many_arguments)] // eval_node's inputs, and the chain
pub(super) fn eval_family<T: Decide>(
    doc: &Doc<ProfileProgram>,
    env: &crate::expr::VarEnv<T>,
    id: RecipeNodeId,
    chain: &[VarId],
    results: &BTreeMap<RecipeNodeId, NodeResult<T>>,
    prior: Option<&NodeValue<T>>,
    op_env: &super::wire::OpEnv<'_, T>,
    eval_member: impl Fn(
        &crate::expr::VarEnv<T>,
        &BTreeMap<RecipeNodeId, NodeResult<T>>,
        Option<&NodeValue<T>>,
        &super::wire::OpEnv<'_, T>,
        &[(VarId, i64)],
    ) -> NodeStep<T>,
) -> NodeStep<T> {
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
        &|(params, _)| params,
        &|(params, nominal), at| {
            doc.bind_member(params, at);
            doc.bind_member(nominal, at);
        },
    );
    let values = match listed {
        Ok(values) => values,
        Err(fault) => return fail(fault.node_error()),
    };
    let prior_members = prior.and_then(|p| p.members.as_ref());
    let mut members = Vec::with_capacity(values.len());
    let mut reused = true;
    for (value, (member_env, nominal)) in values {
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
                return fail(NodeErrorKind::MemberRefused {
                    at,
                    source: Box::new(kind),
                });
            }
            poisoned @ NodeResult::Poisoned { .. } => {
                return NodeStep {
                    result: poisoned,
                    reused: false,
                };
            }
        }
    }
    let members = Members {
        over: chain.into(),
        members,
    };
    match family_value(id, members) {
        Ok(value) => NodeStep {
            result: NodeResult::Ok(value),
            reused,
        },
        Err(kind) => fail(kind),
    }
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
