//! **The mint**: the pass's refusal, the chart a row is minted on and
//! its periods, a row's derivation, and the minting doors with the walk
//! that drives them.

use geom::Surface;
use geom_brep::{
    CertifyingDoor, ChartImage, Pcurve, PcurveCache, PcurveCertifyError, Period, chart_pcurve_over,
    probe_pick,
};
use geom_core::k_stats::decide;
use geom_core::predicate::{Band, BandError};
use geom_core::{Decide, Indeterminate, Margin, Point2, Real, Sign, SupSpeed, Tol};

use super::{
    DerivedRow, PinMiss, SingularMiss, WalkFail, WalkItem, Walked, loop_rows, stored_edge,
    stored_reach, walk_cycle, walk_item,
};
use crate::body::Body;
use crate::entity::{EdgeKey, EntityId, FaceKey, HalfEdge, HalfEdgeKey, LoopKey};
use crate::joint::JointElement;
use crate::live::{FaceView, linked, proven};
use crate::props::AtRestPolicy;

/// Typed refusal of the pcurve minting pass (D4 ¶3).
#[derive(Clone, Debug, PartialEq)]
// The variant roster the sample-coverage row reads (test builds only).
#[cfg_attr(
    test,
    derive(strum::EnumDiscriminants),
    strum_discriminants(name(PcurveMintErrorKind), vis(pub(crate)), derive(strum::EnumIter))
)]
pub enum PcurveMintError {
    /// A key the caller passed does not resolve in this body: `role`
    /// names the argument. Checked at the door, before any work.
    Stale {
        /// The argument the key was passed as.
        role: &'static str,
        /// The key.
        key: EntityId,
    },
    /// `half_edge`'s edge is null scaffolding
    /// ([`crate::null::CurveGeom::NullScaffold`], minted by
    /// [`crate::Body::mev_null`](crate::construction::Construct::mev_null)): it has no carrier yet, so there is
    /// no chart image to derive over it. Scaffolding is a legal state
    /// mid-surgery; tier 2 refuses it at rest.
    NoCarrier {
        /// The half-edge whose edge has no carrier.
        half_edge: HalfEdgeKey,
    },
    /// A half-edge's pcurve failed certification at mint.
    Certify {
        /// The half-edge whose cache was refused.
        half_edge: HalfEdgeKey,
        /// The typed certification failure, nested whole.
        error: PcurveCertifyError,
    },
    /// The chart image of a half-edge does not meet its predecessor's
    /// in the chart: the loop's one-branch unwrap is not continuous
    /// there. Typed, never patched by picking the nearest branch (the
    /// M2 PR 5 finding — module docs).
    LoopDiscontinuity {
        /// The half-edge whose entry point did not meet its
        /// predecessor's exit point.
        half_edge: HalfEdgeKey,
    },
    /// A loop's chart walk did not close: after one full traversal the
    /// azimuth advance is neither zero nor exactly one period, or the
    /// height does not return.
    LoopNotClosed {
        /// The face whose loop failed to close.
        face: FaceKey,
    },
    /// A joint of a loop sits off the chart's singular set, but so near
    /// the axis that the walk decided its azimuth integer with less room
    /// than the joint bound: half the step to the next point of the
    /// joint's orbit, as the chord it spans at the vertex's own distance
    /// from the axis less the ends' `2ε`, is not definitely past the
    /// band's escalation four times over (`loop_windings_joint_room`).
    /// There a mark may name an orbit point other than the joint's own
    /// (both lifts of the one 3-D point), so the loop's winding may read
    /// zero while the loop truly winds. Raised by [`loop_windings`] alone,
    /// on a sphere near a pole or a cone near its apex.
    ///
    /// [`loop_windings`]: super::loop_windings
    JointWithoutRoom {
        /// The face whose loop holds the joint.
        face: FaceKey,
        /// The loop.
        r#loop: LoopKey,
        /// The half-edge whose ENTRY is the joint.
        half_edge: HalfEdgeKey,
    },
    /// A half-edge carries no stored pcurve at rest although its face
    /// carries some: the face's cache set is incomplete, a defect. A
    /// face that stores none is [`PcurveMintError::Unminted`], or the
    /// refusal its derivation meets; a planar face stores none by
    /// construction, and at a scalar with no fitted door a face only
    /// that door can image stores none ([`not_owed`]).
    ///
    /// **On the output of `mev`, `mef` and `mekr`, and of the doors
    /// that move a loop or run onto a face (`kfmrh`, `ring_move`,
    /// `mfkrh`, `kef` and their `_minting` twins), this is a kernel-bug
    /// detector** for a face that was complete on an analytic chart:
    /// they leave such a face complete or rowless, or refuse
    /// ([`site_rows`], [`crate::Body::plan_moved_rows`]). The doors that can still
    /// produce this state are:
    ///
    /// - the moving doors onto a complete SPLINE face, where the moved
    ///   loop's rows are dropped or missing and the fitted lane that
    ///   could derive them is not theirs ([`crate::Body::drop_rows`]);
    /// - the moving doors carrying a loop's rows, across one chart,
    ///   onto a face that stored none of its own — or a rowless loop
    ///   onto a half-minted one — which leaves the rows they find;
    /// - [`crate::Body::mev_null`](crate::construction::Construct::mev_null), whose scaffolding edge has no
    ///   carrier to derive a row from. The loop it joins misses the
    ///   edge's two rows, and those of any half-edge an operator adds
    ///   to the loop meanwhile, while a null edge holds it open; the
    ///   door that releases the loop — an operator rewiring it out from
    ///   under the edge, as the pipelines' joins do, or the edge's first
    ///   description ([`crate::Body::set_edge_curve`](crate::construction::Construct::set_edge_curve), or a
    ///   [`crate::Body::kev_describing`](crate::construction::Construct::kev_describing) that lists it), or the band kill
    ///   of the edge ([`crate::Body::plan_released_rows`]) — mints it
    ///   whole, except on a spline chart;
    /// - a caller's own [`crate::Body::detach_pcurve`](crate::construction::Construct::detach_pcurve);
    ///
    /// and a face that arrives half-minted any of these ways, but for a
    /// loop a null edge holds open, stays so through the three
    /// operators, which leave it as found — unless the operator takes
    /// the face's last null edge off it, and then only its kept loops
    /// keep their gaps.
    ///
    /// [`site_rows`]: super::site_rows
    MissingCache {
        /// The half-edge with no stored cache.
        half_edge: HalfEdgeKey,
    },
    /// A face on a chart that mints stores none of its rows, and the
    /// minting pass derives and certifies every one of them: the body
    /// was not minted after it was built or edited. Rows are mandatory
    /// at rest (C4), so this is a producer's omission, or an assembly
    /// of Euler operations left without its closing mint.
    Unminted {
        /// The face that stores no row.
        face: FaceKey,
    },
    /// A stored row states a carrier interval that is not its edge's:
    /// every row is stated over exactly its edge's certified interval
    /// (the mint copies it, a split restricts it), so this row is about
    /// another edge — a row left from before a split or a carrier swap.
    RowInterval {
        /// The half-edge whose row states the wrong interval.
        half_edge: HalfEdgeKey,
    },
    /// A classification escalated (sliver band or poison).
    Escalated {
        /// The half-edge under classification.
        half_edge: HalfEdgeKey,
        /// The classifier's diagnostic.
        cause: Indeterminate,
    },
    /// The joint into a half-edge lies on a spline net closed in `v`
    /// whose whole `v` sweep moves a point neither definitely past the
    /// band nor definitely under it (`pcurve_loop_v_stretch`), so the
    /// joint's `v` periods are undecided and no element is written: a
    /// net whose `v` sections are within the band's sliver of collapsing
    /// to a curve. The same of a net closed in `u` whose `u` sweep is
    /// undecided (`pcurve_loop_pole_joint`).
    NetStretchUndecided {
        /// The half-edge whose ENTRY is the joint.
        half_edge: HalfEdgeKey,
        /// The gate's diagnostic.
        cause: Indeterminate,
    },
    /// The joint into a half-edge lies on its chart's singular set (a
    /// sphere's pole, a cone's apex, a spindle sheet's apex) and the turn
    /// there on the face's side is decided by nothing, so no element is
    /// written.
    SingularTurn {
        /// The half-edge whose ENTRY is the joint.
        half_edge: HalfEdgeKey,
        /// Why no turn is decided.
        miss: SingularMiss,
    },
    /// The run's linear band could not be built.
    Band(BandError),
    /// The chart [`loop_windings`] was handed is the placeholder
    /// ([`Surface::is_placeholder_chart`]): it has no description yet,
    /// so there is no lever arm to meter a joint gap through and no
    /// region of it for a loop to bound. Refused before any loop is
    /// walked, so there is no half-edge to carry a
    /// [`PcurveMintError::Certify`] with
    /// [`PcurveCertifyError::PlaceholderChart`] — the same fact, refused
    /// per half-edge by the certification lanes. Never produced at rest:
    /// the minting pass and the validator skip a placeholder face.
    ///
    /// [`loop_windings`]: super::loop_windings
    PlaceholderChart {
        /// The face whose boundary was asked for.
        face: FaceKey,
    },
}

impl PcurveMintError {
    /// **The ending this refusal gives at `at`**, for a door that reports
    /// it under its own sentence (the shell op's closing mint): a nested
    /// certification refusal's decision's ([`PcurveCertifyError::ending`]);
    /// an undecided reading's "not yet", as the at-rest table ends it
    /// (`validate::classify_pcurve`); and the lever of a refusal a lever
    /// answers (a joint too near a pole, a band the tolerance cannot form,
    /// a placeholder chart). `None` for a torn arm (a key that does not
    /// resolve, a null carrier, a loop that does not meet or close, a row
    /// missing, misstated or unminted): on a body the kernel built, the
    /// door's defect.
    #[must_use]
    pub fn ending(&self, at: impl Into<geom_brep::recourse::ReadAt>) -> Option<String> {
        let at = at.into();
        match self {
            Self::Certify { error, .. } => Some(error.ending(at)),
            Self::Escalated { cause, .. } | Self::NetStretchUndecided { cause, .. } => Some(
                geom_brep::recourse::not_yet(geom_brep::recourse::RefusedArm::Undecided(cause), at),
            ),
            Self::JointWithoutRoom { .. } => Some(JOINT_WITHOUT_ROOM_RECOURSE.to_owned()),
            Self::SingularTurn { .. } => Some(SINGULAR_TURN_RECOURSE.to_owned()),
            Self::Band(_) => Some(crate::validate::TOLERANCE.to_owned()),
            Self::PlaceholderChart { .. } => Some(PLACEHOLDER_RECOURSE.to_owned()),
            Self::Stale { .. }
            | Self::NoCarrier { .. }
            | Self::LoopDiscontinuity { .. }
            | Self::LoopNotClosed { .. }
            | Self::MissingCache { .. }
            | Self::Unminted { .. }
            | Self::RowInterval { .. } => None,
        }
    }

    /// This refusal's text, a nested certification refusal ended at `at`
    /// ([`PcurveCertifyError::render`]) and its payload alone where `at`
    /// is `None`. Every other arm reads alike at every door.
    fn write(
        &self,
        f: &mut core::fmt::Formatter<'_>,
        at: Option<geom_brep::recourse::ReadAt>,
    ) -> core::fmt::Result {
        match self {
            Self::Stale { role, key } => write!(
                f,
                "the {role} argument, {key}, does not resolve in this body"
            ),
            Self::NoCarrier { half_edge } => write!(
                f,
                "the edge of half-edge {half_edge:?} is null scaffolding with no carrier yet, \
                 so it has no chart image. Recourse: describe the edge \
                 (`Body::set_edge_curve`), then ask again"
            ),
            Self::Certify { half_edge, error } => write!(
                f,
                "the pcurve of half-edge {half_edge:?} failed certification: {}",
                geom_brep::recourse::read_at(error, at)
            ),
            Self::LoopDiscontinuity { half_edge } => write!(
                f,
                "the pcurve of half-edge {half_edge:?} does not meet its predecessor's in \
                 its face's chart. \
                 Recourse: re-mint the body if it was edited after minting; if a fresh mint \
                 refuses too, repair the face's boundary"
            ),
            Self::LoopNotClosed { face } => write!(
                f,
                "the pcurves of a loop of face {face:?} do not close in its chart (the \
                 azimuth advance is neither zero nor one full period). Recourse: re-mint the body after any \
                 surgery; if a fresh mint refuses too, repair the loop"
            ),
            Self::JointWithoutRoom {
                face,
                r#loop: lp,
                half_edge,
            } => write!(
                f,
                "loop {lp:?} of face {face:?} passes so near a sphere pole or cone apex at \
                 half-edge {half_edge:?} that its azimuth period is decided with less room than \
                 the joint bound, so its winding is not read; there is nothing in the body to \
                 repair. {JOINT_WITHOUT_ROOM_RECOURSE}"
            ),
            Self::MissingCache { half_edge } => write!(
                f,
                "half-edge {half_edge:?} carries no pcurve although its face's chart mints \
                 them: the body changed after minting. Recourse: re-mint the \
                 body, and report the op that returned it"
            ),
            Self::Unminted { face } => write!(
                f,
                "face {face:?} stores no pcurve although its chart mints them and every one \
                 derives and certifies: the body was not minted after it was built or edited. \
                 Recourse: re-mint the body, and report the op that returned it"
            ),
            Self::RowInterval { half_edge } => write!(
                f,
                "the pcurve of half-edge {half_edge:?} is stated over a carrier interval that \
                 is not its edge's: the body changed after minting. Recourse: re-mint the \
                 body, and report the op that returned it"
            ),
            Self::Escalated { half_edge, cause } => write!(
                f,
                "the pcurve at half-edge {half_edge:?} escalated: {cause}"
            ),
            Self::NetStretchUndecided { half_edge, cause } => write!(
                f,
                "the joint into half-edge {half_edge:?} lies on a spline net closed in v whose \
                 v sections are too close to collapsing to decide its v periods: {cause}"
            ),
            Self::SingularTurn { half_edge, miss } => write!(
                f,
                "the joint into half-edge {half_edge:?} lies on its face's pole or apex, and \
                 {miss}, so the face's side of the point is not decided. {SINGULAR_TURN_RECOURSE}"
            ),
            Self::Band(e) => write!(f, "{e}"),
            Self::PlaceholderChart { face } => write!(
                f,
                "the chart offered for face {face:?}: {}, so nothing on it can be metred. \
                 {PLACEHOLDER_RECOURSE}",
                geom::PLACEHOLDER_SURFACE
            ),
        }
    }
}

