//! The per-frame policies the viewport runs — as values, so they are
//! replayable.
//!
//! # Why these live here and not in the frame loop
//!
//! Three decisions used to sit inside `app::ViewerBehavior::viewport_ui`
//! and `ViewerApp::perform_batch`: when a batch of operations clears the
//! status line, when the id pass is asked a question, and when the two
//! picking paths are reported as disagreeing. All three are invariants,
//! and all three lived in `app`-gated code no test can execute — so the
//! crate's own claim that "everything between event conversion and
//! painting is exercised by `tests/`" was false about exactly the rules
//! most likely to be wrong.
//!
//! Each is a pure function or a small value with typed steps here. The
//! frame loop still decides WHEN to call them; it no longer decides what
//! they mean.
//!
//! Module kind: **vocabulary** — it names no driver type and no
//! `app`-only crate (`crates/viewer/README.md`, Module boundaries).
//!
//! # Two questions, and they are not the same question
//!
//! Something the chrome has to say is sorted twice, and the two sorts
//! are INDEPENDENT.
//!
//! **Which channel it goes to.** A [`Badge`] is a READ OF HELD STATE
//! a reader consults; a [`Message`] on the status line is the OUTCOME
//! OF SOMETHING THAT JUST HAPPENED. That is the whole test, and the
//! door makes it mechanical: a badge is a function of state the
//! application HOLDS, so it is recomputed on the frame it is drawn and
//! ends when that state ends, while a door whose input includes the
//! frame's own EVENTS is reporting an outcome and belongs on the line.
//! [`unindexed_refusal`] is where that boundary is visible — it takes
//! this frame's pick stream, and a click that got no answer is an
//! outcome even though what it reports is seam state.
//!
//! A badge therefore outlives the frame that raised it while the line
//! carries one frame's news. That lifetime is the CONSEQUENCE of the
//! test, not the test: it is what the split reads like from outside,
//! and it cannot sort a fact that is both true after its frame and
//! provoked by one.
//!
//! **What retires it.** Both channels carry a [`Subject`] — the
//! recurring event stream whose next event makes the thing the wrong
//! answer — and carrying one never decided which channel a fact goes
//! to. What differs is the ENFORCEMENT. A message is STORED, so
//! retiring it is bookkeeping: [`apply`] matches the held message's
//! subject against a [`StatusUpdate::Expire`] and drops it, and
//! [`StatusUpdate::Clear`] sweeps the line whole. A badge is stored
//! nowhere and nothing retires it; its subject names the event that
//! changes the state it reads, and the badge ends because the read
//! does.
//!
//! So [`projection_badge`] is a badge — a read of the camera and the
//! viewport, true on every frame until a projection can be formed —
//! that has the subject [`Subject::Camera`]. Those two answers are not
//! rivals, and one seam's two doors cannot disagree about the second
//! one: [`SeamSubject`] is where a seam's subject is stated, once, at
//! the type of the refusal.
//!
//! # The line: news, ranked
//!
//! Every sentence on the line is something that HAPPENED — an action
//! the document refused, a pick a tool declined, a dialog that could
//! not open — and which of a frame's news SHOULD win is
//! [`frame_status`]'s ranking. What stops it being the news is an
//! event about its subject: a camera verdict goes on the next camera
//! event, what the cursor said on the next cursor move, and what the
//! document said on the next act the document accepts. That last is
//! [`StatusUpdate::Clear`], which sweeps the whole line because an
//! accepted act makes every standing complaint stale; the other two
//! are [`StatusUpdate::Expire`], which retires one subject and leaves
//! the rest alone. Before this rule the only sweeper was `Clear`, so
//! refusing a camera move and then orbiting left the refusal on the
//! line for as long as the user navigated: navigation acts on nothing.
//!
//! **It does not yet reach the line through the ranking for every
//! writer.** Writers in this crate assign the field outright rather
//! than answering [`frame_status`], so a message a pane wrote is still
//! erased by that frame's [`StatusUpdate::Clear`], which runs after
//! the panes have drawn; two more answer in this vocabulary and apply
//! it without asking the ranking ([`fold_status`] and
//! [`cursor_status`], both at `pane::viewport`). Each names its
//! subject — [`Message`] is the only spelling there is — but naming a
//! subject is not asking the ranking; routing them through it is
//! tracked as its own item, not asserted here as done.
//!
//! # The toolbar: held state, read
//!
//! A badge is a function of the typed value it reads, so each one's
//! SILENCE is a row a test can write; each states its own [`Tone`],
//! which is the actionable-or-not rule the toolbar used to pick a
//! colour for at four call sites; and one draw at the toolbar consumes
//! them all. The members are the at-rest verdict ([`at_rest_badge`]),
//! the advisory checks ([`checks_badge`]), the δ the display budget
//! chose ([`delta_badge`]), the product fault ([`product_badge`]), and
//! the three display seams that hold a refusal — the scene
//! ([`scene_badge`]), the pick index ([`index_badge`]) and the
//! projection ([`projection_badge`]).
//!
//! Two rules follow, and both are values here rather than conditions
//! at a call site. [`fold_status`] never CLEARS for a camera fold:
//! clearing is the acting batch's verdict alone ([`batch_status`]),
//! because an action the document accepted is the one event that makes
//! a standing complaint stale, and a fold that cleared would be
//! deciding the fate of messages written by everyone else in the same
//! frame. And the gather's verdict badges rather than writes, because
//! a fault about the document on screen outlives every frame the
//! camera moves in.

use pncad::document::{ChecksReport, ParamName, ParseError, ProductError, RecipeNodeId, SlotId};
use pncad::prelude::StableName;

use crate::camera::CameraError;
use crate::camera::Folded;
use crate::display::{DisplayFault, Withdrawn};
use crate::evalseam::Generation;
use crate::pick::{IdMap, NotIndexed, PickError, PickIndex, PickIndexError};
use crate::prefs::StoreError;
use crate::scene::FittedDelta;
use crate::scene::SceneError;
use crate::session::{AtRestBadge, Refusal, SessionOp};

/// **What something the chrome shows is ABOUT** — carried by a
/// [`Message`] on the line and by a [`Badge`] on the toolbar alike.
///
/// A fact does not stop being TRUE. A camera refusal is still an
/// accurate report of a move that was refused, five hundred frames of
/// orbiting later; what has changed is that the user has asked the
/// camera five hundred further questions since, and the chrome is
/// answering the wrong one. So a fact names its subject, and an EVENT
/// about that subject retires it.
///
/// **The subject is chosen by the event that retires it**, never by
/// which module wrote the sentence and never by which channel carries
/// it. That is what makes this a rule and not five special cases: each
/// variant below names a recurring event stream, and a fact is about
/// whichever stream's next event makes it the wrong answer.
///
/// **The two channels retire it differently, and that is the whole of
/// the difference.** A message is held in a field, so [`apply`] has to
/// be told: [`StatusUpdate::Expire`] names the subject and drops what
/// the line holds about it. A badge is held nowhere — it is recomputed
/// from the state it reads on the frame it is drawn — so its subject
/// names the event that changes that state, and nothing has to act on
/// it for the badge to go.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Subject {
    /// **The camera and where it is pointed** — retired by the next
    /// camera event, whatever that event says. A refused move and a
    /// projection that could not be formed are both about the camera,
    /// and the fold that follows is the user asking again.
    ///
    /// Issued by [`fold_status`], on every clean fold — for the
    /// LINE. The projection is a badge ([`projection_badge`]) and
    /// needs no issuer: it is read from the camera, so a camera that
    /// projects is a camera whose badge is gone. Before it was a badge
    /// the clean fold's `Expire` retired the projection sentence on
    /// the next move whether or not a projection could be formed yet,
    /// which is the line going quiet about a picture it still cannot
    /// draw.
    Camera,
    /// **The cursor and what lies under it** — retired by the next
    /// cursor move, and by the pointer leaving the pane.
    ///
    /// Issued by [`cursor_status`], off the id pass's own bookkeeping:
    /// a message about what was under the cursor is stale exactly when
    /// the outstanding pick question is, which is a judgement
    /// [`IdQueryLog`] already makes.
    Cursor,
    /// **The document on screen and the acts aimed at it** — retired
    /// by the next act the document ACCEPTS.
    ///
    /// **No [`StatusUpdate::Expire`] issuer** — see the note below,
    /// which this shares with [`Self::Display`] and
    /// [`Self::Preferences`]. What sweeps it today is
    /// [`StatusUpdate::Clear`], and `Clear` is not this subject's
    /// event in any sense a type can check: it sweeps the whole line,
    /// a `Camera` message as readily as this one, because an act the
    /// document accepted makes every standing complaint stale
    /// ([`batch_status`]).
    Document,
    /// **The picture drawn from the document** — its δ, its scene, its
    /// pick index — retired by the next rebuild of the thing the
    /// message is about: the δ the display accepts next, the scene
    /// that lands, the index build that finishes.
    ///
    /// **No [`StatusUpdate::Expire`] issuer**, and the facts that
    /// wanted one are no longer on the line: the display seams HOLD
    /// their refusals, which is what makes them reads of held state,
    /// so the scene, the pick index and the δ the budget chose all
    /// badge ([`scene_badge`], [`index_badge`], [`delta_badge`]) and
    /// retire themselves. What still wears this subject on the line is
    /// news about the picture that an event provoked — a δ the user
    /// typed, a pick the user aimed at an index that is not there —
    /// and for those `Clear` is the only sweeper there is.
    Display,
    /// **The viewer's own settings and the file they are kept in** —
    /// retired by the next write of that file.
    ///
    /// **No [`StatusUpdate::Expire`] issuer**, for [`Self::Display`]'s
    /// reason.
    Preferences,
}

/// **Three of the five subjects are observationally identical on the
/// LINE today**, and saying so is part of the vocabulary rather than a
/// caveat on it. It is a statement about messages: a badge is retired
/// by the state it reads changing and asks no issuer for anything.
///
/// [`Subject::Camera`] and [`Subject::Cursor`] have
/// [`StatusUpdate::Expire`] issuers ([`fold_status`] and
/// [`cursor_status`]), so a message wearing either is retired by an
/// event and a row can see the difference. [`Subject::Document`],
/// [`Subject::Display`] and [`Subject::Preferences`] have none, so
/// nothing yet distinguishes them: each is swept by
/// [`StatusUpdate::Clear`], which is subject-blind, and by nothing
/// else.
///
/// **What each of those three states is therefore a claim about its
/// FUTURE issuer, not about behaviour today** — the event that would
/// retire it once someone marks that event. They are three names
/// because they name three different events, and the alternative
/// (one name for "swept only by `Clear`") would have to be renamed
/// three ways the first time any of them grew an issuer.
pub const SUBJECTS_WITH_AN_EXPIRY_ISSUER: [Subject; 2] = [Subject::Camera, Subject::Cursor];

