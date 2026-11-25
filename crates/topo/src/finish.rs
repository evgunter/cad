//! **The finished body** (`docs/DESIGN.md` `[topology.tiers]`, the
//! at-rest row of `[one-way.places]`): the scalar's
//! [`AtRestPolicy::finish`], the one place a body's verdict is derived,
//! the two types it constructs, and its refusal. A finished body moved
//! by an isometry stays finished ([`FinishedBody::moved`]): the move
//! carries its verdict and asks only the session box again.
//!
//! Each scalar names its finished type ([`AtRestPolicy::Finished`]),
//! the way [`geom_core::Real::Carries`] names what an edge carries. At
//! the scalars with certification rights (`f64`, `Probe`, `Interval`,
//! `Sym`) it is [`AtRestBody`], which only a certifying scalar can
//! construct. At a dual it is [`Replay`]: a body that passed tier 2
//! alone and carries no verdict, so nothing that reads a verdict
//! accepts one (DUAL-DESIGN DL3).
//!
//! The refusal is [`NotFinished`]: every finding the gate made, none
//! folded into another, each read through its family's one text and
//! recourse ([`FindingFamily`]).
//!
//! A sweep door's result is the scalar's **leaf**
//! ([`AtRestPolicy::Leaf`], [`LeafBody`]): its finished body wherever
//! the switch at [`AtRestPolicy::leaf`] finishes it, and a [`Replay`]
//! (tier 2 alone) at the box scalars, where a consumer that takes a
//! finished body finishes it ([`take`]).

use core::fmt;
use core::ops::Deref;
use std::sync::Arc;

use geom_core::{Real, Tol};

use crate::body::Body;
use crate::props::AtRestPolicy;
use crate::validate::{AtRestBody, ValidationError};

/// **What every scalar's finished type is**: a body read through
/// [`Deref`] and never borrowed mutably, handed back only by
/// [`FinishedBody::into_body`], which gives up the finish.
///
/// It names its scalar ([`FinishedBody::Scalar`]), whose finished type it
/// is, so a door generic over the finished body reads its scalar off
/// the operand it is handed.
pub trait FinishedBody:
    Deref<Target = Body<Self::Scalar>> + Clone + fmt::Debug + Send + Sync + 'static
{
    /// The scalar this is the finished type of.
    type Scalar: AtRestPolicy<Finished = Self>;

    /// The body, its finish given up: construction state again.
    #[must_use]
    fn into_body(self) -> Body<Self::Scalar>;

    /// **This finished body under an isometry**, finished: its data
    /// mapped, every verdict and certificate carried, and every family
    /// whose fact is not one of the shape up to isometry asked again
    /// ([`FindingFamily::up_to_isometry`]), which is the session box.
    ///
    /// # Errors
    ///
    /// [`MoveError::Map`] where the data map refuses ([`moved_body`]);
    /// [`MoveError::NotFinished`] holding the session box's finding.
    fn moved_by(&self, iso: &crate::Isometry<Self::Scalar>, tol: Tol) -> Result<Self, MoveError>;

    /// **Whether [`Self::moved_by`]'s ask reaches the node's verdict
    /// log** ([`moved_and_asked`]): at `AtRestBody` always, the finish
    /// carried and nothing else asking the image; at a replay as
    /// [`Replay::moved_by`] says.
    const MOVE_LOGS_ITS_ASK: bool;

    /// **This finished body under a rigid motion** ([`Self::moved_by`]).
    ///
    /// # Errors
    ///
    /// [`NotFinished`] holding the session box's finding. A motion's map
    /// refuses only a state a finished body never holds, returned typed
    /// as [`ValidationError::Unmovable`].
    fn moved(
        &self,
        motion: &geom_core::RigidMotion<Self::Scalar>,
        tol: Tol,
    ) -> Result<Self, NotFinished> {
        self.moved_by(&crate::Isometry::Proper(*motion), tol)
            .map_err(MoveError::under_a_motion)
    }

    /// **This finished body's mirror image** ([`Self::moved_by`]).
    ///
    /// # Errors
    ///
    /// [`MoveError::Map`] where the data map refuses;
    /// [`MoveError::NotFinished`] holding the session box's finding, and
    /// completion's refusal of a row the image derives again
    /// ([`crate::mirror`] drops the rows, [`complete_in_place`] derives
    /// them).
    fn mirrored(
        &self,
        reflection: &crate::Reflection<Self::Scalar>,
        tol: Tol,
    ) -> Result<Self, MoveError> {
        self.moved_by(&crate::Isometry::Improper(*reflection), tol)
    }
}

/// **Why a move refused**: the data map's own refusal, or the image's
/// finding where the move asks one ([`FinishedBody::moved_by`]).
#[derive(Debug)]
pub enum MoveError {
    /// The data map refused ([`moved_body`]): a state a finished body or
    /// a leaf never holds.
    Map(crate::TransformError),
    /// The image's finding: the session box, or a leaf's finish.
    NotFinished(NotFinished),
}

impl MoveError {
    /// **A rigid motion's refusal as findings**: its map refuses only a
    /// state a finished body or a leaf never holds (a motion re-mints
    /// no row), returned typed ([`ValidationError::Unmovable`]) rather
    /// than panicked on.
    #[must_use]
    pub fn under_a_motion(self) -> NotFinished {
        use crate::transform::TransformError as E;
        let found = match self {
            Self::NotFinished(findings) => return findings,
            Self::Map(E::NullScaffold { .. }) => "a null-scaffold edge",
            Self::Map(E::NurbsPlaceholder) => "a placeholder net",
            Self::Map(E::Corrupt { found }) => found,
        };
        NotFinished::new(vec![ValidationError::Unmovable { found }])
    }
}

impl fmt::Display for MoveError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Map(error) => write!(f, "{error}"),
            Self::NotFinished(findings) => write!(f, "{findings}"),
        }
    }
}

impl std::error::Error for MoveError {}

impl<T: AtRestPolicy<Finished = Self>> FinishedBody for AtRestBody<T> {
    type Scalar = T;
    const MOVE_LOGS_ITS_ASK: bool = true;

    fn into_body(self) -> Body<T> {
        AtRestBody::into_body(self)
    }

    fn moved_by(&self, iso: &crate::Isometry<T>, tol: Tol) -> Result<Self, MoveError> {
        AtRestBody::moved_by(self, iso, tol)
    }
}

