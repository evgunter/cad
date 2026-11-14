//! **The doors' limbs**: an Euler operator's site mint, a split's
//! carried rows and a kill's turn.

use geom::Surface;
use geom_brep::{ChartImage, Pcurve, PcurveCache, PcurveCertifyError, chart_pcurve_over};
use geom_core::k_stats::decide;
use geom_core::predicate::Band;
use geom_core::{Decide, Indeterminate, Margin, Real, Sign};

use super::{
    ChartPeriods, DescribedChart, JointHalf, JointSides, PcurveMintError, PinMiss, SingularMiss,
    SiteImage, StoredRows, WalkEdge, WalkItem, Walked, certify_walked, decide_joint, edge_reach,
    edge_vertex_points, entry_exit, entry_vertex, half_edge_carrier, half_edge_face,
    half_edge_record, half_edge_surface, held_open, is_plus, loop_rows, misses_a_row, row_interval,
    stored_edge, stored_reach, stored_rows, walk_cycle,
};
use crate::body::Body;
use crate::entity::{EdgeKey, FaceKey, HalfEdgeKey, LoopKey};
use crate::joint::{JointElement, Winding};
use crate::props::AtRestPolicy;

/// The two rows a parameter split leaves where ONE parent half-edge's
/// row was, named by where they go: the first child's `[t₀, t]` stays
/// on the parent half (which IS the first child's half), the second
/// child's `[t, t₁]` goes to the new half minted beside it.
pub(crate) struct CarriedRows<T: Real> {
    /// The first child's row, for the parent half-edge.
    pub(crate) parent_half: PcurveCache<T>,
    /// The second child's row, for the half-edge minted beside it.
    pub(crate) new_half: PcurveCache<T>,
    /// The element of the joint between the two children, at the split
    /// point: the identity, the two children being one image — the
    /// face's turn where that point is on the chart's singular set.
    pub(crate) joint: JointElement,
    /// Where a child's row was placed again over its own span: the
    /// element of the joint into the child the parent's incoming joint
    /// fed (the first child on a plus half, the second on a minus one),
    /// decided against the half before it.
    pub(crate) entering: Option<JointElement>,
    /// Likewise, the element of the joint out of the child whose exit is
    /// the parent's, into the half after it.
    pub(crate) leaving: Option<JointElement>,
}

/// Why [`split_cache`] could not state a child's row, for one parent
/// half-edge. Raised, never swallowed: the split leaves nothing it
/// could not state.
#[derive(Clone, Debug)]
pub(crate) struct SplitRowError {
    /// The parent half-edge whose children's rows were being stated.
    pub(crate) half_edge: HalfEdgeKey,
    /// What refused.
    pub(crate) refusal: SplitRefusal,
}

/// The two refusals of [`split_cache`].
#[derive(Clone, Debug)]
pub(crate) enum SplitRefusal {
    /// A child's row refused over its sub-interval: the derivation that
    /// places a closed-form or projected child there, or the
    /// certification of the child's row (derived, or a fitted or
    /// general parent's restricted), which the whole image passed. A
    /// covered lane that refuses here is a genuine defect.
    Certify(PcurveCertifyError),
    /// The one decider decided no element for a joint the split makes
    /// ([`decide_joint`]): between the two children, or one of the
    /// parent's end joints decided again against its neighbour (or, on a
    /// one-edge loop, its own other child). At the children's joint the
    /// gap is zero, and a
    /// zero gap meets the image's own orbit point at any lever
    /// ([`lever_periods`]), even a point off a narrow cone's apex by
    /// more than the band whose lever is under it: what refuses is a
    /// reading the band cannot decide (`Some`, with its cause), or a
    /// decided miss (`None`) of a channel that read does not serve.
    ///
    /// [`lever_periods`]: super::lever_periods
    Joint(Option<Indeterminate>),
}