/// **One frame's news**: what it is about, and its own words.
///
/// The text is composed by whoever raised it, from the typed value
/// that failed — nothing here writes prose about someone else's
/// failure. What this type adds is the half a `String` could not
/// carry: which recurring event makes the sentence the wrong answer.
/// **The fields are private and [`Message::new`] is the only door**,
/// for [`Badge`]'s reason: a struct literal is a second way to build
/// one, and a value whose whole point is that a decision was made in
/// one place must not have a spelling that skips it.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Message {
    subject: Subject,
    text: String,
}

impl Message {
    /// A message about `subject`, in `text`'s own words.
    pub fn new(subject: Subject, text: impl Into<String>) -> Self {
        Self {
            subject,
            text: text.into(),
        }
    }

    /// What the message is about, and so what retires it.
    pub fn subject(&self) -> Subject {
        self.subject
    }

    /// The sentence shown on the line.
    pub fn text(&self) -> &str {
        &self.text
    }
}

impl core::fmt::Display for Message {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        f.write_str(&self.text)
    }
}

/// What a frame's events should do to the status line.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum StatusUpdate {
    /// Leave the line as it is.
    Keep,
    /// Clear it: the user acted and nothing refused.
    Clear,
    /// **An event about `Subject` happened and had nothing to say.**
    /// Whatever the line holds about that subject is now the answer to
    /// a question nobody is asking, and goes; a message about anything
    /// else is untouched.
    ///
    /// This is the whole difference between [`Self::Keep`] and a
    /// [`Self::Clear`] that would be far too broad: a clean camera
    /// fold must retire the camera refusal it wrote a moment ago
    /// without deciding the fate of sentences written by writers it
    /// knows nothing about ([`fold_status`]).
    Expire(Subject),
    /// Show this message, replacing whatever the line held.
    Show(Message),
}

/// **Apply a verdict to the status line**: the one place a
/// [`StatusUpdate`] becomes the field it describes.
///
/// Every policy in this module answers in this vocabulary and every
/// consumer applies it here, so [`StatusUpdate::Keep`] is spelled as a
/// decision rather than as the absence of one. A writer that assigns
/// the `Option<Message>` itself has no way to say "I have nothing to
/// add", and the natural-looking spelling of it — assigning what it
/// would have shown — writes `None` over whatever another writer in
/// the same frame put there.
///
/// [`StatusUpdate::Expire`] is the one arm that reads the line before
/// writing it, and it is why the field is an `Option<Message>` and not
/// an `Option<String>`: retiring a message requires knowing what the
/// message was about.
pub fn apply(status: &mut Option<Message>, update: StatusUpdate) {
    match update {
        StatusUpdate::Keep => {}
        StatusUpdate::Clear => *status = None,
        StatusUpdate::Expire(subject) => {
            if status
                .as_ref()
                .is_some_and(|held| held.subject() == subject)
            {
                *status = None;
            }
        }
        StatusUpdate::Show(message) => *status = Some(message),
    }
}

/// Whether an operation counts as an ACTION on the document, for the
/// status line's purpose.
///
/// **Hover does not.** The clear-on-a-clean-batch rule is about the
/// user having tried something that worked, so the last complaint is
/// stale. Moving the pointer is not that: it emits an operation on
/// every frame the cursor changes what it is over, refuses nothing by
/// construction, and left unfiltered it wipes the ratified
/// expression-driven affordance off the screen the instant the mouse
/// drifts over the viewport.
pub fn acts(op: &SessionOp) -> bool {
    !matches!(op, SessionOp::Hover(_))
}

/// The status line after a batch: the refusal worth showing, or the
/// verdict that the line should be cleared or left alone.
///
/// A refusal always shows, even from a hover-only batch — a hover
/// cannot refuse today, and if one ever does, silence is the wrong
/// answer.
pub fn batch_status(ops: &[SessionOp], refusal: Option<&Refusal>) -> StatusUpdate {
    match (ops.iter().any(acts), refusal) {
        // A refusal is the document's answer to the act it was asked
        // for, so its subject is the document: it stops being the news
        // when the document accepts one.
        (_, Some(refusal)) => {
            StatusUpdate::Show(Message::new(Subject::Document, refusal.to_string()))
        }
        (true, None) => StatusUpdate::Clear,
        (false, None) => StatusUpdate::Keep,
    }
}

/// **The status line after a whole FRAME**: what the open tool said
/// about the frame's picks, composed with what the batch of operations
/// did.
///
/// # Why this composition has to exist
///
/// A tool notice and a batch verdict are produced by the SAME frame,
/// from the SAME ops, and they disagree by construction. A pick the
/// blend tool declines is still a `Select` that the session performs
/// cleanly — so [`batch_status`] sees an acting op and no refusal,
/// answers [`StatusUpdate::Clear`], and wipes the notice that was
/// written a few lines earlier. The user's mis-aimed click moved the
/// selection to another body and the sentence explaining why it did
/// not join the blend was on screen for zero frames.
///
/// Every other notice path survived only because nothing else in its
/// frame acted: a survival drop happens on a document change the user
/// did not click for. That is luck, not a rule, so the rule is here.
///
/// # The ranking
///
/// 1. A **refusal** wins, alone. It is the answer to the action the
///    user asked the DOCUMENT for, and it is the louder of the two.
/// 2. Else **every notice the frame produced**, in the order they
///    happened, joined with the separator the preferences path already
///    joins its startup notices with. Not the last one: assigning
///    `status` from each in turn keeps the last and loses the rest,
///    which is the same keep-last defect [`batch_status`] exists to
///    stop for refusals. Not the first one either — a frame CAN drop
///    two picks (a seated tool has two seats), and both drops are news.
/// 3. Else the batch's own verdict — [`StatusUpdate::Clear`] for a
///    clean acting batch, [`StatusUpdate::Keep`] otherwise.
///
/// Joining is a SEPARATOR, not a composed sentence: each notice is
/// still its own typed value's own rendering, which is what the error
/// micro-decision asks. Nothing here writes prose about someone else's
/// failure.
pub fn frame_status(
    notices: &[Message],
    ops: &[SessionOp],
    refusal: Option<&Refusal>,
) -> StatusUpdate {
    match batch_status(ops, refusal) {
        refused @ StatusUpdate::Show(_) => refused,
        verdict if notices.is_empty() => verdict,
        _ => StatusUpdate::Show(Message::new(
            joined_subject(notices),
            notices
                .iter()
                .map(Message::text)
                .collect::<Vec<_>>()
                .join(NOTICE_SEPARATOR),
        )),
    }
}

/// **What a joined rank-2 line is about**: the subject the frame's
/// notices SHARE, or [`Subject::Document`] when they do not.
///
/// One line holds one message, so joining several notices produces one
/// sentence that has to name one subject. When they agree there is
/// nothing to decide. When they do not, no single recurring event can
/// retire a sentence that is about several things at once — expiring
/// the whole line on a cursor move because one of its three clauses was
/// about the cursor would delete the other two — so the answer is the
/// subject whose retiring event is the broad one the line already has:
/// an act the document accepted, which sweeps everything
/// ([`Subject::Document`]).
///
/// **Every notice a frame produces today agrees**, and agrees on
/// `Document`: a tool's declined pick, a tool's survival drop, a
/// supersession and a dropped hide are all provoked by the frame's own
/// document transition. So the disagreeing case is reachable only by a
/// writer that does not exist yet, and this is the rule it will meet
/// rather than a fallback it will discover.
fn joined_subject(notices: &[Message]) -> Subject {
    let mut subjects = notices.iter().map(|notice| notice.subject());
    match subjects.next() {
        Some(first) if subjects.all(|subject| subject == first) => first,
        _ => Subject::Document,
    }
}

/// What several notices in one frame are joined with — one spelling,
/// shared with the preferences path's startup notices so the status
/// line reads the same however many things it is carrying.
pub const NOTICE_SEPARATOR: &str = "; ";

/// **What an accepted edit WITHDREW from the display state**, as a
/// notice for [`frame_status`]'s rank 2.
///
/// # One value, not two functions
///
/// A supersession and a dropped hide are the same class of fact —
/// display state an accepted edit took away, each carrying the
/// [`DisplayFault`] the prune withdrew it on — and they were two free
/// functions composing prose that differed in four format literals.
/// They are a typed value with a `Display` here, which is the shape
/// the crate's other notices already have (`tools::ToolNotice`,
/// `prefs::Notice`) and the shape `tree::RowStatus` is the model for:
/// the payload stays separate from its rendering, and the count-and-join
/// scaffolding is written once.
///
/// `None` for an empty withdrawal set, which is the `None` decision
/// held in one place rather than at each caller.
///
/// # Why the line and not a badge
///
/// It is NEWS by this module's test. It HAPPENED on the frame that
/// carries it, provoked by the act the user just took — the mate that
/// landed on their probed instance, the delete that took it, the redo
/// that stepped forward over the mate again.
///
/// Its subject is [`Subject::Document`], so the event that retires it
/// is the next act the document accepts — which the line already
/// spells [`StatusUpdate::Clear`]. **That is a weaker lifetime than
/// the argument above wants**, and the difference is stated rather
/// than papered over: the fact is true of nothing after its own frame,
/// while the sentence about it survives navigation and is retired by
/// the next accepted edit. A one-frame sentence would be unreadable at
/// sixty frames a second, so the frame is not a subject a reader can
/// use; what the vocabulary buys here is that the lifetime is now
/// STATED and implemented, and the residue is
/// `work/view/a-supersession-outlives-its-own-frame.md`.
///
/// It reaches the line through the frame's NOTICES rather than by
/// assignment, for the reason [`frame_status`] states: the transition
/// that withdraws is an edit the document accepted, so the same
/// frame's batch verdict is [`StatusUpdate::Clear`].
///
/// A refusal in the same frame outranks it and it is then not shown,
/// which rank 1 already says. The two cannot come from one operation:
/// a refused op returns before the prune that fills these lists.
///
/// # The cause is the fault's own sentence
///
/// **Nothing here composes prose about why a placement or a hide
/// went.** Each entry carries the [`DisplayFault`] the prune discarded
/// on, and this renders it through its own `Display` — the rule the
/// rest of the crate follows. So the commonest arm names the mates and
/// the remedy (`MateConstrained`: *delete the mate(s) if free relative
/// motion is intended*), a fuse names the product and the instances
/// fused into it, and a deleted instance says the document does not
/// hold the node — which is the delete arm's whole point, since the id
/// alone names something the tree no longer draws without saying that
/// is why.
///
/// The frame around the faults counts and does not name: every fault
/// [`crate::display::DisplayState::prune`] can put here names its own
/// SUBJECT — the four arms `free_move_check` and `display_check`
/// answer with — so naming the id again in the preamble would say it
/// twice. It does not promise a vocabulary for that subject: three of
/// those four say "instance N" and the absent-node arm says "node N",
/// which is `DisplayFault`'s own rule and the only honest wording
/// there.
///
/// The other three `DisplayFault` arms name no id at all. They are
/// about a gesture or a frame rather than a node, no prune path
/// produces one, and nothing in a type says so — the invariant is
/// established at `prune` and stated here.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Withdrawal<'a> {
    /// Which of the two this is.
    pub kind: WithdrawalKind,
    /// What went. **Never empty** — the constructors are the only
    /// door and each answers `None` for an empty set, so the
    /// rendering below never has to word "nothing was withdrawn".
    withdrawn: &'a [Withdrawn],
}

