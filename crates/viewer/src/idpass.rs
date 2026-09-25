//! The GPU id pass's bookkeeping: what is outstanding, what it was
//! asked about, and what its answer is worth when it comes back.
//!
//! # Why this is not a per-frame policy
//!
//! The id pass is a round trip. One frame issues a query; a later
//! frame reads the answer — and in between the cursor can move, the
//! picture can be rebuilt and the index can be replaced. Nothing
//! visible on the reading frame says whether the answer still
//! describes the question, so the only thing that can say it is state
//! carried ACROSS frames: the serial outstanding, and the cursor and
//! [`IdSubject`] it was asked about.
//!
//! That is the whole difference from [`crate::frame`], whose policies
//! are pure functions of one frame and are values precisely so they
//! can be replayed. Everything here exists because it REMEMBERS.
//! [`IdQueryLog`] holds the question, [`IdSubject`] decides what
//! counts as the same question, [`IdStep`] is the verdict about this
//! frame, and [`Disagreement`] is what the two picking paths amount to
//! once an answer has been matched to the question it answers.
//!
//! **The failure mode the memory exists for is an answer outliving its
//! question**, and it does not look like a fault: a matched-but-stale
//! answer compared against a fresh ray hit reports *the two picking
//! paths disagree*, which is the sentence issue #1097 §4 tells an
//! operator to read as an `R32Uint` clear fault. Each type below says
//! which shape of outliving it closes.
//!
//! Module kind: **vocabulary** — it names no driver type and no
//! `app`-only crate (`crates/viewer/README.md`, Module boundaries).

use pncad::prelude::StableName;

use crate::frame::{Message, Subject};
use crate::generation::Generation;
use crate::pickindex::{IdMap, PickError, PickIndex};

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

/// **What an id query is asked ABOUT**, beside the cursor: the picture
/// on screen and the index whose alphabet its ids are words of.
///
/// **Both halves, because neither is a subset of the other.** The
/// query's answer is an id the GPU read out of the picture identified
/// by `revision`, and it is resolved through the id map of the index
/// identified by `generation` — so a change to either makes the
/// outstanding answer describe something nobody is asking about:
///
/// - **A new picture at the same generation.** Hiding a part rebuilds
///   the scene from the index already in hand
///   ([`crate::app::ViewerApp::sync_scene`] rebuilds on a display
///   revision or a focus-set change too), so the drawn ids lose the
///   hidden part's patches while the generation holds still. Keyed on
///   the generation alone the query holds, the GPU's answer for the
///   picture that still had the part stays matched, and the ray path's
///   fresh *nothing* is reported as *the two picking paths disagree* —
///   the sentence issue #1097 §4 tells an operator to read as an
///   `R32Uint` clear fault.
/// - **A new generation at the same picture.** A rebuild that REFUSES
///   does not bump the revision (`sync_scene` marks the pair current
///   only on success), so an index that landed over a refused rebuild
///   is a new generation beside the picture already on screen. Keyed on
///   the revision alone the query holds, and the hover the pick path
///   skips on a [`IdStep::Hold`] is a question about the DOCUMENT,
///   which has moved.
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct IdSubject {
    /// [`crate::app::ViewerApp`]'s scene revision: the identity of the
    /// mesh the id pass renders, bumped on every successful rebuild.
    pub revision: u64,
    /// The generation of the index in hand, `None` while one is being
    /// built.
    pub generation: Option<Generation>,
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
    /// The cursor and the subject the outstanding query was asked
    /// about. `None` when nothing is outstanding.
    asked: Option<([f64; 2], IdSubject)>,
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

    /// Advance the log for this frame's cursor and subject.
    ///
    /// `cursor` is `None` when the pointer is outside the pane.
    /// `subject` is what the query is about beside the pointer — the
    /// picture and the index ([`IdSubject`], which carries the argument
    /// for asking both) — so a query is re-asked when either changes
    /// under a still cursor.
    pub fn step(&mut self, cursor: Option<[f64; 2]>, subject: IdSubject) -> IdStep {
        let Some(cursor) = cursor else {
            self.asked = None;
            return IdStep::Void;
        };
        if self.asked == Some((cursor, subject)) {
            return IdStep::Hold;
        }
        // Saturating past zero: zero is the "nothing was ever asked"
        // serial the answer channel is initialised to, so a wrap must
        // not land on it.
        self.serial = self.serial.wrapping_add(1).max(1);
        self.asked = Some((cursor, subject));
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
    /// What the ray path named: **a SET**, because the kernel's door
    /// answers one face, nothing, or a certified TIE between several
    /// ([`pncad::select::HitTestError::Ambiguous`]). Empty is nothing
    /// under the cursor; one name is an ordinary answer; several are
    /// faces the arithmetic cannot order, and the rasterizer cannot
    /// either — the pixel there falls to depth rounding, so the id
    /// pass naming ONE of them is not a disagreement
    /// ([`disagreement`]).
    pub from_ray: Vec<StableName>,
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
    ///
    /// Destructured rather than field-read, which is what holds the
    /// paragraph above to the value: the argument is that BOTH halves
    /// are load-bearing, and a third field added to
    /// [`Disagreement`] and left out of this sentence would falsify it
    /// silently. In the pattern it is E0027 instead.
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        let Self { from_gpu, from_ray } = self;
        let one = |name: &StableName| format!("{name} ({:?})", name.path);
        let gpu = match from_gpu {
            Some(name) => one(name),
            None => "nothing".to_owned(),
        };
        let ray = match &from_ray[..] {
            [] => "nothing".to_owned(),
            [name] => one(name),
            tied => format!(
                "tied between {}",
                tied.iter().map(one).collect::<Vec<_>>().join(" and ")
            ),
        };
        write!(
            f,
            "picking paths disagree at the cursor: id buffer {gpu}, ray {ray}"
        )
    }
}