geom_brep::ended_by_write!(PcurveMintError);

impl std::error::Error for PcurveMintError {}

impl PcurveMintError {
    /// This refusal as a kernel driver receives it from a pass it called
    /// with keys it read out of the body: passed through, where it is a
    /// fact about the body's geometry.
    ///
    /// # Panics
    ///
    /// On [`PcurveMintError::Stale`]: a driver's keys are ones it read
    /// out of the body, so they resolve, and one that does not is a
    /// kernel bug and not the user's input.
    #[must_use]
    #[track_caller]
    pub(crate) fn for_driver(self) -> Self {
        if let Self::Stale { role, key } = self {
            unreachable!(
                "a kernel driver passed a pcurve pass a {role} key, {key}, that does not \
                 resolve: a driver's keys are ones it read out of the body, so they resolve"
            );
        }
        self
    }
}

/// The recourse for a joint decided with less room than the joint bound
/// ([`PcurveMintError::JointWithoutRoom`]).
const JOINT_WITHOUT_ROOM_RECOURSE: &str = "Recourse: ask about a face whose loops keep farther \
     from the pole or apex, or ask at a smaller ε";

/// The recourse for a joint on the singular set whose turn is not
/// decided ([`PcurveMintError::SingularTurn`]).
pub(crate) const SINGULAR_TURN_RECOURSE: &str = "Recourse: open the face's corner at that point, so its \
     two edges leave it along different directions on one side of it";

/// The recourse for a placeholder chart, whichever enum refuses it.
pub(crate) const PLACEHOLDER_RECOURSE: &str = geom_brep::pcurve_cache::PLACEHOLDER_RECOURSE;

/// A chart that is not the placeholder
/// ([`Surface::is_placeholder_chart`]) — the only kind of chart the
/// loop walk and its meters ([`chart_u_arm`], [`v_meter`]) accept.
///
/// The placeholder's control net is all-poison: it has no locus, so
/// `geom_brep::chart_stretch_sup` refuses it rather than answer a lever
/// arm, and a gap metred through it would be a verdict about nothing.
/// Every door into the walk takes the refusal here, once, where it can
/// still say what to do about it: the minting pass and the validator
/// skip the face (a placeholder mints nothing), and [`loop_windings`]
/// refuses with [`PcurveMintError::PlaceholderChart`].
///
/// [`loop_windings`]: super::loop_windings
#[derive(Clone, Copy)]
pub(crate) struct DescribedChart<'a, T: Real>(&'a Surface<T>);

impl<'a, T: Real> DescribedChart<'a, T> {
    /// `None` for exactly the surface `geom_brep::chart_stretch_sup`
    /// refuses as [`geom_brep::NoChartSup::Placeholder`]: both read
    /// [`Surface::is_placeholder_chart`].
    pub(crate) fn of(surface: &'a Surface<T>) -> Option<Self> {
        (!surface.is_placeholder_chart()).then_some(Self(surface))
    }

    /// The chart of a face the minting pass writes rows for: described,
    /// and of a kind that mints ([`chart_mints`]). `None` is "this face
    /// carries no rows by construction" — the placeholder and the plane.
    pub(crate) fn minting(surface: &'a Surface<T>) -> Option<Self> {
        Self::of(surface).filter(|c| chart_mints(*c))
    }

    pub(crate) fn surface(self) -> &'a Surface<T> {
        self.0
    }
}

/// Does this chart kind mint stored caches (module docs)? A
/// compile-time routing decision per surface kind, exhaustively
/// matched — adding a kind is a compiler-guided edit (D3).
pub(super) fn chart_mints<T: Real>(chart: DescribedChart<'_, T>) -> bool {
    match chart.surface() {
        Surface::Cylinder { .. } => true,
        // Planar faces keep M2's derive-on-demand status (C4 verbatim).
        Surface::Plane { .. } => false,
        // The analytic-chart completion (M6-3, walk row 4): cone,
        // sphere and torus charts certify their closed-form classes
        // (rim/ruling; polar/meridian; parallel/meridian) and mint
        // stored caches wherever the pass runs.
        Surface::Cone { .. } | Surface::Sphere { .. } | Surface::Torus { .. } => true,
        // A spindle sheet's chart takes the pass, whose mint refuses it
        // typed until its classes are written.
        Surface::Spindle { .. } => true,
        // Described NURBS charts mint (M6-3; RATIONAL charts since
        // M8-3): every loft/sweep wall boundary is an iso-parameter
        // curve of the wall it bounds. Seams and LINE cap rims have
        // exact line images (`Pcurve::IsoLine`); an ARC cap rim on a
        // rational wall has an exact image too — the same boundary
        // line, on the chart's own rational-quadratic parameter
        // (`Pcurve::IsoArc`). The placeholder is not a described chart
        // and never reaches here; it mints nothing. An approximating
        // surface's chart is its fit's, described by construction.
        Surface::Nurbs(_) | Surface::Approx(_) => true,
    }
}

/// The pcurve of `half_edge` on its own face's chart — the **stored
/// cache** when there is one, otherwise **derived on demand**.
///
/// The derived answer is on the chart's *principal* branch (there is no
/// loop context in a single-half-edge query); a caller that needs a
/// face-consistent branch reads the stored cache or walks the loop
/// through [`mint_pcurves`]. For the affine (plane) charts that are the
/// standing derive-on-demand case this distinction is vacuous — a plane
/// chart has no branches.
///
/// # Errors
///
/// [`PcurveMintError::Stale`] when `half_edge` does not resolve,
/// [`PcurveMintError::NoCarrier`] when its edge is null scaffolding,
/// or a typed chart refusal for a frontier chart/carrier kind.
pub fn pcurve_of<T: AtRestPolicy>(
    body: &Body<T>,
    half_edge: HalfEdgeKey,
    band: Band,
) -> Result<Pcurve<T>, PcurveMintError> {
    derived_pcurve_of(body, half_edge, band).map(ChartImage::into_pcurve)
}

/// [`pcurve_of`] with the certificate its derivation read, where its lane
/// choice read one ([`ChartImage::Certified`]: an arc rim on a spline
/// chart, certified on the half-edge's own wall over its edge's carrier
/// and interval at `band`), so a reader that certifies the image takes
/// that certificate. A stored row, or a description's stated image, is a
/// [`ChartImage::Stated`]: its certificate was read at the band its mint
/// ran at, not `band`, and a refusal of it keeps its own term.
pub(crate) fn derived_pcurve_of<T: AtRestPolicy>(
    body: &Body<T>,
    half_edge: HalfEdgeKey,
    band: Band,
) -> Result<ChartImage<T>, PcurveMintError> {
    if !body.half_edges.contains_key(half_edge) {
        return Err(PcurveMintError::Stale {
            role: "half_edge",
            key: EntityId::HalfEdge(half_edge),
        });
    }
    if let Some(cache) = body.pcurve(half_edge) {
        let (t0, t1) = cache.params();
        return Ok(ChartImage::Stated {
            pcurve: cache.pcurve().clone(),
            t0,
            t1,
        });
    }
    let (carrier, t0, t1) = half_edge_carrier(body, half_edge)?;
    let surface = half_edge_surface(body, half_edge);
    // A SPLINE chart's images are description-driven (M6-3) — the iso
    // derivation, not the closed-form harmonic table. An approximating
    // surface's chart IS its fit's, so it takes the same route.
    if surface.spline_chart().is_some() {
        return nurbs_iso_derive(body, half_edge, &surface, band);
    }
    Ok(ChartImage::Image {
        pcurve: analytic_derive(&carrier, t0, t1, &surface, band, half_edge)?,
        t0,
        t1,
    })
}

/// **A face of a seam, as [`seam_reading`] takes it**: a
/// [`geom_brep::SeamFace`] whose spline face is always read at its chart,
/// since only [`seam_face`] mints one (and, in tests, `SeamSide::analytic`,
/// which refuses a spline chart).
#[derive(Clone, Copy, Debug)]
pub(crate) struct SeamSide<'s, T: Real>(geom_brep::SeamFace<'s, T>);

impl<'s, T: Real> SeamSide<'s, T> {
    /// `surface` read by its implicit form; `None` on a spline chart,
    /// which [`seam_face`] reads.
    #[cfg(test)]
    pub(crate) fn analytic(surface: &'s Surface<T>) -> Option<Self> {
        surface
            .spline_chart()
            .is_none()
            .then_some(Self(geom_brep::SeamFace::Analytic(surface)))
    }
}

/// Why [`seam_reading`] read no class: [`geom_brep::SeamEscalation`] less
/// the caller's error a [`SeamSide`] cannot make.
#[derive(Clone, Copy, Debug, PartialEq)]
pub(crate) enum SeamUnread {
    /// The arm gate or the wedge's own reading escalated.
    Lever(geom_brep::LeverEscalation),
    /// A spline face's chart has no one normal at the point.
    Crease(geom_brep::CreaseReading),
}

/// **The dihedral of a seam's two faces**, each read by its kind
/// ([`geom_brep::classify_seam`]), a spline face's arm bracketed by its
/// chart's certified lower bound at this scalar. The boolean's seam
/// describer and the split's section-boundary describer both read it.
pub(crate) fn seam_reading<T: Decide + AtRestPolicy>(
    faces: [SeamSide<'_, T>; 2],
    witness: geom_core::Point3<T>,
    reach: geom_core::Lever<T>,
    band: Band,
) -> Result<geom_brep::DihedralClass, SeamUnread> {
    geom_brep::classify_seam(
        faces.map(|f| f.0),
        witness,
        reach,
        |chart, p, reach| T::spline_chart_arm(chart, p, reach).ok(),
        band,
    )
    .map_err(|refusal| match refusal {
        geom_brep::SeamEscalation::Lever(e) => SeamUnread::Lever(e),
        geom_brep::SeamEscalation::Crease(c) => SeamUnread::Crease(c),
        geom_brep::SeamEscalation::SplineReadAsAnalytic => unreachable!(
            "a SeamSide reads every spline face at its chart: seam_face and \
             SeamSide::analytic mint no other"
        ),
    })
}

/// **A face of a seam edge, as its dihedral reads it** at the edge's
/// witness ([`SeamSide`]), the face `surface` bounded by
/// `half_edge`, whose edge's curve is `curve`: an analytic face at the
/// witness; a spline face at the witness's parameters on its own chart,
/// read off an image of the edge in that chart ([`pcurve_of`]) at the
/// edge's own parameter of the witness, the middle of its interval.
/// The image is the stored one where the half-edge has it, and otherwise
/// the one the chart's description derives, an iso row or arc rim of the
/// chart. Either is the edge on the face's own sheet, where a projection
/// foot of the witness could land on another sheet of the chart; a
/// stored `Pcurve::Fitted` image was itself fitted through the chart's
/// feet of a trace, placed on the face's sheet when it was minted.
///
/// `None`, unplaced, on a spline face with no image of the edge in its
/// chart to read the witness's parameters off, or on a null edge (no
/// `curve`). A null edge's witness could be placed off its neighbour in
/// the face's loop, but no class would follow: its zero-length chord gives
/// the wedge no reach, so its dihedral escalates at the arm whatever the
/// faces read.
pub(crate) fn seam_face<'s, T: Decide + AtRestPolicy>(
    body: &Body<T>,
    half_edge: HalfEdgeKey,
    surface: &'s Surface<T>,
    curve: Option<&geom_brep::EdgeCurve<T>>,
    band: Band,
) -> Option<SeamSide<'s, T>> {
    let Some(chart) = surface.spline_chart() else {
        return Some(SeamSide(geom_brep::SeamFace::Analytic(surface)));
    };
    let (t0, t1) = curve?.params();
    let image = pcurve_of(body, half_edge, band).ok()?;
    let uv = image.eval(geom::mid_param(t0, t1));
    Some(SeamSide(geom_brep::SeamFace::Spline {
        chart,
        uv: (uv.x, uv.y),
    }))
}