/// Which display state an accepted edit took away.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum WithdrawalKind {
    /// **A SUBSTITUTION.** The user's hand placement answered "where
    /// does this part go", and the mate that landed answers it better,
    /// so the probe steps aside and the picture keeps the part.
    ///
    /// [`crate::session::OpOutcome::superseded`] names the instances
    /// whose COMMITTED free-move placement an operation's document
    /// transition discarded — the G3 supersession, reported by the
    /// session rather than inferred (`display::DisplayState::prune` is
    /// where it happens, and `display::free_move_check` is the
    /// condition). A killed in-flight gesture is NOT in that list, so
    /// it is not this channel's to report; the next gesture op refuses
    /// typed instead.
    ///
    /// The standing fact it leaves behind is the instance drawn at its
    /// landed placement, which the picture already says; a badge would
    /// keep saying it about a document the user has moved on from.
    Superseded,
    /// **Not superseded by anything.** The user asked for an instance
    /// not to be DRAWN, and the document did not answer that question
    /// differently — it made the question unaskable. Nothing takes the
    /// hide's place.
    ///
    /// **What happened to the PICTURE is in the sentence.** The two
    /// arms leave the drawing in opposite states, and that is the part
    /// a user needs: on a **fuse** the instance is drawn AGAIN —
    /// material they took out of the picture is back in it, which is
    /// exactly the state reported as a bug against hiding — and on a
    /// **delete** the instance went, and nothing reappears. A preamble
    /// naming neither is true and useless; a preamble naming one is
    /// false half the time. So the consequence is said when the
    /// frame's withdrawals AGREE on it, and dropped when they do not,
    /// leaving the faults to say the rest. That is not this module
    /// writing prose about someone else's failure: the fault renders
    /// itself, unaltered, and what the chrome adds is the chrome's own
    /// subject — what the drawn scene now shows.
    DroppedHide,
}

impl<'a> Withdrawal<'a> {
    /// The frame's supersessions, or `None` when it superseded nothing.
    pub fn superseded(withdrawn: &'a [Withdrawn]) -> Option<Self> {
        Self::of(WithdrawalKind::Superseded, withdrawn)
    }

    /// The frame's dropped hides, or `None` when it dropped none.
    pub fn dropped_hide(withdrawn: &'a [Withdrawn]) -> Option<Self> {
        Self::of(WithdrawalKind::DroppedHide, withdrawn)
    }

    /// The `None` decision, in one place: an empty set is silence.
    fn of(kind: WithdrawalKind, withdrawn: &'a [Withdrawn]) -> Option<Self> {
        (!withdrawn.is_empty()).then_some(Self { kind, withdrawn })
    }

    /// This withdrawal as a notice for [`frame_status`]'s rank 2.
    pub fn notice(&self) -> Message {
        Message::new(Subject::Document, self.to_string())
    }
}

impl core::fmt::Display for Withdrawal<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let withdrawn = self.withdrawn;
        let fused = |w: &Withdrawn| matches!(w.cause, DisplayFault::FusedGeometry { .. });
        let (kind, one, many, consequence) = match self.kind {
            WithdrawalKind::Superseded => (
                "free move",
                "a committed placement was discarded",
                "committed placements were discarded",
                "",
            ),
            WithdrawalKind::DroppedHide => (
                "hide",
                "a hide was dropped",
                "hides were dropped",
                if withdrawn.iter().all(fused) {
                    " and the hidden geometry is drawn again"
                } else if withdrawn.iter().any(fused) {
                    ""
                } else {
                    " with the instance it was on"
                },
            ),
        };
        match withdrawn.len() {
            1 => write!(f, "{kind}: {one}{consequence} — ")?,
            count => write!(f, "{kind}: {count} {many}{consequence} — ")?,
        }
        // Each cause rendered by its own `Display`, in the order the
        // prune found them, joined with [`NOTICE_SEPARATOR`] rather
        // than composed into a sentence, for the reason
        // [`frame_status`] joins notices with it: a list of several
        // typed values must not become one written claim about them.
        // The one spelling, so a line carrying two faults reads like a
        // line carrying two notices.
        //
        // The join is flat, so a fault whose own text contains the
        // separator nests inside it and a reader cannot see where one
        // cause ends. `DisplayFault::NonRigidFrame` is such a text; no
        // prune path produces it here.
        for (position, entry) in withdrawn.iter().enumerate() {
            if position > 0 {
                f.write_str(NOTICE_SEPARATOR)?;
            }
            write!(f, "{}", entry.cause)?;
        }
        Ok(())
    }
}

/// **The status line after a camera fold.**
///
/// A refusal is news: the user asked the camera for something it
/// would not do, and no other channel says so. A CLEAN fold is not
/// news at all — the camera arriving where it was sent is the
/// unremarkable case, and it is the case on every frame of a drag and
/// on the re-frame an opened document books for itself.
///
/// So the clean arm says nothing, and [`StatusUpdate::Expire`] is how
/// it says nothing. It is never [`StatusUpdate::Clear`]: clearing
/// belongs to [`batch_status`], where an action the document ACCEPTED
/// is what makes the last complaint stale; a camera move is not one,
/// and a fold that cleared would be deciding the fate of sentences
/// written by writers it knows nothing about — on the frame a document
/// lands, the ones that landing itself produced.
///
/// **What the clean arm DOES decide is the fate of the camera's own
/// last sentence**, and that is the whole of [`Subject`]'s rule: the
/// refusal this function wrote on an earlier frame is the answer to a
/// move the user has since asked again about, so the next camera event
/// retires it whatever that event says. Without it a refused dolly sat
/// on the line for as long as the user orbited, because orbiting acts
/// on nothing and nothing else ever swept it.
///
/// The refusal renders the operation alongside the error because a
/// camera refusal is about a MOVE: the error alone names the condition
/// without the thing that provoked it.
pub fn fold_status(folded: &Folded) -> StatusUpdate {
    match &folded.refused {
        Some((op, error)) => StatusUpdate::Show(Message::new(
            Subject::Camera,
            format!("camera: {error} (from {op})"),
        )),
        None => StatusUpdate::Expire(Subject::Camera),
    }
}

/// **The status line after this frame's cursor step.**
///
/// A message about what lies under the cursor is stale exactly when
/// the outstanding pick question is, and [`IdQueryLog::step`] already
/// makes that judgement for the id pass: it asks again when the cursor
/// moved OR when the picture changed under a still cursor, and voids
/// the outstanding question when the pointer leaves the pane. Both are
/// events about [`Subject::Cursor`], and neither has anything to say,
/// so both retire what the cursor last said.
///
/// [`IdStep::Hold`] is the one arm that is not an event: the
/// outstanding answer still describes this cursor, so a disagreement
/// reported about it is still about the cursor the user is pointing
/// with.
///
/// This is a policy over a value, not a report: it never SHOWS
/// anything. What the cursor has to say is
/// [`Disagreement`]'s, raised where the two picking paths are
/// compared.
pub fn cursor_status(step: IdStep) -> StatusUpdate {
    match step {
        IdStep::Hold => StatusUpdate::Keep,
        IdStep::Ask { .. } | IdStep::Void => StatusUpdate::Expire(Subject::Cursor),
    }
}

/// **How loudly a badge is drawn, and what the colour MEANS.**
///
/// The toolbar's badges are drawn in two colours and the split is a
/// real rule: `weak` for a report a reader need not act on, the
/// theme's `unresolved` for a verdict they may. The Features pane
/// argues it explicitly for rows — a poisoned row is deliberately
/// QUIET so the eye goes to the failed row a reader can do something
/// about — and until this type existed no value stated it, so four
/// badges each picked a colour at the call site and the rule lived
/// only in prose.
///
/// **The colour is REDUNDANT either way**, which is
/// [`crate::theme::Theme::unresolved`]'s own stated contract: every
/// badge says its own words, so nothing depends on the colour being
/// read.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Tone {
    /// A report. The reader may want to know; there is nothing to do
    /// about it.
    Advisory,
    /// A verdict a reader may need to act on.
    Actionable,
}

/// **What a reader can do with a badge beyond reading it.**
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Affordance {
    /// A label. Its [`Badge::detail`], where it has one, is a tooltip.
    Read,
    /// **A control, not a label** — the ratified argument the checks
    /// badge carries, and the reason this is part of the value rather
    /// than a shape the toolbar picks: the findings were once reachable
    /// only by hovering the badge, which is a poor home for text a
    /// reader needs to keep open while they act on it, because a
    /// tooltip is gone the moment the pointer moves toward the feature
    /// it names. The badge opens a window instead, and the window is
    /// where the sentences live.
    ///
    /// What opening it MEANS is the toolbar's: the draw hands back the
    /// click and the caller decides, so this type never names a window.
    Opens,
}

/// **A read of held state, badged on the toolbar.**
///
/// A badge is a function of state the application HOLDS, so it is
/// recomputed on the frame it is drawn and it ends when that state
/// ends. That is the channel test the module header states, and this
/// type is one half of it as a value; the other half is [`Message`],
/// which reports an outcome.
///
/// Being a read is why a badge survives a mouse drag with nobody
/// arranging it: the status line is swept by the next acting batch,
/// while a badge is redrawn from the same state it was drawn from
/// before.
///
/// # What its [`Subject`] means
///
/// The same thing it means on a [`Message`] — the recurring event
/// whose next occurrence makes this the wrong answer — reached by a
/// different road. Nothing retires a badge, because nothing stores
/// one: the subject names the event that changes the state the badge
/// READS, and the badge goes because the read does. So the field is
/// not consulted by [`apply`] or by any other retiring machinery, and
/// what it buys is that a seam's two channels answer one question
/// once ([`SeamSubject`]) instead of a badge and a line message about
/// the same seam being free to disagree.
///
/// # What being a value buys
///
/// The family was four members implemented four ways, and the
/// differences were not cosmetic. **Where the `None` decision lives
/// decides whether a row can assert it**: [`product_badge`]'s carve-out
/// for the arms another channel carries was testable because it was a
/// function, while the checks badge's "only when there are findings"
/// rule was an `&&` inside a `ui` closure and no test could reach it.
/// Every member is a function here, so every member's silence is a row.
///
/// The other three differences go the same way: the [`Tone`] rule is
/// stated by the value instead of picked per site, the affordance is
/// stated instead of implied by which widget a call site reached for,
/// and each label is composed once from the typed value it reads.
///
/// # The prefix is the chrome's own subject
///
/// A badge label opens by naming which badge it is — *at rest*,
/// *checks*, *δ* — and that is not the chrome writing prose about
/// another value's failure. The failure's own words are the typed
/// value's, rendered through its own `Display` and unaltered; what the
/// chrome adds is which of four badges the reader is looking at, which
/// is a fact about the toolbar and about nothing else.
/// [`product_badge`] adds nothing at all, because
/// [`ProductError`]'s `Display` already opens every arm with
/// "product: ".
/// # The constructors are the only door
///
/// The fields are private. Public fields would have left every call
/// site able to struct-literal a badge with any subject, tone and
/// affordance it liked, which is exactly the "four badges each picked
/// a colour at the call site" state this type exists to end — a rule
/// that can be spelled around is a convention, and the point of
/// making this a value was to stop it being one.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Badge {
    subject: Subject,
    label: String,
    tone: Tone,
    detail: Option<String>,
    affordance: Affordance,
}

