//! The per-frame policies the viewport runs — as values, so they are
//! replayable.
//!
//! # What that sentence decides, and what it therefore excludes
//!
//! A policy belongs here when it is a pure function of ONE frame: hand
//! it the values that frame holds and it answers the same way every
//! time, with no window, no session and no process around it. That is
//! what makes a rule about the chrome testable at all, and these rules
//! used to sit inside [`crate::app::ViewerBehavior::viewport_ui`] and
//! `ViewerApp::perform_batch` — `app`-gated code no test can execute,
//! so the crate's own claim that "everything between event conversion
//! and painting is exercised by `tests/`" was false about exactly the
//! decisions most likely to be wrong.
//!
//! What is here is all of that shape. **What the chrome has to say and
//! which of its two channels says it**: the [`Subject`] / [`Message`] /
//! [`StatusUpdate`] / [`Badge`] vocabulary, the doors that build one,
//! the two that spend one ([`apply`] for a ranked verdict or a
//! retirement, [`deliver`] for a policy that may or may not have news),
//! and [`frame_status`]'s ranking over a frame's news. **The toolbar
//! badge for the landed product** ([`product_badge`]) and the rest of
//! the badge family beside it. **The draft and the offer a refused
//! batch leaves behind** ([`retype_draft`], [`creation_offer`]).
//! **What a folded event stream amounts to** ([`folded_moved`],
//! [`fold_status`]), **what a frame says about work outstanding**
//! ([`progress`]), and **where a file dialog opens** ([`dialog_dir`]).
//!
//! The frame loop still decides WHEN to call one. It no longer decides
//! what one MEANS.
//!
//! **The exclusions are the other half of the charter**, and they are
//! where this module has drifted before. A concern that reads AMBIENT
//! PROCESS STATE is a function of the machine rather than of the
//! frame, and cannot be replayed from any value a test builds: that is
//! [`crate::platform`]. A concern that carries state ACROSS frames is
//! not a function of one frame at all — the id pass's query
//! bookkeeping remembers what it asked and what it asked about, which
//! is the whole of why it works: that is [`crate::idpass`]. Both are
//! CONSUMED here — [`cursor_status`] takes an [`IdStep`] and turns it
//! into news — and neither is decided here.
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
//! A READER CONSULTS; a [`Message`] on the status line is the OUTCOME
//! OF SOMETHING THAT JUST HAPPENED. Both halves of the badge clause
//! are load-bearing, and the second half is the one that decides the
//! hard cases.
//!
//! A badge therefore outlives the frame that raised it while the line
//! carries one frame's news. That lifetime is the CONSEQUENCE of the
//! test, not the test: it is what the split reads like from outside,
//! and it cannot sort a fact that is both true after its frame and
//! provoked by one.
//!
//! **The channel is decided by PROVENANCE** — what caused the sentence
//! to exist — and not by what it is about (Ev, 2026-09-06). Every
//! refusal is about something and caused by something, so both are
//! coherent axes; provenance wins because it is the only one a reader
//! can SEE, in whether the sentence exists on a frame where nobody
//! acted.
//!
//! **"Held state" is the mechanical shadow of that, a strong
//! indicator and not a decision procedure**, and sorting the line's
//! writers on this paragraph needs the three ways it falls short said
//! out loud:
//!
//! * **It is a property of the FACT, not of a signature.**
//!   [`unindexed_refusal`] takes a `&NotIndexed` and nothing else;
//!   what makes it an outcome is that [`crate::pickcache::unindexed`]
//!   raises it for a `Select` and for nothing else, so the sentence
//!   exists because the user clicked. Reading it off the door is
//!   wrong; it has to be traced to whoever raises it.
//! * **Tracing that far does not settle it either.**
//!   [`crate::idpass::Disagreement`] reads only held state — the
//!   outstanding id answer, the index, the cursor — and
//!   [`IdStep::Hold`] recomputes
//!   it on every frame the cursor holds still, so the mechanical form
//!   alone badges it. What sorts it onto the line is *a reader
//!   CONSULTS a badge*: a claim about where the pointer is this
//!   instant is something a reader is told, not something they keep
//!   open and act against.
//! * **Whether a fact is held is a choice the author makes.**
//!   `ViewerApp`'s `scene_fault` and `projection_fault` did not exist
//!   until the badges that read them did, and any outcome can be made
//!   a read by storing it. So the mechanical form CONSTRAINS the
//!   answer and never supplies it: what it rules out is a badge with
//!   nothing to read.
//!
//! **What retires it.** Both channels carry a [`Subject`] — the
//! recurring event stream whose next event makes the thing the wrong
//! answer — and carrying one never decided which channel a fact goes
//! to. What differs is the ENFORCEMENT. A message is STORED as a
//! message, so retiring it is the chrome's own bookkeeping: [`apply`]
//! matches the held message's subject against a
//! [`StatusUpdate::Expire`] and drops it, and [`StatusUpdate::Clear`]
//! sweeps the line whole. **No such machinery touches a badge** — its
//! subject names the event that changes the state it reads, and the
//! badge ends because the read does.
//!
//! That is not the same as saying nobody keeps a badge alive. The
//! state a badge reads may itself be bookkept by hand: `ViewerApp`
//! clears `scene_fault` where a rebuild lands and [`crate::pane::viewport`]
//! clears `projection_fault` where a matrix forms, which is the same
//! work spelled as an assignment about the SEAM instead of a verdict
//! about the chrome. [`index_badge`] needs none for its refusal,
//! because the pick cache was already holding it; what it reads beside
//! that — the landed evaluation, through the tree's blame — is held by
//! the session and ends with the same landing. What the split buys is that
//! no writer has to decide the fate of anyone else's sentence.
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
//! the document refused, a pick a tool declined, a placement a later
//! edit withdrew — and which of a frame's news SHOULD win is
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
//! **Every writer but one reaches the line through that ranking**, and
//! the one that does not is named below. A writer is anything that can
//! put a sentence on the line, and it goes through the ranking by
//! writing to the frame's notices — `ViewerApp`'s `notices`, which
//! `ViewerBehavior` lends the panes.
//!
//! **Nothing enforces the "but one".** `ViewerBehavior` lends the panes
//! the field itself too (its `status`, for the retirements below), and
//! a pane that handed [`apply`] a [`StatusUpdate::Show`] there would put
//! a sentence on the line the ranking never saw, with nothing going
//! red. So this is the tree as read, not a guarantee. It gives no
//! number: the writers are a grep rather than a type, and a row that
//! counted them — the way `frame_policy.rs`'s
//! `the_readme_counts_its_two_populations_correctly` counts
//! `ViewerApp::store`'s reads — would go red at every new writer and
//! not at a writer that bypassed the ranking, which is the defect.
//!
//! The membership test is stated rather than inferred: a writer is
//! outside the ranking if it **can put a SENTENCE on the line that the
//! ranking never saw**. That is not the same as reaching the field
//! outside the ranking, and the difference is a retirement. A
//! retirement says nothing, so there is nothing to weigh it against
//! and nothing to join it to; ranking one is not a stricter discipline
//! but a category error. [`apply`] is
//! therefore a legitimate door and stays one — it is where a
//! retirement belongs — and [`cursor_status`], which returns only
//! [`StatusUpdate::Keep`] and [`StatusUpdate::Expire`], was never one
//! of these writers however directly it reaches the field.
//! [`deliver`] is the door for a policy that can answer either way:
//! news to the frame's notices, retirement to the field.
//!
//! **The exception is the startup initializer**, `ViewerApp::new`'s
//! `status: startup_notices(&notices)` — the preferences file's
//! complaints rendered into the field before any frame has run, where
//! the session's first accepted act silently deletes them. It cannot
//! simply join the notices: a complaint about the file as it stands is
//! a read of held state, so under this module's own rule it wants a
//! BADGE, and badging it means HOLDING it and deciding what retires
//! it. That is a design question, tracked as
//! `work/vseam/startup-notices-need-holding-to-badge.md` and not
//! asserted here as done.
//!
//! # The toolbar: held state, read
//!
//! A badge is a function of the typed value it reads, so each one's
//! SILENCE is a row a test can write; each states its own [`Tone`],
//! which is the actionable-or-not rule the toolbar used to pick a
//! colour for at four call sites; and one draw at the toolbar consumes
//! them all. The members are the at-rest verdict ([`at_rest_badge`]),
//! the advisory checks ([`checks_badge`]), the δ the display budget
//! chose ([`delta_badge`]), the product fault ([`product_badge`]), the
//! store that keeps no preferences ([`prefs_badge`]), the datums this
//! view draws nothing of ([`datums_badge`]), the profiles it draws
//! nothing of ([`profiles_badge`]), and the three display seams that
//! hold a refusal — the scene ([`scene_badge`]), the pick index
//! ([`index_badge`]) and the projection ([`projection_badge`]).
//! The population is every function here returning `Option<Badge>`,
//! which `frame_policy.rs` counts against the README rather than
//! against this sentence.
//!
//! # Two rules that follow, one per channel
//!
//! Both are values here rather than conditions at a call site.
//! [`fold_status`] never CLEARS for a camera fold:
//! clearing is the acting batch's verdict alone ([`batch_status`]),
//! because an action the document accepted is the one event that makes
//! a standing complaint stale, and a fold that cleared would be
//! deciding the fate of messages written by everyone else in the same
//! frame. And the gather's verdict badges rather than writes, because
//! a fault about the document on screen outlives every frame the
//! camera moves in.

use std::path::Path;

use pncad::document::{
    ChecksReport, Evaluation, Maintenance, ParamName, ParseError, ProductError, ProductErrorKind,
    RecipeNodeId, SlotId,
};
use pncad::select::{HitTestError, NodePickError};

use crate::camera::CameraError;
use crate::camera::Folded;
use crate::display::{AdmissionFault, PruneReport, Withdrawn};
use crate::idpass::IdStep;
use crate::pickcache::NotIndexed;
use crate::pickindex::{PickError, PickIndexError};
use crate::prefs::{StoreError, Unusable};
use crate::scene::FittedDelta;
use crate::scene::SceneError;
use crate::session::{AtRestBadge, OpOutcome, Outstanding, Refusal, SessionOp};
use crate::vocab::{partial_mirror, vocabulary};

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
    /// projects is a camera whose badge is gone. This `Expire` never
    /// reached the projection sentence in any case, because both
    /// `land` calls run earlier in the same [`crate::app::ViewerBehavior::viewport_ui`]
    /// that writes it.
    Camera,
    /// **The cursor and what lies under it** — retired by the next
    /// cursor move, and by the pointer leaving the pane.
    ///
    /// Issued by [`cursor_status`], off the id pass's own bookkeeping:
    /// a message about what was under the cursor is stale exactly when
    /// the outstanding pick question is, which is a judgement
    /// [`crate::idpass::IdQueryLog`] already makes.
    Cursor,
    /// **The document on screen and the acts aimed at it** — retired
    /// by the next batch holding an operation [`acts`] counts: swept by
    /// [`StatusUpdate::Clear`] when that batch refused nothing, and
    /// replaced by the refusal when it did.
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
    ///
    /// Like [`Self::Camera`] and [`Self::Display`] it wears both
    /// channels, and for the same reason: a write that was attempted
    /// and failed is an outcome ([`store_refusal`]), while a store
    /// that can never be written at all is a read of held state that
    /// no write ever retires ([`prefs_badge`]).
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
///
/// **Deliberately partial, and told when [`Subject`] grows.** Nothing
/// forces this list to be complete — completeness is what it does not
/// claim, and the three above belong out of it. What the
/// `partial_mirror!` invocation below forces
/// (`crates/viewer/src/vocab.rs` declares the macro) is that every
/// subject is either offered at a seat of this list or named there as
/// deliberately absent with its reason. A sixth subject WITH an issuer
/// would otherwise miss the list with no row going red, and its
/// messages would then be swept only by [`StatusUpdate::Clear`],
/// silently. The
/// suite's own row over this list holds a different direction — that
/// the two named here are the two the policies it calls actually
/// issue — and cannot see a policy it does not call.
pub const SUBJECTS_WITH_AN_EXPIRY_ISSUER: [Subject; 2] = [Subject::Camera, Subject::Cursor];