/// The **rows of an edge's two half-edges stated over the two
/// children of a parameter split** — [`crate::Body::split_edge`](crate::construction::Construct::split_edge)'s
/// pcurve limb, and the reason that op carries its rows across the
/// surgery instead of staling them. The answer holds one entry per
/// given half-edge: that half-edge's [`CarriedRows`], or `None` where
/// there is nothing to carry.
///
/// A row is its image placed over its own edge's span in the chart's
/// principal window (C4), so a child of a split at `t` is stated over
/// `[t₀, t]` or `[t, t₁]` as the mint would state it: a closed-form or
/// projected image is derived there ([`geom_brep::chart_pcurve_over`]),
/// and a fitted or general image, which has no angular placement, is
/// the parent's restricted to the sub-interval. Each is certified
/// through the image's own door ([`PcurveCache::certify_stated_image`], with the scalar's
/// [`crate::AtRestPolicy::fitted_lane`] for a projected row's hull
/// terms and a fitted-grade row's check 4). Children placed again can
/// sit whole turns from each other and from the parent's image, so the
/// joint between them and the parent's joints into and out of them are
/// decided here, as every joint is ([`CarriedRows`]).
///
/// **A fitted-grade row's check 4 reads the WHOLE carrier**, not the
/// child's span: its hull bounds the image's whole domain against the
/// chart. Its inputs (carrier, image, surface, band) are the parent's,
/// so at the band the parent's
/// row was certified at, each child's certificate IS the parent's. The
/// statement is conservative for a child, and such a split never
/// refuses a child at check 4 unless the parent's own row already
/// fails it. At a TIGHTER band than the parent's, check 4 can refuse a
/// child: a typed refusal ([`SplitRefusal::Certify`]), never a false
/// certificate.
///
/// **The joint between the children** is decided at `t`, where both
/// children's images hold one chart point, so its gap is zero and no
/// period is crossed. Off the chart's singular set it decides the
/// identity shift; at a split point on that set it decides the face's
/// turn there ([`decide_joint`]).
///
/// **`t₀` and `t₁` are the EDGE's certified interval**, read from the
/// carrier through [`half_edge_carrier`] — the same interval
/// `EdgeCurve::split_specs` cuts the children's curves from — and not
/// the row's own stored `cache.params()`. The two agree on a face the
/// minting pass wrote; where a stored row disagrees, the edge is what
/// the children are made of and the row is what is being re-certified,
/// so the carrier decides the sub-intervals and the certification
/// measures the row against them.
///
/// Read-only, so a refusal reaches `split_edge` before any mutation
/// and the op's "untouched on `Err`" contract is unaffected.
///
/// # `None`
///
/// `half_edge` carries no row (an all-planar body, a face of an
/// uncovered class, or one a door has left rowless for its producer's
/// closing mint). The caller writes nothing, so the map is left exactly
/// as found. A key that does not resolve is NOT one of them: it
/// panics (below).
///
/// # What still reads a split body short
///
/// The carry is exact on every kind it covers, spline charts included.
/// Two readers downstream of it are narrower:
///
/// - **The iso rim arms** are not among them. On a described-NURBS
///   wall the carried `IsoLine` and `IsoArc` rows restrict like any
///   other, and [`mint_pcurves`] re-derives them: [`nurbs_iso_derive`]'s
///   line rim takes the row its end feet measure, and its arc rim the
///   row over the arc the row measures (`sweep`'s
///   `split_edge_loft_charts`). A carried fitted-grade row is
///   re-derived bit for bit (`sweep`'s `split_edge_fitted_rows`).
/// - **The mesher's chord schedule.** It lands chord points on an arc
///   rim's sub-arc joints only where the row's arc is the edge's
///   interval, so it refuses a split rim's child typed
///   (`work/tess/a-split-arc-rim-meshes-no-chord-point-on-its-sub-arc-joints.md`).
/// - **The trimmed quadrature lane.** It subdivides a `General` image
///   over its whole knot domain, so it refuses a carried child's
///   sub-range typed rather than integrate chart the face does not
///   bound. A split of a seam that holds a `General` row is a standing
///   producer of that shape, filed as
///   `work/quad/a-fitted-cap-cut-by-planes-has-a-sub-range-trim-image.md`.
///   No regression: before this carry the split left the face
///   rowless, which the lane refuses too.
///
/// # Errors
///
/// [`SplitRowError`] — a certification the whole image passed failing
/// over a sub-interval ([`SplitRefusal::Certify`]), or the joint
/// between the two children undecided at the split point
/// ([`SplitRefusal::Joint`]): the split writes no element the one
/// decider did not decide.
///
/// # Panics
///
/// Where a record the restriction reads does not resolve — each half's
/// edge, curve, loop, face and surface — or a half's edge is null
/// scaffolding: `halves` are the live halves of an edge whose curve
/// [`crate::Body::split_edge`](crate::construction::Construct::split_edge) resolved `Certified`.
///
/// [`mint_pcurves`]: super::mint_pcurves
/// [`nurbs_iso_derive`]: super::nurbs_iso_derive
#[track_caller]
pub(crate) fn split_cache<T: AtRestPolicy>(
    body: &Body<T>,
    halves: [HalfEdgeKey; 2],
    t: T,
    band: Band,
) -> Result<[Option<CarriedRows<T>>; 2], SplitRowError> {
    let mut rows = [None, None];
    for (slot, half_edge) in halves.into_iter().enumerate() {
        let Some(cache) = body.pcurve(half_edge) else {
            continue;
        };
        let (carrier, t0, t1) = half_edge_carrier(body, half_edge).unwrap_or_else(|e| {
            unreachable!("split_edge resolved the curve of {half_edge:?}'s edge Certified: {e}")
        });
        let surface = half_edge_surface(body, half_edge);
        let image = cache.pcurve().clone();
        let refused = |error| SplitRowError {
            half_edge,
            refusal: SplitRefusal::Certify(error),
        };
        let certify = |a: T, b: T, image: Pcurve<T>| {
            PcurveCache::certify_stated_image(
                &image,
                (a, b),
                &carrier,
                &surface,
                band,
                T::fitted_lane(),
            )
            .map_err(refused)
        };
        let Some(chart) = DescribedChart::of(&surface) else {
            unreachable!(
                "{half_edge:?} stores a row, and rows are minted only on a described chart \
                 (`DescribedChart::minting`)"
            )
        };
        let periods = ChartPeriods::of(&surface, band);
        let sense = half_edge_face(body, half_edge).sense();
        // The joint of `image`'s entry onto `onto`, the exit of the image
        // before it, each on its half: `arrived` and `leaving`.
        let wedges = crate::wedge_reach::WedgeReach::new(body);
        let pin = |image: &Pcurve<T>,
                   onto: geom_core::Point2<T>,
                   arrived: JointHalf<'_, T>,
                   leaving: JointHalf<'_, T>,
                   rest: Option<Winding>| {
            decide_joint(
                chart,
                image,
                leaving.entry,
                onto,
                leaving.carrier.eval(leaving.entry),
                periods,
                band,
                JointSides::of_halves(sense, arrived, leaving, rest),
            )
        };
        let missed = |miss: PinMiss| SplitRowError {
            half_edge,
            refusal: SplitRefusal::Joint(match miss {
                PinMiss::Escalated(cause) | PinMiss::NetStretchUndecided(cause) => Some(cause),
                PinMiss::Discontinuity | PinMiss::Singular(_) => None,
            }),
        };
        // The two children as the half's loop traverses them, each
        // reaching over its own span between the edge's faces
        // ([`edge_reach`]): two distinct edges once the split lands.
        let plus = is_plus(body, half_edge);
        let faces = crate::wedge_reach::edge_faces(body, half_edge_record(body, half_edge).edge);
        let first_reach = || Some(wedges.reach(faces, &carrier, (t0, t)));
        let second_reach = || Some(wedges.reach(faces, &carrier, (t, t1)));
        let child_half = |second: bool| -> JointHalf<'_, T> {
            let (a, b) = if second { (t, t1) } else { (t0, t) };
            let (entry, exit) = entry_exit(plus, a, b);
            let reach: &dyn Fn() -> Option<geom_core::Lever<T>> =
                if second { &second_reach } else { &first_reach };
            JointHalf {
                carrier: &carrier,
                entry,
                exit,
                plus,
                edge: if second {
                    WalkEdge::New
                } else {
                    stored_edge(body, half_edge)
                },
                reach,
            }
        };
        // C4 places a row over its own edge's span, in the chart's
        // principal window, so a child's row is the placement over its
        // sub-interval: a closed-form or projected image is derived
        // there, exactly as the mint derives it. A fitted, general or iso
        // image has no angular placement, and its children restrict the
        // parent's (`Pcurve::is_placed`).
        let placed_again = image.is_placed();
        // A derivation that refuses over a child's span refuses the split
        // (fail-loud): the parent's image is not the child's row there.
        let child = |a: T, b: T| {
            if placed_again {
                geom_brep::chart_pcurve_over(&carrier, a, b, &surface, band).map_err(refused)
            } else {
                Ok(image.clone())
            }
        };
        let (first, second) = (child(t0, t)?, child(t, t1)?);
        // The loop runs from the child the parent's entry fed into the
        // other: first into second on a plus half, the other way on a
        // minus one.
        let (fed, out) = if plus {
            (&first, &second)
        } else {
            (&second, &first)
        };
        let (fed_half, out_half) = (child_half(!plus), child_half(plus));
        // A neighbour's stored row as a half of the loop. One with no
        // row, or one of this edge's own halves (a strut's tip), keeps
        // its element.
        let neighbour = |he: HalfEdgeKey| -> Option<(Pcurve<T>, T, T, geom::Curve3<T>)> {
            if halves.contains(&he) {
                return None;
            }
            let row = body.pcurve(he)?;
            let (a, b) = row.params();
            let (entry, exit) = entry_exit(is_plus(body, he), a, b);
            let (theirs, ..) = half_edge_carrier(body, he).ok()?;
            Some((row.pcurve().clone(), entry, exit, theirs))
        };
        let record = half_edge_record(body, half_edge);
        let alone = record.next == half_edge;
        // A child placed again over its own span may sit whole turns from
        // the parent's image, so the joints the parent's ends made are
        // decided again against their neighbours' rows, as every joint
        // is; a restricted child keeps them.
        let before = (placed_again && !alone)
            .then(|| neighbour(record.prev))
            .flatten();
        let after = (placed_again && !alone)
            .then(|| neighbour(record.next))
            .flatten();
        let before_reach = || stored_reach(&wedges, body, record.prev);
        let after_reach = || stored_reach(&wedges, body, record.next);
        type Decision<'d> = Box<dyn Fn(Option<Winding>) -> Result<JointElement, PinMiss> + 'd>;
        // The joints the split makes or decides again, in the loop's
        // order from the one into the fed child: `Ok` a decision, `Err`
        // the element a joint the split leaves alone stores.
        let mut made: Vec<Result<Decision<'_>, Option<JointElement>>> = Vec::new();
        // The joint into the fed child: on a one-edge loop the split's
        // closing joint, out of the other child.
        if alone && placed_again {
            made.push(Ok(Box::new(|rest| {
                pin(fed, out.eval(out_half.exit), out_half, fed_half, rest)
            })));
        } else if let Some((image, entry, exit, theirs)) = &before {
            made.push(Ok(Box::new(move |rest| {
                let theirs = JointHalf {
                    carrier: theirs,
                    entry: *entry,
                    exit: *exit,
                    plus: is_plus(body, record.prev),
                    edge: stored_edge(body, record.prev),
                    reach: &before_reach,
                };
                pin(fed, image.eval(*exit), theirs, fed_half, rest)
            })));
        } else {
            made.push(Err(body.joint(half_edge)));
        }
        // The children's own joint, at `t`: decided there as every joint
        // is, which reads whether the split point is on the chart's
        // singular set.
        made.push(Ok(Box::new(|rest| {
            pin(out, fed.eval(t), fed_half, out_half, rest)
        })));
        // The joint out of the other child, into the neighbour after.
        if !alone {
            if let Some((image, entry, exit, theirs)) = &after {
                made.push(Ok(Box::new(move |rest| {
                    let theirs = JointHalf {
                        carrier: theirs,
                        entry: *entry,
                        exit: *exit,
                        plus: is_plus(body, record.next),
                        edge: stored_edge(body, record.next),
                        reach: &after_reach,
                    };
                    pin(image, out.eval(out_half.exit), out_half, theirs, rest)
                })));
            } else {
                made.push(Err(body.joint(record.next)));
            }
        }
        // The loop's other joints, as stored, after those.
        let others: Option<Vec<JointElement>> = if alone {
            Some(Vec::new())
        } else {
            let cycle = loop_rows(body, record.parent_loop);
            let n = cycle.len();
            let at = cycle
                .iter()
                .position(|&he| he == half_edge)
                .unwrap_or_else(|| unreachable!("{half_edge:?} is a member of its own loop"));
            (2..n).map(|k| body.joint(cycle[(at + k) % n])).collect()
        };
        let mut elements: Vec<Result<Option<JointElement>, PinMiss>> = made
            .iter()
            .map(|joint| match joint {
                Ok(decide) => decide(None).map(Some),
                Err(kept) => Ok(*kept),
            })
            .collect();
        // Two distinct edges leaving a singular point along one
        // direction: the one such joint is decided with the rest of the
        // loop, every other joint of it known ([`singular_turn`]).
        let slits: Vec<usize> = elements
            .iter()
            .enumerate()
            .filter(|(_, e)| matches!(e, Err(PinMiss::Singular(SingularMiss::EdgesCoincide))))
            .map(|(i, _)| i)
            .collect();
        if let ([slit], Some(others)) = (&slits[..], &others) {
            let m = elements.len();
            let known = |i: usize| elements[i].as_ref().ok().copied().flatten();
            let after: Option<Vec<JointElement>> = (slit + 1..m).map(known).collect();
            let ahead: Option<Vec<JointElement>> = (0..*slit).map(known).collect();
            if let (Some(after), Some(ahead), Ok(decide)) = (after, ahead, &made[*slit]) {
                let rest =
                    Winding::of(after.into_iter().chain(others.iter().copied()).chain(ahead));
                elements[*slit] = decide(Some(rest)).map(Some);
            }
        }
        drop(made);
        let decided: Vec<Option<JointElement>> = elements
            .into_iter()
            .collect::<Result<_, _>>()
            .map_err(missed)?;
        // What the split writes: the children's joint, and each end
        // joint it decided again. On a one-edge loop the closing joint
        // lands on the parent half on a plus half, where it is both the
        // element into the fed child and the one out of the other, and
        // on the new half alone on a minus one.
        let joint = decided[1].unwrap_or_else(|| unreachable!("the children's joint is decided"));
        let entering = if (alone && placed_again) || before.is_some() {
            decided[0]
        } else {
            None
        };
        let leaving = if alone {
            entering.filter(|_| plus)
        } else if after.is_some() {
            decided[2]
        } else {
            None
        };
        rows[slot] = Some(CarriedRows {
            parent_half: certify(t0, t, first)?,
            new_half: certify(t, t1, second)?,
            joint,
            entering,
            leaving,
        });
    }
    Ok(rows)
}