/// **A transverse seam's `Intersection`, its selector chosen by D2's
/// question: is the curve known in advance?** Where both faces are spline
/// walls and the edge's image on each ([`pcurve_of`]) is a row on its
/// chart's domain side that certifies there, the curve is that shared row
/// ([`geom_brep::IntersectionDraft::shared_row`]); otherwise the draft's
/// witness. Built for boundary rows (the reader's scope, not D2's rule).
/// `sides` are the seam's two half-edges, each with its face's surface
/// key and surface.
pub(crate) fn seam_intersection<T: AtRestPolicy>(
    body: &Body<T>,
    draft: geom_brep::IntersectionDraft<T>,
    sides: [(HalfEdgeKey, geom_brep::SurfaceKey, &Surface<T>); 2],
    band: Band,
) -> geom_brep::DraftedIntersection<T> {
    let [(ha, ka, wa), (hb, kb, wb)] = sides;
    let pair = geom_brep::SurfacePair::new(ka, kb);
    let spline = wa.spline_chart().is_some() && wb.spline_chart().is_some();
    let row = spline
        .then(|| {
            let (ia, ib) = (
                derived_pcurve_of(body, ha, band).ok()?,
                derived_pcurve_of(body, hb, band).ok()?,
            );
            draft.shared_row(pair, [(ka, wa, ia), (kb, wb, ib)], band)
        })
        .flatten();
    match row {
        Some(selector) => draft.into_selected(pair, selector),
        None => draft.into_intersection(pair),
    }
}

/// The chart image of a carrier on an ANALYTIC chart, on the chart's
/// principal branch: the closed form ([`chart_pcurve_over`]) wherever
/// one exists, and the projected image for a carrier with none (a spline
/// carrier, a circle or a cylinder's or cone's line no class images), partitioned
/// over the edge's own
/// interval.
///
/// # Errors
///
/// [`PcurveMintError::Certify`] with the derivation's refusal.
pub(super) fn analytic_derive<T: Decide>(
    carrier: &geom::Curve3<T>,
    t0: T,
    t1: T,
    surface: &Surface<T>,
    band: Band,
    half_edge: HalfEdgeKey,
) -> Result<Pcurve<T>, PcurveMintError> {
    chart_pcurve_over(carrier, t0, t1, surface, band)
        .map_err(|error| PcurveMintError::Certify { half_edge, error })
}

/// **Whether a stored row states its edge's whole interval** — copied
/// from the edge at the mint and at a split; one stated over more or
/// less of the carrier is about another edge
/// ([`PcurveMintError::RowInterval`]). Measured where the edge's own
/// endpoint pinning is (certification's check 3): the carrier at the
/// row's ends against the edge's vertices, in metres. A row that copied
/// its edge's interval evaluates exactly what that check did, so it
/// reads that check's verdict; a parameter difference instead would be
/// radians against a band in metres, and at an interval scalar would
/// carry the copy's own width twice. Read by tier 3 on every stored row
/// ([`validate_pcurves`]) and by a producer's closing mint on every row
/// it carries ([`carry_rows`]).
///
/// [`validate_pcurves`]: super::validate_pcurves
pub(super) fn row_interval<T: Decide>(
    body: &Body<T>,
    he: HalfEdgeKey,
    row: &PcurveCache<T>,
    carrier: &geom::Curve3<T>,
    band: Band,
) -> Result<(), PcurveMintError> {
    let (r0, r1) = row.params();
    let (start, end) = edge_vertex_points(body, he);
    match [(r0, start), (r1, end)]
        .into_iter()
        .map(|(r, p)| {
            decide(
                "pcurve_row_interval",
                Margin::of(carrier.eval(r).distance(p)),
                band,
            )
        })
        .find(|v| !matches!(v, Ok(Sign::Zero)))
    {
        None => Ok(()),
        Some(Ok(_)) => Err(PcurveMintError::RowInterval { half_edge: he }),
        Some(Err(cause)) => Err(PcurveMintError::Escalated {
            half_edge: he,
            cause,
        }),
    }
}

/// The points of `half_edge`'s edge's vertices, in its `he_plus`
/// order: where the carrier's certified interval starts and ends.
#[track_caller]
pub(super) fn edge_vertex_points<T: Decide>(
    body: &Body<T>,
    half_edge: HalfEdgeKey,
) -> (geom_core::Point3<T>, geom_core::Point3<T>) {
    let (edge, data) = edge_record(body, half_edge);
    let point = |he, field| {
        let start = linked(
            &body.half_edges,
            he,
            EntityId::HalfEdge,
            EntityId::Edge(edge),
            field,
        )
        .start;
        body.linked_vertex_point(start, EntityId::HalfEdge(he), "start")
    };
    (
        point(data.he_plus, "he_plus"),
        point(data.he_minus, "he_minus"),
    )
}

/// The point of the vertex `half_edge` starts at: the joint where the
/// loop enters it.
#[track_caller]
pub(super) fn entry_vertex<T: Real>(
    body: &Body<T>,
    half_edge: HalfEdgeKey,
) -> geom_core::Point3<T> {
    let start = half_edge_record(body, half_edge).start;
    body.linked_vertex_point(start, EntityId::HalfEdge(half_edge), "start")
}

/// `he`'s record. Every key this module reads a record through is one
/// a door resolved or one a record of the body holds, so a miss is a
/// kernel bug and panics ([`proven`]).
#[track_caller]
pub(super) fn half_edge_record<T: Real>(body: &Body<T>, he: HalfEdgeKey) -> &HalfEdge {
    proven(&body.half_edges, he, EntityId::HalfEdge)
}

/// `he`'s edge and its record, a link `he`'s record holds.
#[track_caller]
pub(super) fn edge_record<T: Real>(
    body: &Body<T>,
    he: HalfEdgeKey,
) -> (EdgeKey, &crate::entity::Edge) {
    let edge = half_edge_record(body, he).edge;
    let data = linked(
        &body.edges,
        edge,
        EntityId::Edge,
        EntityId::HalfEdge(he),
        "edge",
    );
    (edge, data)
}

/// The face `he` bounds: a link of `he`'s record and of
/// its loop's.
#[track_caller]
pub(crate) fn half_edge_face<T: Real>(body: &Body<T>, he: HalfEdgeKey) -> FaceView<'_, T> {
    let lp = half_edge_record(body, he).parent_loop;
    let face = linked(
        &body.loops,
        lp,
        EntityId::Loop,
        EntityId::HalfEdge(he),
        "parent_loop",
    )
    .face;
    body.linked_face(face, EntityId::Loop(lp), "face")
}

/// The certified curve of `half_edge`'s edge.
///
/// # Errors
///
/// [`PcurveMintError::NoCarrier`] where the edge is null scaffolding.
#[track_caller]
fn half_edge_curve<T: Real>(
    body: &Body<T>,
    half_edge: HalfEdgeKey,
) -> Result<&geom_brep::EdgeCurve<T>, PcurveMintError> {
    let (edge, data) = edge_record(body, half_edge);
    body.edge_curve_linked(edge, data)
        .certified()
        .ok_or(PcurveMintError::NoCarrier { half_edge })
}

/// **The image on `wall`'s chart of a carrier that runs along a side of
/// it**, over `[t0, t1]`, admitted by a shared row's one test
/// ([`geom_brep::side_row_certifies`]): a circle's arc rim over the whole
/// side or part of it ([`geom_brep::pick_arc_rim`]), any other carrier's
/// boundary row or column traversing the chart's whole domain along its
/// moving axis, in either direction ([`probe_pick`]). `None` where no candidate
/// certifies. The image is on the carrier's own parameter, so a second
/// wall's image of one edge's carrier is stated over that edge.
pub(crate) fn side_image_of<T: Decide>(
    carrier: &geom::Curve3<T>,
    (t0, t1): (T, T),
    wall: &Surface<T>,
    band: Band,
) -> Option<Pcurve<T>> {
    use geom_core::{Point2, Vec2};
    let chart = wall.spline_chart()?;
    let span = t1 - t0;
    let picked = if matches!(carrier, geom::Curve3::Circle(..)) {
        geom_brep::pick_arc_rim(carrier, (t0, span), wall, band).ok()?
    } else {
        let ((u0, u1), (v0, v1)) = (
            chart.knot_values_u().domain(),
            chart.knot_values_v().domain(),
        );
        let line = |p0: Point2<T>, pl: Vec2<T>| Pcurve::IsoLine { p0, pl };
        let rows = [v0, v1].into_iter().flat_map(|y| {
            [(u0, u1), (u1, u0)].map(|(a, b)| {
                let slope = (b - a) / span;
                line(Point2::new(a - slope * t0, y), Vec2::new(slope, T::zero()))
            })
        });
        let columns = [u0, u1].into_iter().flat_map(|x| {
            [(v0, v1), (v1, v0)].map(|(a, b)| {
                let slope = (b - a) / span;
                line(Point2::new(x, a - slope * t0), Vec2::new(T::zero(), slope))
            })
        });
        probe_pick(
            "pcurve_shared_side",
            carrier,
            (t0, span),
            wall,
            band,
            rows.chain(columns),
        )
        .ok()?
    }?;
    let picked = ChartImage::Image {
        pcurve: picked,
        t0,
        t1,
    };
    geom_brep::side_row_certifies(wall, &picked, (carrier, (t0, t1)), band)
        .then(|| picked.into_pcurve())
}

/// The certified carrier and parameter interval of `half_edge`'s edge.
///
/// # Errors
///
/// [`half_edge_curve`]'s.
#[track_caller]
pub(crate) fn half_edge_carrier<T: Decide>(
    body: &Body<T>,
    half_edge: HalfEdgeKey,
) -> Result<(geom::Curve3<T>, T, T), PcurveMintError> {
    let curve = half_edge_curve(body, half_edge)?;
    let (t0, t1) = curve.params();
    Ok((curve.carrier().clone(), t0, t1))
}

/// The surface of the face `half_edge` bounds.
#[track_caller]
pub(super) fn half_edge_surface<T: Decide>(body: &Body<T>, half_edge: HalfEdgeKey) -> Surface<T> {
    half_edge_face(body, half_edge).surface().clone()
}

/// The surface KEY of the face `half_edge` bounds (the key twin of
/// [`half_edge_surface`], for the iso derivation's own-side test).
#[track_caller]
fn half_edge_surface_key<T: Decide>(
    body: &Body<T>,
    half_edge: HalfEdgeKey,
) -> geom_brep::SurfaceKey {
    half_edge_face(body, half_edge).record().surface
}

/// The certified description of `half_edge`'s edge.
///
/// # Errors
///
/// [`half_edge_curve`]'s.
#[track_caller]
fn half_edge_description<T: Decide>(
    body: &Body<T>,
    half_edge: HalfEdgeKey,
) -> Result<geom_brep::EdgeDescription<T>, PcurveMintError> {
    Ok(half_edge_curve(body, half_edge)?.description().clone())
}

/// Which ways a carrier over `carrier` can run a chart row over `row`
/// in the row's own spline space — the relation the seam certificate
/// reads, per direction: forward on the same knots, run back on an
/// exact reflection of them ([`KnotVector::is_reflection_of`]). A row
/// candidate is offered only in a direction this admits.
///
/// [`KnotVector::is_reflection_of`]: geom_core::spline::KnotVector::is_reflection_of
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) struct RowSpace {
    pub(super) forward: bool,
    pub(super) backward: bool,
}

impl RowSpace {
    pub(super) fn of(
        carrier: &geom_core::spline::KnotVector,
        row: &geom_core::spline::KnotVector,
    ) -> Self {
        RowSpace {
            forward: carrier.degree() == row.degree() && carrier.knots() == row.knots(),
            backward: row.is_reflection_of(carrier),
        }
    }

    /// Whether a carrier whose `u` rises (`rising`) or falls along it
    /// is in this space.
    fn admits(self, rising: bool) -> bool {
        if rising { self.forward } else { self.backward }
    }
}