/// **A body whose pcurve rows are complete**: the argument of a
/// scalar's verdict ([`AtRestPolicy::finish_completed`],
/// [`AtRestPolicy::leaf_completed`]), constructed only by the completion
/// [`AtRestPolicy::finish`] and [`AtRestPolicy::leaf`] run first. It
/// drops the rows on dead half-edges, keeps every stored row and derives
/// each missing one (`crate::pcurves`' completion), so no producer mints
/// to close and a stale row reaches the verdict to be refused. A face
/// whose rows do not complete is left as found, its refusal held for the
/// door to report beside the verdict's findings.
#[derive(Debug)]
pub struct Completed<T: Real> {
    body: Body<T>,
    refused: Vec<ValidationError>,
}

impl<T: Real> Completed<T> {
    /// The body, its rows complete.
    #[must_use]
    pub fn into_body(self) -> Body<T> {
        self.body
    }

    /// Completion's refusals, taken for the door to report.
    pub(crate) fn take_refused(&mut self) -> Vec<ValidationError> {
        core::mem::take(&mut self.refused)
    }
}

/// **`body` completed** ([`Completed`]): [`complete_in_place`], its
/// refusals held beside the body. Its derivations are a build's, not the
/// verdict's: they run before, and outside, the checking scope.
///
/// # Errors
///
/// [`NotFinished`] holding tiers 1 and 2's findings.
pub(crate) fn complete<T: AtRestPolicy>(
    mut body: Body<T>,
    tol: Tol,
) -> Result<Completed<T>, NotFinished> {
    let refused = complete_in_place(&mut body, tol)?;
    Ok(Completed { body, refused })
}

/// **A verdict with completion's refusals beside it**: each refusal the
/// verdict's own findings do not already hold, so a face tier 3 derives
/// again and refuses alike is reported once; at a replay, which runs no
/// tier 3, the refusals alone.
///
/// # Errors
///
/// `verdict`'s findings and the refusals, or the refusals alone.
pub(crate) fn with_refusals<F>(
    verdict: Result<F, NotFinished>,
    refused: Vec<ValidationError>,
) -> Result<F, NotFinished> {
    if refused.is_empty() {
        return verdict;
    }
    match verdict {
        Ok(_) => Err(NotFinished::new(refused)),
        Err(mut found) => {
            for r in refused {
                if !found.findings.contains(&r) {
                    found.findings.push(r);
                }
            }
            Err(found)
        }
    }
}

/// Tiers 1 and 2 over `body`, as a check.
fn closed<T: Real>(body: &Body<T>) -> Result<(), NotFinished> {
    geom_core::k_stats::checking(|| crate::validate::validate_closed(body))
        .map_err(NotFinished::new)
}

/// Completion's refusals as findings.
fn refusals(completed: Result<(), Vec<crate::pcurves::PcurveMintError>>) -> Vec<ValidationError> {
    completed
        .err()
        .unwrap_or_default()
        .into_iter()
        .map(|finding| ValidationError::Pcurve { finding })
        .collect()
}

/// **`body`'s rows completed in place**: the one home of completion
/// (`crate::pcurves`' completion), which [`complete`] — the finish's and
/// the leaf's — runs, and which runs early where a reader inside the
/// pipeline needs the rows: the result sort's ray probes
/// (`crate::pieces`, at the Boolean's gate and a split's sides), the
/// shell's cavity questions, and a mirror's image
/// ([`moved_and_asked`]). It keeps every stored row, so it launders
/// none, and the finish that follows finds nothing more to fill.
///
/// Tiers 1 and 2 are asked first, the one precondition: completion
/// walks every face's loops and derives from every edge's carrier, which
/// only a closed body of resolving records holds, and the verdict's own
/// first act would report the same findings.
///
/// Completion is lenient: a face whose rows do not complete is left as
/// found and its refusal returned, never thrown alone, so the door
/// reports it beside every other finding of the body
/// ([`with_refusals`], [`refused_finding_set`]).
///
/// # Errors
///
/// Tiers 1 and 2's findings.
pub(crate) fn complete_in_place<T: AtRestPolicy>(
    body: &mut Body<T>,
    tol: Tol,
) -> Result<Vec<ValidationError>, NotFinished> {
    closed(body)?;
    Ok(refusals(crate::pcurves::complete_pcurves(body, tol)))
}

/// **The finding set of a body whose completion refused**, for a door
/// that completed it early ([`complete_in_place`]): its finish's
/// findings, which hold `refused` beside the verdict's own, so the door
/// reports what the finish would of the same body. A reader that would
/// meet an unfilled face — the sort's probe crossing it reads no chart
/// polygon there, and its cavity questions no rows — is not asked it.
pub(crate) fn refused_finding_set<T: AtRestPolicy>(
    body: Body<T>,
    tol: Tol,
    refused: Vec<ValidationError>,
) -> NotFinished {
    match with_refusals(T::finish(body, tol).map(drop), refused.clone()) {
        Err(found) => found,
        Ok(()) => NotFinished::new(refused),
    }
}

/// What [`fed`] refused: the body's finding set where its completion
/// refused or tiers 1 and 2 did, or the reader's own refusal on a body
/// whose rows completed.
#[derive(Debug)]
pub(crate) enum Fed<E> {
    /// The finding set ([`refused_finding_set`]), or tiers 1 and 2's.
    NotFinished(NotFinished),
    /// The reader refused a body whose rows completed.
    Read(E),
}

/// **A reader inside the pipeline, fed `body`'s rows completed**
/// ([`complete_in_place`]): the result sort at the Boolean's gate and a
/// split's sides, and the shell's cavity questions. Where completion
/// refused, the door answers the body's finding set — the finish's
/// findings with the refusals beside them — whatever `read` answers: a
/// reader that meets an unfilled face reads no rows there (the sort's
/// probe crossing it refuses, naming one face and hiding the rest), so
/// the set is read off the body `read` left where it read it whole, and
/// off the body as completion left it where `read` refused.
///
/// # Errors
///
/// [`Fed::NotFinished`] holding the finding set, or tiers 1 and 2's;
/// [`Fed::Read`] holding `read`'s refusal of a completed body.
pub(crate) fn fed<T: AtRestPolicy, E>(
    body: &mut Body<T>,
    tol: Tol,
    read: impl FnOnce(&mut Body<T>) -> Result<(), E>,
) -> Result<(), Fed<E>> {
    let refused = complete_in_place(body, tol).map_err(Fed::NotFinished)?;
    if refused.is_empty() {
        return read(body).map_err(Fed::Read);
    }
    let unread = body.clone();
    let answered = match read(body) {
        Ok(()) => body.clone(),
        Err(_) => unread,
    };
    Err(Fed::NotFinished(refused_finding_set(
        answered, tol, refused,
    )))
}