/// A site half's carrier, its interval, whether the loop traverses it
/// forward, and the point of the vertex it enters at ([`site_rows`]).
type SiteTraversal<T> = (geom::Curve3<T>, T, T, bool, geom_core::Point3<T>);

/// One half-edge of a face as a site mint's plan phase names it,
/// before its door mutates: an Euler operator's two new half-edges have
/// no key yet, and a null edge's two halves have no carrier yet.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum SiteHalf {
    /// A half-edge the body already holds, under its edge's carrier.
    Existing(HalfEdgeKey),
    /// The new edge's `he_plus`, which the surgery mints.
    NewPlus,
    /// The new edge's `he_minus`, which the surgery mints.
    NewMinus,
    /// A half-edge the body already holds, of an edge being described:
    /// the walk reads it under the carrier the description installs
    /// ([`SiteCarriers::Described`]).
    Described(HalfEdgeKey),
}

/// The carriers a site mint's plan reads for the halves it names that
/// the body does not carry yet ([`SiteHalf`]): no door both mints an
/// edge and describes one.
pub(crate) enum SiteCarriers<'a, T: Real> {
    /// A door that names existing halves alone.
    Existing,
    /// An Euler operator's new edge, for [`SiteHalf::NewPlus`] and
    /// [`SiteHalf::NewMinus`].
    New(&'a geom_brep::EdgeCurve<T>),
    /// The curves a description installs, by edge, for every
    /// [`SiteHalf::Described`] half of those edges.
    Described(&'a [(EdgeKey, &'a geom_brep::EdgeCurve<T>)]),
}