impl Badge {
    /// What the badge is about, and so what ends it.
    pub fn subject(&self) -> Subject {
        self.subject
    }

    /// The words, carrying their own subject.
    pub fn label(&self) -> &str {
        &self.label
    }

    /// Whether a reader may need to act on it.
    pub fn tone(&self) -> Tone {
        self.tone
    }

    /// What hovering says, where there is more than the label.
    pub fn detail(&self) -> Option<&str> {
        self.detail.as_deref()
    }

    /// Whether the badge is a control.
    pub fn affordance(&self) -> Affordance {
        self.affordance
    }

    /// A badge that only reports.
    fn read(subject: Subject, label: String, tone: Tone) -> Self {
        Self {
            subject,
            label,
            tone,
            detail: None,
            affordance: Affordance::Read,
        }
    }

    /// This badge with a tooltip.
    fn detailed(mut self, detail: impl Into<String>) -> Self {
        self.detail = Some(detail.into());
        self
    }

    /// This badge as a control a reader opens.
    fn opens(mut self) -> Self {
        self.affordance = Affordance::Opens;
        self
    }
}

/// **A seam's subject, stated once at the type of its refusal.**
///
/// A display seam can speak on both channels — the pick index badges
/// the build it is holding a refusal for ([`index_badge`]) and puts a
/// refused CLICK on the line ([`unindexed_refusal`]) — and the crate's
/// rule is that one seam must not speak with two voices. Written at
/// each door that would be a convention: two doors, two literals, and
/// nothing but a reader to notice when they drift.
///
/// Written here it is not a convention. Both doors read one constant,
/// so the two answers are the same answer by construction and a
/// reviewer changing a seam's subject changes every channel it speaks
/// on at once — which is what [`tool_news`] buys for its twelve sites
/// by having one door, done for a seam that needs two.
///
/// Not public: the doors below are the API, and a caller that could
/// read this could also assign a subject without one.
trait SeamSubject {
    /// The event stream whose next event makes this seam's refusal the
    /// wrong answer.
    const SUBJECT: Subject;
}

/// The camera and the viewport it is projected into.
impl SeamSubject for CameraError {
    const SUBJECT: Subject = Subject::Camera;
}

/// The picture drawn from the document — the build that lands next is
/// what ends it.
impl SeamSubject for SceneError {
    const SUBJECT: Subject = Subject::Display;
}

/// The pick index seam, on the badge channel.
impl SeamSubject for PickIndexError {
    const SUBJECT: Subject = Subject::Display;
}

/// The pick index seam, on the line: the same seam, so the same
/// subject, which is the whole reason this is a constant on a type
/// rather than a literal at a door.
impl SeamSubject for NotIndexed {
    const SUBJECT: Subject = Subject::Display;
}

// # The subject-assigning doors
//
// **A subject is a decision, so it lives where a decision can be
// asserted.** The dozen writers that assign `ViewerApp::status`
// directly all sit inside `app`-gated draw paths no headless row
// executes, so a subject chosen at one of those sites is
// unfalsifiable — a reviewer can change `Camera` to `Preferences` and
// the whole suite stays green. That is the same argument `Badge` makes
// about the `None` decision, applied to the half of a `Message` that a
// `String` could not carry.
//
// So each door below answers the subject from the TYPED refusal it is
// handed, and the writer hands its refusal over rather than picking.
// Most are pinned twice over: the door takes one error type, so
// calling the wrong door does not compile. `tool_news` is the
// exception and says so.

/// **What a pick against a missing index says** — the one seam
/// refusal that stays on the line, and the boundary the channel test
/// is visible at.
///
/// What it REPORTS is seam state, which reads like a badge. What it
/// IS, is an outcome: [`crate::pick::unindexed`] answers `Some` for a
/// SELECT and `None` for an observation, so half its input is this
/// frame's own pick stream and the sentence exists because the user
/// clicked and got no answer. A badge would be lit whenever the index
/// is absent, clicked or not — and the seam state itself is already
/// read by two badges that would then say it a second way
/// ([`index_badge`] for a build that refused, [`Progress::Indexing`]
/// for one under way, whose hover text is this very sentence).
///
/// Its subject is the pick index seam's own ([`SeamSubject`]), because
/// a `Building` refusal stops being the answer when the build lands —
/// the same event that ends the badge.
pub fn unindexed_refusal(refusal: &NotIndexed) -> Message {
    Message::new(NotIndexed::SUBJECT, refusal.to_string())
}

/// **What a δ the display refused says** — [`Subject::Display`], the
/// picture keeping the δ it had until the next one is accepted.
///
/// The error's own words, whole: [`SceneError`] states the condition a
/// δ has to meet, and no prefix here says it a second way.
pub fn delta_refusal(error: &SceneError) -> Message {
    Message::new(SceneError::SUBJECT, error.to_string())
}

/// **What a δ field holding something that is not a number says** —
/// [`Subject::Display`], for [`delta_refusal`]'s reason. It never
/// reached [`crate::scene::DisplayTolerance`], so the parser's words
/// are what there is.
pub fn delta_not_a_number(typed: &str, error: &core::num::ParseFloatError) -> Message {
    Message::new(
        Subject::Display,
        format!("display δ: {typed:?} is not a number ({error})"),
    )
}

/// **What a preferences store that could not be written says** —
/// [`Subject::Preferences`], retired by the next write of that file.
pub fn store_refusal(error: &StoreError) -> Message {
    Message::new(Subject::Preferences, error.to_string())
}

/// **What the preferences file had to say at startup**, and `None`
/// when it had nothing.
///
/// [`Subject::Preferences`]. **Not type-pinned**: the notices arrive
/// already rendered, from three sources with three types
/// ([`crate::prefs::Notice`], [`crate::prefs::PrefsError`], and the
/// theme and preset resolutions), so what this door buys is one place
/// the decision is made rather than a type that forbids the other
/// answer.
pub fn startup_notices(notices: &[String]) -> Option<Message> {
    (!notices.is_empty())
        .then(|| Message::new(Subject::Preferences, notices.join(NOTICE_SEPARATOR)))
}

/// **What a cursor action the pick index refused says.**
///
/// [`Subject::Document`], not [`Subject::Cursor`]: the refusal is the
/// answer to an operation the user aimed at the document through the
/// cursor, and moving the pointer does not answer it. The cursor
/// subject is for a message ABOUT what lies under the pointer, which
/// is [`Disagreement`]'s.
pub fn pick_refusal(error: &PickError) -> Message {
    Message::new(Subject::Document, error.to_string())
}

/// **What a tool has to say** — an authoring panel's refusal, a
/// survival drop, a pick a tool declined. [`Subject::Document`],
/// retired by the next act the document accepts.
///
/// **The one door here that a type does not pin**, because its twelve
/// sites render through `tools::ToolKind::says`, `tools::ToolNotice`
/// and the typed forms vocabulary, and arrive as text. What it buys is
/// that all twelve share one decision: changing the subject of one
/// changes the subject of all twelve, and a row can see it.
pub fn tool_news(text: impl Into<String>) -> Message {
    Message::new(Subject::Document, text)
}

/// **What the chrome badges about the A5 at-rest verdict**, and `None`
/// for a part document and before anything lands — which is
/// [`crate::session::DocSession::at_rest`]'s own `None`, passed
/// through.
///
/// A certified assembly is [`Tone::Advisory`]: the verdict is good
/// news and there is nothing to act on. A refusal is
/// [`Tone::Actionable`] — it is the gate declining to certify the
/// product on screen, and the reader is the only one who can answer
/// it.
///
/// The refusal's words are [`crate::session::AtRestBadge`]'s own, the
/// typed refusal rendered unaltered; the "at rest: " opening is this
/// badge naming itself.
pub fn at_rest_badge(at_rest: Option<&AtRestBadge>) -> Option<Badge> {
    Some(match at_rest? {
        AtRestBadge::Certified { minted } => Badge::read(
            Subject::Document,
            format!("at rest: certified ({minted} declaration(s))"),
            Tone::Advisory,
        ),
        AtRestBadge::Refused { message } => Badge::read(
            Subject::Document,
            format!("at rest: {message}"),
            Tone::Actionable,
        ),
    })
}

/// **What the chrome badges about the advisory checks**, and `None`
/// when there is nothing to say.
///
/// # The two `None`s, and why they are one function now
///
/// `None` from the session means the registry refused or nothing has
/// landed; an EMPTY report means the checks ran and found nothing.
/// Both are silence here, and the second is the rule that used to be
/// an `&&` in a `ui` closure — the one a row could not reach, which is
/// this item's own argument for the vocabulary. The report's SKIPPED
/// checks do not light the badge either: "not checked" is a different
/// answer from "checked and found something", and the window is where
/// that distinction is drawn.
///
/// It REPORTS rather than blocks: the scene below is drawn either way,
/// because a product whose roots interpenetrate renders a picture that
/// looks almost right and the finding is the only thing that says
/// otherwise. So it is [`Tone::Actionable`] and it
/// [`Affordance::Opens`] — the findings' own sentences, each carrying
/// its own recourse, live in the window it opens and never here.
pub fn checks_badge(report: Option<&ChecksReport>) -> Option<Badge> {
    let count = report
        .map(|report| report.findings.len())
        .filter(|c| *c > 0)?;
    Some(
        Badge::read(
            Subject::Document,
            format!("checks: {count} finding(s)"),
            Tone::Actionable,
        )
        .opens()
        .detailed("show what the checks found"),
    )
}

