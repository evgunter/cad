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
//! [`sign_walk`], which is the reporting walk ([`mass_properties_with`])
//! stopped at the round the validator's own decision is complete, and
//! hands back the [`SignCertificate`] a caller continues to the number.

use core::fmt;

use geom::Surface;
use geom_brep::props::quad::{FaceCutBounds, RoundOutcome, RoundWindow};
use geom_brep::props::{
    CarrierId, FaceContribution, LoopEdge, PropsError, curved_face, planar_face,
};
use geom_core::k_stats::Detached;
use geom_core::{Band, BandError, Decide, Indeterminate, Margin, Real, Sign, Tol};
use slotmap::Key;

use crate::body::Body;
use crate::boolean::ContactRecords;
use crate::entity::{FaceKey, HalfEdgeKey, LoopBoundary, LoopKey, ShellKey, SolidKey, VertexKey};
use crate::shell::{ShellError, Shelled};
use crate::validate::ValidationError;

/// Exact-B-rep integral properties of a body.
///
/// Closed-form faces contribute exact values; curved-CUT faces (M5
/// PR 11) contribute **certified quadrature enclosures**, carried as
/// the enclosure midpoint in `volume`/`surface_area` plus the summed
/// half-widths in the `*_pad` fields — so the certified brackets are
/// `volume ± volume_pad` and `surface_area ± area_pad`. Both pads are
/// exactly `0.0` for bodies whose every face has a closed form (the
/// pre-PR-11 behaviour, bit-identical).
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
    /// Certified half-width of the volume bracket (m³) — the summed
    /// quadrature enclosure half-widths; `0.0` when every face is
    /// closed-form. The tier-3 +V backstop consumes this.
    pub volume_pad: f64,
    /// Certified half-width of the area bracket (m²).
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
    /// **The band absorbs it, with the numbers.** The reconstruction
    /// error is at most a few ulps of the midpoint's magnitude, so at
    /// most `|volume|·2⁻⁵²`. The passing exit requires
    /// `volume_lo / surface_area ≥ K·ε` (the band's escalation
    /// threshold), and `volume ≤ volume_lo + 2·volume_pad`, so the
    /// error can only reach that threshold on a body whose bracket
    /// half-width, metered on the same lever, is at least `K·ε·2⁵¹` —
    /// of order `10⁷` metres of mean boundary displacement at
    /// `ε = 10⁻⁹`. A face that wide refuses at
    /// `props_quad_face_extent` or carries a poisoned enclosure long
    /// before it reaches here. (The same shape of argument, with the
    /// same conclusion, is written out at `geom-brep`'s
    /// `last_round_width_lo`.)
    ///
    /// The UPPER end's arithmetic is untouched and deliberately so:
    /// it is the end the +V refusal has always read, its rounding is
    /// the rounding that refusal has always carried, and nudging it
    /// would move a margin the reporting door's telemetry pins.
    #[must_use]
    pub fn enclosure(&self) -> VolumeEnclosure<T> {
        VolumeEnclosure {
            volume_lo: self.volume - T::from_f64(self.volume_pad),
            volume_hi: self.volume + T::from_f64(self.volume_pad),
            surface_area: self.surface_area,
        }
    }
}

/// **A volume BRACKET and its lever** — what a certified quadrature
/// is entitled to say before it has met the reporting target.
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
    /// A curved face carries interior rings — no M2 construction
    /// produces one (curved patches are swept UV rectangles).
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
}

impl fmt::Display for MassPropsError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Band { error } => write!(f, "mass properties: {error}"),
            Self::Face { face, source } => {
                write!(f, "mass properties: face {face:?}: {source}")
            }
            Self::RingOnCurvedFace { face } => {
                write!(
                    f,
                    "mass properties: curved face {face:?} carries interior rings — curved \
                     patches are swept UV rectangles and no construction produces one, so \
                     report this rather than repairing a body"
                )
            }
            Self::Corrupt { what } => {
                write!(
                    f,
                    "mass properties: corrupt body ({what}) — the structural validators own \
                     this diagnosis: read the tier-1/tier-2 report and repair what it names"
                )
            }
            Self::NullScaffoldEdge { edge } => {
                write!(
                    f,
                    "mass properties: edge {edge:?} is null-edge scaffolding \
                     (mid-surgery body; tier 2 refuses null entities at rest) — finish or \
                     revert the surgery and ask again at rest"
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
/// face, rings on a curved face, or unresolvable structure.
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
/// use topo::{Body, mass_properties_structural};
/// fn measure(b: &Body<Dual64>, tol: Tol) {
///     let _ = mass_properties_structural(b, tol);
/// }
/// ```
///
/// ```compile_fail,E0277
/// use geom_core::{Dual64, Tol};
/// use topo::{Body, mass_properties};
/// fn measure(b: &Body<Dual64>, tol: Tol) {
///     let _ = mass_properties(b, tol);
/// }
/// ```
///
/// The two rows differ in one identifier and every path either names
/// resolves, so the second fails on the bound alone — `E0277`,
/// `CertifiedEnclosure` not implemented for `Dual<f64>`.
pub fn mass_properties<T: Decide + geom_core::CertifiedBounds>(
    body: &Body<T>,
    tol: Tol,
) -> Result<MassProperties<T>, MassPropsError> {
    let band = Band::linear(tol).map_err(|error| MassPropsError::Band { error })?;
    mass_properties_with(body, band, tol, Some(QuadLane::certified()))
}

/// [`mass_properties`] holding NO quadrature lane — the closed-form
/// walk, at the certified door's own signature and at plain
/// `T: Decide`: no bracket is read anywhere on this path, so no bracket
/// term is spelled. It IS `mass_properties_closed_form` with the band
/// built inside from `tol`, the one public door onto that walk (the
/// band-outside entry is `pub(crate)`, for the boolean engine's
/// backstops, which hold a band already). Every closed-form face
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
    body: &Body<T>,
    tol: Tol,
) -> Result<MassProperties<T>, MassPropsError> {
    let band = Band::linear(tol).map_err(|error| MassPropsError::Band { error })?;
    mass_properties_closed_form(body, band, tol)
}

/// [`mass_properties`] against a caller-built band and the caller's
/// quadrature lane, over the whole face arena in arena order — the
/// reporting walk, run to the target in one entry per face.
pub(crate) fn mass_properties_with<T: Decide>(
    body: &Body<T>,
    band: Band,
    tol: Tol,
    quad: Option<QuadLane<T>>,
) -> Result<MassProperties<T>, MassPropsError> {
    let faces = crate::query::all_faces(body);
    mass_properties_impl(body, &faces, band, &reporting_hook(quad), tol)
}

/// **The hook at the REPORTING level, one home**: the lane the caller
/// holds, entered once over the whole schedule and read at the target
/// ([`RoundOutcome::into_target`]), its answer always `Converged`
/// because a reporting read has no window to stop in — and `Ok(None)`,
/// no lane and not attempted, when the caller holds none. One function
/// rather than a closure at each site: the whole-body walk
/// ([`mass_properties_with`]) and the per-shell walk
/// ([`classify_shells_of`]) read at the same level through the same
/// hook, and two copies of that are two things to keep equal. The
/// closure captures a `Copy` of the lane and nothing else, so it is
/// `Sync` as [`QuadHook`] needs.
#[allow(clippy::type_complexity)]
fn reporting_hook<T: Decide>(
    quad: Option<QuadLane<T>>,
) -> impl Fn(
    &Body<T>,
    &Surface<T>,
    &[LoopEdge<T>],
    &[HalfEdgeKey],
    Band,
    Tol,
    RoundWindow,
) -> Result<Option<RoundOutcome>, PropsError>
+ Sync {
    move |body, surface, outer, hes, band, tol, _window| match quad {
        Some(lane) => lane
            .cut_face(body, surface, outer, hes, band, tol)
            .map(|bounds| Some(RoundOutcome::Converged(bounds))),
        None => Ok(None),
    }
}

/// **The hook at SIGN level, one home**: the lane the caller holds,
/// entered over exactly the [`RoundWindow`] the walk asks for, so a
/// walk may stop between rounds and resume later — and `Ok(None)`, the
/// closed form, when the caller holds none. [`reporting_hook`]'s
/// windowed twin: the same lane, entered at two levels.
///
/// With [`QuadLane::certified`] this is the certified quadrature, round
/// by round; with `None` every face is the closed form's, finished at
/// round 0, and a face that needed the quadrature refuses typed there
/// ([`face_flux`]'s `None` arm) — so a walk through this hook refuses
/// exactly where the reporting walk over the same lane does.
#[allow(clippy::type_complexity)]
fn round_hook<T: Decide>(
    quad: Option<QuadLane<T>>,
) -> impl Fn(
    &Body<T>,
    &Surface<T>,
    &[LoopEdge<T>],
    &[HalfEdgeKey],
    Band,
    Tol,
    RoundWindow,
) -> Result<Option<RoundOutcome>, PropsError>
+ Sync {
    move |body, surface, outer, hes, band, tol, window| match quad {
        Some(lane) => lane
            .cut_face_rounds(body, surface, outer, hes, band, tol, window)
            .map(Some),
        None => Ok(None),
    }
}

/// **The face walk at SIGN level** — the caller's lane, refined only
/// as far as the caller's own decision needs and no further.
///
/// The lane is an argument, exactly as it is to [`mass_properties_with`]:
/// [`QuadLane::certified`] from a door whose bound names the right —
/// which is how [`crate::validate_geometric`] and the certified tier-3′
/// doors enter it — or `None` from a `_structural` door, where every
/// face is the closed form's and one that needed the quadrature refuses
/// typed. A scalar that may not certify cannot construct the first, so
/// what this walk can claim is fixed by what the caller could hand it,
/// and the certificate it returns carries the same lane on to
/// [`SignCertificate::refine_to_target`].
///
/// `settled` is handed the body's running volume enclosure after every
/// round and answers whether what IT is deciding is decided. The walk
/// runs round 0 for every face, sums, asks; if the answer is no it
/// refines every still-open face by one round, sums, asks again; and
/// it stops at the first round `settled` accepts or at the reporting
/// target, whichever comes first.
///
/// **The order, stated, because the bits depend on it.** Faces are
/// visited in arena order within every round, and the sum accumulates
/// in that order — so a face's enclosure at round `r` is the
/// enclosure the reporting walk computes at its round `r`
/// (the lanes' rounds are independent recomputations, so a window
/// changes no arithmetic), and a walk that reaches the target
/// accumulates the same terms in the same order. A certificate
/// continued to the target with [`SignCertificate::refine_to_target`]
/// is therefore bit-identical to [`mass_properties`] on the
/// same body, band and `tol`, and pays the same piece evaluations.
///
/// **A face with no enclosure at all stops the walk**: poison, a
/// degenerate lever and an escalated funnel decision leave the sum
/// undefined, so they refuse here exactly as they refuse the
/// reporting door, naming the first such face in arena order at the
/// round it happens. A face that refuses on BUDGET is different — it
/// has an enclosure, and the sum keeps it — so the refusal rides on
/// the certificate, and whether it is REPORTED is the caller's
/// decision, taken by `last_word` below.
///
/// **The verdict is decided once, inside the loop.** `settle` is the
/// only reading of the enclosure, and the round it accepts is the
/// round the certificate stops at; `last_word` is asked exactly when
/// `settle` never accepted — the schedule ran to the reporting target
/// or every face ran out of rounds — and is handed the outstanding
/// target refusal, which is the whole of what distinguishes "the sign
/// is undecided and the body IS measurable" from "the sign is
/// undecided and the quadrature ran out". A verdict comes back
/// unconditionally, so there is no undecided state for a caller to
/// forget: the type has no arm for one.
///
/// **The subject is `faces`, and it is an argument.** An enclosure is
/// a claim about whatever its faces bound, so a caller asking whether
/// ONE solid's volume is positive, or whether one shell bounds
/// material or a cavity, is asking a question no other solid's faces
/// enter — [`classify_shells_of`]'s argument, made at SIGN level
/// instead of at the reporting one. Handing the face arena in arena
/// order is the whole-body walk, term for term and round for round;
/// the restriction is which faces are visited and nothing else.
///
/// **This family is a single door where two of its neighbours are
/// pairs** — [`mass_properties_closed_form`]/[`mass_properties_closed_form_of`]
/// and [`classify_shells`]/[`classify_shells_of`] each keep a
/// whole-body wrapper beside the restricted spelling, and this one has
/// none. That is deliberate: every caller decides per solid, so a
/// wrapper handing [`crate::query::all_faces`] would be dead code
/// carrying a whole-body claim nothing exercises.
///
/// # Errors
///
/// [`MassPropsError`], as [`mass_properties`].
// Seven parameters, and each is one the caller must state: the subject
// (`body`, `faces`), the level it is read at (`band`, `tol`), the lane
// that reads it (`quad`), and the decision that stops the walk
// (`settle`, `last_word`).
#[allow(clippy::too_many_arguments)]
pub(crate) fn sign_walk<'b, T: Decide, V>(
    body: &'b Body<T>,
    faces: &[FaceKey],
    band: Band,
    tol: Tol,
    quad: Option<QuadLane<T>>,
    settle: impl Fn(VolumeEnclosure<T>) -> Option<V>,
    last_word: impl Fn(Option<MassPropsError>) -> V,
) -> Result<(V, SignCertificate<'b, T>), MassPropsError> {
    let hook = round_hook(quad);
    // Round 0 over every face, then the rounds after it over the faces
    // still open — both idiom 1 into slots in the caller's order, both
    // composed sequentially in it ([`mass_properties_impl`]'s note).
    let mut runs = decide_faces(faces, |&face_key| {
        face_flux(
            body,
            face_key,
            band,
            &hook,
            tol,
            RoundWindow::at(0),
        )
    })?;
    let mut round = 0usize;
    loop {
        let (props, refused) = fold_runs(&runs);
        let exhausted = !runs.iter().any(|r| r.open_at == Some(round));
        let verdict = match settle(props.enclosure()) {
            Some(verdict) => Some(verdict),
            // The schedule has nothing further to offer and `settle`
            // did not accept: the caller says what that means, with
            // the refusal a target-level reading of this body earns
            // in hand.
            None if exhausted => Some(last_word(refused.as_ref().map(|(face, source)| {
                MassPropsError::Face {
                    face: *face,
                    source: source.clone(),
                }
            }))),
            None => None,
        };
        if let Some(verdict) = verdict {
            return Ok((
                verdict,
                SignCertificate {
                    body,
                    band,
                    tol,
                    quad,
                    runs,
                    refused,
                },
            ));
        }
        round += 1;
        // The open faces, in arena order — the same subsequence the
        // serial walk re-entered, so the recordings splice back in the
        // order it made them. A face that is not open contributes no
        // decision this round, exactly as before.
        let open: Vec<(usize, FaceKey)> = runs
            .iter()
            .enumerate()
            .filter(|(_, run)| run.open_at == Some(round - 1))
            .map(|(slot, run)| (slot, run.face))
            .collect();
        let decided = decide_faces(&open, |&(_, face_key)| {
            face_flux(
                body,
                face_key,
                band,
                &hook,
                tol,
                RoundWindow::at(round),
            )
        })?;
        for ((slot, _), run) in open.iter().zip(decided) {
            runs[*slot] = run;
        }
    }
}