impl<T: Real> Clone for SiteCarriers<'_, T> {
    fn clone(&self) -> Self {
        *self
    }
}

impl<T: Real> Copy for SiteCarriers<'_, T> {}

/// Why a site mint refused to write a face's rows — [`site_rows`]'
/// refusal, raised before its door mutates anything, so the body is
/// untouched.
#[derive(Clone, Debug, PartialEq)]
pub enum SiteRowRefusal {
    /// **The fitted frontier.** The face is on a SPLINE chart (a
    /// described NURBS or an approximating surface), where a chart
    /// image derives from the edge's description through the iso and
    /// general lanes (`nurbs_iso_derive`), which read their fitted
    /// door through [`crate::AtRestPolicy`]. The Euler operators are
    /// generic over `Decide`, so they refuse here rather than leave a
    /// COMPLETE face half-minted. A face a null edge holds open is
    /// already incomplete, and is left as found instead
    /// ([`site_rows`]).
    SplineChart,
    /// **A keys-only door owes the face a row it holds no band to
    /// derive.** The door moves a loop or run onto a face whose rows
    /// are complete on an analytic chart, and the moved rows do not
    /// stand there — stated in another chart, or missing — so the face
    /// would leave half-minted. Or a null edge's kill releases a loop,
    /// which would leave missing the rows it missed while the edge held
    /// it open ([`releases_a_gap`]). The keys-only kills and moves
    /// (`kef`, `kemr`, `kev`, `kfmrh`, `ring_move`, `mfkrh`) take no
    /// band and refuse here, before mutating; their band siblings
    /// (`_minting`, or `kev_describing`) take one and re-mint the face
    /// ([`site_rows_owed`]). `kev` refuses here too where its unsplice
    /// would bridge a joint through a killed half's turn, which only a
    /// band decides.
    KeysOnly,
}

impl core::fmt::Display for SiteRowRefusal {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::SplineChart => write!(
                f,
                "the face is on a spline chart, where a new edge's pcurve row derives only \
                 through the fitted lane, which the Euler operators do not carry. Recourse: \
                 run the operator before the face is minted, and mint once the surgery is done"
            ),
            Self::KeysOnly => write!(
                f,
                "the door would leave a face half-minted: the rows it moves do not stand \
                 on a complete face, the joint it bridges needs an element only a band \
                 decides, or the loop it releases from its last null edge misses rows; this \
                 door takes no band. Recourse: call the door's band-taking sibling \
                 (`_minting`, or `kev_describing`) with the run's tolerance, or edit the \
                 loop before the face is minted"
            ),
        }
    }
}

impl std::error::Error for SiteRowRefusal {}

/// One loop of a face after a site mint's door, as its plan phase
/// knows it before mutating.
pub(crate) enum SiteLoop {
    /// A loop the surgery rewires: its half-edges in `next` order from
    /// the loop's `first` AFTER the surgery — the order the minting
    /// walk reads it in.
    Rewired(Vec<SiteHalf>),
    /// A loop of the face the surgery leaves as it is.
    Kept(LoopKey),
}

/// A face a site mint re-mints — one an Euler operator's new
/// half-edges land on, one a null edge's halves are on at its
/// description, one a door moves a loop or run onto, or one a null
/// edge's kill releases a loop of — described as its door leaves it.
pub(crate) struct SiteFace<T: Real> {
    /// The face whose rows decide whether this one is minted: the face
    /// itself, or — for a face a door makes (`mef`'s new face,
    /// `mfkrh`'s) — the face it is carved or promoted from. Named by
    /// the refusal.
    pub(crate) rows_from: FaceKey,
    /// The chart the face is on after the surgery.
    pub(crate) surface: Surface<T>,
    /// The face's sense after the surgery: the side of its loops the
    /// joints on its chart's singular set are decided on.
    pub(crate) sense: bool,
    /// Whether a door moves a loop or run onto this face whose rows do
    /// not stand on it: stated in another chart, or missing
    /// ([`crate::Body::drop_rows`]). Such a loop is a rewired loop like
    /// any other, and on an analytic chart it is minted like one; on a
    /// spline chart the face is left as found rather than refused
    /// ([`site_rows`] says why).
    pub(crate) moved: bool,
    /// Its loops after the surgery, outer first.
    pub(crate) loops: Vec<SiteLoop>,
}

