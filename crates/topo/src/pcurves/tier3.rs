//! **Tier 3**: the pass over a body's stored rows, its validator and
//! the loops' windings.

use geom::Surface;
use geom_brep::{CertifyingDoor, ChartImage, Pcurve, PcurveCache, Period};
use geom_core::Indeterminate;
use geom_core::predicate::Band;

use super::{
    ChartPeriods, DescribedChart, JointHalf, JointSides, Orbit, PcurveMintError, PinMiss, Singular,
    SingularMiss, Walked, decide_joint, derive_face, derive_image, entry_exit, entry_vertex,
    half_edge_carrier, has_singular_set, is_plus, joint_has_room, joint_refusal, loop_rows,
    not_owed, row_interval, singular_at, stored_edge, stored_reach, stored_rows, walk_halves,
};
use crate::body::Body;
use crate::entity::{EdgeKey, EntityId, FaceKey, HalfEdgeKey, LoopKey};
use crate::joint::{JointElement, Winding};
use crate::props::AtRestPolicy;

/// One loop of a face as [`loop_windings`] reads it off the walk.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct LoopWinding {
    /// The loop.
    pub r#loop: LoopKey,
    /// Some joint of the loop lies on the chart's singular set (a
    /// sphere pole, a cone apex), decided as 3-D incidence
    /// ([`singular_at`]). This reading stops at such a joint and
    /// reports it, with `zero` `false`: the loop's winding through the
    /// point is its lift's ([`Lift::winding`]).
    ///
    /// [`Lift::winding`]: super::Lift::winding
    pub singular: bool,
    /// The loop's joints compose to the identity deck element, in both
    /// channels ([`Winding::is_zero`]): its lift closes on its start.
    pub zero: bool,
}

/// One loop walked as [`loop_windings`] reads it ([`read_loop`]).
struct ReadLoop {
    /// Each joint's element, in walk order.
    elements: Vec<JointElement>,
    /// The first joint not definitely off the chart's singular set, and
    /// the reading's diagnostic where it was undecided rather than `On`.
    /// Nothing past the walk is read of such a loop.
    singular: Option<(HalfEdgeKey, Option<Indeterminate>)>,
}