impl Disagreement {
    /// This disagreement as a message for the status line.
    ///
    /// [`Subject::Cursor`]: it is a claim about what lies under THIS
    /// cursor over THIS picture, and [`crate::frame::cursor_status`]
    /// retires it on
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
/// # A tie is not a disagreement when the raster chose inside it
///
/// `from_ray` is a SET ([`Disagreement::from_ray`]). The two paths
/// AGREE when the id buffer's name is one of it — the kernel said
/// these faces cannot be ordered, and the rasterizer picking one of
/// them is the depth buffer's rounding, not a contradiction of
/// anything the kernel claimed. They disagree when the id buffer
/// names a face outside the set, nothing where the ray named
/// something, or something where the ray named nothing.
///
/// # A refused ray path is no verdict
///
/// `from_ray` is the ray path's answer OR its refusal
/// ([`crate::pickindex::PickIndex::faces_under_cursor`]'s own
/// `Result`), because an empty answer and a refusal are different
/// facts: the first says nothing is under the cursor, the second says
/// nothing about the cursor at all. A refused path made no claim for
/// the id pass to contradict, so the two are not compared — the same
/// answer this function gives when the id pass has no fresh claim of
/// its own. Reading the refusal as an empty set would publish *ray
/// nothing* on exactly the cursors the kernel declines, which are the
/// ones the id pass is likeliest to answer with a face.
///
/// The refusal is not dropped by being declined here. The hover path
/// asks the same un-projection and the same hit test on the same
/// frame (`PickIndex::hovered_for` seeds exactly as
/// `faces_under_cursor` does), and says what refused through
/// [`crate::frame::pick_refusal`]; a second, cursor-subject sentence
/// here would announce that refusal twice, once as a disagreement it
/// is not.
///
/// `answer` is the raw channel word (`serial << 32 | id`); `expected`
/// is [`IdQueryLog::outstanding`]. `None` means "no verdict": no query
/// outstanding, a stale answer, a refused ray path, or the two agree.
pub fn disagreement(
    index: &PickIndex,
    answer: u64,
    expected: Option<u32>,
    from_ray: Result<&[StableName], &PickError>,
) -> Option<Disagreement> {
    if expected? != (answer >> 32) as u32 {
        return None;
    }
    let Ok(from_ray) = from_ray else {
        return None;
    };
    let id = answer as u32;
    let from_gpu = if id == IdMap::NOTHING {
        None
    } else {
        index
            .name_of(id)
            .and_then(|name| name.as_ref().ok())
            .cloned()
    };
    let agrees = match &from_gpu {
        None => from_ray.is_empty(),
        Some(name) => from_ray.contains(name),
    };
    (!agrees).then(|| Disagreement {
        from_gpu,
        from_ray: from_ray.to_vec(),
    })
}