/// What a site mint writes into the pcurve map for one face once its
/// door has mutated ([`apply_site_rows`]).
pub(crate) enum SiteRows<T: Real> {
    /// The face is not re-minted — [`StoredRows::remints`] did not
    /// select it, its chart mints nothing, it is on a spline chart with
    /// a null edge holding it open or a loop or run moved onto it, or
    /// every loop the door rewires still runs through a null edge — and
    /// the door leaves its rows exactly as found.
    Leave,
    /// What the door creates on the loops it rewires that run through
    /// no null edge: the image of every half it adds, describes, moves
    /// or finds rowless, and the element of every joint it makes or
    /// finds missing. Everything else those loops hold, the loops it
    /// keeps, and a rewired loop a null edge still holds open keep what
    /// they have.
    Mint {
        /// The certified images, by half.
        images: Vec<(SiteHalf, PcurveCache<T>)>,
        /// The joint elements, by the half each joint enters.
        joints: Vec<(SiteHalf, JointElement)>,
    },
    /// The face as the surgery leaves it has no closed-form row set
    /// that certifies, so it stores nothing ([`site_rows`] says when).
    /// Every half-edge of the face after the surgery.
    Clear(Vec<SiteHalf>),
}

/// A face as a site mint found it, and read further
/// ([`site_rows_from`]).
pub(crate) struct SiteFrom {
    /// Its rows as found.
    pub(crate) rows: StoredRows,
    /// Its loops a null edge holds open ([`StoredRows::open_loops`]).
    pub(crate) open: Vec<LoopKey>,
}

/// `face` as found, when a site mint's door could re-mint it: its
/// chart mints ([`chart_mints`]) and [`StoredRows::remints`] selects it
/// were the door to release every loop a null edge holds open, the most
/// a door can do. `None` for every other face, which the door leaves as
/// found; [`site_rows`] asks the predicate again with what the door
/// does.
///
/// The chart is read first, so a face on a chart that mints nothing
/// costs no walk.
///
/// # Panics
///
/// Where a loop of `face`, a face of the body, does not resolve or walk
/// as its own ([`crate::live::FaceView::cycles`]), or a half's edge or curve does
/// not resolve ([`held_open`]).
///
/// [`chart_mints`]: super::chart_mints
#[track_caller]
pub(crate) fn site_rows_from<T: Decide>(
    body: &Body<T>,
    face: FaceKey,
    surface: &Surface<T>,
) -> Option<SiteFrom> {
    DescribedChart::minting(surface)?;
    let rows = stored_rows(body, face);
    let open = rows.open_loops(body);
    rows.remints(&open, !open.is_empty())
        .then_some(SiteFrom { rows, open })
}

/// **The loops a site mint walks on `face`**, band-free: the part of
/// [`site_rows`] that decides before it derives, and the one reading
/// of it. Empty where the face is left as found ([`SiteRows::Leave`]):
/// its chart mints nothing, [`StoredRows::remints`] does not select it,
/// or every loop the door rewires still runs through a null edge.
/// Otherwise each rewired loop no null edge holds open as the door
/// leaves it, in `face`'s loop order; [`site_rows`] walks exactly
/// these and writes the face — rows, or a clear.
///
/// # Errors
///
/// [`SiteRowRefusal::SplineChart`] on a complete spline face a door
/// adds half-edges to.
#[track_caller]
fn site_walks<'a, T: Decide>(
    body: &Body<T>,
    chart: DescribedChart<'_, T>,
    face: &'a SiteFace<T>,
    from: &SiteFrom,
) -> Result<Vec<&'a [SiteHalf]>, SiteRowRefusal> {
    // A spline chart's rows derive through the fitted lane, which a
    // `Decide` door does not hold, so no loop of it is minted here and
    // the question is per face, not per loop: refuse, or leave as
    // found. A face a null edge holds open anywhere is left as found,
    // and an operator on one of its complete loops leaves that loop's
    // new halves rowless too, by intent: the face is already
    // incomplete, and a refusal would strand the pipeline mid-surgery
    // with its null edge, which tier 2 refuses at rest. A face a door
    // moves a loop or run onto is left as found too, its moved rows
    // dropped: the doors that move a loop are the ones that fuse and
    // merge bodies that arrive minted (a boolean's seam zip, the merge
    // door, a blend's kills), which have no "move before minting" to
    // take as a refusal's recourse. A complete face the door adds
    // half-edges to refuses rather than go half-minted.
    if chart.surface().spline_chart().is_some() {
        return if from.open.is_empty() && !face.moved {
            Err(SiteRowRefusal::SplineChart)
        } else {
            Ok(Vec::new())
        };
    }
    let mut open_after = Vec::with_capacity(face.loops.len());
    for lp in &face.loops {
        open_after.push(match lp {
            SiteLoop::Rewired(halves) => held_open(
                body,
                halves.iter().filter_map(|&at| match at {
                    SiteHalf::Existing(he) => Some(he),
                    SiteHalf::NewPlus | SiteHalf::NewMinus | SiteHalf::Described(_) => None,
                }),
            ),
            SiteLoop::Kept(key) => from.open.contains(key),
        });
    }
    let released = !from.open.is_empty() && !open_after.contains(&true);
    if !from.rows.remints(&from.open, released) {
        return Ok(Vec::new());
    }
    Ok(face
        .loops
        .iter()
        .zip(open_after)
        .filter_map(|(lp, open)| match lp {
            SiteLoop::Rewired(halves) if !open && !halves.is_empty() => Some(halves.as_slice()),
            SiteLoop::Rewired(_) | SiteLoop::Kept(_) => None,
        })
        .collect())
}

/// **Whether a site mint writes `face`**: [`site_walks`] walks a loop
/// of it. Band-free, for a keys-only door, which refuses
/// [`SiteRowRefusal::KeysOnly`] exactly where its `_minting` twin's
/// [`site_rows`] would write the face.
///
/// # Errors
///
/// [`site_walks`]'.
pub(crate) fn site_rows_owed<T: Decide>(
    body: &Body<T>,
    face: &SiteFace<T>,
    from: &SiteFrom,
) -> Result<bool, SiteRowRefusal> {
    let Some(chart) = DescribedChart::minting(&face.surface) else {
        return Ok(false);
    };
    Ok(!site_walks(body, chart, face, from)?.is_empty())
}