/// **A body that passed tier 2 alone** ([`crate::validate_closed`]),
/// and carries no verdict.
///
/// # A tier-3 premise at a dual
///
/// A reader that states its premise as tier 3's ("tier 3 refuses it
/// first", "tier 3 places each edge on its carriers", "the operand is a
/// finished body") holds it at a certifying scalar, whose finished body
/// passed tier 3. At a dual the finished body is this replay, tier 2
/// alone (DUAL-DESIGN DL3), and the premise is held one level up: a dual
/// evaluates only bound to its build of record (`editor_core`'s
/// `evaluate_recorded`), which refuses every node that record built no
/// value at, inside each part as at the top. So an operand a dual's Boolean, split, blend or shell reads through the
/// evaluation service is the same bits a certifying finish validated, and
/// the readers that cite this section need no check of their own. That
/// no production caller reaches this crate's doors at a dual another way
/// is held by a gate, not by type (`scripts/gates/dual-doors-only-through-the-record.sh`);
/// the type-level form waits on a ruling
/// (`work/onecopy/topos-doors-at-a-dual-take-no-tier-3-evidence-by-type.md`).
///
/// At a dual it is the finished body: the base-scalar evaluation it
/// replays is the validation of record (DUAL-DESIGN DL3). At the box
/// scalars (`Interval`, `Sym`) it is a sweep door's leaf, not a
/// finished body ([`AtRestPolicy::leaf`]): no door that takes a
/// finished body accepts it, and the consumer that needs one finishes
/// it ([`take`]).
#[derive(Clone, Debug)]
pub struct Replay<T: Real> {
    body: Body<T>,
}

impl<T: Real> Replay<T> {
    /// Tier 2 over `body`, the replay kept where it passes.
    ///
    /// # Errors
    ///
    /// Tier 2's findings.
    pub(crate) fn of(body: Body<T>) -> Result<Self, NotFinished> {
        geom_core::k_stats::checking(|| crate::validate::validate_closed(&body))
            .map_err(NotFinished::new)?;
        Ok(Self { body })
    }

    /// The body, the replay given up.
    #[must_use]
    pub fn into_body(self) -> Body<T> {
        self.body
    }
}

impl<T: Real> Deref for Replay<T> {
    type Target = Body<T>;

    fn deref(&self) -> &Body<T> {
        &self.body
    }
}

impl<T: AtRestPolicy> Replay<T> {
    /// **This replay under an isometry**: its data mapped, and every
    /// family not up to isometry asked of the image
    /// ([`asked_after_isometry`]), the session box: a move asks the box
    /// whatever it moves. A replay carries no other verdict and its tier
    /// 2 is topology, which no map moves.
    ///
    /// **Two logs, and which one this ask reaches.** A node's VERDICT log
    /// (`geom_core::k_stats`' record of the decisions a node made, which a
    /// drive compares lane against lane) is not the decision counters the
    /// census pins read (the symbolic and numeric tallies per predicate,
    /// which count every decision made, logged or not). The ask always
    /// runs, so it always counts: the `+1 numeric` session-box decision per
    /// move the pins carry, whichever log it reaches. Where this scalar's taker runs tier 3
    /// ([`AtRestPolicy::TAKE_FINISHES`], the box scalars) it asks the box
    /// of the same image and logs it there, so the ask is held off this
    /// node's verdict log, as the `f64` and `Probe` leaf's moved finish is
    /// ([`AtRestPolicy::leaf`], `docs/predicate-dimension-audit.md` F34),
    /// and both lanes log the box once, at the taker. Where it does not
    /// (`f64`, `Probe` and a dual, whose taker runs no tier 3), nothing
    /// else would log it, so the ask is logged here. A refusal has no
    /// taker: its record is relogged here at every scalar.
    ///
    /// # Errors
    ///
    /// [`MoveError::Map`] where the data map refuses ([`moved_body`]);
    /// [`MoveError::NotFinished`] holding the session box's finding.
    pub fn moved_by(&self, iso: &crate::Isometry<T>, tol: Tol) -> Result<Self, MoveError> {
        let body = moved_and_asked(&self.body, iso, !T::TAKE_FINISHES, tol)?;
        Ok(Self { body })
    }
}

/// **Which kind of value a body is held as** ([`AtRestPolicy::Finished`]
/// or [`AtRestPolicy::Leaf`]): what decides whether its move's ask
/// reaches the verdict log ([`HeldKind::move_logs_its_ask`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum HeldKind {
    /// A finished body ([`FinishedBody`]).
    Finished,
    /// A sweep door's leaf ([`LeafBody`]).
    Leaf,
}

impl HeldKind {
    /// **Whether a move of a body held this way logs its ask** on the
    /// node's verdict log, at `T` ([`FinishedBody::MOVE_LOGS_ITS_ASK`],
    /// [`LeafBody::MOVE_LOGS_ITS_ASK`]).
    #[must_use]
    pub fn move_logs_its_ask<T: AtRestPolicy>(self) -> bool {
        match self {
            Self::Finished => <T::Finished as FinishedBody>::MOVE_LOGS_ITS_ASK,
            Self::Leaf => <T::Leaf as LeafBody>::MOVE_LOGS_ITS_ASK,
        }
    }
}

/// **The one move-and-ask**: `body`'s data under `iso` ([`moved_body`])
/// and every family not up to isometry asked of the image
/// ([`asked_after_isometry`]), the ask bracketed and relogged onto the
/// node's verdict log where `logged`, and on a refusal always (a refusal
/// has no taker to log it). Every move of a finished value, and every
/// graft of a placed copy ([`crate::boolean::graft_placed`]), asks
/// through it, each passing the rule of the kind it moves
/// ([`HeldKind::move_logs_its_ask`]).
///
/// # Errors
///
/// [`MoveError::Map`] where the data map refuses; [`MoveError::NotFinished`]
/// holding the session box's finding, or the band's.
pub(crate) fn moved_and_asked<T: AtRestPolicy>(
    body: &Body<T>,
    iso: &crate::Isometry<T>,
    logged: bool,
    tol: Tol,
) -> Result<Body<T>, MoveError> {
    let mut body = moved_body(body, iso).map_err(MoveError::Map)?;
    let refused = match iso {
        crate::Isometry::Improper(_) => {
            complete_in_place(&mut body, tol).map_err(MoveError::NotFinished)?
        }
        crate::Isometry::Proper(_) => Vec::new(),
    };
    let scope = geom_core::k_stats::Bracket::open();
    let asked = asked_after_isometry(&body, tol);
    let record = scope.finish();
    if asked.is_err() || !refused.is_empty() || logged {
        geom_core::k_stats::relog(&record);
    }
    with_refusals(asked, refused).map_err(MoveError::NotFinished)?;
    Ok(body)
}

