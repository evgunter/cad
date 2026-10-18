//! **The door's symbolic rungs** (D10, FORK-S4F): the document replayed
//! in the `Sym` lane with every continuous variable a symbol of its own
//! ([`crate::analysis::var_env_symbolic`]), profile geometry elaborated
//! in that lane (symbols make the lift [`crate::ProfileLift::Guided`]),
//! and each row read
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
//! there, between cells of the same names.

use std::collections::BTreeMap;

use geom_core::sym::deferred;
use geom_core::{Band, Sym, SymBudget, SymRules, Tol};
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
/// variable a symbol or varies a box, so its profile geometry is
/// elaborated in the lane ([`crate::eval::ProfileLift::Guided`], read off
/// what the evaluation binds by [`EvalOptions::profile_lift_over`]). Every
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

/// **The symbolic rungs for each row of each node**: in the order
/// given, a rung that proved the row or why the replay could not.
/// `nodes` holds each deciding node with its rows in decision order,
/// which is what a row's ordinal is counted in.
pub(super) fn rungs(
    doc: &crate::doc::Doc<ProfileProgram>,
    nodes: &[(RecipeNodeId, &[NamedCoincidence])],
    resolver: Option<&std::sync::Arc<dyn crate::part::PartResolver>>,
    tol: Tol,
    cancel: &crate::CancelToken,
) -> Result<Vec<Vec<Result<Rung, Left>>>, Canceled> {
    let symbols = EvalOptions {
        symbols: true,
        resolver: resolver.cloned(),
        ..EvalOptions::default()
    };
    let (out, _) = with_replay_session(symbols, |opts| {
        let ev: Evaluation<Lane> = crate::eval::evaluate(doc, None, cancel, opts, tol);
        if ev.outcome == EvalOutcome::Canceled {
            return Err(Canceled);
        }
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
                        prove_row(&ev, node, row, at, band, cancel)
                    })
                    .collect()
            })
            .collect()
    });
    out
}

/// The two symbolic rungs for one row, `at` its ordinal among its
/// node's rows at its site. Canceled before it discharges a margin.
fn prove_row(
    ev: &Evaluation<Lane>,
    node: RecipeNodeId,
    row: &NamedCoincidence,
    at: usize,
    band: Option<Band>,
    cancel: &crate::CancelToken,
) -> Result<Result<Rung, Left>, Canceled> {
    // A row the replay left with no form of its own: nothing the door
    // could read, so the recourse is the assertion.
    let left = |why| Ok(Err((why, Recourse::Assert)));
    if row.site == topo::DecisionSite::CensusAtRest {
        // The census is not re-run: its row is read off the copies.
        if cancel.is_canceled() {
            return Err(Canceled);
        }
        return Ok(match band.map(|band| carrier_verdict(ev, row, band)) {
            Some(Some(Ok(()))) => Ok(Rung::CarrierVerdict),
            Some(Some(Err(why))) => Err(numeric(why)),
            Some(None) | None => Err((at_rest_unread(ev, node, row), Recourse::Assert)),
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
    if cancel.is_canceled() {
        return Err(Canceled);
    }
    if let Some(band) = band
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
/// as far as the door can tell, its recourse one construction unless
/// the form keeps a value computed apart from the variables.
fn numeric(why: deferred::Undischarged) -> Left {
    match why {
        deferred::Undischarged::KeepsAnOpaqueAtom => (Unreplayed::Numeric, Recourse::Assert),
        deferred::Undischarged::Otherwise => (Unreplayed::Numeric, Recourse::OneConstruction),
    }
}

/// Why the replay offers no carrier verdict on an at-rest row: a copy
/// it could not build (its placement or what it reads refused, or a
/// part it resolves none for), else a pair of cells that is not two
/// faces.
fn at_rest_unread(ev: &Evaluation<Lane>, node: RecipeNodeId, row: &NamedCoincidence) -> Unreplayed {
    let unbuilt = row.cells.iter().find_map(|cell| match cell {
        NamedCell::Entity { input, .. } => ev
            .table(*input)
            .is_none()
            .then(|| ev.read_port(*input).map_or(node, |(at, _)| at)),
        NamedCell::Tool { .. } | NamedCell::Piece { .. } => None,
    });
    match unbuilt {
        Some(at) => why_no_value(ev, at),
        None => Unreplayed::NotReplayed,
    }
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
/// build, or another relation decided).
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
    let [a, b] = row.cells.each_ref().map(|cell| face_of(ev, cell));
    let ((a, fa), (b, fb)) = (a?, b?);
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