/// **What the chrome badges about the δ the display budget chose**,
/// and `None` the moment the user picks their own.
///
/// Shown while the δ on screen is the one the budget CHOSE when the
/// document opened. A read of held state, like its sibling badges,
/// which is why it is a badge and not a line: "this δ was chosen for
/// you" has to outlive a mouse drag.
///
/// [`Tone::Advisory`] — a δ chosen by the budget is a report, and the
/// remedy, if a reader wants one, is the δ field beside it.
///
/// **Both halves of the `None` are here**: a δ the user set
/// ([`crate::scene::FittedDelta`] absent) and a fit with nothing to
/// say (`wording` absent, which is a fit that did not move δ). The
/// second was a second condition at the call site.
pub fn delta_badge(fitted: Option<&FittedDelta>) -> Option<Badge> {
    let fitted = fitted?;
    let wording = fitted.wording()?;
    Some(
        Badge::read(
            Subject::Display,
            format!("δ {:.3} mm chosen", fitted.delta.get() * 1.0e3),
            Tone::Advisory,
        )
        .detailed(wording),
    )
}

/// **What the chrome badges about the landed product**, and `None`
/// when there is nothing to say.
///
/// The gather's verdict is a STANDING FACT: computed once when a pair
/// lands, and true of the picture on screen until another pair lands.
/// It is therefore not news, and the status line — which carries one
/// frame's news — is the wrong home for it. The frame an Open lands on
/// is exactly the frame that also re-frames the camera, so the line is
/// the one place a fault raised by a landing cannot survive the
/// landing.
///
/// **Redundant colour beside its own words.** It is
/// [`Tone::Actionable`], the tone the at-rest refusal and the checks
/// findings already carry, and that tone's stated contract is that its
/// colour is REDUNDANT — every badge using it says its own words, so
/// nothing depends on the colour being read. This badge satisfies it,
/// because [`ProductError`]'s `Display` opens every arm with
/// "product: ".
///
/// It is **not** simply louder than the line it left, and the argument
/// must not lean on that: chromatically it is far more salient than an
/// uncoloured label, and in LUMINANCE contrast it is lower in both
/// palettes. What justifies the home is the lifetime — a standing fact
/// cannot live on a line that carries one frame's news — and what
/// justifies the colour is that it is the spelling its three sibling
/// badges already use for a verdict a reader may need to act on.
///
/// # The arms that stay silent, and why
///
/// **A document with no body root is EMPTY, not malformed.** A fresh
/// document is in that state, and so is one whose last feature was just
/// deleted — and the blank viewport says so more plainly than any words
/// could. Reporting it makes deleting the last feature look like a
/// failure.
///
/// **A per-node state the feature tree already badges is not this
/// channel's to repeat.** [`crate::tree::RowStatus`] has exactly three
/// non-`Ok` states — `Failed`, `Poisoned`, `Unevaluated` — and
/// [`ProductError::RootFailed`], [`ProductError::RootPoisoned`] and
/// [`ProductError::UnknownNode`] are those same three states seen from
/// the gather. The tree badges each AT the node and carries the typed
/// cause with it, so this badge would say strictly less, in a louder
/// colour, one row above a status line already reporting the same
/// root's tessellation refusal. The Features pane goes further and
/// draws a poisoned row deliberately QUIET, reserving
/// [`Tone::Actionable`] for the row a reader can act on; a badge
/// shouting about the same poisoning would have the chrome saying both
/// things at once.
///
/// What is left is what this channel is FOR: the gather-level faults no
/// per-node badge can carry — a naming collision across roots, a graft
/// the kernel refused, a validity verdict on the assembled product, an
/// evaluation of the wrong document.
pub fn product_badge(fault: Option<&ProductError>) -> Option<Badge> {
    fault
        .filter(|fault| {
            !matches!(
                fault,
                ProductError::NoBodyRoots
                    | ProductError::RootFailed { .. }
                    | ProductError::RootPoisoned { .. }
                    | ProductError::UnknownNode { .. }
            )
        })
        .map(|fault| Badge::read(Subject::Document, fault.to_string(), Tone::Actionable))
}

/// **What the chrome badges about the scene the picture is drawn
/// from**, and `None` while the last rebuild stands.
///
/// A read of held state: `ViewerApp` keeps the refusal until a rebuild
/// succeeds, and it keeps drawing the mesh it already has — so the
/// picture on screen is stale for exactly as long as this is `Some`.
/// It was a line message, where an accepted act's
/// [`StatusUpdate::Clear`] swept it off a picture that had not been
/// rebuilt and the line then said nothing about a scene it still could
/// not build.
///
/// [`Tone::Actionable`]: it is the picture declining to follow the
/// document, and the reader is the only one who can answer it. The
/// error's own words, behind this badge naming itself.
pub fn scene_badge(error: Option<&SceneError>) -> Option<Badge> {
    error.map(|error| {
        Badge::read(
            SceneError::SUBJECT,
            format!("scene: {error}"),
            Tone::Actionable,
        )
    })
}

/// **What the chrome badges about the pick-index seam**, and `None`
/// when the cache holds no refusal.
///
/// The purest read of the three: the refusal is held by
/// [`crate::pick::PickCache`] under its one-attempt-per (generation,
/// δ) policy, so this asks the value that already knows and the badge
/// stands for exactly as long as the policy holds the refusal.
///
/// It says the SEAM refused. What a pick against the missing index
/// gets is [`unindexed_refusal`], on the line, because that is an
/// outcome — the two carry one subject and neither states it
/// ([`SeamSubject`]).
pub fn index_badge(error: Option<&PickIndexError>) -> Option<Badge> {
    error.map(|error| {
        Badge::read(
            PickIndexError::SUBJECT,
            format!("pick index: {error}"),
            Tone::Actionable,
        )
    })
}

/// **What the chrome badges about a camera that cannot be
/// projected**, and `None` while the view matrix forms.
///
/// [`Subject::Camera`] ([`SeamSubject`]) and a badge: the two answers
/// are to different questions. It is read from the camera and the
/// viewport it is drawn into, both held, and it is true on every frame
/// until the camera moves somewhere a projection can be formed from —
/// which is also the event its subject names.
///
/// **The subject is why it could not stay on the line.**
/// [`fold_status`] issues `Expire(Camera)` on every clean fold, so a
/// camera message was retired by the next MOVE whether or not a
/// projection could be formed after it — the line going quiet about a
/// picture it still cannot draw. A badge is retired by the read, so it
/// stands until the projection does.
pub fn projection_badge(error: Option<&CameraError>) -> Option<Badge> {
    error.map(|error| {
        Badge::read(
            CameraError::SUBJECT,
            format!("projection: {error}"),
            Tone::Actionable,
        )
    })
}

/// What the toolbar has to say about work the picture is waiting on.
///
/// **One state, not a badge per seam.** The chrome had three
/// conditions and grew a fourth when the pick index moved onto its own
/// seam; expressing that as a second `if` beside the first would have
/// given the toolbar two indicators that can both be lit, for one
/// wait, with no rule anywhere saying which the reader should believe.
/// The rule is here instead, and it is a total function of three
/// booleans.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Progress {
    /// A run is in flight: the picture is older than the document and
    /// someone is doing something about it.
    Evaluating,
    /// The picture is older than the document and **no EVALUATION is
    /// running** — what a cancel leaves behind. A spinner over that
    /// alone would be a lie about work nobody is doing.
    ///
    /// `indexing` is whether the OTHER seam is nonetheless busy, and
    /// it is carried here rather than answered by a second indicator
    /// because this is the one state where the two seams disagree
    /// about whether anything is happening: an index build submitted
    /// before the cancel is still running, and it will change the
    /// picture. The rule the payload buys is **the spinner follows the
    /// work, never the name** — so a canceled evaluation with a live
    /// index build spins, and the status line's own *still being
    /// indexed* refusal agrees with the toolbar instead of describing
    /// the same moment a second way.
    Canceled {
        /// Whether an index build is in flight behind the cancel.
        indexing: bool,
    },
    /// The document is evaluated and its index is being built: the
    /// picture is the last one that finished, and picks are refused
    /// until this lands ([`crate::pick::unindexed`]).
    Indexing,
}

/// The one state, from the session's two answers and the pick cache's.
///
/// **Evaluation outranks indexing**, because an index built for a
/// generation the session has already moved past is about to be
/// discarded by [`crate::pick::PickCache::land`] anyway — restart
/// without cancel means both can be in flight at once, and naming the
/// index build there would tell a reader the wait was nearly over when
/// a whole evaluation is still ahead of it.
pub fn progress(busy: bool, running: bool, indexing: bool) -> Option<Progress> {
    match (busy, running, indexing) {
        (true, true, _) => Some(Progress::Evaluating),
        (true, false, indexing) => Some(Progress::Canceled { indexing }),
        (false, _, true) => Some(Progress::Indexing),
        (false, _, false) => None,
    }
}

/// The name a refused batch offers to CREATE.
///
/// The parse door's unknown-parameter refusal is deliberate
/// typo-safety — text naming an undeclared parameter never creates
/// one. The ratified pattern is refuse-then-offer, and this is the
/// offer as a value: the undeclared name, for the frame loop to
/// prefill into the add-parameter affordance (name only — the
/// expression's context does not determine the new parameter's
/// DIMENSION, so that stays the user's explicit pick there). `None`
/// for every other refusal and for a clean batch.
pub fn creation_offer(refusal: Option<&Refusal>) -> Option<ParamName> {
    match refusal {
        Some(Refusal::Parse(error)) => match error.as_ref() {
            // The parse error carries the identifier as text (it is a
            // fact about the SOURCE); the offer mints the name the
            // create door would declare.
            ParseError::UnknownParam { name, .. } => Some(ParamName::new(name.as_str())),
            _ => None,
        },
        _ => None,
    }
}

/// The expression draft a parse-refused batch should hand back.
///
/// The chrome clears the expression field the moment Set is clicked —
/// a draft is transient state and a committed one leaves nothing
/// behind. But a PARSE refusal means nothing was committed, and for
/// the unknown-parameter case the offer above sends the user off to
/// create the parameter first: coming back to an empty field would
/// make acting on the offer cost the very text that raised it. So a
/// parse-refused batch restores the draft — the slot the text was
/// aimed at and the text itself, read from the batch's own op.
pub fn retype_draft(
    ops: &[SessionOp],
    refusal: Option<&Refusal>,
) -> Option<(RecipeNodeId, SlotId, String)> {
    if !matches!(refusal, Some(Refusal::Parse(_))) {
        return None;
    }
    ops.iter().rev().find_map(|op| match op {
        SessionOp::SetSlotExpression { node, slot, text } => Some((*node, *slot, text.clone())),
        _ => None,
    })
}

/// What the environment offers `rfd` as a file-chooser backend.
///
/// Probed ONCE at startup ([`chooser_backend`]) — it is a fact about
/// the environment, not per-frame state — and consulted wherever a
/// dialog is offered. The point is to fail LOUD at first sight (first
/// light, issue #1097: Open/Save As "silently did nothing" on a WSL
/// distro shipping neither backend) instead of hedging after a dead
/// click: `rfd`'s blocking dialogs return the same bare `None` for a
/// user cancel and for a backend that could not put a dialog up, so
/// the time to know is before the click.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum ChooserBackend {
    /// `zenity` is on `PATH`: dialogs work with no portal at all.
    ZenityPresent,
    /// No `zenity`, but a D-Bus session-bus address exists, so an
    /// `xdg-desktop-portal` file chooser is possible. **A HINT, not a
    /// verdict**: a session bus without a working portal frontend
    /// still ends in a silent `None` the process cannot tell from a
    /// cancel — that residue is the README's troubleshooting entry,
    /// not a message this code can honestly print.
    PortalPossible,
    /// Neither: no dialog can possibly appear. The one CONFIDENT
    /// arm, and the one the chrome disables the dialogs over.
    Absent,
}