/// **Every family not up to isometry, asked of `image`**
/// ([`FindingFamily::up_to_isometry`]): what a move asks again.
///
/// # Errors
///
/// [`NotFinished`] holding each such family's finding, or the band's.
pub(crate) fn asked_after_isometry<T: geom_core::Decide>(
    image: &Body<T>,
    tol: Tol,
) -> Result<(), NotFinished> {
    let band = geom_core::Band::linear(tol)
        .map_err(|error| NotFinished::new(vec![ValidationError::Band { error }]))?;
    let findings: Vec<ValidationError> = FindingFamily::ALL
        .into_iter()
        .filter_map(|family| geom_core::k_stats::checking(|| family.asked_again(image, band)).err())
        .collect();
    if findings.is_empty() {
        Ok(())
    } else {
        Err(NotFinished::new(findings))
    }
}

/// **Every family not up to isometry, asked of `own` where it stands
/// under `motion`** ([`FindingFamily::asked_at`]): what
/// [`asked_after_isometry`] asks of the moved image, asked of the
/// image's own data with no body moved. The door a copy is placed by
/// asks it of the copy's placement (`[placement.frame]`: a copy's world
/// placement is asked by the door that places it, not by the readers
/// that read it).
///
/// # Errors
///
/// [`MoveError::Map`] where the motion does not map a carrier;
/// [`MoveError::NotFinished`] holding each such family's finding, or the
/// band's.
pub fn session_box_at<T: geom_core::Decide>(
    own: &Body<T>,
    motion: &geom_core::RigidMotion<T>,
    tol: Tol,
) -> Result<(), MoveError> {
    let band = geom_core::Band::linear(tol).map_err(|error| {
        MoveError::NotFinished(NotFinished::new(vec![ValidationError::Band { error }]))
    })?;
    let mut findings = Vec::new();
    for family in FindingFamily::ALL {
        match family.asked_at(own, motion, band) {
            Ok(()) => {}
            Err(MoveError::NotFinished(found)) => findings.extend(found.into_findings()),
            Err(map) => return Err(map),
        }
    }
    if findings.is_empty() {
        Ok(())
    } else {
        Err(MoveError::NotFinished(NotFinished::new(findings)))
    }
}

/// **`body` under `iso`**, the reflection's reversal included and its
/// rows dropped ([`crate::mirror`]): the data map behind every `moved`
/// door. A reflected image's rows are derived again by the door that
/// takes it ([`moved_and_asked`], a leaf's finish).
///
/// # Errors
///
/// The map's own refusal: a state the gate refuses, which a finished
/// body or a leaf never holds.
pub(crate) fn moved_body<T: AtRestPolicy>(
    body: &Body<T>,
    iso: &crate::Isometry<T>,
) -> Result<Body<T>, crate::TransformError> {
    match iso {
        crate::Isometry::Proper(_) => crate::transform::map_isometry(body, iso),
        crate::Isometry::Improper(r) => crate::mirror::mirror_body(body, r),
    }
}

impl<T: AtRestPolicy<Finished = Self>> FinishedBody for Replay<T> {
    type Scalar = T;
    const MOVE_LOGS_ITS_ASK: bool = !T::TAKE_FINISHES;

    fn into_body(self) -> Body<T> {
        Replay::into_body(self)
    }

    fn moved_by(&self, iso: &crate::Isometry<T>, tol: Tol) -> Result<Self, MoveError> {
        Replay::moved_by(self, iso, tol)
    }
}

/// **What every scalar's leaf is** ([`AtRestPolicy::Leaf`]): a sweep
/// door's result, read through [`Deref`] like a finished body. A
/// consumer that takes a finished body takes it through the one taker
/// door, [`take`].
pub trait LeafBody:
    Deref<Target = Body<Self::Scalar>> + Clone + fmt::Debug + Send + Sync + 'static
{
    /// The scalar this is the leaf of.
    type Scalar: AtRestPolicy<Leaf = Self>;

    /// The body, construction state again.
    #[must_use]
    fn into_body(self) -> Body<Self::Scalar>;

    /// **This leaf under an isometry**, a leaf of the same kind: the
    /// image's leaf at `f64` and `Probe` ([`AtRestPolicy::leaf`] of the
    /// moved body, its finish held off the log), [`Replay::moved_by`] where
    /// it is a replay. While the box-scalar switch lives a leaf's held
    /// record must be what the box lane's taker decides of the same body,
    /// and that taker runs tier 3 on the image, so a moved leaf is
    /// finished again on its image; that re-finish goes with the switch,
    /// and a finished value's move ([`FinishedBody::moved`]) runs none.
    ///
    /// # Errors
    ///
    /// [`MoveError::Map`] where the data map refuses ([`moved_body`]);
    /// [`MoveError::NotFinished`] holding the image's leaf's findings,
    /// the session box among them.
    fn moved_by(&self, iso: &crate::Isometry<Self::Scalar>, tol: Tol) -> Result<Self, MoveError>;

    /// **Whether a move of this leaf logs its session-box ask** on the
    /// node's verdict log, where the move asks alone
    /// ([`moved_and_asked`]): at `f64` and `Probe` never, the image's
    /// leaf finish being held off the log ([`AtRestPolicy::leaf`]); at a
    /// replay as [`Replay::moved_by`] says.
    const MOVE_LOGS_ITS_ASK: bool;

    /// **This leaf under a rigid motion** ([`Self::moved_by`]).
    ///
    /// # Errors
    ///
    /// [`NotFinished`]: the image's leaf's findings, and a motion's map
    /// refusal as [`MoveError::under_a_motion`] states it.
    fn moved(
        &self,
        motion: &geom_core::RigidMotion<Self::Scalar>,
        tol: Tol,
    ) -> Result<Self, NotFinished> {
        self.moved_by(&crate::Isometry::Proper(*motion), tol)
            .map_err(MoveError::under_a_motion)
    }

    /// **This leaf's mirror image** ([`Self::moved_by`]).
    ///
    /// # Errors
    ///
    /// As [`Self::moved_by`].
    fn mirrored(
        &self,
        reflection: &crate::Reflection<Self::Scalar>,
        tol: Tol,
    ) -> Result<Self, MoveError> {
        self.moved_by(&crate::Isometry::Improper(*reflection), tol)
    }
}

