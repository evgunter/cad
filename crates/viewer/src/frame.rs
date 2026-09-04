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
//! # The status line's two lifetimes
//!
//! **The line carries NEWS, for the frame that produced it.** Every
//! sentence on it is something that HAPPENED — an action the document
//! refused, a pick a tool declined, a dialog that could not open — and
//! which of a frame's news SHOULD win is [`frame_status`]'s ranking.
//!
//! **It does not yet win for every writer, and this module is one of
//! the exceptions.** Nineteen writers in this crate reach the line
//! without asking the ranking; eighteen assign the field outright, and
//! the nineteenth is [`fold_status`], which answers in the right
//! vocabulary and applies it at `pane::viewport::land`. So a camera
//! refusal raised in a frame that also carries a clean acting op is
//! overwritten by that batch's [`StatusUpdate::Clear`], which runs
//! after the panes have drawn. The rule below is what the line is FOR;
//! routing the writers through it is tracked as its own item, not
//! asserted here as done.
//!
//! **A fact that is still true after the frame ends is not news.** It
//! is a standing fact about the landed document or the picture drawn
//! from it, it has to survive a mouse drag, and its home is a toolbar
//! badge: the at-rest verdict, the advisory checks, the δ the display
//! budget chose, and [`product_badge`].
//!
//! Two rules follow, and both are values here rather than conditions
//! at a call site. [`fold_status`] answers [`StatusUpdate::Keep`] for
//! a clean camera fold: a camera arriving where it was sent is not
//! news, and clearing on its behalf would decide the fate of messages
//! written by everyone else in the same frame. Clearing is the acting
//! batch's verdict alone ([`batch_status`]), because an action the
//! document accepted is the one event that makes a standing complaint
//! stale. And the gather's verdict badges rather than writes, because
//! a fault about the document on screen outlives every frame the
//! camera moves in.

use pncad::document::{ParamName, ParseError, ProductError, RecipeNodeId, SlotId};
use pncad::prelude::StableName;

use crate::camera::Folded;
use crate::display::Withdrawn;
use crate::evalseam::Generation;
use crate::pick::{IdMap, PickIndex};
use crate::session::{Refusal, SessionOp};

/// What a frame's batch of operations should do to the status line.
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum StatusUpdate {
    /// Leave the line as it is.
    Keep,
    /// Clear it: the user acted and nothing refused.
    Clear,
    /// Show this refusal.
    Show(String),
}

