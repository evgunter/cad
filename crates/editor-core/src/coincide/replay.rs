//! **The door's symbolic rungs** (D10, FORK-S4F): the document replayed
//! in the `Sym` lane with every continuous variable a symbol of its own
//! ([`crate::analysis::var_env_symbolic`]), profile geometry elaborated
//! in that lane ([`crate::ProfileLane`]), and each row read
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
//! (`scripts/gates/from-f64-sites.tsv`'s `launders(…)` and
//! `stored-carrier-data(…)` classes name the open ones), and the
//! re-valuation row (`editor-core/tests/revalue_corpus.rs`) is what
//! catches one.
//!
//! A row of the replay is the f64 row's when it was decided at the same
//! node, at the same site, as the same ordinal among that site's rows
//! there, between cells of the same names. An at-rest row is matched
//! by no ordinal (the census is not re-run): its cells are read off the
//! replay's copies by name, which stand where the f64 run's do because
//! the deferring session takes every decision as the numeric channel
//! takes it.

use std::collections::BTreeMap;

use geom_core::sym::deferred;
use geom_core::{Band, BandError, Sym, SymBudget, SymRules, Tol};
use topo::{Body, CarrierRelation, FaceKey};

use super::{Canceled, NamedCell, NamedCoincidence, Recourse, Rung, Unreplayed};
use crate::eval::{EvalOptions, EvalOutcome, Evaluation};
use crate::names::{EntityKey, EntityRef, Entry};
use crate::node::RecipeNodeId;
use crate::program::ProfileProgram;

/// **The door's one strength**: the tier's budget, with the drive's
/// retry ladder. A margin whose form outgrows it is not proved, and
/// fails fast. Measured in release: a filleted disc's two coaxial
/// supports prove here and its two joints only from 1024 terms; a
/// filleted box's eight mitres prove at no budget
/// (`work/sym/the-mitres-feet-distance-freezes-at-sym.md`), and cost
/// 0.9 s here against 172 s at 1024 terms.
const BUDGET: SymBudget = SymBudget {
    max_terms: 256,
    max_degree: crate::drive::DEFAULT_SYM_MAX_DEGREE,
};

/// **The replay with every variable a symbol, as one session**: the
/// door's budget ([`BUDGET`]) with the drive's retry ladder, DEFERRING
/// (a margin's form is built only where a reader asks for its
/// discharge), on `base`, the caller's environment — which binds every
/// variable a symbol or varies a box; its profile geometry is
/// elaborated in the lane, as every evaluation's is
/// ([`crate::ProfileLane`]). Every
/// reader of a replay opens it here — the door (`symbols`), its census,
/// and the re-valuation instrument (`editor-core/tests/revalue_corpus.rs`,
/// its own degenerate boxes) — so the instrument reads exactly what
/// the door replays.
///
/// Deferring is what keeps it tractable: a guided profile at `Sym` is a
/// polynomial in the atoms it mints, and an eager session builds every
/// definite margin's form for its contradiction check. Measured on
/// `plate_param`: eager at 4096 terms past 4.9 GB and not done at
/// 120 s, here 136 MB in 1.07 s
/// (`work/inlane/the-sym-profile-lowering-exhausts-memory-under-the-revalue-instrument.md`).
pub fn with_replay_session<R>(
    base: EvalOptions,
    f: impl FnOnce(&EvalOptions) -> R,
) -> (R, geom_core::SymCounts) {
    deferred::with_deferred_session(
        BUDGET,
        SymRules::shipped(),
        crate::drive::DEFAULT_SYM_RETRY,
        || f(&base),
    )
}

/// The scalar the door replays at: `f64` arithmetic, every variable a
/// symbol.
type Lane = Sym<f64>;

/// Why a row is left unproven, and what would prove it.
type Left = (Unreplayed, Recourse);

/// A rung that proved a row, or why the replay could not.
type Proved = Result<Rung, Left>;

