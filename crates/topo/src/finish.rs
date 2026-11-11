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
    /// [`MoveError::Map`] where a re-minted row refuses ([`crate::mirror`]);
    /// [`MoveError::NotFinished`] holding the session box's finding.
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
    /// The data map refused ([`moved_body`]): under a reflection, a
    /// re-minted row ([`crate::mirror`]); otherwise a state a finished
    /// body or a leaf never holds.
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
            Self::Map(E::Pcurve { .. }) => "a re-minted pcurve row",
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

    fn into_body(self) -> Body<T> {
        AtRestBody::into_body(self)
    }

    fn moved_by(&self, iso: &crate::Isometry<T>, tol: Tol) -> Result<Self, MoveError> {
        AtRestBody::moved_by(self, iso, tol)
    }
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
        crate::validate::validate_closed(&body).map_err(NotFinished::new)?;
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
    /// # Errors
    ///
    /// [`MoveError::Map`] where the data map refuses ([`moved_body`]);
    /// [`MoveError::NotFinished`] holding the session box's finding.
    pub fn moved_by(&self, iso: &crate::Isometry<T>, tol: Tol) -> Result<Self, MoveError> {
        let body = moved_body(&self.body, iso, tol).map_err(MoveError::Map)?;
        asked_after_isometry(&body, tol).map_err(MoveError::NotFinished)?;
        Ok(Self { body })
    }
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
        .filter_map(|family| family.asked_again(image, band).err())
        .collect();
    if findings.is_empty() {
        Ok(())
    } else {
        Err(NotFinished::new(findings))
    }
}

/// **`body` under `iso`**, the reflection's reversal and re-minted rows
/// included ([`crate::mirror`]): the data map behind every `moved` door.
///
/// # Errors
///
/// The map's own refusal: under a reflection a re-minted row's
/// ([`crate::TransformError::Pcurve`]); otherwise a state the gate
/// refuses, which a finished body or a leaf never holds.
pub(crate) fn moved_body<T: AtRestPolicy>(
    body: &Body<T>,
    iso: &crate::Isometry<T>,
    tol: Tol,
) -> Result<Body<T>, crate::TransformError> {
    match iso {
        crate::Isometry::Proper(_) => crate::transform::map_isometry(body, iso),
        crate::Isometry::Improper(r) => crate::mirror::mirror_body(body, r, tol),
    }
}

impl<T: AtRestPolicy<Finished = Self>> FinishedBody for Replay<T> {
    type Scalar = T;

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

    fn into_body(self) -> Body<T> {
        AtRestBody::into_body(self)
    }

    fn moved_by(&self, iso: &crate::Isometry<T>, tol: Tol) -> Result<Self, MoveError> {
        T::leaf(moved_body(self, iso, tol).map_err(MoveError::Map)?, tol)
            .map_err(MoveError::NotFinished)
    }
}

impl<T: AtRestPolicy<Leaf = Self>> LeafBody for Replay<T> {
    type Scalar = T;

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
            | Self::ComponentEulerViolation { .. }
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
}