/// Derives the **exact iso chart image** of `half_edge` on a described
/// NURBS chart (M6-3; arc rims since M8-3) — the NURBS-chart
/// counterpart of `geom_brep::chart_pcurve`, driven by the edge's
/// INTENSIONAL description (D2: the description is what is
/// authoritative about which iso this locus is):
///
/// - A chart image naming THIS face's surface IS the answer: since
///   the conventional descriptions collapsed (U2) there is nothing
///   left to derive.
/// - An iso LINE image naming the OTHER wall maps as a column of this
///   chart carrying the neighbour's own `v` map — a domain end, or the
///   column the start's certified chart foot measures (a chart wider
///   than the face it trims) — where one holds the carrier at the
///   probe ([`probe_pick`]); otherwise the carrier takes the
///   boundary-iso schedule below, as an `Intersection` seam does, down
///   to its `General` image. Every pick is then CERTIFIED by the full
///   iso lane.
///
/// **The chart's own domain, not the unit square (#327).** Every
/// boundary above is the payload's KNOT domain end. A chart the kernel
/// BUILT is normalized to `[0, 1]²`, so this reads the same values it
/// always did; a chart the kernel IMPORTED carries the file's
/// parameterization — dm1's cylinder wall is `u ∈ [0, 3√3]` — where
/// `u = 1` is an interior column and every pick against it silently
/// answers about the wrong locus.
/// - A cap–wall rim over a CIRCLE carrier maps to the same boundary
///   line through the chart's own rational-quadratic parameter
///   ([`Pcurve::IsoArc`], M8-3), its boundary row and `u` direction
///   picked together at the probe ([`probe_pick`]): the whole
///   row over the edge's interval, then the row over the arc it
///   measures, which a rim covering part of the row (a sub-edge) takes. Every
///   description form of a circle reaches it — see the arm's own note — and
///   a circle no row holds takes the general lane below in every form.
/// - An [`geom_brep::EdgeDescription::Intersection`] over a spline or
///   line carrier, and a LINE carrier in any other form, that lies on a
///   boundary column maps as that column: the same iso line the
///   `IsoCurve` arm mints, recovered from the carrier because the
///   description names no chart coordinate; one on a boundary row (a
///   cap–wall rim) maps as that row. The residency is a pick over the
///   columns and rows and their two directions, each judged at the
///   edge's ends and a quarter span in ([`probe_pick`]), a candidate
///   it does not show passing to the next step; then over the
///   row, and for a line the column, whose moving map the carrier's
///   end feet measure (a sub-edge, a cap rim an offset moved, and for
///   a line an interior column). Any other locus or
///   carrier — a SPLINE on an interior
///   column is the executed case (#498), an off-iso straight edge
///   another, and an ellipse — takes U2's `General`
///   curve-in-UV arm at the honest Fitted grade, derived from the
///   wall's own foot schedule: at degree 1 where that image's measured
///   miss is within its share of the band, finer where the foot path
///   bends past it. Where its certificate refuses the image, the
///   refusal is the lane's gap, "not yet".
/// - Everything else on a NURBS chart refuses typed with the class
///   named.
pub(super) fn nurbs_iso_derive<T: AtRestPolicy>(
    body: &Body<T>,
    half_edge: HalfEdgeKey,
    surface: &Surface<T>,
    band: Band,
) -> Result<ChartImage<T>, PcurveMintError> {
    use geom_core::{Point2, Vec2};
    let refuse = |what: &'static str| PcurveMintError::Certify {
        half_edge,
        error: PcurveCertifyError::IsoUnsupported { what },
    };
    let (carrier, t0, t1) = half_edge_carrier(body, half_edge)?;
    let span = t1 - t0;
    let image = |pcurve| ChartImage::Image { pcurve, t0, t1 };
    // The chart's OWN domain (doc above): `[0, 1]²` for a kernel-built
    // patch, the file's parameterization for an imported one.
    // The catch-all is SPLIT: an approximating surface's domain is its
    // FIT's knot domain, not the unit square — reading `[0,1]²` off a
    // chart parameterized otherwise would place every derived image on
    // the wrong rectangle.
    let (cu0, cu1, cv0, cv1) = match surface.spline_chart() {
        Some(payload) => {
            let ((a, b), (c, d)) = (
                payload.knot_values_u().domain(),
                payload.knot_values_v().domain(),
            );
            (a, b, c, d)
        }
        None => (T::zero(), T::one(), T::zero(), T::one()),
    };
    // **The general lane**, where nothing exact holds: a `General`
    // curve in UV at the honest Fitted grade (U2). A SPLINE on an interior
    // column has no exact class (the measured column below is a line's
    // alone), a DIAGONAL locus (an off-iso straight edge among them) has
    // none, and a circle no row holds, an ellipse or any other curved
    // carrier has none on a spline chart. Its image is the one
    // `FittedLane::general_image` derives: the degree-1 image the edge
    // certificate reads, climbed where its measured miss asks. It is a lane
    // choice (D3), reached only behind the chart's incidence read: a
    // carrier shown off the wall ends as that read. A read not shown off,
    // in band included, goes on to the general image, whose certificate
    // reads the carrier against its wall itself (the subsumption
    // principle); a refusal there is the image's gap
    // (`geom_brep::minted_refusal`).
    let general = || -> Result<ChartImage<T>, PcurveMintError> {
        let Some(wall) = surface.spline_chart() else {
            unreachable!(
                "nurbs_iso_derive: every caller routes only a chart with a spline payload \
                 here"
            )
        };
        if let Err(error) = geom_brep::read_edge_incidence(&carrier, t0, t1, surface, band)
            && geom_brep::IncidenceRead::of(&error)
                .is_some_and(geom_brep::IncidenceRead::is_definite)
        {
            return Err(PcurveMintError::Certify { half_edge, error });
        }
        derive_general_image(&carrier, (t0, t1), wall, band)
            .map(|general| image(Pcurve::General(std::sync::Arc::new(general))))
            .map_err(|error| PcurveMintError::Certify { half_edge, error })
    };
    // The ARC-RIM class: a circle on one of the chart's boundary rows,
    // over the whole row or part of it, its image the row on the
    // chart's own rational-quadratic parameter (`Pcurve::IsoArc`). The
    // row and the `u` direction are one pick, and the arc the closed
    // form or the row measures, the incidence lane's too
    // ([`geom_brep::pick_arc_rim`]); the row's arc-rim certificate is
    // the pick's second reading. Both are a lane choice (D3): a circle
    // the pick does not show on a row, or whose row the certificate
    // refuses or reads in band, takes the general lane. Any other
    // certificate refusal is about the chart or the run, not the
    // candidate, and leaves with its own words.
    let arc_rim = || -> Result<ChartImage<T>, PcurveMintError> {
        let Ok(Some(rim)) = geom_brep::pick_arc_rim(&carrier, (t0, span), surface, band) else {
            return general();
        };
        match PcurveCache::certify(rim, t0, t1, &carrier, surface, band) {
            Ok(cache) => Ok(ChartImage::Certified(cache)),
            Err(
                PcurveCertifyError::Incidence(_)
                | PcurveCertifyError::Escalated { .. }
                | PcurveCertifyError::FittedEscalated { .. }
                | PcurveCertifyError::ResidualExceeded { .. }
                | PcurveCertifyError::IsoUnsupported { .. },
            ) => general(),
            Err(error) => Err(PcurveMintError::Certify { half_edge, error }),
        }
    };

    // The boundary-iso schedule (the arms' own note, below), named by
    // the probe's predicate label. `stated` leads its closed form: the
    // candidates a description itself states (the wall-seam arm's
    // columns), judged by the same probe.
    let iso_schedule = |what: &'static str,
                        stated: Vec<Pcurve<T>>|
     -> Result<Pcurve<T>, PcurveMintError> {
        let Some(wall) = surface.spline_chart() else {
            unreachable!(
                "nurbs_iso_derive: every caller routes only a chart with a spline payload \
                 here"
            )
        };
        let line = !carrier.is_curved();
        // The iso candidates are offered to the carriers the iso
        // class certifies: a spline and a line. A circle never reaches
        // here (its arm is the arc rim's); an ellipse or a spiric has no
        // exact class on a spline chart.
        let iso = line || matches!(carrier, geom::Curve3::Nurbs(_));
        let columns = [cu0, cu1].into_iter().filter(|_| iso).flat_map(|x| {
            [(cv0, cv1), (cv1, cv0)].map(|(v_at_t0, v_at_t1)| {
                let slope = (v_at_t1 - v_at_t0) / span;
                Pcurve::IsoLine {
                    p0: Point2::new(x, v_at_t0 - slope * t0),
                    pl: Vec2::new(T::zero(), slope),
                }
            })
        });
        // A cap–wall rim traverses a boundary ROW, `u` moving —
        // offered to a spline only in a direction whose carrier is in
        // the row's own spline space (`RowSpace`: the row class
        // compares control nets over it), and to a line in both, whose
        // class reads no net.
        let row_space = match &carrier {
            geom::Curve3::Nurbs(spline) => {
                RowSpace::of(spline.knots_f64_interim(), wall.knots_u_f64_interim())
            }
            geom::Curve3::Line(_) => RowSpace {
                forward: true,
                backward: true,
            },
            _ => RowSpace {
                forward: false,
                backward: false,
            },
        };
        let directions: Vec<(T, T)> = [(true, (cu0, cu1)), (false, (cu1, cu0))]
            .into_iter()
            .filter_map(|(rising, ends)| row_space.admits(rising).then_some(ends))
            .collect();
        let rows = [cv0, cv1].into_iter().flat_map(|y| {
            directions.iter().map(move |&(u_at_t0, u_at_t1)| {
                let slope = (u_at_t1 - u_at_t0) / span;
                Pcurve::IsoLine {
                    p0: Point2::new(u_at_t0 - slope * t0, y),
                    pl: Vec2::new(slope, T::zero()),
                }
            })
        });
        // Each pick is a lane choice (D3): a candidate the probe shows
        // is taken, and one it does not, in band or decided off, passes
        // to the next step and in the end to the general image.
        if let Ok(Some(cand)) = probe_pick(
            what,
            &carrier,
            (t0, span),
            surface,
            band,
            stated.into_iter().chain(columns).chain(rows),
        ) {
            return Ok(cand);
        }
        // ---- The fixed schedule found nothing: MEASURE the map. ----
        // The candidates above assume the carrier traverses the chart's
        // WHOLE domain along its moving axis, because that is what a
        // natively built wall's seam or rim does. An edge that covers
        // PART of a chart does not — a sub-edge a split left, a cap rim
        // an offset moved into the chart's interior, a chart wider than
        // the face it trims — so the carrier's two end feet measure the
        // moving channel's map, and the fixed channel is a boundary
        // where it is definitely one, else the value the start's foot
        // measures. Offered to the same metre-valued probe, so no UV
        // quantity reaches ε (D4 ¶1). A foot that will not converge
        // measures nothing, as at the wall-seam arm: the step is
        // skipped and the schedule goes on.
        //
        // A measured ROW is offered to every carrier the row space
        // admits in its direction. A measured COLUMN only to a line
        // (the `filter` below). A spline's column reaches the exact
        // class only as a STATED candidate (the wall-seam arm's), whose
        // interior or partial column the seam class certifies by de
        // Boor collapse; an `Intersection` spline seam on an interior
        // column takes `General` below (#498)
        // (`work/pcert/a-spline-seam-on-an-interior-column-is-offered-no-measured-column.md`).
        if (row_space.forward || row_space.backward)
            && let (Some(f0), Some(f1)) = (
                derive_chart_foot(carrier.eval(t0), surface).ok().flatten(),
                derive_chart_foot(carrier.eval(t1), surface).ok().flatten(),
            )
        {
            let rising = f1.x > f0.x;
            let (f0, f1) = (computed_foot::<T>(f0), computed_foot::<T>(f1));
            let (su, sv) = ((f1.x - f0.x) / span, (f1.y - f0.y) / span);
            let rows = [cv0, cv1, f0.y]
                .into_iter()
                .filter(|_| row_space.admits(rising))
                .map(|y| Pcurve::IsoLine {
                    p0: Point2::new(f0.x - su * t0, y),
                    pl: Vec2::new(su, T::zero()),
                });
            let columns = [cu0, cu1, f0.x]
                .into_iter()
                .filter(|_| line)
                .map(|x| Pcurve::IsoLine {
                    p0: Point2::new(x, f0.y - sv * t0),
                    pl: Vec2::new(T::zero(), sv),
                });
            if let Ok(Some(cand)) = probe_pick(
                what,
                &carrier,
                (t0, span),
                surface,
                band,
                rows.chain(columns),
            ) {
                return Ok(cand);
            }
        }
        // ---- Nothing exact holds: the general image. ----
        general().map(ChartImage::into_pcurve)
    };
    let own = half_edge_surface_key(body, half_edge);
    match half_edge_description(body, half_edge)? {
        // **This face's OWN chart image is the answer.** Since the
        // conventional descriptions collapsed (U2), an edge described
        // as an image in THIS chart carries the image itself — there
        // is nothing left to derive, and re-deriving it would be a
        // second opinion about a locus the description already states.
        geom_brep::EdgeDescription::Chart(ref c) if c.surface == own => Ok(ChartImage::Stated {
            pcurve: c.pcurve.clone(),
            t0,
            t1,
        }),
        // **The M8-3 ARC-RIM arm.** An ARC cap rim's chart image is
        // the same boundary line the LINE arm mints, but its moving
        // channel is the chart's own rational-quadratic parameter
        // rather than the arc angle — the `Pcurve::IsoArc` map.
        //
        // The arm is keyed on the CARRIER, not on the description
        // form (an `Intersection` circle included), because the carrier
        // is what the certification reads:
        // a circle rim is a circle rim however its own chart writes
        // it down. Keying on the form would have made the SAME
        // geometry mint natively and refuse on the round trip — a
        // description-form accident, not a fact about the rim.
        //
        // The sub-arc count is read off the chart's u structure (one
        // span per sub-arc, by the loft's construction); that read is
        // a SELECTION, and the CHECK is the full arc-rim certification
        // that follows, which compares the chart's boundary row
        // against the carrier circle's own rational-quadratic form and
        // refuses a chart that is not this construction.
        geom_brep::EdgeDescription::Chart(_)
        | geom_brep::EdgeDescription::Scaffold(_)
        | geom_brep::EdgeDescription::OnPair { .. }
        | geom_brep::EdgeDescription::Intersection { .. }
            if matches!(carrier, geom::Curve3::Circle(..)) =>
        {
            arc_rim()
        }
        // **The wall–wall seam stated on the OTHER wall.** An iso LINE
        // image in another chart, read as a column of this one: the
        // stated `v` map is a candidate only, since nothing here says
        // the two charts share the seam's parameterization.
        geom_brep::EdgeDescription::Chart(geom_brep::ChartCurve {
            pcurve: Pcurve::IsoLine { p0, pl },
            ..
        }) => {
            // The neighbour's `v` map, verbatim, as a column of this
            // chart: at each chart boundary, then at the column the
            // start's measured foot names (a chart wider than the face
            // it trims). They lead the boundary-iso schedule, which
            // judges every candidate at the probe, so a stated
            // map this chart does not share, or a stated image that is
            // no column at all, is passed over rather than trusted. A
            // foot that will not converge places no column: that
            // candidate is dropped, and the schedule goes on.
            let column = |x: T| Pcurve::IsoLine {
                p0: Point2::new(x, p0.y),
                pl: Vec2::new(T::zero(), pl.y),
            };
            let at_foot = derive_chart_foot(carrier.eval(t0), surface)
                .ok()
                .flatten()
                .map(|foot| column(computed_foot::<T>(foot).x));
            let stated = [cu0, cu1].map(column).into_iter().chain(at_foot).collect();
            iso_schedule("pcurve_iso_wall_seam", stated).map(image)
        }
        // **The boundary-iso arms: an INTERSECTION seam, and a LINE rim
        // in any other form.** The same wall–wall seam the `IsoCurve`
        // arm maps, stated INTRINSICALLY: the locus is `S₁ ∩ S₂`, and
        // where that locus IS this chart's own `u = u₀`/`u = u₁`
        // boundary column its chart image is the same iso line. The
        // description carries no chart coordinates at all — it names
        // two surfaces and a witness — so the image is recovered from
        // the CARRIER against the chart's own domain. A straight edge
        // described as a chart image or a scaffold is the same question
        // about the same carrier, and takes the same schedule.
        //
        // Keyed on the carrier and on BOUNDARY RESIDENCY, never on the
        // description form or the operand order: which of `s1`/`s2` is
        // the plane is not a fact about the locus, and the same seam
        // stated natively or restated foreign must mint the same image.
        //
        // The pick is ONE fixed schedule (D9): any candidates the
        // description states (the wall-seam arm's columns carrying the
        // neighbour's `v` map), then the chart's two boundary columns
        // and two boundary rows × the two directions the carrier can
        // traverse each, every candidate judged by the probe
        // ([`probe_pick`]): the edge's ends and the point a quarter in. A
        // SELECTION, checked: the full iso certification follows every
        // derivation.
        //
        // **Both directions are offered, and the certification decides.**
        // A LINE carrier's class reads no control net, so a line that
        // traverses the chart's `v` backwards certifies on its backward
        // column. A SPLINE carrier's class compares its net with the
        // column's in one spline space, so a spline traversing `v`
        // backwards certifies on its backward column when its knots are
        // the column's exact reflection, and refuses ("not the chart's
        // own column") otherwise. Either way the candidate offered is the image the
        // carrier actually has, not a forward one that fits the class
        // and states the wrong traversal.
        //
        // A carrier the exact candidates do not claim stores its
        // `General` image.
        geom_brep::EdgeDescription::Chart(_)
        | geom_brep::EdgeDescription::Scaffold(_)
        | geom_brep::EdgeDescription::OnPair { .. }
            if !carrier.is_curved() =>
        {
            iso_schedule("pcurve_iso_line_rim", Vec::new()).map(image)
        }
        // An `OnPair` names the same two surfaces an `Intersection`
        // does, and its locus is the same question about the carrier.
        geom_brep::EdgeDescription::Intersection { .. }
        | geom_brep::EdgeDescription::OnPair { .. } => {
            iso_schedule("pcurve_iso_seam_column", Vec::new()).map(image)
        }
        // No lane is written for the rest, a capability gap reached only
        // behind the chart's incidence read: a carrier shown off the wall,
        // or read in band, ends as that read.
        geom_brep::EdgeDescription::TangentIntersection { .. }
        | geom_brep::EdgeDescription::Chart(_)
        | geom_brep::EdgeDescription::Scaffold(_) => Err(
            match geom_brep::read_edge_incidence(&carrier, t0, t1, surface, band) {
                Err(error) => PcurveMintError::Certify { half_edge, error },
                Ok(()) => refuse(
                    "no iso derivation for this locus on a NURBS chart — only chart images of \
             this chart, iso-line images of the neighbouring wall, boundary-iso \
             Intersection seams, and LINE or CIRCLE cap rims have exact chart images \
             (the trimmed-NURBS pcurve lane is the cut-loft unit's)",
                ),
            },
        ),
    }
}