/// **A volume certificate at SIGN level** — the enclosure a tier-3
/// gate decided its +V invariant on, and the schedule it left
/// unfinished.
///
/// The level is the TYPE (D9 row 0): there is no volume number to
/// read here, because a quadrature stopped at the round its caller's
/// certification was complete has not computed one. A caller that
/// wants the number asks for it — [`Self::refine_to_target`] — and
/// pays only the rounds that were not already run.
///
/// # The refusal rule, stated once
///
/// **A reading of this certificate names the FIRST refusing face in
/// arena order, resumed or already outstanding**, because that is the
/// reporting walk's own rule and the two doors must agree. The
/// reporting walk visits faces in arena order and stops at the first
/// one whose lane refuses; the sign-level walk cannot, because it
/// needs every face's enclosure to have a sum at all, so it carries
/// budget refusals as data and keeps going. A continuation that folded
/// those at the end would let a later face's HARD refusal (a
/// degenerate lever, an unsupported chart, a poisoned bracket)
/// pre-empt an earlier face's budget one, which is a different answer
/// and not merely a different order. Every site below points here
/// rather than restating it.
///
/// # The lane rides with it
///
/// A certificate is continued through the lane it was derived through
/// — [`QuadLane::certified`] from a certified door, `None` from a
/// `_structural` one — so [`Self::refine_to_target`] reads the same
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
    refused: Option<(FaceKey, PropsError)>,
}

impl<T: Decide> fmt::Debug for SignCertificate<'_, T> {
    /// The certificate, not the body it reads: the bracket, the rounds
    /// its faces reached, and whether a number is still refused.
    ///
    /// # Why this does not render in braced struct shape
    ///
    /// **Not one of the four things below is a field of this type, and
    /// not one of this type's six fields is rendered under its own
    /// name.** The bracket and the surface area are folded out of
    /// `runs`, the open round is a maximum over a field of `FaceRun`,
    /// and the refusal comes through [`Self::target_refusal`]. So the
    /// question a braced shape raises — what happens to this render
    /// when a field is added — has no useful answer: `Type { a: …, b:
    /// … }` is what `derive(Debug)` and `debug_struct(…).finish()`
    /// emit, and `finish_non_exhaustive` exists to say when such a
    /// dump is partial, so the braces tell a reader these ARE the
    /// fields. That is already false of every element here, and a
    /// seventh field could not make it any falser. The braces are what
    /// goes, and a reading of the certificate is what this says it is.
    ///
    /// # The correspondence that IS here, and is tied
    ///
    /// An earlier draft of this comment said there was "no
    /// correspondence to be short of". **That was false and a style
    /// review executed it.** [`Self::enclosure`] returns a
    /// [`VolumeEnclosure`], which declares exactly three fields, and
    /// all three are rendered below under their own names and nothing
    /// else of it is. So the render is a field list — that type's —
    /// and a fourth field on it compiled clean while this comment
    /// argued no such list existed.
    ///
    /// Both patterns below are the tie. [`VolumeEnclosure`] is
    /// destructured for the same reason `Self` is: a field added to
    /// either is an E0027 here and has to be given a rendering or a
    /// reason. What stays untied is `FaceRun::open_at`, read through
    /// `runs.iter().filter_map(…)` — a field reached through an
    /// iterator adaptor, which no pattern here can bind and which
    /// `componentwise-equality-of-the-linear-types-is-hand-listed`'s
    /// sibling question covers.
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let Self {
            // The body is what the certificate READS; rendering it
            // here would be a dump of the model, not of this.
            body: _,
            // The bracket below is the answer these two settled; the
            // settings themselves are the caller's, not the
            // certificate's.
            band: _,
            tol: _,
            // The lane the continuation reads with — the door's, not
            // a property of the enclosure below.
            quad: _,
            runs,
            // Rendered through `Self::target_refusal`, which is where
            // the rule for reading it — first refusing face in arena
            // order — is stated.
            refused: _,
        } = self;
        let VolumeEnclosure {
            volume_lo,
            volume_hi,
            surface_area,
        } = self.enclosure();
        write!(
            f,
            "SignCertificate: volume in [{:?}, {:?}], surface area {:?}, \
             rounds still open {:?}, target refusal {:?}",
            volume_lo,
            volume_hi,
            surface_area,
            // The rounds that REMAIN, not the rounds run: `None` here
            // is a finished walk (every face converged, exhausted its
            // schedule, or is closed-form), which is what a certificate
            // stopped at round 0 looks like and must not read as "no
            // rounds".
            runs.iter().filter_map(|r| r.open_at).max(),
            self.target_refusal(),
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
    /// face in it, and [`Self::refine_to_target`]'s bit-identity with
    /// [`mass_properties`] is identity of the same terms summed in it.
    /// So the parts are re-ordered rather than concatenated, and a
    /// single part covering the whole arena — which is what a body
    /// holding one solid hands here — comes back unchanged, run for
    /// run and round for round.
    ///
    /// `quad` is the lane every part was derived through, and the one
    /// the assembled certificate continues with: the parts of one check
    /// are one door's walks, so they share it by construction.
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
        let refused = fold_runs(&runs).1;
        Self {
            body,
            band,
            tol,
            quad,
            runs,
            refused,
        }
    }

    /// The certified volume bracket and its lever, at the round the
    /// walk stopped on.
    #[must_use]
    pub fn enclosure(&self) -> VolumeEnclosure<T> {
        fold_runs(&self.runs).0.enclosure()
    }

    /// **The number, continued from here** — the same quadrature run
    /// on to the reporting target, reusing every round already taken.
    ///
    /// The `Ok` result is bit-identical to [`crate::mass_properties`]
    /// on the same body at the same `tol`, and its piece evaluations
    /// are the same evaluations: a face left open at round `k` resumes
    /// at `k + 1`, a face that already met the target is not touched,
    /// and the sum is over the same terms in the same arena order.
    ///
    /// **The `Err` is the same refusal too** — [`SignCertificate`]'s
    /// refusal rule, which this walk keeps whatever width it runs at:
    /// the open faces are resumed by an indexed parallel map into one
    /// slot per face in arena order (D9's addendum idiom 1, through
    /// [`map_faces_detached`], each resumption under a K-funnel frame
    /// of its own) and the walk over those slots is sequential in that
    /// order (idiom 2). So the verdict log, the escalation log and the
    /// `probe` sample population are the serial continuation's at any
    /// thread count: every recording up to and including the refusing
    /// face spliced in order, every one after it dropped with the
    /// resumption it came with. Under a symbolic session the
    /// continuation is the SERIAL walk instead, for
    /// [`decide_faces_serially`]'s reason.
    ///
    /// **What the failure path costs**: the rounds of the open faces
    /// between the first one and the refusing one, run and thrown
    /// away. It is bounded by the faces a certificate leaves open
    /// BEFORE its first outstanding refusal — the map never reaches
    /// past that face, because the walk does not — so it is the
    /// latency of a REFUSAL and never of an answer.
    ///
    /// **What the continuation costs against one measurement.** The
    /// piece evaluations compose exactly, but each entry into a face's
    /// lane re-derives that face's round-independent SETUP — the
    /// derivative grids, the block hulls, the last round's cut lists
    /// and the bound read off them — because a `RoundWindow` enters the
    /// lane at its front door. **What that costs depends on whether the
    /// sign settled early**, which is a property of the body rather
    /// than of this call: a certificate whose `settle` ran the schedule
    /// to its end leaves no face open, so the continuation re-enters no
    /// lane at all and pays neither the setup nor a pool round trip; a
    /// certificate that stopped part-way pays the setup again for every
    /// face it left open. The map above divides that term by the pool
    /// width; removing it is `work/perf/`'s
    /// `quadrature-setup-is-re-derived-per-round-window`, which carries
    /// the measurements.
    ///
    /// # Errors
    ///
    /// [`MassPropsError`], as [`crate::mass_properties`].
    pub fn refine_to_target(self) -> Result<MassProperties<T>, MassPropsError> {
        let Self {
            body,
            band,
            tol,
            quad,
            mut runs,
            refused,
        } = self;
        let hook = round_hook(quad);
        let resume = |face, round: usize| {
            face_flux(
                body,
                face,
                band,
                &hook,
                tol,
                RoundWindow {
                    first: round + 1,
                    last: usize::MAX,
                },
            )
        };
        // **How far the walk below can get**: it returns at the first
        // face carrying an outstanding refusal, which is the face
        // `refused` names, so no slot after that one is ever reached
        // and resuming one would be pure waste. Nothing before it is
        // skipped by this bound, because a face with an outstanding
        // refusal is never OPEN — `face_flux` gives a run a round to
        // resume at only when its window ended WITHOUT one.
        let reached = refused.map_or(runs.len(), |(face, _)| {
            runs.iter().take_while(|run| run.face != face).count()
        });
        // **Idiom 1**, through the map's one home: one slot per face in
        // ARENA order, holding that face's resumption and everything
        // its lane recorded while it was taken — or nothing, for a face
        // with no round left. Empty when no reachable face is open,
        // which is the common case (most certificates settle with the
        // schedule run out) and skips the pool round trip entirely;
        // empty too under a symbolic session, where every resumption
        // below is taken on the caller's thread instead, for
        // [`decide_faces_serially`]'s reason.
        let decided: Vec<ResumedFace<T>> = if decisions_are_thread_portable()
            && runs[..reached].iter().any(|run| run.open_at.is_some())
        {
            map_faces_detached(&runs[..reached], |run| {
                run.open_at.map(|round| resume(run.face, round))
            })
        } else {
            Vec::new()
        };
        // **Idiom 2**, and the site that carries [`SignCertificate`]'s
        // refusal rule. It is NOT [`splice_in_arena_order`], and one
        // thing differs: this walk stops on a refusal a face it did not
        // resume was already CARRYING, which a fold over decided slots
        // cannot see. So the splice and that check interleave, slot by
        // slot, in arena order. A slot the map did not cover is decided
        // here instead — that is both the session arm and the tail past
        // `reached`, which the return below never gets to.
        let mut decided = decided.into_iter();
        for run in &mut runs {
            match decided.next() {
                Some((Some(resumed), recording)) => {
                    geom_core::k_stats::splice(recording);
                    *run = resumed?;
                }
                Some((None, _nothing_recorded)) => {}
                None => {
                    if let Some(round) = run.open_at {
                        *run = resume(run.face, round)?;
                    }
                }
            }
            if let Some(source) = run.refusal.clone() {
                return Err(MassPropsError::Face {
                    face: run.face,
                    source,
                });
            }
        }
        Ok(fold_runs(&runs).0)
    }

    /// The refusal a target-level reading of this body earns, if any —
    /// the first face in arena order whose quadrature has run out of
    /// schedule. A `Some` here with a definite sign above it is
    /// exactly the case this certificate exists for: the body's
    /// orientation is decided and its volume is not measurable at this
    /// ε.
    #[must_use]
    pub fn target_refusal(&self) -> Option<&PropsError> {
        self.refused.as_ref().map(|(_, e)| e)
    }
}

