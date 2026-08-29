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

use pncad::document::{ParamName, ParseError, RecipeNodeId, SlotId};
use pncad::prelude::StableName;

use crate::camera::Folded;
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
/// camera operation is not a camera event: landing it anyway would
/// clear the status line on every frame the pointer is inside the
/// viewport, which is the same defect [`acts`] guards at the other end.
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
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let show = |name: &Option<StableName>| match name {
            Some(name) => format!("{:?}", name.path),
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