impl ChooserBackend {
    /// Whether attempting a dialog can possibly show one.
    pub fn usable(self) -> bool {
        !matches!(self, Self::Absent)
    }
}

/// The directory this project keeps user files in.
const PREFS_DIR: &str = "pncad";
/// The preferences file's name inside it.
const PREFS_FILE: &str = "viewer.toml";

/// The chooser verdict as a pure function of the two probe readings,
/// so the rows exercising it do not depend on the CI box's `PATH`.
pub fn chooser_backend_of(zenity_on_path: bool, session_bus: bool) -> ChooserBackend {
    match (zenity_on_path, session_bus) {
        (true, _) => ChooserBackend::ZenityPresent,
        (false, true) => ChooserBackend::PortalPossible,
        (false, false) => ChooserBackend::Absent,
    }
}

/// Probe the environment for a chooser backend. Startup calls this
/// once; everything downstream reads the stored value.
pub fn chooser_backend() -> ChooserBackend {
    if cfg!(target_family = "wasm") {
        // The browser build links no `rfd` at all (its wasm backend
        // offers only the async dialog; see `viewer`'s Cargo.toml),
        // so this is the CONFIDENT arm in the strongest possible
        // sense: there is not merely no backend, there is no dialog
        // code. Saying `Absent` is what disables Open…/Save… with
        // their reason showing, which is #1097's whole lesson — a
        // door that cannot open must not answer a click with silence.
        ChooserBackend::Absent
    } else if cfg!(target_os = "linux") {
        chooser_backend_of(zenity_on_path(), session_bus_hinted())
    } else {
        // Off Linux `rfd` speaks the platform's native dialog API and
        // the zenity/portal question does not arise. Grouped under the
        // hint arm because the downstream meaning is the same: attempt
        // the dialog, and read a `None` as a genuine cancel.
        ChooserBackend::PortalPossible
    }
}

/// Whether a `zenity` binary sits in some `PATH` directory. Presence
/// is the signal `rfd`'s own fallback lookup uses; a present but
/// broken zenity is the dialog's own problem to report.
fn zenity_on_path() -> bool {
    std::env::var_os("PATH")
        .is_some_and(|paths| std::env::split_paths(&paths).any(|dir| dir.join("zenity").is_file()))
}

/// Whether a D-Bus session-bus address is advertised — the necessary
/// (never sufficient) condition for the portal chooser.
fn session_bus_hinted() -> bool {
    std::env::var_os("DBUS_SESSION_BUS_ADDRESS").is_some_and(|address| !address.is_empty())
}

/// Where this platform keeps the viewer's preferences:
/// `$XDG_CONFIG_HOME/pncad/viewer.toml`, falling back to
/// `$HOME/.config` as the XDG base-directory specification says to.
///
/// **Here rather than in [`crate::prefs`], because this file is the
/// viewer's ONE ambient door** — the ruling in
/// `scripts/gates/no-ambient-env.sh`, which names this module by path
/// and says so in as many words. `prefs` stays a pure value over a
/// document and a store; where the document lives is a fact about the
/// machine, and facts about the machine are observed here beside the
/// chooser-backend verdict and the WSL probe.
///
/// Against that gate's four rows, the same way the entry beside it
/// argues them. CONTRACT-RATIFIED holds vacuously: a config path is
/// not a model parameter, and no read here can change what any
/// document evaluates to. COMMIT-ONCE: read at startup and stored in
/// the application, never re-read under a running app — a viewer
/// whose config directory moved mid-session would be stranger than
/// one that kept writing where it started. REPORTED: the path is
/// carried in every [`crate::prefs::StoreError`], so a refusal to
/// save names the file it could not write rather than leaving a
/// person guessing. RECONCILED: this is a BOOTSTRAP and never the
/// last word — the actual read or write outcome outranks it, and an
/// environment that names no config directory yields `None`, which
/// disables saving with a reason instead of inventing a path.
///
/// Resolved by hand rather than through `directories`, whose whole
/// value is the two platforms this project does not build for.
///
/// `None` when neither variable is set, which is a real possibility
/// in a stripped environment.
#[must_use]
pub fn prefs_path() -> Option<std::path::PathBuf> {
    prefs_path_in(
        std::env::var_os("XDG_CONFIG_HOME").as_deref(),
        std::env::var_os("HOME").as_deref(),
    )
}

/// The preferences path as a pure function of the two environment
/// readings, so the XDG rules are asserted on rather than trusted.
///
/// Split out for [`chooser_backend_of`]'s reason, and it is the same
/// reason: the ambient read is one line that cannot be exercised in a
/// test without mutating the process's environment — which is a
/// global other tests share — while everything INTERESTING here is
/// the resolution, and the resolution is a function of two `Option`s.
///
/// The rules, from the XDG base-directory specification:
///
/// - `config_home` set and non-empty wins.
/// - **An EMPTY `config_home` counts as unset**, which the spec says
///   in as many words and which is the case a bare
///   `unwrap_or_else` fallback gets wrong: it would take `""` as the
///   base and write to a RELATIVE path, i.e. into whatever directory
///   the viewer happened to be launched from.
/// - Otherwise `$HOME/.config`.
/// - With neither, `None` — no path is invented. The caller's store
///   is then unusable and says so, which is how a person finds out
///   their preferences are not being kept rather than wondering
///   later why nothing was remembered.
#[must_use]
pub fn prefs_path_in(
    config_home: Option<&std::ffi::OsStr>,
    home: Option<&std::ffi::OsStr>,
) -> Option<std::path::PathBuf> {
    let base = match config_home {
        Some(value) if !value.is_empty() => std::path::PathBuf::from(value),
        _ => std::path::PathBuf::from(home.filter(|h| !h.is_empty())?).join(".config"),
    };
    Some(base.join(PREFS_DIR).join(PREFS_FILE))
}

/// Whether this process runs inside WSL, read off the environment
/// markers WSL itself sets for every process (`WSL_DISTRO_NAME`,
/// `WSL_INTEROP`). Either suffices; both are checked because WSL1
/// and WSL2 differ in which they guarantee. Consumed by `app::run`,
/// which prefers the X11 backend under WSL (WSLg's Wayland RAIL shell
/// breaks horizontal resizing — #1097, confirmed).
///
/// Here rather than in `app` so the viewer's ambient-environment
/// reads have ONE home, which is what the `no-ambient-env` gate's
/// allowlist entry for this file ratifies — see the argument in
/// `scripts/gates/no-ambient-env.sh`.
#[cfg(target_os = "linux")]
pub fn running_under_wsl() -> bool {
    std::env::var_os("WSL_DISTRO_NAME").is_some() || std::env::var_os("WSL_INTEROP").is_some()
}

/// What the disabled dialog controls say, and what the status line
/// says should a dialog somehow be attempted anyway: the confident
/// half of the #1097 finding, with the dialog-free workaround.
pub const NO_CHOOSER_BACKEND: &str = "no file chooser backend — install zenity or \
     xdg-desktop-portal; a document path can also be passed on the \
     command line";

/// The status line after a file dialog returns.
///
/// A chosen path leaves the line alone — the `Open`/`Save` batch it
/// feeds owns the verdict through [`batch_status`]. An empty-handed
/// dialog under a plausibly-present backend is read as a genuine
/// cancel and stays QUIET (a cancel should not nag); under
/// [`ChooserBackend::Absent`] it is the loud arm — belt to the
/// chrome's braces, which should have disabled the control before any
/// click could reach here.
pub fn dialog_status(backend: ChooserBackend, chose: bool) -> StatusUpdate {
    match (chose, backend.usable()) {
        (true, _) | (false, true) => StatusUpdate::Keep,
        // The document the user asked for is the subject: they aimed
        // Open or Save at it and this is what came back, so the next
        // act the document accepts is what makes it stale.
        (false, false) => StatusUpdate::Show(Message::new(
            Subject::Document,
            NO_CHOOSER_BACKEND.to_owned(),
        )),
    }
}

/// Whether a folded event stream actually moved the camera.
///
/// The stream carries cursor events too, and a stream that denotes no
/// camera operation is not a camera event.
///
/// **What this buys is a statement, not a fix.** It once stopped an
/// erasure: landing a no-op fold cleared the status line on every frame
/// the pointer was inside the viewport. [`fold_status`] closed that at
/// the other end, so the guard is now near-redundant behaviourally —
/// it saves one call and a `Camera` copy. It is kept because
/// `pane::viewport::land` is documented as the one place a camera MOVE
/// becomes application state, and calling it on frames where nothing
/// moved makes that sentence false and hands any writer later added to
/// it per-frame behaviour nobody asked for.
pub fn folded_moved(folded: &Folded) -> bool {
    !folded.applied.is_empty() || folded.refused.is_some()
}

/// What the viewport should do about the GPU id query this frame.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum IdStep {
    /// Ask: the cursor moved, or the picture changed under it.
    Ask {
        /// The serial to stamp the query with.
        serial: u32,
    },
    /// Nothing to ask — the outstanding answer still describes this
    /// cursor.
    Hold,
    /// The pointer is gone; any outstanding answer is void.
    Void,
}

/// The id pass's query bookkeeping: which query is outstanding, and
/// what it was asked about.
///
/// **Two defects this closes, both of them about a query's answer
/// outliving its question.** The pass used to be asked on every frame
/// the pointer was inside the pane, moved or not — a blocking GPU
/// readback per frame, and a documented movement gate that did not
/// exist. And on leaving the pane no query was issued, no serial was
/// reset, and the last answer stayed matched: with the ray path's
/// hover cleared to `None`, the comparison then reported a permanent
/// disagreement over empty space, which is the one symptom issue
/// #1097 §4 tells the operator to read as a `R32Uint` clear fault.
#[derive(Clone, Copy, Debug, Default)]
pub struct IdQueryLog {
    serial: u32,
    /// The cursor and the picture the outstanding query was asked
    /// about. `None` when nothing is outstanding.
    asked: Option<([f64; 2], Option<Generation>)>,
}

impl IdQueryLog {
    /// A log with nothing outstanding.
    pub fn new() -> Self {
        Self::default()
    }

    /// The serial of the query whose answer is still about the cursor,
    /// or `None` when nothing is outstanding.
    ///
    /// The comparison reads this: an answer whose serial does not match
    /// is about a question nobody is asking any more.
    pub fn outstanding(&self) -> Option<u32> {
        self.asked.map(|_| self.serial)
    }