/// The closed-form-only walk against a caller-held band: plain
/// `T: Decide`, no quadrature lane and no bracket read — the boolean
/// engine's internal backstops (`volume_backstop`, `at_infinity_side`)
/// take it with the band they already hold, and
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
/// bit-for-bit the same (`face_list_door_tests` pins it); the
/// point-in-solid door's per-solid entry hands it one solid's faces so
/// a no-hit ray reads THAT solid's at-infinity side and not the
/// body's total.
pub(crate) fn mass_properties_closed_form_of<T: Decide>(
    body: &Body<T>,
    faces: &[FaceKey],
    band: Band,
    tol: Tol,
) -> Result<MassProperties<T>, MassPropsError> {
    mass_properties_impl(body, faces, band, &|_, _, _, _, _, _, _| Ok(None), tol)
}

/// The per-face certified-quadrature hook: `Ok(None)` = no lane / not
/// attempted (the closed form then answers, refusing typed on trimmed
/// faces), `Ok(Some(outcome))` = the lane's answer over the window it
/// was handed.
type QuadHook<'h, T> = dyn Fn(
        &Body<T>,
        &Surface<T>,
        &[LoopEdge<T>],
        &[HalfEdgeKey],
        Band,
        Tol,
        RoundWindow,
    ) -> Result<Option<RoundOutcome>, PropsError>
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

/// **One slot of a continuation's map** — [`FaceDecision`] for a walk
/// whose items may decide NOTHING: a face with no round left to resume
/// at answers `None` and records nothing, and its slot is skipped by
/// [`SignCertificate::refine_to_target`]'s sequential half.
type ResumedFace<T> = (Option<Result<FaceRun<T>, MassPropsError>>, Detached);

/// **Whether a face may be decided on a worker thread at all.**
///
/// The K-funnel is not the only thread-local a decision writes, and the
/// others cannot be composed back. A symbolic session
/// (`geom_core::sym`) is per-CALL and thread-local, and three things
/// follow from that, only the first of which is about recording:
///
/// - **The decision itself changes.** `Sym`'s `sign_within` consults
///   the session; with none installed `discharge` answers `None`, the
///   identity tier discharges nothing and the answer is the plain
///   numeric one. A face on a worker would decide differently from its
///   siblings on the caller's thread — the one thing D9 forbids
///   outright.
/// - **The receipt is written in place.** `count_decision` and
///   `count_registration_contradicted` mutate the installed session's
///   `SymCounts`, and `Sym::opaque` advances the per-replay `OPAQUE_SEQ`
///   counter — a sequence whose determinism rests on the minting ORDER
///   being a fixed single-threaded walk. Neither is a value handed
///   back, so neither can be spliced. (Node ids are NOT in this list:
///   `intern` is a content hash of the node, so the DAG a replay builds
///   is the same whatever order it is built in.)
/// - **The shape report goes with it.** `geom_core::sym::report`'s
///   `ACTIVE`/`SHAPES`/`NAMES` are thread-locals of the same family,
///   written from `Sym::sign_within` and installed by the evidence rows
///   that wrap a session.
///
/// The walk therefore stays on the caller's thread for exactly as long
/// as a session is installed, which is the driver's leaf replay
/// (`editor_core::drive` opens one per leaf and runs the whole leaf on
/// one worker, so the nesting is real and common). It is a property of
/// the CALL and not of the scalar: `Sym` with no session installed is
/// as portable as `f64`, and the check reads the session rather than
/// the type. The shape report has no query door of its own and needs
/// none — nothing installs one outside a session.
fn decisions_are_thread_portable() -> bool {
    geom_core::sym::session_counts().is_none()
}

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
/// *Not portable* ([`decisions_are_thread_portable`]): this IS the
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
    if decisions_are_thread_portable() {
        splice_in_arena_order(map_faces_detached(items, run))
    } else {
        decide_faces_serially(items, run)
    }
}

/// **Idiom 1, one home**: one slot per item in the caller's order,
/// each item decided on a worker under a K-funnel frame of ITS OWN, so
/// the sequential half can splice what it recorded back into the
/// caller's (`geom_core::k_stats::detached`). Results are written
/// positionally and never combined arithmetically, so the schedule
/// cannot reach the bits.
///
/// The map is shared; the sequential halves are not, because they
/// answer different questions — [`splice_in_arena_order`] for the face
/// walks, and [`SignCertificate::refine_to_target`]'s own, which has a
/// refusal to check on the slots it left empty.
fn map_faces_detached<I: Sync, R: Send>(
    items: &[I],
    run: impl Fn(&I) -> R + Send + Sync,
) -> Vec<(R, Detached)> {
    use rayon::prelude::*;
    items
        .par_iter()
        .map(|item| geom_core::k_stats::detached(|| run(item)))
        .collect()
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
///   PLACE ([`decisions_are_thread_portable`]), so an item decided
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
/// doors, 1 thread against 4. This one pins the rule those rows depend
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
            },
            open_at: None,
            refusal: None,
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

/// **The refusal rule, at the site that carries it.** [`SignCertificate`]
/// says a reading names the FIRST refusing face in arena order,
/// resumed or already outstanding, and
/// [`SignCertificate::refine_to_target`]'s sequential half is where
/// that survives a parallel resumption. The rows below put the two
/// kinds of refusal in the order no fixture body puts them — an
/// EARLIER face carrying an outstanding budget refusal it was never
/// resumed for, and a LATER face whose resumption refuses OUTRIGHT —
/// so a walk that folded refusals at the end, or that read the map's
/// slots before the runs it did not map, answers the later one and
/// reds here.
///
/// The certificate is built by hand because no body produces that
/// order: [`sign_walk`] leaves a face open only when its window
/// ended WITHOUT a refusal, so a real certificate's outstanding
/// refusals and its open faces never collide in one walk this way.
#[cfg(test)]
mod continuation_refusal_order_tests {
    #![allow(clippy::expect_used, clippy::panic)]

    use super::*;
    use geom_core::Point3;

    /// The outstanding refusal an earlier face carries: a target-level
    /// budget refusal, the kind a sign-level walk keeps as DATA.
    fn outstanding() -> PropsError {
        PropsError::QuadratureBudget {
            width_len: 1.0,
            target_len: 0.5,
            rounds: 1,
        }
    }

    fn run_of(face: FaceKey, open_at: Option<usize>, refusal: Option<PropsError>) -> FaceRun<f64> {
        FaceRun {
            face,
            contribution: FaceFlux {
                flux: 0.0,
                area: 0.0,
                flux_pad: 0.0,
                area_pad: 0.0,
            },
            open_at,
            refusal,
        }
    }

    /// The continuation's refusal at an explicit pool width, over a
    /// certificate whose slot 0 carries [`outstanding`] un-resumed and
    /// whose slot 1 is OPEN at a face whose resumption cannot answer at
    /// all (a skeletal `mvfs` face: no loop to flatten, so `face_flux`
    /// refuses outright rather than on budget).
    fn refusal_at(threads: usize) -> MassPropsError {
        let mut body = Body::<f64>::new();
        let skeletal = body
            .mvfs(Point3::new(0.0, 0.0, 0.0))
            .expect("the skeletal body builds")
            .face;
        let early = FaceKey::null();
        let band = Band::linear(Tol::witness()).expect("the witness band builds");
        rayon::ThreadPoolBuilder::new()
            .num_threads(threads)
            .build()
            .expect("the pool builds")
            .install(|| {
                SignCertificate {
                    body: &body,
                    band,
                    tol: Tol::witness(),
                    quad: Some(QuadLane::certified()),
                    runs: vec![
                        run_of(early, None, Some(outstanding())),
                        run_of(skeletal, Some(0), None),
                    ],
                    refused: Some((early, outstanding())),
                }
                .refine_to_target()
            })
            .expect_err("slot 0 carries a refusal, so the continuation cannot answer")
    }

    #[test]
    fn the_earlier_outstanding_refusal_is_the_one_reported_at_one_thread() {
        let err = refusal_at(1);
        assert!(
            matches!(
                err,
                MassPropsError::Face {
                    face,
                    source: PropsError::QuadratureBudget { .. }
                } if face == FaceKey::null()
            ),
            "the continuation reported the LATER face's outright refusal over the earlier \
             face's outstanding budget one: {err}"
        );
    }

    /// Non-vacuity: the LATER slot must really refuse, and refuse with
    /// something other than a budget — otherwise the rows above compare
    /// one refusal with itself and would pass over any ordering at all.
    #[test]
    fn the_later_slots_resumption_refuses_outright() {
        let mut body = Body::<f64>::new();
        let skeletal = body
            .mvfs(Point3::new(0.0, 0.0, 0.0))
            .expect("the skeletal body builds")
            .face;
        let band = Band::linear(Tol::witness()).expect("the witness band builds");
        let err = SignCertificate {
            body: &body,
            band,
            tol: Tol::witness(),
            quad: Some(QuadLane::certified()),
            runs: vec![run_of(skeletal, Some(0), None)],
            refused: None,
        }
        .refine_to_target()
        .expect_err("a skeletal face has no loop to flatten");
        assert!(
            !matches!(
                err,
                MassPropsError::Face {
                    source: PropsError::QuadratureBudget { .. },
                    ..
                }
            ),
            "the later slot refuses on BUDGET, so the rows above cannot tell the two \
             refusals apart: {err}"
        );
    }

    #[test]
    fn the_earlier_outstanding_refusal_is_the_one_reported_at_four_threads() {
        assert_eq!(
            format!("{}", refusal_at(4)),
            format!("{}", refusal_at(1)),
            "the refusal the continuation names moved with the pool width"
        );
    }
}

/// The shared face walk at the REPORTING level: every face's lane run
/// to its convergence or its typed refusal, one slot per face in arena
/// order, the accumulation order fixed (D9).
///
/// **Idiom 1 then idiom 2** (D9's parallelism addendum): the per-face
/// lanes are an indexed parallel map into arena-order slots
/// ([`decide_faces`]) and every combination after it is sequential in
/// that order — the recordings spliced by [`splice_in_arena_order`],
/// the fluxes summed by [`fold_runs`]. Nothing is combined
/// arithmetically across slots in parallel, so the schedule reaches
/// neither the bits nor the logs.
fn mass_properties_impl<T: Decide>(
    body: &Body<T>,
    faces: &[FaceKey],
    band: Band,
    quad: &QuadHook<'_, T>,
    tol: Tol,
) -> Result<MassProperties<T>, MassPropsError> {
    let runs = decide_faces(faces, |&face_key| {
        face_flux(body, face_key, band, quad, tol, RoundWindow::SCHEDULE)
    })?;
    // The refusal arm is not dead, and it is not reachable from
    // either hook this door is called with: both answer `Converged`
    // or `Ok(None)`, so no face carries one. It is what makes this
    // walk correct for the hook SIGNATURE rather than for today's two
    // hooks — a hook that answers `Open` with a refusal would be
    // silently dropping it otherwise, and the shape that drops it is
    // the shape that reads the sum of a body one of whose faces has
    // no number.
    match fold_runs(&runs) {
        (_, Some((face, source))) => Err(MassPropsError::Face { face, source }),
        (props, None) => Ok(props),
    }
}