impl<T: AtRestPolicy<Leaf = Self>> LeafBody for AtRestBody<T> {
    type Scalar = T;
    const MOVE_LOGS_ITS_ASK: bool = false;

    fn into_body(self) -> Body<T> {
        AtRestBody::into_body(self)
    }

    /// The image's data ([`moved_body`]) finished as a leaf
    /// ([`AtRestPolicy::leaf`]). It does not go through
    /// [`moved_and_asked`]: where the leaf is an `AtRestBody` (`f64`,
    /// `Probe`, the duals) the leaf is the full finish of the image,
    /// which asks every family, the session box among them, so the
    /// move-and-ask's ask would be asked twice. The finish holds its
    /// decisions off the log as the leaf's rule says
    /// ([`Self::MOVE_LOGS_ITS_ASK`]).
    fn moved_by(&self, iso: &crate::Isometry<T>, tol: Tol) -> Result<Self, MoveError> {
        T::leaf(moved_body(self, iso).map_err(MoveError::Map)?, tol).map_err(MoveError::NotFinished)
    }
}

impl<T: AtRestPolicy<Leaf = Self>> LeafBody for Replay<T> {
    type Scalar = T;
    const MOVE_LOGS_ITS_ASK: bool = !T::TAKE_FINISHES;

    fn into_body(self) -> Body<T> {
        Replay::into_body(self)
    }

    fn moved_by(&self, iso: &crate::Isometry<T>, tol: Tol) -> Result<Self, MoveError> {
        Replay::moved_by(self, iso, tol)
    }
}

/// **The one taker door** ([`AtRestPolicy::take`]): a scalar's leaf as
/// the finished body a door that takes finished bodies takes. The same
/// `Arc` where the leaf is the finished body (at `f64` and `Probe`, its
/// held finish record relogged here; at a dual, as it is); tier 3 run
/// here, logged, at the box scalars.
///
/// # Errors
///
/// [`NotFinished`]: the finish's findings, at the box scalars.
pub fn take<T: AtRestPolicy>(
    leaf: &Arc<T::Leaf>,
    tol: Tol,
) -> Result<Arc<T::Finished>, NotFinished> {
    T::take(leaf, tol)
}

#[cfg(debug_assertions)]
thread_local! {
    static FINISHES: std::cell::Cell<u64> = const { std::cell::Cell::new(0) };
}

/// One tier-3 run of a certifying scalar's finish
/// ([`AtRestPolicy::finish`], through [`AtRestBody`]'s constructor).
pub(crate) fn count_finish() {
    #[cfg(debug_assertions)]
    FINISHES.with(|n| n.set(n.get().saturating_add(1)));
}

/// **The tier-3 finishes run on this thread so far**: each run of a
/// certifying scalar's [`AtRestPolicy::finish`], refusals included,
/// wherever it was reached from (a door's own, a leaf's, a taker's).
///
/// `cfg(debug_assertions)`-gated, as `editor_core::gathers_on_this_thread`
/// is and for the same reason: this workspace's release profile keeps
/// debug assertions. A caller reads a DIFFERENCE across the operation it
/// asks about; the absolute value means nothing.
#[cfg(debug_assertions)]
#[must_use]
pub fn finishes_on_this_thread() -> u64 {
    FINISHES.with(std::cell::Cell::get)
}

/// **`body` finished at its scalar** ([`AtRestPolicy::finish`]), the
/// one spelling a caller outside the policy names.
///
/// # Errors
///
/// [`NotFinished`]: every finding the scalar's gate made.
pub fn finish<T: AtRestPolicy>(body: Body<T>, tol: Tol) -> Result<T::Finished, NotFinished> {
    T::finish(body, tol)
}

/// **Why a body is not a finished body**: the non-empty list of what
/// the scalar's gate found, every finding kept and none folded into
/// another. A door's error holds it whole in one variant and names only
/// its subject.
#[derive(Clone, Debug, PartialEq)]
pub struct NotFinished {
    findings: Vec<ValidationError>,
}

impl NotFinished {
    /// The gate's findings.
    ///
    /// # Panics
    ///
    /// On an empty list: a gate that refused found something.
    pub(crate) fn new(findings: Vec<ValidationError>) -> Self {
        assert!(
            !findings.is_empty(),
            "a gate refused a body and found nothing: every refusal names a finding"
        );
        Self { findings }
    }

    /// Every finding, in the order the gate made them; never empty.
    #[must_use]
    pub fn as_slice(&self) -> &[ValidationError] {
        &self.findings
    }

    /// A refusal over `findings`, for the rows that render a door's
    /// error without running its gate.
    ///
    /// # Panics
    ///
    /// On an empty list.
    #[cfg(any(test, feature = "test-support", feature = "sweep-testing"))]
    #[doc(hidden)]
    #[must_use]
    pub fn for_tests(findings: Vec<ValidationError>) -> Self {
        Self::new(findings)
    }

    /// The findings, the refusal given up.
    #[must_use]
    pub fn into_findings(self) -> Vec<ValidationError> {
        self.findings
    }

    /// Each family the findings fall in, in the order its first finding
    /// was made, with its findings in theirs.
    #[must_use]
    pub fn by_family(&self) -> Vec<(FindingFamily, Vec<&ValidationError>)> {
        let mut groups: Vec<(FindingFamily, Vec<&ValidationError>)> = Vec::new();
        for finding in &self.findings {
            let family = finding.family();
            match groups.iter_mut().find(|(f, _)| *f == family) {
                Some((_, members)) => members.push(finding),
                None => groups.push((family, vec![finding])),
            }
        }
        groups
    }
}

impl Deref for NotFinished {
    type Target = [ValidationError];

    /// Every finding, in the order the gate made them; never empty.
    fn deref(&self) -> &[ValidationError] {
        &self.findings
    }
}

impl PartialEq<Vec<ValidationError>> for NotFinished {
    fn eq(&self, other: &Vec<ValidationError>) -> bool {
        self.findings == *other
    }
}

impl<const N: usize> PartialEq<[ValidationError; N]> for NotFinished {
    fn eq(&self, other: &[ValidationError; N]) -> bool {
        self.findings[..] == other[..]
    }
}

