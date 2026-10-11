//! **The door's symbolic rungs** (D10, FORK-S4F): the document replayed
//! in the `Sym` lane with every continuous variable a symbol of its own
//! ([`crate::analysis::var_env_symbolic`]), profile geometry elaborated
//! in that lane ([`crate::ProfileLift::Guided`]), and each row read
//! twice off the replay.
//!
//! - **The carrier-pair verdict** ([`Rung::CarrierVerdict`]): for a row
//!   that decided two faces one carrier, the kernel's own verdict
//!   ([`topo::boolean::carrier_pair_decided`]) run on the two cells'
//!   carriers as the replay built them, read at each cell's name. The
//!   verdict reads each kind modulo its own symmetry, so the door adds
//!   no reduction of its own.
//! - **The deciding margin's identity** ([`Rung::MarginIdentity`]): the
//!   same decision, read in the replay, zero as a polynomial in the
//!   symbols.
//!
//! The replay is a deferring session ([`deferred`]): it takes every
//! decision as the numeric channel takes it, so its branches are the
//! build's, and the identity tier is run only on the margins a rung
//! asks about.
//!
//! Either is a theorem for every value at which the construction takes
//! the decisions it took here (ERROR-DESIGN E12). A replay that refuses,
//! escalates or takes other decisions at the row's node proves nothing.
//!
//! **A theorem about the expression the replay saw.** A computed value
//! the kernel re-enters as a constant (`T::from_f64` of a computed
//! `f64`) is a different function of the variables than the one it came
//! from, so a proof whose expression crosses such a site can be false.
//! The sites are listed with the rows that owe their fix
//! (`scripts/gates/from-f64-sites.tsv`), and the re-valuation row
//! (`editor-core/tests/revalue_corpus.rs`) is what catches one.
//!
//! A row of the replay is the f64 row's when it was decided at the same
//! node, at the same site, as the same ordinal among that site's rows
//! there, between cells of the same names.

use std::collections::BTreeMap;
use std::sync::Arc;

use geom_core::sym::deferred;
use geom_core::{Band, Sym, SymBudget, SymRules, Tol};
use topo::{Body, CarrierRelation, FaceKey};

use super::{NamedCell, NamedCoincidence, Rung, Unreplayed};
use crate::eval::{BooleanValue, EvalOptions, Evaluation, ProfileLift, ValuePayload};
use crate::names::{EntityKey, EntityRef, Entry};
use crate::node::RecipeNodeId;
use crate::program::ProfileProgram;

/// **The tier's budget at the door's full pass**, with the drive's
/// retry ladder: a margin whose form outgrows it is not proved, and
/// fails fast. Measured in release: a filleted disc's two coaxial
/// supports prove here and its two joints only from 1024 terms; a
/// filleted box's eight mitres prove at no budget
/// (`work/sym/the-mitres-feet-distance-freezes-at-sym.md`), and cost
/// 0.9 s here against 172 s at 1024 terms.
const BUDGET: SymBudget = SymBudget {
    max_terms: 256,
    max_degree: crate::drive::DEFAULT_SYM_MAX_DEGREE,
};

/// The scalar the door replays at: `f64` arithmetic, every variable a
/// symbol.
type Lane = Sym<f64>;

/// **The symbolic rungs for each row of each node**: in the order
/// given, a rung that proved the row or why the replay could not.
/// `nodes` holds each deciding node with its rows in decision order,
/// which is what a row's ordinal is counted in.
pub(super) fn rungs(
    doc: &crate::doc::Doc<ProfileProgram>,
    nodes: &[(RecipeNodeId, &[NamedCoincidence])],
    tol: Tol,
) -> Vec<Vec<Result<Rung, Unreplayed>>> {
    let opts = EvalOptions {
        symbols: true,
        profile_lift: ProfileLift::Guided,
        ..EvalOptions::default()
    };
    let (out, _) = deferred::with_deferred_session(
        BUDGET,
        SymRules::shipped(),
        crate::drive::DEFAULT_SYM_RETRY,
        || {
            let ev: Evaluation<Lane> =
                crate::eval::evaluate(doc, None, &crate::CancelToken::new(), &opts, tol);
            let band = Band::linear(tol).ok();
            nodes
                .iter()
                .map(|&(node, rows)| {
                    let mut ordinals = BTreeMap::new();
                    rows.iter()
                        .map(|row| {
                            let ordinal = ordinals.entry(row.site).or_insert(0usize);
                            let at = *ordinal;
                            *ordinal += 1;
                            prove_row(&ev, node, row, at, band)
                        })
                        .collect()
                })
                .collect()
        },
    );
    out
}

/// The two symbolic rungs for one row, `at` its ordinal among its
/// node's rows at its site.
fn prove_row(
    ev: &Evaluation<Lane>,
    node: RecipeNodeId,
    row: &NamedCoincidence,
    at: usize,
    band: Option<Band>,
) -> Result<Rung, Unreplayed> {
    if row.site == topo::DecisionSite::CensusAtRest {
        return Err(Unreplayed::NotReplayed);
    }
    let value = ev.value(node).ok_or(Unreplayed::Refused)?;
    let replayed = value
        .coincidences
        .iter()
        .filter(|r| r.site == row.site)
        .nth(at)
        .filter(|r| r.cells == row.cells && r.relation == row.relation)
        .ok_or(Unreplayed::OtherDecisions)?;
    if let Some(band) = band
        && let Some(how) = carrier_verdict(ev, row, band)
    {
        return Ok(Rung::CarrierVerdict(how));
    }
    deferred::discharge(replayed.margin)
        .map(Rung::MarginIdentity)
        .ok_or(Unreplayed::Numeric)
}

/// **The kernel's carrier-pair verdict on the row's two cells**, where
/// the row decided two faces one carrier: the theorem that decided
/// every Zero it stands on, or `None` where it is not one.
fn carrier_verdict(
    ev: &Evaluation<Lane>,
    row: &NamedCoincidence,
    band: Band,
) -> Option<geom_core::Theorem> {
    let wanted = match row.relation {
        topo::Relation::SameOriented => CarrierRelation::SameOriented,
        topo::Relation::SameOpposite => CarrierRelation::SameOpposite,
        _ => return None,
    };
    let [a, b] = row.cells.each_ref().map(|cell| face_of(ev, cell));
    let ((a, fa), (b, fb)) = (a?, b?);
    match topo::boolean::carrier_pair_decided(&a, fa, &b, fb, band) {
        Ok(Ok((relation, Some(margin)))) if relation == wanted => deferred::discharge(margin),
        _ => None,
    }
}

/// The face `cell` names, as the replay built it: its input's body and
/// its key there.
fn face_of(ev: &Evaluation<Lane>, cell: &NamedCell) -> Option<(Arc<Body<Lane>>, FaceKey)> {
    let NamedCell::Entity { input, name } = cell else {
        return None;
    };
    let value = ev.value(*input)?;
    let Some(Entry::Unique(EntityRef {
        body,
        key: EntityKey::Face(face),
    })) = value.name_table.lookup(name)
    else {
        return None;
    };
    let body = match (&value.payload, body) {
        (ValuePayload::Body(b), 0)
        | (ValuePayload::Boolean(BooleanValue::Body { body: b, .. }), 0) => Arc::clone(b),
        (ValuePayload::Instances(bodies), i) => Arc::clone(bodies.get(usize::try_from(*i).ok()?)?),
        _ => return None,
    };
    Some((body, *face))
}