/// The face walk's sum, and the refusal the REPORTING level owes.
///
/// **The order is the RUNS' order**, and the sum accumulates in it
/// (D9's idiom 2): this fold never re-orders, so whichever walk built
/// the slice decides the vocabulary — face-arena order for the
/// whole-body walks and the continuation, the shell's own face list
/// for [`classify_shells_of`]. The enclosure exists whenever every
/// face produced bounds at all, which a face carrying an outstanding
/// budget refusal still does. The second half names the FIRST face in
/// that same order whose lane has such a refusal outstanding: the face
/// a caller wanting a NUMBER is refused on, and the one a caller
/// deciding a SIGN may still finish without.
fn fold_runs<T: Decide>(runs: &[FaceRun<T>]) -> (MassProperties<T>, Option<(FaceKey, PropsError)>) {
    let mut flux = T::zero();
    let mut area = T::zero();
    let mut flux_pad = 0.0f64;
    let mut area_pad = 0.0f64;
    let mut refused = None;
    for run in runs {
        if let (None, Some(refusal)) = (&refused, &run.refusal) {
            refused = Some((run.face, refusal.clone()));
        }
        flux = flux + run.contribution.flux;
        area = area + run.contribution.area;
        flux_pad += run.contribution.flux_pad;
        area_pad += run.contribution.area_pad;
    }
    (
        MassProperties {
            volume: flux / T::from_f64(3.0),
            surface_area: area,
            volume_pad: flux_pad / 3.0,
            area_pad,
        },
        refused,
    )
}

/// One face's divergence-theorem contribution, with the certified
/// quadrature half-widths it carries (`0.0` for closed-form faces).
/// The unit shared by the whole-body walk ([`mass_properties_impl`])
/// and the per-shell walk ([`classify_shells`]) — one flux
/// implementation, restricted by choosing which faces to visit, never
/// re-derived.
struct FaceFlux<T> {
    flux: T,
    area: T,
    flux_pad: f64,
    area_pad: f64,
}

/// One face's contribution and where its refinement stopped — the unit
/// a walk that may be RESUMED carries, one per face in the walk's own
/// order ([`fold_runs`] says which walk uses which).
struct FaceRun<T> {
    /// The face this run is of.
    face: FaceKey,
    /// Its contribution at the round the run reached.
    contribution: FaceFlux<T>,
    /// The round its quadrature reached, when rounds remain. `None`
    /// for a closed-form face, for one that met the reporting target,
    /// and for one whose schedule has nothing further to offer — in
    /// every case there is no round to resume at.
    open_at: Option<usize>,
    /// The refusal a target-level reading of this face earns. A face
    /// can carry one and still contribute a sound enclosure: that is
    /// the whole difference between the two levels.
    refusal: Option<PropsError>,
}

/// The per-face body of the flux walk (module docs): resolve the
/// surface, flatten the loops, dispatch closed form vs certified
/// quadrature over the round window asked for. Every refusal that
/// leaves the face with NO enclosure is a typed [`MassPropsError`];
/// a refusal that leaves one rides on the returned [`FaceRun`].
fn face_flux<T: Decide>(
    body: &Body<T>,
    face_key: FaceKey,
    band: Band,
    quad: &QuadHook<'_, T>,
    tol: Tol,
    window: RoundWindow,
) -> Result<FaceRun<T>, MassPropsError> {
    let Some(face) = body.faces.get(face_key) else {
        return Err(MassPropsError::Corrupt {
            what: "face key does not resolve",
        });
    };
    let Some(surface) = body.surfaces.get(face.surface) else {
        return Err(MassPropsError::Corrupt {
            what: "face surface key does not resolve",
        });
    };
    let wrap = |source| MassPropsError::Face {
        face: face_key,
        source,
    };
    let mut flux_pad = 0.0f64;
    let mut area_pad = 0.0f64;
    let mut open_at = None;
    let mut refusal = None;
    let contribution: FaceContribution<T> = match *surface {
        Surface::Plane { origin, .. } => {
            let mut loops = Vec::with_capacity(1 + face.rings.len());
            for &lk in core::iter::once(&face.outer).chain(&face.rings) {
                loops.push(loop_edges(body, lk)?.0);
            }
            planar_face(origin, &loops).map_err(wrap)?
        }
        _ => {
            if !face.rings.is_empty() {
                return Err(MassPropsError::RingOnCurvedFace { face: face_key });
            }
            let (outer, hes) = loop_edges(body, face.outer)?;
            // Structural dispatch (C5: on the carrier KIND, never a
            // runtime fallback): a conic/NURBS trim carrier routes
            // the face to the PR 11 certified-quadrature lane; an
            // iso boundary keeps its closed form. The S10 sense bit
            // does NOT enter the quadrature lane: its Green form is
            // winding-derived end to end (the signed UV area IS
            // s_f·|Ω| through the stored loop traversal), exactly
            // the class the S10 module docs keep bit-free.
            let is_trimmed = outer.iter().any(|e| {
                matches!(
                    e.carrier,
                    geom::Curve3::Ellipse { .. }
                        | geom::Curve3::Spiric { .. }
                        | geom::Curve3::Nurbs(_)
                )
            });
            // A described NURBS face ALWAYS takes the quadrature
            // lane (M6-3): its flux has no closed form regardless
            // of what bounds it, and the patch engine reads the
            // stored iso pcurves rather than the carriers.
            // A described SPLINE face always takes the quadrature lane
            // (M6-3): its flux has no closed form regardless of what
            // bounds it. An approximating face is one — the flux of
            // its fit, which is the geometry the face actually carries.
            let quad_out = if is_trimmed || surface.spline_chart().is_some() {
                quad(body, surface, &outer, &hes, band, tol, window).map_err(wrap)?
            } else {
                None
            };
            match quad_out {
                Some(outcome) => {
                    let bounds = match outcome {
                        RoundOutcome::Converged(bounds) => bounds,
                        RoundOutcome::Open {
                            bounds,
                            round,
                            refusal: lane_refusal,
                        } => {
                            // A window that ended early leaves a round
                            // to resume at; one that ended in a
                            // refusal leaves none, and the refusal is
                            // what a caller wanting a number earns.
                            open_at = lane_refusal.is_none().then_some(round);
                            refusal = lane_refusal;
                            bounds
                        }
                    };
                    let (fc, fp) = quad_lane::mid_pad(bounds.flux);
                    let (ac, ap) = quad_lane::mid_pad(bounds.area);
                    flux_pad += fp;
                    area_pad += ap;
                    FaceContribution {
                        flux: T::from_f64(fc),
                        area: T::from_f64(ac),
                    }
                }
                // Either an iso boundary (the closed forms — the
                // face's S10 sense entering at `curved_face`'s one
                // sanctioned site, the rimless sphere band) or a
                // walk holding no [`QuadLane`] (a `_structural` door,
                // which is how a dual measures) — whose honest outcome
                // on a trimmed face is the closed form's typed refusal.
                None => curved_face(surface, &outer, face.sense, band).map_err(wrap)?,
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
        },
        open_at,
        refusal,
    })
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
    let mut hes = Vec::with_capacity(cycle.len());
    for &he_key in &cycle {
        hes.push(he_key);
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
            // A lineage that cycles is one a graft aliased (issue 1597:
            // records are copied with their source keys, which in the
            // destination chain into strangers); the flattening then
            // stamps NO identity, so no two such edges are ever folded
            // into one — the fold declines rather than trusting a
            // record it cannot read, and a split meridian on such a
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
    Ok((edges, hes))
}

/// The derived outer/void designation of one shell. The shell list
/// stores no such designation ([`crate::entity::Solid`]'s documented
/// invariant): the role IS the sign of the shell's signed volume —
/// a shell's loops wind about the outward normal, so a boundary that
/// bounds material from outside integrates positive and a cavity wall
/// integrates negative.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ShellRole {
    /// Definitely-positive signed volume: the shell is the outer
    /// boundary of one connected component of material.
    Outer,
    /// Definitely-negative signed volume: the shell bounds an internal
    /// cavity.
    Void,
}

/// One shell's flux-derived properties and its decided role.
///
/// `volume` is the shell's SIGNED enclosed volume (divergence theorem
/// over exactly this shell's faces — the same per-face closed
/// forms/quadrature the whole-body [`mass_properties`] sums, so the
/// per-shell volumes of a body sum to its body volume and the pads to
/// its pad). For quadrature faces the value is the certified
/// enclosure's midpoint and the bracket is `± volume_pad`
/// ([`MassProperties`]' convention).
#[derive(Clone, Copy, Debug)]
pub struct ShellClassification<T: Real> {
    /// The classified shell.
    pub shell: ShellKey,
    /// The solid owning it ([`crate::entity::Shell::solid`]).
    pub solid: SolidKey,
    /// The shell's signed enclosed volume (midpoint; bracket
    /// `± volume_pad`).
    pub volume: T,
    /// Certified half-width of the volume bracket (m³); `0.0` when
    /// every face of the shell is closed-form.
    pub volume_pad: f64,
    /// The shell's surface area (midpoint; bracket `± area_pad`).
    pub surface_area: T,
    /// Certified half-width of the area bracket (m²).
    pub area_pad: f64,
    /// The decided role.
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
    /// The sign read escalated: the shell's `V/A` margin sits inside
    /// the ambiguity band. F6: an in-band orientation is never
    /// guessed to a side.
    Escalated {
        /// The unclassifiable shell.
        shell: ShellKey,
        /// The named escalation from the funnel.
        source: Indeterminate,
    },
    /// The signed volume is definitely zero, or its certified bracket
    /// definitely straddles zero — there is no side to classify to.
    ZeroVolume {
        /// The unclassifiable shell.
        shell: ShellKey,
    },
}

impl fmt::Display for ShellClassifyError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Band { error } => write!(f, "shell classification: {error}"),
            Self::Props { shell, source } => {
                write!(f, "shell classification: shell {shell:?}: {source}")
            }
            Self::Escalated { shell, source } => {
                write!(f, "shell classification: shell {shell:?}: {source}")
            }
            Self::ZeroVolume { shell } => write!(
                f,
                "shell classification: shell {shell:?}'s signed volume is \
                 definitely zero (or its certified bracket straddles zero) — \
                 no outer/void side exists to classify to"
            ),
        }
    }
}

impl std::error::Error for ShellClassifyError {}

/// Per-shell signed volume and outer/void role for every shell of
/// `body`, in shell-arena slot order (deterministic per D9).
///
/// The flux machinery is tier-3 check 7's, SHARED (engineering
/// convention 2): each shell sums the same per-face contributions the
/// whole-body [`mass_properties`] sums, restricted to that shell's
/// face list — never a second flux implementation. The sign is a
/// decided predicate through the crate funnel (`chk_shell_volume_sign`)
/// with check 7's margin convention: the comparand is `V/A`, the mean
/// boundary displacement the volume corresponds to — a length. For
/// quadrature faces the read is bracket-honest: `Outer` requires the
/// bracket's LOW end definitely positive, `Void` its HIGH end
/// definitely negative; anything else refuses typed
/// ([`ShellClassifyError::Escalated`] / [`ShellClassifyError::ZeroVolume`]),
/// never a guess.
///
/// # Errors
///
/// [`ShellClassifyError`] — the first shell that cannot be classified
/// refuses the call: an inherited flux refusal, an in-band sign, or a
/// definite zero. (A component count over a partial classification
/// would be a guess; the caller gets the refusal instead.)
pub fn classify_shells<T: Decide + geom_core::CertifiedBounds>(
    body: &Body<T>,
    tol: Tol,
) -> Result<Vec<ShellClassification<T>>, ShellClassifyError> {
    let every: Vec<ShellKey> = body.shells.iter().map(|(k, _)| k).collect();
    classify_shells_of(body, &every, tol)
}

/// [`classify_shells`] holding NO quadrature lane — the same
/// classification at any deciding scalar with a bracket, each shell's
/// flux summed through the closed form alone
/// ([`mass_properties_structural`]'s walk, per shell). A shell with a
/// face that needs the certified quadrature refuses typed
/// ([`ShellClassifyError::Props`]); a closed-form shell classifies
/// exactly as it does through the certified door.
///
/// # Errors
///
/// [`ShellClassifyError`] — [`classify_shells`]'s, plus the closed
/// form's typed refusal on every face the quadrature lane would have
/// enclosed.
pub fn classify_shells_structural<T: Decide>(
    body: &Body<T>,
    tol: Tol,
) -> Result<Vec<ShellClassification<T>>, ShellClassifyError> {
    let every: Vec<ShellKey> = body.shells.iter().map(|(k, _)| k).collect();
    classify_shells_via(body, &every, tol, None)
}

/// [`classify_shells`] restricted to `shells` — the same classification,
/// made of exactly the shells named and no others, in shell-arena slot
/// order.
///
/// **A shell's role is a property of that shell alone**: the signed
/// volume its own faces integrate, decided at its own bracket. So a
/// caller asking about one solid's shells is asking a question that
/// does not involve any other solid's, and it should not pay another
/// solid's refusal for it — a classification that escalates on a
/// neighbour would otherwise refuse a body this caller can answer for.
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
    body: &Body<T>,
    shells: &[ShellKey],
    tol: Tol,
) -> Result<Vec<ShellClassification<T>>, ShellClassifyError> {
    classify_shells_via(body, shells, tol, Some(QuadLane::certified()))
}