impl NotFinished {
    /// Each family once, its text, then each of its findings, which ends
    /// in the family's recourse; each finding read at rest where the
    /// holder is read at a door (`at` is `Some`: a finding of the gate
    /// reported inside an operation's refusal,
    /// [`geom_brep::recourse::at_rest`]), its payload alone otherwise. A
    /// lone finding is written alone: its own sentence already says what
    /// its family's text would.
    fn write(
        &self,
        f: &mut fmt::Formatter<'_>,
        at: Option<geom_brep::recourse::ReadAt>,
    ) -> fmt::Result {
        use geom_brep::recourse::{at_rest, read_at};
        let at = at_rest(at);
        if let [only] = self.as_slice() {
            return write!(f, "{}", read_at(only, at));
        }
        for (i, (family, members)) in self.by_family().into_iter().enumerate() {
            if i > 0 {
                f.write_str("; ")?;
            }
            write!(f, "{} ({}): ", family.text(), members.len())?;
            for (j, finding) in members.into_iter().enumerate() {
                if j > 0 {
                    f.write_str(" | ")?;
                }
                write!(f, "{}", read_at(finding, at))?;
            }
        }
        Ok(())
    }
}

geom_brep::ended_by_write!(NotFinished);

impl std::error::Error for NotFinished {}

/// **The decision or check that made a finding** — the unit a
/// refusal's text and recourse are written once for
/// (`docs/DESIGN.md` `[topology.tiers]`, `[tolerance.recourse]` (i)).
///
/// A family's findings share its text ([`FindingFamily::text`]) and its
/// recourse. Where the recourse is one sentence whatever the finding
/// ([`FindingFamily::recourse`] answers it), [`ValidationError`]'s
/// display ends every member in it and no member writes it. Where the
/// family is one decision with a verdict (definite, or a margin in the
/// band), its recourse is that decision's ending, written once at the
/// decision and read off the finding's own verdict and margin, so the
/// margin rides the finding as data.
#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord, Hash)]
#[cfg_attr(any(test, feature = "test-support"), derive(strum::EnumIter))]
pub enum FindingFamily {
    /// The body reaches past the session box (D4 ¶4), so nothing
    /// geometric is read on it.
    SessionBox,
    /// The run's tolerance forms no band, so nothing geometric is read.
    Tolerance,
    /// Tier 1: the topology's own structure (links, orbits, ownership,
    /// provenance, construction records) is broken.
    Structure,
    /// Tier 2 and the transience fence: what a construction leaves
    /// behind before it comes to rest (an empty loop, a strut, a shell
    /// in pieces, a null edge or face, a scaffold description).
    Scaffolding,
    /// A stored description disagrees with the geometry it describes,
    /// or holds numbers that describe nothing.
    Description,
    /// A surface's stored size breaks its kind's convention (a ring
    /// torus, a datum in range): a size the user may intend.
    Convention,
    /// A certificate re-derived at rest failed, or the lane that derives
    /// it does not run at this scalar.
    Certificate,
    /// A flat face's vertices and edges stay on its plane (check 2).
    Planar,
    /// How the faces at an edge meet (check 4: the dihedral and the
    /// material wedge).
    Dihedral,
    /// Maximal edges (check 11): no vertex the join would take.
    Join,
    /// A face's sense and its region on its surface (check 6, its
    /// seam-region cover and its chart singularities).
    FaceSense,
    /// A face's loops bound one region (check 9 and one region per
    /// face).
    FaceRegion,
    /// Which side of the boundary is material (checks 7 and 10: the
    /// volume's sign, the shells' roles and winding).
    Orientation,
    /// The census of the body's contacts against its records (tier 3′).
    Census,
    /// The boundary is embedded off its records (tier 3′): no face
    /// through a wedge, no overlapping wedges, no self-crossing.
    Embedding,
    /// Instances interfere (the census's containment arm).
    Interference,
}

/// **A question a move asks again of its image** ([`FindingFamily::under_isometry`]).
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum AskedAgain {
    /// Whether the image lies in the session box.
    SessionBox,
}

impl FindingFamily {
    /// **What an isometry's image is asked again for this family**, the
    /// one place the partition is stated (`[topology.tiers]`): `None`
    /// where the family decides a fact of the shape up to isometry (a
    /// distance, an angle, a side or an incidence of the body with
    /// itself, or a fact of the run), so the image keeps the verdict; the
    /// question where it does not. Only the session box is asked again.
    /// A new family states its answer here.
    const fn under_isometry(self) -> Option<AskedAgain> {
        match self {
            Self::SessionBox => Some(AskedAgain::SessionBox),
            Self::Tolerance
            | Self::Structure
            | Self::Scaffolding
            | Self::Description
            | Self::Convention
            | Self::Certificate
            | Self::Planar
            | Self::Dihedral
            | Self::Join
            | Self::FaceSense
            | Self::FaceRegion
            | Self::Orientation
            | Self::Census
            | Self::Embedding
            | Self::Interference => None,
        }
    }

    /// **Whether this family decides a fact of the shape up to
    /// isometry** ([`Self::under_isometry`]): every family but the session
    /// box does, so an isometry's image keeps the verdict, and
    /// [`FinishedBody::moved_by`] asks again every family that does not.
    #[must_use]
    pub const fn up_to_isometry(self) -> bool {
        self.under_isometry().is_none()
    }

    /// **This family's question, asked of an isometry's image**
    /// ([`Self::under_isometry`]): its finding where it finds one, and
    /// nothing asked of a family up to isometry.
    pub(crate) fn asked_again<T: geom_core::Decide>(
        self,
        image: &Body<T>,
        band: geom_core::Band,
    ) -> Result<(), ValidationError> {
        match self.under_isometry() {
            None => Ok(()),
            Some(AskedAgain::SessionBox) => crate::session_box::in_session_box(image, band),
        }
    }

    /// **This family's question, asked of `own` where it stands under
    /// `motion`**, the body unmoved: what [`Self::asked_again`] asks of
    /// the image, read off the image's data as the placement holds it
    /// ([`crate::session_box::in_session_box_at`]).
    pub(crate) fn asked_at<T: geom_core::Decide>(
        self,
        own: &Body<T>,
        motion: &geom_core::RigidMotion<T>,
        band: geom_core::Band,
    ) -> Result<(), MoveError> {
        match self.under_isometry() {
            None => Ok(()),
            Some(AskedAgain::SessionBox) => {
                crate::session_box::in_session_box_at(own, motion, band)
            }
        }
    }

    /// Every family, in declaration order.
    pub const ALL: [Self; 16] = [
        Self::SessionBox,
        Self::Tolerance,
        Self::Structure,
        Self::Scaffolding,
        Self::Description,
        Self::Convention,
        Self::Certificate,
        Self::Planar,
        Self::Dihedral,
        Self::Join,
        Self::FaceSense,
        Self::FaceRegion,
        Self::Orientation,
        Self::Census,
        Self::Embedding,
        Self::Interference,
    ];