    /// Advance the log for this frame's cursor and picture.
    ///
    /// `cursor` is `None` when the pointer is outside the pane.
    /// `generation` is the evaluation the index describes: a query is
    /// re-asked when the picture changes under a still cursor, because
    /// the answer is about the picture and not only about the pointer.
    pub fn step(&mut self, cursor: Option<[f64; 2]>, generation: Option<Generation>) -> IdStep {
        let Some(cursor) = cursor else {
            self.asked = None;
            return IdStep::Void;
        };
        if self.asked == Some((cursor, generation)) {
            return IdStep::Hold;
        }
        // Saturating past zero: zero is the "nothing was ever asked"
        // serial the answer channel is initialised to, so a wrap must
        // not land on it.
        self.serial = self.serial.wrapping_add(1).max(1);
        self.asked = Some((cursor, generation));
        IdStep::Ask {
            serial: self.serial,
        }
    }
}

/// The two picking paths' answers for one cursor, when they differ.
#[derive(Clone, Debug, PartialEq)]
pub struct Disagreement {
    /// What the id buffer named, `None` for nothing under the cursor.
    pub from_gpu: Option<StableName>,
    /// What the ray path named.
    pub from_ray: Option<StableName>,
}

impl core::fmt::Display for Disagreement {
    /// Each side renders through [`StableName`]'s own `Display` — kind
    /// and minting node, the half a user can act on — followed by the
    /// role path.
    ///
    /// BOTH halves are load-bearing here, which is what makes this
    /// message different from every other one in this crate. The name's
    /// `Display` omits the path deliberately, so two names differing
    /// only in their derivation would render identically; the path
    /// alone drops kind and node, so two names on different nodes
    /// sharing a role path would. A message whose entire subject is
    /// that two answers DIFFER cannot afford either collapse.
    ///
    /// The path rides as `Debug` because `RoleSeg` has no `Display` in
    /// this workspace — the one rendering here that is not prose, and
    /// it is a derivation, not a sentence.
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let show = |name: &Option<StableName>| match name {
            Some(name) => format!("{name} ({:?})", name.path),
            None => "nothing".to_owned(),
        };
        write!(
            f,
            "picking paths disagree at the cursor: id buffer {}, ray {}",
            show(&self.from_gpu),
            show(&self.from_ray)
        )
    }
}

impl Disagreement {
    /// This disagreement as a message for the status line.
    ///
    /// [`Subject::Cursor`]: it is a claim about what lies under THIS
    /// cursor over THIS picture, and [`cursor_status`] retires it on
    /// the id log's own judgement that the question has moved on.
    pub fn notice(&self) -> Message {
        Message::new(Subject::Cursor, self.to_string())
    }
}

/// Compare the id pass's answer against the ray path's, **by name**.
///
/// # Why names and not ids
///
/// One stable name can be drawn under several ids — two `Transform`
/// roots over one extrude carry the same names on both copies — so
/// comparing raw ids reports a disagreement whenever the two paths
/// name the same face on different drawn copies. The property the two
/// lanes are supposed to share is "the same face is under the cursor",
/// and a face is a name.
///
/// # The role inversion, recorded at the seam
///
/// GQ6-RESURVEY §3 assigns the GPU id buffer to hover/click exactness
/// and the CPU ray cast to snapping. **This unit inverts that**: the
/// ray path is authoritative because it is the path CI can execute,
/// and the id pass is advisory — it runs beside the ray and
/// contradicts it out loud rather than deciding anything. That is the
/// whole reason this function reports and never resolves, and it is
/// what makes issue #1097 §4's hardware check one cursor sweep.
///
/// `answer` is the raw channel word (`serial << 32 | id`); `expected`
/// is [`IdQueryLog::outstanding`]. `None` means "no verdict": no query
/// outstanding, a stale answer, or the two agree.
pub fn disagreement(
    index: &PickIndex,
    answer: u64,
    expected: Option<u32>,
    from_ray: Option<&StableName>,
) -> Option<Disagreement> {
    if expected? != (answer >> 32) as u32 {
        return None;
    }
    let id = answer as u32;
    let from_gpu = if id == IdMap::NOTHING {
        None
    } else {
        index
            .name_of(id)
            .and_then(|name| name.as_ref().ok())
            .cloned()
    };
    (from_gpu.as_ref() != from_ray).then(|| Disagreement {
        from_gpu,
        from_ray: from_ray.cloned(),
    })
}

/// **The two lifetimes, as policy over values.**
///
/// Both rules this module states about the status line are silent when
/// they break: a cleared line looks exactly like a line nobody wrote
/// to, and a fault with no home looks exactly like a document with no
/// fault.
///
/// Everything here is a pure function of a value, and the values are
/// built by hand — `frame` is a vocabulary, so it is read and tested
/// with no session and no window in existence
/// (`crates/viewer/README.md`). The other half, where a real fold meets
/// a real landing, is `pane::viewport`'s: that is the driver, and the
/// rows that need one live there.
#[cfg(test)]
mod tests {
    // Panicking is a test's failure mechanism (workspace lint note).
    #![allow(clippy::expect_used)]
    #![allow(clippy::panic)]

    use super::*;

    use bvh::Aabb;
    use pncad::document::RecipeNodeId;
    use pncad::prelude::EntityKind;

    use crate::camera::{Camera, CameraOp, CameraOpError};
    use crate::display::DisplayFault;

    /// A camera — any camera. Nothing here reads it: [`fold_status`]
    /// judges what a fold REFUSED, and [`Folded`] has to carry one.
    fn a_camera() -> Camera {
        let unit = Aabb {
            min_x: 0.0,
            min_y: 0.0,
            min_z: 0.0,
            max_x: 1.0,
            max_y: 1.0,
            max_z: 1.0,
        };
        Camera::framing(&unit, 16.0 / 9.0).expect("a unit box frames")
    }

    /// A fold that applied everything it was given — the shape
    /// `fold_recorded` returns for a drag that worked, and for the
    /// re-frame an opened document books for itself.
    fn a_clean_fold() -> Folded {
        Folded {
            camera: a_camera(),
            applied: vec![CameraOp::Orbit {
                yaw: 0.2,
                pitch: 0.1,
            }],
            refused: None,
        }
    }

    #[test]
    fn a_clean_fold_keeps_a_message_it_did_not_write() {
        // The defect this closes: the camera is the fastest-moving
        // writer the line has, and one that assigned on every clean
        // fold decided the fate of every other writer's sentence.
        let folded = a_clean_fold();
        assert!(
            folded_moved(&folded),
            "the fold MOVED, so the frame loop lands it — a fold that \
             moved nothing never reaches the line at all, and this row \
             would be asserting about a case that cannot happen"
        );
        assert_eq!(fold_status(&folded), StatusUpdate::Expire(Subject::Camera));

        let elsewhere = Message::new(Subject::Document, "someone else's news");
        let mut status = Some(elsewhere.clone());
        apply(&mut status, fold_status(&folded));
        assert_eq!(
            status,
            Some(elsewhere),
            "a clean fold is not news, and it retires nothing it did \
             not write"
        );
    }

    #[test]
    fn a_clean_fold_retires_the_camera_refusal_it_did_write() {
        // The item's own reproduction: refuse a camera operation, then
        // navigate. Nothing acts, so nothing clears, and before the
        // subject rule the refusal sat on the line for as long as the
        // user orbited.
        let mut status = None;
        apply(&mut status, fold_status(&a_refused_fold()));
        assert!(status.is_some(), "a refused fold is news");

        apply(&mut status, fold_status(&a_clean_fold()));
        assert_eq!(
            status, None,
            "the next camera event retires a camera verdict whatever \
             that event says"
        );
    }

    #[test]
    fn expiry_reaches_one_subject_and_no_other() {
        // The two ways this can be wrong, and they fail in opposite
        // directions: a message retired by an event about something
        // else, and a message that survives an event about itself.
        for (held, event, survives) in [
            (Subject::Camera, Subject::Camera, false),
            (Subject::Camera, Subject::Cursor, true),
            (Subject::Cursor, Subject::Camera, true),
            (Subject::Cursor, Subject::Cursor, false),
            (Subject::Document, Subject::Camera, true),
            (Subject::Display, Subject::Display, false),
            (Subject::Preferences, Subject::Document, true),
        ] {
            let mut status = Some(Message::new(held, "the sentence on the line"));
            apply(&mut status, StatusUpdate::Expire(event));
            assert_eq!(
                status.is_some(),
                survives,
                "a message about {held:?} met an event about {event:?}"
            );
        }
    }

    #[test]
    fn a_cursor_that_has_not_moved_retires_nothing() {
        // `IdStep::Hold` is the one arm that is not an event: the
        // outstanding answer still describes this cursor, so what the
        // cursor said is still about the cursor the user is pointing
        // with.
        let disagreement = Message::new(Subject::Cursor, "picking paths disagree");
        let mut status = Some(disagreement.clone());
        apply(&mut status, cursor_status(IdStep::Hold));
        assert_eq!(status, Some(disagreement));

        // And both of the other two ARE events, including the pointer
        // leaving the pane — where the id log voids the outstanding
        // question rather than asking a new one.
        for event in [IdStep::Ask { serial: 7 }, IdStep::Void] {
            let mut status = Some(Message::new(Subject::Cursor, "picking paths disagree"));
            apply(&mut status, cursor_status(event));
            assert_eq!(status, None, "{event:?} is a cursor event");
        }
    }

    /// A fold the camera refused: a dolly by zero, which is not a
    /// factor.
    fn a_refused_fold() -> Folded {
        Folded {
            camera: a_camera(),
            applied: Vec::new(),
            refused: Some((
                CameraOp::Dolly { factor: 0.0 },
                CameraOpError::NonPositiveDolly { factor: 0.0 },
            )),
        }
    }

    #[test]
    fn a_refused_fold_is_news_and_outranks_what_the_line_held() {
        let folded = a_refused_fold();
        assert!(folded_moved(&folded), "a refusal is a camera event too");
        let StatusUpdate::Show(message) = fold_status(&folded) else {
            panic!("a refused fold is news: {:?}", fold_status(&folded));
        };
        assert_eq!(
            message.subject(),
            Subject::Camera,
            "a camera verdict is about the camera: {message}"
        );
        assert!(
            message.text().contains("camera:") && message.text().contains("dolly by a factor"),
            "the refusal names the move that provoked it: {message}"
        );

        let mut status = Some(Message::new(Subject::Document, "older news"));
        apply(&mut status, fold_status(&folded));
        assert_eq!(status, Some(message));
    }