partial_mirror! {
    Subject, bare SUBJECTS_WITH_AN_EXPIRY_ISSUER,
    offered [Camera, Cursor],
    absent [
        Document => "its event is the next act the document ACCEPTS, \
                     which nothing marks yet; what sweeps it today is \
                     the subject-blind `StatusUpdate::Clear`",
        Display => "its event is the next rebuild of the thing the \
                    message is about, which nothing marks yet; the \
                    held facts about the picture badge instead, and \
                    the news that does wear this subject is swept only \
                    by `StatusUpdate::Clear`",
        Preferences => "its event is the next write of the preferences \
                        file, which nothing marks yet; swept only by \
                        `StatusUpdate::Clear`, for `Display`'s reason",
    ],
}

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
///
/// **A message's text carries no [`NOTICE_MARK`], and that is what
/// makes a joined line readable.** [`frame_status`] puts several
/// notices on one line separated by [`NOTICE_SEPARATOR`], so a reader
/// can only find the boundaries if the mark the separator is built
/// from means "a new notice starts here" and nothing else. That is a
/// claim about every string any producer will ever hand this type,
/// which no signature can carry — so the only door enforces it
/// instead of asserting it, and the invariant belongs to the value
/// rather than to the one function that happens to join today.
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Message {
    subject: Subject,
    text: String,
}

impl Message {
    /// A message about `subject`, in `text`'s own words — with any
    /// [`NOTICE_MARK`] in them rewritten to the within-a-notice mark.
    ///
    /// **Rewritten rather than refused.** A door that panicked would
    /// be reachable from the keyboard: [`delta_not_a_number`] echoes
    /// what the user typed into the δ field, so a pasted bullet would
    /// take the application down. And a text that asks for a boundary
    /// mark is asking for a list mark one level in — it is inside a
    /// notice, which is what [`LIST_SEPARATOR`] is for — so the
    /// rewrite says what the author meant at the level they are at.
    pub fn new(subject: Subject, text: impl Into<String>) -> Self {
        Self {
            subject,
            text: text.into().replace(NOTICE_MARK, LIST_SEPARATOR.trim()),
        }
    }