    /// The family's one text: what its findings say about the body.
    #[must_use]
    pub const fn text(self) -> &'static str {
        match self {
            Self::SessionBox => "the body reaches past the session box",
            Self::Tolerance => "the tolerance forms no band",
            Self::Structure => "the topology is broken",
            Self::Scaffolding => "construction scaffolding is left at rest",
            Self::Description => "a stored description does not describe its geometry",
            Self::Convention => "a surface breaks its kind's convention",
            Self::Certificate => "a certificate does not hold at rest",
            Self::Planar => "a flat face leaves its plane",
            Self::Dihedral => "faces meet badly at an edge",
            Self::Join => "edges are not joined",
            Self::FaceSense => "a face's sense or region on its surface is wrong",
            Self::FaceRegion => "a face's loops do not bound one region",
            Self::Orientation => "the material's side is wrong or unread",
            Self::Census => "a contact disagrees with its records",
            Self::Embedding => "the boundary meets itself off its records",
            Self::Interference => "instances interfere",
        }
    }

    /// The family's one recourse, where it is one sentence whatever the
    /// finding; `None` where the family is one decision whose ending
    /// reads the finding's verdict and margin.
    #[must_use]
    pub const fn recourse(self) -> Option<&'static str> {
        match self {
            Self::SessionBox => Some(crate::validate::SESSION_BOX),
            Self::Tolerance => Some(crate::validate::TOLERANCE),
            Self::Structure | Self::Scaffolding | Self::Description => {
                Some(geom_core::KERNEL_OR_FILE_DEFECT_ENDING)
            }
            Self::Convention
            | Self::Certificate
            | Self::Planar
            | Self::Dihedral
            | Self::Join
            | Self::FaceSense
            | Self::FaceRegion
            | Self::Orientation
            | Self::Census
            | Self::Embedding
            | Self::Interference => None,
        }
    }
}

