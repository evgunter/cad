//! Mass properties over the **exact** B-rep (M2 PR 7): volume and
//! surface area assembled from `geom-brep`'s closed-form per-face
//! contributions ([`geom_brep::props`] — divergence-theorem flux split
//! against per-surface anchors; Mäntylä §13.3 generalized off the
//! polyhedral case) — at two granularities over ONE per-face walk:
//! whole-body ([`mass_properties`]) and per-shell
//! ([`classify_shells`], which also decides each shell's outer/void
//! role from its volume's sign).
//!
//! This is the exact-geometry counterpart of the mesh oracle
//! (`mesh::validate::signed_volume`): no tessellation and no sampling.
//! A face with a closed form contributes that form over the stored
//! analytic data, scalar-generic over [`Decide`] so the same formulas
//! instantiate at `f64` (a value) and at the certified interval scalar
//! (an enclosure that **is** the certified bound, Q1); a curved-CUT or
//! described-spline face has no closed form and contributes
//! `geom-brep`'s certified QUADRATURE enclosure instead (M5 PR 11),
//! which is bounded and typed rather than sampled and is why the pads
//! below exist.
//! The coned-polyhedron fan over boundary vertices (Mäntylä's
//! `svolume`) is deliberately absent: on curved faces its magnitude is
//! wrong (it measures the cone over the boundary, not the face — the
//! M2 PR 5 review's finding); the divergence formulation here is exact
//! for the whole M2 face inventory.
//!
//! Layering: `geom-brep` owns the key-free per-face math; this module
//! walks the body's arenas (face → loops → half-edge cycles), flattens
//! each loop into [`geom_brep::LoopEdge`]s, and sums. The tier-3
//! validator's two check-7 derivations consume that walk without any
//! new inter-crate dependency: every check-7 door decides through
//! [`role_walk_set`], the measurement door's sign walk ([`measure_to`]),
//! stopped at the round the validator's own decision is complete, and
//! hands back the [`SignCertificate`] a caller continues to the number.
//!
//! The file is also the per-scalar POLICY home: [`AtRestPolicy`], which
//! says for each scalar which at-rest gates it runs and which injected
//! doors it holds, beside two of those door values ([`QuadLane`],
//! [`ShellDoor`]).

use core::fmt;

use geom::Surface;
use geom_brep::props::quad::{LaneEnd, LaneRun, LaneStop, RoundWindow};
use geom_brep::props::{
    CarrierId, FaceContribution, LoopEdge, PropsError, curved_face_loops, planar_face,
};
use geom_brep::recourse::{Classified, Reading, RefusedArm, RefusedSide, SizedDecision, SizedPass};
use geom_core::interval::{self, Interval};
use geom_core::k_stats::Detached;
use geom_core::{
    Band, BandError, Bounds, Decide, Decided, Indeterminate, Margin, Point3, Real, Sign, Tol,
};
use quantity::Length;
use slotmap::Key;

use crate::body::Body;
use crate::boolean::ContactRecords;
use crate::entity::{FaceKey, HalfEdgeKey, LoopBoundary, LoopKey, ShellKey, SolidKey, VertexKey};
use crate::shell::{ShellError, Shelled};
use crate::validate::ValidationError;

/// Exact-B-rep integral properties of a body.
///
/// A face with a closed form contributes it, evaluated at the scalar;
/// a curved-CUT face (M5 PR 11) contributes a **certified quadrature
/// enclosure**, carried as the enclosure midpoint in
/// `volume`/`surface_area` plus the summed half-widths in the `*_pad`
/// fields, so the quadrature's brackets are `volume ± volume_pad` and
/// `surface_area ± area_pad`.
///
/// **A zero pad means every face took a closed form**, not that the value
/// is exact (the pre-PR-11 behaviour, bit-identical). The value carries
/// its closed forms' rounding and its sum's, which no pad states: a few
/// ulps of the faces' flux terms where each face is read about its own
/// geometry, and more where a face's closed form is levered by a carrier
/// stored far from it
/// (`work/nearpoint/a-planar-face-sums-its-area-about-a-far-carrier-origin`),
/// however small the body.
///
/// **The rule for a reader:** a decision on a zero-pad value re-derives
/// it (`certify_role`: the body's interval re-derivation, or a stated
/// bound on its rounding where there is no lane), or decides on an
/// enclosure of it. It never takes a zero pad as exactness.
#[derive(Clone, Copy, Debug)]
pub struct MassProperties<T: Real> {
    /// The **signed** enclosed volume by the divergence theorem —
    /// positive for a correctly oriented (outward-normal) closed body;
    /// a definitely negative value is orientation corruption (the
    /// tier-3 +V invariant). For quadrature faces this is the
    /// certified enclosure's midpoint; the bracket is `± volume_pad`.
    pub volume: T,
    /// The total surface area (a sum of unsigned face areas); for
    /// quadrature faces the enclosure midpoint (bracket `± area_pad`).
    pub surface_area: T,
    /// Half-width of the quadrature's volume bracket (m³): the summed
    /// half-widths of the certified quadrature enclosures; `0.0` when
    /// every face took a closed form, which states nothing of the
    /// value's rounding (the type's docs), so it certifies nothing there.
    /// The tier-3 +V backstop consumes this.
    pub volume_pad: f64,
    /// Half-width of the quadrature's area bracket (m²), as `volume_pad`.
    pub area_pad: f64,
}

impl<T: Real> MassProperties<T> {
    /// This certificate's volume BRACKET and the lever check 7 meters
    /// it against — the reading a decision about the volume's SIGN is
    /// entitled to make of a certified quadrature.
    ///
    /// # The ends are RECONSTRUCTED, and what that costs
    ///
    /// The stored form is a midpoint and a half-width
    /// (`quad_lane::mid_pad`), so neither end here is interval arithmetic
    /// interval's own endpoint: each is two `f64` roundings away from
    /// it (the halving that built the pair, and this arithmetic). The
    /// LOWER end is the one that now decides an ACCEPTANCE — check 7
    /// passes on a definitely-positive `volume_lo` — so a
    /// reconstruction that landed above the true lower end would err
    /// toward admitting, which is the direction that matters and the
    /// direction that did not exist before that exit did.
    ///
    /// **What the ends are off by, and what reads them.** Three
    /// roundings stand between them and the sum they stand for:
    ///
    /// - the fold's (`fold_runs`): `volume` is the faces' flux terms
    ///   summed at the walk's scalar, round to nearest, so off by up to
    ///   `≈ (n − 1)·u·Σ_f |flux_f| / 3` over `n` faces. That scales with
    ///   the terms, not with `|volume|`: a body at distance `D` from the
    ///   origin has `Σ_f |flux_f| ≈ D·A`, so its fold error is
    ///   `≈ n·u·D·A/3` however small its volume. Against check 7's
    ///   threshold `volume_lo / surface_area ≥ K·ε` it reaches the band
    ///   at `D ≈ 3·K·ε / (n·u)`: `≈ 10⁷ m` for ten faces at `ε = 10⁻⁹`,
    ///   `K = 10`, and proportionally nearer for more;
    /// - the pads': each face's half-width is its enclosure's rounded to
    ///   nearest (`quad_lane::mid_pad`), and their sum, and its third,
    ///   are rounded up, so the pad stands for no less than the faces'
    ///   half-widths summed;
    /// - this reconstruction's: a few ulps of `|volume|`.
    ///
    /// The pads cover the quadrature, not the fold, so these ends are
    /// no certificate against the fold's rounding. **No certified sign
    /// reads them**: check 7's certified read (`certify_role`) decides
    /// off the walk's faces re-derived in interval arithmetic about a
    /// corner of the body (`rederive`), whose terms are the body's own
    /// size wherever it sits, whenever the walk holds a lane; these ends
    /// decide only a walk holding none
    /// (`work/tally/lane-free-volume-sign-reads-decide-on-a-rounded-sum`),
    /// and give a refusal its words.
    ///
    /// The UPPER end's arithmetic is untouched and deliberately so:
    /// it is the end the +V refusal has always read, its rounding is
    /// the rounding that refusal has always carried, and nudging it
    /// would move a margin the reporting door's telemetry pins.
    #[must_use]
    pub fn enclosure(&self) -> VolumeEnclosure<T> {
        // A body with no quadrature face has no pad, structurally rather
        // than as a measured zero; any other pad is the quadrature's.
        let pad = if self.volume_pad == 0.0 {
            T::zero()
        } else {
            T::from_computed(self.volume_pad)
        };
        VolumeEnclosure {
            volume_lo: self.volume - pad,
            volume_hi: self.volume + pad,
            surface_area: self.surface_area,
        }
    }
}

/// **A volume BRACKET and its lever** — what a certified quadrature
/// is entitled to say of a volume's sign.
///
/// There is no `volume` here, deliberately (D9 row 0): a quadrature
/// stopped early has an enclosure and no number, and a type that
/// offered one would be offering an arbitrary point of it. The two
/// ends are what a sign decision reads, and `surface_area` is the
/// V/A lever the +V invariant meters both against.
#[derive(Clone, Copy, Debug)]
pub struct VolumeEnclosure<T: Real> {
    /// The volume enclosure's lower end — definitely positive means
    /// the body's volume is definitely positive, at this round and at
    /// every finer one.
    pub volume_lo: T,
    /// The volume enclosure's upper end — the end the +V invariant
    /// refuses on (a thin positive volume inside a wide bracket must
    /// never refuse).
    pub volume_hi: T,
    /// The surface-area enclosure's midpoint: check 7's lever.
    pub surface_area: T,
}

/// The two states flattening one loop into [`LoopEdge`]s can reach —
/// [`loop_edges`]' own error, absorbed into [`MassPropsError`] by
/// `From` on this crate's path and matched exactly by the other
/// consumer (`mesh`'s shape door).
#[derive(Clone, Debug, PartialEq)]
pub enum LoopEdgesError {
    /// A loop is empty or a referenced key fails to resolve —
    /// tier-1/tier-2 scaffolding or corruption; the structural
    /// validators own the diagnosis, this is the fail-loud surface.
    Corrupt {
        /// What failed to resolve (static description).
        what: &'static str,
    },
    /// An edge is M3 null-edge scaffolding (no carrier by type): the
    /// body is mid-surgery.
    NullScaffoldEdge {
        /// The scaffolding edge.
        edge: crate::entity::EdgeKey,
    },
}

impl From<LoopEdgesError> for MassPropsError {
    fn from(e: LoopEdgesError) -> Self {
        match e {
            LoopEdgesError::Corrupt { what } => MassPropsError::Corrupt { what },
            LoopEdgesError::NullScaffoldEdge { edge } => MassPropsError::NullScaffoldEdge { edge },
        }
    }
}

/// Typed failure of [`mass_properties`] (closed enum, D4 ¶3).
#[derive(Clone, Debug, PartialEq)]
// The variant roster the sample-coverage row reads (test builds only).
#[cfg_attr(
    test,
    derive(strum::EnumDiscriminants),
    strum_discriminants(name(MassPropsErrorKind), vis(pub(crate)), derive(strum::EnumIter))
)]
pub enum MassPropsError {
    /// The run's tolerance cannot form a band.
    Band {
        /// The band construction failure.
        error: BandError,
    },
    /// A face's closed form failed: boundary outside the M2
    /// iso-rectangle inventory, a definite consistency failure, or an
    /// escalated classification.
    Face {
        /// The offending face.
        face: FaceKey,
        /// The per-face failure.
        source: PropsError,
    },
    /// A curved face carries interior rings and is neither a cone wall,
    /// a cylinder wall (its closed form and its quadrature both read
    /// every loop: `geom_brep::props::curved_face_loops`,
    /// `geom_brep::props::quad::cylinder_cut_face`), nor a torus wall
    /// bounded by rims and rulings, nor a torus wall any of whose loops
    /// carries a spiric (the spiric lane reads every loop,
    /// `geom_brep::props::spiric::rim_face_rounds`), nor a sphere face
    /// bounded by circles. A boolean pierce leaves a ring in the wall it
    /// pierces.
    RingOnCurvedFace {
        /// The offending face.
        face: FaceKey,
    },
    /// A loop is empty or a referenced key fails to resolve —
    /// tier-1/tier-2 scaffolding or corruption; the structural
    /// validators own the diagnosis, this is the fail-loud surface.
    Corrupt {
        /// What failed to resolve (static description).
        what: &'static str,
    },
    /// An edge is M3 null-edge scaffolding (no carrier by type — see
    /// `crate::null`): the body is mid-surgery, and mass properties are
    /// defined on at-rest bodies only (tier 2 refuses null entities).
    NullScaffoldEdge {
        /// The scaffolding edge.
        edge: crate::entity::EdgeKey,
    },
    /// The caller's [`Settle::Resolution`] is finer than ε: the model
    /// defines its measures only to ε of boundary displacement, so no
    /// finer resolution is offered.
    ResolutionFinerThanEps {
        /// The resolution asked for (m).
        resolution: f64,
        /// The run's ε (m).
        eps: f64,
    },
    /// The caller's [`Settle::Contains`] has its least volume definitely
    /// above its greatest: the claim names no volume, so there is
    /// nothing to decide.
    EmptyContains,
    /// A deciding predicate's margin was poisoned: no enclosure decides
    /// it.
    Poisoned {
        /// The poisoned decision.
        cause: Indeterminate,
    },
    /// **A deciding predicate still undecided where no face can refine**
    /// (`[tolerance.measure]`: the one quadrature refusal): the walk ran
    /// every face to its own stop and the enclosure still straddles the
    /// predicate's threshold. A [`Settle::Resolution`] left unmet is no
    /// refusal; it is the bracket, with its stop.
    Undecided {
        /// The volume's outward bracket `(lo, hi)` (m³) where the walk
        /// stopped: the true volume lies between them. It is
        /// `SignCertificate::outward_volume`, the one check 7's
        /// unresolved sign reports too — the faces' interval
        /// re-derivation, falling back to the walk's own enclosure stepped
        /// outward where no lane re-derives it.
        volume: (f64, f64),
        /// Why no face could refine further: [`Stop::Wall`] or
        /// [`Stop::Floor`].
        stop: Stop,
    },
}

impl fmt::Display for MassPropsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Band { error } => {
                write!(f, "the volume and surface area cannot be computed: {error}")
            }
            Self::Face { source, .. } => write!(
                f,
                "a face's share of the volume and surface area cannot be computed: {source}"
            ),
            Self::RingOnCurvedFace { .. } => write!(
                f,
                "the kernel cannot yet measure the volume of a curved face with a hole, \
                 other than a cone wall, a cylinder wall, a sphere face bounded by \
                 circles, or a torus wall bounded by its rims and meridians or cut by a \
                 plane. Recourse: move the cut so it crosses the face's edge rather than \
                 closing inside it"
            ),
            Self::Corrupt { what } => write!(
                f,
                "the body's structure is incomplete ({what}). {}",
                geom_core::KERNEL_OR_FILE_DEFECT_ENDING
            ),
            Self::NullScaffoldEdge { .. } => write!(
                f,
                "an edge is a placeholder a construction leaves while it is under way, which a \
                 body at rest never carries. {}",
                geom_core::KERNEL_OR_FILE_DEFECT_ENDING
            ),
            Self::ResolutionFinerThanEps { resolution, eps } => write!(
                f,
                "a measurement was asked for to {resolution:e} m, finer than the model's \
                 tolerance of {eps:e} m, which is the most it defines a measure to. Recourse: \
                 ask for a resolution of at least the tolerance"
            ),
            Self::EmptyContains => write!(
                f,
                "a measurement was asked whether the volume lies between a least and a \
                 greatest value, and the least is above the greatest, so no volume could. \
                 Recourse: give the least value first"
            ),
            Self::Poisoned { cause } => {
                write!(
                    f,
                    "the measurement's question could not be decided: {cause}"
                )
            }
            Self::Undecided {
                volume: (lo, hi),
                stop,
            } => {
                let at = match stop {
                    Stop::Wall => "a face reached its work wall",
                    _ => "refinement stopped paying",
                };
                write!(
                    f,
                    "the measurement's question is still undecided where no face can refine \
                     ({at}); the volume lies in [{lo:e}, {hi:e}] m³. {}",
                    geom_core::NOT_YET_ENDING
                )
            }
        }
    }
}

impl std::error::Error for MassPropsError {}

/// Volume and surface area of `body` over the exact B-rep (module
/// docs). Faces are visited in face-arena order; the accumulation
/// order is fixed (D9).
///
/// # Errors
///
/// [`MassPropsError`] — a misconfigured band, an out-of-inventory
/// face, a ring on a curved face no closed form reads, or unresolvable
/// structure.
///
/// **Not every valid body computes**, and the sentence that used to
/// stand here (*"bodies that pass the structural tiers and were built
/// by M2's public operations always compute"*) is falsified by #649's
/// own fixture: `merge_coplanar_faces` is a public operation that
/// moves no geometry and conserves χ, and running it on a body whose
/// cylindrical walls are authored as rectangular sub-faces produces a
/// structurally valid body carrying a plus-shaped iso domain, which
/// the closed forms refuse (`props_rim_level`, S58). A refusal here is
/// D2-addendum row 2 for that arm — valid input, lane not built — not
/// a claim about the body's integrity. See
/// [`ValidationError::VolumeUncomputable`](crate::ValidationError)'s
/// per-source breakdown.
///
/// **The certified measurement door.** Its quadrature is
/// [`QuadLane::certified`], handed to the walk by name, so the bound is
/// the quadrature's own and a scalar that may not certify cannot form
/// the call — there is no arm and no refusal, the call cannot be
/// written. [`mass_properties_structural`] is the same walk holding no
/// lane, and is the door such a scalar measures through:
///
/// ```
/// use geom_core::{Dual64, Tol};
/// use topo::{AtRestBody, mass_properties_structural};
/// fn measure(b: &AtRestBody<Dual64>, tol: Tol) {
///     let _ = mass_properties_structural(b, tol);
/// }
/// ```
///
/// ```compile_fail,E0277
/// use geom_core::{Dual64, Tol};
/// use topo::{AtRestBody, Settle, mass_properties};
/// fn measure(b: &AtRestBody<Dual64>, tol: Tol, settle: Settle<Dual64>) {
///     let _ = mass_properties(b, tol, settle);
/// }
/// ```
///
/// The two rows differ in the door and its predicate, and every path
/// either names resolves, so the second fails on the bound alone —
/// `E0277`, `CertifiedEnclosure` not implemented for `Dual<f64>`.
///
/// **It measures a finished body** (`&AtRestBody`, Ev's decision on
/// PR 4822): one tier 3 has passed, so the lanes read loops that bound
/// one region, shells wound as their roles say and structure that
/// resolves. A test whose subject is the walk on a body tier 3 refuses
/// measures through `topo::test_support::measure_unvalidated`, and says
/// why.
pub fn mass_properties<T: Decide + geom_core::CertifiedBounds>(
    body: &crate::AtRestBody<T>,
    tol: Tol,
    settle: Settle<T>,
) -> Result<MassMeasurement<T>, MassPropsError> {
    measure_body(body, tol, settle)
}

pub(crate) use raw_door::{
    classify_body, classify_body_of, classify_body_structural, measure_body,
    measure_body_structural,
};

/// The doors' walks over any body, finished or not: each typed door
/// ([`mass_properties`], [`mass_properties_structural`],
/// [`classify_shells`], [`classify_shells_structural`],
/// [`classify_shells_of`]) is one of these over a finished body. The
/// module is private, so its items leave the crate only through
/// `topo::test_support`'s re-exports (the raw doors), and carry this
/// seam's bounds there unrestated; each is
/// `pub` for those re-exports, so without test-support it is reachable
/// only inside the crate (the `allow(unreachable_pub)` on each).
pub(crate) mod raw_door {
    use super::*;

    /// [`mass_properties`]' walk over any body, finished or not: the
    /// raw door (`topo::test_support::measure_unvalidated`) a test
    /// measures a body tier 3 refuses through.
    ///
    /// # Errors
    ///
    /// As [`mass_properties`].
    #[cfg_attr(not(any(test, feature = "test-support")), allow(unreachable_pub))]
    pub fn measure_body<T: Decide + geom_core::CertifiedBounds>(
        body: &Body<T>,
        tol: Tol,
        settle: Settle<T>,
    ) -> Result<MassMeasurement<T>, MassPropsError> {
        let band = Band::linear(tol).map_err(|error| MassPropsError::Band { error })?;
        let deciding = !matches!(settle, Settle::Resolution(_));
        let faces = crate::query::all_faces(body);
        let lane = Some(QuadLane::certified());
        let (stop, certificate) = measure_to(body, &faces, band, tol, (lane, lane), settle)?;
        certificate.answer(deciding, stop)
    }

    /// [`mass_properties_structural`]'s closed-form walk over any body
    /// (`topo::test_support::measure_unvalidated_structural`).
    ///
    /// # Errors
    ///
    /// As [`mass_properties_structural`].
    #[cfg_attr(not(any(test, feature = "test-support")), allow(unreachable_pub))]
    pub fn measure_body_structural<T: Decide>(
        body: &Body<T>,
        tol: Tol,
    ) -> Result<MassProperties<T>, MassPropsError> {
        let band = Band::linear(tol).map_err(|error| MassPropsError::Band { error })?;
        mass_properties_closed_form(body, band, tol)
    }

    /// [`classify_shells`] over any body, finished or not
    /// (`topo::test_support::classify_unvalidated`).
    ///
    /// # Errors
    ///
    /// As [`classify_shells`].
    #[cfg_attr(not(any(test, feature = "test-support")), allow(unreachable_pub))]
    pub fn classify_body<T: Decide + geom_core::CertifiedBounds>(
        body: &Body<T>,
        tol: Tol,
    ) -> Result<Vec<ShellClassification<T>>, ShellClassifyError> {
        let every: Vec<ShellKey> = body.shells.iter().map(|(k, _)| k).collect();
        classify_body_of(body, &every, tol)
    }

    /// [`classify_shells_of`] over any body, finished or not
    /// (`topo::test_support::classify_unvalidated_of`).
    ///
    /// # Errors
    ///
    /// As [`classify_shells_of`].
    #[cfg_attr(not(any(test, feature = "test-support")), allow(unreachable_pub))]
    pub fn classify_body_of<T: Decide + geom_core::CertifiedBounds>(
        body: &Body<T>,
        shells: &[ShellKey],
        tol: Tol,
    ) -> Result<Vec<ShellClassification<T>>, ShellClassifyError> {
        classify_shells_through(body, shells, tol, Some(QuadLane::certified()))
    }

    /// [`classify_shells_structural`] over any body, finished or not
    /// (`topo::test_support::classify_unvalidated_structural`).
    ///
    /// # Errors
    ///
    /// As [`classify_shells_structural`].
    #[cfg_attr(not(any(test, feature = "test-support")), allow(unreachable_pub))]
    pub fn classify_body_structural<T: Decide>(
        body: &Body<T>,
        tol: Tol,
    ) -> Result<Vec<ShellClassification<T>>, ShellClassifyError> {
        let every: Vec<ShellKey> = body.shells.iter().map(|(k, _)| k).collect();
        classify_shells_through(body, &every, tol, None)
    }
}

/// [`mass_properties`] holding NO quadrature lane — the closed-form
/// walk, at the certified door's own signature and at plain
/// `T: Decide`: no bracket is read anywhere on this path, so no bracket
/// term is spelled. It IS `mass_properties_closed_form` with the band
/// built inside from `tol`, the one public door onto that walk. Every closed-form face
/// computes exactly as it does through the certified door, and a face
/// that needs the certified quadrature refuses typed rather than
/// passing unbounded, so on a closed-form body this door answers the
/// same volume and the same sign with pads of `0`. It is the door a
/// [`geom_core::Dual`] measures through, and it instantiates none of
/// the quadrature machinery.
///
/// # Errors
///
/// As [`mass_properties`], plus the closed form's typed refusal on
/// every face the quadrature lane would have enclosed.
pub fn mass_properties_structural<T: Decide>(
    body: &crate::AtRestBody<T>,
    tol: Tol,
) -> Result<MassProperties<T>, MassPropsError> {
    measure_body_structural(body, tol)
}

/// **A resolution the model defines**: `len` meters, refused typed where
/// it is finer than ε, the most the model defines a measure to.
fn resolution_len(len: Length, tol: Tol) -> Result<f64, MassPropsError> {
    let len = len.meters();
    if len >= tol.eps() {
        Ok(len)
    } else {
        Err(MassPropsError::ResolutionFinerThanEps {
            resolution: len,
            eps: tol.eps(),
        })
    }
}

/// **A resolution's stop over the faces as they stand**: [`Stop::Met`]
/// where every face meets `len`; else the first face, in the runs'
/// order, that a later round refused outright or whose kept enclosure
/// has no width to meter ([`FaceRun::width_len`]) is the refusal; else
/// why the rest could refine no further ([`SignCertificate::ran_out`]).
fn resolution_stop<T: Decide>(
    certificate: &SignCertificate<'_, T>,
    len: f64,
) -> Result<Stop, MassPropsError> {
    if certificate.runs.iter().all(|run| run.meets(len)) {
        return Ok(Stop::Met);
    }
    if let Some(refusal) = certificate.held_refusal() {
        return Err(refusal);
    }
    if let Some((face, Err(source))) = certificate
        .runs
        .iter()
        .map(|run| (run.face, run.width_len()))
        .find(|(_, width)| width.is_err())
    {
        return Err(MassPropsError::Face { face, source });
    }
    Ok(certificate.ran_out())
}

/// **The hook, one home**: the lane the caller holds, entered over
/// exactly the [`RoundWindow`] the walk asks for with the walk's
/// resolution, so a walk may stop between rounds and resume later — and
/// `Ok(None)`, the closed form, when the caller holds none.
///
/// With [`QuadLane::certified`] this is the certified quadrature; with
/// `None` every face is the closed form's, finished at round 0, and a
/// face that needed the quadrature refuses typed there ([`face_flux`]'s
/// `None` arm) — so a walk through this hook refuses exactly where any
/// other walk over the same lane does.
#[allow(clippy::type_complexity)]
fn round_hook<T: Decide>(
    quad: Option<QuadLane<T>>,
) -> impl Fn(
    &Body<T>,
    &Surface<T>,
    &[FlatLoop<T>],
    Band,
    Tol,
    RoundWindow,
    Point3<T>,
    Option<f64>,
) -> Result<Option<LaneRun>, PropsError>
+ Sync
+ Copy {
    move |body, surface, loops, band, tol, window, about: Point3<T>, resolution| match quad {
        Some(lane) => {
            let about = Point3::new(
                (lane.lift)(about.x),
                (lane.lift)(about.y),
                (lane.lift)(about.z),
            );
            (lane.cut_face_rounds)(
                body,
                surface,
                loops,
                band,
                tol,
                window,
                Some(about),
                resolution,
            )
            .map(Some)
        }
        None => Ok(None),
    }
}

/// **Every face to `len`**, each in one entry over its whole schedule:
/// a face's lane ends at the first round whose mean boundary
/// displacement is at most `len`, or at its own stop, keeping its
/// narrowest round. A face left short of `len` is no refusal: its
/// enclosure is sound, and the sum keeps it.
fn resolution_runs<T: Decide>(
    body: &Body<T>,
    faces: &[FaceKey],
    band: Band,
    tol: Tol,
    quad: Option<QuadLane<T>>,
    len: f64,
) -> Result<Vec<FaceRun<T>>, MassPropsError> {
    let hook = round_hook(quad);
    let refs = References::of(body, faces.iter().copied(), band, quad);
    decide_faces(faces, |&face_key| {
        face_flux(
            body,
            face_key,
            band,
            &hook,
            tol,
            RoundWindow::SCHEDULE,
            &refs,
            Some(len),
        )
    })
}

/// **The measurement door** (`[tolerance.measure]`): `faces` measured
/// through `measure` until `settle` resolves over their enclosure, until
/// refinement stops paying on every face it still needs, or until a
/// face it needs reaches the work wall. It answers the narrowest
/// enclosure each face reached, why the measurement stopped, and the
/// certificate the walk holds.
///
/// - [`Settle::Resolution`] is per face: each face refines until its own
///   width is at most the length, in one entry over its whole schedule.
///   [`Stop::Met`] when every face met it; otherwise [`Stop::Wall`] when a
///   face that did not reached its work wall, and [`Stop::Floor`].
/// - A deciding predicate is over the whole: round 0 for every face, then
///   every face its lane can refine a round further at a time
///   ([`SignCertificate::settle_on`]).
///
/// # Errors
///
/// [`MassPropsError`] where a face has no enclosure at all, a
/// predicate's margin is poisoned, or a resolution is finer than ε,
/// which the model defines no measure to.
pub(crate) fn measure_to<'b, T: Decide>(
    body: &'b Body<T>,
    faces: &[FaceKey],
    band: Band,
    tol: Tol,
    (measure, certify): (Option<QuadLane<T>>, Option<QuadLane<T>>),
    settle: Settle<T>,
) -> Result<(Stop, SignCertificate<'b, T>), MassPropsError> {
    if let Settle::Resolution(len) = settle {
        let len = resolution_len(len, tol)?;
        let runs = resolution_runs(body, faces, band, tol, measure, len)?;
        let certificate = SignCertificate::of_runs(body, band, tol, measure, runs);
        return Ok((resolution_stop(&certificate, len)?, certificate));
    }
    let mut certificate =
        SignCertificate::at_round_zero(body, faces, band, tol, measure, FaceWalk::Mapped)?;
    let stop = certificate.settle_on(certify, settle)?;
    Ok((stop, certificate))
}

/// [`SignCertificate::settle_on`]'s walk for a predicate read off the
/// volume enclosure ([`Settle::AtMost`], [`Settle::AtLeast`], [`Settle::Contains`]): every
/// refinable face a round further at a time, until `step` settles or
/// stops in the band, or no face can refine.
///
/// **A settlement is confirmed on the round re-derived in interval
/// arithmetic** ([`Round::interval_volume`]) wherever the walk holds a
/// lane to re-derive through, as a sign is ([`Round::certify`]): the
/// walk's own sums round at its scalar, so `step` reads the re-derived
/// enclosure again, its ends carried into the scalar as computed values
/// and metered over the area's upper end. A settlement the
/// re-derivation does not confirm refines on, and a re-derivation that
/// does not certify confirms nothing.
///
/// # Errors
///
/// [`MassPropsError`] where the re-derivation refuses a face, the
/// predicate's margin is poisoned, or the walk runs out undecided on a
/// face a later round refused outright.
fn deciding_walk<T: Decide>(
    certificate: &mut SignCertificate<'_, T>,
    certify: Option<QuadLane<T>>,
    step: impl Fn(VolumeEnclosure<T>) -> settle::Step,
) -> Result<Stop, MassPropsError> {
    let walked = walk_set(&mut [&mut *certificate], certify, |rounds| {
        let round = &rounds[0];
        let read = match step(round.props().enclosure()) {
            settle::Step::Settled(_) => match round.interval_volume() {
                Some(Ok((volume, area))) if volume.is_certified() && area.is_certified() => {
                    step(VolumeEnclosure {
                        volume_lo: T::from_computed(volume.lo()),
                        volume_hi: T::from_computed(volume.hi()),
                        surface_area: T::from_computed(area.hi()),
                    })
                }
                Some(Ok(_)) => settle::Step::Refine,
                Some(Err(source)) => return Ask::Decided(Err(source)),
                None => step(round.props().enclosure()),
            },
            read => read,
        };
        match read {
            settle::Step::Settled(holds) => {
                Ask::Decided(Ok(Stop::Settled(SettleVerdict::Holds(holds))))
            }
            settle::Step::Band => Ask::Decided(Ok(Stop::Band)),
            settle::Step::Poisoned(cause) => Ask::Decided(Err(MassPropsError::Poisoned { cause })),
            settle::Step::Empty => Ask::Decided(Err(MassPropsError::EmptyContains)),
            settle::Step::Refine => Ask::Refine(vec![true]),
        }
    });
    match walked {
        Walked::Decided(stop) => stop,
        Walked::Exhausted { refused } => match refused.into_iter().next() {
            Some((_, refusal)) => Err(refusal),
            None => Ok(certificate.ran_out()),
        },
    }
}

/// **What a set walk's question answers at a round** ([`walk_set`]).
pub(crate) enum Ask<V> {
    /// The set's question is decided: the walk returns this.
    Decided(V),
    /// Undecided: refine the members marked, one flag per member in the
    /// set's order. A member left unmarked stands where it is (its own
    /// question is decided).
    Refine(Vec<bool>),
}

/// **How a set walk ended** ([`walk_set`]).
#[derive(Debug)]
pub(crate) enum Walked<V> {
    /// The question decided.
    Decided(V),
    /// No member it asked to refine has a face its lane can refine.
    /// `refused` is each such member's first face, in its own order, that
    /// a later round refused outright and that stands at the enclosure it
    /// held: what the walk could not take further. Empty where every face
    /// ran to its own stop.
    Exhausted {
        /// `(member, refusal)`, in member order.
        refused: Vec<(usize, MassPropsError)>,
    },
}

/// **One walk over a set of certificates**: each round, `settle` reads
/// every member's enclosure as it stands ([`Round`], one per member in
/// the set's order) and answers whether the set's question is decided
/// ([`Ask`]). If not, every face its lane can refine, in every member it
/// marked, is taken one round further (its next round alone,
/// [`RoundWindow::at`]) and keeps the narrower of its two enclosures
/// ([`FaceRun::advance`]); and the walk asks again.
///
/// **A face a later round refuses outright** stands at the enclosure it
/// held, which is sound, and refines no further ([`FaceStop::Refused`]);
/// every other face of the round still advances, and the walk asks again
/// on what it holds. The refusal surfaces only where the walk then runs
/// out undecided ([`Walked::Exhausted`]), the decision being one it
/// could not make without that face.
///
/// A round is taken member by member in the set's order, each member's
/// faces one indexed map in its own order ([`decide_each`]), so the
/// recordings splice in that order.
pub(crate) fn walk_set<'b, T: Decide, V>(
    members: &mut [&mut SignCertificate<'b, T>],
    certify: Option<QuadLane<T>>,
    mut settle: impl FnMut(&[Round<'_, 'b, T>]) -> Ask<V>,
) -> Walked<V> {
    loop {
        let wants = {
            let rounds: Vec<Round<'_, 'b, T>> = members
                .iter()
                .map(|m| Round {
                    body: m.body,
                    band: m.band,
                    tol: m.tol,
                    certify,
                    runs: &m.runs,
                })
                .collect();
            match settle(&rounds) {
                Ask::Decided(verdict) => return Walked::Decided(verdict),
                Ask::Refine(wants) => wants,
            }
        };
        assert_eq!(
            wants.len(),
            members.len(),
            "a set walk's question marks one flag per member"
        );
        let mut moved = false;
        for (member, _) in members.iter_mut().zip(&wants).filter(|(_, want)| **want) {
            // Every refinable face: (slot, face, next round).
            let open: Vec<(usize, FaceKey, usize)> = member
                .runs
                .iter()
                .enumerate()
                .filter_map(|(slot, run)| run.next_round().map(|round| (slot, run.face, round)))
                .collect();
            if open.is_empty() {
                continue;
            }
            moved = true;
            let hook = round_hook(member.quad);
            let (body, band, tol) = (member.body, member.band, member.tol);
            // Each face about what round 0 read it about.
            let runs = &member.runs;
            let decided = decide_each(&open, |&(slot, face, round)| {
                face_flux_read(
                    body,
                    face,
                    band,
                    &hook,
                    tol,
                    RoundWindow::at(round),
                    runs[slot].about,
                    None,
                )
            });
            for (&(slot, _, _), run) in open.iter().zip(decided) {
                member.runs[slot].advance(run);
            }
        }
        if !moved {
            let refused = members
                .iter()
                .enumerate()
                .filter(|&(m, _)| wants[m])
                .filter_map(|(m, member)| member.held_refusal().map(|refusal| (m, refusal)))
                .collect();
            return Walked::Exhausted { refused };
        }
    }
}

/// **How a walk's faces are decided**: [`decide_faces`]' map, or on the
/// caller's thread ([`decide_faces_serially`], whose docs say when).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum FaceWalk {
    /// The face map.
    Mapped,
    /// The caller's thread, in the caller's order.
    Serial,
}

/// **A volume certificate at SIGN level** — the enclosure a tier-3
/// gate decided its +V invariant on, and the faces' rounds as the walk
/// left them.
///
/// The level is the TYPE (D9 row 0): there is no volume number to
/// read here, because a quadrature stopped at the round its caller's
/// certification was complete has not computed one. A caller that
/// wants the number asks for it under its own predicate —
/// [`Self::measure`] — and pays only the rounds that were not already
/// run.
///
/// **Its faces stand where the deciding walk left them, which is not
/// where the number would.** The walk takes every face its lane can
/// refine a round further while the decision is open ([`walk_set`]), a
/// face that has already met the number's resolution included, so a face
/// can stand past the round [`crate::mass_properties`] stops it at. Its
/// continuation keeps that narrower enclosure: the number it reaches is
/// never wider than the reporting door's, face by face, and is that
/// door's in its bits only where no face stood past the resolution.
///
/// # The refusal rule, stated once
///
/// **A reading of this certificate refuses as the door does**, and the
/// two must agree. A face refused outright (a degenerate lever, an
/// unsupported chart, a poisoned bracket) at round 0 leaves no enclosure
/// and no certificate; one refused at a later round stands at the
/// enclosure it held ([`FaceStop::Refused`]), and a reading that needs it
/// refuses, the first such face in arena order. A face whose lane stopped
/// short of a resolution still has a sound enclosure: it is read as data,
/// and the measurement's [`Stop`] says the resolution went unmet. Every
/// site below points here rather than restating it.
///
/// # The lane rides with it
///
/// A certificate is continued through the lane it was derived through
/// — [`QuadLane::certified`] from a certified door, `None` from a
/// `_structural` one — so [`Self::measure`] reads the same
/// quadrature the check read, at whatever scalar the door was called
/// at. A certificate derived with no lane has no round left open (every
/// face is closed-form, finished at round 0), so its continuation is
/// the fold of what it already holds.
pub struct SignCertificate<'b, T: Decide> {
    body: &'b Body<T>,
    band: Band,
    tol: Tol,
    quad: Option<QuadLane<T>>,
    runs: Vec<FaceRun<T>>,
}

impl<'b, T: Decide> SignCertificate<'b, T> {
    /// `faces` at round 0, in the caller's order: the walk's first
    /// reading, its faces decided as `walk` says. The faces' seam regions
    /// are read here, once.
    ///
    /// # Errors
    ///
    /// [`MassPropsError`] where a face has no enclosure.
    pub(crate) fn at_round_zero(
        body: &'b Body<T>,
        faces: &[FaceKey],
        band: Band,
        tol: Tol,
        quad: Option<QuadLane<T>>,
        walk: FaceWalk,
    ) -> Result<Self, MassPropsError> {
        let hook = round_hook(quad);
        let refs = References::of(body, faces.iter().copied(), band, quad);
        let run = |&face_key: &FaceKey| {
            face_flux(
                body,
                face_key,
                band,
                &hook,
                tol,
                RoundWindow::at(0),
                &refs,
                None,
            )
        };
        let runs = match walk {
            FaceWalk::Mapped => decide_faces(faces, run),
            FaceWalk::Serial => decide_faces_serially(faces, run),
        }?;
        Ok(Self::of_runs(body, band, tol, quad, runs))
    }

    /// The certificate holding `runs`.
    fn of_runs(
        body: &'b Body<T>,
        band: Band,
        tol: Tol,
        quad: Option<QuadLane<T>>,
        runs: Vec<FaceRun<T>>,
    ) -> Self {
        Self {
            body,
            band,
            tol,
            quad,
            runs,
        }
    }

    /// The first face, in the runs' order, that a later round refused
    /// outright and that stands at the enclosure it held
    /// ([`FaceStop::Refused`]).
    fn held_refusal(&self) -> Option<MassPropsError> {
        self.runs.iter().find_map(|run| run.refused().cloned())
    }

    /// The properties as they stand, as properties whose `volume_pad`
    /// and `area_pad` are its half-widths.
    pub(crate) fn props(&self) -> MassProperties<T> {
        fold_runs(&self.runs)
    }

    /// Why no face of this certificate can refine: [`Stop::Wall`] where a
    /// face reached its work wall, else [`Stop::Floor`].
    fn ran_out(&self) -> Stop {
        let walled = self.runs.iter().any(|run| {
            matches!(
                run.state,
                FaceState::Quad {
                    stop: FaceStop::Lane(LaneStop::Wall),
                    ..
                }
            )
        });
        if walled { Stop::Wall } else { Stop::Floor }
    }

    /// **A deciding predicate over this certificate's faces**, walked on
    /// from where they stand: every face its lane can refine a round
    /// further at a time ([`walk_set`]), until the predicate settles
    /// ([`Stop::Settled`]) or the enclosure lies wholly in its band
    /// ([`Stop::Band`]), or no face can refine ([`Stop::Wall`] or
    /// [`Stop::Floor`], by the stops the faces reached). [`Settle::Sign`]
    /// is [`role_walk_set`]'s, certified through `certify`.
    ///
    /// # Errors
    ///
    /// [`MassPropsError`] where a predicate's margin is poisoned, or a
    /// face the decision needs refused.
    ///
    /// # Panics
    ///
    /// On [`Settle::Resolution`], which decides nothing and is no walk's
    /// question: its callers size the faces instead.
    pub(crate) fn settle_on(
        &mut self,
        certify: Option<QuadLane<T>>,
        settle: Settle<T>,
    ) -> Result<Stop, MassPropsError> {
        let band = self.band;
        match settle {
            Settle::Resolution(len) => {
                unreachable!("a resolution of {len:?} sizes faces; it is no walk's question")
            }
            Settle::Sign(names) => {
                let Some(role) = role_walk_set(&mut [&mut *self], certify, names).pop() else {
                    unreachable!("a role walk over one member reads one role")
                };
                match role {
                    Ok(role) => Ok(Stop::Settled(SettleVerdict::Sign(role))),
                    Err(RoleWalkRefusal::Refused(refusal)) => Err(refusal),
                    Err(RoleWalkRefusal::Poisoned(last)) => {
                        let Some(cause) = last.escalation() else {
                            unreachable!("a poisoned reading holds an escalation")
                        };
                        Err(MassPropsError::Poisoned { cause })
                    }
                    Err(RoleWalkRefusal::Unread { in_band: true, .. }) => Ok(Stop::Band),
                    Err(RoleWalkRefusal::Unread { .. }) => Ok(self.ran_out()),
                }
            }
            Settle::AtMost { bound, name } => deciding_walk(self, certify, |e| {
                settle::relation_step(e, RelOp::Le, bound, name, band)
            }),
            Settle::AtLeast { bound, name } => deciding_walk(self, certify, |e| {
                settle::relation_step(e, RelOp::Ge, bound, name, band)
            }),
            Settle::Contains { lo, hi, name } => deciding_walk(self, certify, |e| {
                settle::contains_step(e, lo, hi, name, band)
            }),
        }
    }
}

/// **One round of a sign walk, as its `settle` reads it**: the walk's
/// own sums, and the same round re-derived in interval arithmetic.
///
/// The two are different claims. [`Self::reading`] is summed at the
/// walk's scalar, where at `f64` a closed-form face's flux and the fold
/// round with no pad to say so. [`Self::certify`] reads the role off the
/// re-derivation, which encloses the divergence sum about a corner of the
/// body over the stored geometry ([`rederive`], [`certify_role`]).
pub(crate) struct Round<'r, 'b, T: Decide> {
    body: &'b Body<T>,
    band: Band,
    tol: Tol,
    /// The lane the round's closed forms are re-derived through.
    certify: Option<QuadLane<T>>,
    runs: &'r [FaceRun<T>],
}

impl<T: Decide> Round<'_, '_, T> {
    /// The walk's own sums at this round.
    pub(crate) fn props(&self) -> MassProperties<T> {
        fold_runs(self.runs)
    }

    /// The walk's own sums at this round, as a role read takes them.
    pub(crate) fn reading(&self) -> SignReading<T> {
        self.props().reading()
    }

    /// This round re-derived in interval arithmetic ([`rederive`]),
    /// where the walk holds a lane to re-derive through.
    ///
    /// # Errors
    ///
    /// [`rederive`]'s.
    pub(crate) fn interval_volume(&self) -> Option<Result<(Interval, Interval), MassPropsError>> {
        let lane = self.certify?;
        Some(
            rederive(self.body, self.band, self.tol, lane, self.runs, true)
                .map(|r| (r.volume, r.area)),
        )
    }

    /// The role this round certifies ([`certify_role`]): off the round's
    /// interval re-derivation under `certified` where the walk holds a
    /// lane, and off its sums under `names` where it holds none.
    pub(crate) fn certify(&self, names: RoleNames, certified: RoleNames) -> Certified {
        certify_role(
            self.reading(),
            |tight| {
                self.certify
                    .map(|lane| rederived(self.body, self.band, self.tol, lane, self.runs, tight))
            },
            || rounding_bound(self.body, self.runs),
            names,
            certified,
        )
    }
}

/// **A volume as a role read takes it.**
#[derive(Clone, Copy, Debug)]
pub(crate) enum SignReading<U: Real> {
    /// One value, its sign read once and at both ends: a sum with no
    /// quadrature pad, or an interval, whose own width the decision
    /// already reads.
    Exact {
        /// The signed volume.
        volume: U,
        /// The surface area it is metered over.
        lever: U,
    },
    /// A quadrature bracket, read end by end.
    Bracket(VolumeEnclosure<U>),
}

impl<U: Real> SignReading<U> {
    /// One value and its lever, read once.
    pub(crate) fn exact(volume: U, lever: U) -> Self {
        Self::Exact { volume, lever }
    }

    /// `(low end, high end, lever)`: an exact volume's two ends are one.
    pub(crate) fn ends(self) -> (U, U, U) {
        match self {
            Self::Exact { volume, lever } => (volume, volume, lever),
            Self::Bracket(e) => (e.volume_lo, e.volume_hi, e.surface_area),
        }
    }
}

impl<T: Real> MassProperties<T> {
    /// This sum as a role read takes it: one value where no face carried
    /// a quadrature pad, else the bracket. One value is not an exact one:
    /// its rounding is the reader's to state ([`certify_role`]).
    pub(crate) fn reading(&self) -> SignReading<T> {
        if self.volume_pad == 0.0 {
            SignReading::exact(self.volume, self.surface_area)
        } else {
            SignReading::Bracket(self.enclosure())
        }
    }
}

/// The predicate names a role read decides under: the bracket's high
/// end (and an exact volume) under `high`, its low end under `low`.
#[derive(Clone, Copy, Debug)]
pub struct RoleNames {
    /// The name the high end, and an exact volume, are decided under.
    pub high: &'static str,
    /// The name the low end is decided under.
    pub low: &'static str,
}

impl RoleNames {
    /// One name for both ends — every decision but check 7's, which keeps
    /// its two historical names.
    #[must_use]
    pub const fn one(name: &'static str) -> Self {
        Self {
            high: name,
            low: name,
        }
    }
}

/// A role read that decided no role: the two ends' decisions (one and
/// the same for an exact volume).
#[derive(Clone, Copy, Debug)]
pub(crate) struct RoleUnread {
    lo: Result<Decided, Indeterminate>,
    hi: Result<Decided, Indeterminate>,
}

impl RoleUnread {
    /// The escalation either end hit, the high end's first.
    pub(crate) fn escalation(&self) -> Option<Indeterminate> {
        self.hi.err().or(self.lo.err())
    }

    /// Whether an end's margin was poisoned (a NaN): no enclosure decides
    /// it.
    pub(crate) fn poisoned(&self) -> bool {
        self.escalation()
            .is_some_and(|cause| cause.margin.is_invalid())
    }

    /// Whether this read is a zero volume, a verdict: both its ends
    /// decided zero. The ends are the certified reading's wherever one
    /// was taken, and otherwise the walk's, which then decide (a walk
    /// holding no lane, [`certify_role`]). The true volume lies between
    /// the ends, so only two ends in the zero band place it there; an end
    /// decided off zero with no role read (a bracket straddling zero, or
    /// reaching from zero to one side) is a volume left unmeasured, not a
    /// zero one.
    pub(crate) fn reads_zero(&self) -> bool {
        let zero = |end: Result<Decided, Indeterminate>| end.is_ok_and(|d| d.sign == Sign::Zero);
        zero(self.lo) && zero(self.hi)
    }

    /// `shell`'s refusal for this read, its ends picking the words
    /// ([`shell_role_refusal`]).
    pub(crate) fn refusal(self, shell: ShellKey) -> ShellClassifyError {
        shell_role_refusal(shell, self.lo, self.hi)
    }
}

/// **The bandless sign read** (D10): `reading`'s ends read for their
/// sign alone ([`Margin::sign_of`] against [`Band::enclosure_sign`]), the
/// high end first, the low end only where it decides nothing
/// ([`ShellRole::decided_at`]). No length is held against a tolerance:
/// a certified enclosure on one side of zero is that role however thin
/// the body. The margins it records are the volume's own.
pub(crate) fn read_sign<U: Decide>(reading: SignReading<U>, names: RoleNames) -> RoleRead {
    let sign = |name, volume| {
        crate::validate::decide_reported(name, Margin::sign_of(volume), Band::enclosure_sign())
    };
    let role_at = |end, decided: Result<Decided, Indeterminate>| {
        decided
            .ok()
            .and_then(|decided| ShellRole::decided_at(end, decided.sign))
    };
    let read = |role: Option<ShellRole>, unread| match role {
        Some(role) => RoleRead::Decided(role),
        None => RoleRead::Unread(unread),
    };
    match reading {
        SignReading::Exact { volume, .. } => {
            let one = sign(names.high, volume);
            read(
                role_at(BracketEnd::Low, one).or_else(|| role_at(BracketEnd::High, one)),
                RoleUnread { lo: one, hi: one },
            )
        }
        SignReading::Bracket(ends) => {
            let hi = sign(names.high, ends.volume_hi);
            if let Some(role) = role_at(BracketEnd::High, hi) {
                return RoleRead::Decided(role);
            }
            let lo = sign(names.low, ends.volume_lo);
            read(role_at(BracketEnd::Low, lo), RoleUnread { lo, hi })
        }
    }
}

/// What [`read_sign`] read.
#[derive(Clone, Copy, Debug)]
pub(crate) enum RoleRead {
    /// An end decided the role.
    Decided(ShellRole),
    /// No end did.
    Unread(RoleUnread),
}

/// What a certified role read concluded.
#[derive(Clone, Debug)]
pub(crate) enum Certified {
    /// The role, decided on the exact value.
    Role(ShellRole),
    /// No role: the certified enclosure holds zero, taken about the body
    /// with every face in closed form, so its width is the body's own
    /// rounding — the kernel's or the file's, since a body whose pairs
    /// are definite holds a volume clear of that rounding.
    Open(RoleUnread),
    /// No role, and the read's width is a limit of how it was measured,
    /// not the body's ([`SignLimit`]): the sign is unresolved.
    Unresolved(RoleUnread, SignLimit),
    /// The interval re-derivation refused a face.
    Refused(MassPropsError),
    /// No role: an end's margin was poisoned ([`RoleUnread::poisoned`]),
    /// which no enclosure decides.
    Poisoned(RoleUnread),
}

/// **Why a volume's sign is unresolved where the body is not at fault**:
/// a limit of how the volume was measured, which no change to the part
/// is known to move.
///
/// **NOT carried to the façade's curated list, and that is a decision
/// rather than an omission** (`scripts/payload-rung-sweep.py` names this
/// rung; the disposition table cites this paragraph as its home). A Rust
/// caller can already name and match it — `pncad` re-exports `topo`
/// whole — and the Python door says which refusal it rides in
/// (`sign_unresolved`, `volume_sign_unresolved`) and carries its
/// sentence, which names the limit. Publishing it owes the CUR3 property
/// row `carried_refusal_payloads_are_matchable_through_the_prelude` and a
/// Python word per arm, both the façade crate's to write.
///
/// **The falsifier is a Python caller who must tell a lane-free scalar's
/// limit from a quadrature's** without reading the sentence. When the
/// façade carries the reading, delete this paragraph and the disposition
/// row with it.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum SignLimit {
    /// A face that could not be read about its shell's own point kept
    /// the width of its read about the world origin, so the enclosure's
    /// width may be the body's distance from the origin.
    WorldOrigin,
    /// A curved face's quadrature enclosure was still open at the last
    /// round its schedule runs.
    Quadrature,
    /// The scalar holds no interval lane, so its sums are read against a
    /// stated bound on their rounding ([`rounding_bound`]), and the volume
    /// lies within it.
    Scalar,
}

impl SignLimit {
    /// What left the sign unresolved, in words, after "the sign of … volume
    /// cannot be read: ".
    #[must_use]
    pub fn words(self) -> &'static str {
        match self {
            Self::WorldOrigin => {
                "its enclosure holds zero, and a face that could not be measured about the \
                 body itself keeps the width it was measured with far from the world origin"
            }
            Self::Quadrature => {
                "its enclosure still holds zero at the last round a curved face's \
                 quadrature runs"
            }
            Self::Scalar => {
                "this scalar carries no interval to certify its sums, and the volume lies \
                 within their rounding"
            }
        }
    }

    /// The refusal's ending: a capability limit, never a defect.
    #[must_use]
    pub fn ending(self) -> &'static str {
        match self {
            Self::WorldOrigin => "Recourse: model the part nearer the origin",
            Self::Quadrature | Self::Scalar => geom_core::NOT_YET_ENDING,
        }
    }
}

/// **A role, certified**: read off `interval` — the walk's faces
/// re-derived in interval arithmetic — wherever the walk holds a lane,
/// and off the walk's own sums `reading` only where it holds none.
///
/// **The sign alone decides** (D10: the volume's sign is certified by its
/// enclosure alone, never held against the band): every read is
/// [`read_sign`]'s, so an enclosure wholly on one side of zero is that
/// role however thin the body.
///
/// - **With a lane**, the re-derivation encloses the divergence sum of
///   the stored geometry about a vertex of the body ([`rederive`] says
///   which value). One that holds zero is [`Certified::Open`] where every
///   face was taken about the body in closed form, its width the body's
///   own rounding, and [`Certified::Unresolved`] where a face kept its
///   world-origin width or a quadrature face its last round's
///   ([`SignLimit`]).
/// - **With none** (a scalar with no interval lane, or a structural door),
///   the walk's sums are read as the enclosure `[V − b, V + b]`, `b` the
///   stated bound on their rounding (`rounding`, [`rounding_bound`]);
///   one that holds zero is [`SignLimit::Scalar`], a capability limit.
///   Never `V/A` against the run's band
///   (`work/tally/lane-free-volume-sign-reads-decide-on-a-rounded-sum.md`).
///
/// `interval(tight)` is read in two stages, so a decision costs one read
/// of each face wherever one suffices. Untight (`false`), a quadrature
/// face is the walk's own enclosure less `c · A⃗`, no new quadrature; a
/// role it decides stands, the enclosure being sound. Only where it
/// leaves the sign open at a world-origin width is `interval(true)` read,
/// which measures those faces again about the vertex.
pub(crate) fn certify_role<T: Decide>(
    reading: SignReading<T>,
    interval: impl Fn(bool) -> Option<IntervalRead>,
    rounding: impl FnOnce() -> Result<T, MassPropsError>,
    names: RoleNames,
    certified: RoleNames,
) -> Certified {
    let read = |got: Option<IntervalRead>| {
        got.map(|got| match got {
            Err(source) => Certified::Refused(source),
            Ok((exact, limit)) => match (read_sign(exact, certified), limit) {
                (RoleRead::Decided(role), _) => Certified::Role(role),
                (RoleRead::Unread(unread), _) if unread.poisoned() => Certified::Poisoned(unread),
                (RoleRead::Unread(unread), None) => Certified::Open(unread),
                (RoleRead::Unread(unread), Some(limit)) => Certified::Unresolved(unread, limit),
            },
        })
    };
    let lane_free = || match rounding() {
        Err(source) => Certified::Refused(source),
        Ok(bound) => {
            let (lo, hi, lever) = reading.ends();
            let widened = SignReading::Bracket(VolumeEnclosure {
                volume_lo: lo - bound,
                volume_hi: hi + bound,
                surface_area: lever,
            });
            match read_sign(widened, names) {
                RoleRead::Decided(role) => Certified::Role(role),
                // An end that could not be read (a NaN sum) is no rounding
                // the bound covers: the sums are the body's or the kernel's.
                RoleRead::Unread(unread) if unread.poisoned() => Certified::Poisoned(unread),
                RoleRead::Unread(unread) if unread.escalation().is_some() => {
                    Certified::Open(unread)
                }
                RoleRead::Unread(unread) => Certified::Unresolved(unread, SignLimit::Scalar),
            }
        }
    };
    match read(interval(false)) {
        Some(Certified::Unresolved(_, SignLimit::WorldOrigin)) => {
            read(interval(true)).unwrap_or_else(lane_free)
        }
        Some(certified) => certified,
        None => lane_free(),
    }
}

/// **The stated bound on a lane-free walk's rounding** ([`certify_role`]'s
/// `b`): `ROUNDING_TERMS · u · Σ_f (n_f + 1) · R_f · D_f²` over the walk's
/// faces, `u` the unit roundoff and `n_f` a face's edge count. A face's
/// closed form is taken about its surface's anchor (a plane's or a
/// cylinder's origin, a cone's apex, a sphere's or a torus's centre), on
/// its geometry moved by `−c`, `c` its shell's reference
/// ([`shell_reference`]): `R_f` is the larger 1-norm of the anchor and of
/// the face's loop points about `c` (their distance from the point the
/// face is read about, from above), and `D_f` the larger of the loop
/// points' box and their 1-norm distance from the anchor (the spread of
/// the vectors the closed form crosses, from above).
///
/// The model: a face's flux is a sum over its edges of cross products of
/// vectors `D_f` long, each rounding by about `u · D_f²`, levered by a
/// point `R_f` from `c`; moving a point by `−c` rounds by `u` of the moved
/// point, which `R_f` already counts; the fold adds one rounding per
/// face.
/// `ROUNDING_TERMS` covers the few rounded operations of each term. It is
/// a stated bound, not a certificate: a scalar with no interval lane has
/// none to give, and what this read certifies is only that the sums' sign
/// is not within the rounding the model puts on them.
fn rounding_bound<T: Decide>(body: &Body<T>, runs: &[FaceRun<T>]) -> Result<T, MassPropsError> {
    let one_norm = |x: T, y: T, z: T| x.abs() + y.abs() + z.abs();
    let mut bound = T::zero();
    for run in runs {
        let (face, surface) = resolve_face(body, run.face);
        let c = run.about.point();
        let about = |p: Point3<T>| one_norm(p.x - c.x, p.y - c.y, p.z - c.z);
        let anchor = match *surface {
            Surface::Plane { origin, .. } | Surface::Cylinder { origin, .. } => Some(origin),
            Surface::Cone { apex, .. } => Some(apex),
            Surface::Sphere { center, .. } | Surface::Torus { center, .. } => Some(center),
            Surface::Nurbs(_) | Surface::Approx(_) => None,
        };
        // One plus the face's edge count, kept in `T`.
        let mut terms = T::one();
        let mut reach = anchor.map_or(T::zero(), about);
        let mut spread = T::zero();
        let mut box_lo: Option<Point3<T>> = None;
        let mut box_hi: Option<Point3<T>> = None;
        for lp in face_loops(body, face)? {
            for e in &lp {
                terms = terms + T::one();
                for p in [e.carrier.eval(e.t0), e.carrier.eval(e.t1)] {
                    reach = reach.max(about(p));
                    if let Some(a) = anchor {
                        spread = spread.max(one_norm(p.x - a.x, p.y - a.y, p.z - a.z));
                    }
                    box_lo = Some(
                        box_lo.map_or(p, |b| Point3::new(b.x.min(p.x), b.y.min(p.y), b.z.min(p.z))),
                    );
                    box_hi = Some(
                        box_hi.map_or(p, |b| Point3::new(b.x.max(p.x), b.y.max(p.y), b.z.max(p.z))),
                    );
                }
            }
        }
        if let (Some(lo), Some(hi)) = (box_lo, box_hi) {
            let spread = spread.max(one_norm(hi.x - lo.x, hi.y - lo.y, hi.z - lo.z));
            bound = bound + terms * reach * spread.powi(2);
        }
    }
    Ok(bound * T::from_f64(ROUNDING_TERMS * f64::EPSILON * 0.5))
}

/// An interval re-derivation as a role read takes it: its volume, and
/// what an enclosure of it holding zero is a limit of ([`Rederived`]).
pub(crate) type IntervalRead = Result<(SignReading<Interval>, Option<SignLimit>), MassPropsError>;

/// [`rounding_bound`]'s allowance for the rounded operations of one term.
const ROUNDING_TERMS: f64 = 16.0;

/// [`rederive`] as a role read takes it: one interval, read once.
fn rederived<T: Decide>(
    body: &Body<T>,
    band: Band,
    tol: Tol,
    lane: QuadLane<T>,
    runs: &[FaceRun<T>],
    tight: bool,
) -> Result<(SignReading<Interval>, Option<SignLimit>), MassPropsError> {
    rederive(body, band, tol, lane, runs, tight)
        .map(|r| (SignReading::exact(r.volume, r.area), r.limit()))
}

/// The interval signature of a closed form read about a centre
/// ([`QuadLane`]'s `closed_form`).
type ClosedFormAt<T> = fn(
    &Surface<T>,
    &[Vec<LoopEdge<T>>],
    bool,
    Band,
    Point3<Interval>,
    Option<((Interval, Interval), Point3<Interval>)>,
) -> Result<(FaceContribution<Interval>, bool), PropsError>;

/// **A closed-form face's flux about `from`, its shell's reference**, as
/// [`rederive_faces`] reads it, and whether it was read there
/// (recentred). The closed form read again about `from` decides its
/// inventory again there, at the interval's rounding about that point,
/// and may read it in band where the walk decided it: the face is then
/// read about the world origin, where a refusal the face itself earns
/// refuses, and carried to `from` by its vector area, at the world-origin
/// width. A closed-form face is bounded by no spiric, so its loops'
/// vector area has a closed form.
fn closed_form_or_carried<T: Decide>(
    closed_form: ClosedFormAt<T>,
    surface: &Surface<T>,
    loops: &[Vec<LoopEdge<T>>],
    sense: bool,
    band: Band,
    from: Point3<Interval>,
) -> Result<(FaceContribution<Interval>, bool), PropsError> {
    if let Ok(read) = closed_form(surface, loops, sense, band, from, None) {
        return Ok(read);
    }
    let (origin, _) = closed_form(surface, loops, sense, band, Point3::origin(), None)?;
    let (carried, _) = closed_form(
        surface,
        loops,
        sense,
        band,
        from,
        Some(((origin.flux, origin.area), Point3::origin())),
    )?;
    Ok((carried, false))
}

/// **A walk's runs re-derived in interval arithmetic**, each face about
/// the vertex `c` the walk read it about ([`corner_of`]: its shell's, or
/// its carried quadrature region's, whose first face takes the region's
/// carry), `(volume, surface area)`,
/// whether every face was taken about its `c`, and whether a face was
/// read off its quadrature ([`Rederived`]).
///
/// **What the volume encloses**:
/// - **a shell of planes bounded by lines** ([`shell_polygons`]): the
///   exact volume of the polyhedron its vertex points span, summed with
///   no rounding ([`quad_lane::exact_polygon_volume`]), as an interval a
///   few ulps wide; where a point is not exact, each face's plane distance
///   times its polygon's area, widened by the polygon's tilt off its plane
///   ([`quad_lane::polygon_face_about`]), which encloses the same value;
/// - **otherwise** the divergence sum about `c` of the faces as stored:
///   each closed-form face's flux about `c` from its surface and loops
///   lifted exactly and moved by `−c` (a plane as its distance from `c`
///   along its stored normal times its loops' signed area, widened by the
///   loops' tilt off the plane, so the term encloses the loops' own flux:
///   `quad_lane::planar_face_about`), and each quadrature face measured
///   again about `c`, at the round the walk reached
///   (`quad_lane::cut_face_rounds`: a cylinder's position term taken as
///   `(origin − c)·A⃗`, a patch's control net carried by `−c`). A loop
///   closes only to the rounding of its carriers' ends, and a fan reads
///   that gap at the face's length times its lever
///   (`work/tally/a-fan-over-carrier-ends-reads-an-ulp-gap-at-the-faces-length-times-its-lever`).
///
/// Taken about a point of its shell, no face's width is scaled by the
/// body's distance from the world origin. A quadrature face whose lane
/// refuses about `c` keeps the enclosure the walk measured, which is
/// about `c` already; a closed-form face that refuses about `c` (its
/// inventory read again there in band) is read about the world origin
/// and carried to `c` by its vector area, unrecentred. A walk with no
/// faces is the zero sum, unrecentred.
///
/// # Errors
///
/// The property layer's refusal of a face whose closed form does not
/// hold at the interval scalar — the face's geometry read with no
/// rounding to hide behind, which can leave a classification open that
/// the walk's scalar decided.
fn rederive<T: Decide>(
    body: &Body<T>,
    band: Band,
    tol: Tol,
    lane: QuadLane<T>,
    runs: &[FaceRun<T>],
    tight: bool,
) -> Result<Rederived, MassPropsError> {
    if runs.is_empty() {
        return Ok(Rederived {
            volume: Interval::zero(),
            area: Interval::zero(),
            recentred: false,
            quadrature: false,
        });
    }
    if let Some(polygons) = shell_polygons(body, lane, runs)? {
        return rederive_polygons(lane, &polygons, runs);
    }
    rederive_faces(body, band, tol, lane, runs, tight)
}

/// [`rederive`] face by face, each about the point `c` the walk read it
/// about ([`corner_of`]), which every run's enclosure is about already,
/// and each carried region's first face with its carry ([`References`]).
fn rederive_faces<T: Decide>(
    body: &Body<T>,
    band: Band,
    tol: Tol,
    lane: QuadLane<T>,
    runs: &[FaceRun<T>],
    tight: bool,
) -> Result<Rederived, MassPropsError> {
    let (mut flux, mut area) = (Interval::zero(), Interval::zero());
    let mut recentred = true;
    let mut quadrature = false;
    for run in runs {
        let (face, surface) = resolve_face(body, run.face);
        let flat = flat_loops(body, face)?;
        let loops = edges_of(&flat);
        let refused = |source| MassPropsError::Face {
            face: run.face,
            source,
        };
        let from = corner_of(lane, run);
        #[cfg(any(test, feature = "test-support"))]
        read_points::note(
            body,
            run.face,
            matches!(run.about, ReadAbout::Region { .. }),
        );
        // A carried region's first face takes its carry on every read
        // below but the walk's own enclosure, which holds it already.
        let carry = run.about.carry().unwrap_or_else(Interval::zero);
        let (f, a) = match run.contribution.enclosure {
            Some(enclosure) => {
                // The quadrature again, about `from`, at the round the
                // walk reached; failing that, the walk's own enclosure,
                // which is about `from` already.
                let FaceState::Quad { taken, .. } = &run.state else {
                    unreachable!("a face holding a quadrature enclosure is in a quadrature state")
                };
                let taken = *taken;
                let rerun = if tight {
                    (lane.cut_face_rounds)(
                        body,
                        surface,
                        &flat,
                        band,
                        tol,
                        RoundWindow::at(taken),
                        Some(from),
                        None,
                    )
                } else {
                    Err(PropsError::QuadratureUnsupported {
                        what: "the untight read keeps the walk's enclosure",
                    })
                };
                quadrature = true;
                match rerun {
                    Ok(LaneRun { bounds: b, .. }) => (b.flux + carry, b.area),
                    Err(_) => enclosure,
                }
            }
            None => {
                let (c, moved) = closed_form_or_carried(
                    lane.closed_form,
                    surface,
                    &loops,
                    face.sense,
                    band,
                    from,
                )
                .map_err(refused)?;
                recentred &= moved;
                (c.flux + carry, c.area)
            }
        };
        flux = flux + f;
        area = area + a;
    }
    Ok(Rederived {
        volume: flux / Interval::from_f64(3.0),
        area,
        recentred,
        quadrature,
    })
}

/// A face's loops as their vertex points, in traversal order, the outer
/// loop first ([`vertex_rings`]).
type Rings = Vec<Vec<Point3<Interval>>>;

/// A plane face read as its polygon: its [`Rings`] and its plane's
/// stored normal, lifted.
type PlanePolygon = (Rings, geom_core::Vec3<Interval>);

/// **A shell read as the polygons of its vertex points**: every run's
/// face as its loops' vertex points in traversal order, lifted through
/// `lane` ([`vertex_rings`]), with its plane's stored normal, or `None`
/// where any face is not a plane bounded by lines. All or nothing: a line edge read as its
/// vertices on one side and as its carrier's ends on the other would
/// open a gap between the two readings.
///
/// The volume read off these polygons is the polyhedron's their points
/// span ([`rederive_polygons`]). A face's plane distance times its
/// polygon's area is not that polyhedron's flux: it drops the in-plane
/// part of the anchor's lever against the polygon's tilt off the plane,
/// which only the closed shell cancels, so a turned needle's terms once
/// missed its exact volume by 1.4% at three hundred thousand widths
/// (review R2 of PR 5119's second delta, measured in exact rationals);
/// a face's own term is widened by that tilt where it is read at all
/// ([`quad_lane::polygon_face_about`]).
fn shell_polygons<T: Decide>(
    body: &Body<T>,
    lane: QuadLane<T>,
    runs: &[FaceRun<T>],
) -> Result<Option<Vec<PlanePolygon>>, MassPropsError> {
    let lift = |v: geom_core::Vec3<T>| {
        geom_core::Vec3::new((lane.lift)(v.x), (lane.lift)(v.y), (lane.lift)(v.z))
    };
    let mut polygons = Vec::with_capacity(runs.len());
    for run in runs {
        let (face, surface) = resolve_face(body, run.face);
        match (
            vertex_rings(body, surface, &flat_loops(body, face)?, lane)?,
            surface,
        ) {
            (Some(rings), &Surface::Plane { normal, .. }) => polygons.push((rings, lift(normal))),
            _ => return Ok(None),
        }
    }
    Ok(Some(polygons))
}

/// [`rederive`] over [`shell_polygons`]: the exact volume of the
/// polyhedron the faces' vertex points span
/// ([`quad_lane::exact_polygon_volume`]) wherever every point is exact;
/// otherwise each face's flux about its shell's `c` ([`corner_of`]), its
/// plane's distance times
/// its polygon's signed area widened by the polygon's tilt off the plane
/// ([`quad_lane::polygon_face_about`]). The area is the faces' own,
/// either way.
fn rederive_polygons<T: Decide>(
    lane: QuadLane<T>,
    polygons: &[PlanePolygon],
    runs: &[FaceRun<T>],
) -> Result<Rederived, MassPropsError> {
    let (mut flux, mut area) = (Interval::zero(), Interval::zero());
    for ((rings, normal), run) in polygons.iter().zip(runs) {
        let centre = corner_of(lane, run);
        let c = quad_lane::polygon_face_about(rings, *normal, centre).map_err(|source| {
            MassPropsError::Face {
                face: run.face,
                source,
            }
        })?;
        flux = flux + c.flux;
        area = area + c.area;
    }
    let rings: Vec<Rings> = polygons.iter().map(|(rings, _)| rings.clone()).collect();
    Ok(Rederived {
        volume: quad_lane::exact_polygon_volume(&rings)
            .unwrap_or_else(|| flux / Interval::from_f64(3.0)),
        area,
        recentred: true,
        quadrature: false,
    })
}

/// A planar face bounded by lines, as its loops' vertex points in
/// traversal order, lifted through `lane`; `None` for any other face.
fn vertex_rings<T: Decide>(
    body: &Body<T>,
    surface: &Surface<T>,
    loops: &[FlatLoop<T>],
    lane: QuadLane<T>,
) -> Result<Option<Rings>, MassPropsError> {
    let lines = loops
        .iter()
        .flat_map(|(edges, _)| edges)
        .all(|e| matches!(e.carrier, geom::Curve3::Line { .. }));
    if !matches!(surface, Surface::Plane { .. }) || !lines {
        return Ok(None);
    }
    let lift = |p: Point3<T>| Point3::new((lane.lift)(p.x), (lane.lift)(p.y), (lane.lift)(p.z));
    loops
        .iter()
        .map(|(_, hes)| {
            hes.iter()
                .map(|&he| {
                    body.half_edge_start_point(he)
                        .map(lift)
                        .ok_or(MassPropsError::Corrupt {
                            what: "a loop's half-edge has no start point",
                        })
                })
                .collect()
        })
        .collect::<Result<_, _>>()
        .map(Some)
}

/// A walk's runs re-derived in interval arithmetic ([`rederive`]).
#[derive(Clone, Copy, Debug)]
struct Rederived {
    /// The volume, about the centre it was taken at.
    volume: Interval,
    /// The surface area.
    area: Interval,
    /// Whether every face was taken about the body's own centre, so the
    /// width is the body's own rounding: `false` where a quadrature face
    /// whose lane refused about the centre (or a face whose geometry has
    /// no translated twin) kept the world-origin width
    /// ([`quad_lane::closed_form`]).
    recentred: bool,
    /// Whether a face was read off its quadrature's last round rather
    /// than in closed form.
    quadrature: bool,
}

impl Rederived {
    /// What an enclosure holding zero is a limit of, if not the body's
    /// own rounding: a world-origin width first, then a quadrature's.
    fn limit(&self) -> Option<SignLimit> {
        if !self.recentred {
            Some(SignLimit::WorldOrigin)
        } else if self.quadrature {
            Some(SignLimit::Quadrature)
        } else {
            None
        }
    }
}

/// **The reference point the re-derivation takes the flux about**: the
/// face's shell's ([`shell_reference`]), or its carried quadrature
/// region's ([`References`]), a vertex of the body, lifted
/// through `lane` (a point interval at `f64`). A point of the body lies
/// within the body's extent along each face's normal of that face's
/// plane: a needle's or a slab's thickness for its long faces. A plane's
/// flux is its distance times its area ([`quad_lane::polygon_face_about`]),
/// so this keeps each long face's term the size of its own share of the
/// volume. And its coordinates are the body's own, so moving a face's
/// geometry by it rounds no more than the body's coordinates do: a
/// closed form's decision at the band's edge reads as it reads about the
/// body's own points.
///
/// The least corner of the loop points, which this was, lies off a body
/// turned off the axes by up to its length: on a needle 2e-11 across
/// turned at ε 1e-12, its long faces' terms read ~6e-12 each with ~2e-16
/// of width, cancelling in pairs to a volume of 4e-22 inside an
/// enclosure that held zero. The mean of the points lies on the body but
/// between its coordinates, and moving a disc's wall by it read its
/// height, exactly `Kε`, across the band's edge.
///
/// Any fixed point is a sound centre for a closed surface; this is the
/// same point for one stored body every time (D9), and two bodies
/// storing one boundary in different orders take different vertices. The
/// value about it also reads a plane's flux from its loop's first point,
/// and where a loop's points stand off their plane two such bodies
/// re-derive values that differ by up to `Σ_f δ_f·(Σ|T^a| + Σ|T^b|)` over
/// those faces, `|T^a|` and `|T^b|` the unsigned areas of face `f`'s fan
/// triangles from either first point. On the polygon route
/// ([`shell_polygons`]) that is the whole difference; on the fan route
/// the loops' gaps at their carrier ends add their own
/// (`work/tally/a-fan-over-carrier-ends-reads-an-ulp-gap-at-the-faces-length-times-its-lever`).
fn corner_of<T: Decide>(lane: QuadLane<T>, run: &FaceRun<T>) -> Point3<Interval> {
    let c = run.about.point();
    Point3::new((lane.lift)(c.x), (lane.lift)(c.y), (lane.lift)(c.z))
}

impl<T: Decide> fmt::Debug for SignCertificate<'_, T> {
    /// The certificate, not the body it reads: the bracket and the rounds
    /// its faces can still run. Both patterns tie the render to the
    /// fields: one added to either type is an E0027 here.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self {
            // What the certificate reads and was derived against, not
            // what it holds.
            body: _,
            band: _,
            tol: _,
            quad: _,
            runs,
        } = self;
        let VolumeEnclosure {
            volume_lo,
            volume_hi,
            surface_area,
        } = self.enclosure();
        write!(
            f,
            "SignCertificate: volume in [{volume_lo:?}, {volume_hi:?}], surface area \
             {surface_area:?}, rounds still open {:?}",
            // The rounds that REMAIN: `None` is a finished walk, which is
            // what a certificate stopped at round 0 looks like too.
            runs.iter().filter_map(FaceRun::open_at).max(),
        )
    }
}

impl<'b, T: Decide> SignCertificate<'b, T> {
    /// **The body's certificate, assembled from its parts'** — the
    /// runs of certificates taken over disjoint face sets, re-ordered
    /// into FACE-ARENA order.
    ///
    /// Arena order is the vocabulary every claim on this type is stated
    /// in: [`SignCertificate`]'s refusal rule names the first refusing
    /// face in it, and [`Self::measure`]'s bit-identity with
    /// [`mass_properties`] is identity of the same terms summed in it.
    /// So the parts are re-ordered rather than concatenated, and a
    /// single part covering the whole arena — which is what a body
    /// holding one solid hands here — comes back unchanged, run for
    /// run and round for round.
    ///
    /// `body`, `band`, `tol` and `quad` are what every part was derived
    /// against and what the assembled certificate continues with. The
    /// parts of one check are one door's walks, so they agree, and that
    /// is asserted below rather than assumed: a part read at another
    /// band, of another body or through another lane would sum terms
    /// that are not one enclosure. The lane is compared by PRESENCE —
    /// the only thing a caller chooses, since [`QuadLane::certified`] is
    /// the one value it can hold; its pointer is `wiring_rows`' to pin,
    /// and function-pointer equality is not a reliable identity to
    /// panic on (a function may have several addresses).
    ///
    /// # Panics
    ///
    /// When the parts do not cover the face arena exactly once — and
    /// all three ways of failing that are separate reads, because two
    /// of them cancel in any one of the others. A face handed in twice
    /// raises the count handed in above the count placed; a face no
    /// solid of `body` owns is left over; and a face NO part named is
    /// neither, so the count of placed runs is read against the arena's
    /// own length, which is the only thing a subset walk disagrees
    /// with. Every caller partitions the arena by an ownership relation
    /// tier 1 has already validated, so a gap is a bug in the
    /// composition above rather than a body state (D9's bug-state
    /// half).
    pub(crate) fn assembled(
        body: &'b Body<T>,
        band: Band,
        tol: Tol,
        quad: Option<QuadLane<T>>,
        parts: Vec<Self>,
    ) -> Self {
        let mut by_face: slotmap::SecondaryMap<FaceKey, FaceRun<T>> = slotmap::SecondaryMap::new();
        let mut handed = 0usize;
        for part in parts {
            assert!(
                core::ptr::eq(part.body, body)
                    && part.band == band
                    && part.tol == tol
                    && part.quad.is_some() == quad.is_some(),
                "a sign certificate assembled from a part derived against another body, band, \
                 tolerance or lane"
            );
            for run in part.runs {
                handed += 1;
                by_face.insert(run.face, run);
            }
        }
        let runs: Vec<FaceRun<T>> = body
            .faces
            .iter()
            .filter_map(|(face_key, _)| by_face.remove(face_key))
            .collect();
        assert!(
            handed == runs.len() && by_face.is_empty() && runs.len() == body.faces.len(),
            "a sign certificate assembled from parts that do not partition the face arena: \
             {handed} runs handed in, {} of them in the arena of {}, {} left over",
            runs.len(),
            body.faces.len(),
            by_face.len(),
        );
        Self::of_runs(body, band, tol, quad, runs)
    }

    /// The volume bracket and its lever, at the round the walk stopped
    /// on — as sound as the lane it was derived through: a CERTIFIED
    /// bracket from [`QuadLane::certified`] at a certifying scalar, and
    /// the closed form's value with pads of `0` from a `_structural`
    /// door, which at a point scalar such as `f64` is a computed value
    /// and not a certified enclosure.
    #[must_use]
    pub fn enclosure(&self) -> VolumeEnclosure<T> {
        fold_runs(&self.runs).enclosure()
    }

    /// The volume as the faces stand, re-derived in interval arithmetic
    /// through the certificate's lane ([`rederive`]): the bracket a
    /// refusal reports, whose ends are directed outward over the stored
    /// geometry rather than the walk's own sums. `None` where the
    /// certificate holds no lane or the re-derivation refuses.
    pub(crate) fn interval_volume(&self) -> Option<(f64, f64)> {
        let lane = self.quad?;
        rederive(self.body, self.band, self.tol, lane, &self.runs, true)
            .ok()
            .map(|r| (r.volume.lo(), r.volume.hi()))
    }

    /// The faces walked on under `settle` from where they stand: sized to
    /// a resolution, or walked until a deciding predicate settles or
    /// no face can refine, answering why the walk stopped.
    fn settle_to(&mut self, settle: Settle<T>) -> Result<Stop, MassPropsError> {
        match settle {
            Settle::Resolution(len) => {
                let len = resolution_len(len, self.tol)?;
                self.resume_to(len);
                resolution_stop(self, len)
            }
            settle => self.settle_on(self.quad, settle),
        }
    }

    /// The certificate's parts without its borrow of the body, for
    /// [`Certifier`] to keep beside the body it certified.
    fn into_parts(self) -> Held<T> {
        let Self {
            body: _,
            band,
            tol,
            quad,
            runs,
        } = self;
        Held {
            band,
            tol,
            quad,
            runs,
        }
    }

    /// Every face short of `len` whose lane can refine, resumed at its
    /// next round over the rest of its schedule.
    fn resume_to(&mut self, len: f64) {
        let Self {
            body,
            band,
            tol,
            quad,
            runs,
        } = self;
        let (body, band, tol) = (*body, *band, *tol);
        let hook = round_hook(*quad);
        // One slot per face in ARENA order, holding that face's
        // resumption ([`decide_each`]: every resumption decided, its
        // recording spliced in arena order), each face read about what the
        // walk read it about.
        let wanted: Vec<(usize, usize)> = runs
            .iter()
            .enumerate()
            .filter_map(|(slot, run)| run.resumes_to(len).map(|first| (slot, first)))
            .collect();
        if wanted.is_empty() {
            return;
        }
        let resumed = decide_each(&wanted, |&(slot, first)| {
            face_flux_read(
                body,
                runs[slot].face,
                band,
                &hook,
                tol,
                RoundWindow {
                    first,
                    last: usize::MAX,
                },
                runs[slot].about,
                Some(len),
            )
        });
        for (&(slot, _), resumed) in wanted.iter().zip(resumed) {
            runs[slot].advance(resumed);
        }
    }
}

impl<T: Decide + geom_core::Bounds> SignCertificate<'_, T> {
    /// **The measurement, continued from here** — the door
    /// ([`crate::mass_properties`]) under `settle`, over the faces as the
    /// certificate holds them, reusing every round already taken.
    ///
    /// Under [`Settle::Resolution`] a face short of the resolution at
    /// round `k` resumes at `k + 1`, a face whose enclosure already meets
    /// it is not touched, and the sum is over the same terms in the same
    /// arena order. Where the walk stopped before a face met the
    /// resolution, the face's enclosure and its piece evaluations are the
    /// door's, so a certificate whose sign settled before any face met it
    /// continues bit-identical to that door. A face the walk had refined
    /// past the resolution keeps its narrower enclosure. A deciding
    /// predicate walks on from where the faces stand
    /// ([`Self::settle_on`]), certified through the certificate's lane.
    ///
    /// **Its refusal is the door's too** — [`SignCertificate`]'s refusal
    /// rule: the open faces are resumed by an indexed parallel map into
    /// one slot per face in arena order (D9's addendum idiom 1, through
    /// [`geom_core::k_stats::map_detached`], each resumption under a
    /// K-funnel frame of its own) and the walk over those slots is
    /// sequential in that order (idiom 2), so the verdict log, the
    /// escalation log and the `probe` sample population are the serial
    /// continuation's at any thread count. Under a symbolic session the
    /// continuation is the SERIAL walk instead, for
    /// [`decide_faces_serially`]'s reason.
    ///
    /// **What the continuation costs against one measurement.** The
    /// piece evaluations compose exactly, but each entry into a face's
    /// lane re-derives that face's round-independent SETUP — the
    /// derivative grids, the block hulls — because a `RoundWindow` enters
    /// the lane at its front door. A certificate whose walk ran every face
    /// to its own stop re-enters no lane; one that stopped part-way pays
    /// the setup again for every face it left open. Removing it is
    /// `work/perf/`'s `quadrature-setup-is-re-derived-per-round-window`,
    /// which carries the measurements.
    ///
    /// # Errors
    ///
    /// [`MassPropsError`], as [`crate::mass_properties`]: a deciding
    /// predicate still undecided where no face can refine is
    /// [`MassPropsError::Undecided`].
    pub fn measure(mut self, settle: Settle<T>) -> Result<MassMeasurement<T>, MassPropsError> {
        let deciding = !matches!(settle, Settle::Resolution(_));
        let stop = self.settle_to(settle)?;
        self.answer(deciding, stop)
    }

    /// **The walk's answer where it stopped at `stop`**: a deciding
    /// predicate run out at [`Stop::Wall`] or [`Stop::Floor`] is the
    /// refusal, [`MassPropsError::Undecided`] carrying
    /// [`Self::outward_volume`] and the stop (`[tolerance.measure]`);
    /// anything else is the measurement. A resolution decides nothing,
    /// so its unmet stop is the bracket and passes.
    ///
    /// # Errors
    ///
    /// [`MassPropsError::Undecided`], as above.
    pub(crate) fn answer(
        &self,
        deciding: bool,
        stop: Stop,
    ) -> Result<MassMeasurement<T>, MassPropsError> {
        match stop {
            Stop::Wall | Stop::Floor if deciding => Err(MassPropsError::Undecided {
                volume: self.outward_volume(),
                stop,
            }),
            _ => Ok(MassMeasurement {
                enclosure: self.props(),
                stop,
            }),
        }
    }

    /// **The volume's one outward bracket**, `(lo, hi)` (m³), the one a
    /// refusal reports — [`MassPropsError::Undecided`]'s `volume` and
    /// check 7's [`crate::ValidationError::VolumeSignUnresolved`] read
    /// it alike. It is the faces' interval re-derivation
    /// ([`Self::interval_volume`]), whose ends are directed outward over
    /// the stored geometry. Where the certificate holds no lane to
    /// re-derive through, or the re-derivation refuses, it FALLS BACK to
    /// the walk's own enclosure, `volume ∓ pad` with each end stepped one
    /// ulp outward. Either way the volume lies between the ends: the
    /// fallback is as sound as the lane the walk ran on, and no tighter.
    pub(crate) fn outward_volume(&self) -> (f64, f64) {
        self.interval_volume().unwrap_or_else(|| {
            let enclosure = self.enclosure();
            (
                geom_core::Bounds::lo(enclosure.volume_lo).next_down(),
                geom_core::Bounds::hi(enclosure.volume_hi).next_up(),
            )
        })
    }
}

/// A [`SignCertificate`]'s parts without its borrow: what
/// [`CertifiedBody`] keeps beside the one body they were derived over.
#[derive(Clone)]
struct Held<T: Decide> {
    band: Band,
    tol: Tol,
    quad: Option<QuadLane<T>>,
    runs: Vec<FaceRun<T>>,
}

/// **A body held together with the at-rest gate's certificate of it**:
/// what an import hands out, so a reader measures the body through the
/// rounds the gate already ran ([`CertifiedBody::measure`]) instead of
/// running its quadrature again.
///
/// **The pair is one value, and the certificate is of this body by
/// construction.** It is minted only by [`CertifiedBody::certify`], whose
/// gate certifies the very body the value then owns ([`Certifier`]); the
/// body is reached by shared borrow ([`CertifiedBody::body`]) and leaves
/// only by [`CertifiedBody::into_body`], which drops the certificate. So
/// no certificate is ever measured against a body it was not derived
/// over: not a twin built the same way, whose face keys are the same, and
/// not this body changed after it was certified. Neither has a spelling.
/// A measurement names no body to measure against:
///
/// ```compile_fail,E0061
/// fn cross(a: &topo::CertifiedBody, b: &topo::Body<f64>, s: topo::Settle<f64>) {
///     let _ = a.measure(b, s);
/// }
/// ```
///
/// and the body held is never reached mutably:
///
/// ```compile_fail,E0308
/// fn change(a: &topo::CertifiedBody) -> &mut topo::Body<f64> {
///     a.body()
/// }
/// ```
#[derive(Clone)]
pub struct CertifiedBody {
    body: Body<f64>,
    held: Held<f64>,
}

/// **The gate's handle on the body [`CertifiedBody::certify`] is
/// certifying**: it certifies that body and no other, and keeps the
/// certificate of the last validation that passed.
pub struct Certifier<'a> {
    body: &'a Body<f64>,
    kept: Option<Held<f64>>,
}

impl<'a> Certifier<'a> {
    /// The body being certified.
    #[must_use]
    pub fn body(&self) -> &'a Body<f64> {
        self.body
    }

    /// **Tier 3′ over the body being certified**
    /// ([`crate::validate_pseudomanifold_certificate`] under `contacts`),
    /// keeping its certificate where it passes.
    ///
    /// # Errors
    ///
    /// The validator's findings, as that door returns them; a refusal
    /// keeps nothing and leaves an earlier pass's certificate kept.
    pub fn validate_pseudomanifold(
        &mut self,
        contacts: &crate::boolean::ContactRecords,
        tol: Tol,
    ) -> Result<(), Vec<crate::ValidationError>> {
        let certificate = crate::validate_pseudomanifold_certificate(self.body, contacts, tol)?;
        self.kept = Some(certificate.into_parts());
        Ok(())
    }
}

impl CertifiedBody {
    /// **`body`, certified by `gate`**: `gate` runs over a [`Certifier`]
    /// of `body`, and where it kept a certificate the body and that
    /// certificate come back as one value. Where it kept none, the body
    /// comes back alone. Either way `gate`'s own answer comes back beside
    /// it.
    pub fn certify<R>(
        body: Body<f64>,
        gate: impl FnOnce(&mut Certifier<'_>) -> R,
    ) -> (Result<Self, Body<f64>>, R) {
        let (kept, answer) = {
            let mut certifier = Certifier {
                body: &body,
                kept: None,
            };
            let answer = gate(&mut certifier);
            (certifier.kept, answer)
        };
        match kept {
            Some(held) => (Ok(Self { body, held }), answer),
            None => (Err(body), answer),
        }
    }

    /// The certified body.
    #[must_use]
    pub fn body(&self) -> &Body<f64> {
        &self.body
    }

    /// The body, with the certificate dropped.
    #[must_use]
    pub fn into_body(self) -> Body<f64> {
        self.body
    }

    /// **The body measured under `settle` through its certificate**:
    /// [`SignCertificate::measure`] continued from the rounds the gate
    /// took. It leaves the certificate as it was, so each call continues
    /// from the gate's rounds.
    ///
    /// # Errors
    ///
    /// As [`SignCertificate::measure`].
    pub fn measure(&self, settle: Settle<f64>) -> Result<MassMeasurement<f64>, MassPropsError> {
        let Held {
            band,
            tol,
            quad,
            runs,
        } = &self.held;
        SignCertificate::of_runs(&self.body, *band, *tol, *quad, runs.clone()).measure(settle)
    }
}

impl fmt::Debug for CertifiedBody {
    /// The certificate as [`SignCertificate`]'s render reads it.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self { body: _, held } = self;
        let e = fold_runs(&held.runs).enclosure();
        write!(
            f,
            "CertifiedBody: volume in [{:?}, {:?}], surface area {:?}, {} faces",
            e.volume_lo,
            e.volume_hi,
            e.surface_area,
            held.runs.len()
        )
    }
}

/// The closed-form-only walk against a caller-held band: plain
/// `T: Decide`, no quadrature lane and no bracket read —
/// [`mass_properties_structural`] is this door with the band built
/// inside, the public spelling of the same walk. On a conic-trimmed
/// face the closed form refuses typed (fail-loud). The at-rest
/// measurement door is [`mass_properties`], which carries the certified
/// lane.
///
/// # Errors
///
/// [`MassPropsError`], as [`mass_properties_structural`].
pub(crate) fn mass_properties_closed_form<T: Decide>(
    body: &Body<T>,
    band: Band,
    tol: Tol,
) -> Result<MassProperties<T>, MassPropsError> {
    let faces = crate::query::all_faces(body);
    mass_properties_closed_form_of(body, &faces, band, tol)
}

/// [`mass_properties_closed_form`] over exactly `faces` — the enclosure
/// those faces bound, summed in the order given. The whole-body door
/// is this one handed the face arena in arena order, so its answer is
/// bit-for-bit the same (`face_list_door_tests` pins it).
pub(crate) fn mass_properties_closed_form_of<T: Decide>(
    body: &Body<T>,
    faces: &[FaceKey],
    band: Band,
    tol: Tol,
) -> Result<MassProperties<T>, MassPropsError> {
    // No lane refines, so the resolution is ε's, which every closed form
    // meets.
    let runs = resolution_runs(body, faces, band, tol, None, tol.eps())?;
    Ok(fold_runs(&runs))
}

/// The per-face certified-quadrature hook: `Ok(None)` = no lane / not
/// attempted (the closed form then answers, refusing typed on trimmed
/// faces), `Ok(Some(run))` = the lane's answer over the window it was
/// handed, each round asked the resolution it was handed.
type QuadHook<'h, T> = dyn Fn(
        &Body<T>,
        &Surface<T>,
        &[FlatLoop<T>],
        Band,
        Tol,
        RoundWindow,
        Point3<T>,
        Option<f64>,
    ) -> Result<Option<LaneRun>, PropsError>
    // `Sync` because the walks below hand this hook to an indexed
    // parallel map over faces: every face calls the same hook, so the
    // hook is shared across workers. Both shipped hooks are
    // non-capturing (a `fn` item and a closure over nothing), so the
    // bound costs their call sites nothing.
    + Sync
    + 'h;

/// **One face's decision, taken on a worker** — what `face_flux`
/// answered, and everything the K-funnel recorded while it answered.
///
/// The recording rides WITH the answer because the funnel is
/// thread-local: a face decided on a rayon worker records into that
/// worker's frame and sink, and only a value handed back to the fold
/// can reach the caller's (`geom_core::k_stats::detached`). The serial
/// arm of [`decide_faces`] never builds one — its decisions land in the
/// caller's own frame and sink, because they are taken there.
type FaceDecision<T> = (Result<FaceRun<T>, MassPropsError>, Detached);

/// **The face walk, in one spelling and two dispatches.** `run` decides
/// one face; the answer is the faces in arena order, or the first
/// refusal in arena order.
///
/// *Thread-portable* (the shipped case): **D9 addendum idiom 1** — one
/// slot per item in the caller's order, results written positionally
/// and never combined arithmetically, so the schedule cannot reach the
/// bits — then [`splice_in_arena_order`], which is idiom 2's
/// sequential half for the recordings as [`fold_runs`] is for the
/// fluxes.
///
/// *Not portable* ([`geom_core::sym::decisions_are_thread_portable`]): this IS the
/// serial walk. Faces are decided on the caller's thread in arena
/// order and the walk stops at the first refusal, exactly as it did
/// before any of this — no detached frame (the decisions are already
/// landing in the caller's frame and sink) and no face after the
/// refusing one decided. That matters beyond cost: a decision under a
/// session also writes the session's receipt and the shape report in
/// place, and a face decided past the point the serial walk stopped
/// would inflate both with no way to take it back.
fn decide_faces<I: Sync, T: Decide>(
    items: &[I],
    run: impl Fn(&I) -> Result<FaceRun<T>, MassPropsError> + Send + Sync,
) -> Result<Vec<FaceRun<T>>, MassPropsError> {
    if geom_core::sym::decisions_are_thread_portable() {
        splice_in_arena_order(geom_core::k_stats::map_detached(items, run))
    } else {
        decide_faces_serially(items, run)
    }
}

/// **The serial face walk, one home**: items decided on the CALLER's
/// thread in the caller's order, stopping at the first refusal — no
/// detached frame, because the decisions are already landing in the
/// caller's frame and sink, and no item after the refusing one
/// decided.
///
/// Two callers, for two unrelated reasons, and both need this exact
/// behaviour rather than merely "a loop":
///
/// - [`decide_faces`]'s non-portable arm, where a decision also writes
///   the installed symbolic session's receipt and the shape report IN
///   PLACE ([`geom_core::sym::decisions_are_thread_portable`]), so an item decided
///   past the point this walk stops inflates both with no way to take
///   it back;
/// - [`classify_shells_of`], where mapping is simply the wrong trade:
///   every shell the census meets is below the per-face map's
///   break-even (`work/perf/`'s
///   `parallel-map-costs-a-fixed-price-on-a-cheap-body` carries the
///   numbers — a shell of the corpus heat sink has six planar faces,
///   and the map made that document's census about three times slower
///   at four threads while gaining on no body in the corpus, because
///   no corpus shell is many-faced on the quadrature lane). The grain
///   that could repay the price there is the loop over SHELLS, not the
///   loop over a shell's faces, and it measures at about break-even on
///   the same document — so the face grain is settled, and that one is
///   the census's own question.
fn decide_faces_serially<I, T: Decide>(
    items: &[I],
    run: impl Fn(&I) -> Result<FaceRun<T>, MassPropsError>,
) -> Result<Vec<FaceRun<T>>, MassPropsError> {
    items.iter().map(run).collect()
}

/// **Every item decided, each with its own answer** — a walk's later
/// round ([`walk_set`]), where a face refused outright stands at the
/// enclosure it held and every other face still advances. Every item is
/// decided in both arms, its recording spliced in arena order, so the
/// round records the same at every thread count; under a symbolic
/// session the items are decided on the caller's thread, for
/// [`decide_faces_serially`]'s reason.
fn decide_each<I: Sync, T: Decide>(
    items: &[I],
    run: impl Fn(&I) -> Result<FaceRun<T>, MassPropsError> + Send + Sync,
) -> Vec<Result<FaceRun<T>, MassPropsError>> {
    if geom_core::sym::decisions_are_thread_portable() {
        geom_core::k_stats::map_detached(items, run)
            .into_iter()
            .map(|(run, recording)| {
                geom_core::k_stats::splice(recording);
                run
            })
            .collect()
    } else {
        items.iter().map(run).collect()
    }
}

/// **The parallel arm's sequential half, and where the escalation path
/// lives.** Splices each face's recording into the caller's frame and
/// sink in ARENA ORDER and answers the runs, stopping at the first face
/// whose lane refused outright.
///
/// The serial walk stops AT that face: faces before it have recorded,
/// the refusing face has recorded whatever it decided before refusing,
/// and faces after it are never visited. The map has already decided
/// every face, so this makes the logs say the same thing: every
/// recording up to and including the refusing face is spliced, in
/// order, and every recording after it is dropped on the floor with the
/// run it came with.
///
/// **What the failure path costs**: the fluxes of the faces after the
/// refusing one, computed and thrown away. That is a real wall-clock
/// cost and it is paid at every width, this arm's one and four alike —
/// it is the latency of a REFUSAL, never of an answer, and a body that
/// answers never reaches it.
fn splice_in_arena_order<T: Decide>(
    decided: Vec<FaceDecision<T>>,
) -> Result<Vec<FaceRun<T>>, MassPropsError> {
    let mut runs = Vec::with_capacity(decided.len());
    for (run, recording) in decided {
        geom_core::k_stats::splice(recording);
        runs.push(run?);
    }
    Ok(runs)
}

/// **The escalation path, read directly**: the fold splices every
/// recording up to and including the refusing face and drops every one
/// after it, in arena order.
///
/// The end-to-end rows live in `sweep`'s
/// `mass_props_are_thread_count_invariant` — a real body, the public
/// doors, 4 threads against the serial walk's golden. This one pins the rule those rows depend
/// on at the site that carries it, with the refusal placed where no
/// fixture body puts it: at a chosen slot, with a recording on every
/// slot before and after, so a fold that spliced one face too few or
/// one too many reds here.
#[cfg(test)]
mod face_walk_composition_tests {
    #![allow(clippy::expect_used, clippy::panic)]

    use super::*;
    use geom_core::k_stats::{Bracket, detached};

    fn band() -> Band {
        Band::linear(Tol::witness()).expect("the witness band builds")
    }

    /// A detached run that records exactly one verdict, under `name`.
    fn one_verdict(name: &'static str) -> Detached {
        detached(|| {
            let _ = crate::validate::decide(name, Margin::of(1.0f64), band());
        })
        .1
    }

    /// A face run that contributes nothing and refuses nothing — the
    /// fold's business here is the recording and the stop, not the sum.
    fn silent_run() -> FaceRun<f64> {
        FaceRun {
            face: FaceKey::null(),
            contribution: FaceFlux {
                flux: 0.0,
                area: 0.0,
                flux_pad: 0.0,
                area_pad: 0.0,
                enclosure: None,
            },
            state: FaceState::Closed,
            about: ReadAbout::Shell(Point3::origin()),
        }
    }

    /// The slots, with a hard refusal at `refuse_at` when one is asked
    /// for; every slot carries a recording naming itself.
    fn slots(names: &[&'static str], refuse_at: Option<usize>) -> Vec<FaceDecision<f64>> {
        names
            .iter()
            .enumerate()
            .map(|(i, &name)| {
                let recording = one_verdict(name);
                let run = if refuse_at == Some(i) {
                    Err(MassPropsError::Corrupt {
                        what: "the injected refusal",
                    })
                } else {
                    Ok(silent_run())
                };
                (run, recording)
            })
            .collect()
    }

    const NAMES: [&str; 4] = ["walk_slot_a", "walk_slot_b", "walk_slot_c", "walk_slot_d"];

    /// A face's round `round` with flux enclosure `[-w/2, w/2] + 10`
    /// and a unit-ish area, its lane's stop `stop`.
    fn round_run(round: usize, w: f64, stop: LaneStop) -> FaceRun<f64> {
        let flux = Interval::from_bounds(10.0 - 0.5 * w, 10.0 + 0.5 * w);
        let area = Interval::from_bounds(3.0, 3.0);
        FaceRun {
            face: FaceKey::null(),
            contribution: FaceFlux {
                flux: 10.0,
                area: 3.0,
                flux_pad: 0.5 * w,
                area_pad: 0.0,
                enclosure: Some((flux, area)),
            },
            state: FaceState::Quad {
                taken: round,
                ran: round,
                stop: FaceStop::Lane(stop),
            },
            about: ReadAbout::Shell(Point3::origin()),
        }
    }

    /// The mean boundary displacement of [`round_run`]'s width `w`.
    fn len_of(w: f64) -> f64 {
        geom_brep::props::quad::mean_boundary_displacement(
            Interval::from_bounds(10.0 - 0.5 * w, 10.0 + 0.5 * w),
            Interval::from_bounds(3.0, 3.0),
        )
        .expect("a lever")
    }

    fn width(run: &FaceRun<f64>) -> f64 {
        use geom_core::interval::certification::Certification;
        run.contribution
            .enclosure
            .expect("a quadrature face")
            .0
            .width()
    }

    fn taken(run: &FaceRun<f64>) -> usize {
        match run.state {
            FaceState::Quad { taken, .. } => taken,
            FaceState::Closed => panic!("a quadrature face"),
        }
    }

    /// **A face walked one round at a time ends where the whole-schedule
    /// call does**: the lane answers its narrowest round, so the walk
    /// keeps the narrowest enclosure it has held, including when a later
    /// round is wider. Rounds of widths 4, 1, 3 and 2 end on the round of
    /// width 1, with the last round's place to resume.
    #[test]
    fn a_face_walked_round_by_round_keeps_its_narrowest_enclosure() {
        let mut run = round_run(0, 4.0, LaneStop::Refinable);
        for (round, w) in [(1, 1.0), (2, 3.0), (3, 2.0)] {
            run.advance(Ok(round_run(round, w, LaneStop::Refinable)));
        }
        assert!(
            (width(&run) - 1.0).abs() < 1e-12,
            "the walk dropped its narrowest round: width {}",
            width(&run)
        );
        assert_eq!(taken(&run), 1, "the kept round is named");
        assert_eq!(
            run.next_round(),
            Some(4),
            "where it goes next follows the last round"
        );
    }

    /// **A face stopped short of a resolution keeps the narrower held
    /// enclosure**: a wider round that reaches the wall leaves the face at
    /// the held round's width, short of the resolution, refining no
    /// further.
    #[test]
    fn a_face_stopped_short_keeps_its_held_width() {
        let mut run = round_run(4, 1.0, LaneStop::Refinable);
        run.advance(Ok(round_run(5, 2.0, LaneStop::Wall)));
        assert_eq!(
            run.width_len(),
            Ok(len_of(1.0)),
            "the width is the held round's"
        );
        assert!(!run.meets(len_of(0.5)), "short of the resolution");
        assert_eq!(taken(&run), 4);
        assert_eq!(run.next_round(), None, "and refines no further");
    }

    /// **A face stopped at its floor keeps its held width**, as at the
    /// wall: a wider round that floors keeps the narrower held
    /// enclosure, short of the resolution however close that width is
    /// to it, and refines no further.
    #[test]
    fn a_floor_stop_keeps_the_held_width() {
        for over in [2.0, 1.0 + 1e-6] {
            let mut run = round_run(4, over, LaneStop::Refinable);
            run.advance(Ok(round_run(5, 2.0 * over, LaneStop::Floor)));
            assert_eq!(
                run.width_len(),
                Ok(len_of(over)),
                "{over}× the resolution: the width is the held round's"
            );
            assert!(!run.meets(len_of(1.0)), "{over}×: short of the resolution");
            assert_eq!(taken(&run), 4);
            assert_eq!(run.next_round(), None, "{over}×: and refines no further");
        }
    }

    /// **A round that met the resolution keeps its number** when the
    /// next, wider round stops short: the face refuses nothing, and
    /// refines no further. A narrower round replaces a wider held one.
    #[test]
    fn a_met_round_keeps_its_number_past_a_round_that_stops_short() {
        let len = len_of(1.5);
        let mut run = round_run(3, 1.0, LaneStop::Refinable);
        run.advance(Ok(round_run(4, 2.0, LaneStop::Wall)));
        assert!(run.meets(len), "the met round's number stands");
        assert_eq!(run.resumes_to(len), None, "and the face refines no further");
        assert_eq!(taken(&run), 3);
        let mut run = round_run(3, 2.0, LaneStop::Refinable);
        run.advance(Ok(round_run(4, 1.0, LaneStop::Refinable)));
        assert_eq!((taken(&run), run.next_round()), (4, Some(5)));
    }

    #[test]
    fn every_recording_splices_in_arena_order_when_no_face_refuses() {
        let decided = slots(&NAMES, None);
        let bracket = Bracket::open();
        let runs = splice_in_arena_order(decided).expect("no face refused");
        let log = bracket.finish();
        assert_eq!(runs.len(), NAMES.len());
        assert_eq!(
            log.verdicts.iter().map(|v| v.predicate).collect::<Vec<_>>(),
            NAMES,
            "the fold did not splice the slots in arena order"
        );
    }

    /// **The serial arm stops where the serial walk stopped.** Under an
    /// installed symbolic session `decide_faces` decides on the caller's
    /// thread, and it must not decide a face past the first refusal: a
    /// decision writes the session's receipt and the shape report IN
    /// PLACE, where no splice can take it back, so a face the serial
    /// walk never reached would inflate both.
    #[test]
    fn the_serial_arm_decides_no_face_past_the_first_refusal() {
        use core::sync::atomic::{AtomicUsize, Ordering};
        let seen = AtomicUsize::new(0);
        let items: Vec<usize> = (0..5).collect();
        let (out, _counts) = geom_core::sym::with_session(
            geom_core::sym::SymBudget {
                max_terms: 8,
                max_degree: 2,
            },
            || {
                decide_faces(&items, |&i| {
                    seen.fetch_add(1, Ordering::Relaxed);
                    if i == 2 {
                        Err(MassPropsError::Corrupt {
                            what: "the injected refusal",
                        })
                    } else {
                        Ok::<FaceRun<f64>, MassPropsError>(silent_run())
                    }
                })
            },
        );
        assert!(out.is_err(), "slot 2 refused and the walk answered Ok");
        assert_eq!(
            seen.load(Ordering::Relaxed),
            3,
            "the serial arm decided a face past the first refusal — the \
             session's receipt and the shape report cannot be un-written"
        );
    }

    /// The other arm's price, stated as a row rather than only in prose:
    /// with no session installed the map decides EVERY face and the fold
    /// then drops what it cannot use. That is the disclosed failure-path
    /// cost, and it is a property of a refusing body at every width.
    #[test]
    fn the_parallel_arm_decides_every_face_and_then_drops_the_rest() {
        use core::sync::atomic::{AtomicUsize, Ordering};
        let seen = AtomicUsize::new(0);
        let items: Vec<usize> = (0..5).collect();
        let out = decide_faces(&items, |&i| {
            seen.fetch_add(1, Ordering::Relaxed);
            if i == 2 {
                Err(MassPropsError::Corrupt {
                    what: "the injected refusal",
                })
            } else {
                Ok::<FaceRun<f64>, MassPropsError>(silent_run())
            }
        });
        assert!(out.is_err(), "slot 2 refused and the walk answered Ok");
        assert_eq!(seen.load(Ordering::Relaxed), 5);
    }

    #[test]
    fn the_refusing_faces_recording_splices_and_every_later_one_is_dropped() {
        let decided = slots(&NAMES, Some(2));
        let bracket = Bracket::open();
        let err = match splice_in_arena_order(decided) {
            Err(err) => err,
            Ok(_) => panic!("slot 2 refused and the fold answered Ok"),
        };
        let log = bracket.finish();
        assert!(
            matches!(err, MassPropsError::Corrupt { what } if what == "the injected refusal"),
            "the fold reported another face's refusal: {err}"
        );
        assert_eq!(
            log.verdicts.iter().map(|v| v.predicate).collect::<Vec<_>>(),
            &NAMES[..3],
            "the log is not the serial walk's: it stops at the first refusing face, \
             which had already decided what it decided before it refused"
        );
    }
}

#[cfg(test)]
mod fold_pad_rows {
    #![allow(clippy::float_cmp)]

    use super::*;

    fn run_with(pad: f64) -> FaceRun<f64> {
        FaceRun {
            face: FaceKey::null(),
            contribution: FaceFlux {
                flux: 0.0,
                area: 0.0,
                flux_pad: pad,
                area_pad: pad,
                enclosure: None,
            },
            state: FaceState::Closed,
            about: ReadAbout::Shell(Point3::origin()),
        }
    }

    /// **The whole's pads are no less than the faces'**: `1` and a
    /// quarter of its ulp sum to `1` rounded to nearest, which would
    /// report a half-width below the faces' summed; summed rounding up
    /// it is `1`'s successor, and the volume pad is that sum's third
    /// rounded up. A body of closed-form faces has pads of exactly `0`.
    #[test]
    fn the_folded_pads_round_up_and_zero_stays_exact() {
        let quarter = f64::EPSILON / 4.0;
        let props = fold_runs(&[run_with(1.0), run_with(quarter)]);
        assert_eq!(props.area_pad, 1.0f64.next_up());
        assert_eq!(props.volume_pad, interval::div_up(1.0f64.next_up(), 3.0));
        assert!(3.0 * props.volume_pad >= props.area_pad);
        let props = fold_runs(&[run_with(0.0), run_with(0.0)]);
        assert_eq!((props.area_pad, props.volume_pad), (0.0, 0.0));
    }
}

#[cfg(test)]
mod fold_order_rows {
    use super::*;

    /// Every order of six signed terms whose volume is small against
    /// them (a body `10⁵` m out), each summed as the fold sums them and
    /// divided by 3.
    fn volumes(terms: &[f64]) -> Vec<f64> {
        fn perms(rest: &mut Vec<f64>, at: usize, out: &mut Vec<f64>) {
            if at == rest.len() {
                out.push(rest.iter().fold(0.0, |s, &t| s + t) / 3.0);
                return;
            }
            for i in at..rest.len() {
                rest.swap(at, i);
                perms(rest, at + 1, out);
                rest.swap(at, i);
            }
        }
        let mut out = Vec::new();
        perms(&mut terms.to_vec(), 0, &mut out);
        out
    }

    /// **Two fold orders round apart by the terms, not the volume**: over
    /// every order of these terms the volumes spread by `≈ 6.4e-12`,
    /// thousands of the volume's ulps (an ulp count of `faces − 1` is the
    /// wrong shape for signed terms), and within [`fold_order_bound`].
    #[test]
    fn two_fold_orders_round_apart_within_the_terms_bound() {
        let terms = [
            18528.182125433123,
            -73915.44078297145,
            83188.96234619622,
            -5189.292690574687,
            16170.416870011173,
            -38781.41946879917,
        ];
        let v = volumes(&terms);
        let (lo, hi) = v
            .iter()
            .fold((f64::INFINITY, f64::NEG_INFINITY), |(l, h), &x| {
                (l.min(x), h.max(x))
            });
        let face_terms: Vec<FaceTerm> = terms.iter().map(|&t| (FaceKey::null(), t, 0.0)).collect();
        let (bound, pad) = fold_order_bound(&face_terms);
        assert!(lo > 0.0, "a positive volume: {lo}");
        assert!(
            hi - lo > 5.0 * (lo.next_up() - lo),
            "the orders spread past faces − 1 ulps: {}",
            hi - lo
        );
        assert!(hi - lo <= bound, "spread {} within {bound}", hi - lo);
        assert_eq!(pad, 0.0, "no pads, no pad gap");
    }
}

#[cfg(test)]
mod region_carry_tests {
    #![allow(clippy::expect_used)]

    use super::*;
    use geom_core::interval::certification::Certification as _;

    fn iv(p: Point3<f64>) -> Point3<Interval> {
        Point3::new(
            Interval::from_f64(p.x),
            Interval::from_f64(p.y),
            Interval::from_f64(p.z),
        )
    }

    /// The unit square in `z = 0`, counter-clockwise seen from `+z`, or
    /// clockwise: its vector area is `±ẑ`.
    fn square(counter: bool) -> Vec<LoopEdge<Interval>> {
        let p = |x: f64, y: f64| Point3::new(x, y, 0.0);
        let corners = [p(0.0, 0.0), p(1.0, 0.0), p(1.0, 1.0), p(0.0, 1.0)];
        (0..4u32)
            .map(|i| {
                let (a, b) = (corners[i as usize], corners[((i + 1) % 4) as usize]);
                LoopEdge::hand_built(
                    geom::Curve3::Line {
                        origin: iv(a),
                        dir: iv(Point3::origin() + (b - a)) - Point3::origin(),
                    },
                    Interval::from_f64(0.0),
                    Interval::from_f64(1.0),
                    counter,
                    i,
                    (i + 1) % 4,
                )
            })
            .collect()
    }

    /// **The carry is `(c_g − c)·A⃗(∂g)`, with its sign**: a region bounded
    /// by the unit square, read about a corner of it and carried to a point
    /// 5 below, carries `5`; the same square traversed the other way, the
    /// boundary of the region on its other side, carries `−5`. Each is
    /// exact but for rounding.
    #[test]
    fn a_squares_carry_is_its_lever_along_its_vector_area_with_its_sign() {
        let c_g = iv(Point3::new(0.0, 0.0, 0.0));
        let c = iv(Point3::new(0.3, -0.2, -5.0));
        for (counter, truth) in [(true, 5.0), (false, -5.0)] {
            let carry = carry_of(&square(counter), c_g, c).expect("a square's carry reads");
            assert!(
                carry.lo() <= truth && truth <= carry.hi() && carry.width() < 1e-14,
                "counter-clockwise {counter}: {carry:?} must hold {truth}"
            );
        }
    }

    /// **A boundary whose vector area does not read has no carry**: a
    /// spiric arc on it leaves [`carry_of`] with none, so
    /// [`region_carries`] reads the region about the shell's point. No
    /// measuring body reaches that arm: an edge of `∂g` lies on a face that
    /// does not take the quadrature, and the one such face bounded by a
    /// carrier [`loop_vector_area`] refuses is a cone with a spiric or
    /// rational rim, whose own closed form refuses it first. So this row
    /// is the arm's only reader.
    #[test]
    fn carry_of_refuses_a_boundary_whose_vector_area_does_not_read() {
        let mut edges = square(true);
        edges.push(LoopEdge::hand_built(
            geom::Curve3::Spiric {
                center: iv(Point3::origin()),
                axis: iv(Point3::new(0.0, 0.0, 1.0)) - Point3::origin(),
                u_ref: iv(Point3::new(1.0, 0.0, 0.0)) - Point3::origin(),
                major_radius: Interval::from_f64(2.0),
                minor_radius: Interval::from_f64(1.0),
                offset: Interval::from_f64(0.5),
            },
            Interval::from_f64(-0.5),
            Interval::from_f64(0.5),
            true,
            4,
            5,
        ));
        assert!(
            carry_of(
                &edges,
                iv(Point3::origin()),
                iv(Point3::new(0.0, 0.0, -5.0))
            )
            .is_none(),
            "a spiric on the boundary leaves the region uncarried"
        );
    }

    /// **A region is far only at 16 times its own extent**: the unit
    /// square's edges read as a curved face's, about its corner, whose
    /// greatest distance from it is `√2`, are far from a point 1.01 × 16√2
    /// below its centre and not from one 0.99 × 16√2 below it. (The reach
    /// is spelled 16 here, not read off [`CARRY_REACH`], so the row says
    /// what it is: two tori 900 m apart on one bar read about 53, every
    /// multi-region shell of the sweep and step-import suites under 8.)
    /// Read as a plane's, about its corner, the corner lies on the plane, so
    /// the lever about it is nothing and a point one unit below is already
    /// far. About a point half a unit above the plane, the plane's extent is
    /// that half unit, squared: a point 10 below is far (`10² ≥ 16²·½²`)
    /// and one 7 below is not (`7² < 16²·½²`). A lever left unsquared
    /// would read the first as near too (`10 < 16²·½`).
    #[test]
    fn a_region_is_far_only_past_its_reach() {
        assert_eq!(CARRY_REACH, 16.0, "the reach this row argues for");
        let c_g = iv(Point3::origin());
        let reach = 16.0 * 2.0_f64.sqrt();
        let face = |plane: Option<geom_core::Vec3<Interval>>| -> Vec<_> {
            square(true).into_iter().map(|e| (e, plane)).collect()
        };
        let curved = face(None);
        let plane = face(Some(iv(Point3::new(0.0, 0.0, 1.0)) - Point3::origin()));
        for (edges, off, far) in [
            (&curved, 1.01 * reach, true),
            (&curved, 0.99 * reach, false),
            (&curved, 1.0, false),
            (&plane, 1.0, true),
        ] {
            assert_eq!(
                far_from(edges, c_g, iv(Point3::new(0.5, 0.5, -off))),
                far,
                "a point {off} below the square, as a plane: {}",
                edges[0].1.is_some()
            );
        }
        let above = iv(Point3::new(0.0, 0.0, 0.5));
        for (off, far) in [(10.0, true), (7.0, false)] {
            assert_eq!(
                far_from(&plane, above, iv(Point3::new(0.5, 0.5, -off))),
                far,
                "a point {off} below the plane, about one half a unit above it"
            );
        }
    }
}

/// **The refusal rule, at the site that carries it.** [`SignCertificate`]
/// says a face refused outright outranks one left short of the number,
/// wherever each stands, and [`SignCertificate::measure`]'s
/// sequential half is where that survives a parallel resumption. The
/// rows below put the two kinds in the order that would let a walk
/// stopping at the first short face answer the wrong one: an EARLIER
/// face its lane stopped short, and a LATER face whose resumption
/// refuses OUTRIGHT. The certificate is built by hand, so the refusing
/// face can be one no body would hand the walk.
#[cfg(test)]
mod continuation_refusal_order_tests {
    #![allow(clippy::expect_used, clippy::panic)]

    use super::*;
    use geom_core::Point3;

    /// A quadrature face's run, its kept enclosure a mean boundary
    /// displacement of `1 m` (far short of any resolution), its lane's
    /// stop `stop` after round 0.
    fn run_of(face: FaceKey, stop: LaneStop) -> FaceRun<f64> {
        FaceRun {
            face,
            contribution: FaceFlux {
                flux: 0.0,
                area: 1.0,
                flux_pad: 1.5,
                area_pad: 0.0,
                enclosure: Some((
                    Interval::from_bounds(-1.5, 1.5),
                    Interval::from_bounds(1.0, 1.0),
                )),
            },
            state: FaceState::Quad {
                taken: 0,
                ran: 0,
                stop: FaceStop::Lane(stop),
            },
            about: ReadAbout::Shell(Point3::origin()),
        }
    }

    /// A certificate over `runs` of `body`.
    fn certificate(body: &Body<f64>, runs: Vec<FaceRun<f64>>) -> SignCertificate<'_, f64> {
        let band = Band::linear(Tol::witness()).expect("the witness band builds");
        SignCertificate::of_runs(
            body,
            band,
            Tol::witness(),
            Some(QuadLane::certified()),
            runs,
        )
    }

    /// The continuation's refusal at an explicit pool width, over a
    /// certificate whose slot 0 is a face its lane stopped short of the
    /// resolution, and whose slot 1 is OPEN at a face whose resumption
    /// cannot answer at all (a skeletal `mvfs` face: no loop to flatten,
    /// so `face_flux` refuses outright).
    fn refusal_at(threads: usize) -> MassPropsError {
        let mut body = Body::<f64>::new();
        let skeletal = body
            .mvfs(Point3::new(0.0, 0.0, 0.0), true)
            .expect("the skeletal body builds")
            .face;
        let early = FaceKey::null();
        rayon::ThreadPoolBuilder::new()
            .num_threads(threads)
            .build()
            .expect("the pool builds")
            .install(|| {
                certificate(
                    &body,
                    vec![
                        run_of(early, LaneStop::Wall),
                        run_of(skeletal, LaneStop::Refinable),
                    ],
                )
                .measure(crate::test_support::pinned_resolution(Tol::witness()))
            })
            .expect_err("the skeletal face refuses when resumed")
    }

    /// **A face a later round refuses outright stands at the enclosure it
    /// held, and the walk decides on it** (the set walk's rule, as
    /// `PastTarget::refine` had it): the skeletal face holds a round-0
    /// enclosure and refuses when resumed at round 1. A question the held
    /// enclosure answers is decided there, the refusal never surfacing; a
    /// question it cannot answer runs out naming the refusal; and the
    /// measurement keeps the held bracket beside it.
    #[test]
    fn a_later_rounds_outright_refusal_keeps_the_held_enclosure() {
        let mut body = Body::<f64>::new();
        let skeletal = body
            .mvfs(Point3::new(0.0, 0.0, 0.0), true)
            .expect("the skeletal body builds")
            .face;
        let held = run_of(skeletal, LaneStop::Refinable);
        let width = |c: &SignCertificate<'_, f64>| {
            let e = c.props().enclosure();
            e.volume_hi - e.volume_lo
        };

        let mut decided = certificate(&body, vec![held.clone()]);
        let before = width(&decided);
        let mut asked = 0;
        let walked = walk_set(&mut [&mut decided], None, |rounds| {
            asked += 1;
            if asked == 1 {
                Ask::Refine(vec![true])
            } else {
                Ask::Decided(rounds[0].props().enclosure())
            }
        });
        let Walked::Decided(at) = walked else {
            panic!("the held enclosure answers the second ask: {walked:?}")
        };
        assert_eq!(
            asked, 2,
            "asked at round 0, and again on what the face held"
        );
        assert_eq!(
            at.volume_hi - at.volume_lo,
            before,
            "the face keeps the enclosure it held"
        );
        assert!(
            decided.runs[0].refused().is_some() && decided.runs[0].next_round().is_none(),
            "the face stands on its refusal and refines no further"
        );

        let mut open = certificate(&body, vec![held.clone()]);
        let walked = walk_set(&mut [&mut open], None, |_| Ask::<()>::Refine(vec![true]));
        let Walked::Exhausted { refused } = walked else {
            panic!("a question nothing answers runs out: {walked:?}")
        };
        assert_eq!(refused.len(), 1, "the one member names its face's refusal");
        assert_eq!(Some(&refused[0].1), open.runs[0].refused());

        let refusal = certificate(&body, vec![held])
            .measure(crate::test_support::pinned_resolution(Tol::witness()))
            .expect_err("the skeletal face has no number");
        assert_eq!(
            Some(&refusal),
            open.runs[0].refused(),
            "the refusal is the face's own, outright"
        );
    }

    /// **A face left short of the resolution is no refusal; one refused
    /// outright is**, wherever it stands: the short face's enclosure is
    /// sound and its sum is kept, while the refused face has none.
    #[test]
    fn a_later_outright_refusal_is_the_refusal_not_an_earlier_face_short_of_the_resolution() {
        let err = refusal_at(1);
        assert!(
            !matches!(err, MassPropsError::Face { face, .. } if face == FaceKey::null()),
            "the continuation refused on the face short of the resolution: {err}"
        );
    }

    /// **A face its lane stopped short of the resolution is not
    /// resumed**: it has no round to run, so the measurement answers its
    /// bracket, stopped at the wall. The face is skeletal, so a
    /// resumption would refuse outright.
    #[test]
    fn a_stopped_face_short_of_the_resolution_answers_its_bracket_without_resuming() {
        let mut body = Body::<f64>::new();
        let skeletal = body
            .mvfs(Point3::new(0.0, 0.0, 0.0), true)
            .expect("the skeletal body builds")
            .face;
        let measured = certificate(&body, vec![run_of(skeletal, LaneStop::Wall)])
            .measure(crate::test_support::pinned_resolution(Tol::witness()))
            .unwrap_or_else(|err| {
                panic!("the continuation resumed a face its lane had stopped: {err}")
            });
        assert_eq!(measured.stop, Stop::Wall, "the bracket, at the wall");
    }

    #[test]
    fn the_reported_refusal_is_the_same_at_four_threads() {
        assert_eq!(
            format!("{}", refusal_at(4)),
            format!("{}", refusal_at(1)),
            "the refusal the continuation names moved with the pool width"
        );
    }
}

/// **The measurement door's predicates on a body whose every face is
/// closed-form**: the unit cube, volume 1, whose enclosure is one value
/// at round 0, so each predicate answers there and the rows read what
/// it decided.
#[cfg(test)]
mod measure_door_rows {
    #![allow(clippy::expect_used, clippy::panic)]

    use super::*;

    fn unit_cube() -> Body<f64> {
        crate::splitting::reassembly::quad_prism(
            &[(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)],
            1.0,
            Tol::witness(),
        )
    }

    fn stop(body: &Body<f64>, settle: Settle<f64>) -> Stop {
        let tol = Tol::witness();
        let band = Band::linear(tol).expect("the witness band builds");
        let lane = Some(QuadLane::certified());
        let faces = crate::query::all_faces(body);
        measure_to(body, &faces, band, tol, (lane, lane), settle)
            .expect("the cube measures")
            .0
    }

    #[test]
    fn each_predicate_reads_the_cube_as_its_closed_form_says() {
        let cube = unit_cube();
        let relation = |op, bound| match op {
            RelOp::Le => Settle::AtMost {
                bound,
                name: "measure_door_row",
            },
            RelOp::Ge => Settle::AtLeast {
                bound,
                name: "measure_door_row",
            },
        };
        // ε itself: the finest resolution offered, which a closed form
        // meets.
        let len = Length::from_meters(Tol::witness().eps());
        assert_eq!(
            stop(&cube, Settle::Resolution(len)),
            Stop::Met,
            "resolution"
        );
        assert_eq!(
            stop(&cube, Settle::Sign(crate::validate::PLUS_V_SIGN)),
            Stop::Settled(SettleVerdict::Sign(ShellRole::Outer)),
            "sign"
        );
        assert_eq!(
            stop(&cube, relation(RelOp::Le, 2.0)),
            Stop::Settled(SettleVerdict::Holds(true)),
            "V ≤ 2"
        );
        assert_eq!(
            stop(&cube, relation(RelOp::Ge, 2.0)),
            Stop::Settled(SettleVerdict::Holds(false)),
            "V ≥ 2"
        );
        assert_eq!(stop(&cube, relation(RelOp::Le, 1.0)), Stop::Band, "V ≤ 1");
        assert_eq!(
            stop(
                &cube,
                Settle::Contains {
                    lo: 0.5,
                    hi: 1.5,
                    name: "measure_door_row"
                }
            ),
            Stop::Settled(SettleVerdict::Holds(true)),
            "0.5 ≤ V ≤ 1.5"
        );
    }

    /// **A containment whose least volume is above its greatest names no
    /// volume**: the door refuses it, typed, before reading any volume
    /// against it.
    #[test]
    fn a_containment_with_its_ends_reversed_is_refused() {
        let tol = Tol::witness();
        let cube = unit_cube();
        let band = Band::linear(tol).expect("the witness band builds");
        let lane = Some(QuadLane::certified());
        let faces = crate::query::all_faces(&cube);
        let settle = Settle::Contains {
            lo: 1.5,
            hi: 0.5,
            name: "measure_door_row",
        };
        let refusal = measure_to(&cube, &faces, band, tol, (lane, lane), settle)
            .expect_err("an empty containment is refused");
        assert_eq!(refusal, MassPropsError::EmptyContains);
    }

    /// **No resolution finer than ε is offered**: the door refuses it,
    /// typed, naming both lengths.
    #[test]
    fn a_resolution_finer_than_eps_is_refused() {
        let tol = Tol::witness();
        let cube = unit_cube();
        let fine = 0.5 * tol.eps();
        let band = Band::linear(tol).expect("the witness band builds");
        let lane = Some(QuadLane::certified());
        let faces = crate::query::all_faces(&cube);
        let settle = Settle::Resolution(Length::from_meters(fine));
        let refusal = measure_to(&cube, &faces, band, tol, (lane, lane), settle)
            .expect_err("a resolution finer than ε is refused");
        assert_eq!(
            refusal,
            MassPropsError::ResolutionFinerThanEps {
                resolution: fine,
                eps: tol.eps()
            }
        );
    }
}

/// One face's flux term about the point the walk reads it about (its
/// shell's reference, [`shell_reference`], or its carried quadrature
/// region's, whose first face's term holds the region's carry:
/// [`References`]) and its half-width, keyed by the face.
#[cfg(any(test, feature = "test-support"))]
#[doc(hidden)]
pub type FaceTerm = (FaceKey, f64, f64);

/// **How far two fold orders of one body's face terms may round apart**:
/// `(volume, volume_pad)` bounds on `|V₁ − V₂|` and `|P₁ − P₂|` for two
/// bodies whose faces carry these terms, stored in different orders
/// (D9: storage is part of the input, so the bits may differ; the gap
/// may not exceed this). An `n`-term sum rounded to nearest in any
/// order is within `E = γₙ₋₁·Σ|tᵢ|` of the exact one (`γₖ = k·u/(1 − k·u)`,
/// `u = 2⁻⁵³`), and its third rounds once more, by at most
/// `u·(Σ|tᵢ| + E)/3`, so the volumes differ by at most twice their
/// distance from the exact third. The pads are non-negative and summed
/// rounding up, so each order's sum is within `γₙ₋₁·Σpᵢ` above the exact
/// one, and its third is rounded up once more. The bound scales with the
/// terms, not the volume: a body far from the origin has terms far larger
/// than its volume. Read with a margin of `1%` for this arithmetic's own
/// rounding.
#[cfg(any(test, feature = "test-support"))]
#[doc(hidden)]
#[must_use]
pub fn fold_order_bound(terms: &[FaceTerm]) -> (f64, f64) {
    let u = f64::EPSILON / 2.0;
    #[allow(clippy::cast_precision_loss)]
    let k = terms.len().saturating_sub(1) as f64;
    let gamma = k * u / (1.0 - k * u);
    let (abs, pads) = terms.iter().fold((0.0, 0.0), |(a, p), &(_, flux, pad)| {
        (a + flux.abs(), p + pad)
    });
    let e = gamma * abs;
    let volume = 2.0 * (e + u * (abs + e)) / 3.0;
    let pad = (gamma * pads + 2.0 * u * pads * (1.0 + gamma)) / 3.0;
    (1.01 * volume, 1.01 * pad)
}

/// [`mass_properties`] together with every face's own flux term and
/// its half-width, in the order the fold sums them (face-arena order).
/// Test vocabulary: the same walk and the same fold as
/// [`mass_properties`], so the properties are its bits, and the terms
/// are what a caller compares face by face where two bodies store
/// their faces in different orders (the fold's sum depends on the
/// order; each face's term depends only on the point it is about, its
/// shell's reference ([`shell_reference`]) or its carried region's
/// ([`References`]), so two such bodies compare face by face where each
/// shell's first quadrature face, and each region's, starts at one vertex
/// of both). A carried region's first face's term holds the region's
/// whole carry to the shell's reference, as the fold sums it.
///
/// # Errors
///
/// As [`mass_properties`].
#[cfg(any(test, feature = "test-support"))]
#[doc(hidden)]
pub fn mass_properties_with_face_terms(
    body: &Body<f64>,
    tol: Tol,
    settle: Settle<f64>,
) -> Result<(MassMeasurement<f64>, Vec<FaceTerm>), MassPropsError> {
    let band = Band::linear(tol).map_err(|error| MassPropsError::Band { error })?;
    let faces = crate::query::all_faces(body);
    let lane = Some(QuadLane::certified());
    let (stop, certificate) = measure_to(body, &faces, band, tol, (lane, lane), settle)?;
    let props = MassMeasurement {
        enclosure: certificate.props(),
        stop,
    };
    let terms = certificate
        .runs
        .iter()
        .map(|run| (run.face, run.contribution.flux, run.contribution.flux_pad))
        .collect();
    Ok((props, terms))
}

/// [`mass_properties`], with the kept width (mean boundary
/// displacement, m) of the first face in arena order that a
/// [`Settle::Resolution`] left unmet: `None` where every face met it, or
/// under a deciding predicate. Test vocabulary: the rows that pin how far
/// short of a resolution a lane stopped.
///
/// # Errors
///
/// As [`mass_properties`].
#[cfg(any(test, feature = "test-support"))]
#[doc(hidden)]
pub fn measure_with_unmet_width(
    body: &crate::AtRestBody<f64>,
    tol: Tol,
    settle: Settle<f64>,
) -> Result<(MassMeasurement<f64>, Option<f64>), MassPropsError> {
    let band = Band::linear(tol).map_err(|error| MassPropsError::Band { error })?;
    let faces = crate::query::all_faces(body);
    let lane = Some(QuadLane::certified());
    let (stop, certificate) = measure_to(body, &faces, band, tol, (lane, lane), settle)?;
    let unmet = match settle {
        Settle::Resolution(len) => certificate
            .runs
            .iter()
            .find(|run| !run.meets(len.meters()))
            .and_then(|run| run.width_len().ok()),
        _ => None,
    };
    Ok((
        MassMeasurement {
            enclosure: certificate.props(),
            stop,
        },
        unmet,
    ))
}

/// Every face's own flux term and its area, in face-arena order,
/// each read about its shell's reference, or its carried quadrature
/// region's ([`References`]), by its own lane ([`face_flux`])
/// WITHOUT the door's seam-region reads ([`seam_refusals`]): what each
/// face's arm measures of a body the door refuses as a whole, such as a
/// sphere seam region whose members' bits disagree. A carried region's
/// first face's term holds the region's whole carry to the shell's
/// reference. Test vocabulary: the rows that pin how one face's arm reads
/// its own bit, each face run to `resolution`
/// (m) or its own stop.
///
/// # Errors
///
/// A face's own refusal, as [`mass_properties`] reports it, or a
/// resolution finer than ε.
#[cfg(any(test, feature = "test-support"))]
#[doc(hidden)]
pub fn face_flux_terms(
    body: &Body<f64>,
    tol: Tol,
    resolution: f64,
) -> Result<Vec<(FaceKey, f64, f64)>, MassPropsError> {
    let band = Band::linear(tol).map_err(|error| MassPropsError::Band { error })?;
    let faces = crate::query::all_faces(body);
    let hook = round_hook(Some(QuadLane::certified()));
    let refs = References::points(body, faces.iter().copied(), Some(QuadLane::certified()));
    let len = resolution_len(Length::from_meters(resolution), tol)?;
    faces
        .iter()
        .map(|&face| {
            let run = face_flux(
                body,
                face,
                band,
                &hook,
                tol,
                RoundWindow::SCHEDULE,
                &refs,
                Some(len),
            )?;
            Ok((face, run.contribution.flux, run.contribution.area))
        })
        .collect()
}

/// [`mass_properties`]' reporting walk with every face read about its
/// shell's reference and no region carried ([`References`]): the one
/// point per shell the walk read before regions. Test vocabulary: the
/// rows that weigh a region's carry against that reading, each face run
/// to `resolution` (m) or its own stop.
///
/// # Errors
///
/// As [`mass_properties`], or a resolution finer than ε.
#[cfg(any(test, feature = "test-support"))]
#[doc(hidden)]
pub fn mass_properties_one_point(
    body: &Body<f64>,
    tol: Tol,
    resolution: f64,
) -> Result<MassProperties<f64>, MassPropsError> {
    let band = Band::linear(tol).map_err(|error| MassPropsError::Band { error })?;
    let faces = crate::query::all_faces(body);
    let hook = round_hook(Some(QuadLane::certified()));
    let refs = References::of(body, faces.iter().copied(), band, None);
    let len = resolution_len(Length::from_meters(resolution), tol)?;
    let runs = decide_faces(&faces, |&face| {
        face_flux(
            body,
            face,
            band,
            &hook,
            tol,
            RoundWindow::SCHEDULE,
            &refs,
            Some(len),
        )
    })?;
    Ok(fold_runs(&runs))
}

/// **Every walk a door enters, on `body`**, each through the certified
/// lane over the whole face arena, one [`DoorRead`] each, under `settle`:
/// the reporting walk ([`mass_properties`]'s), the target walk
/// ([`measure_to`]) and its continuation ([`SignCertificate::measure`]),
/// a sign walk settled at round 0 ([`walk_set`]) and its continuation,
/// which resumes only the faces round 0 left short of the resolution, the sign walk refined until no face
/// can refine and its interval re-derivation ([`rederive`]), and each
/// shell's role walk ([`shell_volume_role`]). Test vocabulary: the
/// rows that pin every door to one point per region.
///
/// # Errors
///
/// A walk's refusal, as [`mass_properties`] reports it.
#[cfg(any(test, feature = "test-support"))]
#[doc(hidden)]
pub fn every_door(
    body: &Body<f64>,
    tol: Tol,
    settle: Settle<f64>,
) -> Result<Vec<DoorRead>, MassPropsError> {
    let band = Band::linear(tol).map_err(|error| MassPropsError::Band { error })?;
    let faces = crate::query::all_faces(body);
    let lane = Some(QuadLane::certified());
    let span = |p: &MassProperties<f64>| {
        let e = p.enclosure();
        Some((e.volume_lo, e.volume_hi))
    };
    let mut out = Vec::new();
    let mut door = |door, read: &mut dyn FnMut() -> Result<DoorOutcome, MassPropsError>| {
        let (outcome, reads) = read_points::recorded(body, read);
        out.push(DoorRead {
            door,
            outcome: outcome?,
            reads,
        });
        Ok::<(), MassPropsError>(())
    };
    door("the reporting walk", &mut || {
        Ok(DoorOutcome::Span(span(
            &measure_to(body, &faces, band, tol, (lane, lane), settle)?
                .1
                .props(),
        )))
    })?;
    let mut certificate = None;
    door("the target walk", &mut || {
        let (_, walked) = measure_to(body, &faces, band, tol, (lane, lane), settle)?;
        certificate = Some(walked);
        Ok(DoorOutcome::Span(None))
    })?;
    door("the target walk's continuation", &mut || {
        let walked = certificate.take().ok_or(MassPropsError::Corrupt {
            what: "the target walk ran",
        })?;
        Ok(DoorOutcome::Span(span(&walked.measure(settle)?.enclosure)))
    })?;
    door("a sign walk settled at round 0", &mut || {
        let mut walked =
            SignCertificate::at_round_zero(body, &faces, band, tol, lane, FaceWalk::Mapped)?;
        let _ = walk_set(&mut [&mut walked], lane, |_| Ask::Decided(()));
        certificate = Some(walked);
        Ok(DoorOutcome::Span(None))
    })?;
    door("its continuation to the target", &mut || {
        let walked = certificate.take().ok_or(MassPropsError::Corrupt {
            what: "the sign walk ran",
        })?;
        Ok(DoorOutcome::Span(span(&walked.measure(settle)?.enclosure)))
    })?;
    let mut past = None;
    door("the sign walk refined past the target", &mut || {
        let mut walked =
            SignCertificate::at_round_zero(body, &faces, band, tol, lane, FaceWalk::Mapped)?;
        let _ = walk_set(&mut [&mut walked], lane, |_| Ask::<()>::Refine(vec![true]));
        let props = walked.props();
        past = Some(walked);
        Ok(DoorOutcome::Span(span(&props)))
    })?;
    door("its interval re-derivation", &mut || {
        let walked = past.as_ref().ok_or(MassPropsError::Corrupt {
            what: "the sign walk ran",
        })?;
        let interval = rederive(body, band, tol, QuadLane::certified(), &walked.runs, true)?.volume;
        Ok(DoorOutcome::Span(Some((
            geom_core::Bounds::lo(interval),
            geom_core::Bounds::hi(interval),
        ))))
    })?;
    for (shell, _) in body.shells.iter() {
        door("a shell's role walk", &mut || {
            Ok(DoorOutcome::Role(shell_volume_role(
                body, shell, band, tol, lane,
            )))
        })?;
    }
    Ok(out)
}

/// One door [`every_door`] entered: its name, what it answered, and every
/// face read it made ([`read_points::recorded`]). Test vocabulary.
#[cfg(any(test, feature = "test-support"))]
#[doc(hidden)]
#[derive(Debug)]
pub struct DoorRead {
    /// The door.
    pub door: &'static str,
    /// What it answered.
    pub outcome: DoorOutcome,
    /// Each face it read and whether it read it about its carried
    /// region's point, in the order it read them.
    pub reads: Vec<(FaceKey, bool)>,
}

/// What a door [`every_door`] entered answered. Test vocabulary.
#[cfg(any(test, feature = "test-support"))]
#[doc(hidden)]
#[derive(Debug)]
pub enum DoorOutcome {
    /// A volume enclosure `(lo, hi)`, or `None` for a walk read only by
    /// the door after it.
    Span(Option<(f64, f64)>),
    /// A shell's role.
    Role(Result<ShellRole, ShellClassifyError>),
}

/// Every shell's quadrature regions ([`quadrature_regions`]), in arena
/// order, with each carried region's point and carry
/// ([`References`]; `None` for one that is not carried), read through the scalar's own
/// certified lane ([`AtRestPolicy::quad_lane`]; none carried at a scalar
/// with none). Test vocabulary.
#[cfg(any(test, feature = "test-support"))]
#[doc(hidden)]
#[must_use]
#[allow(clippy::type_complexity)]
pub fn region_carries_of<T: AtRestPolicy>(
    body: &Body<T>,
) -> Vec<(ShellKey, Vec<(Vec<FaceKey>, Option<(Point3<T>, Interval)>)>)> {
    let held: std::collections::BTreeSet<FaceKey> =
        crate::query::all_faces(body).into_iter().collect();
    body.shells
        .iter()
        .map(|(shell, _)| {
            let quadrature = quadrature_regions(body, shell);
            let c = reference_of(body, shell, &quadrature);
            let carried = T::quad_lane().map_or_else(Vec::new, |lane| {
                region_carries(body, c, quadrature.clone(), &held, lane)
            });
            let regions = quadrature
                .into_iter()
                .map(|faces| {
                    let carry = carried
                        .iter()
                        .find(|r| r.faces == faces)
                        .map(|r| (r.point, r.carry));
                    (faces, carry)
                })
                .collect();
            (shell, regions)
        })
        .collect()
}

/// One face's reading as [`mass_properties`] takes it: its flux term
/// about its shell's reference ([`shell_reference`]), or its carried
/// region's ([`References`]), and its area, each with its half-width, and
/// the first round a quadrature face meets the reporting target at
/// (`None` for a closed-form face, or where no round of the schedule meets
/// it). A carried region's first face's term holds the region's whole
/// carry to the shell's reference, as the walk folds it. Test vocabulary:
/// a row that pins one face of a body another face of which refuses.
#[cfg(any(test, feature = "test-support"))]
#[doc(hidden)]
#[derive(Clone, Copy, Debug)]
pub struct FaceReading {
    /// The flux term about the point the face is read about, with its
    /// region's carry where it is a carried region's first face.
    pub flux: f64,
    /// Its half-width.
    pub flux_pad: f64,
    /// The face's area.
    pub area: f64,
    /// Its half-width.
    pub area_pad: f64,
    /// The first round meeting the resolution.
    pub met_at: Option<usize>,
}

/// [`FaceReading`] of `face`, read alone.
///
/// # Errors
///
/// The face's own refusal, as [`mass_properties`] reports it.
#[cfg(any(test, feature = "test-support"))]
#[doc(hidden)]
pub fn face_reading(
    body: &Body<f64>,
    face: FaceKey,
    tol: Tol,
    resolution: f64,
) -> Result<FaceReading, MassPropsError> {
    face_reading_with(body, face, tol, Some(QuadLane::certified()), resolution)
}

/// [`face_reading`] through the structural door's closed forms alone
/// ([`mass_properties_structural`]).
///
/// # Errors
///
/// The face's own refusal, as [`mass_properties_structural`] reports it.
#[cfg(any(test, feature = "test-support"))]
#[doc(hidden)]
pub fn face_reading_structural(
    body: &Body<f64>,
    face: FaceKey,
    tol: Tol,
    resolution: f64,
) -> Result<FaceReading, MassPropsError> {
    face_reading_with(body, face, tol, None, resolution)
}

#[cfg(any(test, feature = "test-support"))]
fn face_reading_with(
    body: &Body<f64>,
    face: FaceKey,
    tol: Tol,
    lane: Option<QuadLane<f64>>,
    resolution: f64,
) -> Result<FaceReading, MassPropsError> {
    let band = Band::linear(tol).map_err(|error| MassPropsError::Band { error })?;
    let faces = crate::query::all_faces(body);
    let hook = round_hook(lane);
    let refs = References::of(body, faces.iter().copied(), band, lane);
    let len = resolution_len(Length::from_meters(resolution), tol)?;
    let read = |window| face_flux(body, face, band, &hook, tol, window, &refs, Some(len));
    let run = read(RoundWindow::SCHEDULE)?;
    let mut met_at = None;
    if run.contribution.enclosure.is_some() {
        // Each lane's schedule ends long before this bound; a lane that
        // did not would read here as a bound reached, not a round met.
        let mut ended = false;
        let mut round = Some(0);
        for _ in 0..64 {
            let Some(at) = round else {
                ended = true;
                break;
            };
            let run = read(RoundWindow::at(at))?;
            if run.meets(len) {
                met_at = Some(at);
                ended = true;
                break;
            }
            round = run.next_round();
        }
        assert!(ended, "a face's schedule ran past 64 rounds");
    }
    let c = run.contribution;
    Ok(FaceReading {
        flux: c.flux,
        flux_pad: c.flux_pad,
        area: c.area,
        area_pad: c.area_pad,
        met_at,
    })
}

/// Every shell of `body` with the point its faces are read about
/// ([`shell_reference`]), in arena order. Test vocabulary.
#[cfg(any(test, feature = "test-support"))]
#[doc(hidden)]
#[must_use]
pub fn shell_references(body: &Body<f64>) -> Vec<(ShellKey, Point3<f64>)> {
    body.shells
        .iter()
        .map(|(key, _)| (key, shell_reference(body, key)))
        .collect()
}

/// Every quadrature face of `body` with the first round its lane meets
/// `resolution` (m) at, each read alone at that round about the
/// point [`mass_properties`] reads it about ([`References`]):
/// `(face, its surface's kind, round)`, `None` where no round of the
/// schedule meets it. Test vocabulary: two poses of one body compare their faces'
/// rounds, which say whether a face's width depends on where it sits.
///
/// # Errors
///
/// As [`mass_properties`], for a face refused outright, or a resolution
/// finer than ε.
#[cfg(any(test, feature = "test-support"))]
#[doc(hidden)]
pub fn quadrature_rounds(
    body: &Body<f64>,
    tol: Tol,
    resolution: f64,
) -> Result<Vec<(FaceKey, geom::SurfaceKind, Option<usize>)>, MassPropsError> {
    let band = Band::linear(tol).map_err(|error| MassPropsError::Band { error })?;
    let faces = crate::query::all_faces(body);
    let hook = round_hook(Some(QuadLane::certified()));
    let refs = References::of(
        body,
        faces.iter().copied(),
        band,
        Some(QuadLane::certified()),
    );
    let len = resolution_len(Length::from_meters(resolution), tol)?;
    let mut rounds = Vec::new();
    for &face in &faces {
        let (mut met, mut quadrature) = (None, false);
        let mut round = Some(0);
        while let Some(at) = round {
            let run = face_flux(
                body,
                face,
                band,
                &hook,
                tol,
                RoundWindow::at(at),
                &refs,
                Some(len),
            )?;
            quadrature = run.contribution.enclosure.is_some();
            if quadrature && run.meets(len) {
                met = Some(at);
                break;
            }
            round = run.next_round();
        }
        if quadrature {
            rounds.push((face, resolve_face(body, face).1.kind(), met));
        }
    }
    Ok(rounds)
}

/// The face walk's sum.
///
/// **The order is the RUNS' order**, and the sum accumulates in it
/// (D9's idiom 2): this fold never re-orders, so whichever walk built
/// the slice decides the vocabulary — face-arena order for the
/// whole-body walks and the continuation, the shell's own face list
/// for [`classify_shells_of`]. The enclosure exists whenever every
/// face produced bounds at all, which a face whose lane stopped short
/// of a resolution still does.
fn fold_runs<T: Decide>(runs: &[FaceRun<T>]) -> MassProperties<T> {
    let mut flux = T::zero();
    let mut area = T::zero();
    let mut flux_pad = 0.0f64;
    let mut area_pad = 0.0f64;
    for run in runs {
        flux = flux + run.contribution.flux;
        area = area + run.contribution.area;
        // The pads are half-widths, summed and divided rounding up so
        // the whole's is no less than the faces' (`0.0` stays exact).
        flux_pad = interval::add_up(flux_pad, run.contribution.flux_pad);
        area_pad = interval::add_up(area_pad, run.contribution.area_pad);
    }
    MassProperties {
        volume: flux / T::from_f64(3.0),
        surface_area: area,
        volume_pad: interval::div_up(flux_pad, 3.0),
        area_pad,
    }
}

/// One face's divergence-theorem contribution, with the certified
/// quadrature half-widths it carries (`0.0` for closed-form faces).
/// The unit shared by the whole-body walk ([`mass_properties`])
/// and the per-shell walk ([`classify_shells`]) — one flux
/// implementation, restricted by choosing which faces to visit, never
/// re-derived.
#[derive(Clone)]
struct FaceFlux<T> {
    flux: T,
    area: T,
    flux_pad: f64,
    area_pad: f64,
    /// The quadrature's flux and area enclosures as the lane returned
    /// them, before [`quad_lane::mid_pad`] rounded them to a midpoint
    /// and a half-width; `None` for a closed-form face, whose
    /// enclosure [`rederive`] re-derives.
    enclosure: Option<(Interval, Interval)>,
}

/// One face's contribution and where its refinement stands — the unit
/// a walk that may be RESUMED carries, one per face in the walk's own
/// order ([`fold_runs`] says which walk uses which).
#[derive(Clone)]
struct FaceRun<T: Real> {
    /// The face this run is of.
    face: FaceKey,
    /// Its contribution: its narrowest enclosure, at the round
    /// [`Self::state`] names.
    contribution: FaceFlux<T>,
    /// Where its refinement stands.
    state: FaceState,
    /// What the face was read about ([`References`]): a continuation
    /// reads it about the same point, with the same carry, and so does an
    /// interval re-derivation.
    about: ReadAbout<T>,
}

/// **Where a face's refinement stands**: one state, minted by
/// [`face_flux_about`] from the lane's answer and moved by
/// [`FaceRun::advance`] alone.
#[derive(Clone, Debug, PartialEq)]
enum FaceState {
    /// A closed-form face: no rounds.
    Closed,
    /// A quadrature face.
    Quad {
        /// The round its kept enclosure was taken at — the round
        /// [`rederive`] reads again.
        taken: usize,
        /// The last round its lane ran.
        ran: usize,
        /// Why the face would stop after `ran`.
        stop: FaceStop,
    },
}

/// **Why a quadrature face would stop after its last round.**
#[derive(Clone, Debug, PartialEq)]
enum FaceStop {
    /// The lane's own stop: [`LaneStop::Refinable`] where a later round
    /// fits in its work wall and can narrow.
    Lane(LaneStop),
    /// A later round refused outright. The face stands at the enclosure
    /// it held, which is sound, and refines no further; the refusal
    /// surfaces only where a reading cannot be made without the face
    /// ([`walk_set`], [`SignCertificate::measure`]).
    Refused(MassPropsError),
}

impl<T: Real> FaceRun<T> {
    /// **The one transition**: this face's next window, `run`, in place —
    /// keeping the narrower enclosure, always, by its flux width, as the
    /// lane keeps it within one call ([`geom_brep::props::quad::narrower`]),
    /// so a face walked one round at a time ends where a whole-schedule
    /// call does.
    ///
    /// Where the face goes next (the round its lane ran and its stop) is
    /// the new window's: it is the round the lane just ran, and a
    /// resumption follows it. The answer (the contribution, and the
    /// round it was taken at) is the kept enclosure's.
    ///
    /// A round refused outright leaves the face at the enclosure it held,
    /// stopped on the refusal ([`FaceStop::Refused`]).
    fn advance(&mut self, next: Result<Self, MassPropsError>) {
        let run = match next {
            Ok(run) => run,
            Err(refusal) => {
                if let FaceState::Quad { stop, .. } = &mut self.state {
                    *stop = FaceStop::Refused(refusal);
                }
                return;
            }
        };
        let held = core::mem::replace(self, run);
        let (Some((was, _)), Some((now, _))) =
            (held.contribution.enclosure, self.contribution.enclosure)
        else {
            return;
        };
        if !geom_brep::props::quad::narrower(was, now) {
            return;
        }
        self.contribution = held.contribution;
        if let (FaceState::Quad { taken, .. }, FaceState::Quad { taken: kept, .. }) =
            (&mut self.state, &held.state)
        {
            *taken = *kept;
        }
    }

    /// The refusal a later round of this face met, where one stopped it.
    fn refused(&self) -> Option<&MassPropsError> {
        match &self.state {
            FaceState::Quad {
                stop: FaceStop::Refused(refusal),
                ..
            } => Some(refusal),
            _ => None,
        }
    }

    /// The last round run, where the face's lane can refine past it.
    fn open_at(&self) -> Option<usize> {
        match self.state {
            FaceState::Quad {
                ran,
                stop: FaceStop::Lane(LaneStop::Refinable),
                ..
            } => Some(ran),
            _ => None,
        }
    }

    /// The round a deciding walk takes this face to next: the one after
    /// [`Self::open_at`].
    fn next_round(&self) -> Option<usize> {
        self.open_at().map(|ran| ran + 1)
    }

    /// The kept enclosure's mean boundary displacement
    /// ([`geom_brep::props::quad::mean_boundary_displacement`]); `0` for
    /// a closed-form face, which carries no width.
    fn width_len(&self) -> Result<f64, PropsError> {
        match self.contribution.enclosure {
            Some((flux, area)) => geom_brep::props::quad::mean_boundary_displacement(flux, area),
            None => Ok(0.0),
        }
    }

    /// Whether this face's kept enclosure meets a resolution of `len`.
    fn meets(&self, len: f64) -> bool {
        self.width_len().is_ok_and(|width| width <= len)
    }

    /// The round a continuation to a resolution of `len` resumes this
    /// face at: a face short of it whose lane can refine.
    fn resumes_to(&self, len: f64) -> Option<usize> {
        self.next_round().filter(|_| !self.meets(len))
    }
}

/// A face and its surface, for a face the walk read out of the arena
/// or a solid's record: a miss of either panics naming it (D2 row 4).
#[track_caller]
fn resolve_face<T: Real>(body: &Body<T>, face_key: FaceKey) -> (&crate::entity::Face, &Surface<T>) {
    let face = crate::live::proven(&body.faces, face_key, crate::entity::EntityId::Face);
    (face, body.face_surface_linked(face_key, face))
}

/// One loop flattened ([`loop_edges`]): its edges and the half-edges
/// walked, in traversal order.
pub(crate) type FlatLoop<T> = (Vec<LoopEdge<T>>, Vec<HalfEdgeKey>);

/// A face's loops as every integrator reads them: each loop's chart
/// image ([`chart_image_edges`]) with its half-edges, the outer first,
/// then the rings, those that bound area
/// ([`geom_brep::chart_image::bounds_area`]). Empty where none does.
fn flat_loops<T: Decide>(
    body: &Body<T>,
    face: &crate::entity::Face,
) -> Result<Vec<FlatLoop<T>>, MassPropsError> {
    let mut loops = Vec::new();
    for &lk in core::iter::once(&face.outer).chain(&face.rings) {
        let image = chart_image_edges(body, lk)?;
        if geom_brep::chart_image::bounds_area(&image.0) {
            loops.push(image);
        }
    }
    Ok(loops)
}

/// A face's loops flattened ([`flat_loops`]), their edges alone.
fn face_loops<T: Decide>(
    body: &Body<T>,
    face: &crate::entity::Face,
) -> Result<Vec<Vec<LoopEdge<T>>>, MassPropsError> {
    Ok(edges_of(&flat_loops(body, face)?))
}

/// Flattened loops' edges alone.
fn edges_of<T: Decide>(loops: &[FlatLoop<T>]) -> Vec<Vec<LoopEdge<T>>> {
    loops.iter().map(|(edges, _)| edges.clone()).collect()
}

/// **A face's closed form**, at whatever scalar its geometry is read
/// at, over every loop: a plane, a cylinder, a torus, a cone, and a
/// sphere (which also reads its sense). The face walk runs it
/// at the walk's scalar, and [`QuadLane`]'s `closed_form` at the
/// interval scalar over the same geometry lifted.
fn closed_form_of<U: Decide>(
    surface: &Surface<U>,
    loops: &[Vec<LoopEdge<U>>],
    sense: bool,
    band: Band,
) -> Result<FaceContribution<U>, PropsError> {
    match *surface {
        // A plane with no loop that bounds area takes the empty rule's
        // dispatch below, which refuses it.
        Surface::Plane { origin, .. } if !loops.is_empty() => planar_face(origin, loops),
        _ => {
            let loops: Vec<&[LoopEdge<U>]> = loops.iter().map(Vec::as_slice).collect();
            curved_face_loops(surface, &loops, sense, band)
        }
    }
}

/// [`closed_form_of`] about `about`: the face's surface and loops
/// carried by `−about`, so its flux is read about that point and its
/// width is the face's own size's, wherever the body sits. A kind with
/// no analytic datum to move refuses ([`quad_lane::translated_surface`];
/// a spline face takes the quadrature, never this).
fn closed_form_about<U: Decide>(
    surface: &Surface<U>,
    loops: &[Vec<LoopEdge<U>>],
    sense: bool,
    band: Band,
    about: Point3<U>,
) -> Result<FaceContribution<U>, PropsError> {
    let unmoved = || PropsError::QuadratureUnsupported {
        what: "a closed-form face whose carrier has no datum to carry to its shell's reference",
    };
    let (moved, loops) = quad_lane::moved_by(surface, loops, about).ok_or_else(unmoved)?;
    closed_form_of(&moved, &loops, sense, band)
}

/// The stored start vertex of the outer loop of the face `face`.
fn face_start<T: Decide>(body: &Body<T>, face: FaceKey) -> Point3<T> {
    use crate::entity::EntityId;
    let (face, _) = resolve_face(body, face);
    let outer = crate::live::proven(&body.loops, face.outer, EntityId::Loop);
    let vertex = match outer.boundary {
        LoopBoundary::Empty { vertex } => vertex,
        LoopBoundary::Cycle { first } => {
            crate::live::proven(&body.half_edges, first, EntityId::HalfEdge).start
        }
    };
    let record = crate::live::proven(&body.vertices, vertex, EntityId::Vertex);
    body.point_of(vertex, record)
}

/// **Whether a face of a shell takes the quadrature**, as the shell's
/// reference and its regions read it: its loops flattened, where they
/// flatten and its surface and carriers take the quadrature
/// ([`takes_quadrature`]). A face whose loops do not flatten is not one
/// the walk reads by quadrature: its own read refuses, typed, in its turn.
fn quadrature_loops<T: Decide>(body: &Body<T>, face: FaceKey) -> Option<Vec<FlatLoop<T>>> {
    let (face, surface) = resolve_face(body, face);
    let loops = flat_loops(body, face).ok()?;
    let carriers = loops
        .iter()
        .flat_map(|(edges, _)| edges)
        .map(|e| &e.carrier);
    takes_quadrature(surface, carriers).then_some(loops)
}

/// **A shell's quadrature regions**: its faces that take the quadrature
/// ([`quadrature_loops`]), grouped by sharing an edge whose vector area
/// [`loop_vector_area`](geom_brep::props::loop_vector_area) does not read
/// ([`vector_area_reads`](geom_brep::props::vector_area_reads): a spiric,
/// a rational spline, or a polynomial one past degree 5), each region in
/// the shell's stored order and the regions in the order their first
/// faces are stored ([`References`] says why the join stops at the other
/// edges). The partition itself is the stored order's no more than the
/// body's edges are; only which face of a region is first is.
fn quadrature_regions<T: Decide>(body: &Body<T>, shell: ShellKey) -> Vec<Vec<FaceKey>> {
    use crate::entity::EntityId;
    let shell = crate::live::proven(&body.shells, shell, EntityId::Shell);
    let quadrature: Vec<(FaceKey, Vec<FlatLoop<T>>)> = shell
        .faces
        .iter()
        .filter_map(|&key| quadrature_loops(body, key).map(|loops| (key, loops)))
        .collect();
    let index: std::collections::BTreeMap<FaceKey, usize> = quadrature
        .iter()
        .enumerate()
        .map(|(i, (key, _))| (*key, i))
        .collect();
    // Over the faces' indices in stored order: a class is rooted at its
    // least index, so each region's first face is its root.
    let mut roots = crate::boolean::zip::Roots::new();
    for (i, (_, loops)) in quadrature.iter().enumerate() {
        let edges = loops.iter().flat_map(|(edges, hes)| edges.iter().zip(hes));
        for (e, &he) in edges {
            if geom_brep::props::vector_area_reads(&e.carrier) {
                continue;
            }
            if let Some(&j) = body
                .mate(he)
                .and_then(|m| body.face_of_half_edge(m))
                .and_then(|mate| index.get(&mate))
            {
                roots.union(i, j);
            }
        }
    }
    let mut regions: std::collections::BTreeMap<usize, Vec<FaceKey>> =
        std::collections::BTreeMap::new();
    for (i, (face, _)) in quadrature.iter().enumerate() {
        regions.entry(roots.find(i)).or_default().push(*face);
    }
    regions.into_values().collect()
}

/// **The point a shell's faces are read about** — its reference, and its
/// first quadrature region's point:
/// the stored start vertex of the outer loop of the shell's first face, in
/// its stored order, that takes the quadrature ([`quadrature_loops`]), or
/// of its first face where none does (the world origin for a shell of no
/// face, which sums nothing). A shell's sum does not depend on the point;
/// each quadrature face's width does, by its lever from it, and only a
/// quadrature face carries one at the walk's scalar. The point is a stored
/// vertex, so moving geometry by it rounds no more than the body's own
/// coordinates do. Which quadrature face is first is the stored order's
/// choice, and the lever is unbounded by it: a shell whose quadrature
/// faces lie a kilometre apart reads pads up to 90× apart across
/// rotations of its face list at 1e-6
/// (`a-shells-pad-still-depends-on-which-quadrature-face-it-stores-first`, closed by PR 5699).
/// A region after the first, far from it, is read about a point of its own
/// and carried here ([`References`]).
#[cfg_attr(not(any(test, feature = "test-support")), expect(dead_code))]
fn shell_reference<T: Decide>(body: &Body<T>, shell: ShellKey) -> Point3<T> {
    reference_of(body, shell, &quadrature_regions(body, shell))
}

/// [`shell_reference`], over the shell's quadrature regions read already.
fn reference_of<T: Decide>(body: &Body<T>, shell: ShellKey, regions: &[Vec<FaceKey>]) -> Point3<T> {
    use crate::entity::EntityId;
    let first = regions.first().map(|region| region[0]).or_else(|| {
        let shell = crate::live::proven(&body.shells, shell, EntityId::Shell);
        shell.faces.first().copied()
    });
    first.map_or_else(Point3::origin, |face| face_start(body, face))
}

/// **The points a door call reads its faces about**, found once per door
/// call: each shell's reference ([`shell_reference`]), and for each of
/// its carried quadrature regions ([`region_carries`]) the region's own
/// point and its carry ([`RegionCarry`]). With them, what each sphere
/// seam region the faces reach says of its members ([`seam_refusals`]).
/// A walk finds them once, and each face's run keeps the point it was
/// read about and its carry ([`ReadAbout`]), so a continuation of the
/// walk and an interval re-derivation of it read each face as the walk
/// did without finding them again.
///
/// # One point per far quadrature region, and its carry
///
/// A quadrature face's width scales with its lever from the point it is
/// read about, and one point per shell is far from a region of the shell
/// that lies far from the shell's first quadrature face
/// (`a-shells-pad-still-depends-on-which-quadrature-face-it-stores-first`, closed by PR 5699:
/// two tori 900 m apart on one bar read pads 90× apart across rotations of
/// the stored face list at 1e-6). So a region `g` may be read about a
/// point of its own, `c_g` ([`region_point`]), and its sum carried to the
/// shell's point `c`. For each face,
/// `∫(p − c)·n dA = ∫(p − c_g)·n dA + (c_g − c)·A⃗_f`, with `A⃗_f = ∫n dA`
/// its outward vector area, so the region's faces sum about `c` to their
/// sum about `c_g` plus `(c_g − c)·Σ_f A⃗_f`.
///
/// That sum is not read off the faces, whose own vector areas are as wide
/// as their quadratures. By Stokes, `A⃗_f = ½∮_{∂f} p × dp`, read along
/// the face's loops as the walk stores them, and an edge between two
/// faces of the region is traversed once by each, in opposite senses, and
/// cancels, so `Σ_f A⃗_f = A⃗(∂g)`: the vector area of the region's
/// boundary, the half-edges of its faces whose mate lies on a face outside
/// it ([`region_edges`]). The carry is `(c_g − c)·A⃗(∂g)`, read by
/// [`loop_vector_area`](geom_brep::props::loop_vector_area) in interval
/// arithmetic over the lifted boundary ([`carry_of`]), and folded into
/// the region's first face's enclosure ([`FaceRun::carried`]).
///
/// # What a region is
///
/// Two quadrature faces are one region only where they share an edge
/// whose vector area
/// [`loop_vector_area`](geom_brep::props::loop_vector_area) does not read
/// ([`vector_area_reads`](geom_brep::props::vector_area_reads), the one
/// spelling of its refusals: a spiric, a rational spline, or a polynomial
/// one past the degree 5 its Gauss inventory covers;
/// [`quadrature_regions`]). Across any other edge (a line, a conic arc, a
/// polynomial spline it covers) each side is its own region, and the edge
/// lies on both boundaries, traversed once each way, so the two carries
/// read it and the sum is the region's whole. So every edge between two
/// regions reads, and a region is no wider than its curved faces and the
/// rims that hold them together. (An edge to a closed-form face that does
/// not read leaves its region uncarried: [`carry_of`] refuses, below.) Joined across every shared edge, a long plane taking the
/// quadrature (a bar's top face cut by a torus) joined everything it
/// touched across its straight edges into one region spanning the bar:
/// a torus on that face and a torus at the bar's far end read pads 456×
/// apart across rotations at 1e-6, and a torus midway, joining both
/// ends, 83×
/// (`a-shells-pad-still-depends-on-face-order-where-one-quadrature-region-spans-the-lever`, closed by PR 5801).
///
/// **Its point** is the stored start of a curved face of it ([`region_point`]),
/// not of a plane: a curved face's width is levered by its distance from
/// the point, and the long top face's own start may lie a bar's length
/// from the torus it carries. Measured, that choice is what matters: read
/// about its first face's start, the tori on a bar's top face read pads
/// more than 1.1× apart across rotations
/// (`a_shell_joined_along_a_long_face_reads_alike_whichever_face_it_stores_first`
/// goes red), and about a curved face's, within 1.008× at each ε. Which
/// curved face is not: its last one's start reads every row's pads
/// within their pins too. The first, in the shell's stored order, is taken
/// so that the point is a function of that order alone.
///
/// **The first region is read about `c`**, the start of its first face,
/// which may be a long plane's. That rests on measurement, not on an
/// argument: read about its own point and carried, the first region pads
/// the same as about `c` to six digits across every rotation of a torus on
/// a bar's top face, one midway on it, and one astride the bar's end edge
/// (whose rims lie on two planes, the end cap's and the top face's), at
/// each ε.
///
/// # Only where it narrows
///
/// The carry is not free: its interval rounds at `|c_g − c|·|A⃗(∂g)|`,
/// and the region's faces read about `c_g` round at their own extent
/// from it. Where the region lies near `c` those are the widths the faces
/// had about `c`, and the carry only adds to them: carried ungated, four
/// step-import shells read pads up to 5.2× wider than before regions. So
/// a region is carried only where its least lever from `c` is at least
/// [`CARRY_REACH`] times its greatest from `c_g` ([`far_from`]), over the
/// samples of its faces' edges: a curved face's sample levered by its
/// distance, a plane's by the point's distance off the plane, and the
/// least lever read over the curved faces where the region has any. Two
/// tori 900 apart on one bar read about 53. Every door reads the same
/// [`References`], so all of them carry the same regions.
///
/// A region is read about `c`, uncarried, where it is the first, where it
/// is not that far, where the door does not hold every face of it or
/// holds no quadrature lane (a carry stands for the whole region, and only
/// a lane encloses it), where its boundary does not resolve, and where
/// its boundary's vector area does not read or its carry does not certify
/// (a cone's spiric or rational rim, which its own closed form refuses
/// too).
struct References<T: Decide> {
    shells: std::collections::BTreeMap<ShellKey, Point3<T>>,
    regions: std::collections::BTreeMap<FaceKey, Point3<T>>,
    carries: std::collections::BTreeMap<FaceKey, Interval>,
    seams: std::collections::BTreeMap<FaceKey, MassPropsError>,
}

/// How far a quadrature region must lie from its shell's point, in its
/// own extent, to be carried ([`References`]).
const CARRY_REACH: f64 = 16.0;

/// One carried quadrature region of a shell: its faces, its point `c_g`,
/// and its carry `(c_g − c)·A⃗(∂g)` to the shell's point `c`
/// ([`References`]).
struct RegionCarry<T: Decide> {
    faces: Vec<FaceKey>,
    point: Point3<T>,
    carry: Interval,
}

impl<T: Decide> References<T> {
    /// The references of the shells `faces` lie on, the carried regions
    /// among them through `lane`, and the seam regions they reach, read
    /// in `band`.
    fn of(
        body: &Body<T>,
        faces: impl IntoIterator<Item = FaceKey>,
        band: Band,
        lane: Option<QuadLane<T>>,
    ) -> Self {
        let faces: Vec<FaceKey> = faces.into_iter().collect();
        Self {
            seams: seam_refusals(body, &faces, band),
            ..Self::points(body, faces.iter().copied(), lane)
        }
    }

    /// The references alone, with no seam region read.
    fn points(
        body: &Body<T>,
        faces: impl IntoIterator<Item = FaceKey>,
        lane: Option<QuadLane<T>>,
    ) -> Self {
        let held: std::collections::BTreeSet<FaceKey> = faces.into_iter().collect();
        let mut shells = std::collections::BTreeMap::new();
        let mut regions = std::collections::BTreeMap::new();
        let mut carries = std::collections::BTreeMap::new();
        for &key in &held {
            // A face that does not resolve is one the caller never reads:
            // reading it is what refuses it, at [`References::at`].
            let Some(shell) = body.get_face(key).map(|face| face.shell) else {
                continue;
            };
            if shells.contains_key(&shell) {
                continue;
            }
            let quadrature = quadrature_regions(body, shell);
            let c = reference_of(body, shell, &quadrature);
            shells.insert(shell, c);
            let Some(lane) = lane else { continue };
            for region in region_carries(body, c, quadrature, &held, lane) {
                carries.insert(region.faces[0], region.carry);
                for face in region.faces {
                    regions.insert(face, region.point);
                }
            }
        }
        Self {
            shells,
            regions,
            carries,
            seams: std::collections::BTreeMap::new(),
        }
    }

    /// What `face` is read about: its carried region's point, with the
    /// region's carry on its first face, else its shell's reference. A
    /// face whose shell the door did not hold is a door that read a face
    /// it did not hand [`References::of`], refused as corrupt; a face its
    /// seam region refuses is refused here, before any lane reads it.
    fn at(&self, body: &Body<T>, face: FaceKey) -> Result<ReadAbout<T>, MassPropsError> {
        if let Some(refusal) = self.seams.get(&face) {
            return Err(refusal.clone());
        }
        if let Some(&point) = self.regions.get(&face) {
            return Ok(ReadAbout::Region {
                point,
                carry: self.carries.get(&face).copied(),
            });
        }
        self.shells
            .get(&resolve_face(body, face).0.shell)
            .map(|&point| ReadAbout::Shell(point))
            .ok_or(MassPropsError::Corrupt {
                what: "a walk read a face of a shell it holds no reference for",
            })
    }
}

/// **The point a face's run was read about** ([`References`]), kept on
/// the run so a continuation and a re-derivation read the face about it
/// again.
#[derive(Clone, Copy, Debug)]
enum ReadAbout<T: Real> {
    /// Its shell's reference.
    Shell(Point3<T>),
    /// Its carried quadrature region's point, and on the region's first
    /// face the region's carry to its shell's reference.
    Region {
        point: Point3<T>,
        carry: Option<Interval>,
    },
}

impl<T: Real> ReadAbout<T> {
    /// The point.
    fn point(self) -> Point3<T> {
        match self {
            Self::Shell(point) | Self::Region { point, .. } => point,
        }
    }

    /// The carry the face takes, as its carried region's first face.
    fn carry(self) -> Option<Interval> {
        match self {
            Self::Shell(_) => None,
            Self::Region { carry, .. } => carry,
        }
    }
}

/// The carried regions among a shell's quadrature regions `quadrature`
/// ([`References`]): each region after the first whose every face is
/// `held` and that lies far from the shell's point `c` ([`far_from`]),
/// with its point ([`region_point`]) and its carry to `c`, read through
/// `lane`.
fn region_carries<T: Decide>(
    body: &Body<T>,
    c: Point3<T>,
    quadrature: Vec<Vec<FaceKey>>,
    held: &std::collections::BTreeSet<FaceKey>,
    lane: QuadLane<T>,
) -> Vec<RegionCarry<T>> {
    let lift = |p: Point3<T>| Point3::new(lane.lift(p.x), lane.lift(p.y), lane.lift(p.z));
    let lift_edge = |e: &LoopEdge<T>| LoopEdge {
        carrier: e.carrier.map_scalar(lane.lift),
        carrier_id: e.carrier_id,
        t0: lane.lift(e.t0),
        t1: lane.lift(e.t1),
        forward: e.forward,
        start: e.start,
        end: e.end,
    };
    let mut out = Vec::new();
    for faces in quadrature.into_iter().skip(1) {
        if !faces.iter().all(|f| held.contains(f)) {
            continue;
        }
        let Some((boundary, every)) = region_edges(body, &faces) else {
            continue;
        };
        let point = region_point(body, &faces);
        let every: Vec<(LoopEdge<Interval>, Option<geom_core::Vec3<Interval>>)> = every
            .iter()
            .map(|(e, plane)| (lift_edge(e), plane.map(|n| n.map(lane.lift))))
            .collect();
        if !far_from(&every, lift(point), lift(c)) {
            continue;
        }
        let boundary: Vec<LoopEdge<Interval>> = boundary.iter().map(lift_edge).collect();
        let Some(carry) = carry_of(&boundary, lift(point), lift(c)) else {
            continue;
        };
        out.push(RegionCarry {
            faces,
            point,
            carry,
        });
    }
    out
}

/// **The point a region is read about** ([`References`]): the stored
/// start vertex of its first face, in the shell's stored order, that is
/// not a plane, or of its first face where every face is one. A curved
/// face's width is levered by its distance from the point, a plane's only
/// by the point's distance off its plane.
fn region_point<T: Decide>(body: &Body<T>, faces: &[FaceKey]) -> Point3<T> {
    let curved = faces
        .iter()
        .copied()
        .find(|&f| !matches!(resolve_face(body, f).1, Surface::Plane { .. }));
    face_start(body, curved.unwrap_or(faces[0]))
}

/// **Whether a region lies far enough from its shell's point `c` to be
/// carried** ([`References`]): over its faces' edges `every`, each read
/// at its two ends and its middle parameter, the least squared lever from
/// `c` at least [`CARRY_REACH`]² times the greatest from `c_g`, each read
/// at the end of its interval that makes it hardest to meet. A sample is
/// levered by its distance from the point, or, on a plane face (its unit
/// normal beside it), by the point's distance off the plane along it:
/// that is all a plane's width scales with. The least lever is read over
/// the samples of the region's curved faces where it has any, every plane
/// dropped, not only one through `c`. The gate decides width, not
/// soundness: a carried region's enclosure is as sound as an uncarried
/// one's, so dropping samples (which can only raise the least lever, and
/// carry more) risks only a wider pad. The planes are dropped because a
/// region's planes are held to it by its curved faces' rims, so the curved
/// faces are what reading about `c_g` narrows. Measured over sweep and
/// step-import at each ε, every reading these rules move traces to the
/// partition or the point, none to the drop.
fn far_from(
    every: &[(LoopEdge<Interval>, Option<geom_core::Vec3<Interval>>)],
    c_g: Point3<Interval>,
    c: Point3<Interval>,
) -> bool {
    use geom_core::Bounds as _;
    let lever =
        |p: Point3<Interval>, x: Point3<Interval>, plane: Option<geom_core::Vec3<Interval>>| {
            plane.map_or_else(|| p.distance_squared(x), |n| (p - x).dot(n).powi(2))
        };
    let curved = every.iter().any(|(_, plane)| plane.is_none());
    let (mut near, mut extent) = (f64::INFINITY, 0.0_f64);
    for (e, plane) in every {
        let mid = (e.t0 + e.t1) * Interval::from_f64(0.5);
        for t in [e.t0, mid, e.t1] {
            let p = e.carrier.eval(t);
            let (to_c, to_g) = (lever(p, c, *plane).lo(), lever(p, c_g, *plane).hi());
            // A sample that does not read leaves the region near.
            if to_c.is_nan() || to_g.is_nan() {
                return false;
            }
            if plane.is_none() || !curved {
                near = near.min(to_c);
            }
            extent = extent.max(to_g);
        }
    }
    near >= CARRY_REACH * CARRY_REACH * extent
}

/// **A region's carry** `(c_g − c)·A⃗(∂g)` over its lifted boundary
/// `edges`, its vector area read about `c_g` ([`References`]); `None`
/// where the boundary's vector area does not read or the carry does not
/// certify, so the region is read about `c`.
fn carry_of(
    edges: &[LoopEdge<Interval>],
    c_g: Point3<Interval>,
    c: Point3<Interval>,
) -> Option<Interval> {
    let va = geom_brep::props::loop_vector_area(edges, c_g).ok()?;
    let carry = (c_g - c).dot(va);
    carry.is_certified().then_some(carry)
}

/// A region's edges: its boundary `∂g`, the loop edges of its faces whose
/// half-edge's mate lies on a face outside it, each in its own face's
/// traversal sense ([`References`]), and every loop edge of its faces,
/// each with its face's unit normal where the face is a plane
/// ([`far_from`]). `None` where a face's loops do not flatten or a mate
/// does not resolve.
#[allow(clippy::type_complexity)]
fn region_edges<T: Decide>(
    body: &Body<T>,
    faces: &[FaceKey],
) -> Option<(
    Vec<LoopEdge<T>>,
    Vec<(LoopEdge<T>, Option<geom_core::Vec3<T>>)>,
)> {
    let (mut boundary, mut every) = (Vec::new(), Vec::new());
    for &key in faces {
        let (face, surface) = resolve_face(body, key);
        let plane = match *surface {
            Surface::Plane { normal, .. } => Some(normal),
            _ => None,
        };
        for (edges, hes) in flat_loops(body, face).ok()? {
            for (e, he) in edges.into_iter().zip(hes) {
                let mate_face = body.mate(he).and_then(|m| body.face_of_half_edge(m))?;
                if !faces.contains(&mate_face) {
                    boundary.push(e.clone());
                }
                every.push((e, plane));
            }
        }
    }
    Some((boundary, every))
}

/// **A sphere seam region, read once** for tier 3's check 6 and the
/// measurement door alike: its members, each member's bit and run of
/// loops, the loops both its reads take ([`geom_brep::props::SeamLoops`];
/// `None` for a pinched region or one whose loops do not parse, either of
/// which is left unread), its cover census, and the side the pole read
/// gives it, read at most once ([`ReadRegion::side`]).
pub(crate) struct ReadRegion<T: Real> {
    pub(crate) members: Vec<FaceKey>,
    pub(crate) claims: Vec<(bool, core::ops::Range<usize>)>,
    pub(crate) loops: Option<geom_brep::props::SeamLoops<T>>,
    /// The census (`geom_brep::props::sphere_region_cover`), for a region
    /// of two faces or more whose loops parse. A region of one face is
    /// not read: its boundary is the face's own loops, so its census is
    /// zero by construction.
    pub(crate) cover: Option<Result<geom_brep::props::SeamCover, PropsError>>,
    /// What the walk could not follow, on a body whose structure is
    /// corrupt ([`seam_region`]'s `Err`): the region is then its start
    /// face alone, unread.
    pub(crate) corrupt: Option<&'static str>,
    side: Option<Option<geom_brep::props::MaterialSign>>,
}

impl<T: Decide> ReadRegion<T> {
    /// The side the pole read gives the region
    /// (`geom_brep::props::sphere_region_material_sign`), read once; `None`
    /// where it is unread or refuses.
    pub(crate) fn side(&mut self, band: Band) -> Option<geom_brep::props::MaterialSign> {
        let loops = self.loops.as_ref();
        *self.side.get_or_insert_with(|| {
            loops.and_then(|loops| geom_brep::props::sphere_region_material_sign(loops, band).ok())
        })
    }
}

/// **Every seam region of a sphere that `faces` reach**, each read once
/// ([`ReadRegion`]), whatever its members' own boundaries encode: the
/// census reads no pole, so a region whose every member reads its own
/// side is read as one whose members encode none. A region whose loops
/// do not flatten ([`seam_region`]'s `None`), or whose structure the walk
/// cannot follow ([`ReadRegion::corrupt`]), is a region of that face
/// alone, unread.
pub(crate) fn read_seam_regions<T: Decide>(
    body: &Body<T>,
    faces: impl IntoIterator<Item = FaceKey>,
    band: Band,
) -> Vec<ReadRegion<T>> {
    let mut regions: Vec<ReadRegion<T>> = Vec::new();
    for key in faces {
        let (_, surface) = resolve_face(body, key);
        if !matches!(surface, Surface::Sphere { .. })
            || regions.iter().any(|r| r.members.contains(&key))
        {
            continue;
        }
        let region = match seam_region(body, key) {
            Ok(Some(region)) => region,
            unread => {
                regions.push(ReadRegion {
                    members: vec![key],
                    claims: Vec::new(),
                    loops: None,
                    cover: None,
                    corrupt: unread.err(),
                    side: None,
                });
                continue;
            }
        };
        let loops = (!region.pinched)
            .then(|| {
                let boundary: Vec<&[_]> = region.boundary.iter().map(Vec::as_slice).collect();
                let held: Vec<&[_]> = region.loops.iter().map(Vec::as_slice).collect();
                geom_brep::props::seam_loops(surface, &boundary, &held, band).ok()
            })
            .flatten();
        let cover = loops
            .as_ref()
            .filter(|_| region.members.len() > 1)
            .map(|loops| geom_brep::props::sphere_region_cover(loops, &region.claims, band));
        regions.push(ReadRegion {
            members: region.members,
            claims: region.claims,
            loops,
            cover,
            corrupt: None,
            side: None,
        });
    }
    regions
}

/// **What the seam regions `faces` reach say of their members at the
/// door**, as tier 3's check 6 reads them ([`read_seam_regions`]), by
/// member, each member's own finding named over the region's:
///
/// 1. a member whose own boundary encodes a side its bit contradicts is
///    left to the flux arm, which refuses it by its own variant
///    ([`PropsError::SenseContradicted`], or [`PropsError::SenseInverted`]
///    for a ringed one), so a ringed member's flip needs nothing here;
/// 2. a member whose own boundary encodes no side, and whose bit
///    contradicts the region's pole read, refuses
///    [`PropsError::SenseContradicted`], the refusal an encoded face's own
///    contradicted bit earns in the flux arm;
/// 3. an over-cover refuses every other member, whatever its own boundary
///    encodes ([`PropsError::SeamRegionOvercovered`]), as does a census in
///    the band ([`PropsError::SeamRegionCoverUnresolved`]);
/// 4. members whose bits disagree are the lamina tier 3's check 4 names
///    (`ValidationError::LaminaWedge`), which the door does not run. Where
///    the pole read encodes the region's side, the members it contradicts
///    are refused by (1) and (2), and no member whose bit agrees is named.
///    Where it encodes none, each member whose own boundary encodes no
///    side refuses [`PropsError::SeamRegionSensesDisagree`].
///
/// A region with no boundary (its members close the sphere) is the case
/// where (4) cannot tell the flipped member from the others: nothing on
/// it encodes a side (`sphere_region_material_sign`'s `Unencoded`), and
/// flipping either set of members would make a ball of it. Each member
/// whose own boundary reads and encodes no side refuses
/// [`PropsError::SeamRegionSensesDisagree`], so the door names the first
/// of them in arena order, as it names any body's first refusing face.
/// The member named is one face of the lamina check 4 names, not
/// necessarily the one whose bit is wrong.
///
/// The match reads a member's own boundary first, in every region:
/// - a member whose own read encodes a side its bit contradicts is the
///   flux arm's (1), even where the census over-covers;
/// - a member whose own read fails (its loops do not flatten, or their
///   side does not decide) is given no seam refusal: it is not one that
///   "encodes no side", and what its own flux arm reads of it stands.
///
/// So an over-cover (3) refuses every member but those two kinds.
///
/// A region whose structure the walk cannot follow refuses its start face
/// [`MassPropsError::Corrupt`], typed, where tier 1 would refuse the body.
///
/// The precedence rests on the member's own read and the region's pole
/// read agreeing wherever both encode a side: both read the same loops'
/// traversal against the same chart, so a member's bit that agrees with
/// one agrees with the other on a body whose seam traversals agree, which
/// check 4 owns.
fn seam_refusals<T: Decide>(
    body: &Body<T>,
    faces: &[FaceKey],
    band: Band,
) -> std::collections::BTreeMap<FaceKey, MassPropsError> {
    use geom_brep::props::{MaterialSign, SeamCover};
    let own = |member: FaceKey| {
        let (rec, surface) = resolve_face(body, member);
        face_loops(body, rec).ok().and_then(|loops| {
            let loops: Vec<&[LoopEdge<T>]> = loops.iter().map(Vec::as_slice).collect();
            geom_brep::props::boundary_material_sign_loops(surface, &loops, band).ok()
        })
    };
    let mut seams = std::collections::BTreeMap::new();
    for mut region in read_seam_regions(body, faces.iter().copied(), band) {
        if let (Some(what), Some(&face)) = (region.corrupt, region.members.first()) {
            seams.insert(face, MassPropsError::Corrupt { what });
            continue;
        }
        let side = region.side(band);
        let cover = region.cover.clone();
        for (&member, &(sense, _)) in region.members.iter().zip(&region.claims) {
            // Whether a read side contradicts this member's bit.
            let against = |side: MaterialSign| match side {
                MaterialSign::Encoded(s) => (s == Sign::Positive) != sense,
                MaterialSign::Unencoded => false,
            };
            let refusal = match (own(member), side, &cover) {
                (Some(own), _, _) if against(own) => None,
                (Some(MaterialSign::Unencoded), Some(side), _) if against(side) => {
                    Some(PropsError::SenseContradicted)
                }
                (_, _, Some(Ok(SeamCover::Over { times }))) => {
                    Some(PropsError::SeamRegionOvercovered { times: *times })
                }
                (_, _, Some(Ok(SeamCover::Unresolved))) => {
                    Some(PropsError::SeamRegionCoverUnresolved)
                }
                (_, Some(MaterialSign::Encoded(_)), Some(Ok(SeamCover::MixedSense))) => None,
                (Some(MaterialSign::Unencoded), _, Some(Ok(SeamCover::MixedSense))) => {
                    Some(PropsError::SeamRegionSensesDisagree)
                }
                _ => None,
            };
            if let Some(source) = refusal {
                seams.insert(
                    member,
                    MassPropsError::Face {
                        face: member,
                        source,
                    },
                );
            }
        }
    }
    seams
}

/// **A face's seam region**: the faces on its surface entity it reaches
/// across shared edges, every loop of each, and the region's boundary,
/// the members' half-edges whose mates lie outside it, chained into
/// cycles ([`seam_region`]).
pub(crate) struct SeamRegion<T: Real> {
    pub(crate) members: Vec<FaceKey>,
    pub(crate) loops: Vec<Vec<geom_brep::props::LoopEdge<T>>>,
    /// Each member's sense bit and the run of `loops` that is its own,
    /// in `members` order.
    pub(crate) claims: Vec<(bool, core::ops::Range<usize>)>,
    pub(crate) boundary: Vec<Vec<geom_brep::props::LoopEdge<T>>>,
    /// Some boundary cycle passes through one vertex twice: the region
    /// touches itself there, and its boundary is no set of simple loops
    /// the side readers take. No body reaches this today: the face that
    /// meets the region at the vertex alone would lie on a second sphere
    /// entity coincident with the first, which `mef` refuses
    /// (`RechartUnvouched`) and STEP import merges into one entity
    /// (`work/measuredoor/a-pinched-seam-region-has-no-witness.md`).
    pub(crate) pinched: bool,
}

/// [`SeamRegion`] of `start`. `Ok(None)` where the flattening refuses
/// (a scaffold edge at rest), which leaves the region unread as a face's
/// refused derivation does. A body whose structure the walk cannot
/// follow (a key that does not resolve, a vertex whose half-edges form
/// no finite rotation, a boundary half-edge with no one successor) is
/// `Err`, naming what failed: tier 1 refuses such a body, and a door that
/// runs no tier 1 refuses it typed by that cause
/// ([`MassPropsError::Corrupt`]).
pub(crate) fn seam_region<T: Decide>(
    body: &Body<T>,
    start: FaceKey,
) -> Result<Option<SeamRegion<T>>, &'static str> {
    let unresolved = "a seam region walked a key that does not resolve";
    let face = |f: FaceKey| body.faces.get(f).ok_or(unresolved);
    let half_edge = |h: HalfEdgeKey| body.half_edges.get(h).ok_or(unresolved);
    let across = |h: HalfEdgeKey| {
        body.mate(h)
            .and_then(|m| body.face_of_half_edge(m))
            .ok_or(unresolved)
    };
    let carrier = face(start)?.surface;
    let mut members = vec![start];
    let mut loops = Vec::new();
    let mut claims = Vec::new();
    let mut half_edges = Vec::new();
    let mut i = 0;
    while let Some(&f) = members.get(i) {
        let rec = face(f)?;
        let first = loops.len();
        for lk in core::iter::once(rec.outer).chain(rec.rings.iter().copied()) {
            let Ok((edges, hes)) = loop_edges(body, lk) else {
                return Ok(None);
            };
            for &h in &hes {
                let other = across(h)?;
                if face(other)?.surface == carrier && !members.contains(&other) {
                    members.push(other);
                }
            }
            loops.push(edges);
            half_edges.extend(hes);
        }
        claims.push((rec.sense, first..loops.len()));
        i += 1;
    }
    let mut inside_of = Vec::with_capacity(half_edges.len());
    for &h in &half_edges {
        inside_of.push((h, members.contains(&across(h)?)));
    }
    let inside =
        |h: HalfEdgeKey| -> Result<bool, &'static str> { Ok(members.contains(&across(h)?)) };
    let exits: Vec<HalfEdgeKey> = inside_of
        .iter()
        .filter(|(_, interior)| !interior)
        .map(|&(h, _)| h)
        .collect();
    let mut boundary = Vec::new();
    let mut pinched = false;
    for cycle in crate::surface_boundary::boundary_cycles(body, exits, half_edges.len(), inside)? {
        let mut starts = Vec::with_capacity(cycle.len());
        for &h in &cycle {
            starts.push(half_edge(h)?.start);
        }
        pinched |= starts
            .iter()
            .enumerate()
            .any(|(i, v)| starts[..i].contains(v));
        let Ok(run) = run_edges(body, &cycle) else {
            return Ok(None);
        };
        boundary.push(run);
    }
    Ok(Some(SeamRegion {
        members,
        loops,
        claims,
        boundary,
        pinched,
    }))
}

/// **Whether a face's flux is read by the certified quadrature** rather
/// than a closed form: a plane or a cone bounded by a spiric or a
/// rational spline rim on any loop (the boundary's vector area its
/// closed form reads is an elliptic integral, or a rational one:
/// [`curved_rim`]), a curved face other than a cone trimmed by an
/// ellipse, a spiric or a spline, and a described spline face whatever
/// bounds it. Structural, on the surface and carrier kinds,
/// never a runtime fallback: [`face_flux_about`] dispatches on it and
/// [`shell_reference`] reads it.
fn takes_quadrature<'a, T: Decide + 'a>(
    surface: &Surface<T>,
    carriers: impl Iterator<Item = &'a geom::Curve3<T>> + Clone,
) -> bool {
    let carriers = || carriers.clone();
    match surface {
        Surface::Plane { .. } | Surface::Cone { .. } => carriers().any(curved_rim),
        _ => {
            surface.spline_chart().is_some()
                || (!matches!(surface, Surface::Cone { .. }) && carriers().any(trims))
        }
    }
}

/// Whether a carrier takes a plane off its closed form: a spiric, or a
/// spline rim with varying weights, whose vector-area term is a rational
/// integrand no fixed rule integrates exactly
/// ([`geom_brep::props::rational_rim`]).
fn curved_rim<T: Decide>(carrier: &geom::Curve3<T>) -> bool {
    match carrier {
        geom::Curve3::Spiric { .. } => true,
        geom::Curve3::Nurbs(c) => geom_brep::props::is_rational(c),
        geom::Curve3::Line { .. } | geom::Curve3::Circle { .. } | geom::Curve3::Ellipse { .. } => {
            false
        }
    }
}

/// Whether a carrier trims a curved face off its closed form: an ellipse,
/// a spiric or a spline, where a line or a circle keeps it.
fn trims<T: Decide>(carrier: &geom::Curve3<T>) -> bool {
    match carrier {
        geom::Curve3::Line { .. } | geom::Curve3::Circle { .. } => false,
        geom::Curve3::Ellipse { .. } | geom::Curve3::Spiric { .. } | geom::Curve3::Nurbs(_) => true,
    }
}

/// **Which point each face read was taken about**, as a test reads it:
/// [`face_flux`] and [`rederive_faces`] note every face they read, the body
/// they read it on, and whether they read it about its carried region's
/// point ([`References`]),
/// while a [`read_points::recorded`] call is open; the recording keeps the
/// reads of the body it names, so it sees its own door's reads and no
/// other test's. Test vocabulary: the rows that pin every door to one
/// point per region.
#[cfg(any(test, feature = "test-support"))]
pub mod read_points {
    use crate::entity::FaceKey;
    use std::sync::Mutex;
    use std::sync::atomic::{AtomicUsize, Ordering};

    static OPEN: AtomicUsize = AtomicUsize::new(0);
    static READS: Mutex<Vec<(usize, FaceKey, bool)>> = Mutex::new(Vec::new());
    static GATE: Mutex<()> = Mutex::new(());

    pub(super) fn note<T: geom_core::Real>(body: &crate::Body<T>, face: FaceKey, regional: bool) {
        if OPEN.load(Ordering::Acquire) > 0 {
            READS
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner)
                .push((core::ptr::from_ref(body).addr(), face, regional));
        }
    }

    /// `door` run with every face read it made of `body`: the face and
    /// whether it was read about its carried region's point, in the order
    /// the reads were noted.
    pub fn recorded<T: geom_core::Real, R>(
        body: &crate::Body<T>,
        door: impl FnOnce() -> R,
    ) -> (R, Vec<(FaceKey, bool)>) {
        let _gate = GATE
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner);
        READS
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
            .clear();
        OPEN.fetch_add(1, Ordering::AcqRel);
        let out = door();
        OPEN.fetch_sub(1, Ordering::AcqRel);
        let at = core::ptr::from_ref(body).addr();
        let reads = core::mem::take(
            &mut *READS
                .lock()
                .unwrap_or_else(std::sync::PoisonError::into_inner),
        )
        .into_iter()
        .filter(|&(on, _, _)| on == at)
        .map(|(_, face, regional)| (face, regional))
        .collect();
        (out, reads)
    }
}

/// The per-face body of the flux walk (module docs), about the point
/// `refs` holds for the face (its shell's reference, or its carried
/// quadrature region's), with the region's carry on the region's first
/// face ([`References`]).
#[allow(clippy::too_many_arguments)]
fn face_flux<T: Decide>(
    body: &Body<T>,
    face_key: FaceKey,
    band: Band,
    quad: &QuadHook<'_, T>,
    tol: Tol,
    window: RoundWindow,
    refs: &References<T>,
    resolution: Option<f64>,
) -> Result<FaceRun<T>, MassPropsError> {
    face_flux_read(
        body,
        face_key,
        band,
        quad,
        tol,
        window,
        refs.at(body, face_key)?,
        resolution,
    )
}

/// [`face_flux`] about what `about` says the face is read about: a
/// walk's first read, through [`References::at`], and a continuation's,
/// through the run it continues ([`FaceRun::about`]).
#[allow(clippy::too_many_arguments)]
fn face_flux_read<T: Decide>(
    body: &Body<T>,
    face_key: FaceKey,
    band: Band,
    quad: &QuadHook<'_, T>,
    tol: Tol,
    window: RoundWindow,
    about: ReadAbout<T>,
    resolution: Option<f64>,
) -> Result<FaceRun<T>, MassPropsError> {
    let mut run = face_flux_about(
        body,
        face_key,
        band,
        quad,
        tol,
        window,
        about.point(),
        resolution,
    )?;
    run.about = about;
    #[cfg(any(test, feature = "test-support"))]
    read_points::note(body, face_key, matches!(about, ReadAbout::Region { .. }));
    Ok(match about.carry() {
        Some(carry) => run.carried(carry),
        None => run,
    })
}

/// [`face_flux`] about `about`: resolve the surface, flatten the loops,
/// dispatch closed form vs certified quadrature over the round window
/// asked for, each round asked `resolution` (the lane's own question,
/// `quad_lane::cut_face_rounds`). Every refusal that leaves the face
/// with NO enclosure is a typed [`MassPropsError`]; a lane that stopped
/// short of the resolution leaves one, and the returned [`FaceRun`]'s
/// state says where it stopped.
#[allow(clippy::too_many_arguments)]
fn face_flux_about<T: Decide>(
    body: &Body<T>,
    face_key: FaceKey,
    band: Band,
    quad: &QuadHook<'_, T>,
    tol: Tol,
    window: RoundWindow,
    about: Point3<T>,
    resolution: Option<f64>,
) -> Result<FaceRun<T>, MassPropsError> {
    let (face, surface) = resolve_face(body, face_key);
    let wrap = |source| MassPropsError::Face {
        face: face_key,
        source,
    };
    let mut flux_pad = 0.0f64;
    let mut area_pad = 0.0f64;
    let mut enclosure = None;
    let mut state = FaceState::Closed;
    let loops = flat_loops(body, face)?;
    let plane_loops = match surface {
        Surface::Plane { .. } => Some(edges_of(&loops)),
        _ => None,
    };
    let contribution: FaceContribution<T> = match plane_loops {
        // No loop bounds area: the empty rule's one dispatch reads the
        // face whole, or refuses it on a surface that is not closed or
        // has no closed form (`geom_brep::props::whole_surface`).
        _ if loops.is_empty() => {
            geom_brep::props::curved_face_loops(surface, &[], face.sense, band).map_err(wrap)?
        }
        Some(loops) if !takes_quadrature(surface, loops.iter().flatten().map(|e| &e.carrier)) => {
            closed_form_about(surface, &loops, face.sense, band, about).map_err(wrap)?
        }
        _ => {
            let trimmed = loops
                .iter()
                .flat_map(|(edges, _)| edges)
                .any(|e| trims(&e.carrier));
            // Which lanes read a ring: the cone's closed form (every
            // loop's vector area, whatever trims it), the cylinder's
            // closed form and quadrature alike (each a Green form summed
            // over every loop), and the torus's closed form over rims
            // and rulings, the sphere's over circles (Gauss–Bonnet over
            // every loop), and the spiric lane every loop of a
            // spiric-bounded plane or torus wall: on a wall, every loop's
            // spirics, rims and meridians, each ring wound against the
            // face, and any other edge refuses typed in that lane. No
            // other kind reads one.
            let spiric = loops
                .iter()
                .flat_map(|(edges, _)| edges)
                .any(|e| matches!(e.carrier, geom::Curve3::Spiric { .. }));
            let reads_rings = match surface {
                // A plane reaches this arm only bounded by a spiric or a
                // rational spline rim, and the plane cap reads every loop
                // of its face.
                Surface::Cone { .. } | Surface::Cylinder { .. } | Surface::Plane { .. } => true,
                Surface::Torus { .. } => !trimmed || spiric,
                Surface::Sphere { .. } => !trimmed,
                _ => false,
            };
            if loops.len() > 1 && !reads_rings {
                return Err(MassPropsError::RingOnCurvedFace { face: face_key });
            }
            // The S10 sense bit does not enter the quadrature lane: its
            // Green form is winding-derived end to end (the signed UV area
            // IS s_f·|Ω| through the stored loop traversal). An
            // approximating face takes it as its fit's flux, the geometry
            // the face carries.
            let quad_out = if takes_quadrature(
                surface,
                loops
                    .iter()
                    .flat_map(|(edges, _)| edges)
                    .map(|e| &e.carrier),
            ) {
                quad(body, surface, &loops, band, tol, window, about, resolution).map_err(wrap)?
            } else {
                None
            };
            match quad_out {
                Some(LaneRun {
                    bounds,
                    round,
                    rounds,
                    end,
                }) => {
                    state = FaceState::Quad {
                        taken: round,
                        ran: rounds.saturating_sub(1),
                        stop: FaceStop::Lane(match end {
                            LaneEnd::Accepted(stop) => stop,
                            LaneEnd::Window => LaneStop::Refinable,
                            LaneEnd::Floor => LaneStop::Floor,
                            LaneEnd::Wall => LaneStop::Wall,
                        }),
                    };
                    let (at, fp, ap) = quadrature_contribution(bounds.flux, bounds.area);
                    flux_pad += fp;
                    area_pad += ap;
                    enclosure = Some((bounds.flux, bounds.area));
                    at
                }
                // Either an iso boundary (the closed forms — the
                // face's S10 sense entering at `curved_face`'s one
                // sanctioned site, the rimless sphere band) or a
                // walk holding no [`QuadLane`] (a `_structural` door,
                // which is how a dual measures) — whose honest outcome
                // on a trimmed face is the closed form's typed refusal.
                None => {
                    let loops: Vec<Vec<LoopEdge<T>>> =
                        loops.into_iter().map(|(edges, _)| edges).collect();
                    closed_form_about(surface, &loops, face.sense, band, about).map_err(wrap)?
                }
            }
        }
    };
    Ok(FaceRun {
        face: face_key,
        contribution: FaceFlux {
            flux: contribution.flux,
            area: contribution.area,
            flux_pad,
            area_pad,
            enclosure,
        },
        state,
        about: ReadAbout::Shell(about),
    })
}

impl<T: Real> FaceRun<T> {
    /// This run with its region's carry ([`References`]) folded into its
    /// flux: into the enclosure, and the midpoint and pad read off it
    /// again, where the face has one; else onto the flux, its midpoint
    /// and half-width.
    fn carried(mut self, carry: Interval) -> Self {
        let c = &mut self.contribution;
        match c.enclosure {
            Some((flux, area)) => {
                let flux = flux + carry;
                let (at, fp, ap) = quadrature_contribution::<T>(flux, area);
                *c = FaceFlux {
                    flux: at.flux,
                    area: at.area,
                    flux_pad: fp,
                    area_pad: ap,
                    enclosure: Some((flux, area)),
                };
            }
            None => {
                let (mid, pad) = quad_lane::mid_pad(carry);
                c.flux = c.flux + T::from_computed(mid);
                c.flux_pad += pad;
            }
        }
        self
    }
}

/// A face's contribution as its two quadrature enclosures' midpoints,
/// and their half-widths: each midpoint is an `f64` the quadrature
/// computed, so it enters as a computed value.
fn quadrature_contribution<T: Real>(
    flux: Interval,
    area: Interval,
) -> (FaceContribution<T>, f64, f64) {
    let (fc, fp) = quad_lane::mid_pad(flux);
    let (ac, ap) = quad_lane::mid_pad(area);
    let at = FaceContribution {
        flux: T::from_computed(fc),
        area: T::from_computed(ac),
    };
    (at, fp, ap)
}

/// Flatten one loop's half-edge cycle into [`LoopEdge`]s (traversal
/// order; vertex tags are loop-local first-seen indices; each edge's
/// carrier identity is the root of its split lineage, minted from this
/// body's own keys and comparable only within this flattening),
/// alongside the half-edge keys walked (the PR 11 quadrature lane
/// reads stored pcurve caches through them).
///
/// Public because `mesh` is its second consumer: the curved lane hands
/// a face's outer loop to `geom_brep::props`' iso-rectangle door before
/// walking it, and this is the same half-edge cycle that walk reads —
/// one flattening, not two. Its error type is its own
/// ([`LoopEdgesError`]) so that a consumer across the crate boundary
/// matches exactly the two states a flatten can reach, and a third
/// arm added here is a compile error there rather than an
/// `unreachable!`.
#[allow(clippy::type_complexity)]
pub fn loop_edges<T: Decide>(
    body: &Body<T>,
    lk: LoopKey,
) -> Result<(Vec<LoopEdge<T>>, Vec<crate::entity::HalfEdgeKey>), LoopEdgesError> {
    let corrupt = |what| LoopEdgesError::Corrupt { what };
    let Some(loop_) = body.loops.get(lk) else {
        return Err(corrupt("loop key does not resolve"));
    };
    let LoopBoundary::Cycle { first } = loop_.boundary else {
        return Err(corrupt("empty loop (construction scaffolding at rest)"));
    };
    let Some(cycle) = body.loop_cycle(first) else {
        return Err(corrupt("broken half-edge cycle"));
    };
    Ok((run_edges(body, &cycle)?, cycle))
}

/// **A loop's chart image, flattened**: [`loop_edges`] over the loop's
/// cycle with its spurs taken out ([`crate::surface_boundary::chart_image`]),
/// its seams kept. The view every integrator and chart reader takes; empty
/// where the loop is all spurs.
///
/// # Errors
///
/// As [`loop_edges`].
#[allow(clippy::type_complexity)]
pub fn chart_image_edges<T: Decide>(
    body: &Body<T>,
    lk: LoopKey,
) -> Result<(Vec<LoopEdge<T>>, Vec<crate::entity::HalfEdgeKey>), LoopEdgesError> {
    let (_, cycle) = loop_edges(body, lk)?;
    image_of_cycle(body, &cycle)
}

/// [`chart_image_edges`] of a loop's cycle the caller has walked.
///
/// # Errors
///
/// As [`loop_edges`].
pub(crate) fn image_of_cycle<T: Decide>(
    body: &Body<T>,
    cycle: &[crate::entity::HalfEdgeKey],
) -> Result<FlatLoop<T>, LoopEdgesError> {
    let image =
        crate::surface_boundary::chart_image(body, cycle).ok_or(LoopEdgesError::Corrupt {
            what: "half-edge mate does not resolve",
        })?;
    Ok((run_edges(body, &image)?, image))
}

/// [`loop_edges`]' flattening over any closed run of half-edges, in
/// the order given: a loop's cycle, or a chain of half-edges from
/// several faces that closes on itself.
pub(crate) fn run_edges<T: Decide>(
    body: &Body<T>,
    cycle: &[crate::entity::HalfEdgeKey],
) -> Result<Vec<LoopEdge<T>>, LoopEdgesError> {
    let corrupt = |what| LoopEdgesError::Corrupt { what };
    let mut tags: Vec<VertexKey> = Vec::new();
    let mut tag_of = |v: VertexKey| -> u32 {
        if let Some(i) = tags.iter().position(|&t| t == v) {
            i as u32
        } else {
            tags.push(v);
            (tags.len() - 1) as u32
        }
    };
    let mut edges = Vec::with_capacity(cycle.len());
    for &he_key in cycle {
        let Some(he) = body.half_edges.get(he_key) else {
            return Err(corrupt("half-edge key does not resolve"));
        };
        let Some(edge) = body.edges.get(he.edge) else {
            return Err(corrupt("edge key does not resolve"));
        };
        let Some(entry) = body.curves.get(edge.curve) else {
            return Err(corrupt("curve key does not resolve"));
        };
        let Some(curve) = entry.certified() else {
            return Err(LoopEdgesError::NullScaffoldEdge { edge: he.edge });
        };
        let Some(end) = body.half_edge_end(he_key) else {
            return Err(corrupt("half-edge mate does not resolve"));
        };
        let (t0, t1) = curve.params();
        edges.push(LoopEdge {
            carrier: curve.carrier().clone(),
            // A lineage that cycles is a corrupt record; the flattening
            // then stamps NO identity, so no two such edges are ever
            // folded into one — the fold declines rather than trusting
            // a record it cannot read, and a split meridian on such a
            // body refuses at the far rim as it did before any fold.
            carrier_id: body
                .split_root(he.edge, |_| false)
                .ok()
                .map(|root| CarrierId::minted(root.data().as_ffi())),
            t0,
            t1,
            forward: he_key == edge.he_plus,
            start: tag_of(he.start),
            end: tag_of(end),
        });
    }
    Ok(edges)
}

/// The derived outer/void designation of one shell. The shell list
/// stores no such designation ([`crate::entity::Solid`]'s documented
/// invariant): the role IS the side at infinity of the shell's faces
/// ([`shell_role`]) — a boundary that bounds material from outside
/// leaves infinity outside its material, and a cavity wall leaves it
/// inside. Its signed volume has the same sign: positive for the one,
/// negative for the other.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ShellRole {
    /// Infinity outside the material: the shell is the outer boundary
    /// of one connected component of material.
    Outer,
    /// Infinity inside the material: the shell bounds an internal
    /// cavity.
    Void,
}

/// Which end of a volume bracket a sign was read at.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) enum BracketEnd {
    /// The bracket's lower end.
    Low,
    /// The bracket's upper end.
    High,
}

impl ShellRole {
    /// **The role as its word**: its variant's name in snake case, the
    /// one spelling every surface that names a role reads (Python's
    /// `MassMeasurement.verdict` for a sign).
    #[must_use]
    pub const fn word(self) -> &'static str {
        match self {
            Self::Outer => "outer",
            Self::Void => "void",
        }
    }
}

impl ShellRole {
    /// **The enclosure-to-role reading**: the one place a decided end
    /// of a volume bracket becomes a role.
    ///
    /// Every bracket contains the true volume, so a LOW end definitely
    /// positive makes the boundary `Outer` and a HIGH end definitely
    /// negative makes it `Void`, and no finer bracket can say otherwise.
    /// Any other sign decides nothing at that end. An exact volume is a
    /// bracket whose two ends coincide, so its one sign is read at both.
    ///
    /// Which end is read first, under which predicate name, and what a
    /// bracket that neither end decides costs (a refinement, an
    /// "undecided", a typed refusal) belong to the caller.
    pub(crate) fn decided_at(end: BracketEnd, sign: Sign) -> Option<Self> {
        match (end, sign) {
            (BracketEnd::Low, Sign::Positive) => Some(Self::Outer),
            (BracketEnd::High, Sign::Negative) => Some(Self::Void),
            _ => None,
        }
    }
}

/// One shell's decided role, and the enclosure its read reached.
#[derive(Clone, Copy, Debug)]
pub struct ShellClassification<T: Real> {
    /// The classified shell.
    pub shell: ShellKey,
    /// The solid owning it ([`crate::entity::Shell::solid`]).
    pub solid: SolidKey,
    /// The shell's SIGNED volume and its area, as the role's read left
    /// their enclosures: the divergence theorem over exactly this shell's
    /// faces (on a body of closed-form faces the shells' volumes sum to
    /// the body's), each quadrature face at its sign walk's round where
    /// the role was read off the volume, and at round 0 otherwise. Sound
    /// and possibly wide: a caller that wants a shell's number measures
    /// it.
    pub enclosure: MassProperties<T>,
    /// The decided role. A solid's only shell is `Outer` by arity, not
    /// by a reading of its orientation: on an unvalidated body it is
    /// `Outer` beside a negative volume where that shell is wound
    /// inside out, which the at-rest gate (check 7) refuses and this
    /// door does not check.
    pub role: ShellRole,
}

/// Typed refusal of [`classify_shells`] (closed enum, D4 ¶3). Never a
/// silent skip: a shell this door cannot classify refuses with the
/// shell named, exactly as tier 3's check 7 refuses a body whose flux
/// the props inventory cannot compute.
#[derive(Clone, Debug, PartialEq)]
pub enum ShellClassifyError {
    /// The run's tolerance cannot form a band.
    Band {
        /// The band construction failure.
        error: BandError,
    },
    /// A face of this shell refused in the flux inventory — the
    /// [`MassPropsError`] posture inherited unaltered (rational walls,
    /// out-of-inventory boundaries, corrupt structure).
    Props {
        /// The shell whose face refused.
        shell: ShellKey,
        /// The per-face/per-body failure.
        source: MassPropsError,
    },
    /// The sign read escalated on the shell's certified volume: its
    /// enclosure, taken about the body in closed form, holds zero (the
    /// sign is read off it alone, D10), or could not be read. F6: an
    /// undecided orientation is never guessed to a side.
    Escalated {
        /// The unclassifiable shell.
        shell: ShellKey,
        /// The named escalation from the funnel.
        source: Indeterminate,
    },
    /// An end of the shell's certified volume enclosure read as zero, and
    /// no end read to a side.
    ZeroVolume {
        /// The unclassifiable shell.
        shell: ShellKey,
        /// The zero end's verdict, with its reporting margin.
        verdict: Classified,
    },
    /// The shell's certified volume bracket straddles zero: its low end
    /// reads on the void side and its high end on the outer side.
    Straddles {
        /// The unclassifiable shell.
        shell: ShellKey,
        /// The low end's reading.
        low: geom_core::MarginDiag,
        /// The high end's reading.
        high: geom_core::MarginDiag,
    },
    /// The sign of the shell's volume is unresolved by a limit of how it
    /// was measured, not by the body ([`SignLimit`]): a capability limit.
    SignUnresolved {
        /// The unclassifiable shell.
        shell: ShellKey,
        /// What left the sign unresolved.
        limit: SignLimit,
    },
    /// A ray aimed at one of the shell's faces from beyond it met two of
    /// its faces in band of each other along it, and no other aimed ray
    /// read the shell's side: the pair is in band at this tolerance.
    RayInBand {
        /// The unclassifiable shell.
        shell: ShellKey,
        /// The ray's pair: the face it was aimed at, the two faces it
        /// met, and their gap along it.
        pair: Box<crate::boolean::AimedPair>,
    },
    /// No aimed ray read the shell's side, one was set aside at a corner
    /// or a wedge (`SetAsideWhy::Corner`, `SetAsideWhy::Wedge`), and the
    /// volume then refused too: `volume` is that refusal, and `corner`
    /// the first such ray. Neither is a pair, so this refusal is the
    /// volume's, naming where the rays went.
    Corner {
        /// The unclassifiable shell.
        shell: ShellKey,
        /// The first ray set aside at a corner or a wedge.
        corner: Box<crate::side_reader::SetAside>,
        /// The volume's refusal.
        volume: Box<ShellClassifyError>,
    },
    /// An aimed ray's walk refused on a face of the shell that does not
    /// walk: a broken body.
    Ray {
        /// The unclassifiable shell.
        shell: ShellKey,
        /// The walk's refusal.
        source: crate::boolean::PointInSolidError,
    },
}

/// The side read off a ray aimed at one of a shell's or a solid's faces
/// (`side_at_infinity_order`): its margin is the gap along the ray
/// between the two faces it met first, a magnitude, and it passes on a
/// definite gap. One decision and one lever, at rest and at a boolean's
/// door alike (`BooleanDecision::ShellRole`).
pub const AIMED_PAIR: SizedDecision = SizedDecision {
    lever: "move the geometry so the two faces the aimed ray met are clearly apart",
    size: "gap",
    passes: SizedPass::Positive,
    side: RefusedSide::Lever,
    no_size: None,
};

/// **A volume's enclosure, in words**: the volume itself, in cubic
/// metres — its enclosure's two ends, the one value read, or that it
/// could not be read — through the reading's own `Display`. The volume's
/// sign is read off it alone ([`read_sign`]), so the volume is what the
/// refusal has to say, and no band is quoted against it.
#[derive(Clone, Copy, Debug)]
pub(crate) struct EnclosureWords(pub(crate) geom_core::MarginDiag);

impl fmt::Display for EnclosureWords {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0.kind() {
            geom_core::MarginKind::Enclosure => write!(f, "its volume lies in {:e} m³", self.0),
            geom_core::MarginKind::Value => write!(f, "its volume reads {:e} m³", self.0),
            geom_core::MarginKind::Invalid => {
                f.write_str("its volume could not be read (NaN or a refused enclosure)")
            }
        }
    }
}

/// One end of a volume bracket, in words: `reads {v} m³`, or that it
/// could not be read.
#[derive(Clone, Copy, Debug)]
struct VolumeEnd(geom_core::MarginDiag);

impl fmt::Display for VolumeEnd {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0.kind() {
            geom_core::MarginKind::Invalid => f.write_str("could not be read"),
            geom_core::MarginKind::Enclosure => write!(f, "lies in {:e} m³", self.0),
            geom_core::MarginKind::Value => write!(f, "reads {:e} m³", self.0),
        }
    }
}

/// What a [`ShellClassifyError`] found, without the shell's key or a
/// stage label: the one text its `Display` and a document-level finding
/// both render.
#[derive(Clone, Copy, Debug)]
pub struct ShellClassifyPayload<'a>(&'a ShellClassifyError);

impl fmt::Display for ShellClassifyPayload<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self.0 {
            ShellClassifyError::Band { error } => write!(f, "{error}"),
            ShellClassifyError::Props { source, .. } => write!(f, "{source}"),
            ShellClassifyError::Escalated { source, .. } => write!(
                f,
                "the sign of a shell's volume cannot be read: {}",
                EnclosureWords(source.margin)
            ),
            ShellClassifyError::ZeroVolume { verdict, .. } => write!(
                f,
                "a shell's certified volume reads zero: {}",
                EnclosureWords(verdict.margin)
            ),
            ShellClassifyError::Straddles { low, high, .. } => write!(
                f,
                "a shell's certified volume bracket holds zero: its low end {} and its high \
                 end {}",
                VolumeEnd(*low),
                VolumeEnd(*high)
            ),
            ShellClassifyError::SignUnresolved { limit, .. } => write!(
                f,
                "the sign of a shell's volume cannot be read: {}",
                limit.words()
            ),
            ShellClassifyError::RayInBand { pair, .. } => write!(
                f,
                "which side of a shell its material lies on cannot be read: {}",
                crate::side_reader::PairPayload(pair)
            ),
            ShellClassifyError::Corner { corner, volume, .. } => write!(
                f,
                "{}; and no aimed ray read its side: {}",
                volume.payload(),
                crate::side_reader::UnreadRays {
                    aimed: 1,
                    set_aside: core::slice::from_ref(corner),
                }
            ),
            ShellClassifyError::Ray { source, .. } => write!(f, "{source}"),
        }
    }
}

impl ShellClassifyError {
    /// The refusal's data, with no key, label or ending.
    #[must_use]
    pub fn payload(&self) -> ShellClassifyPayload<'_> {
        ShellClassifyPayload(self)
    }

    /// The refused arm of the shell-role reading this refusal is; `None`
    /// where the refusal is not that reading's. A flux refusal ends in its
    /// own payload's recourse, and a band failure is the run's
    /// configuration.
    ///
    /// A pair's gap is band-decided, its arm the gap. The volume's sign is
    /// read off its certified enclosure alone (D10), so an enclosure
    /// holding zero, an end at zero and a straddle are each the kernel's,
    /// which no tolerance reads otherwise ([`Self::recourse`]).
    #[must_use]
    pub fn arm(&self) -> Option<RefusedArm<'_>> {
        match self {
            Self::Escalated { source, .. } => Some(RefusedArm::Undecided(source)),
            Self::RayInBand { pair, .. } => Some(RefusedArm::Undecided(&pair.gap)),
            Self::Corner { volume, .. } => volume.arm(),
            Self::ZeroVolume { verdict, .. } => Some(RefusedArm::Zero(*verdict)),
            Self::Straddles { .. } => Some(RefusedArm::SignCertain(None)),
            Self::Props { .. }
            | Self::Band { .. }
            | Self::Ray { .. }
            | Self::SignUnresolved { .. } => None,
        }
    }

    /// The shell-role reading's one ending for this refusal (D4 ¶1 (i))
    /// read `at`: the ray's ([`AIMED_PAIR`]) for a pair in band; for the
    /// volume, whose sign is certified by its enclosure alone, the
    /// kernel's defect ending, a corner's included (a corner is no pair);
    /// `None` where the refusal is not that reading's.
    #[must_use]
    pub fn recourse(&self, at: impl Into<geom_brep::recourse::ReadAt>) -> Option<String> {
        if let Self::SignUnresolved { limit, .. } = self {
            return Some(limit.ending().to_owned());
        }
        let at = at.into();
        let arm = self.arm()?;
        Some(match self {
            Self::RayInBand { .. } => AIMED_PAIR.recourse(arm, at),
            Self::Corner { volume, .. } => volume.recourse(at)?,
            Self::Escalated { .. }
            | Self::ZeroVolume { .. }
            | Self::Straddles { .. }
            | Self::Props { .. }
            | Self::Band { .. }
            | Self::Ray { .. }
            | Self::SignUnresolved { .. } => geom_brep::recourse::Unsized::Defect.recourse(arm, at),
        })
    }

    /// [`ShellClassifyError::recourse`] read at a build, where the shell
    /// door's own `Display` reads it. A reader over stored geometry reads
    /// it at rest itself, where a poisoned margin's note names the file.
    #[must_use]
    pub fn ending(&self) -> Option<String> {
        self.recourse(Reading::Build)
    }
}

impl fmt::Display for ShellClassifyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Band { .. } => write!(f, "the shells cannot be classified: {}", self.payload()),
            Self::Escalated { .. }
            | Self::Props { .. }
            | Self::ZeroVolume { .. }
            | Self::Straddles { .. }
            | Self::SignUnresolved { .. }
            | Self::RayInBand { .. }
            | Self::Corner { .. }
            | Self::Ray { .. } => write!(f, "{}", self.payload()),
        }?;
        match self.ending() {
            Some(ending) => write!(f, ". {ending}"),
            None => Ok(()),
        }
    }
}

impl std::error::Error for ShellClassifyError {}

/// Per-shell signed volume and outer/void role for every shell of
/// `body`, in shell-arena slot order (deterministic per D9).
///
/// Each shell's volume and area sum the same per-face contributions the
/// whole-body [`mass_properties`] sums, restricted to that shell's face
/// list — never a second flux implementation (engineering convention 2) —
/// at the rounds its role's read reached ([`ShellClassification`]): no
/// face is refined for a number.
/// The role is the one reader's ([`shell_role`]): a solid's only shell is
/// `Outer` by ARITY — its role, not its orientation, which this door does
/// not read: a lone shell wound inside out on a body that has not passed
/// the at-rest gate classifies `Outer` beside a negative volume, and it
/// is check 7 that refuses it. Any other shell's role is the side at
/// infinity of its faces, read off rays aimed at them and off its
/// volume's certified sign only where no face offers an aim that reads.
/// A role no reading decides refuses typed, never a guess.
///
/// # Errors
///
/// [`ShellClassifyError`] — the first shell that cannot be classified
/// refuses the call: an inherited flux refusal, an aimed ray's pair in
/// band, or, where no aim read, an escalated sign, a volume zero at
/// this tolerance, or a bracket straddling zero. (A component count over
/// a partial classification would be a guess; the caller gets the
/// refusal instead.)
///
/// It classifies a finished body (`&AtRestBody`, Ev's decision on PR
/// 4822), as [`mass_properties`] measures one;
/// `topo::test_support::classify_unvalidated` is the raw door.
pub fn classify_shells<T: Decide + geom_core::CertifiedBounds>(
    body: &crate::AtRestBody<T>,
    tol: Tol,
) -> Result<Vec<ShellClassification<T>>, ShellClassifyError> {
    classify_body(body, tol)
}

/// [`classify_shells`] holding NO quadrature lane — the same
/// classification at any deciding scalar with a bracket, each shell's
/// flux summed through the closed form alone
/// ([`mass_properties_structural`]'s walk, per shell). A shell with a
/// face that needs the certified quadrature refuses typed
/// ([`ShellClassifyError::Props`]). Where a shell's role falls back to
/// its volume, a closed-form shell's is read off the walk's own sums, so
/// at a rounding scalar a shell whose volume is below that rounding can
/// read a role the certified door refuses
/// (`work/tally/lane-free-volume-sign-reads-decide-on-a-rounded-sum`).
///
/// # Errors
///
/// [`ShellClassifyError`] — [`classify_shells`]'s, plus the closed
/// form's typed refusal on every face the quadrature lane would have
/// enclosed.
pub fn classify_shells_structural<T: Decide>(
    body: &crate::AtRestBody<T>,
    tol: Tol,
) -> Result<Vec<ShellClassification<T>>, ShellClassifyError> {
    classify_body_structural(body, tol)
}

/// [`classify_shells`] restricted to `shells` — the same classification,
/// made of exactly the shells named and no others, in shell-arena slot
/// order.
///
/// **A shell's role is a property of that shell alone** (and of how
/// many shells its solid holds): the side at infinity its own faces
/// leave. So a caller asking about one solid's shells is asking a
/// question that does not involve any other solid's, and it should not
/// pay another solid's refusal for it — a classification that escalates
/// on a neighbour would otherwise refuse a body this caller can answer
/// for.
/// [`classify_shells`] is this door over every shell of the body.
///
/// A shell the body does not hold is skipped rather than refused: the
/// list is a restriction, and a caller naming a stale key gets fewer
/// rows, which its own arity check reads.
///
/// # Errors
///
/// [`ShellClassifyError`] — [`classify_shells`]'s, raised by the first
/// NAMED shell that cannot be classified.
pub fn classify_shells_of<T: Decide + geom_core::CertifiedBounds>(
    body: &crate::AtRestBody<T>,
    shells: &[ShellKey],
    tol: Tol,
) -> Result<Vec<ShellClassification<T>>, ShellClassifyError> {
    classify_body_of(body, shells, tol)
}

/// The per-shell classification through `quad` — the shared body of
/// [`classify_shells_of`] and [`classify_shells_structural`]: each named
/// shell's role read by the one reader of it ([`shell_role`]), and the
/// enclosure that read reached — its volume walk's, where the role was
/// read off the volume, and otherwise every face's round 0. No face is
/// refined for a number.
pub(crate) fn classify_shells_through<T: Decide>(
    body: &Body<T>,
    shells: &[ShellKey],
    tol: Tol,
    quad: Option<QuadLane<T>>,
) -> Result<Vec<ShellClassification<T>>, ShellClassifyError> {
    let band = Band::linear(tol).map_err(|error| ShellClassifyError::Band { error })?;
    let mut out = Vec::new();
    for (shell_key, shell) in body.shells.iter() {
        if !shells.contains(&shell_key) {
            continue;
        }
        let props = |source| ShellClassifyError::Props {
            shell: shell_key,
            source,
        };
        // Serial, for [`decide_faces_serially`]'s reason.
        let mut certificate =
            SignCertificate::at_round_zero(body, &shell.faces, band, tol, quad, FaceWalk::Serial)
                .map_err(props)?;
        let role = shell_role_reading(body, shell_key, band, || {
            shell_volume_role_of(shell_key, &mut certificate, quad)
        })?;
        out.push(ShellClassification {
            shell: shell_key,
            solid: shell.solid,
            enclosure: certificate.props(),
            role,
        });
    }
    Ok(out)
}

/// **A shell's role** — whether `shell` bounds material (`Outer`) or a
/// cavity (`Void`): the side at infinity its faces leave (D10, Booleans).
/// The one reader of it: tier 3's check 10, the result sort
/// ([`crate::stands::ShellRead`]), the census's void screen, `shell`'s
/// cavity read and the shell classification read it, and check 7 reads a
/// solid's orientation off the same reader.
///
/// A solid's only shell is its outer shell by arity, and is read no
/// further. Any other shell is read off rays aimed at its faces from
/// beyond it ([`crate::side_reader::side_at_infinity`]), and
/// off the sign of its volume, walked through `quad`
/// ([`shell_volume_role`]), only where no face of it offers an aim that
/// reads.
///
/// A shell the body does not hold reads no face, and refuses as a
/// volume of zero.
///
/// # Errors
///
/// The shell's typed refusal: an aimed ray read in band
/// ([`ShellClassifyError::RayInBand`], naming its pair) or walked into a
/// broken face ([`ShellClassifyError::Ray`]); or, where no aim read,
/// the volume walk's [`ShellClassifyError::Props`] or the role its last
/// round left undecided (escalated, zero or straddling).
pub(crate) fn shell_role<T: Decide>(
    body: &Body<T>,
    shell: ShellKey,
    band: Band,
    tol: Tol,
    quad: Option<QuadLane<T>>,
) -> Result<ShellRole, ShellClassifyError> {
    shell_role_reading(body, shell, band, || {
        shell_volume_role(body, shell, band, tol, quad)
    })
}

/// [`shell_role`], its volume read by `volume` where no aim reads.
fn shell_role_reading<T: Decide>(
    body: &Body<T>,
    shell: ShellKey,
    band: Band,
    volume: impl FnOnce() -> Result<ShellRole, ShellClassifyError>,
) -> Result<ShellRole, ShellClassifyError> {
    use crate::boolean::SolidContainment;
    use crate::boolean::solid_contain::SolidFaces;
    use crate::side_reader::{SideUnread, side_at_infinity};
    let record = body.get_shell(shell);
    if record
        .and_then(|s| body.get_solid(s.solid))
        .is_some_and(|solid| solid.shells.len() == 1)
    {
        return Ok(ShellRole::Outer);
    }
    let volume = || match volume()? {
        ShellRole::Outer => Ok(SolidContainment::Out),
        ShellRole::Void => Ok(SolidContainment::In),
    };
    match side_at_infinity(body, &SolidFaces::of_shell(body, shell), band, volume) {
        Ok(Ok(SolidContainment::In)) => Ok(ShellRole::Void),
        Ok(Ok(_)) => Ok(ShellRole::Outer),
        Ok(Err(SideUnread::InBand(pair))) => Err(ShellClassifyError::RayInBand {
            shell,
            pair: Box::new(pair),
        }),
        Ok(Err(SideUnread::Fallback {
            volume, set_aside, ..
        })) => Err(
            match set_aside.into_iter().find(|s| {
                matches!(
                    s.why,
                    crate::side_reader::SetAsideWhy::Corner { .. }
                        | crate::side_reader::SetAsideWhy::Wedge { .. }
                )
            }) {
                Some(corner) => ShellClassifyError::Corner {
                    shell,
                    corner: Box::new(corner),
                    volume: Box::new(volume),
                },
                None => volume,
            },
        ),
        Err(source) => Err(ShellClassifyError::Ray { shell, source }),
    }
}

/// **A shell's role off the sign of its volume** — [`shell_role`]'s
/// fallback, where no face of the shell offers an aim that reads: the
/// role walk ([`role_walk`]) over its own faces through `quad`, logged
/// under check 7's names.
///
/// # Errors
///
/// The walk's or the re-derivation's [`ShellClassifyError::Props`], or
/// the role the last round left undecided (escalated, zero or
/// straddling).
pub(crate) fn shell_volume_role<T: Decide>(
    body: &Body<T>,
    shell: ShellKey,
    band: Band,
    tol: Tol,
    quad: Option<QuadLane<T>>,
) -> Result<ShellRole, ShellClassifyError> {
    let faces = body.get_shell(shell).map_or(&[][..], |s| &s.faces[..]);
    let mut certificate =
        SignCertificate::at_round_zero(body, faces, band, tol, quad, FaceWalk::Mapped)
            .map_err(|source| ShellClassifyError::Props { shell, source })?;
    shell_volume_role_of(shell, &mut certificate, quad)
}

/// [`shell_volume_role`] on the shell's faces as `certificate` holds
/// them, walked on from there.
fn shell_volume_role_of<T: Decide>(
    shell: ShellKey,
    certificate: &mut SignCertificate<'_, T>,
    quad: Option<QuadLane<T>>,
) -> Result<ShellRole, ShellClassifyError> {
    let props = |source| ShellClassifyError::Props { shell, source };
    let Some(role) = role_walk_set(&mut [certificate], quad, crate::validate::PLUS_V_SIGN).pop()
    else {
        unreachable!("a role walk over one member reads one role")
    };
    match role {
        Ok(role) => Ok(role),
        Err(RoleWalkRefusal::Refused(source)) => Err(props(source)),
        Err(RoleWalkRefusal::Poisoned(last)) => Err(last.refusal(shell)),
        Err(RoleWalkRefusal::Unread {
            limit: Some(limit), ..
        }) => Err(ShellClassifyError::SignUnresolved { shell, limit }),
        Err(RoleWalkRefusal::Unread { last, .. }) => Err(last.refusal(shell)),
    }
}

/// Why a role read decided no role.
#[derive(Clone, Debug)]
pub(crate) enum RoleWalkRefusal {
    /// The interval re-derivation refused a face, or the walk ran out
    /// undecided on a face a later round refused outright
    /// ([`Walked::Exhausted`]).
    Refused(MassPropsError),
    /// An end's margin was poisoned at the last reading
    /// ([`Certified::Poisoned`]): no enclosure decides it.
    Poisoned(Box<RoleUnread>),
    /// The sign is undecided at the last reading.
    Unread {
        /// The last round's two ends.
        last: Box<RoleUnread>,
        /// What left that round's sign unresolved, where it is a limit of
        /// the measurement ([`Certified::Unresolved`]); `None` where the
        /// enclosure holds zero at the body's own rounding
        /// ([`Certified::Open`]).
        limit: Option<SignLimit>,
        /// Whether the walk stopped because the reading lies wholly in the
        /// sign's band (both ends read zero, [`RoleUnread::reads_zero`]),
        /// where no narrower enclosure decides it, rather than because
        /// no face could refine.
        in_band: bool,
    },
}

/// **The certified sign of a closed surface's volume, as a role: the
/// one home of that reading** — [`Settle::Sign`]'s walk over `faces`,
/// measured through `measure` and certified through `certify` round by
/// round ([`Round::certify`], under `names`), stopping at the first round
/// that certifies a role or refuses, or once no face can refine. `Outer`
/// is a positive volume, `Void` a negative one ([`certify_role`]).
///
/// Its callers read the same fact, and differ in what they hand it and
/// what they keep of its answer, never in how a reading becomes a role:
/// - a shell's role ([`shell_volume_role`]): the shell's faces, the
///   door's lane for both measuring and certifying, under check 7's
///   names; it keeps the role, or the last round's refusal;
/// - check 7 (`validate`'s `plus_v_by_sign`): every solid's faces, one
///   member each of one walk ([`role_walk_set`]), the same lane and
///   names, and also the certificate the walk leaves, the
///   open/unresolved split ([`RoleWalkRefusal::Unread`]'s `limit`, which
///   tells `VolumeSignUnresolved` from an undecided orientation) and the
///   walk's refusal of a face it needed (`VolumeUncomputable`);
/// - point containment's side at infinity (`boolean::solid_contain`'s
///   `at_infinity_side`): the selection's faces, measured in closed form
///   only and certified through the scalar's own lane, under its own
///   names (`bool_point_in_solid_infinity`), so its decisions log apart.
///
/// # Errors
///
/// [`MassPropsError`] where a face has no enclosure at round 0. A face
/// refused at a later round is the role's [`RoleWalkRefusal::Refused`].
#[allow(clippy::too_many_arguments)]
pub(crate) fn role_walk<'b, T: Decide>(
    body: &'b Body<T>,
    faces: &[FaceKey],
    band: Band,
    tol: Tol,
    measure: Option<QuadLane<T>>,
    certify: Option<QuadLane<T>>,
    names: SignNames,
) -> Result<(Result<ShellRole, RoleWalkRefusal>, SignCertificate<'b, T>), MassPropsError> {
    let mut certificate =
        SignCertificate::at_round_zero(body, faces, band, tol, measure, FaceWalk::Mapped)?;
    let Some(role) = role_walk_set(&mut [&mut certificate], certify, names).pop() else {
        unreachable!("a role walk over one member reads one role")
    };
    Ok((role, certificate))
}

/// **[`role_walk`] over a set**: each member's role read off its own
/// enclosure, every member one walk ([`walk_set`]) that refines a member
/// only while its sign straddles zero, so each member stops at the round
/// it would stop at walked alone.
///
/// The sign is read through its funnel band, [`Band::enclosure_sign`]
/// ([`read_sign`]): a member settles where its enclosure lies on one side
/// of zero, refines while it straddles, and stops at once where both
/// ends read zero, wholly in the band ([`RoleWalkRefusal::Unread`]'s
/// `in_band`). A poisoned margin stops it too, typed
/// ([`RoleWalkRefusal::Poisoned`]), as does the interval re-derivation's
/// refusal. A member the walk runs out on with a face a later round
/// refused outright reads that refusal ([`RoleWalkRefusal::Refused`]).
pub(crate) fn role_walk_set<'b, T: Decide>(
    members: &mut [&mut SignCertificate<'b, T>],
    certify: Option<QuadLane<T>>,
    names: SignNames,
) -> Vec<Result<ShellRole, RoleWalkRefusal>> {
    let n = members.len();
    let mut read: Vec<Option<Result<ShellRole, RoleWalkRefusal>>> = (0..n).map(|_| None).collect();
    let mut last: Vec<Option<(RoleUnread, Option<SignLimit>)>> = vec![None; n];
    let walked = walk_set(members, certify, |rounds| {
        for (m, round) in rounds.iter().enumerate() {
            if read[m].is_some() {
                continue;
            }
            match round.certify(names.walk, names.certified) {
                Certified::Role(role) => read[m] = Some(Ok(role)),
                Certified::Refused(source) => read[m] = Some(Err(RoleWalkRefusal::Refused(source))),
                Certified::Poisoned(unread) => {
                    read[m] = Some(Err(RoleWalkRefusal::Poisoned(Box::new(unread))));
                }
                Certified::Open(unread) => last[m] = Some((unread, None)),
                Certified::Unresolved(unread, limit) => last[m] = Some((unread, Some(limit))),
            }
            if let (None, Some((unread, limit))) = (&read[m], &last[m])
                && unread.reads_zero()
            {
                read[m] = Some(Err(RoleWalkRefusal::Unread {
                    last: Box::new(*unread),
                    limit: *limit,
                    in_band: true,
                }));
            }
        }
        if read.iter().all(Option::is_some) {
            Ask::Decided(())
        } else {
            Ask::Refine(read.iter().map(Option::is_none).collect())
        }
    });
    let held: Vec<Option<MassPropsError>> = match walked {
        Walked::Decided(()) => vec![None; n],
        Walked::Exhausted { refused } => {
            let mut held = vec![None; n];
            for (m, refusal) in refused {
                held[m] = Some(refusal);
            }
            held
        }
    };
    read.into_iter()
        .zip(last)
        .zip(held)
        .map(|((read, last), held)| {
            read.unwrap_or_else(|| match (held, last) {
                (Some(source), _) => Err(RoleWalkRefusal::Refused(source)),
                (None, Some((unread, limit))) => Err(RoleWalkRefusal::Unread {
                    last: Box::new(unread),
                    limit,
                    in_band: false,
                }),
                (None, None) => unreachable!("a sign walk reads every member it asks"),
            })
        })
        .collect()
}

/// The refusal of a shell whose bracket `[lo, hi]` classified to neither
/// side (`lo` not outer, `hi` not void), read from the decided signs:
/// an escalated end is the refusal (the high end's first), ends decided
/// to opposite sides straddle zero, and otherwise an end is zero (the
/// low end's first), carried with its reporting margin.
fn shell_role_refusal(
    shell: ShellKey,
    lo: Result<Decided, Indeterminate>,
    hi: Result<Decided, Indeterminate>,
) -> ShellClassifyError {
    match (lo, hi) {
        (_, Err(source)) | (Err(source), _) => ShellClassifyError::Escalated { shell, source },
        (Ok(lo), Ok(hi)) => match (lo.sign, hi.sign) {
            (Sign::Negative, Sign::Positive) => ShellClassifyError::Straddles {
                shell,
                low: lo.margin,
                high: hi.margin,
            },
            _ => {
                let zero = if lo.sign == Sign::Zero { lo } else { hi };
                ShellClassifyError::ZeroVolume {
                    shell,
                    verdict: Classified {
                        margin: zero.margin,
                        band: Band::enclosure_sign(),
                    },
                }
            }
        },
    }
}

// SHELL-TOLERANCE-CHAIN BEGIN — the sentinel
// `tests/shell_tolerance_chain.rs` reads. Between here and the END
// sentinel is the kernel's last stretch of the shell's offset chain:
// the quadrature door the shell verbs' one flux read passes the run's
// witness through. No signature in this region may take an `f64`
// epsilon; the quadrature lane's own ε reads elsewhere in this file are
// a different chain and are deliberately outside the region.

/// **The certified quadrature, as an injected door** — the one value
/// that says a scalar may certify a body at rest.
///
/// Certification is the certifying scalars' business and derivative
/// transport is the dual's, and that split lives in the TYPES. The
/// quadrature machinery (`quad_lane`) is bounded `Decide + Bounds +
/// CertifiedEnclosure` and instantiates only for scalars with
/// certification rights — `f64`, the telemetry probe, the interval
/// scalar, and `Sym` over any of those; a [`geom_core::Dual`] carries a
/// bracket (D1) and still may not certify (DL1), which is the missing
/// [`geom_core::CertifiedEnclosure`] impl and nothing else. That roster
/// is not left to this sentence:
/// `topo/tests/certified_enclosure_impl_census.rs` counts the impls in
/// the tree against `wiring_rows`' instantiations and reds on a
/// certifying scalar that has no row here. This type
/// carries that fact to the passes that run at both kinds of scalar:
/// its one constructor is [`QuadLane::certified`], at
/// `Decide + CertifiedBounds`, so holding a value IS the statement that
/// the scalar it is parameterised by can certify. A pass holding `None`
/// runs the closed form alone — it instantiates no quadrature code, and
/// a face that needed the quadrature refuses typed. Nothing dispatches
/// on the scalar at run time and there is no blanket impl: the certified
/// door keeps its name and its quadrature for every certifying caller,
/// and a `_structural` twin carries the `None` by name.
///
/// **The symbolic tier over a certifying scalar** (`geom_core::sym`)
/// holds the BASE scalar's lane, run at `Sym<T>` itself: the tier
/// changes exactly one thing — how a margin whose expression is
/// identically zero decides — and it changes it inside the scalar, so
/// the quadrature runs here unaltered. Wrapping a certifying base must
/// not silently demote a certifying lane to a refusing one, or the
/// driver's leaf replay would stop validating the bodies it certifies;
/// `Sym<T>: CertifiedBounds` whenever `T` is, so the constructor is
/// there for it.
///
/// A scalar that may not certify cannot hold one — the constructor's
/// `impl` block is bounded on the right, so the value cannot be
/// written, let alone handed to a walk:
///
/// ```compile_fail,E0599
/// use geom_core::Dual64;
/// use topo::QuadLane;
/// let _ = QuadLane::<Dual64>::certified();
/// ```
///
/// The code is `E0599` and not [`mass_properties`]'s `E0277`, because
/// the two are refused at different places: a free function's bound is
/// an unsatisfied trait obligation on the call (`E0277`), while
/// `certified` is an associated function that EXISTS on
/// `QuadLane<Dual64>` and whose `impl` block's bounds are not met, which
/// `rustc` reports as "the associated function exists … but its trait
/// bounds were not satisfied" (`E0599`, read off `rustc` on the
/// snippet). Stable rustdoc verifies only that the block fails to build
/// (`geom_core::spline::hull`'s rule), so the code beside the fence is a
/// statement and not a check.
#[derive(Clone, Copy)]
#[allow(clippy::type_complexity)]
pub struct QuadLane<T: Decide> {
    /// The certified flux/area enclosures of one curved-cut face, over
    /// the round window it is handed, its flux about the world origin
    /// (`None`) or about a centre — `quad_lane::cut_face_rounds`, and
    /// nothing else can be written here (`wiring_rows` pins the
    /// pointer).
    cut_face_rounds: fn(
        &Body<T>,
        &Surface<T>,
        &[FlatLoop<T>],
        Band,
        Tol,
        RoundWindow,
        Option<Point3<Interval>>,
        Option<f64>,
    ) -> Result<LaneRun, PropsError>,
    /// One closed-form face's flux about a centre and its area,
    /// re-derived in interval arithmetic over its stored geometry —
    /// `quad_lane::closed_form` (`wiring_rows` pins the pointer). The
    /// closed forms run at the walk's scalar, where an `f64` sum rounds
    /// with no pad to say so; this is the enclosure a decision about a
    /// rounding-scale sign reads instead ([`rederive`]).
    closed_form: ClosedFormAt<T>,
    /// A value of the walk's scalar as the interval holding it —
    /// `Interval::from_certified` (`wiring_rows` pins the pointer).
    lift: fn(T) -> Interval,
}

impl<T: Decide + geom_core::CertifiedBounds> QuadLane<T> {
    /// The certified quadrature — the whole inventory of this door, and
    /// the only constructor there is. Its body is
    /// `quad_lane::cut_face_rounds`, entered by the number walk over
    /// the whole schedule and by [`walk_set`] one round window at a
    /// time, so [`mass_properties`] at a certifying scalar and the
    /// certified validator are one quadrature entered at two levels
    /// rather than two quadratures.
    #[must_use]
    pub const fn certified() -> Self {
        Self {
            cut_face_rounds: quad_lane::cut_face_rounds::<T>,
            closed_form: quad_lane::closed_form::<T>,
            lift: Interval::from_certified::<T>,
        }
    }
}

impl<T: Decide> QuadLane<T> {
    /// A value of the walk's scalar as the interval holding it.
    pub(crate) fn lift(self, x: T) -> Interval {
        (self.lift)(x)
    }
}

/// **The shell's op door**, as a value the passes take rather than a
/// trait a scalar implements.
///
/// [`crate::shell_open`] validates what it built — its last act is the
/// certified at-rest validator, whose `+V` invariant is a certified
/// claim — so the call is formed only at a scalar with certification
/// rights. Holding one of these IS that statement about the scalar it
/// is parameterised by, and the one constructor
/// ([`ShellDoor::certified`]) is the only way to make one.
///
/// The shape is [`QuadLane`]'s, for [`QuadLane`]'s reason: the door is
/// injected, so the layers above it — the verb seat and the document
/// lowering — stay generic over every evaluation scalar and refuse
/// typed where the seam answers [`None`]. The seam that answers it is
/// [`AtRestPolicy::shell_door`], the per-scalar policy home.
///
/// A scalar that may not certify cannot hold one — the constructor's
/// `impl` block is bounded on the right, so the value cannot be
/// written, let alone handed to a verb:
///
/// ```compile_fail,E0599
/// use geom_core::Dual64;
/// use topo::ShellDoor;
/// let _ = ShellDoor::<Dual64>::certified();
/// ```
///
/// Why the code is `E0599`, and what the fence does not check:
/// [`QuadLane`]'s paragraph.
#[derive(Clone, Copy)]
#[allow(clippy::type_complexity)]
pub struct ShellDoor<T: Decide> {
    /// [`ShellDoor::open`]'s body — `crate::shell_open`, and nothing
    /// else can be written here (`wiring_rows` pins the pointer).
    open: fn(&crate::AtRestBody<T>, T, &[FaceKey], Tol) -> Result<Shelled<T>, ShellError<T>>,
}

impl<T: Decide + geom_core::CertifiedBounds + AtRestPolicy> ShellDoor<T> {
    /// The certified hollowing door — the whole inventory of this
    /// door, and the only constructor there is. Its body is
    /// [`crate::shell_open`], so the sealed hollow ([`crate::shell()`],
    /// an empty designation) is reached through the same door.
    #[must_use]
    pub const fn certified() -> Self {
        Self {
            open: crate::shell::shell_open::<T>,
        }
    }
}

impl<T: Decide> ShellDoor<T> {
    /// The door's one operation: hollow `body` to `thickness`, opening
    /// the designated faces into rims.
    ///
    /// Every check, every refusal and every minted entity is
    /// [`crate::shell_open`]'s; this hands the arguments on and adds no
    /// decision of its own.
    ///
    /// # Errors
    ///
    /// [`ShellError`] — the door's own, verbatim.
    pub fn open(
        self,
        body: &crate::AtRestBody<T>,
        thickness: T,
        open_faces: &[FaceKey],
        tol: Tol,
    ) -> Result<Shelled<T>, ShellError<T>> {
        (self.open)(body, thickness, open_faces, tol)
    }
}

// SHELL-TOLERANCE-CHAIN END.

/// **The doors' WIRING** — the rows that say which free function
/// [`QuadLane::certified`] and [`ShellDoor::certified`] hold, rather
/// than what they answered.
///
/// Why a wiring row compares pointers rather than outputs:
/// `certified_enclosure_impl_census`'s module doc.
///
/// Each door has one helper, instantiated once per certifying scalar.
/// `certified_enclosure_impl_census` counts those instantiations
/// against the `CertifiedEnclosure` impls in the tree, both directions,
/// and counts the tree's door values against its roster of helpers.
#[cfg(test)]
mod wiring_rows {
    use super::{AtRestPolicy, QuadLane, ShellDoor, quad_lane};

    /// `Ok(())` when the quadrature door holds
    /// `quad_lane::cut_face_rounds` and `quad_lane::closed_form`;
    /// otherwise the name of the field that moved.
    fn holds_the_certified_quadrature<T: super::Decide + geom_core::CertifiedBounds>()
    -> Result<(), &'static str> {
        if !std::ptr::fn_addr_eq(
            QuadLane::<T>::certified().cut_face_rounds,
            quad_lane::cut_face_rounds::<T> as fn(_, _, _, _, _, _, _, _) -> _,
        ) {
            return Err("cut_face_rounds is not `quad_lane::cut_face_rounds`");
        }
        if !std::ptr::fn_addr_eq(
            QuadLane::<T>::certified().closed_form,
            quad_lane::closed_form::<T> as fn(_, _, _, _, _, _) -> _,
        ) {
            return Err("closed_form is not `quad_lane::closed_form`");
        }
        if !std::ptr::fn_addr_eq(
            QuadLane::<T>::certified().lift,
            geom_core::interval::Interval::from_certified::<T> as fn(_) -> _,
        ) {
            return Err("lift is not `Interval::from_certified`");
        }
        Ok(())
    }

    /// `Ok(())` when the shell door holds `shell_open`; otherwise the
    /// name of the field that moved.
    fn holds_the_certified_shell_door<T: geom_core::CertifiedBounds + AtRestPolicy>()
    -> Result<(), &'static str> {
        if !std::ptr::fn_addr_eq(
            ShellDoor::<T>::certified().open,
            crate::shell::shell_open::<T> as fn(_, _, _, _) -> _,
        ) {
            return Err("open is not `shell_open`");
        }
        Ok(())
    }

    /// Each certifying scalar's region lane is the certified doors
    /// ([`crate::RegionLane::certified`]); a dual's is absent
    /// (`dual_gate_is_absent_not_a_verdict`).
    #[test]
    fn every_certifying_scalar_is_wired_to_the_certified_region_lane() {
        assert!(
            <f64 as AtRestPolicy>::region_lane().is_some_and(|l| l.is_certified()),
            "f64"
        );
        assert!(
            <geom_core::Sym<f64> as AtRestPolicy>::region_lane().is_some_and(|l| l.is_certified()),
            "Sym<f64>"
        );
        assert!(
            <geom_core::interval::Interval as AtRestPolicy>::region_lane()
                .is_some_and(|l| l.is_certified()),
            "Interval"
        );
    }

    #[cfg(feature = "probe")]
    #[test]
    fn probe_is_wired_to_the_certified_region_lane() {
        assert!(
            <geom_core::Probe as AtRestPolicy>::region_lane().is_some_and(|l| l.is_certified()),
            "Probe"
        );
    }

    #[test]
    fn f64_is_wired_to_the_certified_quadrature() {
        assert_eq!(
            holds_the_certified_quadrature::<f64>(),
            Ok(()),
            "`QuadLane::<f64>::certified()` holds something other than `quad_lane::cut_face_rounds`"
        );
    }

    /// The symbolic tier holds the base scalar's lane: the same
    /// pointer, instantiated at `Sym<f64>`.
    #[test]
    fn sym_over_f64_is_wired_to_the_certified_quadrature() {
        assert_eq!(
            holds_the_certified_quadrature::<geom_core::Sym<f64>>(),
            Ok(()),
            "`QuadLane::<Sym<f64>>::certified()` holds something other than `quad_lane::cut_face_rounds`"
        );
    }

    #[cfg(feature = "probe")]
    #[test]
    fn probe_is_wired_to_the_certified_quadrature() {
        assert_eq!(
            holds_the_certified_quadrature::<geom_core::Probe>(),
            Ok(()),
            "`QuadLane::<Probe>::certified()` holds something other than `quad_lane::cut_face_rounds`"
        );
    }

    #[test]
    fn interval_is_wired_to_the_certified_quadrature() {
        assert_eq!(
            holds_the_certified_quadrature::<geom_core::interval::Interval>(),
            Ok(()),
            "`QuadLane::<Interval>::certified()` holds something other than `quad_lane::cut_face_rounds`"
        );
    }

    #[test]
    fn f64_is_wired_to_the_certified_shell_door() {
        assert_eq!(
            holds_the_certified_shell_door::<f64>(),
            Ok(()),
            "`ShellDoor::<f64>::certified()` holds something other than `shell_open`"
        );
    }

    /// The symbolic tier holds the base scalar's door: the same
    /// pointer, instantiated at `Sym<f64>`.
    #[test]
    fn sym_over_f64_is_wired_to_the_certified_shell_door() {
        assert_eq!(
            holds_the_certified_shell_door::<geom_core::Sym<f64>>(),
            Ok(()),
            "`ShellDoor::<Sym<f64>>::certified()` holds something other than `shell_open`"
        );
    }

    #[cfg(feature = "probe")]
    #[test]
    fn probe_is_wired_to_the_certified_shell_door() {
        assert_eq!(
            holds_the_certified_shell_door::<geom_core::Probe>(),
            Ok(()),
            "`ShellDoor::<Probe>::certified()` holds something other than `shell_open`"
        );
    }

    #[test]
    fn interval_is_wired_to_the_certified_shell_door() {
        assert_eq!(
            holds_the_certified_shell_door::<geom_core::interval::Interval>(),
            Ok(()),
            "`ShellDoor::<Interval>::certified()` holds something other than `shell_open`"
        );
    }
}

/// The **scalar policy for the certified at-rest gates**
/// (`docs/DUAL-DESIGN.md` DL3): whether an evaluation-service
/// consumer of [`crate::validate_geometric`] /
/// [`crate::validate_pseudomanifold`] runs them at this scalar.
///
/// Certified validation is an act of certification — its tier-3
/// battery re-derives surface certificates
/// ([`geom_brep::OffsetFitLane::recertify`]), encloses volume flux
/// through the quadrature lane, and certifies the contact census —
/// so it belongs to the scalars with certification rights (`f64`,
/// the telemetry probe, the interval scalar, and the symbolic tier
/// over any of them), whose impls here
/// delegate to the validation doors verbatim. At a
/// [`Dual`](geom_core::Dual) the gate is **structurally absent**:
/// the impl calls nothing, and its success arm SAYS so — the outcome
/// type separates [`AtRestOutcome::Validated`] from
/// [`AtRestOutcome::NotRunAtThisScalar`], so a dual gate's `Ok` can
/// never be read as a certification at any call site. What makes the
/// absence sound is the PAIRING OBLIGATION: a dual evaluation rides
/// BESIDE a base-scalar evaluation of the same recipe, whose value
/// channel is bit-identical (the dual contract) and which these same
/// gates validate. Nothing in the type system enforces that pairing
/// at the public doors today; the E4/E5 report enforces it where a
/// dual's numbers are read, binding each pass to the validated base
/// build by build identity (DL3). That is sound because
/// a dual evaluation's value channel is bit-identical to the base
/// scalar's run of the same recipe (the dual contract), which the
/// base-scalar evaluation it rides beside already validates;
/// re-validating the same bits through the dual's refusing
/// certification arms adds no information. Asking a dual to
/// validate IS asking it to certify, and a dual never certifies
/// (DL1).
///
/// This is a compile-time, per-scalar policy — never a runtime flag,
/// and never a swallowed per-face error: the dual arm does not run
/// validation and discard refusals, it runs nothing. The validation
/// doors themselves keep their meaning at every scalar their own
/// bounds admit; this trait only decides which scalars'
/// evaluation-service gates consult them.
///
/// The trait also carries the nine INJECTED DOORS whose presence is a
/// per-scalar fact, for the same reason it carries the gates: it is
/// the per-scalar policy home. [`AtRestPolicy::offset_fit_lane`] is
/// the offset fit's, [`AtRestPolicy::fitted_lane`] is the fitted
/// pcurve derivations', [`AtRestPolicy::spline_foot`] is the spline
/// re-anchor's, [`AtRestPolicy::section_lane`] and
/// [`AtRestPolicy::ssi_lane`] are the two marches', [`AtRestPolicy::shell_door`]
/// is the hollowing verb's, [`AtRestPolicy::quad_lane`] the
/// quadrature's, [`AtRestPolicy::region_lane`] the chart-region
/// doors', and [`AtRestPolicy::clearance_lane`] the clearance engine's;
/// each answers `None` for its own reason — a derivation written at
/// some scalars, certification rights (DL1), or a value only an
/// enclosure can hold — and the doc on each method says which. The edge door is not one of them: it is
/// two functions every scalar implements ([`AtRestPolicy::attach_edge`],
/// [`AtRestPolicy::reattach_edge`]), and the scalar names what its
/// edges carry ([`geom_core::Real::Carries`]). What a reader gets from the
/// one trait is every per-scalar answer the at-rest machinery needs,
/// in one place, rather than a lane trait apiece; the scalar's name,
/// which the refusals those `None`s produce carry, is not one of them
/// but the scalar's own ([`geom_core::Real::NAME`]).
///
/// [`geom_core::Bounds`] deliberately does not ride along: a name that
/// hands out a bracket door is a bound the `Bounds` scope rule's gate
/// cannot read at its use sites, so every door that reads a bracket
/// spells `Bounds` where the reader can see it.
pub trait AtRestPolicy: Decide {
    /// **This scalar's offset-fit door, or `None` where the fit is not
    /// derived here** — the ONE seam the `Some` comes from, read by
    /// check 1's tier-3 battery, the offset mint
    /// ([`crate::replace_face_offset`]) and the transform's surface
    /// map ([`crate::transform_rigid`]).
    ///
    /// `None` is a statement about the DERIVATION and never about
    /// which values may arrive: an `ApproxSurface<T>` is representable
    /// at every scalar, so a face carrying one does reach those passes
    /// at a scalar with no door, and each refuses typed rather than
    /// passing. That makes it a different fact from
    /// [`AtRestOutcome::NotRunAtThisScalar`] below, which is about
    /// certification rights (DL1).
    ///
    /// It is a per-scalar seam and not a lane trait of its own
    /// (H5's ratified ruling 3, PR 2701, which keeps this trait
    /// as the per-scalar policy that cut leaves standing): the door
    /// itself is a value the passes take as a parameter, and this is
    /// the one place each scalar's answer is written. The same holds
    /// of every door beside it — nine doors, one policy, no trait
    /// apiece.
    fn offset_fit_lane() -> Option<geom_brep::OffsetFitLane<Self>>;

    /// **This scalar's fitted-pcurve door, or `None` where it may not
    /// certify** — the ONE seam the `Some` comes from, read by every
    /// consumer: the pcurve mint ([`crate::mint_pcurves`]: its general
    /// image, its chart-foot rim arms and its `certify_general` call)
    /// and the tier-3 pcurve pass ([`crate::pcurves::validate_pcurves`],
    /// through [`geom_brep::PcurveCache::recertify`]), whichever
    /// validation door reaches it.
    ///
    /// `None` is certification rights (DL1), the same fact as
    /// [`AtRestPolicy::shell_door`]'s: a fitted (rung-3) pcurve's
    /// between-samples obligation is a C2 certificate reached through
    /// certification arithmetic, exactly as the quadrature's flux
    /// enclosures are, and [`geom_brep::FittedLane`]'s one constructor
    /// is bounded on [`geom_core::CertifiedBounds`]. A consumer holding
    /// `None` refuses typed with
    /// [`geom_brep::PcurveCertifyError::FittedLaneUnsupported`] — or,
    /// at the mint's rim arms, keeps the refusal the arm already had,
    /// since no foot is measured.
    fn fitted_lane() -> Option<geom_brep::FittedLane<Self>>;

    /// **This scalar's edge door** — the ONE seam every operation
    /// generic over its scalar attaches an edge carrier through, read
    /// through `crate::policy_lane`: the transform
    /// ([`crate::transform_rigid`]); the Euler surface's attachment gate,
    /// which every door that mints or re-describes an edge runs — the
    /// new-edge mints ([`Body::mev`](crate::construction::Construct::mev), [`Body::mef`](crate::construction::Construct::mef), [`Body::mekr`](crate::construction::Construct::mekr) and
    /// the ring doors), [`Body::set_edge_curve`](crate::construction::Construct::set_edge_curve), [`Body::split_edge`](crate::construction::Construct::split_edge)
    /// and [`Body::kev_describing`](crate::construction::Construct::kev_describing)'s list; the re-chart
    /// ([`Body::set_face_surfaces_describing`](crate::construction::Construct::set_face_surfaces_describing)); the re-basing gate of a
    /// fan `mev` and of the kill doors' merge; and a graft's bridge.
    ///
    /// Certification is a property of the scalar's type (C2): a
    /// certifying scalar's door mints and certifies in full
    /// ([`geom_brep::EdgeCurve::certify_stated`] through
    /// [`geom_brep::NurbsLane::certified`]), and a tangent-transport
    /// scalar's door only mints ([`geom_brep::EdgeCurve::mint`], held by
    /// [`geom_brep::EdgeCurve::uncertified`]), so no code path at a dual
    /// reaches a certifying function (DUAL-DESIGN DL1, DL3). `stated` is
    /// a chart image the body already holds for the edge
    /// ([`geom_brep::EdgeCurve::certify_stated`]); `start`, `end` and
    /// `reach` are the certificate's inputs.
    ///
    /// # Errors
    ///
    /// The door's refusal, typed.
    #[allow(clippy::too_many_arguments)] // the door's own inputs
    fn attach_edge(
        spec: geom_brep::EdgeCurveSpec<Self>,
        start: Point3<Self>,
        end: Point3<Self>,
        surfaces: impl Fn(crate::geometry::SurfaceKey) -> Option<Surface<Self>>,
        reach: Self,
        band: Band,
        stated: Option<&geom_brep::StatedImage<Self>>,
    ) -> Result<geom_brep::EdgeCurve<Self>, geom_brep::CertifyError>;

    /// **This scalar's edge door at rest**: what
    /// [`AtRestPolicy::attach_edge`] gives an edge to carry, derived
    /// again from the edge as it stands — the certificate re-derived in
    /// full at a certifying scalar
    /// ([`geom_brep::EdgeCurve::recertify_stated`]; the stored one is
    /// never read), the mint re-run at a tangent-transport scalar.
    ///
    /// # Errors
    ///
    /// The door's refusal, typed.
    #[allow(clippy::too_many_arguments)] // the door's own inputs
    fn reattach_edge(
        curve: &geom_brep::EdgeCurve<Self>,
        start: Point3<Self>,
        end: Point3<Self>,
        surfaces: impl Fn(crate::geometry::SurfaceKey) -> Option<Surface<Self>>,
        reach: Self,
        band: Band,
        stated: Option<&geom_brep::StatedImage<Self>>,
    ) -> Result<Self::Carries<geom_brep::Certificate<Self>>, geom_brep::CertifyError>;

    /// **This scalar's spline foot, or `None` where it is not derived
    /// here** — the offset door's re-anchor of a spline-carried edge
    /// ([`crate::replace_face_offset`], through
    /// [`geom_brep::SplineFoot::carrier_foot`]). Newton's foot is written
    /// at the certified-bracket scalars
    /// ([`geom::NurbsCurve3::project_from_seed`]'s bound), so an offset
    /// whose edge needs it refuses at any other scalar by name
    /// ([`crate::ReplaceFaceError::NurbsLaneUnsupported`]).
    fn spline_foot() -> Option<geom_brep::SplineFoot<Self>>;

    /// **A spline span's crossings of an analytic face's carrier at this
    /// scalar, or `None` where it is not derived here** — the boolean's
    /// planar lane ([`geom_brep::NurbsLane::surface_crossings`], the
    /// curve × face table's ring-composite arm). The arm crosses its
    /// scalars into certification arithmetic by their certified
    /// enclosures, so it is read at the certifying scalars alone, and
    /// a dual answers `None` and the lane refuses the class by name.
    fn spline_face_crossings(
        net: &geom::NurbsCurve3<Self>,
        params: (Self, Self),
        surface: &geom::Surface<Self>,
        band: geom_core::Band,
    ) -> Option<Result<Vec<Self>, geom_brep::curve_face::composite::CompositeRefusal>>;

    /// **An analytic span's meetings with a spline chart at this
    /// scalar, or `None` where it is not derived here** — the boolean's
    /// spline-face lane ([`geom_brep::NurbsLane::chart_meetings`], the
    /// curve × face table's spline-face arm), read at the certifying
    /// scalars as [`AtRestPolicy::spline_face_crossings`] is, and `None`
    /// at a dual, where the lane refuses the class by name.
    fn spline_chart_meetings(
        carrier: &geom::Curve3<Self>,
        params: (Self, Self),
        chart: &geom::NurbsSurface<Self>,
        band: geom_core::Band,
    ) -> Option<
        Result<
            Vec<geom_brep::curve_face::spline_face::ChartMeeting<Self>>,
            geom_brep::curve_face::spline_face::SplineFaceRefusal,
        >,
    >;

    /// **A certified lower bound on a spline chart's arm at `p` over
    /// `reach`, at this scalar, or `None` where none is read here** — the
    /// lower end of the bracket the boolean's seam reading levers a spline
    /// face's normal over ([`geom_brep::NurbsLane::chart_arm`],
    /// [`geom_brep::classify_seam`]). It is read from certified curvature
    /// bounds, at the certifying scalars alone; a dual answers `None`, and
    /// the seam reads smooth or escalates, never transverse.
    fn spline_chart_arm(
        chart: &geom::NurbsSurface<Self>,
        p: geom_core::Point3<Self>,
        reach: Self,
    ) -> Option<f64>;

    /// **This scalar's section lane, or `None` where it is not
    /// derived here** — the offset door's plane × spline-wall
    /// section and the plane's root along a spline edge
    /// ([`crate::offset_derive::SectionLane`]). Its march is written at
    /// `f64` alone, so that arm answers `Some`; an offset whose edge or
    /// corner needs it refuses at any other scalar by name
    /// ([`crate::ReplaceFaceError::NurbsLaneUnsupported`]).
    fn section_lane() -> Option<crate::offset_derive::SectionLane<Self>>;

    /// **This scalar's general-rung march door, or `None` where it is not
    /// derived here** — the boolean's section table carries a section on
    /// the general rung through it ([`geom_brep::PairSection::carry`]).
    /// The ℝ³ march is written at `f64` alone, so that arm answers
    /// `Some`; a join whose germs ride such a section refuses at any
    /// other scalar by name ([`geom_brep::CarryError::NoLane`]).
    fn ssi_lane() -> Option<geom_brep::SsiLane<Self>>;

    /// **This scalar's shell door, or `None` where it may not form the
    /// call** — the ONE seam the `Some` comes from, read by the verb
    /// seat's `verbs::Shell::run_shell` and, above it, the document
    /// layer's shell lowering.
    ///
    /// `None` is an answer and never a fallback: [`ShellDoor`]'s body
    /// is [`crate::shell_open`], whose last act is the certified
    /// at-rest validator, so a scalar without certification rights
    /// cannot hold one and the lowering refuses TYPED rather than
    /// building an unvalidated hollow. That makes it the same fact as
    /// [`AtRestOutcome::NotRunAtThisScalar`] below — certification
    /// rights (DL1) — and a different one from
    /// [`AtRestPolicy::offset_fit_lane`] above, which is about where a
    /// derivation is written.
    fn shell_door() -> Option<ShellDoor<Self>>;

    /// **This scalar's quadrature lane, or `None` where it may not
    /// certify** — the lane a door that reads shell roles outside the
    /// validator reads them through ([`crate::pieces`]), the same one
    /// the at-rest gate makes check 7 and check 10 through. Certification
    /// rights, as [`AtRestPolicy::shell_door`]: `None` reads closed forms
    /// only.
    fn quad_lane() -> Option<QuadLane<Self>>;

    /// **This scalar's chart-region doors, or `None` where they may not
    /// certify** — what an op that confirms a record outside the
    /// census's own doors reads a face–face pair through
    /// ([`crate::census`]'s record confirm, run by the boolean's carry),
    /// as `validate_pseudomanifold` hands the census at a certifying
    /// scalar. `None` leaves a face–face pair unread.
    fn region_lane() -> Option<crate::RegionLane<Self>>;

    /// **This scalar's clearance engine, or `None` where it does not
    /// answer** — the ONE seam the `Some` comes from, read by the
    /// `min_clearance` measure and the document-level clearance query.
    ///
    /// The engine's answer is a BRACKET on a minimum over a box, so the
    /// door answers at the enclosure scalars and wherever the symbolic
    /// tier wraps one. `None` at a dual is certification rights (DL1),
    /// as [`AtRestPolicy::shell_door`]'s is. `None` at `f64` and at the
    /// recording scalar is not a right but a shape: a point build
    /// evaluates one parameter point, where the engine's walk would
    /// answer an upper bound on the minimum found by search, not the
    /// minimum, and that value would sit in the witness build beside
    /// a leaf's bracket as if the two were one quantity
    /// (`work/clear/min-separation-tightening-crosses-the-drive.md`).
    fn clearance_lane() -> Option<crate::ClearanceLane<Self>>;

    /// The at-rest gate over a body ([`crate::validate_geometric`] at
    /// certifying scalars; absent at duals, and the outcome says
    /// which).
    ///
    /// # Errors
    ///
    /// The validator's own findings, verbatim, where the scalar runs
    /// it.
    fn gate_at_rest(body: &Body<Self>, tol: Tol) -> Result<AtRestOutcome, Vec<ValidationError>>;

    /// [`AtRestPolicy::gate_at_rest`] over a body the caller hands on,
    /// keeping the verdict with it ([`crate::AtRestBody::validate`] at
    /// certifying scalars; the body carried unvalidated at duals), so
    /// [`AtRestPolicy::gate_at_rest_declared`] over the same body pays
    /// the census alone.
    ///
    /// # Errors
    ///
    /// The validator's own findings, verbatim, where the scalar runs
    /// it.
    fn gate_at_rest_kept(
        body: Body<Self>,
        tol: Tol,
    ) -> Result<crate::AtRestBody<Self>, Vec<ValidationError>>;

    /// The at-rest gate over a gated body with contact records — the
    /// tier-3′ pass ([`crate::AtRestBody::validate_pseudomanifold`],
    /// which is [`crate::validate_pseudomanifold`] less the battery the
    /// kept verdict already answers, at certifying scalars; absent at
    /// duals).
    ///
    /// # Errors
    ///
    /// The validator's own findings, verbatim, where the scalar runs
    /// it.
    fn gate_at_rest_declared(
        body: &crate::AtRestBody<Self>,
        contacts: &ContactRecords,
        tol: Tol,
    ) -> Result<AtRestOutcome, Vec<ValidationError>>;

    /// The flush pairs between the groups `group` partitions a body at
    /// rest into ([`crate::census::flush_at_rest`]), through this
    /// scalar's region doors, at the scalars that run the at-rest gate;
    /// nothing where the gate does not run.
    fn flush_at_rest(
        body: &Body<Self>,
        group: &dyn Fn(crate::SolidKey) -> Option<u32>,
        band: Band,
    ) -> crate::FlushAtRest;
}

/// What an [`AtRestPolicy`] gate's success MEANS — the word that keeps
/// a non-certifying scalar's `Ok` from reading as a certification.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AtRestOutcome {
    /// The certified at-rest validator ran on these bits and passed.
    Validated,
    /// This scalar's policy runs no validator (a dual): nothing was
    /// checked here and nothing is granted. The base-scalar evaluation
    /// beside this one is the validation of record — the DL3 pairing
    /// obligation, stated at the doors that return this.
    NotRunAtThisScalar,
}

impl AtRestPolicy for f64 {
    /// The fit IS written here: `geom_brep::offset_fit` is an `f64`
    /// module throughout, so this is the one arm that answers `Some`.
    fn offset_fit_lane() -> Option<geom_brep::OffsetFitLane<Self>> {
        Some(geom_brep::OffsetFitLane::fit())
    }

    /// The decide-with-escalation lane certifies, so it derives fitted
    /// pcurves.
    fn fitted_lane() -> Option<geom_brep::FittedLane<Self>> {
        Some(geom_brep::FittedLane::certified())
    }

    /// The decide-with-escalation lane certifies, so its door mints and
    /// certifies.
    fn attach_edge(
        spec: geom_brep::EdgeCurveSpec<Self>,
        start: Point3<Self>,
        end: Point3<Self>,
        surfaces: impl Fn(crate::geometry::SurfaceKey) -> Option<Surface<Self>>,
        reach: Self,
        band: Band,
        stated: Option<&geom_brep::StatedImage<Self>>,
    ) -> Result<geom_brep::EdgeCurve<Self>, geom_brep::CertifyError> {
        crate::policy_lane::certifying_attach(spec, start, end, surfaces, reach, band, stated)
    }

    fn reattach_edge(
        curve: &geom_brep::EdgeCurve<Self>,
        start: Point3<Self>,
        end: Point3<Self>,
        surfaces: impl Fn(crate::geometry::SurfaceKey) -> Option<Surface<Self>>,
        reach: Self,
        band: Band,
        stated: Option<&geom_brep::StatedImage<Self>>,
    ) -> Result<Self::Carries<geom_brep::Certificate<Self>>, geom_brep::CertifyError> {
        crate::policy_lane::certifying_reattach(curve, start, end, surfaces, reach, band, stated)
    }

    fn spline_foot() -> Option<geom_brep::SplineFoot<Self>> {
        Some(geom_brep::SplineFoot::certified())
    }

    fn spline_face_crossings(
        net: &geom::NurbsCurve3<Self>,
        params: (Self, Self),
        surface: &geom::Surface<Self>,
        band: geom_core::Band,
    ) -> Option<Result<Vec<Self>, geom_brep::curve_face::composite::CompositeRefusal>> {
        Some(geom_brep::NurbsLane::certified().surface_crossings(net, params, surface, band))
    }

    fn spline_chart_meetings(
        carrier: &geom::Curve3<Self>,
        params: (Self, Self),
        chart: &geom::NurbsSurface<Self>,
        band: geom_core::Band,
    ) -> Option<
        Result<
            Vec<geom_brep::curve_face::spline_face::ChartMeeting<Self>>,
            geom_brep::curve_face::spline_face::SplineFaceRefusal,
        >,
    > {
        Some(geom_brep::NurbsLane::certified().chart_meetings(carrier, params, chart, band))
    }

    fn spline_chart_arm(
        chart: &geom::NurbsSurface<Self>,
        p: geom_core::Point3<Self>,
        reach: Self,
    ) -> Option<f64> {
        geom_brep::NurbsLane::certified().chart_arm(chart, p, reach)
    }

    /// The march is written here.
    fn section_lane() -> Option<crate::offset_derive::SectionLane<Self>> {
        Some(crate::offset_derive::SectionLane::f64())
    }

    /// The march is written here.
    fn ssi_lane() -> Option<geom_brep::SsiLane<Self>> {
        Some(geom_brep::SsiLane::marched())
    }

    /// The decide-with-escalation lane certifies, so it runs the door.
    fn shell_door() -> Option<ShellDoor<Self>> {
        Some(ShellDoor::certified())
    }

    fn quad_lane() -> Option<QuadLane<Self>> {
        Some(QuadLane::certified())
    }

    fn region_lane() -> Option<crate::RegionLane<Self>> {
        Some(crate::RegionLane::certified())
    }

    fn clearance_lane() -> Option<crate::ClearanceLane<Self>> {
        None
    }

    fn gate_at_rest(body: &Body<Self>, tol: Tol) -> Result<AtRestOutcome, Vec<ValidationError>> {
        crate::validate::validate_geometric(body, tol).map(|()| AtRestOutcome::Validated)
    }

    fn gate_at_rest_kept(
        body: Body<Self>,
        tol: Tol,
    ) -> Result<crate::AtRestBody<Self>, Vec<ValidationError>> {
        crate::AtRestBody::validate(body, tol)
    }

    fn gate_at_rest_declared(
        body: &crate::AtRestBody<Self>,
        contacts: &ContactRecords,
        tol: Tol,
    ) -> Result<AtRestOutcome, Vec<ValidationError>> {
        body.validate_pseudomanifold(contacts, tol)
            .map(|()| AtRestOutcome::Validated)
    }

    fn flush_at_rest(
        body: &Body<Self>,
        group: &dyn Fn(crate::SolidKey) -> Option<u32>,
        band: Band,
    ) -> crate::FlushAtRest {
        crate::census::flush_at_rest(body, group, band, Self::region_lane())
    }
}

#[cfg(feature = "probe")]
impl AtRestPolicy for geom_core::Probe {
    /// The fit is derived at `f64` only. The recording scalar is `f64`
    /// with a sink attached, and that is still not the type
    /// `geom_brep::offset_fit` is written in.
    fn offset_fit_lane() -> Option<geom_brep::OffsetFitLane<Self>> {
        None
    }

    /// Certification rights, not the scalar a derivation is written
    /// in, decide this door: the recording scalar's arithmetic is
    /// `f64`'s, observed, and it holds `geom_core::CertifiedBounds`, so
    /// it derives fitted pcurves through the same generic bodies —
    /// where the offset fit above, written at `f64` alone, stays out.
    fn fitted_lane() -> Option<geom_brep::FittedLane<Self>> {
        Some(geom_brep::FittedLane::certified())
    }

    /// As the fitted door above: certification rights decide it, and
    /// the recording scalar holds them.
    fn attach_edge(
        spec: geom_brep::EdgeCurveSpec<Self>,
        start: Point3<Self>,
        end: Point3<Self>,
        surfaces: impl Fn(crate::geometry::SurfaceKey) -> Option<Surface<Self>>,
        reach: Self,
        band: Band,
        stated: Option<&geom_brep::StatedImage<Self>>,
    ) -> Result<geom_brep::EdgeCurve<Self>, geom_brep::CertifyError> {
        crate::policy_lane::certifying_attach(spec, start, end, surfaces, reach, band, stated)
    }

    fn reattach_edge(
        curve: &geom_brep::EdgeCurve<Self>,
        start: Point3<Self>,
        end: Point3<Self>,
        surfaces: impl Fn(crate::geometry::SurfaceKey) -> Option<Surface<Self>>,
        reach: Self,
        band: Band,
        stated: Option<&geom_brep::StatedImage<Self>>,
    ) -> Result<Self::Carries<geom_brep::Certificate<Self>>, geom_brep::CertifyError> {
        crate::policy_lane::certifying_reattach(curve, start, end, surfaces, reach, band, stated)
    }

    fn spline_foot() -> Option<geom_brep::SplineFoot<Self>> {
        Some(geom_brep::SplineFoot::certified())
    }

    fn spline_face_crossings(
        net: &geom::NurbsCurve3<Self>,
        params: (Self, Self),
        surface: &geom::Surface<Self>,
        band: geom_core::Band,
    ) -> Option<Result<Vec<Self>, geom_brep::curve_face::composite::CompositeRefusal>> {
        Some(geom_brep::NurbsLane::certified().surface_crossings(net, params, surface, band))
    }

    fn spline_chart_meetings(
        carrier: &geom::Curve3<Self>,
        params: (Self, Self),
        chart: &geom::NurbsSurface<Self>,
        band: geom_core::Band,
    ) -> Option<
        Result<
            Vec<geom_brep::curve_face::spline_face::ChartMeeting<Self>>,
            geom_brep::curve_face::spline_face::SplineFaceRefusal,
        >,
    > {
        Some(geom_brep::NurbsLane::certified().chart_meetings(carrier, params, chart, band))
    }

    fn spline_chart_arm(
        chart: &geom::NurbsSurface<Self>,
        p: geom_core::Point3<Self>,
        reach: Self,
    ) -> Option<f64> {
        geom_brep::NurbsLane::certified().chart_arm(chart, p, reach)
    }

    /// The march is written at `f64` alone.
    fn section_lane() -> Option<crate::offset_derive::SectionLane<Self>> {
        None
    }

    /// The march is written at `f64` alone, as the section lane's
    /// above.
    fn ssi_lane() -> Option<geom_brep::SsiLane<Self>> {
        None
    }

    /// The recording scalar is `f64` with a sink attached, so it
    /// carries exactly what `f64` carries — here, the door.
    fn shell_door() -> Option<ShellDoor<Self>> {
        Some(ShellDoor::certified())
    }

    fn quad_lane() -> Option<QuadLane<Self>> {
        Some(QuadLane::certified())
    }

    fn region_lane() -> Option<crate::RegionLane<Self>> {
        Some(crate::RegionLane::certified())
    }

    fn clearance_lane() -> Option<crate::ClearanceLane<Self>> {
        None
    }

    fn gate_at_rest(body: &Body<Self>, tol: Tol) -> Result<AtRestOutcome, Vec<ValidationError>> {
        crate::validate::validate_geometric(body, tol).map(|()| AtRestOutcome::Validated)
    }

    fn gate_at_rest_kept(
        body: Body<Self>,
        tol: Tol,
    ) -> Result<crate::AtRestBody<Self>, Vec<ValidationError>> {
        crate::AtRestBody::validate(body, tol)
    }

    fn gate_at_rest_declared(
        body: &crate::AtRestBody<Self>,
        contacts: &ContactRecords,
        tol: Tol,
    ) -> Result<AtRestOutcome, Vec<ValidationError>> {
        body.validate_pseudomanifold(contacts, tol)
            .map(|()| AtRestOutcome::Validated)
    }

    fn flush_at_rest(
        body: &Body<Self>,
        group: &dyn Fn(crate::SolidKey) -> Option<u32>,
        band: Band,
    ) -> crate::FlushAtRest {
        crate::census::flush_at_rest(body, group, band, Self::region_lane())
    }
}

impl AtRestPolicy for geom_core::interval::Interval {
    /// The fit is derived at `f64` only — a fact about the scalar the
    /// derivation was written in, not about this scalar's
    /// certification rights, which it has in full.
    fn offset_fit_lane() -> Option<geom_brep::OffsetFitLane<Self>> {
        None
    }

    /// The certified interval scalar derives fitted pcurves: its
    /// brackets are what the C2 certificate's hull bound is made of.
    fn fitted_lane() -> Option<geom_brep::FittedLane<Self>> {
        Some(geom_brep::FittedLane::certified())
    }

    /// The certified interval scalar certifies: its brackets are what
    /// the limbs' hull bounds are made of.
    fn attach_edge(
        spec: geom_brep::EdgeCurveSpec<Self>,
        start: Point3<Self>,
        end: Point3<Self>,
        surfaces: impl Fn(crate::geometry::SurfaceKey) -> Option<Surface<Self>>,
        reach: Self,
        band: Band,
        stated: Option<&geom_brep::StatedImage<Self>>,
    ) -> Result<geom_brep::EdgeCurve<Self>, geom_brep::CertifyError> {
        crate::policy_lane::certifying_attach(spec, start, end, surfaces, reach, band, stated)
    }

    fn reattach_edge(
        curve: &geom_brep::EdgeCurve<Self>,
        start: Point3<Self>,
        end: Point3<Self>,
        surfaces: impl Fn(crate::geometry::SurfaceKey) -> Option<Surface<Self>>,
        reach: Self,
        band: Band,
        stated: Option<&geom_brep::StatedImage<Self>>,
    ) -> Result<Self::Carries<geom_brep::Certificate<Self>>, geom_brep::CertifyError> {
        crate::policy_lane::certifying_reattach(curve, start, end, surfaces, reach, band, stated)
    }

    fn spline_foot() -> Option<geom_brep::SplineFoot<Self>> {
        Some(geom_brep::SplineFoot::certified())
    }

    fn spline_face_crossings(
        net: &geom::NurbsCurve3<Self>,
        params: (Self, Self),
        surface: &geom::Surface<Self>,
        band: geom_core::Band,
    ) -> Option<Result<Vec<Self>, geom_brep::curve_face::composite::CompositeRefusal>> {
        Some(geom_brep::NurbsLane::certified().surface_crossings(net, params, surface, band))
    }

    fn spline_chart_meetings(
        carrier: &geom::Curve3<Self>,
        params: (Self, Self),
        chart: &geom::NurbsSurface<Self>,
        band: geom_core::Band,
    ) -> Option<
        Result<
            Vec<geom_brep::curve_face::spline_face::ChartMeeting<Self>>,
            geom_brep::curve_face::spline_face::SplineFaceRefusal,
        >,
    > {
        Some(geom_brep::NurbsLane::certified().chart_meetings(carrier, params, chart, band))
    }

    fn spline_chart_arm(
        chart: &geom::NurbsSurface<Self>,
        p: geom_core::Point3<Self>,
        reach: Self,
    ) -> Option<f64> {
        geom_brep::NurbsLane::certified().chart_arm(chart, p, reach)
    }

    /// The march is written at `f64` alone.
    fn section_lane() -> Option<crate::offset_derive::SectionLane<Self>> {
        None
    }

    /// The march is written at `f64` alone.
    fn ssi_lane() -> Option<geom_brep::SsiLane<Self>> {
        None
    }

    /// The certified interval scalar runs the door: its brackets are
    /// what the validator's certified claim is made of.
    fn shell_door() -> Option<ShellDoor<Self>> {
        Some(ShellDoor::certified())
    }

    fn quad_lane() -> Option<QuadLane<Self>> {
        Some(QuadLane::certified())
    }

    fn region_lane() -> Option<crate::RegionLane<Self>> {
        Some(crate::RegionLane::certified())
    }

    fn clearance_lane() -> Option<crate::ClearanceLane<Self>> {
        Some(crate::ClearanceLane::certified())
    }

    fn gate_at_rest(body: &Body<Self>, tol: Tol) -> Result<AtRestOutcome, Vec<ValidationError>> {
        crate::validate::validate_geometric(body, tol).map(|()| AtRestOutcome::Validated)
    }

    fn gate_at_rest_kept(
        body: Body<Self>,
        tol: Tol,
    ) -> Result<crate::AtRestBody<Self>, Vec<ValidationError>> {
        crate::AtRestBody::validate(body, tol)
    }

    fn gate_at_rest_declared(
        body: &crate::AtRestBody<Self>,
        contacts: &ContactRecords,
        tol: Tol,
    ) -> Result<AtRestOutcome, Vec<ValidationError>> {
        body.validate_pseudomanifold(contacts, tol)
            .map(|()| AtRestOutcome::Validated)
    }

    fn flush_at_rest(
        body: &Body<Self>,
        group: &dyn Fn(crate::SolidKey) -> Option<u32>,
        band: Band,
    ) -> crate::FlushAtRest {
        crate::census::flush_at_rest(body, group, band, Self::region_lane())
    }
}

/// **The symbolic tier over a certifying scalar**: the gates are the
/// base scalar's, run at `Sym<T>`. A leaf the driver certifies is
/// validated at rest exactly as it was before the tier existed —
/// demoting to the dual's `NotRunAtThisScalar` arm here would quietly
/// drop the validator from the one replay that certifies.
impl<T> AtRestPolicy for geom_core::Sym<T>
where
    geom_core::Sym<T>: Decide + geom_core::Bounds,
    T: geom_core::CertifiedBounds,
    T: AtRestPolicy,
{
    /// The fit is derived at `f64` only. The tier changes how a margin
    /// DECIDES, not which derivations exist, so wrapping a scalar
    /// cannot add one.
    fn offset_fit_lane() -> Option<geom_brep::OffsetFitLane<Self>> {
        None
    }

    /// Every door is the base scalar's, run at `Sym<T>`: the tier
    /// alters one decision rule inside the scalar, so a `Sym`-wrapped
    /// certifying scalar still mints and re-derives fitted caches.
    fn fitted_lane() -> Option<geom_brep::FittedLane<Self>> {
        Some(geom_brep::FittedLane::certified())
    }

    /// The base scalar's door, run at `Sym<T>`, for the reason the
    /// fitted door above gives.
    fn attach_edge(
        spec: geom_brep::EdgeCurveSpec<Self>,
        start: Point3<Self>,
        end: Point3<Self>,
        surfaces: impl Fn(crate::geometry::SurfaceKey) -> Option<Surface<Self>>,
        reach: Self,
        band: Band,
        stated: Option<&geom_brep::StatedImage<Self>>,
    ) -> Result<geom_brep::EdgeCurve<Self>, geom_brep::CertifyError> {
        crate::policy_lane::certifying_attach(spec, start, end, surfaces, reach, band, stated)
    }

    fn reattach_edge(
        curve: &geom_brep::EdgeCurve<Self>,
        start: Point3<Self>,
        end: Point3<Self>,
        surfaces: impl Fn(crate::geometry::SurfaceKey) -> Option<Surface<Self>>,
        reach: Self,
        band: Band,
        stated: Option<&geom_brep::StatedImage<Self>>,
    ) -> Result<Self::Carries<geom_brep::Certificate<Self>>, geom_brep::CertifyError> {
        crate::policy_lane::certifying_reattach(curve, start, end, surfaces, reach, band, stated)
    }

    fn spline_foot() -> Option<geom_brep::SplineFoot<Self>> {
        Some(geom_brep::SplineFoot::certified())
    }

    fn spline_face_crossings(
        net: &geom::NurbsCurve3<Self>,
        params: (Self, Self),
        surface: &geom::Surface<Self>,
        band: geom_core::Band,
    ) -> Option<Result<Vec<Self>, geom_brep::curve_face::composite::CompositeRefusal>> {
        Some(geom_brep::NurbsLane::certified().surface_crossings(net, params, surface, band))
    }

    fn spline_chart_meetings(
        carrier: &geom::Curve3<Self>,
        params: (Self, Self),
        chart: &geom::NurbsSurface<Self>,
        band: geom_core::Band,
    ) -> Option<
        Result<
            Vec<geom_brep::curve_face::spline_face::ChartMeeting<Self>>,
            geom_brep::curve_face::spline_face::SplineFaceRefusal,
        >,
    > {
        Some(geom_brep::NurbsLane::certified().chart_meetings(carrier, params, chart, band))
    }

    fn spline_chart_arm(
        chart: &geom::NurbsSurface<Self>,
        p: geom_core::Point3<Self>,
        reach: Self,
    ) -> Option<f64> {
        geom_brep::NurbsLane::certified().chart_arm(chart, p, reach)
    }

    /// The march is written at `f64` alone.
    fn section_lane() -> Option<crate::offset_derive::SectionLane<Self>> {
        None
    }

    /// The march is written at `f64` alone.
    fn ssi_lane() -> Option<geom_brep::SsiLane<Self>> {
        None
    }

    /// For the reason [`QuadLane`] gives at the symbolic tier: the
    /// tier changes how an identically-zero margin decides and
    /// nothing else, so wrapping a certifying base must not demote a
    /// certifying door to an absent one — the driver's leaf replay
    /// would otherwise stop hollowing the bodies it certifies.
    fn shell_door() -> Option<ShellDoor<Self>> {
        Some(ShellDoor::certified())
    }

    fn quad_lane() -> Option<QuadLane<Self>> {
        Some(QuadLane::certified())
    }

    fn region_lane() -> Option<crate::RegionLane<Self>> {
        Some(crate::RegionLane::certified())
    }

    /// Exactly where the base scalar's engine answers: the tier
    /// changes how a margin decides, not whether a bracket has
    /// somewhere to go.
    fn clearance_lane() -> Option<crate::ClearanceLane<Self>> {
        T::clearance_lane().map(|_| crate::ClearanceLane::certified())
    }

    fn gate_at_rest(body: &Body<Self>, tol: Tol) -> Result<AtRestOutcome, Vec<ValidationError>> {
        crate::validate::validate_geometric(body, tol).map(|()| AtRestOutcome::Validated)
    }

    fn gate_at_rest_kept(
        body: Body<Self>,
        tol: Tol,
    ) -> Result<crate::AtRestBody<Self>, Vec<ValidationError>> {
        crate::AtRestBody::validate(body, tol)
    }

    fn gate_at_rest_declared(
        body: &crate::AtRestBody<Self>,
        contacts: &ContactRecords,
        tol: Tol,
    ) -> Result<AtRestOutcome, Vec<ValidationError>> {
        body.validate_pseudomanifold(contacts, tol)
            .map(|()| AtRestOutcome::Validated)
    }

    fn flush_at_rest(
        body: &Body<Self>,
        group: &dyn Fn(crate::SolidKey) -> Option<u32>,
        band: Band,
    ) -> crate::FlushAtRest {
        crate::census::flush_at_rest(body, group, band, Self::region_lane())
    }
}

/// The dual arm: STRUCTURALLY ABSENT — no validation door is named,
/// so nothing runs, nothing refuses, and no error exists to swallow;
/// the success arm is [`AtRestOutcome::NotRunAtThisScalar`], never a
/// claim about the geometry (trait docs). The base-scalar evaluation
/// of the same recipe is where these bits are validated — the DL3
/// pairing obligation.
impl<T> AtRestPolicy for geom_core::Dual<T>
where
    geom_core::Dual<T>: Decide + geom_core::TangentTransport,
{
    /// The fit is derived at `f64` only — the same reason every other
    /// arm here gives, and a separate fact from the gates below, which
    /// are absent because a dual does not certify.
    fn offset_fit_lane() -> Option<geom_brep::OffsetFitLane<Self>> {
        None
    }

    /// **A dual does not certify** (DL1), and a fitted pcurve's
    /// certificate — like the image and chart feet it is certified
    /// over — is certification arithmetic, so no `Dual` can hold the
    /// door ([`geom_brep::FittedLane::certified`]'s bound). No fitted
    /// or `General` cache is built at a dual: the mint derives no
    /// general image and measures no chart foot here, and a `General`
    /// image a construction STATES in the face's own chart reaches the
    /// fitted lane's checks, which refuse typed at check 4 — after
    /// checks 1–3, which read no door and so answer as at every other
    /// scalar.
    fn fitted_lane() -> Option<geom_brep::FittedLane<Self>> {
        None
    }

    /// **A dual's door only mints** (DL1, DL3): it constructs the
    /// canonical description, carrier and interval, and the edge
    /// carries no certificate. What the dual measures is read against
    /// the input-identical certifying evaluation, which holds it.
    fn attach_edge(
        spec: geom_brep::EdgeCurveSpec<Self>,
        _start: Point3<Self>,
        _end: Point3<Self>,
        surfaces: impl Fn(crate::geometry::SurfaceKey) -> Option<Surface<Self>>,
        _reach: Self,
        band: Band,
        _stated: Option<&geom_brep::StatedImage<Self>>,
    ) -> Result<geom_brep::EdgeCurve<Self>, geom_brep::CertifyError> {
        crate::policy_lane::minting_attach(spec, surfaces, band)
    }

    /// The mint, run again on the edge as it stands.
    fn reattach_edge(
        curve: &geom_brep::EdgeCurve<Self>,
        _start: Point3<Self>,
        _end: Point3<Self>,
        surfaces: impl Fn(crate::geometry::SurfaceKey) -> Option<Surface<Self>>,
        _reach: Self,
        band: Band,
        _stated: Option<&geom_brep::StatedImage<Self>>,
    ) -> Result<Self::Carries<geom_brep::Certificate<Self>>, geom_brep::CertifyError> {
        crate::policy_lane::minting_reattach(curve, surfaces, band)
    }

    /// Newton's foot is written at the certified-bracket scalars, which
    /// a dual is not.
    fn spline_foot() -> Option<geom_brep::SplineFoot<Self>> {
        None
    }

    /// The arm reads certified enclosures, which a dual does not hold.
    fn spline_face_crossings(
        _net: &geom::NurbsCurve3<Self>,
        _params: (Self, Self),
        _surface: &geom::Surface<Self>,
        _band: geom_core::Band,
    ) -> Option<Result<Vec<Self>, geom_brep::curve_face::composite::CompositeRefusal>> {
        None
    }

    /// The arm reads certified enclosures, which a dual does not hold.
    fn spline_chart_meetings(
        _carrier: &geom::Curve3<Self>,
        _params: (Self, Self),
        _chart: &geom::NurbsSurface<Self>,
        _band: geom_core::Band,
    ) -> Option<
        Result<
            Vec<geom_brep::curve_face::spline_face::ChartMeeting<Self>>,
            geom_brep::curve_face::spline_face::SplineFaceRefusal,
        >,
    > {
        None
    }

    /// A dual does not certify (DL1), so it reads no certified bound.
    fn spline_chart_arm(
        _chart: &geom::NurbsSurface<Self>,
        _p: geom_core::Point3<Self>,
        _reach: Self,
    ) -> Option<f64> {
        None
    }

    /// The march is written at `f64` alone.
    fn section_lane() -> Option<crate::offset_derive::SectionLane<Self>> {
        None
    }

    /// The march is written at `f64` alone.
    fn ssi_lane() -> Option<geom_brep::SsiLane<Self>> {
        None
    }

    /// **A dual does not certify** (the DL3 ruling, unmoved), and the
    /// shell door's last act is a certified validation of what it
    /// built, so no `Dual` can hold one: a document evaluated for
    /// sensitivities meets a typed refusal at its shell node rather
    /// than an unvalidated hollow.
    fn shell_door() -> Option<ShellDoor<Self>> {
        None
    }

    fn quad_lane() -> Option<QuadLane<Self>> {
        None
    }

    fn region_lane() -> Option<crate::RegionLane<Self>> {
        None
    }

    fn clearance_lane() -> Option<crate::ClearanceLane<Self>> {
        None
    }

    fn gate_at_rest(_body: &Body<Self>, _tol: Tol) -> Result<AtRestOutcome, Vec<ValidationError>> {
        Ok(AtRestOutcome::NotRunAtThisScalar)
    }

    fn gate_at_rest_kept(
        body: Body<Self>,
        _tol: Tol,
    ) -> Result<crate::AtRestBody<Self>, Vec<ValidationError>> {
        Ok(crate::AtRestBody::not_run(body))
    }

    fn gate_at_rest_declared(
        _body: &crate::AtRestBody<Self>,
        _contacts: &ContactRecords,
        _tol: Tol,
    ) -> Result<AtRestOutcome, Vec<ValidationError>> {
        Ok(AtRestOutcome::NotRunAtThisScalar)
    }

    fn flush_at_rest(
        _body: &Body<Self>,
        _group: &dyn Fn(crate::SolidKey) -> Option<u32>,
        _band: Band,
    ) -> crate::FlushAtRest {
        crate::FlushAtRest::default()
    }
}

#[cfg(test)]
pub(crate) mod role_unread_rows {
    #![allow(clippy::expect_used, clippy::panic)]
    //! A role read that decided nothing is a zero volume only where both
    //! ends of the reading that decides it read zero: the certified
    //! reading where one is taken, else the walk's. A bracket with an end
    //! off zero leaves the volume unmeasured.

    use super::{Certified, RoleNames, RoleUnread, SignReading, VolumeEnclosure, certify_role};
    use geom_core::{Band, Interval, Tol};

    /// The witness band the rows read at.
    pub(crate) fn band() -> Band {
        Band::linear(Tol::witness()).expect("the witness band builds")
    }

    /// `walk` read as a role, certified off the interval `certified`
    /// (taken about the body) where one is given: a read that decides no
    /// role.
    pub(crate) fn unread(walk: SignReading<f64>, certified: Option<(f64, f64)>) -> RoleUnread {
        let names = RoleNames::one("role_unread_rows");
        let interval = |_| {
            certified.map(|(lo, hi)| {
                let volume = Interval::from_bounds(lo, hi);
                Ok((
                    SignReading::exact(volume, Interval::from_bounds(1.0, 1.0)),
                    None,
                ))
            })
        };
        let rounding = || Ok(0.0);
        match certify_role(walk, interval, rounding, names, names) {
            Certified::Open(unread) | Certified::Unresolved(unread, _) => unread,
            other => panic!("{walk:?} certified by {certified:?} read {other:?}"),
        }
    }

    fn bracket(volume_lo: f64, volume_hi: f64) -> SignReading<f64> {
        SignReading::Bracket(VolumeEnclosure {
            volume_lo,
            volume_hi,
            surface_area: 1.0,
        })
    }

    /// An interval wholly inside the band's sliver range.
    fn sliver() -> (f64, f64) {
        let (z, e) = (band().zero(), band().escalate());
        let mid = 0.5 * (z + e);
        (0.1f64.mul_add(-(e - z), mid), 0.1f64.mul_add(e - z, mid))
    }

    /// What a row's read is.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub(crate) enum Read {
        /// A zero volume.
        Zero,
        /// The certificate holds zero across it: its enclosure is the
        /// reading's escalation, which the refusal reports.
        HoldsZero,
        /// Neither: the volume is unmeasured.
        Unmeasured,
    }

    /// One row: its name, a walk, the certified interval if one is
    /// taken, and what the read is.
    pub(crate) type Row = (&'static str, SignReading<f64>, Option<(f64, f64)>, Read);

    /// The rows. A certificate is read by its enclosure alone
    /// ([`certify_role`]), so it reads zero only where the enclosure is
    /// zero at both ends; one that holds zero across it, however
    /// narrowly, is no zero volume but that enclosure's escalation.
    pub(crate) fn rows() -> Vec<Row> {
        let exact_zero = SignReading::exact(0.0, 1.0);
        vec![
            (
                "an exact zero, no certificate",
                exact_zero,
                None,
                Read::Zero,
            ),
            (
                "a bracket reaching across zero, no band to hold it",
                bracket(-1e-15, 1e-15),
                None,
                Read::Unmeasured,
            ),
            (
                "a bracket straddling zero",
                bracket(-1.0, 1.0),
                None,
                Read::Unmeasured,
            ),
            (
                "a bracket from zero up",
                bracket(0.0, 1.0),
                None,
                Read::Unmeasured,
            ),
            (
                "a bracket from below up to zero",
                bracket(-1.0, 0.0),
                None,
                Read::Unmeasured,
            ),
            (
                "an exact zero certified zero",
                exact_zero,
                Some((0.0, 0.0)),
                Read::Zero,
            ),
            (
                "an exact zero the certificate holds zero across",
                exact_zero,
                Some((-1e-15, 1e-15)),
                Read::HoldsZero,
            ),
            (
                "an exact zero the certificate straddles",
                exact_zero,
                Some((-1.0, 1.0)),
                Read::HoldsZero,
            ),
            (
                "a wide bracket certified zero",
                bracket(-1.0, 1.0),
                Some((0.0, 0.0)),
                Read::Zero,
            ),
        ]
    }

    #[test]
    fn a_read_is_a_zero_volume_only_where_its_deciding_ends_both_read_zero() {
        for (what, walk, certified, read) in rows() {
            let unread = unread(walk, certified);
            assert_eq!(unread.reads_zero(), read == Read::Zero, "{what}: zero");
            assert_eq!(
                unread.escalation().is_some(),
                read == Read::HoldsZero,
                "{what}: the enclosure's escalation"
            );
        }
    }

    /// A certificate wholly on one side of zero is that role, however
    /// thin, where the walk read zero: the band never holds it.
    #[test]
    fn a_certificate_inside_the_sliver_range_decides_its_role() {
        let names = RoleNames::one("role_unread_rows");
        let (lo, hi) = sliver();
        let interval = |_| {
            Some(Ok((
                SignReading::exact(
                    Interval::from_bounds(lo, hi),
                    Interval::from_bounds(1.0, 1.0),
                ),
                None,
            )))
        };
        assert!(matches!(
            certify_role(
                SignReading::exact(0.0, 1.0),
                interval,
                || Ok(0.0),
                names,
                names
            ),
            Certified::Role(super::ShellRole::Outer)
        ));
    }
}

#[cfg(test)]
mod at_rest_policy_tests {
    #![allow(clippy::expect_used)]
    //! The certifying policy arms ARE the validation doors — pinned on
    //! a REFUSING subject, because the passing direction is pinned all
    //! day by every green corpus gather while an arm gutted into a
    //! grant stays invisible to it. Each certifying (scalar, method)
    //! pair is asserted equal to its door on a body the door refuses,
    //! so `Ok(Validated)`-without-validating cannot survive these rows.
    //! The tier-3′ arm over a kept verdict is pinned the same way, on a
    //! subject tier 3 passes and the census refuses.
    //! The shell door is the one arm that is not a gate on that
    //! subject: it is a VALUE, so what these rows pin is the arm's
    //! answer — `ShellDoor::certified()` at a certifying scalar, `None`
    //! at a dual — and which function that value holds is
    //! `wiring_rows`' pin, per scalar, beside the quadrature door's.

    use super::{AtRestOutcome, AtRestPolicy};
    use crate::AtRestBody;
    use crate::body::Body;
    use crate::boolean::ContactRecords;
    use crate::test_support_fixtures::identity_map;
    use geom_core::{Decide, Point3, Tol};

    /// The `mvfs` seed body: its face surface is the placeholder, which
    /// a body at rest may not carry (`Body::mvfs` docs) — the cheapest
    /// scalar-generic refusing subject.
    fn refusing_body<T: Decide>() -> Body<T> {
        let mut b = Body::new();
        b.mvfs(Point3::new(T::zero(), T::zero(), T::zero()), true)
            .expect("mvfs has no preconditions");
        b
    }

    fn certifying_arms_are_the_doors<T: AtRestPolicy + geom_core::CertifiedBounds>() {
        let tol = Tol::witness();
        let b = refusing_body::<T>();
        let door = crate::validate::validate_geometric(&b, tol);
        assert!(door.is_err(), "the seed body must refuse validation");
        assert_eq!(
            T::gate_at_rest(&b, tol),
            door.map(|()| AtRestOutcome::Validated),
            "gate_at_rest must be validate_geometric verbatim at a certifying scalar"
        );
        assert_eq!(
            T::gate_at_rest_kept(b.clone(), tol).map(|kept| kept.outcome()),
            crate::validate::validate_geometric(&b, tol).map(|()| AtRestOutcome::Validated),
            "gate_at_rest_kept must refuse as validate_geometric at a certifying scalar"
        );
        let contacts = ContactRecords::default();
        // The certified door, and its name is the assertion: a
        // certifying arm takes the door whose bound names the right it
        // has, so check 2 re-derives the M7-8 carrier class here.
        // `validate_pseudomanifold_structural` is the lane-free sibling
        // a dual takes, and is not what this arm runs. A body carried
        // with no verdict takes the whole pass.
        let door = crate::validate::validate_pseudomanifold(&b, &contacts, tol);
        assert!(door.is_err(), "the seed body must refuse the census door");
        assert_eq!(
            T::gate_at_rest_declared(&AtRestBody::not_run(b), &contacts, tol),
            door.map(|()| AtRestOutcome::Validated),
            "gate_at_rest_declared must be validate_pseudomanifold verbatim over an unvalidated \
             body at a certifying scalar"
        );
        // Over a KEPT verdict the pass is the census alone, and the
        // subject must be one the census refuses and tier 3 does not —
        // two unit cubes overlapping by half — or a census skipped with
        // the battery would pass here unseen.
        let mut overlap = crate::test_support_fixtures::mapped_cube(identity_map::<T>, tol);
        crate::test_support_fixtures::cube_into(
            &mut overlap,
            |x, y, z| identity_map::<T>(x + 0.5, y + 0.5, z + 0.5),
            tol,
        );
        let door = crate::validate::validate_pseudomanifold(&overlap, &contacts, tol);
        assert!(
            door.is_err(),
            "the overlapping pair must refuse the census door"
        );
        let kept = T::gate_at_rest_kept(overlap, tol).expect("tier 3 alone passes the pair");
        assert_eq!(
            kept.outcome(),
            AtRestOutcome::Validated,
            "a certifying arm keeps a Validated verdict"
        );
        assert_eq!(
            T::gate_at_rest_declared(&kept, &contacts, tol),
            door.map(|()| AtRestOutcome::Validated),
            "gate_at_rest_declared over a kept verdict must refuse as validate_pseudomanifold, \
             finding for finding"
        );
        // The arm's `Some` is the door's one constructor and not a
        // value spelled beside it: the `impl AtRestPolicy for …` arms
        // live in the parent module, which owns the private field, so a
        // `ShellDoor { open: … }` literal in an arm would hold whatever
        // it names and `wiring_rows`, which pins `certified()`, would
        // not see it.
        let door = T::shell_door().expect("a certifying scalar holds the shell door");
        assert!(
            std::ptr::fn_addr_eq(door.open, super::ShellDoor::<T>::certified().open),
            "the certifying arm hands out something other than `ShellDoor::certified()`"
        );
        // The fitted door's fields are private to `geom_brep`, so the
        // arm cannot spell a literal: its `Some` is
        // `FittedLane::certified()` or nothing, and which functions that
        // holds is `geom_brep`'s own wiring pin.
        assert!(
            T::fitted_lane().is_some(),
            "a certifying scalar holds the fitted-pcurve door"
        );
        // The spline foot likewise: `SplineFoot::certified()` or nothing.
        assert!(
            T::spline_foot().is_some(),
            "a certifying scalar holds the spline foot"
        );
    }

    #[test]
    fn f64_gates_run_the_doors() {
        certifying_arms_are_the_doors::<f64>();
    }

    /// The symbolic tier over a certifying base, the arm
    /// `QuadLane`'s and `RegionLane`'s wiring rows already carry and
    /// this roster did not: `Sym<f64>` certifies, so it runs the
    /// gates and holds the shell door, and a tier that silently
    /// stopped handing the door out would otherwise red nothing here.
    #[test]
    fn sym_over_f64_gates_run_the_doors() {
        certifying_arms_are_the_doors::<geom_core::Sym<f64>>();
    }

    #[cfg(feature = "probe")]
    #[test]
    fn probe_gates_run_the_doors() {
        certifying_arms_are_the_doors::<geom_core::Probe>();
    }

    #[test]
    fn interval_gates_run_the_doors() {
        certifying_arms_are_the_doors::<geom_core::interval::Interval>();
    }

    /// The dual arm on the SAME refusing subject: the gate does not run
    /// and says so — [`AtRestOutcome::NotRunAtThisScalar`], never a
    /// verdict about geometry the door itself refuses.
    ///
    /// The direct door is no longer part of this row's contrast, and
    /// the reason is the point: `validate_geometric` cannot be CALLED
    /// at a dual — the composed entry carries the certified half's
    /// bound, so there is no refusal left to observe here. What a dual
    /// can still do is the `_structural` twin, and this row pins that
    /// instead: the seed body's placeholder surface is a check-1
    /// failure, which is structural, so the dual sees the same refusal
    /// the certifying scalars see through the same checks.
    #[test]
    fn dual_gate_is_absent_not_a_verdict() {
        let tol = Tol::witness();
        let b = refusing_body::<geom_core::Dual64>();
        assert!(
            crate::validate::validate_geometric_structural(&b, tol).is_err(),
            "the `_structural` twin still runs, and still refuses, at a dual"
        );
        assert_eq!(
            <geom_core::Dual64 as AtRestPolicy>::gate_at_rest(&b, tol),
            Ok(AtRestOutcome::NotRunAtThisScalar)
        );
        let kept = <geom_core::Dual64 as AtRestPolicy>::gate_at_rest_kept(b, tol)
            .expect("a dual keeps every body, gating none");
        assert_eq!(kept.outcome(), AtRestOutcome::NotRunAtThisScalar);
        assert_eq!(
            <geom_core::Dual64 as AtRestPolicy>::gate_at_rest_declared(
                &kept,
                &ContactRecords::default(),
                tol
            ),
            Ok(AtRestOutcome::NotRunAtThisScalar)
        );
        // The shell door's absence is the same fact one step earlier:
        // the call is never formed at all, so there is no refusal to
        // read and nothing validated the caller could mistake for one.
        assert!(
            <geom_core::Dual64 as AtRestPolicy>::shell_door().is_none(),
            "a dual may not certify, so it holds no shell door"
        );
        assert!(
            <geom_core::Dual64 as AtRestPolicy>::region_lane().is_none(),
            "a dual may not certify, so it holds no region lane"
        );
    }
}

/// The PR 11 certified-quadrature lane's body-side plumbing: stored
/// pcurve caches → ring-bracketed [`geom_brep::props::quad::TrimEdgeQ`]s (C4's first hot
/// consumer). Key-free math stays in `geom_brep::props::quad`; this
/// module owns everything that needs half-edges and vertex points.
mod quad_lane;
pub(crate) mod settle;
pub(crate) use settle::RelOp;
pub use settle::{MassMeasurement, Settle, SettleVerdict, SignNames, Stop, Unmet};

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod recourse_tests {
    use super::*;
    use geom_brep::props::PropsError;

    /// **The recourse claim for the carrier tier 3 renders whole.**
    /// `ValidationError::VolumeUncomputable { source }` contributes no
    /// prose of its own, so whatever this enum fails to say is simply
    /// absent from the message a user reads. Three arms said nothing
    /// past the condition (`RingOnCurvedFace`, `Corrupt`,
    /// `NullScaffoldEdge`); the other two delegate, and this row is
    /// **transitive over them** — `Band` passes only while
    /// `geom_core::BandError` names a recourse and `Face` only while
    /// `geom_brep::PropsError` does, which is the assumption the
    /// wrapper makes, here made to fail loudly instead of silently.
    ///
    /// **A floor, not a proof** (the terms `topo`'s
    /// `every_chart_region_arm_names_a_recourse` states): a vocabulary
    /// check cannot tell a recourse from a sentence containing a verb.
    /// Payloads below carry no verb of their own.
    #[test]
    fn every_mass_props_error_arm_names_a_recourse() {
        const RECOURSE_VERBS: &[&str] = &[
            "lower", "raise", "name", "classify", "state", "declare", "move", "loosen", "simplify",
            "report", "re-cut", "re-mint", "repair", "finish", "revert", "read",
        ];
        let arms = [
            MassPropsError::Band {
                error: BandError::Empty {
                    zero: 1e-8,
                    escalate: 1e-9,
                },
            },
            MassPropsError::Face {
                face: FaceKey::default(),
                source: PropsError::NappeSpanning,
            },
            MassPropsError::RingOnCurvedFace {
                face: FaceKey::default(),
            },
            MassPropsError::Corrupt {
                what: "a loop of the face",
            },
            MassPropsError::NullScaffoldEdge {
                edge: crate::entity::EdgeKey::default(),
            },
        ];
        assert_eq!(arms.len(), 5, "an arm was added without a row here");
        for arm in &arms {
            let msg = arm.to_string();
            let lower = msg.to_lowercase();
            // Where there is no way through, the sentence says so.
            assert!(
                RECOURSE_VERBS.iter().any(|v| lower.contains(v))
                    || lower.contains("there is no way through"),
                "no recourse in: {msg}"
            );
        }
    }
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod face_list_door_tests {
    use super::*;
    use geom_core::Tol;

    /// The closed-form corpus this module's pins are taken over: the
    /// in-crate geometric prisms, alone and grafted into two-solid
    /// arenas (the census's subject), all planar so the closed form
    /// answers at every scalar.
    fn corpus() -> Vec<(&'static str, Body<f64>)> {
        use crate::splitting::reassembly::quad_prism;
        let tol = Tol::witness();
        let unit = quad_prism(&[(0.0, 0.0), (1.0, 0.0), (1.0, 1.0), (0.0, 1.0)], 1.0, tol);
        let skew = quad_prism(&[(0.0, 0.0), (2.0, 0.3), (1.7, 1.9), (-0.4, 1.2)], 0.7, tol);
        let tall = quad_prism(&[(3.0, 3.0), (3.5, 3.0), (3.5, 3.5), (3.0, 3.5)], 4.0, tol);
        let mut pair = unit.clone();
        crate::instance::graft_disjoint(&mut pair, &tall).unwrap();
        let mut trio = skew.clone();
        crate::instance::graft_disjoint(&mut trio, &tall).unwrap();
        crate::instance::graft_disjoint(&mut trio, &unit).unwrap();
        vec![
            ("unit", unit),
            ("skew", skew),
            ("tall", tall),
            ("pair", pair),
            ("trio", trio),
        ]
    }

    /// The whole-body closed-form door's bits on the corpus, recorded
    /// before the face-list door existed (base `3f2336b21`): the door
    /// became the face-list door handed the arena, and these are what
    /// say it changed no number.
    const PINNED: [(&str, u64, u64); 5] = [
        ("unit", 0x3ff0_0000_0000_0000, 0x4018_0000_0000_0000),
        ("skew", 0x4001_0d4f_df3b_6459, 0x4026_2907_4669_574e),
        ("tall", 0x3ff0_0000_0000_0000, 0x4021_0000_0000_0000),
        ("pair", 0x4000_0000_0000_0000, 0x402d_0000_0000_0000),
        ("trio", 0x4010_86a7_ef9d_b22d, 0x4039_9483_a334_aba7),
    ];

    #[test]
    fn the_whole_body_door_is_bitwise_the_face_list_door_over_the_arena() {
        let tol = Tol::witness();
        let band = Band::linear(tol).unwrap();
        for ((name, body), (pin_name, volume, area)) in corpus().into_iter().zip(PINNED) {
            assert_eq!(name, pin_name);
            let whole = mass_properties_closed_form(&body, band, tol).unwrap();
            assert_eq!(whole.volume.to_bits(), volume, "{name}: volume moved");
            assert_eq!(whole.surface_area.to_bits(), area, "{name}: area moved");
            let faces: Vec<FaceKey> = body.faces().map(|(k, _)| k).collect();
            let listed = mass_properties_closed_form_of(&body, &faces, band, tol).unwrap();
            assert_eq!(listed.volume.to_bits(), whole.volume.to_bits(), "{name}");
            assert_eq!(
                listed.surface_area.to_bits(),
                whole.surface_area.to_bits(),
                "{name}"
            );
        }
    }

    /// **The body certificate assembled from PER-SOLID parts is
    /// bitwise the whole-body read** — check 7 certifies one solid at a
    /// time and [`SignCertificate::assembled`] re-orders those parts
    /// into arena order, so the continuation has to land on
    /// [`crate::mass_properties`]'s own bits in all four fields. The
    /// corpus's `pair` and `trio` are the two- and three-solid
    /// subjects, where the re-ordering does work; the single-solid rows
    /// are the same claim where the part IS the arena.
    #[test]
    fn the_assembled_per_solid_certificate_is_bitwise_the_whole_body_read() {
        let tol = Tol::witness();
        let band = Band::linear(tol).unwrap();
        let bodies = corpus();
        assert_eq!(
            bodies
                .iter()
                .filter(|(_, b)| b.solids().count() > 1)
                .count(),
            2,
            "the claim is about several solids; the corpus must carry some"
        );
        for (name, body) in bodies {
            let parts: Vec<SignCertificate<'_, f64>> = body
                .solids()
                .map(|(solid, _)| {
                    let faces = body
                        .faces_of_solid(solid)
                        .expect("a solid the body yielded");
                    let mut part = SignCertificate::at_round_zero(
                        &body,
                        &faces,
                        band,
                        tol,
                        Some(QuadLane::certified()),
                        FaceWalk::Mapped,
                    )
                    .unwrap();
                    let positive = matches!(
                        walk_set(
                            &mut [&mut part],
                            Some(QuadLane::certified()),
                            |r: &[Round<'_, '_, f64>]| {
                                let positive = match r[0].reading() {
                                    SignReading::Exact { volume, .. } => volume > 0.0,
                                    SignReading::Bracket(e) => e.volume_lo > 0.0,
                                };
                                if positive {
                                    Ask::Decided(())
                                } else {
                                    Ask::Refine(vec![true])
                                }
                            },
                        ),
                        Walked::Decided(())
                    );
                    assert!(positive, "{name}: {solid:?} encloses positive volume");
                    part
                })
                .collect();
            let assembled =
                SignCertificate::assembled(&body, band, tol, Some(QuadLane::certified()), parts)
                    .measure(crate::test_support::pinned_resolution(tol))
                    .unwrap()
                    .met()
                    .expect("the pinned resolution is met");
            // Raw door: the measured body is a body still carrying construction scaffolding, which tier 3 refuses (ScaffoldAtRest).
            let whole = crate::test_support::measure_unvalidated(
                &body,
                tol,
                crate::test_support::pinned_resolution(tol),
            )
            .unwrap();
            assert_eq!(
                (
                    assembled.volume.to_bits(),
                    assembled.surface_area.to_bits(),
                    assembled.volume_pad.to_bits(),
                    assembled.area_pad.to_bits()
                ),
                (
                    whole.enclosure.volume.to_bits(),
                    whole.enclosure.surface_area.to_bits(),
                    whole.enclosure.volume_pad.to_bits(),
                    whole.enclosure.area_pad.to_bits()
                ),
                "{name}: the assembled certificate is not the whole-body read"
            );
        }
    }

    /// One solid's faces enclose that solid's volume, whichever other
    /// solids share the arena: the per-solid read the point-in-solid
    /// door's at-infinity fold depends on.
    #[test]
    fn a_solid_s_faces_enclose_that_solid_s_volume_in_a_shared_arena() {
        let tol = Tol::witness();
        let band = Band::linear(tol).unwrap();
        let bodies = corpus();
        let (_, pair) = &bodies[3];
        for (solid, _) in pair.solids() {
            let faces = pair
                .faces_of_solid(solid)
                .expect("a solid the body yielded");
            assert_eq!(faces.len(), 6);
            let one = mass_properties_closed_form_of(pair, &faces, band, tol).unwrap();
            // Both prisms of the pair are unit cubes.
            assert_eq!(one.volume.to_bits(), 0x3ff0_0000_0000_0000, "{solid:?}");
        }
    }
}

#[cfg(test)]
#[allow(clippy::expect_used)]
mod shell_role_refusal_tests {
    use super::*;
    use geom_core::MarginDiag;

    /// Every bracket that classifies to neither side, as `(lo, hi)`
    /// reads, and the refusal it takes, read from decided signs: any
    /// escalated end is the refusal (the high end's first, whatever its
    /// margin says), ends decided to opposite sides straddle, and
    /// otherwise an end is zero and its reporting margin rides along.
    #[test]
    fn a_bracket_refuses_by_its_decided_ends() {
        let shell = ShellKey::default();
        let band = Band::new(1e-9, 1e-8).expect("a band");
        let diag = |margin| Indeterminate {
            margin,
            band,
            predicate: Some("chk_shell_volume_sign"),
            terminal_sliver: false,
        };
        let esc = |margin| Err(diag(margin));
        let escalated = |margin| ShellClassifyError::Escalated {
            shell,
            source: diag(margin),
        };
        let v = MarginDiag::value;
        let ok = |sign, m| Ok(Decided { sign, margin: v(m) });
        let across = MarginDiag::enclosure(-2e-9, 3e-9);
        let straddles = ShellClassifyError::Straddles {
            shell,
            low: v(-1.0),
            high: v(1.0),
        };
        // The zero end's verdict rides the refusal, the low end's first.
        let zero = |m| ShellClassifyError::ZeroVolume {
            shell,
            verdict: Classified {
                margin: v(m),
                band: Band::enclosure_sign(),
            },
        };
        let rows = [
            // Closed form: one read for both ends.
            (
                "in band, outer",
                esc(v(5e-9)),
                esc(v(5e-9)),
                escalated(v(5e-9)),
            ),
            (
                "in band, void",
                esc(v(-5e-9)),
                esc(v(-5e-9)),
                escalated(v(-5e-9)),
            ),
            (
                "enclosure across zero",
                esc(across),
                esc(across),
                escalated(across),
            ),
            (
                "unreadable",
                esc(MarginDiag::INVALID),
                esc(MarginDiag::INVALID),
                escalated(MarginDiag::INVALID),
            ),
            ("zero", ok(Sign::Zero, 0.0), ok(Sign::Zero, 0.0), zero(0.0)),
            // Padded: the high end's escalation first, else the low end's.
            (
                "both ends in band",
                esc(v(-2e-9)),
                esc(v(3e-9)),
                escalated(v(3e-9)),
            ),
            (
                "high end in band",
                ok(Sign::Negative, -1.0),
                esc(v(3e-9)),
                escalated(v(3e-9)),
            ),
            (
                "low end in band",
                esc(v(-2e-9)),
                ok(Sign::Positive, 1.0),
                escalated(v(-2e-9)),
            ),
            (
                "low end in band, high zero",
                esc(v(2e-9)),
                ok(Sign::Zero, 0.0),
                escalated(v(2e-9)),
            ),
            (
                "decided straddle",
                ok(Sign::Negative, -1.0),
                ok(Sign::Positive, 1.0),
                straddles,
            ),
            (
                "zero low, high outer",
                ok(Sign::Zero, 5e-10),
                ok(Sign::Positive, 1.0),
                zero(5e-10),
            ),
            (
                "void low, zero high",
                ok(Sign::Negative, -1.0),
                ok(Sign::Zero, -5e-10),
                zero(-5e-10),
            ),
        ];
        for (name, lo, hi, want) in rows {
            assert_eq!(shell_role_refusal(shell, lo, hi), want, "{name}");
        }
    }
}

#[cfg(test)]
mod from_computed_rows {
    use super::{MassProperties, quadrature_contribution};
    use crate::revalue_pin::{Revalued, S, revalued};
    use geom_core::Real;
    use geom_core::interval::Interval;

    /// The bracket's ends take the quadrature's pad as a computed value:
    /// built at `Sym<f64>` with the volume a function of the parameter
    /// and the pad the `f64` the quadrature computed from it, neither end
    /// re-values to the pad `p0` measured.
    #[test]
    fn the_volume_bracket_takes_its_pad_as_computed() {
        let found = revalued([1.0, 1.0 + 1.0e-3], |p, x| {
            let props = MassProperties {
                volume: x * x * x,
                surface_area: S::from_f64(6.0) * x * x,
                volume_pad: 1.0e-6 * p * p,
                area_pad: 0.0,
            };
            let e = props.enclosure();
            vec![e.volume_lo, e.volume_hi]
        });
        assert_eq!(
            found,
            Revalued {
                kept: 0,
                opaque: 2,
                laundered: 0,
            },
            "each end is the volume's expression and one unknown for the pad"
        );
    }

    /// The quadrature's midpoints enter as computed values: built at
    /// `Sym<f64>` from enclosures the lane would measure off geometry
    /// the parameter moves, neither re-values to the midpoint `p0`
    /// measured.
    #[test]
    fn the_quadrature_midpoints_enter_as_computed() {
        let found = revalued([1.0, 1.0 + 1.0e-3], |p, _| {
            let x = Interval::from_f64(p);
            let (at, _, _) = quadrature_contribution::<S>(x * x * x, x * x);
            vec![at.flux, at.area]
        });
        assert_eq!(
            found,
            Revalued {
                kept: 0,
                opaque: 2,
                laundered: 0,
            }
        );
    }
}

#[cfg(test)]
mod exact_enclosure_rows {
    #![allow(clippy::expect_used, clippy::panic, clippy::unwrap_used)]
    //! The re-derivation's volume encloses the exact volume of the body's
    //! vertex polyhedron (review R2's oracle on PR 5119's second delta,
    //! `review-probes/pr5119-r2-delta2/enc_oracle.py`): every point of a
    //! line-bounded plane shell an `f64`, the polyhedron's volume a
    //! rational, summed here with no rounding at all over big integers and
    //! compared with the interval's ends exactly.

    use super::*;
    use geom_core::{Bounds, Tol};
    use num_bigint::BigInt;

    /// `x` as `(m, e)` with `x = m·2ᵉ` exactly.
    fn split(x: f64) -> (BigInt, i32) {
        if x == 0.0 {
            return (BigInt::from(0), 0);
        }
        let bits = x.to_bits();
        let sign = if bits >> 63 == 1 { -1i64 } else { 1 };
        let exp = ((bits >> 52) & 0x7ff) as i32;
        let frac = bits & ((1u64 << 52) - 1);
        let (m, e) = if exp == 0 {
            (frac, -1074)
        } else {
            (frac | (1u64 << 52), exp - 1075)
        };
        (BigInt::from(sign) * BigInt::from(m), e)
    }

    /// An exact dyadic rational `m·2ᵉ`.
    #[derive(Clone, Debug)]
    struct Dyadic(BigInt, i32);

    impl Dyadic {
        fn of(x: f64) -> Self {
            let (m, e) = split(x);
            Self(m, e)
        }
        fn mul(&self, o: &Self) -> Self {
            Self(&self.0 * &o.0, self.1 + o.1)
        }
        fn add(&self, o: &Self) -> Self {
            let e = self.1.min(o.1);
            let a = &self.0 << (self.1 - e) as usize;
            let b = &o.0 << (o.1 - e) as usize;
            Self(a + b, e)
        }
        fn neg(&self) -> Self {
            Self(-&self.0, self.1)
        }
        /// `self ≤ o`, exactly.
        fn le(&self, o: &Self) -> bool {
            let d = o.add(&self.neg());
            d.0 >= BigInt::from(0)
        }
    }

    /// Six times the exact volume of the polyhedron the rings span.
    fn six_volume(polygons: &[Vec<Vec<[f64; 3]>>]) -> Dyadic {
        let mut sum = Dyadic::of(0.0);
        for rings in polygons {
            for ring in rings {
                let r0 = ring[0];
                for pair in ring[1..].windows(2) {
                    let (q, r) = (pair[0], pair[1]);
                    for (i, j, k) in [(0, 1, 2), (1, 2, 0), (2, 0, 1)] {
                        let p = Dyadic::of(r0[i]);
                        let plus = Dyadic::of(q[j]).mul(&Dyadic::of(r[k])).mul(&p);
                        let minus = Dyadic::of(q[k]).mul(&Dyadic::of(r[j])).mul(&p);
                        sum = sum.add(&plus).add(&minus.neg());
                    }
                }
            }
        }
        sum
    }

    fn rot(axis: [f64; 3], ang: f64) -> [[f64; 3]; 3] {
        let n = (axis[0] * axis[0] + axis[1] * axis[1] + axis[2] * axis[2]).sqrt();
        let (x, y, z) = (axis[0] / n, axis[1] / n, axis[2] / n);
        let (s, c) = ang.sin_cos();
        let k = 1.0 - c;
        [
            [c + x * x * k, x * y * k - z * s, x * z * k + y * s],
            [y * x * k + z * s, c + y * y * k, y * z * k - x * s],
            [z * x * k - y * s, z * y * k + x * s, c + z * z * k],
        ]
    }

    /// A needle `1 × w × w` turned by `r` and centred at `c`.
    fn needle(w: f64, r: [[f64; 3]; 3], c: [f64; 3], tol: Tol) -> Body<f64> {
        crate::test_support_fixtures::mapped_cube::<f64>(
            move |x, y, z| {
                let p = [x, (y - 0.5) * w, (z - 0.5) * w];
                Point3::new(
                    c[0] + r[0][0] * p[0] + r[0][1] * p[1] + r[0][2] * p[2],
                    c[1] + r[1][0] * p[0] + r[1][1] * p[1] + r[1][2] * p[2],
                    c[2] + r[2][0] * p[0] + r[2][1] * p[1] + r[2][2] * p[2],
                )
            },
            tol,
        )
    }

    /// **The enclosure holds the vertex polyhedron's exact volume, and
    /// decides its sign**, on turned needles `10.5ε` and `30ε` across in
    /// four poses, at the origin, at `(37, −21, 13)` and at
    /// `(1234.5, −987.25, 3333)`, either way out.
    #[test]
    fn the_enclosure_holds_the_vertex_polyhedrons_exact_volume() {
        let tol = Tol::witness();
        let Ok(band) = Band::linear(tol) else {
            return;
        };
        let lane = QuadLane::<f64>::certified();
        let e = tol.eps();
        let poses: [([f64; 3], f64); 4] = [
            ([0.0, 0.0, 1.0], 0.0),
            ([1.0, 2.0, 3.0], 1.1),
            ([0.3, -0.7, 0.2], 2.4),
            ([1.0, 1.0, 1.0], 0.955_316_618_124_509_3),
        ];
        let centres = [
            [0.0, 0.0, 0.0],
            [37.0, -21.0, 13.0],
            [1234.5, -987.25, 3333.0],
        ];
        let mut read = 0;
        for m in [10.5, 30.0] {
            for (axis, ang) in poses {
                for c in centres {
                    let Ok(built) =
                        std::panic::catch_unwind(|| needle(m * e, rot(axis, ang), c, tol))
                    else {
                        continue;
                    };
                    for inverted in [false, true] {
                        let body = if inverted {
                            built.revert().expect("the needle reverts")
                        } else {
                            built.clone()
                        };
                        let faces: Vec<FaceKey> = body.faces().map(|(k, _)| k).collect();
                        let hook = round_hook(Some(lane));
                        let refs = References::of(
                            &body,
                            faces.iter().copied(),
                            band,
                            Some(QuadLane::certified()),
                        );
                        let runs = decide_faces_serially(&faces, |&f| {
                            face_flux(
                                &body,
                                f,
                                band,
                                &hook,
                                tol,
                                RoundWindow::SCHEDULE,
                                &refs,
                                Some(crate::test_support::pinned_len(tol)),
                            )
                        })
                        .unwrap();
                        let got = rederive(&body, band, tol, lane, &runs, true).unwrap();
                        let polygons: Vec<Vec<Vec<[f64; 3]>>> = runs
                            .iter()
                            .map(|run| {
                                let (face, surface) = resolve_face(&body, run.face);
                                vertex_rings(
                                    &body,
                                    surface,
                                    &flat_loops(&body, face).unwrap(),
                                    lane,
                                )
                                .unwrap()
                                .expect("line-bounded")
                                .iter()
                                .map(|ring| {
                                    ring.iter()
                                        .map(|p| [p.x.lo(), p.y.lo(), p.z.lo()])
                                        .collect()
                                })
                                .collect()
                            })
                            .collect();
                        let exact = six_volume(&polygons);
                        let what =
                            format!("needle {m}ε, axis {axis:?}, at {c:?}, inverted {inverted}");
                        let (lo, hi) = (got.volume.lo(), got.volume.hi());
                        let six = Dyadic::of(6.0);
                        assert!(
                            Dyadic::of(lo).mul(&six).le(&exact)
                                && exact.le(&Dyadic::of(hi).mul(&six)),
                            "{what}: [{lo:e}, {hi:e}] holds the exact volume"
                        );
                        assert!(
                            if inverted { hi < 0.0 } else { lo > 0.0 },
                            "{what}: [{lo:e}, {hi:e}] decides the sign"
                        );
                        read += 1;
                    }
                }
            }
        }
        assert!(read > 0, "a needle built");
    }
}

/// **A shell's reference moves no volume in the reals**
/// ([`shell_reference`]): a closed surface's flux sum does not depend on
/// the point it is about, so a closed-form body's volume about its
/// shells' first vertices is its volume about the world origin, to the
/// rounding of the sum.
#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
mod shell_reference_rows {
    use super::*;

    /// `body`'s volume, every face read about `about` (`None`: its
    /// shell's reference).
    fn volume_about(body: &Body<f64>, about: Option<Point3<f64>>, tol: Tol) -> f64 {
        let band = Band::linear(tol).unwrap();
        let faces = crate::query::all_faces(body);
        let hook = round_hook(Some(QuadLane::<f64>::certified()));
        let refs = References::of(
            body,
            faces.iter().copied(),
            band,
            Some(QuadLane::certified()),
        );
        let runs = decide_faces_serially(&faces, |&f| {
            let about = about.map_or_else(|| refs.at(body, f).map(ReadAbout::point), Ok)?;
            face_flux_about(
                body,
                f,
                band,
                &hook,
                tol,
                RoundWindow::SCHEDULE,
                about,
                Some(crate::test_support::pinned_len(tol)),
            )
        })
        .unwrap();
        assert!(
            runs.iter().all(|r| r.contribution.enclosure.is_none()),
            "every face of the row's bodies has a closed form"
        );
        fold_runs(&runs).volume
    }

    /// Closed-form bodies: every face of each reads a closed form.
    fn closed_form_corpus(tol: Tol) -> Vec<(&'static str, Body<f64>)> {
        let notch = [(0.0, 0.0), (4.0, 0.0), (4.0, 2.0), (2.0, 1.0), (0.0, 2.0)];
        let disc = {
            let p = crate::test_support::prism::<f64>(&notch, 1.0, tol);
            let mut body = p.body;
            let outer = body.get_face(p.top_face).unwrap().outer;
            let LoopBoundary::Cycle { first } = body.get_loop(outer).unwrap().boundary else {
                panic!("the top's outer loop is a cycle");
            };
            crate::test_support::plant_disc_face(
                &mut body,
                first,
                Point3::new(1.0, 0.6, 1.0),
                0.4,
                tol,
            );
            body
        };
        vec![
            (
                "unit brick",
                crate::test_support::brick::<f64>((0.0, 1.0), (0.0, 1.0), (0.0, 1.0), tol),
            ),
            (
                "brick off the axes",
                crate::test_support::brick::<f64>((0.3, 1.7), (-0.9, 0.1), (0.2, 2.9), tol),
            ),
            (
                "notch prism",
                crate::test_support::prism::<f64>(&notch, 1.0, tol).body,
            ),
            ("notch prism with a disc", disc),
            (
                "holed block",
                crate::test_support::holed_block::<f64>(4.0, &[1.0, 3.0], tol),
            ),
        ]
    }

    #[test]
    fn a_closed_form_bodys_volume_is_its_volume_about_the_world_origin() {
        let tol = Tol::witness();
        let bodies = closed_form_corpus(tol);
        for (name, body) in &bodies {
            let (new, old) = (
                volume_about(body, None, tol),
                volume_about(body, Some(Point3::origin()), tol),
            );
            assert!(
                (new - old).abs() <= 4.0 * f64::EPSILON * old.abs(),
                "{name}: about the reference {new:e}, about the origin {old:e}"
            );
        }
    }

    /// **A shell with no quadrature face is read about its first face's
    /// start**, the point every face was read about before the reference
    /// moved to the first quadrature face: the reference is that vertex,
    /// and the volume and area read the same bits as a walk about it.
    #[test]
    fn a_shell_of_no_quadrature_face_reads_about_its_first_faces_start() {
        let tol = Tol::witness();
        for (name, body) in &closed_form_corpus(tol) {
            for (key, shell) in body.shells() {
                let first = body.get_face(shell.faces[0]).unwrap();
                let start = match body.get_loop(first.outer).unwrap().boundary {
                    LoopBoundary::Empty { vertex } => vertex,
                    LoopBoundary::Cycle { first } => body.get_half_edge(first).unwrap().start,
                };
                let want = body.vertex_points().find(|(v, _)| *v == start).unwrap().1;
                let got = shell_reference(body, key);
                assert_eq!(
                    [got.x, got.y, got.z].map(f64::to_bits),
                    [want.x, want.y, want.z].map(f64::to_bits),
                    "{name}: the reference is the first face's start"
                );
            }
            let first_start = |f: FaceKey| {
                let shell = body.get_face(f).unwrap().shell;
                let first = body
                    .get_face(body.get_shell(shell).unwrap().faces[0])
                    .unwrap();
                let start = match body.get_loop(first.outer).unwrap().boundary {
                    LoopBoundary::Empty { vertex } => vertex,
                    LoopBoundary::Cycle { first } => body.get_half_edge(first).unwrap().start,
                };
                body.vertex_points().find(|(v, _)| *v == start).unwrap().1
            };
            let band = Band::linear(tol).unwrap();
            let faces = crate::query::all_faces(body);
            let hook = round_hook(Some(QuadLane::<f64>::certified()));
            let runs = decide_faces_serially(&faces, |&f| {
                face_flux_about(
                    body,
                    f,
                    band,
                    &hook,
                    tol,
                    RoundWindow::SCHEDULE,
                    first_start(f),
                    Some(crate::test_support::pinned_len(tol)),
                )
            })
            .unwrap();
            let about_first = fold_runs(&runs);
            let walked = match crate::AtRestBody::validate(body.clone(), tol) {
                Ok(finished) => {
                    mass_properties(&finished, tol, crate::test_support::pinned_resolution(tol))
                }
                Err(errors) => {
                    assert!(
                        errors.iter().all(|e| matches!(
                            e,
                            ValidationError::ScaffoldAtRest { .. }
                                | ValidationError::LoopRoleInverted { .. }
                        )),
                        "{name}: tier 3 refuses the body for more than its scaffolding: {errors:?}"
                    );
                    // Raw door: a body still carrying construction scaffolding or a loop flipped against its role, which tier 3 refuses (ScaffoldAtRest, LoopRoleInverted), as the assert above admits.
                    crate::test_support::measure_unvalidated(
                        body,
                        tol,
                        crate::test_support::pinned_resolution(tol),
                    )
                }
            }
            .unwrap();
            assert_eq!(
                (
                    walked.enclosure.volume.to_bits(),
                    walked.enclosure.surface_area.to_bits()
                ),
                (
                    about_first.volume.to_bits(),
                    about_first.surface_area.to_bits()
                ),
                "{name}: the walk reads the bits it reads about the first face's start"
            );
        }
    }

    static REFUSED: core::sync::atomic::AtomicUsize = core::sync::atomic::AtomicUsize::new(0);

    /// `quad_lane::closed_form`, refusing an up-facing plane read about
    /// any point but the world origin, as a closed form refuses where its
    /// inventory reads in band about the shell's reference.
    fn refusing_up_planes(
        surface: &Surface<f64>,
        loops: &[Vec<LoopEdge<f64>>],
        sense: bool,
        band: Band,
        centre: Point3<Interval>,
        measured: Option<((Interval, Interval), Point3<Interval>)>,
    ) -> Result<(FaceContribution<Interval>, bool), PropsError> {
        use geom_core::Bounds;
        let up = matches!(surface, Surface::Plane { normal, .. } if normal.z > 0.5);
        let off = [centre.x, centre.y, centre.z]
            .iter()
            .any(|x| x.lo() != 0.0 || x.hi() != 0.0);
        if up && off && measured.is_none() {
            REFUSED.fetch_add(1, core::sync::atomic::Ordering::Relaxed);
            return Err(PropsError::QuadratureUnsupported {
                what: "the row's planted refusal about the shell's reference",
            });
        }
        quad_lane::closed_form(surface, loops, sense, band, centre, measured)
    }

    /// **A closed-form face that refuses about its shell's reference is
    /// carried there from the world origin** ([`closed_form_or_carried`],
    /// `rederive_faces`' closed-form read). The
    /// notch prism with a disc in its top, moved off the origin so its
    /// reference is too, keeps its faces off the polygon route; a lane
    /// whose closed form refuses its two up-facing planes about the
    /// reference reads those two about the origin and carries them by
    /// their vector area `(0, 0, A)`, `c·A⃗` with `c` on the top, 3 m up.
    /// Summed without the carry the volume reads `c_z·A/3 = 6 m³` over:
    /// 12 m³ for the prism's 6, measured.
    #[test]
    fn a_closed_form_face_refusing_about_its_reference_is_carried_from_the_origin() {
        let tol = Tol::witness();
        let band = Band::linear(tol).unwrap();
        let notch = [(0.0, 0.0), (4.0, 0.0), (4.0, 2.0), (2.0, 1.0), (0.0, 2.0)];
        let p = crate::test_support::prism::<f64>(&notch, 1.0, tol);
        let mut body = p.body;
        let outer = body.get_face(p.top_face).unwrap().outer;
        let LoopBoundary::Cycle { first } = body.get_loop(outer).unwrap().boundary else {
            panic!("the top's outer loop is a cycle");
        };
        crate::test_support::plant_disc_face(
            &mut body,
            first,
            Point3::new(1.0, 0.6, 1.0),
            0.4,
            tol,
        );
        let body = crate::transform_rigid(
            &body,
            &geom_core::RigidMotion::translation(geom_core::Vec3::new(5.0, -3.0, 2.0)).unwrap(),
            tol,
        )
        .unwrap();
        let faces = crate::query::all_faces(&body);
        let lane = QuadLane::<f64>::certified();
        let hook = round_hook(Some(lane));
        let refs = References::of(
            &body,
            faces.iter().copied(),
            band,
            Some(QuadLane::certified()),
        );
        let runs = decide_faces_serially(&faces, |&f| {
            face_flux(
                &body,
                f,
                band,
                &hook,
                tol,
                RoundWindow::SCHEDULE,
                &refs,
                Some(crate::test_support::pinned_len(tol)),
            )
        })
        .unwrap();
        assert!(
            shell_polygons(&body, lane, &runs).unwrap().is_none(),
            "the disc keeps every face off the polygon route"
        );
        REFUSED.store(0, core::sync::atomic::Ordering::Relaxed);
        let mut flux = Interval::zero();
        let mut carried = 0;
        for run in &runs {
            let (face, surface) = resolve_face(&body, run.face);
            let loops = face_loops(&body, face).unwrap();
            let from = corner_of(lane, run);
            let (c, moved) =
                closed_form_or_carried(refusing_up_planes, surface, &loops, face.sense, band, from)
                    .unwrap();
            carried += usize::from(!moved);
            flux = flux + c.flux;
        }
        assert_eq!(
            REFUSED.load(core::sync::atomic::Ordering::Relaxed),
            2,
            "the top and the disc refuse about the reference"
        );
        assert_eq!(carried, 2, "two faces were carried from the origin");
        use geom_core::Bounds;
        let volume = flux / Interval::from_f64(3.0);
        assert!(
            volume.lo() <= 6.0 && 6.0 <= volume.hi(),
            "the carried sum encloses the prism's 6 m³: {volume:?}"
        );
    }
}