/// **Whether a null-edge kill leaves a loop it releases with a gap**:
/// a loop of `face`, as the kill leaves it, that runs through no null
/// edge and has a half-edge that [`misses_a_row`] — the gaps
/// [`StoredRows::gaps`] reads, taken on the loop the kill leaves.
/// `made` are the joints the kill writes, each keyed by the half-edge
/// it is into, with the element it writes there; every other joint
/// keeps its element. Band-free; the band kills' site mint runs only
/// where this holds ([`crate::Body::plan_released_rows`]).
///
/// A loop a null edge still holds open is not the kill's to release,
/// and its gaps do not count: the site mint cannot walk it.
///
/// # Panics
///
/// Where a half of `face`'s rewired loops, each one the kill read out
/// of the body's records, does not resolve.
#[track_caller]
pub(crate) fn releases_a_gap<T: Decide>(
    body: &Body<T>,
    face: &SiteFace<T>,
    made: &[(HalfEdgeKey, Option<JointElement>)],
) -> bool {
    let element = |he: HalfEdgeKey| {
        made.iter()
            .find(|&&(into, _)| into == he)
            .map_or(body.joint(he).is_some(), |(_, element)| element.is_some())
    };
    face.loops.iter().any(|lp| {
        let SiteLoop::Rewired(halves) = lp else {
            return false;
        };
        let cycle: Vec<HalfEdgeKey> = halves
            .iter()
            .filter_map(|&at| match at {
                SiteHalf::Existing(he) => Some(he),
                SiteHalf::NewPlus | SiteHalf::NewMinus | SiteHalf::Described(_) => None,
            })
            .collect();
        if held_open(body, cycle.iter().copied()) {
            return false;
        }
        let n = cycle.len();
        (0..n).any(|i| {
            let (before, he) = (cycle[(i + n - 1) % n], cycle[i]);
            misses_a_row(body, before, he, element(he))
        })
    })
}