impl ValidationError {
    /// **The family this finding belongs to** — the one place each
    /// finding's family is stated.
    #[must_use]
    pub const fn family(&self) -> FindingFamily {
        use FindingFamily as F;
        match self {
            Self::OutsideSessionBox { .. } => F::SessionBox,
            Self::Band { .. } => F::Tolerance,
            Self::DanglingTopology { .. }
            | Self::DanglingGeometry { .. }
            | Self::NextPrevMismatch { .. }
            | Self::LoopCycleOverrun { .. }
            | Self::ParentLoopMismatch { .. }
            | Self::UnreachableHalfEdge { .. }
            | Self::EdgeHalvesIdentical { .. }
            | Self::EdgeSlotBackpointerMismatch { .. }
            | Self::HalfEdgeUnclaimed { .. }
            | Self::HalfEdgeMultiplyClaimed { .. }
            | Self::EdgeNotAntiparallel { .. }
            | Self::EmanatingStartMismatch { .. }
            | Self::EmptyLoopVertexWithEmanating { .. }
            | Self::LoneVertexWithIncidence { .. }
            | Self::VertexOrbitOverrun { .. }
            | Self::OrbitForeignMember { .. }
            | Self::SplitVertexOrbit { .. }
            | Self::OuterListedAsRing { .. }
            | Self::BackPointerMismatch { .. }
            | Self::OrphanEntity { .. }
            | Self::MultiplyOwned { .. }
            | Self::OrphanGeometry { .. }
            | Self::Unmovable { .. }
            | Self::SolidWithoutShells { .. }
            | Self::ShellWithoutFaces { .. }
            | Self::EdgeAcrossShells { .. }
            | Self::MissingProvenance { .. }
            | Self::LeakedProvenance { .. }
            | Self::NullScaffoldShared { .. }
            | Self::LeakedNullFaceRecord { .. }
            | Self::StaleNullFaceLoop { .. }
            | Self::StaleNullFaceOwnership { .. } => F::Structure,
            Self::ScaffoldingEmptyLoop { .. }
            | Self::ScaffoldingStrutVertex { .. }
            | Self::ShellDisconnected { .. }
            | Self::NullEdgeAtRest { .. }
            | Self::NullFaceAtRest { .. }
            | Self::ScaffoldAtRest { .. } => F::Scaffolding,
            Self::DanglingDescription { .. }
            | Self::UncertifiableSurface { .. }
            | Self::PoisonedSurfaceDescription { .. }
            | Self::PoisonedSurfaceDatum { .. }
            | Self::PoisonedCurveDatum { .. }
            // The curve half of `UnrepresentableSurfaceDatum` sits here,
            // not in `Convention`: its sentence states the breach alone
            // and ends in this family's fixed recourse, where the
            // surface datum writes its own (`convention_recourse`, which
            // reads the datum's measure and the end it broke).
            | Self::UnrepresentableCurveDatum { .. }
            | Self::DescriptionNotAdjacent { .. }
            | Self::TransverseNotIntrinsic { .. }
            | Self::TangentNotIntrinsic { .. } => F::Description,
            Self::UnrepresentableSurfaceDatum { .. }
            | Self::SurfaceDatumUndecided { .. }
            | Self::CurveDatumUndecided { .. }
            | Self::DegenerateTorus { .. }
            | Self::DegenerateSpindle { .. }
            | Self::DegenerateTorusEscalated { .. } => F::Convention,
            Self::ApproxCertification { .. }
            | Self::ApproxLaneUnsupported { .. }
            | Self::EdgeCertification { .. }
            | Self::Pcurve { .. }
            | Self::VolumeUncomputable { .. } => F::Certificate,
            Self::PlanarFaceResidual { .. }
            | Self::PlanarFaceEscalated { .. }
            | Self::PlanarBoundaryResidual { .. }
            | Self::PlanarBoundaryEscalated { .. } => F::Planar,
            Self::SliverDihedral { .. }
            | Self::MixedDihedral { .. }
            | Self::NoDihedralArm { .. }
            | Self::LaminaWedge { .. } => F::Dihedral,
            Self::JoinableVertexAtRest { .. } | Self::JoinUndecidedAtRest { .. } => F::Join,
            Self::LoopRoleInverted { .. }
            | Self::CurvedSenseInverted { .. }
            | Self::SeamRegionOvercovered { .. }
            | Self::SeamRegionCoverUnresolved { .. }
            | Self::ChartSingularityNotAVertex { .. }
            | Self::SheetApexReached { .. }
            | Self::ChartSingularityUndecided { .. }
            | Self::WrapTipOffChartSingularity { .. }
            | Self::WrapTipWithoutChartSingularity { .. }
            | Self::WrapTipUndecided { .. } => F::FaceSense,
            Self::RingMeetsOuter { .. }
            | Self::RingContactEscalated { .. }
            | Self::RingOutsideOuter { .. }
            | Self::RingNestingUndecided { .. }
            | Self::CycleOffFaceSide { .. }
            | Self::CycleSideUndecided { .. }
            | Self::EssentialCyclesUnpaired { .. }
            | Self::RingMeetsRing { .. }
            | Self::RingPairContactEscalated { .. }
            | Self::PinchCornerCrossed { .. }
            | Self::PinchCornerEscalated { .. } => F::FaceRegion,
            Self::NegativeVolume { .. }
            | Self::SliverInsideOut { .. }
            | Self::VolumeSignUnresolved { .. }
            | Self::OrientationUndecided { .. }
            | Self::ShellWinding { .. }
            | Self::SolidOuterShells { .. }
            | Self::ShellRoleUndecided { .. } => F::Orientation,
            Self::UnrecordedContact { .. }
            | Self::StaleContactDeclaration { .. }
            | Self::ContactContradicted { .. }
            | Self::CensusEscalated { .. }
            | Self::CensusUnsupported { .. }
            | Self::CensusLaneUnsupported { .. }
            | Self::CensusUndecidable { .. }
            | Self::ArcEndOffCorner { .. }
            | Self::CuspEdgesMissing { .. }
            | Self::MaterialOnBothSides { .. } => F::Census,
            Self::FaceThroughEdgeWedge { .. }
            | Self::FaceThroughEdgeWedgeUndecided { .. }
            | Self::CoincidentWedgesOverlap { .. }
            | Self::CoincidentWedgesOverlapUndecided { .. }
            | Self::BoundaryCrossesItself { .. }
            | Self::BoundaryCrossesItselfUndecided { .. } => F::Embedding,
            Self::InstanceInterference { .. }
            | Self::SameSideInterference { .. }
            | Self::TouchInterference { .. } => F::Interference,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::FindingFamily;

    /// **`ALL` names every family once, in declaration order**: the
    /// move asks each family of `ALL` ([`super::asked_after_isometry`]), so
    /// a family missing from it would never be asked again.
    #[test]
    fn all_names_every_family_once_in_declaration_order() {
        let declared: Vec<FindingFamily> =
            <FindingFamily as strum::IntoEnumIterator>::iter().collect();
        assert_eq!(FindingFamily::ALL.to_vec(), declared);
    }

    /// **Only the session box is asked again of a moved body**.
    #[test]
    fn only_the_session_box_is_asked_again() {
        let asked: Vec<FindingFamily> = FindingFamily::ALL
            .into_iter()
            .filter(|family| !family.up_to_isometry())
            .collect();
        assert_eq!(asked, [FindingFamily::SessionBox]);
    }

    /// The `fn finish` / `fn leaf` items written inside an
    /// `impl … AtRestPolicy for …` in `code` (a `code_only` view), each
    /// as the impl's self type and the item's name; and how many such
    /// impls `code` holds.
    fn overrides(code: &str) -> (Vec<(String, &'static str)>, usize) {
        use test_utils::source::{balanced_end, impl_head, word_at};
        let mut found = Vec::new();
        let mut impls = 0;
        for (at, _) in code.match_indices("impl") {
            if !word_at(code, at, "impl") {
                continue;
            }
            let Some(open) = code[at..].find('{').map(|o| at + o) else {
                continue;
            };
            let Some(head) = impl_head(code, at, open) else {
                continue;
            };
            let names_the_trait = head
                .trait_path
                .as_deref()
                .is_some_and(|t| t.rsplit("::").next() == Some("AtRestPolicy"));
            if !names_the_trait {
                continue;
            }
            impls += 1;
            let end = balanced_end(code, open).unwrap_or(code.len());
            let body = &code[open..end];
            for name in ["finish", "leaf"] {
                let spelled = body.match_indices("fn").any(|(f, _)| {
                    word_at(body, f, "fn") && {
                        let rest = body[f + 2..].trim_start();
                        rest.starts_with(name)
                            && rest[name.len()..]
                                .chars()
                                .next()
                                .is_none_or(|c| !c.is_alphanumeric() && c != '_')
                    }
                });
                if spelled {
                    found.push((head.self_type.clone(), name));
                }
            }
        }
        (found, impls)
    }

    /// **No scalar overrides the completing finish**: `AtRestPolicy::finish`
    /// and `::leaf` are provided methods that complete the body and then
    /// ask the required `finish_completed` / `leaf_completed`, and only
    /// completion constructs their `Completed` argument — but a provided
    /// method is overridable, so an impl that wrote its own `fn finish`
    /// would hand its verdict an uncompleted body. This row refuses one,
    /// in every crate's `src/` (the trait is public).
    #[test]
    #[allow(clippy::expect_used)] // a guard's failure mechanism
    fn no_at_rest_policy_impl_writes_its_own_finish_or_leaf() {
        let root = test_utils::source::repo_root(env!("CARGO_MANIFEST_DIR")).join("crates");
        let mut found = Vec::new();
        let mut impls = 0;
        for krate in std::fs::read_dir(&root).expect("the crates directory") {
            let src = krate.expect("a crate entry").path().join("src");
            if !src.is_dir() {
                continue;
            }
            for file in test_utils::source::rust_sources(&src) {
                let text = std::fs::read_to_string(&file).expect("a readable source file");
                let code = test_utils::source::code_only(&text);
                let (here, n) = overrides(&code);
                impls += n;
                found.extend(
                    here.into_iter()
                        .map(|(ty, name)| format!("{file:?}: {ty}::{name}")),
                );
            }
        }
        // f64, Probe, Interval, Sym and Dual: a walk that finds fewer
        // read the wrong tree.
        assert!(impls >= 5, "found {impls} impl(s) of AtRestPolicy");
        assert!(
            found.is_empty(),
            "an impl overrides the completing finish: {found:?}"
        );
    }

    /// The row's reader goes red on an override and stays green on the
    /// `_completed` pair and on another trait's `finish`.
    #[test]
    fn the_override_reader_reads_an_override() {
        let code = test_utils::source::code_only(
            "impl AtRestPolicy for f64 { fn finish_completed() {} fn leaf (b: B) {} }\n\
             impl crate::props::AtRestPolicy for Probe { fn finish<'a>() {} }\n\
             impl Other for f64 { fn finish() {} }",
        );
        let (found, impls) = overrides(&code);
        assert_eq!(impls, 2);
        assert_eq!(
            found,
            [("f64".to_string(), "leaf"), ("Probe".to_string(), "finish")]
        );
    }
}