/// The **general curve-in-UV image** of a carrier on this face's own
/// spline chart over the edge's interval `params` — U2's `General`
/// arm, at the honest Fitted grade.
///
/// The producer is [`geom_brep::FittedLane::general_image`]: the degree-1
/// image the plane × NURBS edge certificate reads at adopt time, climbed
/// to a finer one where its measured miss asks. Nothing is certified here: this returns EVIDENCE,
/// and the mint's next move is `PcurveCache::certify_general`, which
/// bounds `sup_t |S(P(t)) − C(t)|` over the whole span against the
/// chart.
///
/// # Errors
///
/// [`PcurveCertifyError::FittedLaneUnsupported`] at a scalar with no
/// certified lane (a dual body may not certify — D1), or the
/// derivation's own typed refusal.
pub(super) fn derive_general_image<T: AtRestPolicy>(
    carrier: &geom::Curve3<T>,
    params: (T, T),
    wall: &geom::NurbsSurface<T>,
    band: Band,
) -> Result<geom::NurbsCurve2<T>, PcurveCertifyError> {
    let Some(lane) = T::fitted_lane() else {
        return Err(PcurveCertifyError::FittedLaneUnsupported { scalar: T::NAME });
    };
    lane.general_image(carrier, params, wall, band)
}

/// The **certified chart foot** of one model-space point on this face's
/// own spline chart — [`derive_general_image`]'s single-sample sibling,
/// same producer ([`geom_brep::FittedLane::chart_foot`]).
///
/// `None` at a scalar with no certified lane, which lets a caller keep
/// its previous typed refusal rather than inventing a foot: a dual body
/// answers exactly what it answered before this widening existed.
///
/// # Errors
///
/// The lane's refusal when the projection will not converge.
pub(super) fn derive_chart_foot<T: AtRestPolicy>(
    point: geom_core::Point3<T>,
    surface: &Surface<T>,
) -> Result<Option<geom_core::Point2<f64>>, PcurveCertifyError> {
    let (Some(wall), Some(lane)) = (surface.spline_chart(), T::fitted_lane()) else {
        return Ok(None);
    };
    lane.chart_foot(point, wall).map(Some)
}

/// A chart foot [`derive_chart_foot`] measured, in `T`: Newton's
/// `f64`, so each coordinate enters as a computed value.
pub(super) fn computed_foot<T: Real>(foot: geom_core::Point2<f64>) -> Point2<T> {
    foot.map(T::from_computed)
}

/// Is `half_edge` the `he_plus` of its edge (so the loop traverses it
/// forward in the carrier parameter)?
#[track_caller]
pub(crate) fn is_plus<T: Decide>(body: &Body<T>, half_edge: HalfEdgeKey) -> bool {
    edge_record(body, half_edge).1.he_plus == half_edge
}

/// The FIRST-CHANNEL arm of a chart — the metres a unit step of the
/// chart's `u` moves the mapped point (D4 ¶1: no UV-space tolerance
/// ever reaches ε), as a sup over the WHOLE chart:
/// `geom_brep::chart_stretch_sup`'s first component. On a plane `u` IS
/// metres, so the arm is exactly 1; on a spline chart `u` is the net's
/// own parameter, and the arm is the net's `sup |S_u|`. A joint on an
/// azimuth chart reads the vertex's own distance from the axis instead
/// ([`joint_arm`]).
///
/// # Direction of error: why the spline arm is the SUP bound
///
/// Every spline-chart caller of this function divides a chart-space
/// discrepancy against the linear band and asks whether the mapped
/// points are within ε — an ESCAPE claim in both of its two uses, and
/// the sup bound is the conservative side of each:
///
/// - `pcurve_loop_continuity` ([`spline_gap_closes`]) asks *"does this
///   joint gap keep the loop closed?"*. An OVER-stated arm over-states
///   the metre gap, which can only turn a `Zero` (closed) into a
///   definite discontinuity or an escalation. It refuses; it cannot
///   certify a loop closed across a gap the model can see. An
///   UNDER-stated arm does the reverse, and `1` on a chart whose
///   stretch is 100 m per chart unit under-states by exactly that
///   factor.
/// - `pcurve_loop_pole_joint` ([`singular_at`]'s net-level form) asks
///   *"is this lever zero, so that no shift can select a branch?"*.
///   Here too sup is the safe side: `Zero` under the SUP bound means no
///   `u` displacement anywhere on the chart moves the point past the
///   band, so skipping the branch shift is honest. Under an inf reading
///   the same verdict would claim a collapsed lever on a chart that has
///   one.
///
/// `geom_brep::chart_stretch_sup` is that bound and states the same
/// split at the export; it is emphatically not a lower bound, and
/// nothing here may be read as one.
///
/// [`joint_arm`]: super::joint_arm
/// [`singular_at`]: super::singular_at
/// [`spline_gap_closes`]: super::spline_gap_closes
pub(super) fn chart_u_arm<T: Real>(chart: DescribedChart<'_, T>) -> SupSpeed<T> {
    // `chart_stretch_sup` refuses only the cone, whose arm grows with
    // `v` and which no caller asks here, and the placeholder, which a
    // [`DescribedChart`] is not.
    let Ok((sup_u, _)) = geom_brep::chart_stretch_sup(chart.surface()) else {
        unreachable!(
            "chart_u_arm is asked on a plane or spline chart only, and DescribedChart::of \
             excludes the placeholder"
        )
    };
    sup_u
}

/// The chart's **u period** — the whole-number shift the loop walk may
/// apply to land a half-edge's entry on its predecessor's exit
/// (`walk_loop`), and the wrap a loop's winding may carry
/// ([`Winding::closes`]).
///
/// * every AZIMUTH chart (cylinder, cone, sphere, torus) answers `τ`;
/// * a PLANE answers `None`: its `u` is a length, not an angle, so it
///   has no branch to decide;
/// * a NURBS chart CLOSED in `u` — first and last control columns the
///   same locus at the band — answers the payload's own u knot-domain
///   length;
/// * a NURBS chart NOT closed in `u` answers `None`: no shift offered,
///   no wrapped closure accepted.
///
/// A kernel-built chart is normalized to `[0, 1]²`, so a closed-in-u
/// one answers `Some(1.0)`, and a gap can shift by a whole chart period
/// there. The joint's own chart-space gap still certifies the shifted
/// entry ([`spline_gap_closes`]).
///
/// On a spline chart the shift is offered only when the net-level
/// singular gate ([`singular_at`]) reads the metres its `u` knot domain
/// moves a point ([`net_reach`]: the domain's length times the net's own
/// `sup |S_u|`, [`chart_u_arm`]) as definitely past the band. A chart
/// whose reach sits under the band takes no shift at all (no `u`
/// displacement on it moves a point past ε), one whose reach lands
/// inside the band takes none either, and the joint's chart-space gap
/// decides; only a definitely-metric chart reaches the branch decision.
///
/// **Why a NURBS chart needs this (#327).** A full-period cylinder
/// wall stated as ONE B-spline patch with a seam generator used twice
/// is the shape every translator writes and the kernel's own band
/// re-mint produces. Its loop walks the chart's whole u range and its
/// seam edge takes the two u-boundary branches — exactly the analytic
/// seam case, but at the chart's own period rather than `τ`. Without
/// this the walk reports the wrap as a discontinuity: a symptom, not
/// the cause.
///
/// [`Winding::closes`]: crate::joint::Winding::closes
/// [`singular_at`]: super::singular_at
/// [`spline_gap_closes`]: super::spline_gap_closes
pub(crate) fn chart_u_period<T: Decide>(surface: &Surface<T>, band: Band) -> Option<Period<T>> {
    // An analytic chart states its own (`Surface::angle_periods`). A
    // spline chart — the payload's or an approximating surface's fit —
    // has whatever period its knot domain says, and only if the net
    // actually closes; handing it `τ` would let the loop walk wrap a
    // chart that does not.
    if let Some((u, _)) = surface.angle_periods() {
        return u;
    }
    let payload = surface.spline_chart()?;
    let (u0, u1) = payload.knot_values_u().domain();
    net_closes_as(payload, NetChannel::U, band).then(|| Period::new(u1 - u0))
}