/// The per-shell walk over the lane the caller holds — the shared body
/// of [`classify_shells_of`] and [`classify_shells_structural`].
fn classify_shells_via<T: Decide>(
    body: &Body<T>,
    shells: &[ShellKey],
    tol: Tol,
    quad: Option<QuadLane<T>>,
) -> Result<Vec<ShellClassification<T>>, ShellClassifyError> {
    let band = Band::linear(tol).map_err(|error| ShellClassifyError::Band { error })?;
    let hook = reporting_hook(quad);
    let mut out = Vec::new();
    for (shell_key, shell) in body.shells.iter() {
        if !shells.contains(&shell_key) {
            continue;
        }
        let props = |source| ShellClassifyError::Props {
            shell: shell_key,
            source,
        };
        // The per-shell walk reads at the REPORTING level: a shell role
        // is a claim about a volume, and its faces run their whole
        // schedules. Same flux and the same hook as
        // [`mass_properties_impl`], restricted to this shell's faces,
        // then the sequential fold of [`fold_runs`] in the shell's own
        // face list order, which is this walk's order throughout — the
        // whole-body walks' is the face arena's.
        //
        // **Serial, and not [`decide_faces`]**: every shell the census
        // meets is below the per-face map's break-even, so mapping
        // here buys a regression and no body a gain — the numbers are
        // on `work/perf/`'s
        // `parallel-map-costs-a-fixed-price-on-a-cheap-body`, with the
        // reason ([`decide_faces_serially`] restates it at the door).
        // The grain that could repay the price is the loop over SHELLS
        // above, which measures at about break-even on the same
        // document; that is the census's own question and not this
        // loop's, so the face grain does not want re-trying.
        let runs = decide_faces_serially(&shell.faces, |&face_key| {
            face_flux(body, face_key, band, &hook, tol, RoundWindow::SCHEDULE)
        })
        .map_err(props)?;
        // As at [`mass_properties_impl`]: unreachable from the hook
        // above, and what keeps this walk correct for the hook
        // signature rather than for one hook.
        let (sums, refused) = fold_runs(&runs);
        if let Some((face, source)) = refused {
            return Err(props(MassPropsError::Face { face, source }));
        }
        let (area, area_pad) = (sums.surface_area, sums.area_pad);
        let (volume, volume_pad) = (sums.volume, sums.volume_pad);
        // The named sign read — ONE funnel site, evaluated at a
        // bracket end. `V/A` is a length (check 7's margin
        // convention): the mean displacement of this shell's boundary
        // that the volume defect corresponds to.
        let sign_at = |end: T| {
            crate::validate::decide("chk_shell_volume_sign", Margin::over_lever(end, area), band)
        };
        let lo = sign_at(volume - T::from_f64(volume_pad));
        let role = if matches!(lo, Ok(Sign::Positive)) {
            // Even the bracket's low end is definitely positive.
            ShellRole::Outer
        } else {
            // Closed-form shells (pad = 0) reuse the one verdict; a
            // padded bracket reads its own high end.
            let hi = if volume_pad == 0.0 {
                lo
            } else {
                sign_at(volume + T::from_f64(volume_pad))
            };
            match hi {
                Ok(Sign::Negative) => ShellRole::Void,
                Err(source) => {
                    return Err(ShellClassifyError::Escalated {
                        shell: shell_key,
                        source,
                    });
                }
                Ok(Sign::Zero) => {
                    return Err(ShellClassifyError::ZeroVolume { shell: shell_key });
                }
                // High end definitely positive while the low end was
                // not: either the low end escalated (surface that
                // escalation) or the certified bracket definitely
                // straddles zero.
                Ok(Sign::Positive) => match lo {
                    Err(source) => {
                        return Err(ShellClassifyError::Escalated {
                            shell: shell_key,
                            source,
                        });
                    }
                    _ => return Err(ShellClassifyError::ZeroVolume { shell: shell_key }),
                },
            }
        };
        out.push(ShellClassification {
            shell: shell_key,
            solid: shell.solid,
            volume,
            volume_pad,
            surface_area: area,
            area_pad,
            role,
        });
    }
    Ok(out)
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
    /// the round window it is handed — `quad_lane::cut_face_rounds`,
    /// and nothing else can be written here (`wiring_rows` pins the
    /// pointer).
    cut_face_rounds: fn(
        &Body<T>,
        &Surface<T>,
        &[LoopEdge<T>],
        &[HalfEdgeKey],
        Band,
        Tol,
        RoundWindow,
    ) -> Result<RoundOutcome, PropsError>,
}

impl<T: Decide + geom_core::CertifiedBounds> QuadLane<T> {
    /// The certified quadrature — the whole inventory of this door, and
    /// the only constructor there is. Its body is
    /// `quad_lane::cut_face_rounds`, entered by the reporting walk over
    /// the whole schedule and by [`sign_walk`] one round window at a
    /// time, so [`mass_properties`] at a certifying scalar and the
    /// certified validator are one quadrature entered at two levels
    /// rather than two quadratures.
    #[must_use]
    pub const fn certified() -> Self {
        Self {
            cut_face_rounds: quad_lane::cut_face_rounds::<T>,
        }
    }
}

impl<T: Decide> QuadLane<T> {
    /// The door's operation at the REPORTING level, reached by
    /// [`reporting_hook`]: the whole schedule, read at the target.
    ///
    /// # Errors
    ///
    /// [`PropsError`] from the quadrature lane (budget, unsupported
    /// inventory, escalations).
    fn cut_face(
        self,
        body: &Body<T>,
        surface: &Surface<T>,
        outer: &[LoopEdge<T>],
        hes: &[HalfEdgeKey],
        band: Band,
        tol: Tol,
    ) -> Result<FaceCutBounds, PropsError> {
        (self.cut_face_rounds)(body, surface, outer, hes, band, tol, RoundWindow::SCHEDULE)?
            .into_target()
    }

