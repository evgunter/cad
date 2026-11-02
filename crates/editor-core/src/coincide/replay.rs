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
use topo::{CarrierRelation, FaceKey};

use super::{Canceled, Component, NamedCell, NamedCoincidence, Recourse, Rung, Unreplayed};
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

/// A rung that proved a row, or why the replay could not and, for two
/// cells with an axis, the first component of one axis, one radius and
/// one sense it did not establish.
type Proved = Result<Rung, (Left, Option<Component>)>;

/// **The symbolic rungs for each row the first rung left**
/// (`left`, in the shape of `nodes`), each row's two axes read as the
/// mates state them where `axes_one` holds: in the order given, a rung that
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
    (left, axes_one): (&[Vec<bool>], &[Vec<bool>]),
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
        for ((&(node, rows), asks), axes_one) in nodes.iter().zip(asks).zip(axes_one) {
            let mut ordinals = BTreeMap::new();
            let mut answers = Vec::with_capacity(rows.len());
            for ((row, ask), &stated) in rows.iter().zip(asks).zip(axes_one) {
                let ordinal = ordinals.entry(row.site).or_insert(0usize);
                let at = core::mem::replace(ordinal, *ordinal + 1);
                answers.push(match ask {
                    None => None,
                    Some(Err(why)) => Some(Err((why, None))),
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
                        Some(prove_row(ev, (node, row, at), stated, (band, tol), cancel)?)
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
/// node's rows at its site, its two axes read as the mates state them
/// where `stated` ([`carrier_verdict`]); an at-rest row is read off the
/// copies by the carrier verdict alone (the census is not re-run).
/// Canceled before it discharges a margin.
fn prove_row(
    ev: &Evaluation<Lane>,
    (node, row, at): (RecipeNodeId, &NamedCoincidence, usize),
    stated: bool,
    (band, tol): (Result<Band, BandError>, Tol),
    cancel: &crate::CancelToken,
) -> Result<Proved, Canceled> {
    if cancel.is_canceled() {
        return Err(Canceled);
    }
    // A row the replay left with no form of its own: nothing the door
    // could read, so the recourse is the assertion, unless the replay
    // ran out of member budget before it.
    let left = |why| {
        let recourse = match why {
            Unreplayed::Exhausted => Recourse::MemberBudget,
            _ => Recourse::Assert,
        };
        Ok(Err(((why, recourse), None)))
    };
    if row.site == topo::DecisionSite::CensusAtRest {
        if let Some(why) = at_rest_unbuilt(ev, node, row) {
            return left(why);
        }
        // A run whose tolerance makes no band takes no decision.
        let Ok(band) = band else {
            return left(Unreplayed::Refused);
        };
        return Ok(match carrier_verdict(ev, row, band, tol, stated) {
            Some(Ok(())) => Ok(Rung::CarrierVerdict),
            Some(Err(CarrierFail::Undischarged(why))) => Err((numeric(why), None)),
            Some(Err(CarrierFail::Left(component, why))) => Err((numeric(why), Some(component))),
            Some(Err(CarrierFail::Frame)) => Err(((Unreplayed::Frame, Recourse::Assert), None)),
            None => Err(((Unreplayed::OtherDecisions, Recourse::Assert), None)),
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
    let mut component = None;
    if let Ok(band) = band {
        match carrier_verdict(ev, row, band, tol, stated) {
            Some(Ok(())) => return Ok(Ok(Rung::CarrierVerdict)),
            Some(Err(CarrierFail::Frame)) => return left(Unreplayed::Frame),
            Some(Err(CarrierFail::Left(left, _))) => component = Some(left),
            Some(Err(CarrierFail::Undischarged(_))) | None => {}
        }
    }
    if cancel.is_canceled() {
        return Err(Canceled);
    }
    Ok(deferred::discharge_or_why(replayed.margin)
        .map(|()| Rung::MarginIdentity)
        .map_err(|why| (numeric(why), component)))
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

/// Why the replay holds no value at `node`: it ran out of member budget
/// before it, a part it does not resolve where that is what failed
/// there or upstream, else a refusal.
fn why_no_value(ev: &Evaluation<Lane>, node: RecipeNodeId) -> Unreplayed {
    if matches!(ev.outcome, EvalOutcome::Exhausted { .. })
        && matches!(
            ev.usable(node),
            Err(crate::eval::NodeStanding::NotEvaluated { .. })
        )
    {
        return Unreplayed::Exhausted;
    }
    match ev.node_error(node).map(|e| &e.kind) {
        Some(crate::eval::NodeErrorKind::Part {
            fault: crate::eval::PartFault::NoResolver,
            ..
        }) => Unreplayed::ThroughAPart,
        _ => Unreplayed::Refused,
    }
}

/// Why a carrier verdict the replay reads is not proven.
enum CarrierFail {
    /// Its Zero is not discharged.
    Undischarged(deferred::Undischarged),
    /// One cell's body could not be read into the other's frame.
    Frame,
    /// Two cylinders, read as one axis, one radius and one sense
    /// ([`one_cylinder`]): this component is not established, for this
    /// reason.
    Left(Component, deferred::Undischarged),
}

/// **The kernel's carrier-pair verdict on the row's two cells**, where
/// the row decided two faces one carrier: whether every Zero it stands
/// on is proven, or why the first that is not keeps, or that one cell
/// could not be read into the frame the verdict is decided in; `None`
/// where the replay reads no such verdict (not two faces, a face it did
/// not build, or another relation decided). A meeting of cells at a
/// point is read by the margin that decided it, which the replay
/// computes from the cells' symbolic carriers ([`Rung::MarginIdentity`]).
///
/// Two cylinders are also read by their axis and the numbers a rigid
/// motion keeps ([`one_cylinder`]), the axis as the mates state it
/// where `stated` (`[placement.placement]`: the door "reads by what its
/// mates state and replays where they state nothing"), so not as the
/// fold builds it. Either reading proves the row.
fn carrier_verdict(
    ev: &Evaluation<Lane>,
    row: &NamedCoincidence,
    band: Band,
    tol: Tol,
    stated: bool,
) -> Option<Result<(), CarrierFail>> {
    let wanted = match row.relation {
        topo::Relation::SameOriented => CarrierRelation::SameOriented,
        topo::Relation::SameOpposite => CarrierRelation::SameOpposite,
        _ => return None,
    };
    let [a, b] = row.cells.as_slice() else {
        return None;
    };
    let ((a, fa), (b, fb)) = (face_of(ev, a)?, face_of(ev, b)?);
    // Decided in the frame `[placement.frame]` conditions on (the cell
    // in its space's coordinates, else the first), the other read in.
    let computing = crate::eval::Framed::computing_among([a, b])?;
    let (Ok(a_in), Ok(b_in)) = (computing.read_in(a, tol), computing.read_in(b, tol)) else {
        return Some(Err(CarrierFail::Frame));
    };
    let kernel = || match topo::boolean::carrier_pair_decided(&a_in, fa, &b_in, fb, band) {
        Ok(Ok((relation, Some(margin)))) if relation == wanted => {
            Some(deferred::discharge_or_why(margin).map_err(CarrierFail::Undischarged))
        }
        _ => None,
    };
    match one_cylinder((&a_in, fa), (&b_in, fb), wanted, band, stated) {
        None => kernel(),
        Some(Ok(())) => Some(Ok(())),
        Some(Err((component, why))) => match kernel() {
            Some(Ok(())) => Some(Ok(())),
            _ => Some(Err(CarrierFail::Left(component, why))),
        },
    }
}

/// **Two faces on cylinders, read as one axis, one radius and one
/// sense**: the axis as stated where `stated`, else the two axes one
/// line in the symbols; the radii's difference zero in the symbols (one
/// `Length` read twice), and each face's rims ([`rim_radii`]) its own
/// cylinder's radius, so a wall a revolve read as a cylinder from the
/// values of its meridian's two ends is not one radius by that reading
/// alone; the material sides' bits giving `wanted`. The first component
/// not established and why; `None` where either face is not on a
/// cylinder.
fn one_cylinder(
    (a, fa): (&topo::Body<Lane>, FaceKey),
    (b, fb): (&topo::Body<Lane>, FaceKey),
    wanted: CarrierRelation,
    band: Band,
    stated: bool,
) -> Option<Result<(), (Component, deferred::Undischarged)>> {
    use geom_core::Decide;
    use topo::boolean::CarrierDesc::Cylinder;
    let (
        Some(Cylinder {
            origin: oa,
            axis: da,
            radius: ra,
            outward: sa,
        }),
        Some(Cylinder {
            origin: ob,
            axis: db,
            radius: rb,
            outward: sb,
        }),
    ) = (
        topo::boolean::face_carrier(a, fa),
        topo::boolean::face_carrier(b, fb),
    )
    else {
        return None;
    };
    let zero = |x: Lane| {
        x.sign_within(band)
            .map_or(Err(deferred::Undischarged::Otherwise), |d| {
                deferred::discharge_or_why(d.margin)
            })
    };
    let established = || {
        if !stated {
            let (cross, off) = (da.cross(db), (ob - oa).cross(db));
            for r in [cross.x, cross.y, cross.z, off.x, off.y, off.z] {
                zero(r).map_err(|why| (Component::Axis, why))?;
            }
        }
        let radius = |why| (Component::Radius, why);
        for ((body, face), r) in [((a, fa), &ra), ((b, fb), &rb)] {
            let rims = rim_radii(body, face);
            if rims.len() < 2 {
                return Err(radius(deferred::Undischarged::Otherwise));
            }
            for rim in rims {
                zero(rim - *r).map_err(radius)?;
            }
        }
        zero(ra - rb).map_err(radius)?;
        if (sa == sb) != (wanted == CarrierRelation::SameOriented) {
            return Err((Component::Sense, deferred::Undischarged::Otherwise));
        }
        Ok(())
    };
    Some(established())
}

/// **The radii of the circles bounding `face`**: a revolved wall's
/// rims, each the radius of the meridian end it turns, where the
/// wall's cylinder reads one end's alone.
fn rim_radii(body: &topo::Body<Lane>, face: FaceKey) -> Vec<Lane> {
    let Some(f) = body.get_face(face) else {
        return Vec::new();
    };
    let mut radii = Vec::new();
    for lp in core::iter::once(f.outer).chain(f.rings.iter().copied()) {
        let first = match body.get_loop(lp).map(|l| &l.boundary) {
            Some(topo::LoopBoundary::Cycle { first }) => *first,
            // A lone vertex bounds no rim.
            Some(topo::LoopBoundary::Empty { .. }) | None => continue,
        };
        let mut at = first;
        while let Some(he) = body.get_half_edge(at) {
            if let Some(geom::Curve3::Circle { radius, .. }) = body
                .get_edge(he.edge)
                .and_then(|e| body.get_curve_geom(e.curve))
                .and_then(topo::CurveGeom::certified)
                .map(geom_brep::EdgeCurve::carrier)
            {
                radii.push(*radius);
            }
            at = he.next;
            if at == first {
                break;
            }
        }
    }
    radii
}

/// The face `cell` names, as the replay built it: the body of the read
/// it came in through and its key there, read in that read's own table.
/// A product `⊔`'s name of the read's entity, keyed by that read, is
/// the read's own name there.
fn face_of<'e>(
    ev: &'e Evaluation<Lane>,
    cell: &NamedCell,
) -> Option<(&'e crate::eval::Framed<Lane>, FaceKey)> {
    let NamedCell::Entity { input, name } = cell else {
        return None;
    };
    let name = match name.path.as_slice() {
        [crate::names::RoleSeg::From { read, of }] if read == input => &**of,
        _ => name,
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