/// **The symbolic rungs for each row the first rung left**
/// (`left`, in the shape of `nodes`): in the order given, a rung that
/// proved the row or why the replay could not, and `None` for a row
/// the first rung proved. `nodes` holds each deciding node with its
/// rows in decision order, which is what a row's ordinal is counted in.
///
/// The replay resolves the parts a document instances only for an
/// at-rest row it reads off placed copies ([`reads_copies`]). A row of
/// a node that stands on a part ([`stands_on_a_part`]) is
/// [`Unreplayed::ThroughAPart`] without a replay: resolving the parts at
/// `Sym` costs seconds and has not bought such a row a proof
/// (`work/intent/an-unmated-row-on-a-part-resolves-parts-at-sym-for-seconds.md`).
/// Where no left row asks the replay anything, nothing is replayed.
pub(super) fn rungs(
    doc: &crate::doc::Doc<ProfileProgram>,
    nodes: &[(RecipeNodeId, &[NamedCoincidence])],
    left: &[Vec<bool>],
    resolver: Option<&std::sync::Arc<dyn crate::part::PartResolver>>,
    tol: Tol,
    cancel: &crate::CancelToken,
) -> Result<Vec<Vec<Option<Proved>>>, Canceled> {
    if cancel.is_canceled() {
        return Err(Canceled);
    }
    // What each row asks: `None` a row the first rung proved, `Some(Err)`
    // one answered without a replay, `Some(Ok(()))` one the replay reads.
    let asks: Vec<Vec<Option<Result<(), Left>>>> = nodes
        .iter()
        .zip(left)
        .map(|(&(node, rows), left)| {
            let on_a_part = rows
                .iter()
                .any(|row| row.site != topo::DecisionSite::CensusAtRest)
                && stands_on_a_part(doc, node);
            rows.iter()
                .zip(left)
                .map(|(row, &left)| {
                    left.then(|| {
                        let unread = |why| Err((why, Recourse::Assert));
                        if row.site == topo::DecisionSite::CensusAtRest {
                            // The census is not re-run: its row is read
                            // off the copies.
                            if reads_copies(row) {
                                Ok(())
                            } else {
                                unread(Unreplayed::NotReplayed)
                            }
                        } else if on_a_part {
                            unread(Unreplayed::ThroughAPart)
                        } else {
                            Ok(())
                        }
                    })
                })
                .collect()
        })
        .collect();
    let parts = nodes.iter().zip(&asks).any(|(&(_, rows), asks)| {
        rows.iter()
            .zip(asks)
            .any(|(row, ask)| matches!(ask, Some(Ok(()))) && reads_copies(row))
    });
    let symbols = EvalOptions {
        symbols: true,
        resolver: resolver.filter(|_| parts).cloned(),
        ..EvalOptions::default()
    };
    let (out, _) = with_replay_session(symbols, |opts| {
        // Replayed at the first row that asks, and only then.
        let mut ev: Option<Evaluation<Lane>> = None;
        let band = Band::linear(tol);
        let mut out = Vec::with_capacity(nodes.len());
        for (&(node, rows), asks) in nodes.iter().zip(asks) {
            let mut ordinals = BTreeMap::new();
            let mut answers = Vec::with_capacity(rows.len());
            for (row, ask) in rows.iter().zip(asks) {
                let ordinal = ordinals.entry(row.site).or_insert(0usize);
                let at = core::mem::replace(ordinal, *ordinal + 1);
                answers.push(match ask {
                    None => None,
                    Some(Err(why)) => Some(Err(why)),
                    Some(Ok(())) => {
                        let ev = match &mut ev {
                            Some(ev) => &*ev,
                            None => {
                                let replayed: Evaluation<Lane> =
                                    crate::eval::evaluate(doc, None, cancel, opts, tol);
                                if replayed.outcome == EvalOutcome::Canceled {
                                    return Err(Canceled);
                                }
                                &*ev.insert(replayed)
                            }
                        };
                        Some(prove_row(ev, node, row, at, band, cancel)?)
                    }
                });
            }
            out.push(answers);
        }
        Ok(out)
    });
    out
}

/// **Whether a row is read off placed copies**: an at-rest census row
/// the carrier verdict reads, two faces decided one carrier. The census
/// is not re-run, so any other at-rest row has no form the replay
/// builds ([`Unreplayed::NotReplayed`]).
fn reads_copies(row: &NamedCoincidence) -> bool {
    row.site == topo::DecisionSite::CensusAtRest
        && matches!(
            row.relation,
            topo::Relation::SameOriented | topo::Relation::SameOpposite
        )
        && matches!(row.cells.as_slice(), [a, b] if [a, b].iter().all(|cell| matches!(
            cell,
            NamedCell::Entity { name, .. } if name.kind == crate::names::EntityKind::Face
        )))
}

/// **Whether `node` stands on a part**: it, or a node upstream of it,
/// instances one.
fn stands_on_a_part(doc: &crate::doc::Doc<ProfileProgram>, node: RecipeNodeId) -> bool {
    let mut seen = std::collections::BTreeSet::new();
    let mut stack = vec![node];
    while let Some(at) = stack.pop() {
        if !seen.insert(at) {
            continue;
        }
        if matches!(doc.node(at), Some(crate::Node::InstantiatePart { .. })) {
            return true;
        }
        stack.extend(doc.upstream(at));
    }
    false
}