    /// The door's operation at SIGN level, reached by [`round_hook`]:
    /// the rounds `window` names and no others.
    ///
    /// # Errors
    ///
    /// As [`Self::cut_face`].
    #[allow(clippy::too_many_arguments)]
    fn cut_face_rounds(
        self,
        body: &Body<T>,
        surface: &Surface<T>,
        outer: &[LoopEdge<T>],
        hes: &[HalfEdgeKey],
        band: Band,
        tol: Tol,
        window: RoundWindow,
    ) -> Result<RoundOutcome, PropsError> {
        (self.cut_face_rounds)(body, surface, outer, hes, band, tol, window)
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
/// The code is `E0599` for [`QuadLane`]'s reason: `certified` EXISTS on
/// `ShellDoor<Dual64>` and its `impl` block's bounds are not met, which
/// `rustc` reports as "the associated function exists … but its trait
/// bounds were not satisfied". Stable rustdoc verifies only that the
/// block fails to build, so the code beside the fence is a statement
/// and not a check.
#[derive(Clone, Copy)]
#[allow(clippy::type_complexity)]
pub struct ShellDoor<T: Decide> {
    /// [`ShellDoor::open`]'s body — `crate::shell_open`, and nothing
    /// else can be written here (`wiring_rows` pins the pointer).
    open: fn(&Body<T>, T, &[FaceKey], Tol) -> Result<Shelled<T>, ShellError<T>>,
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
        body: &Body<T>,
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
/// A row that compares outputs cannot see a door re-pointed at a
/// routine that agrees on the fixture in front of it; these rows
/// compare the stored function pointer instead, so a re-point is a
/// failure no matter what it computes. Function-pointer identity is
/// what `std::ptr::fn_addr_eq` compares and is not a language guarantee
/// (identical bodies may be merged), which costs nothing here: a false
/// PASS would need the re-pointed routine to be instruction-identical
/// to the one it replaced.
///
/// Each door has one helper, instantiated once per certifying scalar.
/// `certified_enclosure_impl_census` counts those instantiations
/// against the `CertifiedEnclosure` impls in the tree, both directions,
/// and counts the tree's door values against its roster of helpers.
#[cfg(test)]
mod wiring_rows {
    use super::{AtRestPolicy, QuadLane, ShellDoor, quad_lane};

    /// `Ok(())` when the quadrature door holds
    /// `quad_lane::cut_face_rounds`; otherwise the name of the field
    /// that moved.
    fn holds_the_certified_quadrature<T: super::Decide + geom_core::CertifiedBounds>()
    -> Result<(), &'static str> {
        if !std::ptr::fn_addr_eq(
            QuadLane::<T>::certified().cut_face_rounds,
            quad_lane::cut_face_rounds::<T> as fn(_, _, _, _, _, _, _) -> _,
        ) {
            return Err("cut_face_rounds is not `quad_lane::cut_face_rounds`");
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
/// the telemetry probe, the interval scalar), whose impls here
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
/// at the public doors today; the E4 driver's content-key equality
/// assertion (DL3's soundness hook) is a NAMED banked obligation of
/// M10-4's driver. That is sound because
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
/// The trait also carries the two INJECTED DOORS whose presence is a
/// per-scalar fact, for the same reason it carries the gates: it is
/// the per-scalar policy home. [`AtRestPolicy::offset_fit_lane`] is
/// the offset fit's, and [`AtRestPolicy::shell_door`] is the
/// hollowing verb's; each answers `None` for its own reason — a
/// derivation written at one scalar, and certification rights (DL1) —
/// and the doc on each method says which. What a reader gets from the
/// one trait is every per-scalar answer the at-rest machinery needs,
/// in one place, rather than a lane trait apiece.
///
/// **Why the one lane trait rides along.**
/// [`geom_brep::PcurveFittedLane`] is a supertrait because it is the
/// same split, over the same scalars, for the same reason as the
/// gates: a fitted (rung-3) pcurve's between-samples obligation is a
/// certificate reached through a scalar's bracket, exactly as the
/// quadrature's flux enclosures are; `f64`, the telemetry probe, the
/// interval scalar and `Sym` over any of those can derive it, and a
/// [`Dual`](geom_core::Dual) cannot and says so in a refusing impl (a
/// dual carries a bracket since D1 — the refusal stands on DL1, a dual
/// may not certify, which is [`geom_core::CertifiedEnclosure`]'s
/// absence and not [`geom_core::Bounds`]'). So `T: AtRestPolicy` reads
/// at every consumer as "this scalar's at-rest policy, lane included",
/// where the alternative is threading a pointwise-identical lane bound
/// through every tier-3 signature and generic body helper for no
/// additional honesty — the refusing side is the same scalar. The
/// bundle holds until that lane trait goes the way the quadrature
/// lane's ([`QuadLane`]) and the chart-region lane's
/// ([`crate::RegionLane`]) did, a value in place of a trait, taken as
/// an `Option` by the passes that run at both kinds of scalar.
/// [`geom_core::Bounds`] deliberately does
/// not ride along: a name that hands out a bracket door is a bound the `Bounds`
/// scope rule's gate cannot read at its use sites, so every door that
/// reads a bracket spells `Bounds` where the reader can see it.
pub trait AtRestPolicy: Decide + geom_brep::PcurveFittedLane {
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
    /// (`work/scalar/H5.md` §RATIFIED ruling 3, which keeps this trait
    /// as the per-scalar policy that cut leaves standing): the door
    /// itself is a value the passes take as a parameter, and this is
    /// the one place each scalar's answer is written. The same holds
    /// of the shell door beside it ([`AtRestPolicy::shell_door`]) —
    /// two doors, one policy, no trait apiece.
    fn offset_fit_lane() -> Option<geom_brep::OffsetFitLane<Self>>;

    /// **This scalar's shell door, or `None` where it may not form the
    /// call** — the ONE seam the `Some` comes from, read by the verb
    /// seat's `verbs::Verb::run_shell` and, above it, the document
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

    /// The at-rest gate over a body ([`crate::validate_geometric`] at
    /// certifying scalars; absent at duals, and the outcome says
    /// which).
    ///
    /// # Errors
    ///
    /// The validator's own findings, verbatim, where the scalar runs
    /// it.
    fn gate_at_rest(body: &Body<Self>, tol: Tol) -> Result<AtRestOutcome, Vec<ValidationError>>;

    /// The at-rest gate over a body with declared contacts — the
    /// tier-3′ census door ([`crate::validate_pseudomanifold`] at
    /// certifying scalars; absent at duals).
    ///
    /// # Errors
    ///
    /// The validator's own findings, verbatim, where the scalar runs
    /// it.
    fn gate_at_rest_declared(
        body: &Body<Self>,
        contacts: &ContactRecords,
        tol: Tol,
    ) -> Result<AtRestOutcome, Vec<ValidationError>>;
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

    /// The decide-with-escalation lane certifies, so it runs the door.
    fn shell_door() -> Option<ShellDoor<Self>> {
        Some(ShellDoor::certified())
    }

    fn gate_at_rest(body: &Body<Self>, tol: Tol) -> Result<AtRestOutcome, Vec<ValidationError>> {
        crate::validate::validate_geometric(body, tol).map(|()| AtRestOutcome::Validated)
    }

    fn gate_at_rest_declared(
        body: &Body<Self>,
        contacts: &ContactRecords,
        tol: Tol,
    ) -> Result<AtRestOutcome, Vec<ValidationError>> {
        crate::validate::validate_pseudomanifold(body, contacts, tol)
            .map(|()| AtRestOutcome::Validated)
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

    /// The recording scalar is `f64` with a sink attached, so it
    /// carries exactly what `f64` carries — here, the door.
    fn shell_door() -> Option<ShellDoor<Self>> {
        Some(ShellDoor::certified())
    }

    fn gate_at_rest(body: &Body<Self>, tol: Tol) -> Result<AtRestOutcome, Vec<ValidationError>> {
        crate::validate::validate_geometric(body, tol).map(|()| AtRestOutcome::Validated)
    }

    fn gate_at_rest_declared(
        body: &Body<Self>,
        contacts: &ContactRecords,
        tol: Tol,
    ) -> Result<AtRestOutcome, Vec<ValidationError>> {
        crate::validate::validate_pseudomanifold(body, contacts, tol)
            .map(|()| AtRestOutcome::Validated)
    }
}

impl AtRestPolicy for geom_core::interval::Interval {
    /// The fit is derived at `f64` only — a fact about the scalar the
    /// derivation was written in, not about this scalar's
    /// certification rights, which it has in full.
    fn offset_fit_lane() -> Option<geom_brep::OffsetFitLane<Self>> {
        None
    }

    /// The certified interval scalar runs the door: its brackets are
    /// what the validator's certified claim is made of.
    fn shell_door() -> Option<ShellDoor<Self>> {
        Some(ShellDoor::certified())
    }

    fn gate_at_rest(body: &Body<Self>, tol: Tol) -> Result<AtRestOutcome, Vec<ValidationError>> {
        crate::validate::validate_geometric(body, tol).map(|()| AtRestOutcome::Validated)
    }

    fn gate_at_rest_declared(
        body: &Body<Self>,
        contacts: &ContactRecords,
        tol: Tol,
    ) -> Result<AtRestOutcome, Vec<ValidationError>> {
        crate::validate::validate_pseudomanifold(body, contacts, tol)
            .map(|()| AtRestOutcome::Validated)
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
{
    /// The fit is derived at `f64` only. The tier changes how a margin
    /// DECIDES, not which derivations exist, so wrapping a scalar
    /// cannot add one.
    fn offset_fit_lane() -> Option<geom_brep::OffsetFitLane<Self>> {
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

    fn gate_at_rest(body: &Body<Self>, tol: Tol) -> Result<AtRestOutcome, Vec<ValidationError>> {
        crate::validate::validate_geometric(body, tol).map(|()| AtRestOutcome::Validated)
    }

    fn gate_at_rest_declared(
        body: &Body<Self>,
        contacts: &ContactRecords,
        tol: Tol,
    ) -> Result<AtRestOutcome, Vec<ValidationError>> {
        crate::validate::validate_pseudomanifold(body, contacts, tol)
            .map(|()| AtRestOutcome::Validated)
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
    geom_core::Dual<T>: Decide,
{
    /// The fit is derived at `f64` only — the same reason every other
    /// arm here gives, and a separate fact from the gates below, which
    /// are absent because a dual does not certify.
    fn offset_fit_lane() -> Option<geom_brep::OffsetFitLane<Self>> {
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

    fn gate_at_rest(_body: &Body<Self>, _tol: Tol) -> Result<AtRestOutcome, Vec<ValidationError>> {
        Ok(AtRestOutcome::NotRunAtThisScalar)
    }

    fn gate_at_rest_declared(
        _body: &Body<Self>,
        _contacts: &ContactRecords,
        _tol: Tol,
    ) -> Result<AtRestOutcome, Vec<ValidationError>> {
        Ok(AtRestOutcome::NotRunAtThisScalar)
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
    //! The shell door is the one arm that is not a gate on that
    //! subject: it is a VALUE, so what these rows pin is the arm's
    //! answer — `ShellDoor::certified()` at a certifying scalar, `None`
    //! at a dual — and which function that value holds is
    //! `wiring_rows`' pin, per scalar, beside the quadrature door's.

    use super::{AtRestOutcome, AtRestPolicy};
    use crate::body::Body;
    use crate::boolean::ContactRecords;
    use geom_core::{Decide, Point3, Tol};

    /// The `mvfs` seed body: its face surface is the placeholder, which
    /// a body at rest may not carry (`Body::mvfs` docs) — the cheapest
    /// scalar-generic refusing subject.
    fn refusing_body<T: Decide>() -> Body<T> {
        let mut b = Body::new();
        b.mvfs(Point3::new(T::zero(), T::zero(), T::zero()))
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
        let contacts = ContactRecords::default();
        // The certified door, and its name is the assertion: a
        // certifying arm takes the door whose bound names the right it
        // has, so check 2 re-derives the M7-8 carrier class here.
        // `validate_pseudomanifold_structural` is the lane-free sibling
        // a dual takes, and is not what this arm runs.
        let door = crate::validate::validate_pseudomanifold(&b, &contacts, tol);
        assert!(door.is_err(), "the seed body must refuse the census door");
        assert_eq!(
            T::gate_at_rest_declared(&b, &contacts, tol),
            door.map(|()| AtRestOutcome::Validated),
            "gate_at_rest_declared must be validate_pseudomanifold verbatim at a certifying \
             scalar"
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
    /// can still do is the structural half, and this row pins that
    /// instead: the seed body's placeholder surface is a check-1
    /// failure, which is structural, so the dual sees the same refusal
    /// the certifying scalars see through the same checks.
    #[test]
    fn dual_gate_is_absent_not_a_verdict() {
        let tol = Tol::witness();
        let b = refusing_body::<geom_core::Dual64>();
        assert!(
            crate::validate::validate_geometric_structural(&b, tol).is_err(),
            "the structural half still runs, and still refuses, at a dual"
        );
        assert_eq!(
            <geom_core::Dual64 as AtRestPolicy>::gate_at_rest(&b, tol),
            Ok(AtRestOutcome::NotRunAtThisScalar)
        );
        assert_eq!(
            <geom_core::Dual64 as AtRestPolicy>::gate_at_rest_declared(
                &b,
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
    }
}

/// The PR 11 certified-quadrature lane's body-side plumbing: stored
/// pcurve caches → ring-bracketed [`geom_brep::props::quad::TrimEdgeQ`]s (C4's first hot
/// consumer). Key-free math stays in `geom_brep::props::quad`; this
/// module owns everything that needs half-edges and vertex points.
mod quad_lane {
    use geom_brep::Pcurve;
    use geom_brep::props::quad::{
        self, FaceCutBounds, HarmChan, RoundOutcome, RoundWindow, TrimChord, TrimEdgeQ, TrimPiece,
    };
    use geom_brep::props::{LoopEdge, PropsError, loop_vector_area};
    use geom_core::Tol;
    use geom_core::interval::Interval;
    // The compound `Decide + Bounds` bound below is a RATIFIED seam
    // (M5 PR 11, Ev's lane-split ruling; discipline allowlist row):
    // this module is the certified lanes' plumbing and never
    // instantiates for duals. Every signature below is
    // `Decide + Bounds + CertifiedEnclosure`, which no `Dual`
    // implements — a dual carries a bracket since D1 (2026-08-19) and
    // still may not certify — and [`super::QuadLane::certified`], the
    // one door from the face walks into `cut_face_rounds`, carries the
    // same bound. So the module stays uninstantiable at a dual.
    use geom::Curve3;
    use geom::Surface;
    use geom_core::{Band, Bounds, CertifiedEnclosure, Decide, Point3};

    use crate::body::Body;
    use crate::entity::HalfEdgeKey;

    /// Enclosure midpoint and half-width (the [`super::MassProperties`]
    /// pad decomposition).
    ///
    /// A refused enclosure has no midpoint and no width, and answers
    /// `NaN` for both — which is what every consumer of this pair
    /// already carries through `T::from_f64`. The refusal is asked by
    /// name: interval arithmetic keeps it in the decoration, so a refused
    /// enclosure's two endpoints are ordinary numbers and their
    /// average would be a plausible mass property with nothing behind
    /// it.
    pub(super) fn mid_pad(x: Interval) -> (f64, f64) {
        if !x.is_certified() {
            return (f64::NAN, f64::NAN);
        }
        ((x.lo() + x.hi()) * 0.5, (x.hi() - x.lo()) * 0.5)
    }

    /// `(cos t₀, sin t₀)` enclosure at the carrier-interval start,
    /// recovered algebraically from the carrier frame and the interval
    /// start's VERTEX point (within the run's ε of the carrier, D4 ¶2
    /// — the ε rides into the bracket as an explicit pad).
    fn trig_at_start<T: Decide + Bounds + CertifiedEnclosure>(
        carrier: &Curve3<T>,
        p: Point3<T>,
        eps: f64,
    ) -> Result<(Interval, Interval), PropsError> {
        let full = Interval::from_bounds(-1.0, 1.0);
        // `from_bounds` mints a fresh bracket out of whatever
        // endpoints it is handed, so a refused operand would come back
        // clean. The refusal is carried across by hand.
        let clamp = |x: Interval, pad: f64| {
            if !x.is_certified() {
                return Interval::poison();
            }
            Interval::from_bounds(x.lo() - pad, x.hi() + pad).clamped_to(-1.0, 1.0)
        };
        match carrier {
            // A line's harmonic pcurve has zero trig amplitudes; the
            // whole-circle bracket is sound and multiplies away.
            Curve3::Line { .. } => Ok((full, full)),
            Curve3::Circle {
                center,
                axis,
                radius,
                u_ref,
            } => {
                let v_ref = axis.cross(*u_ref);
                let w = p - *center;
                let c = Interval::from_certified(w.dot(*u_ref)) / Interval::from_certified(*radius);
                let s = Interval::from_certified(w.dot(v_ref)) / Interval::from_certified(*radius);
                let pad = (Interval::point(eps) / Interval::from_certified(*radius)).mag();
                Ok((clamp(c, pad), clamp(s, pad)))
            }
            Curve3::Ellipse {
                center,
                axis,
                major,
                minor,
                u_ref,
            } => {
                let v_ref = axis.cross(*u_ref);
                let w = p - *center;
                let c = Interval::from_certified(w.dot(*u_ref)) / Interval::from_certified(*major);
                let s = Interval::from_certified(w.dot(v_ref)) / Interval::from_certified(*minor);
                let pad_c = (Interval::point(eps) / Interval::from_certified(*major)).mag();
                let pad_s = (Interval::point(eps) / Interval::from_certified(*minor)).mag();
                Ok((clamp(c, pad_c), clamp(s, pad_s)))
            }
            // The spiric's chart images are not harmonic (its `m`
            // channel is `√((R + r cos v)² − d²)`), so the trig
            // brackets this lane reads do not exist for it. Unreachable
            // by construction: this lane is entered only for a CYLINDER
            // chart (`cut_face_rounds`'s chart gate), and a spiric lies
            // on no cylinder — the arm names the kind so the gate's
            // removal would meet a typed refusal here rather than a
            // wildcard. The props quadrature lane for a spiric-bounded
            // face is the spiric unit's props PR.
            Curve3::Spiric { .. } => Err(PropsError::QuadratureUnsupported {
                what: "spiric trim carrier on an ANALYTIC chart's quadrature lane — the \
                       hollowed partial revolve's torus wall and plane cap; the spiric \
                       quadrature lane is not yet written",
            }),
            Curve3::Nurbs(_) => Err(PropsError::QuadratureUnsupported {
                what: "B-spline trim carrier on an ANALYTIC chart's quadrature lane — \
                       the cut-loft class (a loft wall cut by a plane/cylinder), which \
                       needs the edge×NURBS-face boolean layer that is not \
                       written; described-NURBS faces with iso-line pcurves route to \
                       the patch engine instead",
            }),
        }
    }

    /// One channel of a stored harmonic pcurve, bracketed.
    fn chan<T: Decide + Bounds + CertifiedEnclosure>(
        c0: T,
        ca: T,
        cb: T,
        cl: T,
    ) -> Result<HarmChan, PropsError> {
        Ok(HarmChan {
            c0: Interval::from_certified(c0),
            ca: Interval::from_certified(ca),
            cb: Interval::from_certified(cb),
            cl: Interval::from_certified(cl),
        })
    }

    /// The certified flux/area enclosures of one curved-cut face
    /// (module docs of `geom_brep::props::quad`) over a [`RoundWindow`]
    /// — the cylinder chart's closed-form lane plus the described-NURBS
    /// patch lane (M6-3), entered and left where the window says (the
    /// quadrature module's two levels); [`super::QuadLane`] holds this
    /// and reads it at either level. Cone/sphere/torus charts MINT
    /// stored pcurves since M6-3 (walk row 4) but their chart-normal
    /// flux algebra is not written — they refuse typed naming that true
    /// blocker.
    #[allow(clippy::too_many_arguments)]
    pub(super) fn cut_face_rounds<T: Decide + Bounds + CertifiedEnclosure>(
        body: &Body<T>,
        surface: &Surface<T>,
        outer: &[LoopEdge<T>],
        hes: &[HalfEdgeKey],
        band: Band,
        tol: Tol,
        window: RoundWindow,
    ) -> Result<RoundOutcome, PropsError> {
        // The NURBS-patch lane (M6-3): a described NURBS face routes
        // to the patch engine over its stored iso-line pcurves.
        // The spline-patch lane (M6-3): a described spline face routes
        // to the patch engine over its stored iso-line pcurves. An
        // approximating face enters on its fit — the certificate's
        // bound is a statement about the DESCRIPTION and does not
        // widen this quadrature (the same deliberate omission the
        // mesh tolerance makes).
        if let Some(payload) = surface.spline_chart() {
            return nurbs_face(body, payload, outer, hes, band, tol, window);
        }
        let Surface::Cylinder { origin, radius, .. } = surface else {
            return Err(PropsError::QuadratureUnsupported {
                what: "conic trim on a cone/sphere/torus chart — those charts mint stored \
                       pcurves, but this lane's chart-normal flux algebra is the \
                       cylinder chart's; the other analytic charts' closed-form flux \
                       has no lane",
            });
        };
        let eps = tol.eps();
        let va = loop_vector_area(outer, *origin)?;
        let o_dot_va = Interval::from_certified((*origin - Point3::origin()).dot(va));
        let mut edges = Vec::with_capacity(outer.len());
        for (le, he) in outer.iter().zip(hes) {
            let Some(cache) = body.pcurve(*he) else {
                return Err(PropsError::QuadratureUnsupported {
                    what: "curved-cut face half-edge carries no stored pcurve cache — \
                           caches mint in the split/boolean pipelines",
                });
            };
            // The certified quadrature lane reads a chart image
            // CHANNEL BY CHANNEL out of its closed form; a fitted
            // (rung-3) image has no such form on an ANALYTIC chart's
            // Green reduction. Typed refusal — the TRUE remaining
            // blocker (M6-3 stale-claims sweep): no at-rest body mints
            // a fitted pcurve on a cylinder chart today (the marched
            // join windows and the edge×NURBS-face boolean layer are
            // both banked past M6), and the fitted-boundary Green lane
            // (`quad::bspline_green_integral`'s remaining consumer)
            // lands WITH whichever of those first produces one.
            let Pcurve::Harmonic { p0, pa, pb, pl } = *cache.pcurve() else {
                return Err(PropsError::QuadratureUnsupported {
                    what: "curved-cut face half-edge carries a FITTED (rung-3) pcurve on an \
                           analytic chart — its Green-form boundary integral \
                           (bspline_green_integral) wires up with the construction that \
                           first mints one at rest (the banked join-window/edge×NURBS-face \
                           boolean layers); nothing does today",
                });
            };
            let (t0, t1) = cache.params();
            // The interval-start vertex: traversal start when forward,
            // traversal end when reversed (`he_plus` start either way).
            let p_start = start_point(body, *he, le.forward)?;
            let trig0 = trig_at_start(&le.carrier, p_start, eps)?;
            edges.push(TrimEdgeQ {
                u: chan(p0.x, pa.x, pb.x, pl.x)?,
                v: chan(p0.y, pa.y, pb.y, pl.y)?,
                t0: Interval::from_certified(t0),
                t1: Interval::from_certified(t1),
                forward: le.forward,
                trig0,
                env: Interval::from_certified(cache.certificate().envelope),
            });
        }
        quad::cylinder_cut_face_rounds::<T>(
            Interval::from_certified(*radius),
            o_dot_va,
            &edges,
            eps,
            band,
            window,
        )
    }

    /// **The NURBS-patch flux lane** (M6-3 Leg C; RATIONAL since
    /// M8-3): certified volume flux + area of a described NURBS face whose
    /// stored pcurves pin its trim region to an exact axis-aligned UV
    /// rectangle (every loft/sweep wall — their boundaries are iso
    /// lines with exact-structure `0`/`1` chart values).
    ///
    /// Structure checks are EXACT `f64` (C6: the minted chart values
    /// are exact by construction; a non-exact or non-rectangular
    /// boundary refuses typed, naming the trimmed-NURBS lane as the
    /// cut-loft unit's). The traversal's shoelace sign IS the S10
    /// orientation input — winding-derived end to end, like the
    /// cylinder lane; no sense bit is read.
    #[allow(clippy::too_many_arguments)]
    fn nurbs_face<T: Decide + Bounds + CertifiedEnclosure>(
        body: &Body<T>,
        payload: &geom::NurbsSurface<T>,
        outer: &[LoopEdge<T>],
        hes: &[HalfEdgeKey],
        band: Band,
        tol: Tol,
        window: RoundWindow,
    ) -> Result<RoundOutcome, PropsError> {
        if payload.is_placeholder() {
            return Err(PropsError::QuadratureUnsupported {
                what: "the mvfs Nurbs placeholder reached the quadrature lane — a \
                       mid-surgery body has no mass properties (tier 2 refuses it at rest)",
            });
        }
        // **The dispatch is by pcurve KIND** (TRIM-2 §8.1): a loop
        // whose every image is an iso class pins the trim region to an
        // axis-aligned rectangle and keeps the rectangle certificate
        // below, bit for bit; a loop carrying a `General` image bounds
        // a region that is not a rectangle of its chart at all, and
        // takes the trimmed lane. `Fitted` keeps its own refusal in
        // both — no shipped construction mints one here.
        if hes
            .iter()
            .filter_map(|he| body.pcurve(*he))
            .any(|c| matches!(c.pcurve(), Pcurve::General(_)))
        {
            return trimmed_face(body, payload, outer, hes, band, tol, window);
        }
        let eps = tol.eps();
        // Exact-structure read of a T scalar (point bracket required).
        let exact = |x: Interval| -> Result<f64, PropsError> {
            // The refusal first: a refused crossing carries the
            // scalar's own endpoints, so a point bracket that may not
            // certify passes both tests below.
            if x.is_certified() && x.lo() == x.hi() && x.lo().is_finite() {
                Ok(x.lo())
            } else {
                Err(PropsError::QuadratureUnsupported {
                    what: "a NURBS-face pcurve endpoint is not exact structure — the \
                           rectangle-trim certificate needs the minted exact 0/1 chart \
                           values (trimmed-NURBS regions are the cut-loft unit's)",
                })
            }
        };
        let mut polygon: Vec<(f64, f64)> = Vec::with_capacity(outer.len());
        let mut boundary_defect = 0.0f64;
        let mut perimeter = 0.0f64;
        for (le, he) in outer.iter().zip(hes) {
            let Some(cache) = body.pcurve(*he) else {
                return Err(PropsError::QuadratureUnsupported {
                    what: "NURBS face half-edge carries no stored pcurve cache — the \
                           loft assembly mints them; a body that lost its caches must \
                           re-mint before mass properties",
                });
            };
            // Both iso classes pin the trim region to the rectangle:
            // `IsoLine` for seams and line rims, `IsoArc` for a
            // rational wall's arc rims (M8-3) — an arc rim's chart
            // image is the SAME boundary line, only its
            // parameterization differs, and this lane reads endpoints.
            if !matches!(
                cache.pcurve(),
                Pcurve::IsoLine { .. } | Pcurve::IsoArc { .. }
            ) {
                return Err(PropsError::QuadratureUnsupported {
                    what: "a NURBS-face half-edge carries a non-iso pcurve — a trimmed \
                           NURBS region's quadrature is the cut-loft unit's (the \
                           edge×NURBS-face boolean layer mints those trims)",
                });
            }
            let (t0, t1) = cache.params();
            let a = cache.pcurve().eval(t0);
            let b = cache.pcurve().eval(t1);
            let (ax, ay) = (
                exact(Interval::from_certified(a.x))?,
                exact(Interval::from_certified(a.y))?,
            );
            let (bx, by) = (
                exact(Interval::from_certified(b.x))?,
                exact(Interval::from_certified(b.y))?,
            );
            if ax != bx && ay != by {
                return Err(PropsError::QuadratureUnsupported {
                    what: "a NURBS-face pcurve is not axis-aligned — a diagonal trim is \
                           outside the rectangle lane (the cut-loft unit's)",
                });
            }
            // Traversal order: the loop walks he_plus-forward edges
            // start→end and reversed ones end→start.
            if le.forward {
                polygon.push((ax, ay));
            } else {
                polygon.push((bx, by));
            }
            // Metric boundary length bound + the map-residual defect.
            let len = carrier_metric_length(&le.carrier, t0, t1)?;
            perimeter += len;
            boundary_defect += len * Interval::from_certified(cache.certificate().envelope).mag();
        }
        // The rectangle certificate: hull of the traversal polygon,
        // every vertex on a corner, and the shoelace equal to ±the
        // rectangle area — the sign IS the S10 winding.
        let (mut u0, mut u1) = (f64::INFINITY, f64::NEG_INFINITY);
        let (mut v0, mut v1) = (f64::INFINITY, f64::NEG_INFINITY);
        for &(x, y) in &polygon {
            u0 = u0.min(x);
            u1 = u1.max(x);
            v0 = v0.min(y);
            v1 = v1.max(y);
        }
        let mut shoelace = 0.0f64;
        for i in 0..polygon.len() {
            let (xa, ya) = polygon[i];
            let (xb, yb) = polygon[(i + 1) % polygon.len()];
            shoelace += xa * yb - xb * ya;
            if (xa != u0 && xa != u1) && (ya != v0 && ya != v1) {
                return Err(PropsError::QuadratureUnsupported {
                    what: "a NURBS-face boundary vertex sits strictly inside the UV \
                           rectangle — a re-entrant trim is outside the rectangle lane \
                           (the cut-loft unit's)",
                });
            }
        }
        shoelace *= 0.5;
        let rect_area = (u1 - u0) * (v1 - v0);
        let winding = if shoelace == rect_area {
            1.0
        } else if shoelace == -rect_area {
            -1.0
        } else {
            return Err(PropsError::QuadratureUnsupported {
                what: "the NURBS-face boundary does not traverse its UV rectangle exactly \
                       once (shoelace ≠ ±rectangle area) — a trimmed or multiply-wound \
                       region is outside the rectangle lane (the cut-loft unit's)",
            });
        };
        let control: Vec<quad::RVec3> = payload
            .control()
            .iter()
            .map(|p| {
                [
                    Interval::from_certified(p.x),
                    Interval::from_certified(p.y),
                    Interval::from_certified(p.z),
                ]
            })
            .collect();
        let out = quad::nurbs_patch_face_rounds::<T>(
            payload.knots_u(),
            payload.knots_v(),
            &control,
            payload.weights(),
            (u0, u1, v0, v1),
            perimeter,
            boundary_defect,
            eps,
            band,
            window,
        )?;
        // The winding sign carries the S10 orientation into the flux;
        // the area is unsigned. It applies at whichever round the
        // window ended: the sign is a property of the traversal, not
        // of the refinement.
        Ok(out.map_bounds(|b| FaceCutBounds {
            flux: if winding < 0.0 { -b.flux } else { b.flux },
            area: b.area,
        }))
    }

    /// A certified UPPER bound on a trim carrier's METRIC length, in
    /// metres — the lever of both honesty pads (`Σ L·envelope` widens
    /// the area, and `× p_bound` the flux) and of the extent gate's
    /// perimeter. ONE home: the rectangle certificate and the trimmed
    /// lane bound the same quantity the same way, and two spellings of
    /// it would be two things to keep equal.
    fn carrier_metric_length<T: Decide + Bounds + CertifiedEnclosure>(
        carrier: &Curve3<T>,
        t0: T,
        t1: T,
    ) -> Result<f64, PropsError> {
        Ok(match carrier {
            Curve3::Line { dir, .. } => {
                (Interval::from_certified(dir.norm()) * Interval::from_certified(t1 - t0)).mag()
            }
            // The control polygon bounds the spline's arc length
            // (the convex-hull/variation-diminishing fact).
            Curve3::Nurbs(c) => {
                let mut l = Interval::zero();
                for w in c.control().windows(2) {
                    l = l + Interval::from_certified(w[0].distance(w[1]));
                }
                l.mag()
            }
            // An ARC cap rim on a rational wall (M8-3): the metric
            // length is exactly `r·Δθ` — the carrier's own parameter
            // IS the angle, so no bound is needed.
            Curve3::Circle { radius, .. } => {
                (Interval::from_certified(*radius) * Interval::from_certified(t1 - t0)).mag()
            }
            _ => {
                return Err(PropsError::QuadratureUnsupported {
                    what: "a NURBS-face boundary carrier outside the loft inventory \
                           (line, spline and circle rims are the minted classes)",
                });
            }
        })
    }

    /// **The TRIMMED-region flux lane** (TRIM-2): a described NURBS
    /// face whose loop carries a `General` chart image, so its trim
    /// region is what the image bounds rather than a rectangle of the
    /// chart.
    ///
    /// This function assembles; the certification is
    /// [`quad::trimmed_patch_face_rounds`]'s. The traversal's own
    /// direction is carried per chord and the S10 winding is the chord
    /// polygon's shoelace sign, read inside the engine — winding-derived
    /// end to end, exactly as the rectangle certificate and the cylinder
    /// lane are.
    #[allow(clippy::too_many_arguments)]
    fn trimmed_face<T: Decide + Bounds + CertifiedEnclosure>(
        body: &Body<T>,
        payload: &geom::NurbsSurface<T>,
        outer: &[LoopEdge<T>],
        hes: &[HalfEdgeKey],
        band: Band,
        tol: Tol,
        window: RoundWindow,
    ) -> Result<RoundOutcome, PropsError> {
        let ring = |x: T| Interval::from_certified(x);
        let mut chords: Vec<TrimChord> = Vec::with_capacity(outer.len());
        for (le, he) in outer.iter().zip(hes) {
            let Some(cache) = body.pcurve(*he) else {
                return Err(PropsError::QuadratureUnsupported {
                    what: "NURBS face half-edge carries no stored pcurve cache — the \
                           loft assembly mints them; a body that lost its caches must \
                           re-mint before mass properties",
                });
            };
            let (t0, t1) = cache.params();
            let (pa, pb) = (cache.pcurve().eval(t0), cache.pcurve().eval(t1));
            let (a, b) = if le.forward {
                ((ring(pa.x), ring(pa.y)), (ring(pb.x), ring(pb.y)))
            } else {
                ((ring(pb.x), ring(pb.y)), (ring(pa.x), ring(pa.y)))
            };
            let piece = match cache.pcurve() {
                // An iso image is one exact chord: its endpoints are
                // structure, so there is no arc to bound.
                Pcurve::IsoLine { .. } | Pcurve::IsoArc { .. } => None,
                Pcurve::General(image) => {
                    // The engine subdivides the WHOLE stored image, so
                    // a cache whose carrier interval is a sub-range of
                    // its image's domain would have the lane integrate
                    // along chart the face does not bound. Exact
                    // structure, like every other read on this path.
                    let (d0, d1) = image.domain();
                    let (r0, r1) = (ring(t0), ring(t1));
                    // NO ROW AND NO KNOWN PRODUCER, stated so a reader
                    // does not take the guard for evidence of the case:
                    // `derive_general_image` mints an image over the
                    // carrier's whole interval, so nothing at rest
                    // stores a sub-range, and nothing in the suites
                    // hand-builds one. It is here because the trimmed
                    // lane subdivides the STORED image whole, and a
                    // future producer that stored a sub-range would get
                    // a certified number for chart the face does not
                    // bound rather than a refusal.
                    // The refusal first, for the reason the
                    // `exact` closure above gives.
                    if !r0.is_certified()
                        || !r1.is_certified()
                        || !(r0.lo() == r0.hi()
                            && r1.lo() == r1.hi()
                            && r0.lo() == d0
                            && r1.hi() == d1)
                    {
                        return Err(PropsError::QuadratureUnsupported {
                            what: "a General trim image whose carrier interval is not its \
                                   own knot domain — the trimmed lane subdivides the \
                                   stored image whole, and a sub-range would integrate \
                                   along chart the face does not bound",
                        });
                    }
                    Some(TrimPiece {
                        knots: image.knots().clone(),
                        control: image
                            .control()
                            .iter()
                            .map(|p| (ring(p.x), ring(p.y)))
                            .collect(),
                        weights: image.weights().to_vec(),
                    })
                }
                Pcurve::Fitted(_) => {
                    return Err(PropsError::QuadratureUnsupported {
                        what: "a NURBS-face half-edge carries a FITTED (rung-3) pcurve — \
                               the trimmed lane certifies the General class, whose \
                               agreement with its carrier is a measurement; nothing \
                               ships that mints a Fitted image on a spline chart",
                    });
                }
                Pcurve::Harmonic { .. } => {
                    return Err(PropsError::QuadratureUnsupported {
                        what: "a NURBS-face half-edge carries a HARMONIC pcurve — that is \
                               an analytic chart's closed form, and this chart is a \
                               spline patch",
                    });
                }
            };
            chords.push(TrimChord {
                a,
                b,
                piece,
                forward: le.forward,
                env: ring(cache.certificate().envelope),
            });
        }
        // **One bracket per shared vertex.** Two consecutive half-edges
        // meet at a vertex, and each reads it through its OWN pcurve —
        // an `IsoLine`'s `eval(t)` against a `General`'s clamped end —
        // so at `f64` the two reads can differ by the certification's
        // own size (2.2e-16 on the P-2 fixture, where the image's
        // control box is `u ∈ [2 − 2.2e-16, 2]` against the rim's exact
        // `u = 2`). Hulling them makes the walk close by construction
        // and hands the door the honest bracket for the vertex; the
        // door's own closure check then guards a CALLER, not this
        // assembler's rounding.
        for i in 0..chords.len() {
            let j = (i + 1) % chords.len();
            let merged = (
                Interval::hull(chords[i].b.0, chords[j].a.0),
                Interval::hull(chords[i].b.1, chords[j].a.1),
            );
            chords[i].b = merged;
            chords[j].a = merged;
        }
        let control: Vec<quad::RVec3> = payload
            .control()
            .iter()
            .map(|p| [ring(p.x), ring(p.y), ring(p.z)])
            .collect();
        quad::trimmed_patch_face_rounds::<T>(
            payload.knots_u(),
            payload.knots_v(),
            &control,
            payload.weights(),
            &chords,
            tol.eps(),
            band,
            window,
        )
    }

    /// The vertex POINT at a half-edge's carrier-interval start (its
    /// edge's `he_plus` start vertex).
    fn start_point<T: Decide + Bounds + CertifiedEnclosure>(
        body: &Body<T>,
        he: HalfEdgeKey,
        forward: bool,
    ) -> Result<Point3<T>, PropsError> {
        let corrupt = PropsError::QuadratureUnsupported {
            what: "corrupt body reaching the quadrature lane (a key did not resolve)",
        };
        let vk = if forward {
            body.half_edges.get(he).ok_or(corrupt.clone())?.start
        } else {
            body.half_edge_end(he).ok_or(corrupt.clone())?
        };
        let v = body.vertices.get(vk).ok_or(corrupt.clone())?;
        body.points.get(v.point).copied().ok_or(corrupt)
    }

    #[cfg(test)]
    mod tests {
        /// The scalar bracket seam, at the `Interval` scalar.
        ///
        /// A bracket can be sound and still inadmissible:
        /// `sqrt([−1, 4]) + 1` is `[1, 3]` with decoration `Trv`.
        /// The crossing into certification arithmetic reads the verdict
        /// here and caps the decoration at `Trv`, so the quadrature
        /// lane's scalars are refused HERE rather than a certified flux
        /// enclosure being built from a quantity that was clamped out of
        /// its own domain.
        #[allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]
        mod bracket_seam_tests {
            use geom_core::{Bounds, CertifiedEnclosure, Interval, Real};

            use super::super::chan;

            /// Finite, strictly positive, and unable to certify — the case
            /// where the laundered answer is a *usable* number.
            fn trv_pos() -> Interval {
                Interval::from_bounds(-1.0, 4.0).sqrt() + Interval::from_f64(1.0)
            }

            #[test]
            fn the_fixture_is_a_finite_bracket_that_cannot_certify() {
                let x = trv_pos();
                assert_eq!((Bounds::lo(x), Bounds::hi(x)), (1.0, 3.0));
                assert!(x.certified_bracket().is_none());
            }

            #[test]
            fn the_certified_door_refuses_a_violated_scalar() {
                let r = Interval::from_certified(trv_pos());
                assert!(
                    !r.is_certified(),
                    "a domain-violated scalar crossed into certification arithmetic as {r:?} — \
                     the bracket door does not read decorations, so the \
                     quadrature lane certifies a flux built from it"
                );
                // Non-vacuity: a certified scalar crosses with its endpoints.
                let ok = Interval::from_certified(Interval::from_bounds(1.0, 4.0).sqrt());
                assert_eq!((ok.lo(), ok.hi()), (1.0, 2.0));
            }

            /// Where a violated scalar would have to come FROM. Every
            /// scalar this lane hands to [`Interval::from_certified`]
            /// is either read straight off
            /// the stored body or built from it by `dot`, `norm`,
            /// `distance` and arithmetic — and none of those can
            /// manufacture a domain violation: a norm is the square root of
            /// a sum of squares, which is never partly negative, so it
            /// certifies even where it is zero and the vector degenerate.
            /// A `Trv` reaching the door therefore has to have been STORED in
            /// the body, not produced here. That is a property of the
            /// arithmetic, not of any guard, so it is pinned rather than
            /// assumed.
            #[test]
            fn the_lanes_own_arithmetic_cannot_manufacture_a_violation() {
                use geom_core::Vec3;
                let iv = geom_core::Interval::from_f64;
                for v in [
                    Vec3::new(iv(0.0), iv(0.0), iv(0.0)),
                    Vec3::new(iv(-3.0), iv(4.0), iv(0.0)),
                    Vec3::new(
                        geom_core::Interval::from_bounds(-1.0, 1.0),
                        iv(0.0),
                        iv(0.0),
                    ),
                ] {
                    assert!(
                        v.norm().certified_bracket().is_some(),
                        "a norm certified nothing for {v:?}"
                    );
                    assert!(Interval::from_certified(v.norm()).is_certified());
                }
            }

            /// The seam is per scalar, not per channel: one violated
            /// coefficient poisons its own slot and leaves the rest intact,
            /// so the poison reaches the flux algebra where it is visible.
            #[test]
            fn chan_poisons_only_the_violated_coefficient() {
                let one = Interval::from_f64(1.0);
                let c = chan(one, trv_pos(), one, one).expect("channel builds");
                assert!(!c.ca.is_certified(), "the violated coefficient survived");
                for (tag, r) in [("c0", c.c0), ("cb", c.cb), ("cl", c.cl)] {
                    assert!(r.is_certified(), "{tag} poisoned a certified coefficient");
                }
            }
        }
    }
}

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
            assert!(
                RECOURSE_VERBS.iter().any(|v| lower.contains(v)),
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
        crate::instance::graft_disjoint(&mut pair, &tall, tol).unwrap();
        let mut trio = skew.clone();
        crate::instance::graft_disjoint(&mut trio, &tall, tol).unwrap();
        crate::instance::graft_disjoint(&mut trio, &unit, tol).unwrap();
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
        ("skew", 0x4001_0d4f_df3b_645a, 0x4026_2907_4669_5750),
        ("tall", 0x3ff0_0000_0000_0000, 0x4021_0000_0000_0000),
        ("pair", 0x4000_0000_0000_0000, 0x402d_0000_0000_0000),
        ("trio", 0x4010_86a7_ef9d_b22d, 0x4039_9483_a334_aba8),
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
                    let (positive, part) = sign_walk(
                        &body,
                        &faces,
                        band,
                        tol,
                        Some(QuadLane::certified()),
                        |e: VolumeEnclosure<f64>| (e.volume_lo > 0.0).then_some(true),
                        |_| false,
                    )
                    .unwrap();
                    assert!(positive, "{name}: {solid:?} encloses positive volume");
                    part
                })
                .collect();
            let assembled =
                SignCertificate::assembled(&body, band, tol, Some(QuadLane::certified()), parts)
                .refine_to_target()
                .unwrap();
            let whole = crate::mass_properties(&body, tol).unwrap();
            assert_eq!(
                (
                    assembled.volume.to_bits(),
                    assembled.surface_area.to_bits(),
                    assembled.volume_pad.to_bits(),
                    assembled.area_pad.to_bits()
                ),
                (
                    whole.volume.to_bits(),
                    whole.surface_area.to_bits(),
                    whole.volume_pad.to_bits(),
                    whole.area_pad.to_bits()
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