/// The chart's SECOND channel period — the `v` companion of
/// [`chart_u_period`] (C4: every closed channel has its own period, which
/// the chart owns). `τ` where `v` is an angle (sphere, torus:
/// [`polar_arm`]); the knot domain's length on a spline net decided
/// closed in `v` ([`net_closes_as`], `pcurve_chart_v_closed`); `None` on a
/// plane, cylinder or cone, whose `v` is a length, and on an open net.
/// The kind of period it is ([`VPeriod`]) is what levers its marks
/// ([`decide_joint`]).
///
/// [`decide_joint`]: super::decide_joint
pub(super) fn chart_v_period<T: Decide>(surface: &Surface<T>, band: Band) -> Option<VPeriod<T>> {
    if let Some((_, v)) = surface.angle_periods() {
        return v.map(|period| VPeriod::Angle {
            period,
            polar: SupSpeed::new(
                polar_arm(surface).unwrap_or_else(|| {
                    unreachable!("a chart whose `v` is an angle has a polar arm")
                }),
            ),
        });
    }
    let payload = surface.spline_chart()?;
    let (v0, v1) = payload.knot_values_v().domain();
    net_closes_as(payload, NetChannel::V, band).then(|| VPeriod::Net(Period::new(v1 - v0)))
}

/// **Which channels of `surface`'s chart close on themselves**, `[u, v]`,
/// as the loop walk reads them ([`chart_u_period`], [`chart_v_period`]): a
/// side of the chart's rectangle in a closed channel is a seam the face
/// may lie across, and one in an open channel is the surface's own
/// boundary.
pub(crate) fn chart_closed_channels<T: Decide>(surface: &Surface<T>, band: Band) -> [bool; 2] {
    [
        chart_u_period(surface, band).is_some(),
        chart_v_period(surface, band).is_some(),
    ]
}

/// **Whether a side of `surface`'s chart rectangle bounds a face lying
/// on it** (`section_cert`'s W1′): the side lies in a channel the chart
/// does not close ([`chart_closed_channels`]), and it is a curve, not a
/// point ([`geom_brep::chart_singularity::spline_side_spans`], the one
/// reading of a collapsed side, a pole a face may lie across with no
/// edge on it); a surface that is no spline has no side read here.
pub(crate) fn chart_side_bounds<T: Decide>(
    surface: &Surface<T>,
    side: geom_brep::ChartSide,
    band: Band,
) -> bool {
    use geom_brep::ChartAxis;
    let [u_closed, v_closed] = chart_closed_channels(surface, band);
    let closed = match side.fixed {
        ChartAxis::U => u_closed,
        ChartAxis::V => v_closed,
    };
    let Some(payload) = surface.spline_chart() else {
        return false;
    };
    !closed && geom_brep::chart_singularity::spline_side_spans(payload, side, band)
}

/// A spline net's channel, for the closure question [`net_closes_as`] asks.
#[derive(Clone, Copy)]
pub(super) enum NetChannel {
    U,
    V,
}

/// **Whether a spline net closes in `channel`**: every control point of
/// its first boundary row (column) is decided at zero distance from the
/// matching one of its last, at `band` (`pcurve_chart_u_closed`,
/// `pcurve_chart_v_closed`). One rule for both channels, so the two
/// periods cannot drift apart.
///
/// Any other answer is "open", an in-band one included; that fold, and
/// a test that compares neither weights nor reads an unclamped knot
/// vector's boundary, are
/// `work/pcert/chart-u-period-folds-an-in-band-net-closure-into-no-period.md`'s,
/// for both channels.
fn net_closes_as<T: Decide>(
    payload: &geom::NurbsSurface<T>,
    channel: NetChannel,
    band: Band,
) -> bool {
    let (nu, nv) = payload.control_counts();
    let control = payload.control();
    let (name, pairs): (_, Vec<(usize, usize)>) = match channel {
        NetChannel::U if nu >= 2 && nv > 0 => (
            "pcurve_chart_u_closed",
            (0..nv).map(|iv| (iv, (nu - 1) * nv + iv)).collect(),
        ),
        NetChannel::V if nv >= 2 && nu > 0 => (
            "pcurve_chart_v_closed",
            (0..nu).map(|iu| (iu * nv, iu * nv + nv - 1)).collect(),
        ),
        NetChannel::U | NetChannel::V => return false,
    };
    pairs.into_iter().all(|(i, j)| {
        let (Some(a), Some(b)) = (control.get(i), control.get(j)) else {
            return false;
        };
        matches!(
            decide(name, Margin::of(a.distance(*b)), band),
            Ok(Sign::Zero)
        )
    })
}

/// **The metres a whole sweep of a spline net's `channel` moves a point,
/// at most**: the channel's knot domain length crossed by the net's sup
/// stretch in it ([`chart_u_arm`] in `u`, [`v_meter`] in `v`). The
/// net-level gates ask whether this is under the band
/// (`pcurve_loop_pole_joint` in [`singular_at`], `pcurve_loop_v_stretch`
/// in [`decide_joint`]): where it is, no displacement in the channel
/// anywhere on the chart, a whole period included, moves a point past
/// ε. The stretch alone is a rate (metres per parameter unit), not a
/// length, so it is never compared with the band: under ε on a knot
/// domain 10¹⁰ long it still moves a point metres.
///
/// The sup side ([`Margin::metered_sup`]) is the escape side here, as in
/// [`spline_gap_closes`]: an over-stated reach can only keep a gate from
/// reading the channel as collapsed.
///
/// [`decide_joint`]: super::decide_joint
/// [`singular_at`]: super::singular_at
/// [`spline_gap_closes`]: super::spline_gap_closes
pub(super) fn net_reach<T: Real>(
    chart: DescribedChart<'_, T>,
    payload: &geom::NurbsSurface<T>,
    channel: NetChannel,
) -> Margin<T> {
    match channel {
        NetChannel::U => {
            let (u0, u1) = payload.knot_values_u().domain();
            Margin::metered_sup(u1 - u0, chart_u_arm(chart))
        }
        NetChannel::V => {
            let (v0, v1) = payload.knot_values_v().domain();
            Margin::metered_sup(v1 - v0, v_meter(chart))
        }
    }
}

/// **The periods the chart owns**, one per channel ([`chart_u_period`],
/// [`chart_v_period`]), decided once where a walk reads its joints
/// ([`ChartPeriods::of`]), and the one band-free reading a reader with no
/// band takes ([`ChartPeriods::stated`]). Built only by those two, so a
/// value never mixes one channel decided at one band with the other at
/// another.
#[derive(Clone, Copy, Debug)]
pub(crate) struct ChartPeriods<T> {
    u: Option<Period<T>>,
    pub(super) v: Option<VPeriod<T>>,
}

/// The second channel's period, and what levers a gap on it: the polar
/// radius where `v` is an angle, the net's `sup |S_v|` ([`v_meter`]) on a
/// net closed in `v`.
#[derive(Clone, Copy, Debug)]
pub(crate) enum VPeriod<T> {
    /// `v` is an angle (sphere, torus): `τ`, metered by the polar radius
    /// ([`polar_arm`]).
    Angle {
        /// `τ`.
        period: Period<T>,
        /// The polar radius, metres per radian: exact, so a sup.
        polar: SupSpeed<T>,
    },
    /// A spline net closed in `v`: its `v` knot domain's length.
    Net(Period<T>),
}

impl<T: Copy> VPeriod<T> {
    /// The period, whichever kind it is.
    pub(crate) fn period(self) -> Period<T> {
        match self {
            Self::Angle { period, .. } | Self::Net(period) => period,
        }
    }
}

impl<T: Copy> ChartPeriods<T> {
    /// The first channel's period, where the chart has one.
    pub(crate) fn u(self) -> Option<Period<T>> {
        self.u
    }

    /// The second channel's period, where the chart has one.
    pub(crate) fn v(self) -> Option<Period<T>> {
        self.v.map(VPeriod::period)
    }
}

impl<T: Decide> ChartPeriods<T> {
    /// The periods `surface`'s chart owns, its closures decided at `band`.
    pub(crate) fn of(surface: &Surface<T>, band: Band) -> Self {
        Self {
            u: chart_u_period(surface, band),
            v: chart_v_period(surface, band),
        }
    }

    /// The periods an element applies in where no band is at hand
    /// ([`Lift::read`], whose rows and elements a decided walk wrote):
    /// `τ` per angle channel, and a net's knot domain lengths whether or
    /// not a band would decide the net closed. Only a net decided closed
    /// carries an element with periods on it, so on every element a
    /// door wrote this agrees with [`ChartPeriods::of`]. One planted by
    /// hand on an OPEN net applies here, where [`ChartPeriods::of`] would
    /// leave a gap; tier 3 re-decides it ([`validate_pcurves`]), and the
    /// disagreement is
    /// `work/pcert/chart-channel-periods-are-spelled-in-several-places.md`'s.
    ///
    /// [`Lift::read`]: super::Lift::read
    /// [`validate_pcurves`]: super::validate_pcurves
    pub(crate) fn stated(surface: &Surface<T>) -> Self {
        // An analytic chart's are its own (`Surface::angle_periods`); a
        // net's are its knot domains' lengths, stated without a band.
        if let Some((u, v)) = surface.angle_periods() {
            let v = v.map(|period| VPeriod::Angle {
                period,
                polar: SupSpeed::new(polar_arm(surface).unwrap_or_else(|| {
                    unreachable!("a chart whose `v` is an angle has a polar arm")
                })),
            });
            return Self { u, v };
        }
        let Some(payload) = surface.spline_chart() else {
            unreachable!("a surface with no analytic chart is a spline chart")
        };
        let (u0, u1) = payload.knot_values_u().domain();
        let (v0, v1) = payload.knot_values_v().domain();
        Self {
            u: Some(Period::new(u1 - u0)),
            v: Some(VPeriod::Net(Period::new(v1 - v0))),
        }
    }
}

#[cfg(test)]
impl<T: Decide> ChartPeriods<T> {
    /// A row's chart handed a `u` period of its own (a fixture that
    /// states a period its net is not decided closed in), the `v` period
    /// still the one `surface` owns at `band`, the band the row decides
    /// at.
    pub(crate) fn given_u(u: Option<Period<T>>, surface: &Surface<T>, band: Band) -> Self {
        Self {
            u,
            ..Self::of(surface, band)
        }
    }
}

/// The meridional (second-channel) lever arm where that channel is
/// itself an angle — sphere `v` (arm `r`), torus `v` (arm `r_minor`).
/// `None` = the channel is NOT an angle, which is a different claim
/// from "already metres": see [`v_meter`], which is what a caller
/// wanting the channel's metre rate must ask.
pub(super) fn polar_arm<T: Real>(surface: &Surface<T>) -> Option<T> {
    // Which `v` is an angle is the chart's answer; the arm is its radius.
    surface.angle_periods()?.1?;
    match *surface {
        Surface::Sphere { radius, .. } => Some(radius),
        Surface::Torus { minor_radius, .. } | Surface::Spindle { minor_radius, .. } => {
            Some(minor_radius)
        }
        // The second channel is not an ANGLE on these charts, so no
        // polar radius levers it. That does not make it metres: on a
        // plane, cylinder or cone `v` IS a length, but a spline
        // chart's `v` is the net's own parameter, whose metre rate
        // [`v_meter`] reads off the chart. `None` here means "no
        // polar arm", and `v_meter` is the door that answers what the
        // rate actually is.
        Surface::Plane { .. }
        | Surface::Cylinder { .. }
        | Surface::Cone { .. }
        | Surface::Nurbs(_)
        | Surface::Approx(_) => None,
    }
}

/// The SECOND channel's metre rate for a loop-continuity gap — the
/// `v` companion of [`chart_u_arm`], and the same escape claim.
///
/// Where the channel is an angle ([`polar_arm`]: sphere, torus) the
/// arm is that exact radius. Where it is not, the channel's metre
/// rate is the chart's `sup |S_v|`: exactly 1 on a plane, cylinder or
/// cone, where `v` IS a length, and the net's own stretch on a spline
/// chart, where it is not. The direction argument is
/// [`chart_u_arm`]'s — an over-stated rate can only refuse a closure,
/// while `1` on a chart with a 100 m/unit stretch under-states the
/// metre gap by that factor and certifies a loop closed across it.
/// Unlike [`chart_u_arm`] this channel's parameter is not always an
/// angle, so the crossing IS a rate per parameter unit and the bound
/// direction rides out as a [`SupSpeed`]: the exact polar radius is a
/// sup by being exact, and the spline stretch is one by derivation.
pub(super) fn v_meter<T: Real>(chart: DescribedChart<'_, T>) -> SupSpeed<T> {
    match polar_arm(chart.surface()) {
        Some(radius) => SupSpeed::new(radius),
        // Every described kind has a second-channel sup, the cone
        // included (its `v` is a slant length, so the rate is exactly
        // 1), which is why this reads the `v`-only door rather than
        // the pair.
        None => geom_brep::chart_stretch_sup_v(chart.surface()).unwrap_or_else(|_| {
            unreachable!(
                "chart_stretch_sup_v refuses only the placeholder, which \
                 DescribedChart::of excludes"
            )
        }),
    }
}

/// A whole-period shift of the SECOND channel — the `v` twin of
/// [`geom_brep::Pcurve::shift_branch`], for the charts closed in `v`
/// (sphere, torus, a net closed in `v`: [`chart_v_period`]): `image`
/// translated `k` periods along the second channel. A translation's linear part is the
/// identity, so every image form takes it exactly
/// ([`Pcurve::map_affine`]); the walk and the lift read the one map.
pub(crate) fn shift_v_branch<T: Real>(pcurve: &Pcurve<T>, k: i32, period: Period<T>) -> Pcurve<T> {
    pcurve.translate(T::zero(), period.times(k))
}