/// **The rows a site mint writes onto one face**, derived before its
/// door mutates: a face an Euler operator adds half-edges to, one a
/// null edge's halves are on at its first description
/// ([`crate::Body::set_edge_curve`](crate::construction::Construct::set_edge_curve)), one a door moves a loop or run
/// onto whose rows do not stand there ([`crate::Body::drop_rows`] names
/// the doors), or one a band kill releases a loop of with a gap
/// ([`crate::Body::plan_released_rows`]). `from` is the face as found, on a face
/// [`site_rows_from`] read further; every other face is left as found, and never reaches
/// here. The face is re-minted where [`StoredRows::remints`] selects
/// it, `released` being whether the door takes the last null edge off
/// it: the face as found runs through one and the face as the door
/// leaves it through none.
///
/// - **On the loops the surgery rewires, it mints what the surgery
///   creates**, exactly as [`mint_pcurves_of`] would after the surgery:
///   the same walk from each loop's `first` ([`walk_cycle`]) and the
///   same certification ([`certify_walked`]), over the image of every
///   half the surgery adds, describes or finds rowless, derived by the
///   closed form ([`chart_pcurve_over`]), and the element of every joint it
///   makes or finds missing, decided between the two images
///   ([`decide_joint`]); the loop must still close ([`Winding`]).
///   Every other image and element stands, and the walk reads it as
///   stored: an image is a function of its edge and the chart, an
///   element of its two images, and the surgery changes none of them.
///   A loop the surgery keeps keeps its rows. A row's certificate reads
///   that row alone, so on a face whose stored rows are the pass's the
///   rows after the op are the pass's, byte for byte.
/// - **A rewired loop that still runs through a null edge keeps the
///   rows it has** ([`held_open`]): the null halves have no carrier,
///   so the loop cannot be walked, and its new halves go rowless until
///   the null edge leaves it. Every rewired loop the door leaves
///   running through no null edge is walked, and the rows it missed
///   while it was held open are minted with the rest of what is
///   missing.
/// - **A moved loop or run is a rewired loop** ([`SiteFace::moved`]):
///   its rows are stated in the chart it left, or missing, so it is
///   walked whole in the destination's chart, exactly as the minting
///   pass would walk it there. Which face's rows decide is the door's
///   (`rows_from`): the destination as found where one receives the
///   loop, and the face a new one is carved or promoted from.
/// - **On a spline chart the site mint refuses a COMPLETE face**
///   ([`SiteRowRefusal::SplineChart`]) and leaves a face a null edge
///   holds open anywhere, or one a door moves a loop or run onto, as
///   found (the arm below says why per face).
///   The mint of an ANALYTIC chart
///   needs nothing the fitted lane holds — [`chart_pcurve_over`],
///   [`walk_cycle`] and [`PcurveCache::certify`] are all `Decide` — and
///   the fitted lane is the spline chart's derivation
///   (`nurbs_iso_derive`) and U2's `General` certificate.
/// - **Where the face as the surgery leaves it has no closed-form row
///   set, it stores nothing** ([`SiteRows::Clear`]): a carrier outside
///   the chart's closed-form classes, a certification that refuses, a
///   branch that meets no neighbour, a loop that does not close. The
///   face is unminted, never half-minted. Where the minting pass would
///   RAISE on that face, the operator does not: it runs mid-surgery, on
///   states a later door finishes describing (C4: a door may drop rows
///   mid-surgery), and a producer's final pass is where such a face is
///   minted or refused. A body left at rest without that pass reads
///   loud at tier 3, which re-derives the rowless face and reports the
///   refusal the mint would raise ([`validate_pcurves`]).
///
/// **Cost.** One derivation and certification per image the surgery
/// creates and one joint decision per joint it makes, whatever the
/// loop's length; a moved loop's images are all derived. Beside them,
/// reads: per half-edge of the rewired loops a presence read and its
/// element (for the winding), and per half-edge of the rest of the
/// face one presence read ([`site_rows_from`]); on a face missing a row, three lookups
/// more per half-edge of the face as found — its half, edge and curve —
/// for which loops a null edge holds open, and as many again per
/// half-edge of the rewired loops.
///
/// # Errors
///
/// [`SiteRowRefusal::SplineChart`]: the spline frontier.
///
/// # Panics
///
/// Where a record a walked half reads does not resolve: every half the
/// plan names is one the door resolved or read out of a record.
///
/// [`mint_pcurves_of`]: super::mint_pcurves_of
/// [`validate_pcurves`]: super::validate_pcurves
#[track_caller]
pub(crate) fn site_rows<T: Decide>(
    body: &Body<T>,
    face: &SiteFace<T>,
    from: &SiteFrom,
    curves: SiteCarriers<'_, T>,
    band: Band,
) -> Result<SiteRows<T>, SiteRowRefusal> {
    let Some(chart) = DescribedChart::minting(&face.surface) else {
        return Ok(SiteRows::Leave);
    };
    let walks = site_walks(body, chart, face, from)?;
    if walks.is_empty() {
        return Ok(SiteRows::Leave);
    }
    let kept_halves = |key: LoopKey| {
        from.rows
            .loops
            .iter()
            .find(|(lk, _)| *lk == key)
            .map(|(_, cycle)| cycle.as_slice())
            .unwrap_or_default()
            .iter()
            .copied()
            .map(SiteHalf::Existing)
    };
    let every_half = || -> Vec<SiteHalf> {
        face.loops
            .iter()
            .flat_map(|lp| -> Vec<SiteHalf> {
                match lp {
                    SiteLoop::Rewired(halves) => halves.clone(),
                    SiteLoop::Kept(key) => kept_halves(*key).collect(),
                }
            })
            .collect()
    };
    // A half the body holds enters at its start vertex. A new half's
    // vertex may not exist before the surgery, so it enters at its
    // carrier's end, which the edge certificate pins to the vertex the
    // surgery makes (`carrier_endpoint_start` / `_end`).
    let traversal = |at: SiteHalf| -> SiteTraversal<T> {
        match at {
            SiteHalf::Existing(he) => {
                let (carrier, t0, t1) = half_edge_carrier(body, he).unwrap_or_else(|e| {
                    unreachable!(
                        "site_walks walks only a loop no null edge holds open (held_open): {e}"
                    )
                });
                (carrier, t0, t1, is_plus(body, he), entry_vertex(body, he))
            }
            SiteHalf::NewPlus | SiteHalf::NewMinus => {
                let SiteCarriers::New(edge) = curves else {
                    unreachable!(
                        "site_rows: a plan names a new half only beside the edge that carries it"
                    )
                };
                let (t0, t1) = edge.params();
                let plus = at == SiteHalf::NewPlus;
                let carrier = edge.carrier().clone();
                let vertex = carrier.eval(if plus { t0 } else { t1 });
                (carrier, t0, t1, plus, vertex)
            }
            SiteHalf::Described(he) => {
                let of = half_edge_record(body, he).edge;
                let SiteCarriers::Described(described) = curves else {
                    unreachable!(
                        "site_rows: a Described half under non-Described carriers; \
                         `Body::description_rows` is the one constructor of \
                         `SiteHalf::Described`, and it plans under \
                         `SiteCarriers::Described` alone"
                    )
                };
                let Some(&(_, edge)) = described.iter().find(|(e, _)| *e == of) else {
                    unreachable!(
                        "site_rows: a Described half whose edge the carriers do not list; \
                         `Body::description_rows` marks a half Described only when it is \
                         a half of one of `described`'s edges, and passes that same slice as \
                         the carriers"
                    )
                };
                let (t0, t1) = edge.params();
                let plus = is_plus(body, he);
                (edge.carrier().clone(), t0, t1, plus, entry_vertex(body, he))
            }
        }
    };
    // A half the door keeps, on a face whose images stand, keeps its
    // image: an image is a function of its edge and the chart, and the
    // door changes neither. A moved loop's images are stated in the chart
    // it left, or missing, and are all derived.
    let standing = |at: SiteHalf| match at {
        SiteHalf::Existing(he) if !face.moved => body.pcurve(he),
        SiteHalf::Existing(_) | SiteHalf::NewPlus | SiteHalf::NewMinus | SiteHalf::Described(_) => {
            None
        }
    };
    let mut derived: Vec<Walked<T, SiteHalf>> = Vec::new();
    let mut joints: Vec<(SiteHalf, JointElement)> = Vec::new();
    for halves in walks {
        let n = halves.len();
        let stands: Vec<Option<&PcurveCache<T>>> = halves.iter().map(|&at| standing(at)).collect();
        // The element of each joint the door leaves as found — between
        // two images that stand, linked as they were — stands too; every
        // other joint is decided.
        let keep: Vec<Option<JointElement>> = (0..n)
            .map(|i| {
                let p = (i + n - 1) % n;
                match (halves[p], halves[i]) {
                    (SiteHalf::Existing(before), SiteHalf::Existing(he))
                        if stands[p].is_some()
                            && stands[i].is_some()
                            && half_edge_record(body, he).prev == before =>
                    {
                        body.joint(he)
                    }
                    _ => None,
                }
            })
            .collect();
        let item = |i: usize| -> Result<WalkItem<T, SiteImage<'_, T>>, PcurveCertifyError> {
            match (stands[i], halves[i]) {
                (Some(cache), SiteHalf::Existing(he)) => {
                    debug_assert!(
                        !matches!(
                            geom_core::k_stats::detached(|| {
                                let (carrier, ..) = traversal(SiteHalf::Existing(he));
                                row_interval(body, he, cache, &carrier, band)
                            })
                            .0,
                            Err(PcurveMintError::RowInterval { .. })
                        ),
                        "site_rows: the image kept for {he:?} spans an interval its edge does not"
                    );
                    let (t0, t1) = cache.params();
                    let (carrier, ..) = traversal(SiteHalf::Existing(he));
                    Ok(WalkItem {
                        image: SiteImage::Kept(cache.pcurve(), carrier),
                        t0,
                        t1,
                        plus: is_plus(body, he),
                        vertex: entry_vertex(body, he),
                        edge: stored_edge(body, he),
                    })
                }
                (_, at) => {
                    let (carrier, t0, t1, plus, vertex) = traversal(at);
                    let pcurve = chart_pcurve_over(&carrier, t0, t1, &face.surface, band)?;
                    Ok(WalkItem {
                        image: SiteImage::Derived((carrier, ChartImage::Image { pcurve, t0, t1 })),
                        t0,
                        t1,
                        plus,
                        vertex,
                        edge: match at {
                            SiteHalf::Existing(he) | SiteHalf::Described(he) => {
                                stored_edge(body, he)
                            }
                            SiteHalf::NewPlus | SiteHalf::NewMinus => WalkEdge::New,
                        },
                    })
                }
            }
        };
        // The closed-form derivation refusing, a branch meeting no
        // neighbour, or a loop that does not close: the face is cleared.
        // Each half's reach is read over the face the door mints, as the
        // door leaves it and alone ([`edge_reach`]).
        let reach = |i: usize| {
            let (carrier, t0, t1, ..) = traversal(halves[i]);
            Some(edge_reach(
                &carrier,
                (t0, t1),
                every_half().into_iter().map(|at| traversal(at).4),
            ))
        };
        let Ok(pinned) = walk_cycle(chart, face.sense, n, item, |i| keep[i], &reach, band, true)
        else {
            return Ok(SiteRows::Clear(every_half()));
        };
        for (i, (image, .., element)) in pinned.into_iter().enumerate() {
            if keep[i].is_none() {
                let element = element.unwrap_or_else(|| {
                    unreachable!("walk_cycle decides every joint of a loop it closes")
                });
                joints.push((halves[i], element));
            }
            if let SiteImage::Derived(row) = image {
                derived.push(Walked::pinned(halves[i], row, None));
            }
        }
    }
    match certify_walked(derived, &face.surface, band, None) {
        Ok(rows) => Ok(SiteRows::Mint {
            images: rows.into_iter().map(|(at, cache, _)| (at, cache)).collect(),
            joints,
        }),
        Err(_) => Ok(SiteRows::Clear(every_half())),
    }
}