/// The two symbolic rungs for one row, `at` its ordinal among its
/// node's rows at its site; an at-rest row is read off the copies by
/// the carrier verdict alone (the census is not re-run). Canceled
/// before it discharges a margin.
fn prove_row(
    ev: &Evaluation<Lane>,
    node: RecipeNodeId,
    row: &NamedCoincidence,
    at: usize,
    band: Result<Band, BandError>,
    cancel: &crate::CancelToken,
) -> Result<Result<Rung, Left>, Canceled> {
    if cancel.is_canceled() {
        return Err(Canceled);
    }
    // A row the replay left with no form of its own: nothing the door
    // could read, so the recourse is the assertion.
    let left = |why| Ok(Err((why, Recourse::Assert)));
    if row.site == topo::DecisionSite::CensusAtRest {
        if let Some(why) = at_rest_unbuilt(ev, node, row) {
            return left(why);
        }
        // A run whose tolerance makes no band takes no decision.
        let Ok(band) = band else {
            return left(Unreplayed::Refused);
        };
        return Ok(match carrier_verdict(ev, row, band) {
            Some(Ok(())) => Ok(Rung::CarrierVerdict),
            Some(Err(why)) => Err(numeric(why)),
            None => Err((Unreplayed::OtherDecisions, Recourse::Assert)),
        });
    }
    let Some(value) = ev.value(node) else {
        return left(why_no_value(ev, node));
    };
    let Some(replayed) = value
        .coincidences
        .iter()
        .filter(|r| r.site == row.site)
        .nth(at)
        .filter(|r| r.cells == row.cells && r.relation == row.relation)
    else {
        return left(Unreplayed::OtherDecisions);
    };
    if let Ok(band) = band
        && let Some(Ok(())) = carrier_verdict(ev, row, band)
    {
        return Ok(Ok(Rung::CarrierVerdict));
    }
    if cancel.is_canceled() {
        return Err(Canceled);
    }
    Ok(deferred::discharge_or_why(replayed.margin)
        .map(|()| Rung::MarginIdentity)
        .map_err(numeric))
}

/// A form the tier read and did not discharge: a coincidence of values
/// as far as the door can tell, its recourse one construction where the
/// tier read the form whole, the budget where the form outgrew it, and
/// the assertion where it keeps a value computed apart from the
/// variables.
fn numeric(why: deferred::Undischarged) -> Left {
    (
        Unreplayed::Numeric,
        match why {
            deferred::Undischarged::KeepsAnOpaqueAtom => Recourse::Assert,
            deferred::Undischarged::OverTheBudget => Recourse::Budget,
            deferred::Undischarged::Otherwise => Recourse::OneConstruction,
        },
    )
}

/// **Why the replay built no copy an at-rest row's cell is in**: its
/// placement, or what that reads, refused, or it stands on a part the
/// replay resolves none for. `None` where every cell's copy was built.
fn at_rest_unbuilt(
    ev: &Evaluation<Lane>,
    node: RecipeNodeId,
    row: &NamedCoincidence,
) -> Option<Unreplayed> {
    row.cells.iter().find_map(|cell| match cell {
        NamedCell::Entity { input, .. } => ev
            .table(*input)
            .is_none()
            .then(|| why_no_value(ev, ev.read_port(*input).map_or(node, |(at, _)| at))),
        NamedCell::Tool { .. } | NamedCell::Piece { .. } => None,
    })
}

/// Why the replay holds no value at `node`: a part it does not resolve
/// where that is what failed there or upstream, else a refusal.
fn why_no_value(ev: &Evaluation<Lane>, node: RecipeNodeId) -> Unreplayed {
    match ev.node_error(node).map(|e| &e.kind) {
        Some(crate::eval::NodeErrorKind::Part {
            fault: crate::eval::PartFault::NoResolver,
            ..
        }) => Unreplayed::ThroughAPart,
        _ => Unreplayed::Refused,
    }
}

/// **The kernel's carrier-pair verdict on the row's two cells**, where
/// the row decided two faces one carrier: whether every Zero it stands
/// on is proven, or why the first that is not keeps; `None` where the
/// replay reads no such verdict (not two faces, a face it did not
/// build, or another relation decided). A meeting of cells at a point
/// is read by the margin that decided it, which the replay computes
/// from the cells' symbolic carriers ([`Rung::MarginIdentity`]).
fn carrier_verdict(
    ev: &Evaluation<Lane>,
    row: &NamedCoincidence,
    band: Band,
) -> Option<Result<(), deferred::Undischarged>> {
    let wanted = match row.relation {
        topo::Relation::SameOriented => CarrierRelation::SameOriented,
        topo::Relation::SameOpposite => CarrierRelation::SameOpposite,
        _ => return None,
    };
    let [a, b] = row.cells.as_slice() else {
        return None;
    };
    let ((a, fa), (b, fb)) = (face_of(ev, a)?, face_of(ev, b)?);
    match topo::boolean::carrier_pair_decided(a, fa, b, fb, band) {
        Ok(Ok((relation, Some(margin)))) if relation == wanted => {
            Some(deferred::discharge_or_why(margin))
        }
        _ => None,
    }
}

/// The face `cell` names, as the replay built it: the body of the read
/// it came in through and its key there, read in that read's own table.
fn face_of<'e>(ev: &'e Evaluation<Lane>, cell: &NamedCell) -> Option<(&'e Body<Lane>, FaceKey)> {
    let NamedCell::Entity { input, name } = cell else {
        return None;
    };
    let Some(Entry::Unique(EntityRef {
        body,
        key: EntityKey::Face(face),
    })) = ev.table(*input)?.lookup(name)
    else {
        return None;
    };
    let body = ev.read_body(*input, *body).ok()?.ok()?;
    Some((body, *face))
}