/// **Apply a verdict to the status line**: the one place a
/// [`StatusUpdate`] becomes the field it describes.
///
/// Every policy in this module answers in this vocabulary and every
/// consumer applies it here, so [`StatusUpdate::Keep`] is spelled as a
/// decision rather than as the absence of one. A writer that assigns
/// the `Option<String>` itself has no way to say "I have nothing to
/// add", and the natural-looking spelling of it — assigning what it
/// would have shown — writes `None` over whatever another writer in
/// the same frame put there.
pub fn apply(status: &mut Option<String>, update: StatusUpdate) {
    match update {
        StatusUpdate::Keep => {}
        StatusUpdate::Clear => *status = None,
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
        (_, Some(refusal)) => StatusUpdate::Show(refusal.to_string()),
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
    notices: &[String],
    ops: &[SessionOp],
    refusal: Option<&Refusal>,
) -> StatusUpdate {
    match batch_status(ops, refusal) {
        refused @ StatusUpdate::Show(_) => refused,
        verdict if notices.is_empty() => verdict,
        _ => StatusUpdate::Show(notices.join(NOTICE_SEPARATOR)),
    }
}

/// What several notices in one frame are joined with — one spelling,
/// shared with the preferences path's startup notices so the status
/// line reads the same however many things it is carrying.
pub const NOTICE_SEPARATOR: &str = "; ";

/// **What a frame's SUPERSESSIONS say**, as a notice for
/// [`frame_status`]'s rank 2 — and `None` when the frame superseded
/// nothing.
///
/// [`crate::session::OpOutcome::superseded`] names the instances whose
/// COMMITTED free-move placement an operation's document transition
/// discarded — the G3 supersession, reported by the session rather
/// than inferred (`display::DisplayState::prune` is where it happens,
/// and `display::free_move_check` is the condition). A killed
/// in-flight gesture is NOT in that list, so it is not this channel's
/// to report; the next gesture op refuses typed instead.
///
/// # Why the line and not a badge
///
/// It is NEWS by this module's test. It HAPPENED on the frame that
/// carries it, provoked by the act the user just took — the mate that
/// landed on their probed instance, the delete that took it, the redo
/// that stepped forward over the mate again — and after that frame it
/// is true of nothing. The standing fact it leaves behind is the
/// instance drawn at its landed placement, which the picture already
/// says; a badge would keep saying it about a document the user has
/// moved on from.
///
/// That lifetime is the ARGUMENT for the line and not yet a mechanism:
/// nothing removes a notice when its frame ends, so this sentence
/// survives on the line until an acting batch clears it, exactly like
/// every other message. Tracked as
/// `work/view/the-news-vocabulary-has-no-expiry.md`, which this is now
/// a named instance of.
///
/// It reaches the line through the frame's NOTICES rather than by
/// assignment, for the reason [`frame_status`] states: the transition
/// that supersedes is an edit the document accepted, so the same
/// frame's batch verdict is [`StatusUpdate::Clear`].
///
/// A refusal in the same frame outranks it and it is then not shown,
/// which rank 1 already says. The two cannot come from one operation:
/// a refused op returns before the prune that fills this list.
///
/// # The cause is the fault's own sentence
///
/// **Nothing here composes prose about why a placement went.** Each
/// entry carries the [`crate::display::DisplayFault`] the prune
/// discarded on, and this function renders it through its own
/// `Display` — the rule the rest of the crate follows. So the
/// commonest arm names the mates and the remedy
/// (`MateConstrained`: *delete the mate(s) if free relative motion is
/// intended*), a fuse names the product and the instances fused into
/// it, and a deleted instance says the document does not hold the node
/// — which is the delete arm's whole point, since the id alone names
/// something the tree no longer draws without saying that is why.
///
/// The frame around the faults counts and does not name: every fault
/// reachable here names its own instance, so naming it again in the
/// preamble would say it twice.
pub fn supersession_notice(superseded: &[Withdrawn]) -> Option<String> {
    let causes = render_causes(superseded)?;
    Some(if superseded.len() == 1 {
        format!("free move: a committed placement was discarded — {causes}")
    } else {
        format!(
            "free move: {} committed placements were discarded — {causes}",
            superseded.len()
        )
    })
}

/// **What a frame's DROPPED HIDES say**, as a notice for
/// [`frame_status`]'s rank 2 — and `None` when the frame dropped none.
///
/// # Why this is not the supersession sentence
///
/// A supersession is a SUBSTITUTION: the user's hand placement
/// answered "where does this part go", and the mate that landed
/// answers it better, so the probe steps aside and the picture keeps
/// the part. A dropped hide is not superseded by anything. The user
/// asked for an instance not to be DRAWN, and the document did not
/// answer that question differently — it made the question unaskable.
/// Nothing takes the hide's place.
///
/// The two cases the fault distinguishes are why one sentence could
/// not carry both: on a fuse the instance is drawn AGAIN — material
/// the user took out of the picture is back in it, which is exactly
/// the state a user reports as a bug against hiding — and on a delete
/// the instance is gone and nothing reappears. The preamble therefore
/// says only what is true of both, that the hide was dropped, and the
/// fault says which happened.
///
/// It is news on the same terms as [`supersession_notice`], reaches
/// the line the same way, and the two are ranked together: both are
/// display state an accepted edit withdrew, and a frame can produce
/// both at once.
pub fn dropped_hide_notice(dropped: &[Withdrawn]) -> Option<String> {
    let causes = render_causes(dropped)?;
    Some(if dropped.len() == 1 {
        format!("hide: a hidden instance stopped being hideable — {causes}")
    } else {
        format!(
            "hide: {} hidden instances stopped being hideable — {causes}",
            dropped.len()
        )
    })
}

/// Every withdrawal's cause, each rendered by its own `Display`, in the
/// order the prune found them — or `None` for an empty set, which both
/// notices answer with silence.
///
/// Joined with a separator rather than composed into a sentence, for
/// the reason [`frame_status`] joins notices that way: a list of
/// several typed values must not become one written claim about them.
fn render_causes(withdrawn: &[Withdrawn]) -> Option<String> {
    if withdrawn.is_empty() {
        return None;
    }
    Some(
        withdrawn
            .iter()
            .map(|w| w.cause.to_string())
            .collect::<Vec<_>>()
            .join("; "),
    )
}

/// **The status line after a camera fold.**
///
/// A refusal is news: the user asked the camera for something it
/// would not do, and no other channel says so. A CLEAN fold is not
/// news at all — the camera arriving where it was sent is the
/// unremarkable case, and it is the case on every frame of a drag and
/// on the re-frame an opened document books for itself.
///
/// So the clean arm is [`StatusUpdate::Keep`], never
/// [`StatusUpdate::Clear`]. Clearing belongs to [`batch_status`],
/// where an action the document ACCEPTED is what makes the last
/// complaint stale; a camera move is not one, and a fold that cleared
/// would be deciding the fate of sentences written by writers it knows
/// nothing about — on the frame a document lands, the ones that
/// landing itself produced.
///
/// The refusal renders the operation alongside the error because a
/// camera refusal is about a MOVE: the error alone names the condition
/// without the thing that provoked it.
pub fn fold_status(folded: &Folded) -> StatusUpdate {
    match &folded.refused {
        Some((op, error)) => StatusUpdate::Show(format!("camera: {error} (from {op})")),
        None => StatusUpdate::Keep,
    }
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
/// **Redundant colour beside its own words.** The chrome draws this in
/// [`crate::theme::Theme::unresolved`], the colour the at-rest refusal
/// and the checks findings already carry, and that colour's stated
/// contract is that it is REDUNDANT — every badge using it says its own
/// words, so nothing depends on the colour being read. This badge
/// satisfies it, because [`ProductError`]'s `Display` opens every arm
/// with "product: ".
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
/// draws a poisoned row deliberately QUIET, reserving the unresolved
/// colour for the row a reader can act on; a badge shouting about the
/// same poisoning would have the chrome saying both things at once.
///
/// What is left is what this channel is FOR: the gather-level faults no
/// per-node badge can carry — a naming collision across roots, a graft
/// the kernel refused, a validity verdict on the assembled product, an
/// evaluation of the wrong document.
pub fn product_badge(fault: Option<&ProductError>) -> Option<String> {
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
        .map(ToString::to_string)
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
        (false, false) => StatusUpdate::Show(NO_CHOOSER_BACKEND.to_owned()),
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
        assert_eq!(fold_status(&folded), StatusUpdate::Keep);

        let mut status = Some("someone else's news".to_owned());
        apply(&mut status, fold_status(&folded));
        assert_eq!(
            status.as_deref(),
            Some("someone else's news"),
            "a clean fold is not news and clears nothing"
        );
    }

    #[test]
    fn a_refused_fold_is_news_and_outranks_what_the_line_held() {
        let refused = CameraOp::Dolly { factor: 0.0 };
        let folded = Folded {
            camera: a_camera(),
            applied: Vec::new(),
            refused: Some((refused, CameraOpError::NonPositiveDolly { factor: 0.0 })),
        };
        assert!(folded_moved(&folded), "a refusal is a camera event too");
        let StatusUpdate::Show(message) = fold_status(&folded) else {
            panic!("a refused fold is news: {:?}", fold_status(&folded));
        };
        assert!(
            message.contains("camera:") && message.contains("dolly by a factor"),
            "the refusal names the move that provoked it: {message}"
        );

        let mut status = Some("older news".to_owned());
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
        assert_eq!(badge, collision.to_string(), "the fault renders itself");
        assert!(
            badge.starts_with("product: "),
            "and says what it is about, so the colour carries nothing \
             alone: {badge}"
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
    fn keep_clear_and_show_are_three_different_sentences() {
        // `Keep` is a decision, not the absence of one — the whole
        // reason every policy here answers in this vocabulary instead
        // of assigning the field.
        let mut status = Some("held".to_owned());
        apply(&mut status, StatusUpdate::Keep);
        assert_eq!(status.as_deref(), Some("held"));
        apply(&mut status, StatusUpdate::Show("news".to_owned()));
        assert_eq!(status.as_deref(), Some("news"));
        apply(&mut status, StatusUpdate::Clear);
        assert_eq!(status, None);
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
        let notice =
            supersession_notice(&[constrained(7, &[9])]).expect("a supersession is news");
        assert!(
            notice.contains("instance 7"),
            "the notice names which of the user's placements went, in the \
             vocabulary the properties panel and `DisplayFault` use for a \
             part instance: {notice}"
        );

        let acting = [SessionOp::Undo];
        assert_eq!(
            batch_status(&acting, None),
            StatusUpdate::Clear,
            "the frame this row is about CLEARS the line on its own — without \
             that, the composition below would be asserting about a case \
             where nothing had to survive anything"
        );
        let update = frame_status(core::slice::from_ref(&notice), &acting, None);
        assert_eq!(update, StatusUpdate::Show(notice.clone()));

        let mut status = None;
        apply(&mut status, update);
        assert_eq!(status, Some(notice));
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
        let notice = supersession_notice(&[constrained(3, &[5])]).expect("news");
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
        let gone = supersession_notice(&[Withdrawn {
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
        let notice = dropped_hide_notice(core::slice::from_ref(&fused)).expect("news");
        assert!(
            notice.starts_with("hide: a hidden instance stopped being hideable — "),
            "its own preamble, not the free-move one: {notice}"
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
            supersession_notice(&[constrained(7, &[9])]).expect("news"),
            notice.clone(),
        ];
        let StatusUpdate::Show(shown) = frame_status(&notices, &[SessionOp::Undo], None) else {
            panic!("two withdrawals are news");
        };
        assert!(shown.contains("free move:") && shown.contains("hide:"));

        assert_eq!(dropped_hide_notice(&[]), None);
    }

    #[test]
    fn every_superseded_instance_is_named_and_none_means_silence() {
        // Not the first and not the last: one transition can discard
        // several probes (a mate lands on two probed instances, a
        // delete takes a subtree), and each is an instance the user
        // placed by hand and no longer has.
        let one = supersession_notice(&[constrained(3, &[5])]).expect("one supersession is news");
        assert_eq!(
            one,
            "free move: a committed placement was discarded — \
             instance 3 is mate-constrained (mate node(s) 5): its pose is \
             mate-derived, so the free-move probe refuses — delete the mate(s) if \
             free relative motion is intended"
        );

        let both = supersession_notice(&[constrained(3, &[5]), constrained(11, &[5])])
            .expect("two supersessions are still news");
        for named in ["instance 3", "instance 11"] {
            assert!(both.contains(named), "both instances named: {both}");
        }
        assert!(
            both.starts_with("free move: 2 committed placements were discarded — "),
            "every word of the preamble agreeing with itself in number — \
             subject, verb and object: {both}"
        );
        assert_eq!(
            both.matches("; ").count(),
            1,
            "two faults, joined by a separator rather than composed into one \
             written claim about them: {both}"
        );

        // Silence has exactly one meaning here: nothing was discarded.
        assert_eq!(supersession_notice(&[]), None);
    }
}