    /// **One line carrying several notices**, separated by
    /// [`NOTICE_SEPARATOR`] — the rank-2 line [`frame_status`]
    /// composes, and the startup line [`startup_notices`] composes
    /// before any frame has run.
    ///
    /// **The one place a [`NOTICE_MARK`] is written into a message's
    /// text, and the reason [`Message::new`] can rewrite every
    /// other.** It takes [`Message`]s and not strings, so the texts it
    /// joins already satisfy the invariant and nothing it produces
    /// can be read as a boundary that was not one. A joiner handed
    /// raw strings would be a second way to mint the mark, which is
    /// the same objection [`Message::new`]'s private fields answer one
    /// level up — so this is private to the module, and the public
    /// door cannot express it.
    ///
    /// The caller decides the subject, because what a joined line is
    /// ABOUT is a separate question with its own rule
    /// ([`joined_subject`]).
    fn joined(subject: Subject, notices: &[Message]) -> Self {
        Self {
            subject,
            text: notices
                .iter()
                .map(Message::text)
                .collect::<Vec<_>>()
                .join(NOTICE_SEPARATOR),
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

/// **The message's own words, and only those.**
///
/// Destructured rather than field-read, so a field added to
/// [`Message`] is E0027 here and its author has to decide whether the
/// line says it. `subject` is the standing decision that it does not:
/// the subject ROUTES the message — it is what retires it
/// ([`StatusUpdate::Expire`]) and what a joined rank-2 line takes as
/// its own subject, so one recurring event can retire the joined
/// sentence. It does not RANK: [`frame_status`] ranks by SOURCE — a
/// refusal, else the frame's notices, else the batch's own verdict —
/// and no rank reads a subject. A line that printed its own routing
/// would be saying to the user what the chrome says to itself.
impl core::fmt::Display for Message {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let Self { subject: _, text } = self;
        f.write_str(text)
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

/// **Hand a policy's verdict to the frame**: what it has to SAY joins
/// the frame's notices, and what it retires goes straight to the field.
///
/// The two halves of a [`StatusUpdate`] reach the line by different
/// routes, and the difference is the whole of what this module ranks.
/// A [`StatusUpdate::Show`] is news — a sentence that competes with
/// every other sentence this frame produced, so it goes on `notices`
/// and meets [`frame_status`]'s ranking, which is what stops the same
/// frame's accepted batch from erasing it before it is painted.
/// [`StatusUpdate::Keep`] and [`StatusUpdate::Expire`] say nothing and
/// therefore compete with nothing: `Keep` is the absence of news
/// spelled as a decision, and `Expire` is a RETIREMENT, which must
/// reach the field directly because a notice cannot un-say anything.
///
/// **This is the door for a policy that may or may not have something
/// to say**, and [`fold_status`] is the only one there is: its refusal
/// is news and its clean arm retires the camera sentence. Read off
/// `deliver`'s callers rather than off the shape — there is one
/// production call site, [`crate::pane::viewport::land`]. A writer that already
/// knows it has a [`Message`] pushes onto `notices` itself; a writer
/// that assigns the field has no way to say "I have nothing to add",
/// which is the defect [`apply`]'s docs describe and this door removes
/// for the policies.
///
/// **Every arm is written out**, and a wildcard for the three
/// non-`Show` ones would defeat the whole door: it would route a
/// variant added later to the field by default, which is exactly the
/// defect this exists to stop, and it would be added at a diff where
/// nothing looked wrong. The variant that most wants that treatment is
/// the one it would be most wrong for — a future `Show`-shaped arm is
/// news by construction. So the compiler carries the rule, and the
/// three arms below say which side each of today's is on rather than
/// leaving it to be read off a binding's name.
pub fn deliver(notices: &mut Vec<Message>, status: &mut Option<Message>, update: StatusUpdate) {
    match update {
        // News: it competes, so it must be ranked.
        StatusUpdate::Show(message) => notices.push(message),
        // Nothing to say, so nothing to rank. `Clear` is not a
        // retirement — it is a subject-blind sweep — but it is on this
        // side for the same reason `Expire` is: it takes something
        // away rather than adding to what the frame has to say, and a
        // notice cannot un-say anything.
        StatusUpdate::Keep => apply(status, StatusUpdate::Keep),
        StatusUpdate::Expire(subject) => apply(status, StatusUpdate::Expire(subject)),
        StatusUpdate::Clear => apply(status, StatusUpdate::Clear),
    }
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
        (_, Some(refusal)) => StatusUpdate::Show(refusal_message(refusal)),
        (true, None) => StatusUpdate::Clear,
        (false, None) => StatusUpdate::Keep,
    }
}

/// **A [`Refusal`] as the line carries it** — the one place a refusal
/// becomes a [`Message`], whichever way it reached the frame.
///
/// A refusal is the document's answer to an act it was asked for, so
/// its subject is [`Subject::Document`]: it stops being the news when
/// the document accepts one. Most arrive as an operation's outcome
/// ([`batch_status`]); a refusal the chrome meets before any operation
/// could carry the value — [`crate::widgets::value_field_ops`]'s typed
/// number that [`crate::props::SlotValue::of`] refuses, the same
/// refusal a drag of that value gets from the session's gesture door —
/// goes onto the frame's notices through this same door, so the two
/// routes say one sentence rather than two spellings of it.
pub fn refusal_message(refusal: &Refusal) -> Message {
    Message::new(Subject::Document, refusal.to_string())
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
///    happened, joined with [`NOTICE_SEPARATOR`] — the same boundary
///    the preferences path writes between its own startup notices
///    ([`startup_notices`]). Not the last one: assigning
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
///
/// # The join is invertible, and that is the whole rule
///
/// `line.split(NOTICE_SEPARATOR)` returns exactly the texts that went
/// in, in order. A reader scanning for the boundary finds `n - 1` of
/// them in a line carrying `n` notices and no more, because
/// [`Message::new`] takes [`NOTICE_MARK`] out of every text that
/// reaches it.
///
/// **The rule lives on the type, in two halves.** "No notice contains
/// the separator" is a claim about strings, and no signature carries
/// it: a joiner cannot check what it was given without either
/// refusing a value the user caused or lying about it. What the crate
/// CAN do is make the claim true where every notice is built and
/// leave exactly one place able to write the mark — [`Message::new`]
/// takes it out, [`Message::joined`] puts it in, and `joined` takes
/// [`Message`]s rather than strings, so the only way to a boundary
/// mark is to have had two notices. Both halves are private-field
/// consequences, so this function asserts nothing and a second joiner
/// written tomorrow inherits the guarantee instead of re-deriving
/// it.
///
/// **Why not the alternatives.** Escaping at the join reads the same
/// only until it fires, and then it shows the reader a sentence its
/// author did not write, on a line whose whole job is to report
/// accurately. Making the line stop being one string — several
/// labels, one per notice — is the better answer and is not this
/// function's to give: it needs a value that carries several
/// subjects, and
/// `work/vnews/one-line-one-subject-loses-a-mixed-frames-expiry.md`
/// owns that fork. Until then the line is one string, and one string
/// needs a mark.
pub fn frame_status(
    notices: &[Message],
    ops: &[SessionOp],
    refusal: Option<&Refusal>,
) -> StatusUpdate {
    match batch_status(ops, refusal) {
        refused @ StatusUpdate::Show(_) => refused,
        verdict if notices.is_empty() => verdict,
        _ => StatusUpdate::Show(Message::joined(joined_subject(notices), notices)),
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
/// **The notices no longer agree, and the fallback is live.** They
/// once did — a tool's declined pick, a tool's survival drop, a
/// supersession and a dropped hide are all provoked by the frame's own
/// document transition, so `Document` was the only subject on the
/// list. The sweep that routed every writer through the ranking put
/// four more on it: [`Subject::Camera`] ([`fold_status`]'s refused
/// fold, delivered at [`crate::pane::viewport::land`]),
/// [`Subject::Cursor`] ([`crate::idpass::Disagreement::notice`]),
/// [`Subject::Display`] (the pick index's
/// refused click and the δ field's two doors, through
/// [`PICK_INDEX_SEAM`] and [`SCENE_SEAM`]) and [`Subject::Preferences`]
/// ([`store_refusal`]).
///
/// So a frame that produces two of them reaches this arm today — a
/// create-pane refusal and a camera fold refusal are one drag apart —
/// and the consequence is a defect rather than a curiosity: the joined
/// line is about `Document`, which has no [`StatusUpdate::Expire`]
/// issuer ([`SUBJECTS_WITH_AN_EXPIRY_ISSUER`]), so the camera event
/// that would have retired the camera half no longer can. That is
/// `work/vnews/one-line-one-subject-loses-a-mixed-frames-expiry.md`,
/// which owns the fork; this function is the rule it is a consequence
/// of, and the rule is unchanged.
fn joined_subject(notices: &[Message]) -> Subject {
    let mut subjects = notices.iter().map(|notice| notice.subject());
    match subjects.next() {
        Some(first) if subjects.all(|subject| subject == first) => first,
        _ => Subject::Document,
    }
}

/// **The mark that means "a new notice starts here"** — the one
/// character a [`Message`]'s text may not carry, which
/// [`Message::new`] is what makes true.
///
/// A bullet and not punctuation, deliberately. The marks a notice's
/// own sentence uses are punctuation — a semicolon between the items
/// of a list it carries, an em-dash between a preamble and what it
/// introduces — and a boundary between two separate pieces of news
/// has to be legible as something other than more of the same
/// sentence. Any in-band mark is a claim about the text it sits in;
/// this is the claim the crate can actually hold, because no notice
/// writes prose with a bullet in it and the door makes that true
/// rather than hoping for it.
pub const NOTICE_MARK: char = '\u{2022}';

/// **What [`frame_status`] puts between two of a frame's notices**,
/// and [`startup_notices`] between two of the preferences file's.
///
/// Built from [`NOTICE_MARK`], so the joined line splits back into
/// exactly the notices it was made from: there are `n - 1` of these
/// in a line carrying `n` notices, never more, whatever any notice's
/// own text says.
///
/// It was [`LIST_SEPARATOR`] until this rule existed, and the two
/// being one spelling is what made the line ambiguous at two notices
/// — a [`Withdrawal`] joins its own causes with the same string, and
/// `DisplayFault::NonRigidFrame` writes one inside a single sentence,
/// so a reader met a separator that might be a boundary or might be
/// the notice talking. The inner level is answered by its element
/// type rather than by a second mark
/// ([`crate::display::AdmissionFault`]).
pub const NOTICE_SEPARATOR: &str = " \u{2022} ";

/// **What ONE notice puts between the items of a list of its own** —
/// a [`Withdrawal`]'s causes, which are the items its counted preamble
/// introduces. The preferences path's startup notices were joined with
/// this and are not such a list: they are several notices and take the
/// boundary mark ([`startup_notices`]).
///
/// A level in from [`NOTICE_SEPARATOR`], and spelled differently for
/// that reason: the two levels are two questions, and one spelling
/// could not answer both. A notice's text may carry this mark freely,
/// which is exactly why the boundary between notices is not it.
pub const LIST_SEPARATOR: &str = "; ";

/// **What an accepted edit WITHDREW from the display state**, as a
/// notice for [`frame_status`]'s rank 2.
///
/// # One value, not a function per kind
///
/// A supersession, a dropped hide and a killed gesture are the same
/// class of fact — display state an accepted edit took away, each
/// carrying the [`AdmissionFault`] the prune withdrew it on — and the
/// first two were free functions composing prose that differed in four
/// format literals.
/// They are a typed value with a `Display` here, which is the shape
/// the crate's other notices already have ([`crate::tools::ToolNotice`],
/// [`crate::prefs::Notice`]) and the shape [`crate::tree::RowStatus`] is the model for:
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
/// Its subject is [`Subject::Document`], so what retires it is
/// [`StatusUpdate::Clear`] — and [`acts`] makes that the next thing
/// the user DOES other than hovering, which is narrower than "the next
/// act the document accepts". Three legs, each with a row:
///
/// - **A hover leaves it standing.** [`batch_status`] answers
///   [`StatusUpdate::Keep`] for a batch of nothing but
///   [`SessionOp::Hover`], so the pointer drifting over the viewport
///   does not take the sentence
///   (`a_hover_only_batch_leaves_the_status_line_alone`).
/// - **Navigation leaves it standing.** A camera fold is not a
///   [`SessionOp`] at all — [`crate::camera::CameraOp`] is its own
///   vocabulary — so it never reaches [`batch_status`], and the
///   retirement it does issue names [`Subject::Camera`] and passes a
///   `Document` message by (`a_clean_fold_keeps_a_message_it_did_not_write`,
///   and `landing_a_clean_fold_does_not_clear_a_message_it_did_not_write`
///   on the live path).
/// - **The next non-hover operation takes the line off it**, whatever
///   that operation's own verdict is: [`StatusUpdate::Clear`] where the
///   document accepted it, the refusal's own sentence where it did not
///   (`a_supersession_survives_the_accepted_edit_that_caused_it`,
///   `an_acting_frame_sweeps_the_line_a_seam_refusal_would_have_been_on`).
///
/// **That is the lifetime this fact should have, and it is TIGHTER
/// than the per-subject alternative a reader reaches for.** A
/// supersession reports something COMPLETED — an accepted edit
/// discarded a committed hand placement — so it cannot become false
/// with age, only stale, and the question is never whether it is still
/// true but whether the reader has moved on. The earliest honest
/// evidence of that is the user doing something that is not moving the
/// pointer, and that is exactly what `Clear` reads.
///
/// Making the subject the withdrawn INSTANCE instead, retired by that
/// instance's own next event, runs the wrong way: a sentence about
/// instance A would survive a selection of B, a hide of C and an edit
/// elsewhere, so the news would live longest exactly where the reader
/// has visibly left it. It would also INTRODUCE the one failure this
/// lifetime does not have — the line contradicting the picture while
/// the user re-places A by hand — because re-placing A is a non-hover
/// operation and `Clear` has already taken the sentence, where a
/// per-instance rule would have to get "A's own event" right to do the
/// same.
///
/// Retiring on the frame boundary instead is tighter still and is not
/// available: a sentence that lives one frame at sixty frames a second
/// is one nobody reads, so the frame is not a subject a reader can use
/// — which is why the fact being true of nothing after its own frame
/// does not make the frame its subject.
///
/// It reaches the line through the frame's NOTICES rather than by
/// assignment, for the reason [`frame_status`] states: the transition
/// that withdraws is an edit the document accepted, so the same
/// frame's batch verdict is [`StatusUpdate::Clear`].
///
/// A refusal in the same frame outranks it and it is then not shown,
/// which rank 1 already says. The two cannot come from one operation:
/// a refused op returns before the prune that fills the report.
///
/// # The cause is the fault's own sentence
///
/// **Nothing here composes prose about why a placement or a hide
/// went.** Each entry carries the [`AdmissionFault`] the prune
/// discarded on, and this renders it through its own `Display` — the rule the
/// rest of the crate follows. So the commonest arm names the mates and
/// the remedy (`MateConstrained`: *delete the mate(s) if free relative
/// motion is intended*), a fuse names the product and the instances
/// fused into it, and a deleted instance says the document does not
/// hold the node — which is the delete arm's whole point, since the id
/// alone names something the tree no longer draws without saying that
/// is why.
///
/// The frame around the faults counts where there is anything to count,
/// and never names: every [`AdmissionFault`] names its own SUBJECT, so
/// naming the id again in the preamble would say it twice. It does not
/// promise a vocabulary for that subject: an arm about an instance says
/// "instance N" and an arm about an id that names no instance
/// (`NoSuchNode`, `NotAnInstance`) says "node N", which is that enum's
/// own rule and the only honest wording there.
///
/// [`crate::display::DisplayFault`]'s own arms name no id at all — they are about a
/// gesture or a frame rather than a node — and a `Withdrawn` cannot
/// carry one: the admission tests answer [`AdmissionFault`] and
/// `Withdrawn::cause` is typed as what they answer. That used to be a
/// property of two functions' error sets with nothing in a type
/// saying so.
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Withdrawal<'a> {
    /// Which of the three this is.
    pub kind: WithdrawalKind,
    /// What went. **Never empty** — the constructors are the only
    /// door and each answers `None` for an empty set, so the
    /// rendering below never has to word "nothing was withdrawn".
    withdrawn: &'a [Withdrawn],
}

vocabulary! {
    /// Which display state an accepted edit took away.
    #[derive(Clone, Copy, Debug, PartialEq, Eq)]
    pub enum WithdrawalKind {
        /// **A SUBSTITUTION.** The user's hand placement answered "where
        /// does this part go", and the mate that landed answers it better,
        /// so the probe steps aside and the picture keeps the part.
        ///
        /// [`crate::display::PruneReport::superseded`], carried to the
        /// chrome on [`crate::session::OpOutcome::withdrawn`], names the
        /// instances whose COMMITTED free-move placement an operation's document
        /// transition discarded — the G3 supersession, reported by the
        /// session rather than inferred ([`crate::display::DisplayState::prune`] is
        /// where it happens, and [`crate::display::free_move_check`] is the
        /// condition). A killed in-flight gesture is NOT in that list, so
        /// it is not this channel's to report; the next gesture op refuses
        /// typed instead.
        ///
        /// What it leaves behind is the instance drawn at its landed
        /// placement, which the picture already says; a badge would keep
        /// saying it about a document the user has moved on from, and
        /// nobody consults a toolbar to learn where a part ended up.
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
        /// **Neither of the above, and the only one the user's hand was
        /// on.** A drag was in flight and the document stopped admitting
        /// the instance under it, so the gesture ended where it stood.
        ///
        /// Nothing substituted for it: the placement it would have landed
        /// was never asked of the document, so this is not a
        /// supersession — and no hide was involved, so it is not that
        /// either. What it leaves behind is the instance drawn where the
        /// document puts it, and a hand that was steering something a
        /// moment ago.
        ///
        /// **Exactly one, never a set.** A session holds one free-move
        /// gesture, so this kind has no plural to word — which is why the
        /// rendering below counts the other two and not this one, and why
        /// this kind's producer takes an `Option` rather than a slice.
        KilledGesture,
    }

    /// Every kind of withdrawal, in the order [`Withdrawal::all`]
    /// produces them — the roster
    /// `every_withdrawal_kind_has_a_producer` holds that fan-out
    /// against, so a kind cannot join this enum without a report field
    /// that reaches it. That row is its only reader: the chrome words
    /// each kind through this type's `Display` and never scans the
    /// list.
    pub const ALL;
}

impl<'a> Withdrawal<'a> {
    /// **Every withdrawal a prune reported**, in the report's own
    /// field order — the ONE door from a [`PruneReport`] to the
    /// sentences the chrome shows, and the only one outside this
    /// module.
    ///
    /// **Destructured rather than field-read**, so a fourth kind of
    /// withdrawal is E0027 *here*, at the call that words it. That is
    /// the site the omission actually happens at: the copy onto
    /// [`crate::session::OpOutcome`] was already exhaustive and the
    /// fan-out was not, so a fourth field reached the chrome's channel
    /// and was never worded — twice, once per kind added.
    ///
    /// The three producers below are PRIVATE for the same reason. A
    /// caller that could reach one could fan out by hand again, which
    /// is a list of kinds nothing holds to the report's; a caller that
    /// can only reach this one gets the report's list or a compile
    /// error. The reverse direction — a [`WithdrawalKind`] with no
    /// report field behind it — is held by
    /// `every_withdrawal_kind_has_a_producer` against
    /// [`WithdrawalKind::ALL`].
    ///
    /// Empty kinds are absent rather than silent entries, which is
    /// [`Withdrawal::of`]'s decision and not this one's: a frame that
    /// withdrew nothing yields nothing to say.
    pub fn all(report: &'a PruneReport) -> impl Iterator<Item = Self> {
        let PruneReport {
            superseded,
            dropped_hides,
            killed_gesture,
        } = report;
        [
            Self::superseded(superseded),
            Self::dropped_hide(dropped_hides),
            Self::killed_gesture(killed_gesture.as_ref()),
        ]
        .into_iter()
        .flatten()
    }

    /// The frame's supersessions, or `None` when it superseded nothing.
    fn superseded(withdrawn: &'a [Withdrawn]) -> Option<Self> {
        Self::of(WithdrawalKind::Superseded, withdrawn)
    }

    /// The frame's dropped hides, or `None` when it dropped none.
    fn dropped_hide(withdrawn: &'a [Withdrawn]) -> Option<Self> {
        Self::of(WithdrawalKind::DroppedHide, withdrawn)
    }

    /// The gesture the frame killed, or `None` when it killed none.
    ///
    /// **An `Option`, not a slice**, because there is at most one
    /// gesture to kill ([`WithdrawalKind::KilledGesture`]). It becomes
    /// the one-element slice the rendering shares with the other two
    /// kinds, so the cause is rendered by its own `Display` here
    /// exactly as it is there.
    fn killed_gesture(killed: Option<&'a Withdrawn>) -> Option<Self> {
        Self::of(
            WithdrawalKind::KilledGesture,
            killed.map_or(&[][..], core::slice::from_ref),
        )
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

/// **Every notice one operation's outcome carries**, for
/// [`frame_status`]'s rank 2 — the ONE door from an [`OpOutcome`] to
/// the frame's notices, and the call `app` makes per operation.
///
/// Two kinds of news, both provoked by the act the user just took and
/// both true of the document it left: what the transition WITHDREW
/// from the display state ([`Withdrawal::all`]), then what the
/// committed edits did that the user did not ask for by name
/// ([`maintenance_notice`], one notice per row in the outcome's own
/// order). They are notices rather than a verdict for [`Withdrawal`]'s
/// reason: the edit that produced them was accepted, so the same
/// frame's batch verdict is [`StatusUpdate::Clear`], which they
/// outrank. What takes them off the line is the next frame whose batch
/// holds any operation [`acts`] counts — [`batch_status`] answers it
/// with [`StatusUpdate::Clear`] when nothing refused and with the
/// refusal otherwise; a hover-only batch keeps them.
///
/// **Destructured rather than field-read**, so a field added to
/// [`OpOutcome`] is E0027 here and its author decides whether the
/// line says it. The four this does not word are not news the line
/// owes: `committed` and `previewed` are the act itself, `minted` is
/// an id a form reads back, and `refusal` is ranked above every
/// notice by [`frame_status`] on its own.
pub fn outcome_notices(outcome: &OpOutcome) -> impl Iterator<Item = Message> + '_ {
    let OpOutcome {
        committed: _,
        previewed: _,
        minted: _,
        refusal: _,
        withdrawn,
        maintenance,
    } = outcome;
    Withdrawal::all(withdrawn)
        .map(|withdrawal| withdrawal.notice())
        .chain(maintenance.iter().filter_map(maintenance_notice))
}

/// **One maintenance row as a notice**, or `None` for a row the line
/// does not carry.
///
/// **The row's own sentence, unaltered.** Each arm of [`Maintenance`]
/// words itself (`Display for Maintenance`), naming the carrier and
/// what the edit removed or rewrote; nothing here composes prose about
/// it, for the rule [`Withdrawal`]'s causes follow. One notice per row
/// rather than one per kind joined with [`LIST_SEPARATOR`], because a
/// strand's own sentence writes that mark and a flat join of such
/// sentences could not be split back into its rows; the boundary mark
/// between notices is the one no sentence can carry ([`Message::new`]).
///
/// **Every arm DM7 makes the door report is worded**: a stranded
/// payload name, a stranded appearance key, and a declaration left
/// with no consumer.
///
/// **A cluster act is not**: it re-keys the mate graph's placement
/// registry — a gauge instance and a frame, bookkeeping the chrome
/// names nowhere — and what it decided about where the parts sit is
/// what the picture draws. It still rides [`OpOutcome::maintenance`],
/// where a reader of the API sees it.
///
/// The match names every arm, so a fifth is a compile error here
/// rather than a row that reaches the outcome and is never worded.
pub fn maintenance_notice(row: &Maintenance) -> Option<Message> {
    match row {
        Maintenance::Strand { .. }
        | Maintenance::StrandedAppearance { .. }
        | Maintenance::OrphanedDeclare { .. } => {
            Some(Message::new(Subject::Document, row.to_string()))
        }
        Maintenance::Cluster(_) => None,
    }
}

/// **Destructured rather than field-read**, so a field added to
/// [`Withdrawal`] is E0027 here rather than joining a value whose
/// whole job is to word itself and going unworded.
impl core::fmt::Display for Withdrawal<'_> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let &Self {
            kind: which,
            withdrawn,
        } = self;
        let fused = |w: &Withdrawn| matches!(w.cause, AdmissionFault::FusedGeometry { .. });
        // The two kinds that are over a SET word themselves by
        // counting it. The third is over the one gesture that can be
        // in flight, so it has no plural and is NOT given one: a
        // `many` string its own constructor cannot reach is prose
        // nothing can ever print.
        let (kind, one, many, consequence) = match which {
            WithdrawalKind::Superseded => (
                "free move",
                "a committed placement was discarded",
                Some("committed placements were discarded"),
                "",
            ),
            WithdrawalKind::DroppedHide => (
                "hide",
                "a hide was dropped",
                Some("hides were dropped"),
                if withdrawn.iter().all(fused) {
                    " and the hidden geometry is drawn again"
                } else if withdrawn.iter().any(fused) {
                    ""
                } else {
                    " with the instance it was on"
                },
            ),
            WithdrawalKind::KilledGesture => {
                ("free move", "the drag in flight was ended", None, "")
            }
        };
        let count = withdrawn.len();
        match many.filter(|_| count > 1) {
            Some(many) => write!(f, "{kind}: {count} {many}{consequence} — ")?,
            None => write!(f, "{kind}: {one}{consequence} — ")?,
        }
        // Each cause rendered by its own `Display`, in the order the
        // prune found them, joined with [`LIST_SEPARATOR`] rather than
        // composed into a sentence, for the reason [`frame_status`]
        // joins notices: a list of several typed values must not
        // become one written claim about them.
        //
        // This is the WITHIN-a-notice level, so it takes the
        // within-a-notice mark. The whole rendering is one notice and
        // [`frame_status`] puts [`NOTICE_SEPARATOR`] around it, so
        // these marks cannot be read as boundaries between notices.
        //
        // The join is flat, so a cause whose own text contains
        // [`LIST_SEPARATOR`] would nest inside it and a reader could
        // not see where one cause ends. What keeps that from being a
        // hope about wording is the element type: this joins
        // [`AdmissionFault`]s, whose sentences are the whole
        // population the claim ranges over and which cannot gain one
        // without an arm in its `Display`. `DisplayFault::NonRigidFrame`
        // writes the mark inside one sentence and is outside that
        // enum, so this join cannot reach it.
        for (position, entry) in withdrawn.iter().enumerate() {
            if position > 0 {
                f.write_str(LIST_SEPARATOR)?;
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
/// the outstanding pick question is, and
/// [`crate::idpass::IdQueryLog::step`] already
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
/// [`crate::idpass::Disagreement`]'s, raised where the two picking
/// paths are compared.
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
/// theme's `unresolved` for a verdict they may. The feature tree's
/// rows follow the same rule, stated by [`crate::tree::RowStatus::tone`]
/// — a poisoned row is [`Tone::Advisory`], deliberately QUIET, so the
/// eye goes to the failed row a reader can do something about. A
/// badge, a row, and a pane message drawn through
/// `widgets::message_toned` each hand over a `Tone` rather than a
/// style, and `app::toned` is where a tone becomes one: `weak` for
/// `Advisory`, the theme's `unresolved` for `Actionable`.
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
/// *checks*, *δ*, *scene*, *pick index*, *projection* — and that is
/// not the chrome writing prose about another value's failure. The
/// failure's own words are the typed value's, rendered through its own
/// `Display` and unaltered; what the chrome adds is which of the
/// badges the reader is looking at, which is a fact about the toolbar
/// and about nothing else.
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
    ///
    /// **Read it to ask which event stream a badge belongs to.** It is
    /// what a door answers when it adds a badge and what the rows
    /// assert about the answer; nothing in the draw path consults it,
    /// and it has no production reader today. That is the shape a
    /// subject has on this channel — see the type's own docs — and not
    /// an oversight to be closed by finding one.
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

/// **A seam's subject, read off the type of its refusal.**
///
/// A seam can speak on both channels — the pick index badges the
/// build it is holding a refusal for ([`index_badge`]) and puts a
/// refused CLICK on the line ([`unindexed_refusal`]) — and the crate's
/// rule is that one seam must not speak with two voices. A subject
/// written as a literal at each door is a convention: two doors, two
/// literals, and nothing but a reader to notice when they drift.
///
/// **What this buys, exactly.** A door answers from the type it was
/// handed rather than from what its author thought, so a door and its
/// refusal cannot disagree; and where one seam's refusal arrives as
/// two types, both impls name ONE constant below, so the seam's
/// subject is one edit and the two channels move together. That is
/// what [`tool_news`] buys for its call sites by having one door,
/// done for a seam that needs two.
///
/// **What it does not buy.** It covers a seam's own refusal TYPE, so
/// a door whose input is not one — [`tool_news`], [`startup_notices`]
/// — spells its subject and says so at the door. And a shared
/// [`Subject`] is not a shared seam: the scene (which the δ field's
/// doors share, [`SCENE_SEAM`]) and the pick index
/// ([`PICK_INDEX_SEAM`]) are separate seams under
/// [`Subject::Display`], which is the coarser question of what retires
/// a fact.
///
/// Not public: the doors below are the API, and a caller that could
/// read this could also assign a subject without one.
trait SeamSubject {
    /// The event stream whose next event makes this seam's refusal the
    /// wrong answer.
    const SUBJECT: Subject;
}

/// **The pick-index seam's subject**, named by both of the types its
/// refusals arrive as. One edit here moves both channels; that is the
/// "by construction" the trait's argument rests on, and it is written
/// as a constant because a seam whose two impls each spelled a literal
/// would be back to the convention.
const PICK_INDEX_SEAM: Subject = Subject::Display;

/// **The scene seam's subject**, named by the rebuild's refusal and by
/// the δ field's two doors — the δ on screen and the mesh drawn at it
/// are one seam, and [`delta_not_a_number`] never reaches a
/// [`SceneError`] to be typed by.
const SCENE_SEAM: Subject = Subject::Display;

/// The camera and the viewport it is projected into.
impl SeamSubject for CameraError {
    const SUBJECT: Subject = Subject::Camera;
}

/// The picture drawn from the document — the build that lands next is
/// what ends it.
impl SeamSubject for SceneError {
    const SUBJECT: Subject = SCENE_SEAM;
}

/// The pick index seam, on the badge channel.
impl SeamSubject for PickIndexError {
    const SUBJECT: Subject = PICK_INDEX_SEAM;
}

/// The pick index seam, on the line: the same seam, so the same
/// constant, which is the whole reason this is not a literal at a
/// door.
impl SeamSubject for NotIndexed {
    const SUBJECT: Subject = PICK_INDEX_SEAM;
}

// # The subject-assigning doors
//
// **A subject is a decision, so it lives where a decision can be
// asserted.** A writer that chose its subject at its own site would
// choose it inside an `app`-gated draw path no headless row executes,
// where the choice is unfalsifiable — a reviewer could change `Camera`
// to `Preferences` and the whole suite would stay green. That is the
// same argument `Badge` makes about the `None` decision, applied to
// the half of a `Message` that a `String` could not carry.
//
// So each door below answers the subject from the TYPED refusal it is
// handed, and the writer hands its refusal over rather than picking.
// Most are pinned twice over: the door takes one error type, so
// calling the wrong door does not compile. A door whose input is text
// rather than a typed refusal, such as `tool_news` or
// `startup_notices`, is not pinned, and says so at its own doc.

/// **What a pick against a missing index says** — the one seam
/// refusal that stays on the line, and the boundary the channel test
/// is visible at.
///
/// What it REPORTS is seam state, which reads like a badge. What it
/// IS, is an outcome: [`crate::pickcache::unindexed`] answers `Some` for a
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
/// the same event that ends the badge. Which is the point: the
/// subject agrees with the badge's and the CHANNEL still differs,
/// because the two questions are independent.
///
/// Ruled (Ev, 2026-09-06) against the worked example that named this a
/// badge. As a badge it would be lit for the whole index window
/// whether or not anyone clicked, it would say what the spinner's
/// hover text already says, and it would undo half of #1843 — which
/// asked for the indicator AND a pick path that distinguishes "not
/// indexed yet" from "nothing under the cursor".
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

/// **What a δ field holding something that is not a number says.**
///
/// [`SCENE_SEAM`], the same constant [`delta_refusal`] reaches through
/// [`SeamSubject`]: the δ field's two refusals are one seam's, and
/// this one is not type-pinned because the text never reached
/// [`crate::scene::DisplayTolerance`] — the parser's words are what
/// there is.
pub fn delta_not_a_number(typed: &str, error: &core::num::ParseFloatError) -> Message {
    Message::new(
        SCENE_SEAM,
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
/// already rendered, as whatever `app::ViewerApp::new` collected before
/// the first frame — at this writing the preferences file's
/// [`crate::prefs::Notice`]s (its own complaints AND the theme and
/// preset resolutions), a [`crate::prefs::PrefsError`] when the
/// document is not TOML at all, a [`crate::prefs::StoreError`] when the
/// store could not be read, and a sentence `ViewerApp::new` writes
/// itself when the launch directory cannot be read. The door takes
/// `&[String]`, so nothing here bounds that list and a new startup
/// complaint joins it without touching this function. What this door
/// buys is one place the decision is made rather than a type that
/// forbids the other answer.
///
/// # Several notices, not the items of one notice's list
///
/// Each of these becomes its own [`Message`] and the line between them
/// is [`NOTICE_SEPARATOR`], the same boundary [`frame_status`] writes.
/// Nothing counts them and no preamble introduces them: an unknown
/// key, an unresolved theme name and an unresolved preset name are
/// separate pieces of news that happen to share a subject, where a
/// [`Withdrawal`]'s causes are the items a single counted sentence
/// carries. Reading them as one notice's list was the category error,
/// and [`LIST_SEPARATOR`] between them was its rendering.
///
/// **So the guarantee is the one the outer level already holds**, and
/// it is needed here rather than merely available. A
/// [`crate::prefs::Notice`] arm may write a [`LIST_SEPARATOR`] inside
/// one sentence — `WrongType`, `UnknownTheme` and `UnknownPreset` do,
/// and so does the launch-directory sentence — so a flat join on that
/// mark made a two-notice line read as four items, reachable with no
/// error path at all from a file naming a theme and a preset the
/// registries no longer hold. No second mark could have been chosen
/// instead: `UnknownKey` and `WrongType` echo a key straight out of
/// the user's file, and a TOML quoted key may hold any character, so
/// nothing is out of band here. Nor can the guarantee rest on the list
/// of sources, which nothing bounds. What holds the line is
/// [`Message::new`] taking the boundary mark out of every text that
/// reaches it and [`Message::joined`] being the only thing that writes
/// one — a claim about the door rather than about anybody's sentences.
pub fn startup_notices(notices: &[String]) -> Option<Message> {
    let notices: Vec<Message> = notices
        .iter()
        .map(|text| Message::new(Subject::Preferences, text.as_str()))
        .collect();
    (!notices.is_empty()).then(|| Message::joined(Subject::Preferences, &notices))
}

/// **Where a file dialog opens**, from the candidates its signature
/// takes: the current document's own directory, the directory the last
/// dialog returned a path in, and the directory the viewer was launched
/// from — in that order, the first that `is_dir` confirms.
///
/// The order is by how recently a person pointed at the place. The
/// document's directory is where THIS work lives; the last dialog's is
/// where they went most recently, and it outlives the session through
/// the preferences (`crate::prefs::Prefs::last_dir`); the launch
/// directory is where they were when they started. `None` — reached
/// only when every candidate is absent or gone — leaves the dialog to
/// its backend's own default, whatever that is.
///
/// **A candidate that is not a directory falls through** rather than
/// refusing. A remembered directory deleted since is the ordinary way
/// a preferences file goes stale, and a dialog refused over it would
/// cost a person the save to protect a memory. `is_dir` is handed in
/// rather than read here so the rule is a function of its arguments —
/// `Path::is_dir` at the one live caller, a table in the rows that
/// exercise it.
///
/// A document's directory is its [`containing_dir`].
pub fn dialog_dir<'a>(
    document: Option<&'a Path>,
    last: Option<&'a Path>,
    launch: Option<&'a Path>,
    is_dir: impl Fn(&Path) -> bool,
) -> Option<&'a Path> {
    [document.and_then(containing_dir), last, launch]
        .into_iter()
        .flatten()
        .find(|dir| is_dir(dir))
}

/// **The directory a file lives in, as a place a dialog can open**:
/// its `parent`, or `None` when that parent is EMPTY. The parent of a
/// bare relative file name is `""`, which names no directory — as a
/// dialog candidate it would stop the search before the candidates
/// behind it, and as a remembered directory it would overwrite a real
/// one with nothing.
pub fn containing_dir(path: &Path) -> Option<&Path> {
    path.parent().filter(|dir| !dir.as_os_str().is_empty())
}

/// **What a cursor action the pick index refused says.**
///
/// [`Subject::Document`], not [`Subject::Cursor`]: the refusal is the
/// answer to an operation the user aimed at the document through the
/// cursor, and moving the pointer does not answer it. The cursor
/// subject is for a message ABOUT what lies under the pointer, which
/// is [`crate::idpass::Disagreement`]'s.
///
/// # The certified tie is re-rendered here, and only here
///
/// The kernel's own [`HitTestError::Ambiguous`] numbers its faces by
/// name — `StableName`'s `Display`, which omits the role path on
/// purpose, so two faces minted by one node render as the SAME phrase
/// and the ordinal is all that tells them apart. That is right for
/// the kernel, whose prose contract forbids a `Debug` derivation in a
/// message and whose typed payload carries the path anyway.
///
/// It is not enough on a status line. The reader has no payload to
/// open, and a sentence whose whole subject is that two answers
/// cannot be told apart cannot render them identically. So this door
/// writes the tie itself, rendering each face the way
/// [`crate::idpass::Disagreement`] renders a name — kind and minting
/// node, then the role path — for the same reason and with the same
/// shape. Every other arm is the typed refusal's own words,
/// unaltered.
pub fn pick_refusal(error: &PickError) -> Message {
    let PickError::HitTest(HitTestError::Ambiguous { hits }) = error else {
        return Message::new(Subject::Document, error.to_string());
    };
    let tied: Vec<String> = hits
        .iter()
        .map(|hit| format!("{} ({:?})", hit.name, hit.name.path))
        .collect();
    Message::new(
        Subject::Document,
        format!(
            "the ray is tied between {} faces the arithmetic cannot order — {} — so the pick \
             names none of them; aim away from the shared edge, or choose one of the tied faces",
            tied.len(),
            tied.join(", ")
        ),
    )
}

/// **What a tool has to say** — an authoring panel's refusal, a
/// survival drop, a pick a tool declined. [`Subject::Document`],
/// retired the way that subject says.
///
/// **A door a type does not pin**, like [`startup_notices`], because
/// its call sites hand it text — rendered through
/// [`crate::tools::ToolKind::says`], [`crate::tools::ToolNotice`] and
/// the typed forms vocabulary, or formatted at the site. What it buys
/// is that every site shares one decision: changing the subject here
/// changes it at all of them, and a row can see it.
///
/// The sites are every `frame::tool_news` call under
/// `crates/viewer/src`, and no number is given for them. They are a
/// grep over the panes and the app, not a population any type bounds;
/// a row could count them by that grep, as `frame_policy.rs` counts
/// `ViewerApp::store`'s reads, but it would go red at every new site,
/// and nothing about a new site is wrong.
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
///
/// **The δ is rendered, not formatted**
/// ([`crate::scene::DisplayTolerance::render_mm`]). The badge's whole
/// sentence is which δ the picture is at, and the δ it announces is the
/// budget's own choice — `constant / TRIANGLE_BUDGET`, a quotient with
/// no short spelling — so a fixed `{:.3}` read `δ 0.000 mm chosen` for
/// every body whose cost constant is under a triangle·millimetre. A
/// label wide enough for the render is the price, and a badge is a
/// label rather than a fixed-width field.
pub fn delta_badge(fitted: Option<&FittedDelta>) -> Option<Badge> {
    let fitted = fitted?;
    let wording = fitted.wording()?;
    Some(
        Badge::read(
            Subject::Display,
            format!("δ {} mm chosen", fitted.delta.render_mm()),
            Tone::Advisory,
        )
        .detailed(wording),
    )
}

/// **Where the chrome reports a gather refusal**, and whether it is a
/// refusal at all — the whole of what this crate decides about a
/// [`ProductError`], and the answer [`product_badge`] gates on.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
enum BadgeSite {
    /// This frame badges it: a gather-level fault no per-node badge
    /// can carry — a naming collision across roots, a graft the kernel
    /// refused, a validity verdict on the assembled product, an
    /// evaluation of the wrong document.
    Frame,
    /// The Features pane badges it AT the node, with the typed cause,
    /// so the frame stays silent.
    FeatureTree,
    /// No channel at all, because the class is one
    /// [`ProductErrorKind::means_no_body`] claims. What that means
    /// about the document is stated there and nowhere else.
    NotAFault,
}

/// Which channel reports a refusal of this class, if any.
///
/// A `match` rather than a predicate, and that is the point: it is
/// exhaustive over [`ProductErrorKind`], so an eleventh class reds
/// this crate — where a reader sees the consequence — instead of being
/// silently badged or silently declined by whichever way an expression
/// happened to be written.
///
/// **The local policy is the three the feature tree owns.**
/// [`crate::tree::RowStatus`] has exactly three non-`Ok` states —
/// `Failed`, `Poisoned`, `Unevaluated` — and
/// [`ProductError::RootFailed`], [`ProductError::RootPoisoned`] and
/// [`ProductError::UnknownNode`] are those same three states seen from
/// the gather. That count is a MEASUREMENT of another module's enum,
/// so it does not stand on this `match` being exhaustive:
/// `the_tree_still_has_exactly_the_three_states_this_policy_pairs_with`
/// is its guard, and a fourth non-`Ok` state reds there. The tree badges each AT the node and carries the typed
/// cause with it, so a frame badge would say strictly less, in a
/// louder colour, one row above a status line already reporting the
/// same root's tessellation refusal. The tree's own tone goes further:
/// [`crate::tree::RowStatus::tone`] makes a poisoned row
/// [`Tone::Advisory`], reserving [`Tone::Actionable`] for the row a
/// reader can act on, and the Features pane draws that value; a badge
/// shouting about the same poisoning would have the chrome saying both
/// things at once. That is a decision about THIS chrome and not a
/// classification of the refusal, which is why it is decided here.
///
/// **Whether what is left is a fault at all is not this crate's to
/// decide**, and it is not re-derived here:
/// [`ProductErrorKind::means_no_body`] is that reading's one home, and
/// the classes it claims reach [`BadgeSite::NotAFault`] through the
/// call rather than by being named again. The blank viewport is
/// already the picture of such a document.
///
/// They are asked in that order because they are independent, which
/// is what [`ProductErrorKind::means_no_body`]'s contract says a
/// `false` does and does not appoint: a class the tree already badges
/// is the tree's, whichever way the cited rule answers it.
///
/// **What the compiler buys here is exhaustiveness over the classes,
/// not liveness of the citation.** A new class cannot dodge this
/// `match`. Moving [`ProductErrorKind::NoBodyRoots`] into the first
/// arm would instead leave a call that can never answer `true` — a
/// dead citation, which nothing reds on and only
/// `the_gather_verdict_badges_only_the_faults_nothing_else_carries`
/// catches.
fn badge_site(kind: ProductErrorKind) -> BadgeSite {
    match kind {
        ProductErrorKind::RootFailed
        | ProductErrorKind::RootPoisoned
        | ProductErrorKind::UnknownNode => BadgeSite::FeatureTree,
        ProductErrorKind::EvaluationOfAnotherDocument
        | ProductErrorKind::PlacedUnderTwoRoots
        | ProductErrorKind::Naming
        | ProductErrorKind::NoBodyRoots
        | ProductErrorKind::Graft
        | ProductErrorKind::SolidInvalid
        | ProductErrorKind::ProductInvalid
        | ProductErrorKind::ContactLineage => {
            if kind.means_no_body() {
                BadgeSite::NotAFault
            } else {
                BadgeSite::Frame
            }
        }
    }
}

/// **What the chrome badges about the landed product**, and `None`
/// when there is nothing to say.
///
/// The gather's verdict is a READ: computed once when a pair lands,
/// held by the session, and consulted by a reader deciding what to do
/// about the product on screen. It is not the outcome of anything the
/// reader just did — the frame an Open lands on is exactly the frame
/// that also re-frames the camera, so the line is the one place a
/// fault raised by a landing cannot survive the landing.
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
/// palettes. What justifies the home is the channel test — this is a
/// read the reader consults, not an outcome they provoked — and what
/// justifies the colour is that it is the spelling its sibling badges
/// already use for a verdict a reader may need to act on.
///
/// # The arms that stay silent, and why
///
/// [`badge_site`] decides it, exhaustively over the error class: a
/// refusal another channel already carries, and a class that is no
/// fault at all, are both `None` here, and the argument for each is
/// there. What is left is what this channel is FOR — the
/// gather-level faults no per-node badge can carry.
pub fn product_badge(fault: Option<&ProductError>) -> Option<Badge> {
    fault
        .filter(|fault| badge_site(fault.kind()) == BadgeSite::Frame)
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
/// The refusal is held by [`crate::pickcache::PickCache`] under its
/// one-attempt-per (generation, δ) policy, so the badge stands for
/// exactly as long as the policy holds the refusal. The only other
/// thing it reads is the landed evaluation, and only to ask the tree
/// which row a refusal that follows from a failed node defers to (the
/// section below); the cache clears its refusal on the landing that
/// replaces that evaluation, so the two describe one run.
///
/// It says the SEAM refused. What a pick against the missing index
/// gets is [`unindexed_refusal`], on the line, because that is an
/// outcome — the two carry one subject and neither states it
/// ([`SeamSubject`]).
///
/// # A refusal that is a consequence, drawn under its cause
///
/// The index is built over every root, and a root whose row the
/// feature tree badges `Failed` or `Poisoned` has no value to index,
/// so the build refuses on it ([`downstream_root`]). That refusal is
/// DERIVED: the failure it follows from is already on screen, as the
/// one [`Tone::Actionable`] row the tree draws for it. So it takes the
/// tree's own reading of a downstream row — [`Tone::Advisory`], naming
/// the row that carries the cause ([`crate::tree::cause_row`], spelled
/// [`crate::tree::node_number`]) — and the index's own words move to
/// the tooltip, unaltered.
///
/// **It is placed under the cause, not dropped**, because it carries
/// two facts the cause does not. The refusal stops EVERY pick, on the
/// healthy roots' bodies too, and it stops the picture: the scene is
/// drawn from the index, so the viewport keeps its last picture until
/// the index builds. Both are in the label, and nowhere else: a pick
/// aimed at the missing index is refused on the line
/// ([`unindexed_refusal`]), whose sentence says only that the last
/// build refused or nothing has been evaluated yet — it gives no reason
/// and names no node, so without this badge the reader would not learn
/// why.
///
/// **The label names where the index stopped, not everything in its
/// way.** The build returns at the FIRST root that refuses, in
/// `doc.roots()` order, so a later root with a refusal of its own is
/// not reached; the label says the index waits on this row and does
/// not promise it builds once the row is fixed.
///
/// **The tooltip may name a different node from the label, on
/// purpose.** The tooltip is the index's own words, which name the
/// root the build refused on and, for a poisoned root, the kernel's
/// nearest failed ancestor. The label names the tree's row, which for
/// a root a mate refusal reached is the mate the fault blames rather
/// than the root. The label is the one that matches the row a reader
/// can act on, and the tooltip is kept unaltered because it is another
/// layer's refusal ([`PickIndexError`]'s `Display`).
///
/// Every other refusal is the index's own and stays
/// [`Tone::Actionable`] in its own words — and so does a standing
/// refusal the tree names no failed row for (a root that never ran,
/// or no evaluation to read), because quieting news is only right
/// where the louder news it defers to is actually drawn.
pub fn index_badge(
    error: Option<&PickIndexError>,
    evaluation: Option<&Evaluation<f64>>,
) -> Option<Badge> {
    let error = error?;
    let cause = downstream_root(error)
        .zip(evaluation)
        .and_then(|(root, evaluation)| crate::tree::cause_row(root, evaluation));
    Some(match cause {
        Some(cause) => Badge::read(
            PickIndexError::SUBJECT,
            format!(
                "pick index: waits on {}, which failed — until the index builds, no pick is \
                 answered and the picture is not redrawn",
                crate::tree::node_number(cause)
            ),
            Tone::Advisory,
        )
        .detailed(format!("pick index: {error}")),
        None => Badge::read(
            PickIndexError::SUBJECT,
            format!("pick index: {error}"),
            Tone::Actionable,
        ),
    })
}

/// **The root a pick-index refusal is a consequence of**, when the
/// refusal is the one a root with no value produces — `None` for a
/// refusal that is the index's own.
///
/// Only [`NodePickError::Standing`] is that: it is how the index says
/// the root has no `Ok` value in the evaluation. Whether that is
/// because the root failed, was poisoned, or never ran is the tree's
/// to read, and [`index_badge`] asks it rather than reading the
/// standing arm here. A tessellation or indexing refusal of a root
/// that DID evaluate is news no other surface carries.
///
/// Exhaustive over both enums, so a new way for the build to refuse
/// has to decide here whether it follows from a node's failure.
fn downstream_root(error: &PickIndexError) -> Option<RecipeNodeId> {
    match error {
        PickIndexError::Node { node, error } => match error {
            NodePickError::Standing(_) => Some(*node),
            NodePickError::NotABody { .. }
            | NodePickError::NoSuchBody { .. }
            | NodePickError::Tessellate(_)
            | NodePickError::Index(_) => None,
        },
        PickIndexError::Ids(_) | PickIndexError::DrawnTwice { .. } | PickIndexError::Names(_) => {
            None
        }
    }
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
/// **What the line could not do with it.** The status line is painted
/// in the toolbar, EARLIER in the same `update` than the pane that
/// writes this, and `perform_batch` runs after both — so the sentence
/// was never drawn on the frame it was written, and on a frame whose
/// batch acted cleanly `StatusUpdate::Clear` wiped it before any
/// frame could draw it. The chrome then said nothing about a picture
/// it could not draw, for as long as the user kept acting. A badge is
/// read where it is drawn, so no ordering decides whether it appears.
///
/// The clean fold's `Expire(Camera)` was NOT what silenced it: both
/// `land` calls run earlier in the same `viewport_ui` invocation,
/// which rewrote the refusal after the expiry.
pub fn projection_badge(error: Option<&CameraError>) -> Option<Badge> {
    error.map(|error| {
        Badge::read(
            CameraError::SUBJECT,
            format!("projection: {error}"),
            Tone::Actionable,
        )
    })
}

/// **What the chrome badges about datums this view draws nothing
/// of**, and `None` when every datum the document holds is on screen
/// — or when it holds none.
///
/// **The fact it exists to make sayable is a DIFFERENCE.** Every mark
/// `crate::datums` draws refuses on its own scale, correctly, and a
/// datum whose every mark refuses contributes no geometry: the
/// viewport then shows exactly what a document with no datums in it
/// shows. This is the read that tells the two apart, and it is the
/// whole of what it claims — `n` datums are in the document and none
/// of their marks reached the picture.
///
/// [`Subject::Camera`], and a badge rather than a sentence, for
/// [`projection_badge`]'s reasons in both halves. The subject: what
/// makes the count the wrong answer is the camera moving, which is
/// also what a reader does about it. No population of causes is named
/// here, because naming one is a claim and there is no sweep rule
/// that produces it: what empties a drawing is `crate::datums`'
/// business and it has at least three ways
/// (`datums::DatumDraws::vanished` enumerates them), each in its own
/// band of the datum's magnitude and the view's. The channel: the
/// count is true on every frame until the view or the document
/// changes, so it is a read of held state and not news.
///
/// **What it does not do is HOLD.** The three display seams above
/// keep a refusal until the seam succeeds; this is a per-frame count
/// its writer re-takes, zeroed by the frame entry point
/// (`<crate::app::ViewerApp as eframe::App>::ui`) before the panes
/// draw whether or not the viewport is one of them. So it needs no
/// sweeper, which is the defect
/// `work/view/projection-fault-has-no-sweeper.md` records against the
/// field beside it.
///
/// **It outlives the view it describes by exactly one frame**, and no
/// further. The toolbar draws BEFORE the panes, so on the frame the
/// viewport stops drawing this badge paints the count the previous
/// frame made, and the zero written after that frame's panes takes it
/// on the next. That is the same one-frame lag `crate::app`'s field
/// docs argue is benign, and it is a bounded lag rather than the
/// unbounded staleness a latch with no sweeper has.
///
/// [`Tone::Actionable`]: a reader can move the camera and get the
/// datums back, which is exactly the difference from an
/// [`Tone::Advisory`] report about something nobody can change.
pub fn datums_badge(vanished: usize) -> Option<Badge> {
    (vanished > 0).then(|| {
        // The noun agrees with the count: "1 datums" is the tell that
        // a sentence was assembled rather than written, and this one
        // is read at a glance beside the others.
        let noun = if vanished == 1 { "datum" } else { "datums" };
        Badge::read(
            Subject::Camera,
            format!("datums: {vanished} {noun} this view draws nothing of"),
            Tone::Actionable,
        )
    })
}

/// **What the chrome badges about committed profiles the viewport draws
/// nothing of**, and `None` when it drew every one.
///
/// A badge, per-frame and unlatched, for [`datums_badge`]'s reasons.
/// The cause is narrower than a datum's: a profile is drawn from its
/// validated value at the display tolerance, and what empties it is an
/// arc the flattener cannot put a point on (`crate::sketch::committed`)
/// — a fact about the document at this tolerance, so the subject is
/// the document and the tone [`Tone::Advisory`]: there is no camera
/// move that brings it back.
pub fn profiles_badge(undrawn: usize) -> Option<Badge> {
    (undrawn > 0).then(|| {
        let noun = if undrawn == 1 { "profile" } else { "profiles" };
        Badge::read(
            Subject::Document,
            format!("profiles: {undrawn} {noun} with an arc the viewport cannot draw"),
            Tone::Advisory,
        )
    })
}

/// **What the chrome badges about a store that keeps nothing**, and
/// `None` while preferences are kept.
///
/// **A badge, by the provenance rule**, and by the same argument Ev
/// ruled on for the absent file chooser: the store's usability is
/// settled when the store is built and true for the whole run, so the
/// sentence exists on a frame where nobody acted. That is a read of
/// held state a reader consults, and a whole-run environmental fact
/// has no correct sentence on a line that carries one frame's news.
/// What DOES belong on the line is [`store_refusal`] — a write that
/// was attempted and failed, which is an outcome and which a store
/// that keeps nothing never produces.
///
/// [`Subject::Preferences`], the settings and the file they are kept
/// in: the event that would make this the wrong answer is a write of
/// that file, and a store this badge is drawn for is one no write ever
/// reaches. It is [`Tone::Advisory`] and the tone is the whole of the
/// judgement here — the theme still applies on screen, nothing in the
/// session can give the store somewhere to write, and so there is
/// nothing for a reader to act on. It is a [`Affordance::Read`] label
/// for the same reason: there is no window of findings behind it.
///
/// The words are [`Unusable`]'s own, rendered unaltered; the
/// "preferences: " opening is this badge naming itself, as its
/// siblings do.
pub fn prefs_badge(unusable: Option<&Unusable>) -> Option<Badge> {
    unusable.map(|unusable| {
        Badge::read(
            Subject::Preferences,
            format!("preferences: {unusable}"),
            Tone::Advisory,
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
/// The rule is here instead, and it is a total function of what the
/// session owes and whether the index seam is busy.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Progress {
    /// A run is in flight: the picture is older than the document and
    /// someone is doing something about it.
    Evaluating,
    /// The picture is older than the document and **no EVALUATION is
    /// running** — what a cancel leaves behind. A spinner over that
    /// alone would be a lie about work nobody is doing.
    ///
    /// `indexing` is whether a seam BELOW the evaluation is
    /// nonetheless busy — an index build, or the display fit the index
    /// waits on — and it is carried here rather than answered by a
    /// second indicator because this is the one state where the seams
    /// disagree about whether anything is happening: a build submitted
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
    /// until this lands ([`crate::pickcache::unindexed`]).
    Indexing,
}

/// The one state, from what the session owes and what the pick cache
/// is doing.
///
/// **Evaluation outranks indexing**, because an index built for a
/// generation the session has already moved past is about to be
/// discarded by [`crate::pickcache::PickCache::land`] anyway — restart
/// without cancel means both can be in flight at once, and naming the
/// index build there would tell a reader the wait was nearly over when
/// a whole evaluation is still ahead of it.
pub fn progress(outstanding: Outstanding, indexing: bool) -> Option<Progress> {
    match outstanding {
        Outstanding::Evaluating => Some(Progress::Evaluating),
        Outstanding::Canceled => Some(Progress::Canceled { indexing }),
        Outstanding::Current if indexing => Some(Progress::Indexing),
        Outstanding::Current => None,
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

/// **Whether a folded event stream is a camera event at all**: it
/// applied a camera operation, refused one, or both — a fold stops at
/// its first refusal and keeps what it applied before it
/// ([`Folded::applied`] is a prefix of the input). So `true` does not
/// say whether the camera moved: a fold whose FIRST operation refused
/// moved nothing, and one that refused later moved as far as it got.
/// The name is narrower than the value.
///
/// The stream carries cursor events too, and a stream that denotes no
/// camera operation is not a camera event.
///
/// **What this buys is a statement, not a fix.** It once stopped an
/// erasure: landing a no-op fold cleared the status line on every frame
/// the pointer was inside the viewport. [`fold_status`] closed that at
/// the other end, so the guard is now near-redundant behaviourally —
/// it saves one call and a `Camera` copy. It is kept because
/// [`crate::pane::viewport::land`] is where a camera event becomes
/// application state — the camera the fold reached, and the refusal
/// that stopped it — and calling it on frames with no camera event
/// hands any writer later added to it per-frame behaviour nobody asked
/// for.
pub fn folded_moved(folded: &Folded) -> bool {
    !folded.applied.is_empty() || folded.refused.is_some()
}

/// **The two channels, as policy over values.**
///
/// Both rules this module states about the chrome are silent when they
/// break: a cleared line looks exactly like a line nobody wrote to,
/// and a fault with no home looks exactly like a document with no
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
    use pncad::prelude::{EntityKind, StableName};

    use crate::camera::{Camera, CameraOp, CameraOpError};
    use crate::display::AdmissionFault;
    use crate::tree::RowStatus;

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
             is no camera event never reaches the line at all, and this \
             row would be asserting about a case that cannot happen"
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

    /// **The first `apply` below is not a live composition.** No
    /// production caller hands a [`fold_status`] `Show` to [`apply`]: a
    /// refused fold reaches the line through [`deliver`], onto the
    /// frame's notices, and the ranking puts it up. It is here as the
    /// nearest way to put a camera refusal on the line, so the row can
    /// ask what the next clean fold's `Expire` does to it — which is
    /// `apply`'s contract and holds whoever placed the refusal.
    /// `pane::viewport`'s
    /// `landing_a_clean_fold_retires_the_camera_refusal_it_landed_before`
    /// is the row on the live path, through `land` and the ranking; it
    /// is not a duplicate of this one, and this one does not cover that
    /// path.
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

    /// **`deliver` splits a verdict by whether it has anything to
    /// SAY**: news joins the frame's notices and meets the ranking, a
    /// retirement reaches the field directly because a notice cannot
    /// un-say anything.
    ///
    /// The two halves are asserted against each other rather than
    /// separately: the same call that must not touch the field must
    /// also have pushed, and the same call that must not push must
    /// have touched the field. Either assertion alone passes for a
    /// `deliver` that does nothing at all.
    ///
    /// **The `Show` block starts from a non-empty `notices`** so that
    /// APPEND is what is asserted and not merely arrival. A vector
    /// seeded with nothing cannot tell an append from a replacement or
    /// an insert at the front, and [`frame_status`]'s rank 2 joins its
    /// notices *"in the order they happened"* — so the order is a
    /// contract and not an accident of how a `Vec` happens to grow.
    #[test]
    fn deliver_sends_news_to_the_notices_and_retirements_to_the_field() {
        let held = Message::new(Subject::Camera, "camera: refused a moment ago");
        let earlier = Message::new(Subject::Document, "extrude: refused earlier this frame");

        // News. The field is left alone — the ranking has not run yet,
        // and writing it here is the defect: this frame's accepted
        // batch would clear it before the toolbar painted it.
        let mut notices = vec![earlier.clone()];
        let mut status = Some(held.clone());
        let news = Message::new(Subject::Camera, "camera: dolly refused");
        deliver(&mut notices, &mut status, StatusUpdate::Show(news.clone()));
        assert_eq!(
            notices,
            vec![earlier, news],
            "a Show is news and joins the frame, AFTER what the frame \
             already had to say"
        );
        assert_eq!(status, Some(held.clone()), "and does not write the field");

        // A retirement. Nothing to say, so nothing to rank — and it
        // must reach the field, which is the one thing a notice cannot
        // do.
        let mut notices = Vec::new();
        let mut status = Some(held.clone());
        deliver(
            &mut notices,
            &mut status,
            StatusUpdate::Expire(Subject::Camera),
        );
        assert!(notices.is_empty(), "an Expire adds nothing to the frame");
        assert_eq!(status, None, "and retires what it was about");

        // `Clear` is the fourth arm and the subject-blind one: not a
        // retirement, but on the retiring side of this door for the
        // same reason — it takes something away, and the thing it
        // takes away is the whole line whatever the line was about.
        let mut notices = Vec::new();
        let mut status = Some(Message::new(Subject::Document, "someone else's news"));
        deliver(&mut notices, &mut status, StatusUpdate::Clear);
        assert!(notices.is_empty(), "a Clear adds nothing to the frame");
        assert_eq!(status, None, "and sweeps the line whatever it held");

        // `Keep` is the absence of news spelled as a decision: neither
        // route is taken, and a `deliver` call that answers `Keep` is a
        // no-op on BOTH sides. `notices` starts non-empty so the
        // assertion can fail for a `Keep` that sweeps the frame's news
        // as well as for one that adds to it — an empty vector cannot
        // tell "did not push" from "cleared what was there".
        let mut notices = vec![Message::new(
            Subject::Document,
            "news from earlier this frame",
        )];
        let mut status = Some(held.clone());
        let before = notices.clone();
        deliver(&mut notices, &mut status, StatusUpdate::Keep);
        assert_eq!(
            notices, before,
            "a Keep neither adds to the frame nor sweeps it"
        );
        assert_eq!(status, Some(held), "and leaves the field exactly as it was");
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

    /// **A refused fold is a `Show` about the camera, in the refusal's
    /// own words — and [`apply`]'s `Show` arm replaces the line.**
    ///
    /// Two claims, and the second is about `apply` and NOT about the
    /// camera. No production caller composes them any more: a refused
    /// fold reaches the line through [`deliver`], which sends it to the
    /// frame's notices, and `pane::viewport`'s
    /// `landing_a_refused_fold_is_news_and_joins_the_frames_notices`
    /// is the row on that live path. What survives here is `apply`'s
    /// contract, which the sweep did not change and which
    /// `app::ViewerApp::apply_status`'s ranked traffic still depends
    /// on: a `Show` handed to `apply` overwrites whatever was held,
    /// whoever hands it over.
    #[test]
    fn a_refused_fold_is_news_about_the_camera_and_apply_overwrites_with_it() {
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

        // `apply`'s contract, asserted through the nearest producer to
        // hand rather than a live composition — see the doc above.
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

        // The silent arms, each paired with the silence it gets:
        // `badge_site` is where the argument for both lives. (Plain
        // backticks: a bracketed link in a `//` comment is checked by
        // nothing, so it must not wear the spelling rustdoc gates.)
        for (quiet, site) in [
            (ProductError::NoBodyRoots, BadgeSite::NotAFault),
            (ProductError::RootFailed { node }, BadgeSite::FeatureTree),
            (
                ProductError::RootPoisoned {
                    node,
                    through: RecipeNodeId(1),
                },
                BadgeSite::FeatureTree,
            ),
            (ProductError::UnknownNode { node }, BadgeSite::FeatureTree),
        ] {
            assert_eq!(
                badge_site(quiet.kind()),
                site,
                "which channel reports it: {quiet}"
            );
            assert_eq!(
                product_badge(Some(&quiet)),
                None,
                "another channel already carries this: {quiet}"
            );
        }
        assert_eq!(product_badge(None), None);

        // And the classes this channel is FOR, by name rather than by
        // the one sample above — the half of the policy a badge that
        // went silent would not fail.
        for kind in [
            ProductErrorKind::EvaluationOfAnotherDocument,
            ProductErrorKind::PlacedUnderTwoRoots,
            ProductErrorKind::Naming,
            ProductErrorKind::Graft,
            ProductErrorKind::SolidInvalid,
            ProductErrorKind::ProductInvalid,
            ProductErrorKind::ContactLineage,
        ] {
            assert_eq!(
                badge_site(kind),
                BadgeSite::Frame,
                "no per-node badge carries it: {kind:?}"
            );
        }
    }

    /// **The guard for the count [`badge_site`]'s doc states about
    /// another module's enum.**
    ///
    /// That policy leaves a class to the Features pane because the
    /// pane has a row status to carry it, one for one. A fourth
    /// non-`Ok` [`RowStatus`] would be a state nothing here pairs
    /// with, and the count in the prose would be silently wrong — so
    /// the `match` below is exhaustive over `RowStatus` and reds on a
    /// new variant, at the claim rather than a schedule away from it.
    #[test]
    fn the_tree_still_has_exactly_the_three_states_this_policy_pairs_with() {
        let non_ok = |status: &RowStatus| match status {
            RowStatus::Ok => 0_usize,
            RowStatus::Failed { .. } | RowStatus::Poisoned { .. } | RowStatus::Unevaluated => 1,
        };
        let states: usize = [
            RowStatus::Ok,
            RowStatus::Failed {
                message: String::new(),
            },
            RowStatus::Poisoned {
                through: RecipeNodeId(1),
                message: None,
            },
            RowStatus::Unevaluated,
        ]
        .iter()
        .map(non_ok)
        .sum();
        // Every class, inline in the row the way this crate's suites
        // hold a complete variant list (`crates/viewer/README.md`).
        // It is hand-written and can be: a class cannot be added
        // without [`badge_site`]'s `match` refusing to compile, so
        // whoever adds one is already standing at the site that sends
        // them here, and no schedule fires sooner than that.
        let left_to_the_tree = [
            ProductErrorKind::EvaluationOfAnotherDocument,
            ProductErrorKind::UnknownNode,
            ProductErrorKind::PlacedUnderTwoRoots,
            ProductErrorKind::Naming,
            ProductErrorKind::RootFailed,
            ProductErrorKind::RootPoisoned,
            ProductErrorKind::NoBodyRoots,
            ProductErrorKind::Graft,
            ProductErrorKind::SolidInvalid,
            ProductErrorKind::ProductInvalid,
            ProductErrorKind::ContactLineage,
        ]
        .into_iter()
        .filter(|kind| badge_site(*kind) == BadgeSite::FeatureTree)
        .count();
        assert_eq!(
            left_to_the_tree, states,
            "every class this policy leaves to the Features pane is left to a row the pane draws"
        );
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
            cause: AdmissionFault::MateConstrained {
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
             subject is an instance. That is `AdmissionFault`'s per-arm rule \
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
        // is written here — both come from `AdmissionFault`'s `Display`.
        let cause = AdmissionFault::MateConstrained {
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
            cause: AdmissionFault::NoSuchNode {
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
            cause: AdmissionFault::FusedGeometry {
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
            cause: AdmissionFault::FusedGeometry {
                instance: RecipeNodeId(instance),
                root: RecipeNodeId(8),
                others: vec![RecipeNodeId(other)],
            },
        };
        let gone = Withdrawn {
            instance: RecipeNodeId(4),
            cause: AdmissionFault::NoSuchNode {
                node: RecipeNodeId(4),
            },
        };

        let two = [fused(3, 5), fused(5, 3)];
        assert_eq!(
            dropped_hide_text(&two).expect("two dropped hides are news"),
            format!(
                "hide: 2 hides were dropped and the hidden geometry is drawn \
                 again — {}{LIST_SEPARATOR}{}",
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
                "free move: 2 committed placements were discarded — {}{LIST_SEPARATOR}{}",
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

    #[test]
    fn a_killed_gesture_is_its_own_sentence_and_has_no_plural() {
        let killed = constrained(3, &[5]);
        let notice = Withdrawal::killed_gesture(Some(&killed))
            .expect("a drag the document ended is news")
            .to_string();
        assert_eq!(
            notice,
            format!("free move: the drag in flight was ended — {}", killed.cause),
            "the drag the user's hand was on, and the fault that ended it \
             rendered by its own `Display` — the same shape the other two \
             kinds use, and a sentence neither of them can say"
        );
        assert_ne!(
            notice,
            superseded_text(core::slice::from_ref(&killed)).expect("news"),
            "NOT the supersession's sentence: nothing substituted for a \
             placement the document was never asked for, so the two must \
             not read alike when both are on one line"
        );

        // The plural this kind has no way to reach is also the plural it
        // is given no words for: one gesture is in flight at a time, so
        // `Option` is the whole domain and a set-shaped door would be a
        // count no caller could produce.
        assert_eq!(Withdrawal::killed_gesture(None), None);
        assert_eq!(
            Withdrawal::killed_gesture(Some(&killed))
                .expect("news")
                .notice()
                .subject(),
            Subject::Document,
            "a killed gesture is about the document that ended it, so the \
             next act the document accepts is what retires it — the same \
             retirement as its two siblings, which is what lets one line \
             carry all three"
        );
    }

    /// **The fan-out is the report's list, in both directions.**
    ///
    /// One direction is the compiler's and is not asserted here: a
    /// fourth field on `PruneReport` is E0027 inside `Withdrawal::all`,
    /// which is the call that words it.
    ///
    /// This row holds the OTHER direction, which no compile error
    /// reaches — a `WithdrawalKind` that `all` never produces. The
    /// chrome's only door to a withdrawal is `all`, so a kind absent
    /// from it is a sentence the crate can spell and nothing can ever
    /// say. Asserting against `WithdrawalKind::ALL` rather than against
    /// a written list of three is what makes a FOURTH variant red here
    /// instead of quietly agreeing: `ALL` is projected from the enum's
    /// own declaration.
    ///
    /// Where it goes red: drop any arm from `all` (the kind's entry
    /// disappears), reorder them (the order is the report's field
    /// order and the chrome shows them in it), or add a variant to
    /// `WithdrawalKind` with no report field behind it.
    #[test]
    fn every_withdrawal_kind_has_a_producer() {
        let report = PruneReport {
            superseded: vec![constrained(7, &[9])],
            dropped_hides: vec![constrained(11, &[9])],
            killed_gesture: Some(constrained(3, &[5])),
        };
        let produced: Vec<WithdrawalKind> = Withdrawal::all(&report)
            .map(|withdrawal| withdrawal.kind)
            .collect();
        assert_eq!(
            produced,
            WithdrawalKind::ALL,
            "a report carrying every kind fans out to every kind, in the \
             order the report declares them"
        );

        // And each kind words itself apart: the notices are what the
        // user reads off one line, so two kinds rendering alike would
        // make the fan-out complete and useless.
        let mut sentences: Vec<String> = Withdrawal::all(&report)
            .map(|withdrawal| withdrawal.notice().text().to_string())
            .collect();
        assert_eq!(sentences.len(), WithdrawalKind::ALL.len());
        sentences.sort();
        sentences.dedup();
        assert_eq!(
            sentences.len(),
            WithdrawalKind::ALL.len(),
            "no two kinds say the same thing"
        );

        // An empty report is silence, not three empty sentences —
        // `Withdrawal::of`'s decision, executed through the one door
        // rather than asserted of each constructor.
        assert_eq!(Withdrawal::all(&PruneReport::default()).count(), 0);
    }
}