/// **The loop walk [`loop_windings`] stands on**: `lp` walked in `chart`, and, off the singular
/// set, every closed-form row certified and every joint's room decided,
/// so the winding read off the elements is the loop's own. `None` for
/// an empty loop, which bounds nothing.
///
/// THE SINGULAR-JOINT FENCE comes first. A joint on the chart's singular
/// set — a sphere pole, a cone apex — takes the turn on the face's
/// side ([`decide_joint`]), which is the face's own on the chart it
/// stores and nothing on a candidate chart the caller offers. So a loop
/// with a joint not definitely off the set ([`singular_at`], the 3-D
/// incidence of each joint's vertex; net-level on a spline chart,
/// [`net_reach`]) is reported singular, and is walked as the open runs
/// between those joints: no turn is asked there and no winding read.
///
/// Off the singular set each row the closed-form door reads
/// ([`Pcurve::certifying_door`]) is certified against `chart` as the row
/// it is: the joint bound rests on each row's envelope (check 4), and
/// these rows are fresh derivations in the CALLER's chart, which no mint
/// certified. A row whose derivation's lane choice already certified it
/// in that chart ([`ChartImage::Certified`]) keeps that certificate, and
/// one only the fitted door reads is not certified here. Then THE ROOM
/// FENCE:
/// the walk decided each joint's azimuth integer with half the step to
/// the next orbit point as room, at the vertex's own lever, and where
/// that room is not past the joint bound a mark may have named an orbit
/// point other than the joint's own, so the winding may read one step
/// off ([`joint_has_room`]).
///
/// # Errors
///
/// The loop walk's own refusals ([`walk_loop`]);
/// [`PcurveMintError::Certify`] for a row that does not certify against
/// `chart`; [`PcurveMintError::JointWithoutRoom`] for a joint without
/// room.
///
/// [`net_reach`]: super::net_reach
/// [`walk_loop`]: super::walk_loop
fn read_loop<T: AtRestPolicy>(
    body: &Body<T>,
    face: FaceKey,
    lp: LoopKey,
    (chart, described): (&Surface<T>, DescribedChart<'_, T>),
    period: Option<Period<T>>,
    band: Band,
) -> Result<Option<ReadLoop>, PcurveMintError> {
    let cycle = loop_rows(body, lp);
    if cycle.is_empty() {
        return Ok(None);
    }
    let n = cycle.len();
    let fenced: Vec<usize> = (0..n)
        .filter(|&i| {
            !matches!(
                singular_at(described, entry_vertex(body, cycle[i]), band),
                Singular::Off
            )
        })
        .collect();
    let mut walked: Vec<Walked<T>> = Vec::new();
    if let Some(&first) = fenced.first() {
        // The runs between fenced joints, each walked open: every joint
        // off the set is decided and every image derived, and no joint
        // on it is asked for a turn in a chart the face may not be on.
        let singular = match singular_at(described, entry_vertex(body, cycle[first]), band) {
            Singular::Undecided(cause) => Some(cause),
            _ => None,
        };
        for (k, &start) in fenced.iter().enumerate() {
            let end = fenced.get(k + 1).map_or(first + n, |&next| next);
            let run: Vec<HalfEdgeKey> = (start..end).map(|i| cycle[i % n]).collect();
            walk_halves(body, face, &run, described, band, false, &mut walked)?;
        }
        return Ok(Some(ReadLoop {
            elements: Vec::new(),
            singular: Some((cycle[first], singular)),
        }));
    }
    walk_halves(body, face, &cycle, described, band, true, &mut walked)?;
    let elements: Vec<JointElement> = walked
        .iter()
        .map(|w| {
            w.element
                .unwrap_or_else(|| unreachable!("a closed walk decides every joint"))
        })
        .collect();
    for w in &walked {
        let (ChartImage::Image { pcurve, t0, t1 } | ChartImage::Stated { pcurve, t0, t1 }) =
            &w.image
        else {
            continue;
        };
        if pcurve.certifying_door() != CertifyingDoor::Fitted {
            PcurveCache::certify(pcurve.clone(), *t0, *t1, &w.carrier, chart, band).map_err(
                |error| PcurveMintError::Certify {
                    half_edge: w.key,
                    error: match w.image {
                        ChartImage::Image { .. } => {
                            geom_brep::minted_refusal(error, pcurve, &w.carrier, chart)
                        }
                        ChartImage::Stated { .. } | ChartImage::Certified(_) => error,
                    },
                },
            )?;
        }
    }
    for w in &walked {
        let orbit = Orbit::of(chart, w.pcurve());
        if !joint_has_room(described, entry_vertex(body, w.key), period, orbit, band) {
            return Err(PcurveMintError::JointWithoutRoom {
                face,
                r#loop: lp,
                half_edge: w.key,
            });
        }
    }
    Ok(Some(ReadLoop {
        elements,
        singular: None,
    }))
}

/// **How each loop of `face` winds in `chart`**, read off the loop walk:
/// what the section certificate's W2 reads
/// (`boolean::ops::ChartCache::describes`). Every closed-form row of a
/// loop off the chart's singular set is certified against `chart`, and
/// every joint's room decided, before its winding is read. A loop with a
/// joint on the singular set is reported `singular`, and nothing else of
/// it is read. Loops come outer first; an empty loop is absent.
///
/// # Errors
///
/// [`PcurveMintError::Stale`] when `face` does not resolve, and
/// [`PcurveMintError::PlaceholderChart`] when `chart` is the mvfs
/// placeholder; the loop walk's own refusals;
/// [`PcurveMintError::Certify`] for a row that does not certify against
/// `chart`; [`PcurveMintError::JointWithoutRoom`] for a joint without
/// room; and [`PcurveMintError::Escalated`] for a joint whose incidence
/// on the singular set is undecided.
pub fn loop_windings<T: AtRestPolicy>(
    body: &Body<T>,
    face: FaceKey,
    chart: &Surface<T>,
    band: Band,
) -> Result<Vec<LoopWinding>, PcurveMintError> {
    let view = body.face(face).ok_or(PcurveMintError::Stale {
        role: "face",
        key: EntityId::Face(face),
    })?;
    let described = DescribedChart::of(chart).ok_or(PcurveMintError::PlaceholderChart { face })?;
    let period = ChartPeriods::of(chart, band).u();
    let mut out = Vec::new();
    for (lp, _, _) in view.loops() {
        let Some(read) = read_loop(body, face, lp, (chart, described), period, band)? else {
            continue;
        };
        out.push(match read.singular {
            Some((half_edge, Some(cause))) => {
                return Err(PcurveMintError::Escalated { half_edge, cause });
            }
            Some((_, None)) => LoopWinding {
                r#loop: lp,
                singular: true,
                zero: false,
            },
            None => LoopWinding {
                r#loop: lp,
                singular: false,
                zero: Winding::of(read.elements).is_zero(),
            },
        });
    }
    Ok(out)
}

/// The at-rest pcurve pass the tier-3 validator runs (C4: on every
/// chart that mints, **every half-edge stores its certified row** —
/// except on a face whose rows are not owed ([`not_owed`]), the
/// exemption C4 keeps until each uncovered class's route lands — and
/// every stored row re-certifies).
///
/// For every face whose chart mints ([`DescribedChart::minting`]):
///
/// 1. **A face that stores none of its rows is re-derived** — the
///    minting pass over it, storing nothing ([`derive_face`]) — so the
///    finding says why the rows are missing: they derive and certify
///    ([`PcurveMintError::Unminted`]: the body was not minted after it
///    was built or edited), or the derivation refuses (that refusal).
///    A face whose rows are not owed ([`not_owed`]: an uncovered class,
///    or a fitted face at a scalar with no fitted door) is excused here
///    by the same predicate as at the mint, read over the whole face:
///    every edge the mint can image is walked through its joints and
///    certified as the mint would, so a covered edge's refusal is the
///    face's whichever uncovered edge shares it.
///    A face a null edge holds open ([`held_open`]) is not derived: the
///    edge has no carrier to derive from, and the scaffold is tier 2's
///    finding at rest.
/// 2. **A face that stores some of its rows** reports each gap
///    ([`PcurveMintError::MissingCache`]), and the derivation's refusal
///    where it refuses; the rows it does store are measured as a
///    complete face's are, below.
/// 3. **Every stored row states its edge's interval**
///    ([`PcurveMintError::RowInterval`] where it does not — a row left
///    from before a split or a carrier swap) **and is re-certified**
///    against its carrier and chart, on a complete face and a
///    half-minted one alike. The stored certificate is never consulted
///    (re-certification re-derives, it does not trust, exactly as
///    [`geom_brep::EdgeCurve::recertify`]). A row is its placed image
///    (C4), or on a sphere its involution twin; where a loop's walk reads
///    it a whole period over, that is the row with its joint's deck
///    element, never a stored row of its own.
/// 4. **Each joint's element is re-decided, and the winding is their
///    sum.** At every joint of every loop — the one into the cycle's
///    `first` read as every other — the element is decided between the
///    two images ([`decide_joint`]), and a stored element other than
///    the decided one is a [`PcurveMintError::LoopDiscontinuity`]. A
///    half-edge that stores no image is carried by the image the mint
///    would derive there, and the joints either side of it by the
///    elements decided there, so the rows either side of a gap are
///    measured across it; a gap the derivation cannot place leaves its
///    two joints unread. Where every joint reads, the elements composed
///    once around must close ([`Winding::closes`],
///    [`PcurveMintError::LoopNotClosed`]). So a body whose elements
///    were tampered with fails here even if each image certifies in
///    isolation, and no verdict hangs on which half-edge is `first`,
///    which is the gap, or which branch the loop's images stand on.
///
/// Returns the findings in face-arena / loop / cycle order (D9),
/// empty when the body is clean. A planar face stores nothing and is
/// not read.
///
/// # Panics
///
/// Where a record the pass reads does not resolve or a loop walk does
/// not close: tier 3, the one caller in the validator, runs only on a
/// body tier 1 passed, and every public door keeps the body
/// tier-1-valid.
///
/// [`held_open`]: super::held_open
#[track_caller]
pub fn validate_pcurves<T: AtRestPolicy>(body: &Body<T>, band: Band) -> Vec<PcurveMintError> {
    pcurve_pass(body, band).findings
}

/// **A wrap tip tier 3 does not read on its face's chart singularity**
/// ([`pcurve_pass`]).
#[derive(Clone, Copy, Debug)]
pub(crate) struct TipOff {
    /// The face whose loop walks the wrap edge out and back.
    pub(crate) face: FaceKey,
    /// The wrap edge.
    pub(crate) edge: EdgeKey,
    /// Its tip.
    pub(crate) tip: crate::entity::VertexKey,
    /// What the incidence reading said.
    pub(crate) reading: TipReading,
}

/// Why a wrap tip is not on its face's chart singularity ([`TipOff`]).
#[derive(Clone, Copy, Debug)]
pub(crate) enum TipReading {
    /// Decided off a sphere's poles or a cone's apex.
    Off,
    /// The chart has no singular point for a tip to sit on: a cylinder,
    /// a torus, a spline net whose `u` sweep moves a point.
    NoSingularSet,
    /// In band: the escalation, carrying the margin.
    Undecided(Indeterminate),
}

/// [`validate_pcurves`]' findings, and the wrap tips its joint reading
/// places off their face's chart singularity.
pub(crate) struct PcurvePass {
    pub(crate) findings: Vec<PcurveMintError>,
    pub(crate) tips: Vec<TipOff>,
}

/// **[`validate_pcurves`], with tier 3's wrap-tip rule read off the same
/// joints** (DESIGN `[topology.tiers]`: every wrap edge's tip lies on its
/// face's chart singularity). The tip ([`crate::validate::wrap_tips`]) is
/// the vertex of the joint between the wrap edge's two halves, and
/// whether that vertex lies on the chart's singular set is
/// [`singular_at`], the reading [`decide_joint`] takes the face's turn
/// by: decided on, the tip is admitted; off, or on a chart with
/// no singular set, it is refused; undecided, it is refused as
/// undecided. A face whose chart mints no rows (a plane) holds no tip: a
/// wrap description is refused on a plane at the mint. A tip whose joint the
/// pass does not read (a gap the derivation cannot place) or reads as a
/// finding of its own ([`PcurveMintError::LoopDiscontinuity`], an
/// escalation) is that finding's, and reports nothing here, so one joint
/// is never refused twice.
#[track_caller]
pub(crate) fn pcurve_pass<T: AtRestPolicy>(body: &Body<T>, band: Band) -> PcurvePass {
    let tips_at = crate::validate::wrap_tips(body);
    let tip_at = |he: HalfEdgeKey| {
        let h = body.get_half_edge(he)?;
        (tips_at.get(h.start) == Some(&he)).then_some((h.edge, h.start))
    };
    let mut tips = Vec::new();
    let mut findings = Vec::new();
    // The fitted door every `Fitted`/`General` row re-derives through,
    // read off the scalar's policy once: the certified and the
    // structural validation doors reach this pass alike, so neither
    // moves a verdict at any scalar.
    let lane = T::fitted_lane();
    for face in body.face_views() {
        let face_key = face.key();
        let surface = face.surface();
        let Some(chart) = DescribedChart::minting(surface) else {
            continue;
        };
        // One walk of the face's loops ([`stored_rows`], shared with
        // `split_cache` and the Euler operators' site mint): the rows
        // it stores and the gaps.
        let stored = stored_rows(body, face_key);
        if !stored.complete() {
            // A loop a null edge holds open cannot be walked (its halves
            // have no carrier), so there is no derivation to read; the
            // scaffold is tier 2's finding at rest, and the gaps are
            // reported as found.
            let open = !stored.open_loops(body).is_empty();
            let refusal = if open {
                None
            } else {
                derive_face(body, face_key, band).err()
            };
            if !stored.stores {
                match refusal {
                    None if stored.gaps.is_empty() || open => {}
                    None => findings.push(PcurveMintError::Unminted { face: face_key }),
                    Some(e) if not_owed::<T>(&e) => {}
                    Some(e) => findings.push(e),
                }
                continue;
            }
            findings.extend(stored.gaps.iter().map(|gap| PcurveMintError::MissingCache {
                half_edge: gap.half_edge,
            }));
            if let Some(e) = refusal.filter(|e| !not_owed::<T>(e)) {
                findings.push(e);
            }
        }
        let cycles: Vec<Vec<HalfEdgeKey>> =
            stored.loops.into_iter().map(|(_, cycle)| cycle).collect();
        // Re-certify every stored row against its carrier and chart.
        for cycle in &cycles {
            for &he in cycle {
                let Some(cache) = body.pcurve(he) else {
                    continue;
                };
                let (carrier, t0, t1) = match half_edge_carrier(body, he) {
                    Ok(found) => found,
                    Err(e) => {
                        findings.push(e);
                        continue;
                    }
                };
                if let Err(e) = row_interval(body, he, cache, &carrier, band) {
                    findings.push(e);
                }
                if let Err(error) = cache.recertify(&carrier, (t0, t1), surface, band, lane) {
                    findings.push(PcurveMintError::Certify {
                        half_edge: he,
                        error,
                    });
                }
            }
        }
        // Each joint's element, re-decided between the two images it
        // joins, and each loop's winding composed from them. A half that
        // stores no image is carried by the image the mint would derive
        // for it ([`derive_image`]), so the rows either side of a gap are
        // measured across it; a gap the derivation cannot place leaves
        // its two joints unread and the loop's winding unread with them.
        // Every joint is read alike, the one into the cycle's `first`
        // included, and the winding off conjugation invariants
        // ([`Winding::closes`]), so no verdict hangs on which half-edge
        // is `first`.
        let periods = ChartPeriods::of(surface, band);
        let wedges = crate::wedge_reach::WedgeReach::new(body);
        let has_singular_points = has_singular_set(surface);
        let sense = face.sense();
        let image_of = |he: HalfEdgeKey| -> Option<(Pcurve<T>, T, T)> {
            let plus = is_plus(body, he);
            let (image, t0, t1) = match body.pcurve(he) {
                Some(cache) => {
                    let (t0, t1) = cache.params();
                    (cache.pcurve().clone(), t0, t1)
                }
                None => {
                    let (_, derived) = derive_image(body, he, surface, band).ok()?;
                    let (t0, t1) = derived.window();
                    (derived.into_pcurve(), t0, t1)
                }
            };
            let (entry_t, exit_t) = entry_exit(plus, t0, t1);
            Some((image, entry_t, exit_t))
        };
        for cycle in &cycles {
            let n = cycle.len();
            let images: Vec<Option<(Pcurve<T>, T, T)>> =
                cycle.iter().map(|&he| image_of(he)).collect();
            let mut elements: Vec<Option<JointElement>> = vec![None; n];
            // The decision of the joint into member `i`, with the rest
            // of the loop where it is known; `None` where an image
            // either side of it is not read.
            let decide_at = |i: usize, rest: Option<Winding>| {
                let he = cycle[i];
                let arrived_on = cycle[(i + n - 1) % n];
                let (
                    Some((image, entry_t, leaving_far)),
                    Some((before, arrived_far, exit_t)),
                    Ok((leaving, ..)),
                    Ok((arrived, ..)),
                ) = (
                    &images[i],
                    &images[(i + n - 1) % n],
                    half_edge_carrier(body, he),
                    half_edge_carrier(body, arrived_on),
                )
                else {
                    return None;
                };
                let arrived_reach = || stored_reach(&wedges, body, arrived_on);
                let leaving_reach = || stored_reach(&wedges, body, he);
                Some(decide_joint(
                    chart,
                    image,
                    *entry_t,
                    before.eval(*exit_t),
                    entry_vertex(body, he),
                    periods,
                    band,
                    JointSides::of_halves(
                        sense,
                        JointHalf {
                            carrier: &arrived,
                            entry: *arrived_far,
                            exit: *exit_t,
                            plus: is_plus(body, arrived_on),
                            edge: stored_edge(body, arrived_on),
                            reach: &arrived_reach,
                        },
                        JointHalf {
                            carrier: &leaving,
                            entry: *entry_t,
                            exit: *leaving_far,
                            plus: is_plus(body, he),
                            edge: stored_edge(body, he),
                            reach: &leaving_reach,
                        },
                        rest,
                    ),
                ))
            };
            // A stored element is read only between two stored images:
            // across a gap the chain is carried as the walk would carry
            // it, by the element decided there.
            let stored_at = |i: usize| {
                body.joint(cycle[i]).filter(|_| {
                    body.pcurve(cycle[i]).is_some() && body.pcurve(cycle[(i + n - 1) % n]).is_some()
                })
            };
            let audit =
                |stored: Option<JointElement>, decided: Result<JointElement, PinMiss>| match (
                    stored, decided,
                ) {
                    (Some(stored), Ok(element)) if stored == element => Ok(stored),
                    (Some(_), Ok(_)) => Err(PinMiss::Discontinuity),
                    (_, decided) => decided,
                };
            // The joints where two distinct edges leave a singular point
            // along one direction: each is decided with the rest of its
            // loop, once every other joint is read ([`singular_turn`]).
            let mut slits: Vec<usize> = Vec::new();
            let mut missed = false;
            for (i, &he) in cycle.iter().enumerate() {
                let Some(decided) = decide_at(i, None) else {
                    continue;
                };
                if matches!(decided, Err(PinMiss::Singular(SingularMiss::EdgesCoincide))) {
                    // Read below; the chain is carried meanwhile by what
                    // the joint stores.
                    slits.push(i);
                    elements[i] = stored_at(i);
                    continue;
                }
                match audit(stored_at(i), decided) {
                    Ok(element) => {
                        elements[i] = Some(element);
                        if let Some((edge, tip)) = tip_at(he) {
                            let reading = match singular_at(chart, entry_vertex(body, he), band) {
                                Singular::On(_) | Singular::Collapsed => continue,
                                Singular::Off if has_singular_points => TipReading::Off,
                                Singular::Off => TipReading::NoSingularSet,
                                Singular::Undecided(cause) => TipReading::Undecided(cause),
                            };
                            tips.push(TipOff {
                                face: face_key,
                                edge,
                                tip,
                                reading,
                            });
                        }
                    }
                    Err(miss) => {
                        missed = true;
                        findings.push(joint_refusal(he, miss));
                    }
                }
            }
            // A slit's finding is its own decision's, read with the rest
            // of the loop. Where another joint of the loop is not read,
            // that joint's finding stands and the slit has no reading to
            // be a second one.
            for &slit in &slits {
                let rest: Option<Vec<JointElement>> =
                    (1..n).map(|k| elements[(slit + k) % n]).collect();
                let Some(rest) = rest else {
                    if !missed {
                        findings.push(joint_refusal(
                            cycle[slit],
                            PinMiss::Singular(SingularMiss::EdgesCoincide),
                        ));
                    }
                    continue;
                };
                let Some(decided) = decide_at(slit, Some(Winding::of(rest))) else {
                    continue;
                };
                match audit(stored_at(slit), decided) {
                    Ok(element) => elements[slit] = Some(element),
                    Err(miss) => findings.push(joint_refusal(cycle[slit], miss)),
                }
            }
            if n > 0
                && elements.iter().all(Option::is_some)
                && !Winding::of(elements.iter().flatten().copied()).closes()
            {
                findings.push(PcurveMintError::LoopNotClosed { face: face_key });
            }
        }
    }
    PcurvePass { findings, tips }
}