/// The sphere chart's INVOLUTION twin of an image: `S(u + π, π − v) =
/// S(u, v)` holds identically on a sphere chart (`radial(u+π) =
/// −radial(u)`, `cos(π−v) = −cos v`, `sin(π−v) = sin v`), so every
/// sphere pcurve has exactly two representations and a pole-crossing
/// walk legitimately needs the OTHER one on the far side — a π azimuth
/// step no whole-period shift can produce. The involution is affine
/// with linear part `diag(1, −1)`, which every image form a sphere
/// chart mints takes exactly ([`Pcurve::map_affine`]).
///
/// `None` off a sphere chart — the torus has no such twin (R > 0 breaks
/// the symmetry), and neither does the cone (cos α > 0) — and where
/// [`Pcurve::map_affine`] refuses the involution: a spiric wall image,
/// which no sphere chart mints and whose form has no image of a map
/// negating one channel (`WallReflected`), and a harmonic row whose
/// polar count has no negation in `i32` (`WindingUnnegatable`), which
/// no derivation mints. The walk reads `None` as no second candidate,
/// the lift as an element that does not apply.
pub(crate) fn sphere_twin<T: Real>(surface: &Surface<T>, pcurve: &Pcurve<T>) -> Option<Pcurve<T>> {
    if !matches!(surface, Surface::Sphere { .. }) {
        return None;
    }
    pcurve
        .map_affine(geom_brep::ChartMap::sphere_involution())
        .ok()
}

/// Mints (and certifies) the pcurve caches of every curved face of
/// `body` whose chart has a certified closed-form lane — the pass the
/// C5 splitting lane runs on each side it produces (spec §1: caches are
/// minted where curved faces are minted).
///
/// Idempotent by construction on a given body: it walks faces in arena
/// order, loops in outer-then-rings order, and half-edges in `next`
/// order, and overwrites any existing row with the same derivation
/// (D9 — same body, same bits).
///
/// # Errors
///
/// [`PcurveMintError`] — a certification refusal, a discontinuous or
/// unclosed loop walk, an escalated classification, or
/// [`PcurveMintError::NoCarrier`] on a face a null edge bounds. Never a
/// silent skip of a face the lane covers. Every row is derived before
/// any is written, so a refusal leaves the body as found.
pub fn mint_pcurves<T: AtRestPolicy>(body: &mut Body<T>, tol: Tol) -> Result<(), PcurveMintError> {
    let band = Band::linear(tol).map_err(PcurveMintError::Band)?;
    let faces: Vec<FaceKey> = body.faces().map(|(k, _)| k).collect();
    let rows = mint_rows(body, &faces, band)?;
    // Start from empty. A body reaching this pass may have been carved
    // from a scratch clone that inherited rows for half-edges the
    // surgery killed (a `SecondaryMap` row outlives its key until the
    // slot is reused), and a stale cache is worse than no cache. What
    // this pass leaves behind is exactly what it minted and certified.
    // The whole-body entry is the only one that can make that claim: a
    // row whose half-edge is dead is reachable from no face, so the
    // subset entry below clears through the faces it is given.
    body.pcurves.clear();
    body.joints.clear();
    for (he, row, element) in rows {
        body.write_row(he, row, element);
    }
    Ok(())
}

/// [`mint_pcurves`] restricted to `faces`: the rows of exactly those
/// faces' half-edges are cleared and re-derived, and **every other row
/// of `body` is left exactly as it was found**.
///
/// The pass is **per face**: a face's branch pinning and
/// certification read that face's own loops, surface and edge
/// descriptions and nothing else, so on the named faces the result is
/// bit-for-bit [`mint_pcurves`]'s, and that pass's idempotence and
/// determinism statements hold verbatim for them — in the order they
/// are named.
///
/// # This is NOT `mint_pcurves` over a subset of the map
///
/// The two differ in what they CLEAR, and the difference is not
/// cosmetic. `mint_pcurves` empties the map first, so it also drops
/// rows whose half-edge no longer exists — a `SecondaryMap` row
/// outlives its key until the slot is reused. This entry reaches rows
/// through the faces it is given, and **a dead half-edge is reachable
/// from no face**: after a kill op, the rows of the killed half-edges
/// survive this pass over *every* face of the body, where the
/// whole-body pass leaves none. `validate_pcurves` cannot see them
/// either — it reaches rows through face loops — so they are invisible
/// to tier 3 until the slot is recycled.
///
/// **A caller that has killed a half-edge and wants the map's at-rest
/// guarantee must use [`mint_pcurves`].** What this entry gives is the
/// weaker, scoped statement a partial producer can honestly make: the
/// rows it may have staled are re-derived, and it asserts nothing at
/// all about the rest of the map. The simultaneous offset doors are
/// exactly that caller: they perform no topological surgery, so no
/// half-edge of theirs ever dies and the two statements coincide.
/// Pinned by `sweep`'s `shell10_r1_probes` and `shell10_r2_probes`,
/// which kill a half-edge through the public `kef` and count what each
/// pass leaves.
///
/// # Returns
///
/// The number of rows the named faces carry when the pass returns.
/// Computed as the map's growth across the mint, which is the same
/// number: every row cleared above belonged to a named face, and a
/// half-edge lies on exactly one loop and so on exactly one face, so
/// minting a named face can only re-fill rows the clearing emptied.
///
/// # Errors
///
/// [`PcurveMintError::Stale`] for a face in `faces` that does not
/// resolve, before any work; otherwise [`mint_pcurves`]'s, raised by a
/// face in `faces`, with the body left as found.
pub fn mint_pcurves_of<T: AtRestPolicy>(
    body: &mut Body<T>,
    faces: &[FaceKey],
    tol: Tol,
) -> Result<usize, PcurveMintError> {
    let band = Band::linear(tol).map_err(PcurveMintError::Band)?;
    if let Some(&face) = faces.iter().find(|&&face| !body.faces.contains_key(face)) {
        return Err(PcurveMintError::Stale {
            role: "faces",
            key: EntityId::Face(face),
        });
    }
    let rows = mint_rows(body, faces, band)?;
    let cleared: Vec<HalfEdgeKey> = faces
        .iter()
        .flat_map(|&face| body.proven_face(face).half_edges())
        .collect();
    body.drop_rows(cleared);
    let before = body.pcurves.len();
    for (he, row, element) in rows {
        body.write_row(he, row, element);
    }
    Ok(body.pcurves.len() - before)
}

/// **The rows the mint writes over `faces`**, derived and certified
/// before it writes any — the shared body of [`mint_pcurves`] and
/// [`mint_pcurves_of`], which differ only in which rows they clear —
/// in `faces` order, each face's in walk order. Read-only, so a
/// refusal, or a panic on a torn record, leaves the body as found.
///
/// A face whose rows are not owed ([`not_owed`]: a pair the chart can
/// hold but no route covers yet — a line, ellipse or spiric offered a
/// fitted image — or a fitted face at a scalar with no fitted door; the
/// at-rest pass excuses exactly these, by the same predicate)
/// contributes the rows it held, carried ([`carry_rows`]), so the mint
/// never drops a certificate it cannot re-derive. Every OTHER refusal
/// — a carrier off its face or grazing it, an image that is not its
/// carrier's, a general sphere circle its fitted
/// route refuses, a covered class whose residuals, envelope, continuity
/// or closure refuse — is a genuine defect and propagates.
fn mint_rows<T: AtRestPolicy>(
    body: &Body<T>,
    faces: &[FaceKey],
    band: Band,
) -> Result<Certified<T, HalfEdgeKey>, PcurveMintError> {
    let mut rows = Vec::new();
    for &face in faces {
        match derive_face(body, face, band) {
            Ok(derived) => rows.extend(derived),
            Err(e) if not_owed::<T>(&e) => rows.extend(carry_rows(body, face, band)?),
            Err(e) => return Err(e),
        }
    }
    Ok(rows)
}

/// **The rows a face holds, carried across a mint that cannot derive
/// them**: each one the face's loops reach is re-certified through its
/// own door against the current carrier and surface. So a row the mint
/// has no route to — a
/// `Fitted` row on a spline chart, stated by a certifying door —
/// survives a producer's closing mint (a
/// transform, a merge) exactly when it still certifies, and a half the
/// face held no row for stays without one, for tier 3 to read.
///
/// # Errors
///
/// [`PcurveMintError::Certify`] for a carried row that no longer
/// certifies, and [`PcurveMintError::RowInterval`] for one whose
/// interval is no longer its edge's ([`row_interval`]): the producer
/// refuses rather than drop it or store it stale.
/// [`PcurveMintError::NoCarrier`] for a row held on a half of a null
/// edge.
fn carry_rows<T: AtRestPolicy>(
    body: &Body<T>,
    face: FaceKey,
    band: Band,
) -> Result<Certified<T, HalfEdgeKey>, PcurveMintError> {
    let view = body.proven_face(face);
    let surface = view.surface();
    let held: Vec<(HalfEdgeKey, &PcurveCache<T>)> = view
        .half_edges()
        .into_iter()
        .filter_map(|he| body.pcurve(he).map(|row| (he, row)))
        .collect();
    let mut carried = Vec::with_capacity(held.len());
    for (he, row) in held {
        let (carrier, _, _) = half_edge_carrier(body, he)?;
        // A producer that moved the edge's interval under the row
        // refuses here, not later at tier 3.
        row_interval(body, he, row, &carrier, band)?;
        let (t0, t1) = row.params();
        let restated = PcurveCache::certify_stated_image(
            row.pcurve(),
            (t0, t1),
            &carrier,
            surface,
            band,
            T::fitted_lane(),
        )
        .map_err(|error| PcurveMintError::Certify {
            half_edge: he,
            error,
        })?;
        carried.push((he, restated, body.joint(he)));
    }
    Ok(carried)
}

/// **Whether a face's refusal means its rows are not owed**: the one
/// reading, shared by the minting pass ([`mint_rows`]), which leaves
/// such a face storing nothing, and the at-rest pass
/// ([`validate_pcurves`]), which reports nothing about it. One arm, and
/// only this: **a scalar with no fitted door**
/// ([`AtRestPolicy::fitted_lane`] answers `None`, as a dual's does)
/// refusing a face only the fitted lane can image
/// ([`PcurveCertifyError::FittedLaneUnsupported`]): that scalar
/// certifies nothing fitted, so it owes no fitted row. The rule is the
/// scalar's, not a class's — at every scalar that holds the door, a
/// fitted refusal is a finding. Every (chart, carrier) class a chart
/// can hold has a route, so no class is excused.
///
/// [`validate_pcurves`]: super::validate_pcurves
pub(super) fn not_owed<T: AtRestPolicy>(e: &PcurveMintError) -> bool {
    match e {
        PcurveMintError::Certify {
            error: PcurveCertifyError::FittedLaneUnsupported { .. },
            ..
        } => T::fitted_lane().is_none(),
        _ => false,
    }
}

/// **The minting pass over one face, storing nothing** (module docs:
/// the two-pass shape — walk the loops to pin branches, then certify
/// every pcurve against its carrier and chart, [`certify_walked`]). [`mint_rows`] writes what this derives;
/// [`validate_pcurves`] reads a face that does not store its rows
/// through it, so "what the mint would write here" has one answer.
///
/// A face on a chart that mints nothing ([`DescribedChart::minting`])
/// derives no row. `face` is a face of the body: an arena member, or a
/// key its door resolved.
///
/// # Errors
///
/// The face's refusal ([`first_owed`]), read whole: the walk's
/// ([`walk_loop`]) or a certification's.
///
/// [`validate_pcurves`]: super::validate_pcurves
#[track_caller]
pub(super) fn derive_face<T: AtRestPolicy>(
    body: &Body<T>,
    face: FaceKey,
    band: Band,
) -> Result<Certified<T, HalfEdgeKey>, PcurveMintError> {
    let view = body.proven_face(face);
    let surface = view.surface();
    let Some(chart) = DescribedChart::minting(surface) else {
        return Ok(Vec::new());
    };
    let loops: Vec<LoopKey> = view.loops().map(|(lk, _, _)| lk).collect();
    let mut walked: Vec<Walked<T>> = Vec::new();
    let mut refused: Vec<PcurveMintError> = Vec::new();
    let mut broken: Vec<LoopKey> = Vec::new();
    for &lp in &loops {
        if let Err(e) = walk_loop(body, face, lp, chart, band, &mut walked) {
            refused.push(e);
            broken.push(lp);
        }
    }
    // The fitted door goes in as the scalar's policy answers it, `None`
    // and all: the own-chart arm of `nurbs_iso_derive` hands a
    // construction's STATED `General` image here at every scalar, a dual
    // included, and an absent door refuses at check 4, so an image that
    // fails checks 1–3 draws those checks' verdict at every scalar.
    let fitted = |w: &Walked<T>| match w.pcurve().certifying_door() {
        CertifyingDoor::Fitted | CertifyingDoor::Either => PcurveCache::certify_stated_image(
            w.pcurve(),
            w.window(),
            &w.carrier,
            surface,
            band,
            T::fitted_lane(),
        ),
        CertifyingDoor::ClosedForm => unreachable!(
            "certify_walked hands the fitted door only the images it certifies: {:?}",
            w.pcurve()
        ),
    };
    if !refused.is_empty() {
        // A walk stops at the first refusal of its loop, so an edge
        // whose rows are not owed would hide every edge after it: the
        // face is read whole before it is excused ([`not_owed`]). Whole
        // means the mint's own reading of every edge it could image —
        // each run of covered edges a broken loop leaves walked through
        // its joints ([`walk_runs`]), and every image derived, on those
        // runs and on the loops that walked, certified through the
        // mint's doors — so a
        // covered edge's refusal, a joint's or a certificate's, is the
        // face's whichever uncovered edge shares the face.
        if refused.iter().all(not_owed::<T>) {
            for lp in broken {
                let cycle = loop_rows(body, lp);
                walk_runs(
                    body,
                    chart,
                    view.sense(),
                    &cycle,
                    band,
                    &mut walked,
                    &mut refused,
                );
            }
            if let Err(certs) = certify_walked(walked, surface, band, Some(&fitted)) {
                refused.extend(
                    certs
                        .into_iter()
                        .map(|(half_edge, error)| PcurveMintError::Certify { half_edge, error }),
                );
            }
        }
        return Err(first_owed::<T>(refused));
    }
    certify_walked(walked, surface, band, Some(&fitted)).map_err(|refused| {
        first_owed::<T>(
            refused
                .into_iter()
                .map(|(half_edge, error)| PcurveMintError::Certify { half_edge, error })
                .collect(),
        )
    })
}