    #[test]
    fn the_gather_verdict_badges_only_the_faults_nothing_else_carries() {
        let node = RecipeNodeId(2);

        // The item's own reproduction: two roots colliding in the name
        // table. Not a node failure, so no per-node badge carries it —
        // which is why this channel exists at all.
        let collision = ProductError::Naming {
            node,
            name: Box::new(StableName {
                kind: EntityKind::Face,
                node,
                path: Vec::new(),
            }),
        };
        let badge = product_badge(Some(&collision)).expect("a naming collision badges");
        assert_eq!(
            badge.label(),
            collision.to_string(),
            "the fault renders itself"
        );
        assert_eq!(
            badge.tone(),
            Tone::Actionable,
            "a product the gather refused is a verdict a reader acts on"
        );
        assert_eq!(badge.affordance(), Affordance::Read, "it opens nothing");
        assert!(
            badge.label().starts_with("product: "),
            "and says what it is about, so the colour carries nothing \
             alone: {}",
            badge.label()
        );

        // The silent arms. An empty document is not malformed, and the
        // three per-node states are the feature tree's to badge — at
        // the node, with the cause, one of them deliberately quiet.
        for quiet in [
            ProductError::NoBodyRoots,
            ProductError::RootFailed { node },
            ProductError::RootPoisoned {
                node,
                through: RecipeNodeId(1),
            },
            ProductError::UnknownNode { node },
        ] {
            assert_eq!(
                product_badge(Some(&quiet)),
                None,
                "another channel already carries this: {quiet}"
            );
        }
        assert_eq!(product_badge(None), None);
    }

    #[test]
    fn keep_clear_and_show_are_four_different_sentences() {
        // `Keep` is a decision, not the absence of one — the whole
        // reason every policy here answers in this vocabulary instead
        // of assigning the field.
        let held = Message::new(Subject::Document, "held");
        let mut status = Some(held.clone());
        apply(&mut status, StatusUpdate::Keep);
        assert_eq!(status, Some(held));
        let news = Message::new(Subject::Camera, "news");
        apply(&mut status, StatusUpdate::Show(news.clone()));
        assert_eq!(status, Some(news));
        // `Clear` is the broad one, and deliberately: an act the
        // document accepted makes every standing complaint stale, not
        // only the ones about the document. It takes a camera message
        // with it.
        apply(&mut status, StatusUpdate::Clear);
        assert_eq!(status, None);
    }

    /// The two withdrawal notices' text: the constructor's `None`
    /// decision and the value's own `Display`, spelled once for the
    /// rows below that are about WORDING. The rows about the value
    /// itself — its subject, its silence — name `Withdrawal` directly.
    fn superseded_text(withdrawn: &[Withdrawn]) -> Option<String> {
        Withdrawal::superseded(withdrawn).map(|withdrawal| withdrawal.to_string())
    }

    /// The dropped-hide half of [`superseded_text`].
    fn dropped_hide_text(withdrawn: &[Withdrawn]) -> Option<String> {
        Withdrawal::dropped_hide(withdrawn).map(|withdrawal| withdrawal.to_string())
    }

    /// A withdrawal on `instance`, mate-constrained by `mates` — the
    /// commonest arm, and the one whose `Display` carries a remedy.
    fn constrained(instance: u64, mates: &[u64]) -> Withdrawn {
        Withdrawn {
            instance: RecipeNodeId(instance),
            cause: DisplayFault::MateConstrained {
                instance: RecipeNodeId(instance),
                mates: mates.iter().copied().map(RecipeNodeId).collect(),
            },
        }
    }

    #[test]
    fn a_supersession_survives_the_accepted_edit_that_caused_it() {
        // The defect this closes: the value reached the chrome and the
        // chrome dropped it. The trap underneath is that the operation
        // which supersedes is one the document ACCEPTED, so the frame's
        // own batch verdict is `Clear` — a supersession written to the
        // line instead of to the notices is erased by its own cause.
        let notice = superseded_text(&[constrained(7, &[9])]).expect("a supersession is news");
        assert!(
            notice.contains("instance 7"),
            "the notice names which of the user's placements went — here in \
             the part-instance vocabulary, because the MateConstrained arm's \
             subject is an instance. That is `DisplayFault`'s per-arm rule \
             and not a promise the notice makes across all of them; the \
             absent-node arm says `node N` and is right to: {notice}"
        );

        let acting = [SessionOp::Undo];
        assert_eq!(
            batch_status(&acting, None),
            StatusUpdate::Clear,
            "the frame this row is about CLEARS the line on its own — without \
             that, the composition below would be asserting about a case \
             where nothing had to survive anything"
        );
        let message = Withdrawal::superseded(&[constrained(7, &[9])])
            .expect("a supersession is news")
            .notice();
        assert_eq!(
            message.subject(),
            Subject::Document,
            "a supersession is about the document that superseded it, so \
             the act the document accepts next is what retires it"
        );
        assert_eq!(message.text(), notice);
        let update = frame_status(core::slice::from_ref(&message), &acting, None);
        assert_eq!(update, StatusUpdate::Show(message.clone()));

        let mut status = None;
        apply(&mut status, update);
        assert_eq!(status, Some(message));
    }

    #[test]
    fn a_supersession_says_the_cause_in_the_faults_own_words() {
        // The whole point of carrying the fault rather than the id: the
        // sentence names the mates AND the remedy, and neither string
        // is written here — both come from `DisplayFault`'s `Display`.
        let cause = DisplayFault::MateConstrained {
            instance: RecipeNodeId(3),
            mates: vec![RecipeNodeId(5)],
        };
        let notice = superseded_text(&[constrained(3, &[5])]).expect("news");
        assert!(
            notice.ends_with(&cause.to_string()),
            "the fault renders itself, verbatim: {notice}"
        );
        assert!(
            notice.contains("delete the mate(s)"),
            "so the remedy the typed value already knew reaches the line: {notice}"
        );

        // The delete arm, which is the other thing the bare id could
        // not say: an instance that is GONE says so, rather than being
        // named as if the tree still drew it.
        let gone = superseded_text(&[Withdrawn {
            instance: RecipeNodeId(4),
            cause: DisplayFault::NoSuchNode {
                node: RecipeNodeId(4),
            },
        }])
        .expect("news");
        assert_eq!(
            gone,
            "free move: a committed placement was discarded — node 4 is not in the document"
        );
    }

    #[test]
    fn a_dropped_hide_is_its_own_sentence_not_a_supersession() {
        // The decision this row pins: re-showing a fused instance is
        // NOT a supersession. Nothing replaced the user's choice — it
        // stopped being expressible — so the word "superseded" and the
        // free-move preamble are both absent, and the fault says which
        // of the two things happened to the picture.
        let fused = Withdrawn {
            instance: RecipeNodeId(3),
            cause: DisplayFault::FusedGeometry {
                instance: RecipeNodeId(3),
                root: RecipeNodeId(8),
                others: vec![RecipeNodeId(5)],
            },
        };
        let notice = dropped_hide_text(core::slice::from_ref(&fused)).expect("news");
        assert!(
            notice
                .starts_with("hide: a hide was dropped and the hidden geometry is drawn again — "),
            "its own preamble, not the free-move one — and the part being \
             back on screen, which is the whole reason this is not a \
             supersession, reaches the words the user reads: {notice}"
        );
        assert!(
            !notice.contains("free move") && !notice.contains("superseded"),
            "and it does not borrow the supersession's word: {notice}"
        );
        assert!(
            notice.ends_with(&fused.cause.to_string()),
            "the fault renders itself: {notice}"
        );

        // Both facts can arrive on one frame, and they are ranked
        // together as two notices rather than merged into one claim.
        let notices = [
            Withdrawal::superseded(&[constrained(7, &[9])])
                .expect("news")
                .notice(),
            Message::new(Subject::Document, notice.clone()),
        ];
        let StatusUpdate::Show(shown) = frame_status(&notices, &[SessionOp::Undo], None) else {
            panic!("two withdrawals are news");
        };
        assert!(shown.text().contains("free move:") && shown.text().contains("hide:"));
        assert_eq!(
            shown.subject(),
            Subject::Document,
            "two notices that agree on a subject are joined under it"
        );

        assert_eq!(dropped_hide_text(&[]), None);
    }

    #[test]
    fn a_frame_that_drops_two_hides_says_so_in_the_plural() {
        // Reachable in production: one boolean fusing two hidden
        // instances withdraws both hides in one prune.
        let fused = |instance: u64, other: u64| Withdrawn {
            instance: RecipeNodeId(instance),
            cause: DisplayFault::FusedGeometry {
                instance: RecipeNodeId(instance),
                root: RecipeNodeId(8),
                others: vec![RecipeNodeId(other)],
            },
        };
        let gone = Withdrawn {
            instance: RecipeNodeId(4),
            cause: DisplayFault::NoSuchNode {
                node: RecipeNodeId(4),
            },
        };

        let two = [fused(3, 5), fused(5, 3)];
        assert_eq!(
            dropped_hide_text(&two).expect("two dropped hides are news"),
            format!(
                "hide: 2 hides were dropped and the hidden geometry is drawn \
                 again — {}{NOTICE_SEPARATOR}{}",
                two[0].cause, two[1].cause
            ),
            "the plural agrees, and the consequence is said because both \
             withdrawals agree on it"
        );

        // A frame whose withdrawals DISAGREE about what the picture
        // now shows says only the part that is true of both.
        let mixed = [fused(3, 5), gone.clone()];
        let notice = dropped_hide_text(&mixed).expect("news");
        assert!(
            notice.starts_with("hide: 2 hides were dropped — "),
            "no consequence claimed over a frame that has two: {notice}"
        );

        // And the delete arm alone says the honest opposite: nothing
        // was re-shown, the instance went.
        assert_eq!(
            dropped_hide_text(core::slice::from_ref(&gone)).expect("news"),
            "hide: a hide was dropped with the instance it was on — \
             node 4 is not in the document"
        );
    }

    #[test]
    fn every_superseded_instance_is_named_and_none_means_silence() {
        // Not the first and not the last: one transition can discard
        // several probes (a mate lands on two probed instances, a
        // delete takes a subtree), and each is an instance the user
        // placed by hand and no longer has.
        let one = superseded_text(&[constrained(3, &[5])]).expect("one supersession is news");
        assert_eq!(
            one,
            "free move: a committed placement was discarded — \
             instance 3 is mate-constrained (mate node(s) 5): its pose is \
             mate-derived, so the free-move probe refuses — delete the mate(s) if \
             free relative motion is intended"
        );

        let two = [constrained(3, &[5]), constrained(11, &[5])];
        let both = superseded_text(&two).expect("two supersessions are still news");
        assert_eq!(
            both,
            format!(
                "free move: 2 committed placements were discarded — {}{NOTICE_SEPARATOR}{}",
                two[0].cause, two[1].cause
            ),
            "every word of the preamble agreeing with itself in number, and \
             the two faults joined by the one separator rather than composed \
             into a written claim about them. Asserted as the exact join and \
             not as a separator COUNT, which a fault whose own text contains \
             the separator would satisfy while reading as three causes"
        );

        // Silence has exactly one meaning here: nothing was discarded.
        assert_eq!(superseded_text(&[]), None);
    }
}