/// Writes what [`site_rows`] decided, once its door has mutated.
/// `minted` is the `(he_plus, he_minus)` an Euler operator's surgery
/// minted for [`SiteHalf::NewPlus`] and [`SiteHalf::NewMinus`], and
/// `None` for a description, whose plan names none. Infallible: it
/// sits in the door's mutation phase and only writes the map.
pub(crate) fn apply_site_rows<T: Decide>(
    body: &mut Body<T>,
    plans: Vec<SiteRows<T>>,
    minted: Option<(HalfEdgeKey, HalfEdgeKey)>,
) {
    let key = |at: SiteHalf| match (at, minted) {
        (SiteHalf::Existing(he) | SiteHalf::Described(he), _) => he,
        (SiteHalf::NewPlus, Some((he_plus, _))) => he_plus,
        (SiteHalf::NewMinus, Some((_, he_minus))) => he_minus,
        (SiteHalf::NewPlus | SiteHalf::NewMinus, None) => unreachable!(
            "apply_site_rows: only an Euler operator's plan names a new half, and it passes \
             the halves its surgery minted"
        ),
    };
    for plan in plans {
        match plan {
            SiteRows::Leave => {}
            SiteRows::Mint { images, joints } => {
                for (at, cache) in images {
                    body.attach_pcurve(key(at), cache);
                }
                for (at, element) in joints {
                    body.write_joint(key(at), Some(element));
                }
            }
            SiteRows::Clear(halves) => body.drop_rows(halves.into_iter().map(key)),
        }
    }
}

/// **A killed half's turn**: the element carrying its image's exit onto
/// its entry, where the two coincide in space — a closed carrier, whose
/// two vertices hold one point. A kill that crosses the half whole sums
/// it with the elements either side ([`crate::Body::kev_describing`](crate::construction::Construct::kev_describing)).
/// `Ok(None)` where the half stores no image or its ends do not meet.
///
/// Whether the ends meet is decided in 3-D, as the distance between the
/// edge's two vertices (`pcurve_turn_closes`): the joint decision
/// ([`decide_joint`]) takes the coincidence of the two points it joins as
/// given, so it is not asked until the coincidence is decided. Where they
/// meet, the turn is decided at that point as every joint is.
///
/// # Errors
///
/// Whether the ends meet escalated at `band` (`Some`, its cause), or
/// they meet and the turn's joint decision there missed: it escalated
/// (`Some`, a near-pole offset in band included), or decided a miss
/// (`None`) — the gap on a mark and decided off every orbit point the
/// near-pole read tries at the vertex's lever. The ends are one point,
/// so neither is "no turn"; a kill writes no element through any of
/// them.
///
/// # Panics
///
/// Where `half_edge`'s face, surface, edge or vertices do not resolve.
#[track_caller]
pub(crate) fn turn_element<T: Decide>(
    body: &Body<T>,
    half_edge: HalfEdgeKey,
    band: Band,
) -> Result<Option<JointElement>, Option<Indeterminate>> {
    let Some(row) = body.pcurve(half_edge) else {
        return Ok(None);
    };
    let surface = half_edge_surface(body, half_edge);
    let Some(chart) = DescribedChart::of(&surface) else {
        return Ok(None);
    };
    let (start, end) = edge_vertex_points(body, half_edge);
    match decide("pcurve_turn_closes", Margin::of(start.distance(end)), band) {
        Ok(Sign::Zero) => {}
        Ok(Sign::Positive | Sign::Negative) => return Ok(None),
        Err(cause) => return Err(Some(cause)),
    }
    let (t0, t1) = row.params();
    let (entry_t, exit_t) = entry_exit(is_plus(body, half_edge), t0, t1);
    let image = row.pcurve();
    let (carrier, ..) = half_edge_carrier(body, half_edge)
        .unwrap_or_else(|e| unreachable!("{half_edge:?} stores a row, so its edge a carrier: {e}"));
    let wedges = crate::wedge_reach::WedgeReach::new(body);
    let reach = || stored_reach(&wedges, body, half_edge);
    // The half against its direction: the joint carries its exit onto
    // its entry, so the edge arrives at the point by its entry end and
    // leaves it by its exit end.
    let own = JointHalf {
        carrier: &carrier,
        entry: exit_t,
        exit: entry_t,
        plus: !is_plus(body, half_edge),
        edge: stored_edge(body, half_edge),
        reach: &reach,
    };
    match decide_joint(
        chart,
        image,
        exit_t,
        image.eval(entry_t),
        entry_vertex(body, half_edge),
        ChartPeriods::of(&surface, band),
        band,
        // One edge, traversed the one way either side of the point, and
        // no loop's joint: there is no rest of a loop to read a
        // coincidence by, and one edge leaving a point along the
        // direction it arrived by is a cusp.
        JointSides::of_halves(half_edge_face(body, half_edge).sense(), own, own, None),
    ) {
        Ok(element) => Ok(Some(element)),
        Err(PinMiss::Escalated(cause) | PinMiss::NetStretchUndecided(cause)) => Err(Some(cause)),
        Err(PinMiss::Discontinuity | PinMiss::Singular(_)) => Err(None),
    }
}