/// The refusal a face reports: its first whose rows are owed, or — when
/// none is ([`not_owed`]) — its first, which excuses the face. So a
/// face is excused only when every refusal it meets is, whichever edge
/// the walk meets first. Asked only of a face that refused, so
/// `refused` holds at least one.
fn first_owed<T: AtRestPolicy>(refused: Vec<PcurveMintError>) -> PcurveMintError {
    let at = refused.iter().position(|e| !not_owed::<T>(e)).unwrap_or(0);
    refused.into_iter().nth(at).unwrap_or_else(|| {
        unreachable!("first_owed is asked only of a face that refused at least once")
    })
}

/// The rows [`certify_walked`] certified, in walk order: each image's
/// certified row, with the element of the joint into it where the walk
/// decided one.
type Certified<T, K> = Vec<(K, PcurveCache<T>, Option<JointElement>)>;

/// A fitted-grade certifier for an image whose door is the fitted one
/// ([`geom_brep::CertifyingDoor`], [`certify_walked`]).
type FittedDoor<'a, T, K> = &'a dyn Fn(&Walked<T, K>) -> Result<PcurveCache<T>, PcurveCertifyError>;

/// **Pass 2 of the minting walk, for one face**: every image the walk
/// derived certified against its carrier and chart, in walk order. The
/// one home of that pass, shared by `mint_face` (the minting pass)
/// and [`site_rows`] (the Euler operators' site mint), so the rows the
/// two write for one walk are one set of bits. A row's certificate
/// reads that row alone; the loop's facts (its branches and winding)
/// are the walk's ([`walk_cycle`]).
///
/// An image whose derivation's lane choice already read its
/// certificate ([`ChartImage::Certified`]) keeps that certificate and
/// is not certified again. Every other image goes to the door its kind
/// names ([`Pcurve::certifying_door`]): the `Decide`-scalar door,
/// [`PcurveCache::certify`], or `fitted` — the fitted door, which only
/// the pass holds. Without it a fitted or general image meets the
/// closed-form door, which refuses it, and a projected one is read
/// there.
///
/// # Errors
///
/// Every half-edge whose certification refused, with its refusal, in
/// walk order.
///
/// [`site_rows`]: super::site_rows
pub(super) fn certify_walked<T: Decide, K: Copy>(
    walked: Vec<Walked<T, K>>,
    surface: &Surface<T>,
    band: Band,
    fitted: Option<FittedDoor<'_, T, K>>,
) -> Result<Certified<T, K>, Vec<(K, PcurveCertifyError)>> {
    let mut rows = Vec::with_capacity(walked.len());
    let mut refused = Vec::new();
    for w in walked {
        let w = match w.into_certified() {
            Ok(row) => {
                rows.push(row);
                continue;
            }
            Err(w) => *w,
        };
        let (t0, t1) = w.window();
        let cache = match (w.pcurve().certifying_door(), fitted) {
            (CertifyingDoor::Fitted | CertifyingDoor::Either, Some(fitted)) => fitted(&w),
            _ => PcurveCache::certify(w.pcurve().clone(), t0, t1, &w.carrier, surface, band),
        };
        // An image the walk derived: a lane choice's refusal of its own
        // image is the lane's gap (`geom_brep::minted_refusal`). A stated
        // one keeps its term.
        match cache {
            Ok(cache) => rows.push((w.key, cache, w.element)),
            Err(error) => refused.push((
                w.key,
                match w.image {
                    ChartImage::Image { .. } => {
                        geom_brep::minted_refusal(error, w.pcurve(), &w.carrier, surface)
                    }
                    ChartImage::Stated { .. } | ChartImage::Certified(_) => error,
                },
            )),
        }
    }
    if refused.is_empty() {
        Ok(rows)
    } else {
        Err(refused)
    }
}

/// The one-branch walk of a single loop (module docs). Appends the
/// branch-pinned chart curves to `out`.
///
/// The derivation is the one step that forks by chart: a SPLINE chart
/// derives from the edge's description ([`nurbs_iso_derive`], the
/// fitted lane's), every analytic chart from the carrier's closed form
/// ([`chart_pcurve_over`]). Everything after the derivation — the branch
/// pin, the continuity margins, the closure — is [`walk_cycle`], which
/// is stated under `Decide`. An empty loop bounds nothing to chart and
/// appends nothing.
///
/// # Panics
///
/// [`loop_rows`]': `lp` is a loop of `face`'s record.
#[track_caller]
pub(crate) fn walk_loop<T: AtRestPolicy>(
    body: &Body<T>,
    face: FaceKey,
    lp: LoopKey,
    chart: DescribedChart<'_, T>,
    band: Band,
    out: &mut Vec<Walked<T>>,
) -> Result<(), PcurveMintError> {
    let cycle = loop_rows(body, lp);
    walk_halves(body, face, &cycle, chart, band, true, out)
}

/// **`halves` walked in `chart`**, consecutive members of one loop of
/// `face` in loop order: the whole cycle where `closes`, and otherwise
/// an open run of it, which decides no joint into its first member and
/// checks no closure. Each half's image is derived on the chart's
/// principal branch and every joint between two of them decided
/// ([`walk_cycle`]), at `band`, on the face's side (its stored sense).
/// The walked rows join `out`, uncertified.
///
/// # Errors
///
/// A derivation's refusal, or a joint's: as [`mint_pcurves`] reports
/// them, by half-edge.
///
/// # Panics
///
/// Where `face` or a member does not resolve.
#[track_caller]
pub(crate) fn walk_halves<T: AtRestPolicy>(
    body: &Body<T>,
    face: FaceKey,
    halves: &[HalfEdgeKey],
    chart: DescribedChart<'_, T>,
    band: Band,
    closes: bool,
    out: &mut Vec<Walked<T>>,
) -> Result<(), PcurveMintError> {
    let surface = chart.surface();
    let sense = proven(&body.faces, face, EntityId::Face).sense;
    let item = |i: usize| -> Result<WalkItem<T, DerivedRow<T>>, PcurveMintError> {
        let he = halves[i];
        let row = derive_image(body, he, surface, band)?;
        Ok(walk_item(
            row,
            is_plus(body, he),
            entry_vertex(body, he),
            stored_edge(body, he),
        ))
    };
    let wedges = crate::wedge_reach::WedgeReach::new(body);
    let reach = |i: usize| stored_reach(&wedges, body, halves[i]);
    let walked = walk_cycle(
        chart,
        sense,
        halves.len(),
        item,
        |_| None,
        &reach,
        band,
        closes,
    )
    .map_err(|fail| match fail {
        WalkFail::Item(e) => e,
        WalkFail::Miss { index, miss } => joint_refusal(halves[index], miss),
        WalkFail::NotClosed => PcurveMintError::LoopNotClosed { face },
    })?;
    out.extend(
        halves
            .iter()
            .zip(walked)
            .map(|(&key, (row, .., element))| Walked::pinned(key, row, element)),
    );
    Ok(())
}

/// The refusal of the joint into `half_edge` that decided no element.
pub(super) fn joint_refusal(half_edge: HalfEdgeKey, miss: PinMiss) -> PcurveMintError {
    match miss {
        PinMiss::Discontinuity => PcurveMintError::LoopDiscontinuity { half_edge },
        PinMiss::Escalated(cause) => PcurveMintError::Escalated { half_edge, cause },
        PinMiss::NetStretchUndecided(cause) => {
            PcurveMintError::NetStretchUndecided { half_edge, cause }
        }
        PinMiss::Singular(miss) => PcurveMintError::SingularTurn { half_edge, miss },
    }
}

/// **The runs of covered edges in a loop the walk could not close**:
/// the loop read as the mint reads it, minus the edges it cannot image.
/// Each half-edge's image is derived ([`derive_image`]), and every
/// maximal run of consecutive derived images — cyclically, so a run may
/// cross the loop's start — is walked as an open chain ([`walk_cycle`]
/// without the closure), its joints pinned and checked exactly as the
/// loop's own walk would. The pinned images join `out` for
/// certification; a derivation's refusal or a joint's joins
/// `refused`.
pub(super) fn walk_runs<T: AtRestPolicy>(
    body: &Body<T>,
    chart: DescribedChart<'_, T>,
    sense: bool,
    cycle: &[HalfEdgeKey],
    band: Band,
    out: &mut Vec<Walked<T>>,
    refused: &mut Vec<PcurveMintError>,
) {
    let surface = chart.surface();
    type Derived<T> = (HalfEdgeKey, WalkItem<T, DerivedRow<T>>);
    let mut items: Vec<Option<Derived<T>>> = cycle
        .iter()
        .map(|&he| {
            let derived = derive_image(body, he, surface, band).map(|row| {
                (
                    he,
                    walk_item(
                        row,
                        is_plus(body, he),
                        entry_vertex(body, he),
                        stored_edge(body, he),
                    ),
                )
            });
            derived.map_err(|e| refused.push(e)).ok()
        })
        .collect();
    let n = items.len();
    let start = items
        .iter()
        .position(Option::is_none)
        .map_or(0, |gap| (gap + 1) % n.max(1));
    let mut runs: Vec<Vec<Derived<T>>> = Vec::new();
    let mut run: Vec<Derived<T>> = Vec::new();
    for k in 0..n {
        match items[(start + k) % n].take() {
            Some(item) => run.push(item),
            None => runs.push(core::mem::take(&mut run)),
        }
    }
    runs.push(run);
    for run in runs.into_iter().filter(|run| !run.is_empty()) {
        let (keys, mut walk_items): (Vec<_>, Vec<_>) =
            run.into_iter().map(|(he, item)| (he, Some(item))).unzip();
        let item = |j: usize| -> Result<WalkItem<T, DerivedRow<T>>, core::convert::Infallible> {
            Ok(walk_items[j]
                .take()
                .unwrap_or_else(|| unreachable!("walk_cycle reads each item of a run once")))
        };
        let wedges = crate::wedge_reach::WedgeReach::new(body);
        let reach = |i: usize| stored_reach(&wedges, body, keys[i]);
        match walk_cycle(
            chart,
            sense,
            keys.len(),
            item,
            |_| None,
            &reach,
            band,
            false,
        ) {
            Ok(pinned) => out.extend(
                keys.iter()
                    .zip(pinned)
                    .map(|(&key, (row, .., element))| Walked::pinned(key, row, element)),
            ),
            Err(WalkFail::Miss { index, miss }) => refused.push(joint_refusal(keys[index], miss)),
            Err(WalkFail::Item(never)) => match never {},
            Err(WalkFail::NotClosed) => unreachable!("an open run checks no closure"),
        }
    }
}

/// **One half-edge's chart image on the chart's principal branch**,
/// with its carrier and interval — the derivation the walk pins
/// ([`walk_loop`]). The one step that forks by chart: a SPLINE chart
/// derives from the edge's description ([`nurbs_iso_derive`]), every
/// analytic chart from the carrier's closed form ([`chart_pcurve_over`]).
///
/// # Errors
///
/// [`PcurveMintError::NoCarrier`] for a half of a null edge, or the
/// derivation's refusal.
#[track_caller]
pub(super) fn derive_image<T: AtRestPolicy>(
    body: &Body<T>,
    he: HalfEdgeKey,
    surface: &Surface<T>,
    band: Band,
) -> Result<DerivedRow<T>, PcurveMintError> {
    let (carrier, t0, t1) = half_edge_carrier(body, he)?;
    let image = if surface.spline_chart().is_some() {
        // Spline charts derive from the edge's intensional
        // description (M6-3) — see `nurbs_iso_derive`.
        nurbs_iso_derive(body, he, surface, band)?
    } else {
        ChartImage::Image {
            pcurve: analytic_derive(&carrier, t0, t1, surface, band, he)?,
            t0,
            t1,
        }
    };
    Ok((carrier, image))
}
