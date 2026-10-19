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
//! frame, and [`IdNews`] is what an answer amounts to once it has been
//! matched to the question it answers: the two picking paths'
//! [`Disagreement`], or an id the picture does not draw beside a ray
//! path that refused.
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

use pncad::document::{Doc, Evaluation, ProfileProgram, Said, Say, Speaker};
use pncad::prelude::StableName;
use pncad::select::UnnamedEntity;

use crate::display::DisplayView;
use crate::frame::{Message, Retold, Subject};
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

/// **What the id buffer said at the cursor**, read through the index
/// that drew the picture and the display view it drew it under.
///
/// Four answers, because the channel word is an id and not a name:
/// the index turns it into a name, or says it has none for a patch it
/// draws, or the picture draws no patch under it at all. The last two
/// are not the clear value, and reading either as *nothing* publishes
/// a claim the id buffer did not make.
#[derive(Clone, Debug, PartialEq)]
pub enum IdAnswer {
    /// [`IdMap::NOTHING`], the id buffer's clear value.
    ///
    /// **Not only "no geometry under the cursor".** The channel also
    /// carries [`IdMap::NOTHING`] for a readback that failed, so this
    /// arm holds both until the channel word tells them apart
    /// (`work/fit/id-readback-failure-reads-as-nothing-under-the-cursor`).
    Nothing,
    /// A drawn patch, by the name the index has for it.
    Named(StableName),
    /// A drawn patch the index has no name for: the naming layer's
    /// loud bug report ([`PickIndex::name_of`]), with its own refusal
    /// — a lookup's, since the index was built by one.
    Unnamed {
        /// The patch id the id buffer read back.
        id: u32,
        /// Why the patch has no name.
        error: UnnamedEntity,
    },
    /// An id no patch of this picture is drawn under: one the index
    /// never assigned, or one it assigned to a part whose placement the
    /// display view hides. [`PickIndex::scene_for`] keeps a hidden
    /// part's ids and draws none of them, so the index NAMES such an
    /// id and the picture does not show it, and reading it as that
    /// name would say the id buffer saw a face that is not on screen.
    /// The picture is the one this index drew under this view (the
    /// viewport compares against no other), so the word itself is
    /// wrong: the symptom of a corrupt readback. The view half of that
    /// holds only while the view in hand is the one the picture was
    /// drawn under, which a refused rebuild breaks
    /// (`work/vnews/the-cursor-reads-the-display-view-in-hand-not-the-one-the-picture-was-drawn-under.md`).
    Undrawn {
        /// The id the id buffer read back.
        id: u32,
    },
}

impl Say for IdAnswer {
    /// Each arm in the words of the layer that raised it: a name in
    /// its words ([`Speaker::name`]), an unnamed patch through its own
    /// refusal's sentence, and an undrawn id as the picture's own
    /// fact. The id is `id N` in every arm that carries one, because
    /// it is one `u32` read out of the id buffer, whatever it turns out
    /// to denote.
    fn say(&self, f: &mut core::fmt::Formatter<'_>, by: Speaker<'_>) -> core::fmt::Result {
        match self {
            Self::Nothing => f.write_str("nothing"),
            Self::Named(name) => write!(f, "{}", by.name(name)),
            Self::Unnamed { id, error } => {
                write!(f, "id {id}, a drawn patch: {}", Said(error, by))
            }
            Self::Undrawn { id } => write!(f, "id {id}, which no patch of this picture draws"),
        }
    }
}

/// The sentence where no document is at hand: each node by its tag.
impl core::fmt::Display for IdAnswer {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.say(f, Speaker::TAG)
    }
}

impl IdAnswer {
    /// The answer the id `id` denotes in `index`'s picture under
    /// `display`.
    ///
    /// **Whether the id is drawn is asked before what it is named**:
    /// the index names every part's ids, hidden or not, so a name is
    /// no evidence the picture shows the patch. A hidden part's id is
    /// [`IdAnswer::Undrawn`] whatever its name, including a name the
    /// naming layer refused, because the picture drawing no patch
    /// under it is the fact about the id buffer's word.
    pub fn of(index: &PickIndex, display: &DisplayView, id: u32) -> Self {
        if id == IdMap::NOTHING {
            return Self::Nothing;
        }
        // The id map is derived from the same windows as the name
        // table, so an id with a name has a patch, and the patch's node
        // is the placement the display view hides or not.
        let hidden = index
            .ids()
            .key_of(id)
            .is_some_and(|patch| display.hidden_placements.contains(&patch.node));
        match index.name_of(id) {
            None => Self::Undrawn { id },
            Some(_) if hidden => Self::Undrawn { id },
            Some(Ok(name)) => Self::Named(name.clone()),
            Some(Err(error)) => Self::Unnamed { id, error: *error },
        }
    }
}

/// The two picking paths' answers for one cursor, when they differ.
#[derive(Clone, Debug, PartialEq)]
pub struct Disagreement {
    /// What the id buffer said ([`IdAnswer`]).
    pub from_gpu: IdAnswer,
    /// What the ray path named: **a SET**, because the kernel's door
    /// answers one face, nothing, or a certified TIE between several
    /// ([`pncad::select::HitTestError::Ambiguous`]). Empty is nothing
    /// under the cursor; one name is an ordinary answer; several are
    /// faces the arithmetic cannot order, and the rasterizer cannot
    /// either — the pixel there falls to depth rounding, so the id
    /// pass naming ONE of them is not a disagreement
    /// ([`compare`]).
    pub from_ray: Vec<StableName>,
}

impl Say for Disagreement {
    /// The id side is [`IdAnswer`]'s own sentence. Every NAME on
    /// either side is said in its words, which tell two names of one
    /// table apart. The sentence reaches the user's status line
    /// ([`Disagreement::notice`]), so it carries no role path: the path
    /// a bug report replays is this value's `Debug`, for a log, and no
    /// surface of the viewer prints it.
    ///
    /// Destructured rather than field-read, so a third field added to
    /// [`Disagreement`] and left out of this sentence is E0027 rather
    /// than silently unsaid.
    fn say(&self, f: &mut core::fmt::Formatter<'_>, by: Speaker<'_>) -> core::fmt::Result {
        let Self { from_gpu, from_ray } = self;
        write!(
            f,
            "picking paths disagree at the cursor: id buffer {}, ray ",
            Said(from_gpu, by)
        )?;
        match &from_ray[..] {
            [] => f.write_str("nothing"),
            [name] => write!(f, "{}", by.name(name)),
            tied => {
                f.write_str("tied between ")?;
                for (i, name) in tied.iter().enumerate() {
                    if i > 0 {
                        f.write_str(" and ")?;
                    }
                    write!(f, "{}", by.name(name))?;
                }
                Ok(())
            }
        }
    }
}

/// The sentence where no document is at hand: each node by its tag.
impl core::fmt::Display for Disagreement {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.say(f, Speaker::TAG)
    }
}

impl Disagreement {
    /// This disagreement as a message for the status line.
    ///
    /// [`Subject::Cursor`]: it is a claim about what lies under THIS
    /// cursor over THIS picture, and [`crate::frame::cursor_status`]
    /// retires it on
    /// the id log's own judgement that the question has moved on.
    ///
    /// Its nodes are said from the landed document the index that drew
    /// the picture was built against, and its names within the tables
    /// of that document's evaluation ([`Speaker::within`]), both in
    /// `landed`.
    ///
    /// [`Retold::Again`]: the same hover says it again while the
    /// paths still disagree.
    pub fn notice(&self, landed: (&Doc<ProfileProgram>, &Evaluation<f64>)) -> Message {
        cursor_notice(self, landed)
    }
}

/// The one construction of a cursor message: [`Subject::Cursor`] and
/// [`Retold::Again`], said from the landed document within its
/// evaluation's tables, as [`Disagreement::notice`] says why.
fn cursor_notice(news: &impl Say, landed: (&Doc<ProfileProgram>, &Evaluation<f64>)) -> Message {
    let (doc, evaluation) = landed;
    Message::new(
        Subject::Cursor,
        Said(news, Speaker::of(doc).within(evaluation)).to_string(),
        Retold::Again,
    )
}

/// **What one fresh id answer has to say**, beside the ray path's
/// answer or refusal at the same cursor ([`compare`]).
#[derive(Clone, Debug, PartialEq)]
pub enum IdNews {
    /// The two paths both answered, and the answers differ.
    Disagreement(Disagreement),
    /// The ray path refused, so there is no ray answer to compare, and
    /// the id buffer answered an id no patch of this picture draws
    /// ([`IdAnswer::Undrawn`]). Such an id is wrong without a ray to
    /// contradict it, so a refusal beside it does not excuse it.
    BesideRefusal {
        /// The id the id buffer read back.
        id: u32,
    },
}

impl Say for IdNews {
    /// A disagreement in its own words; an undrawn id beside a refusal
    /// as what the id buffer read at the cursor, in [`IdAnswer`]'s
    /// words for it. The ray's refusal is not in the sentence: it is
    /// the ray path's news and is said in its own words ([`compare`]).
    fn say(&self, f: &mut core::fmt::Formatter<'_>, by: Speaker<'_>) -> core::fmt::Result {
        match self {
            Self::Disagreement(disagreement) => disagreement.say(f, by),
            Self::BesideRefusal { id } => write!(
                f,
                "id buffer at the cursor: {}",
                Said(&IdAnswer::Undrawn { id: *id }, by)
            ),
        }
    }
}

/// The sentence where no document is at hand: each node by its tag.
impl core::fmt::Display for IdNews {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.say(f, Speaker::TAG)
    }
}

impl IdNews {
    /// This news as a message for the status line: [`Subject::Cursor`]
    /// and [`Retold::Again`] for either arm, said from `landed`, as
    /// [`Disagreement::notice`] gives them.
    ///
    /// **A [`IdNews::BesideRefusal`] is never retired as cursor news.**
    /// It is raised only beside a ray refusal, which is said on the
    /// same frame as a [`Subject::Document`] notice
    /// ([`crate::frame::pick_refusal`]), and a frame joins its notices
    /// into one line under one subject. So the line wears `Document`,
    /// and [`crate::frame::cursor_status`]'s expiry of the cursor's
    /// subject does not reach it
    /// (`work/vnews/one-line-one-subject-loses-a-mixed-frames-expiry.md`).
    pub fn notice(&self, landed: (&Doc<ProfileProgram>, &Evaluation<f64>)) -> Message {
        cursor_notice(self, landed)
    }
}

/// Compare the id pass's answer against the ray path's, **by name**.
///
/// # Why names and not ids
///
/// One stable name can be drawn under several ids, so comparing raw
/// ids reports a disagreement whenever the two paths
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
/// something, or something where the ray named nothing — and
/// whenever it answers an id the index cannot name
/// ([`IdAnswer::Unnamed`], [`IdAnswer::Undrawn`]; below).
///
/// # A refused ray path is no comparison
///
/// `from_ray` is the ray path's answer OR its refusal
/// ([`crate::pickindex::PickIndex::faces_under_cursor`]'s own
/// `Result`), because an empty answer and a refusal are different
/// facts: the first says nothing is under the cursor, the second says
/// nothing about the cursor at all. A refused path made no claim for
/// the id pass to contradict, so the two are not compared. Reading the
/// refusal as an empty set would publish *ray nothing* on exactly the
/// cursors the kernel declines, which are the ones the id pass is
/// likeliest to answer with a face.
///
/// **Except an id the picture does not draw** ([`IdAnswer::Undrawn`]),
/// which needs no ray to be wrong: it is said on its own beside the
/// refusal ([`IdNews::BesideRefusal`]). *Nothing* and a name are
/// claims about the cursor that only a ray answer could contradict, and
/// a refused ray gives them no verdict — the same answer this function
/// gives when the id pass has no fresh claim of its own.
///
/// **An unnamed patch beside a refusal is no verdict either.** The
/// refusal a ray meets over a drawn patch is the hit test's
/// unnamed-entity arm, and there the id buffer's patch carries the same
/// naming error, so
/// saying [`IdAnswer::Unnamed`] beside that refusal would say one fact
/// twice.
///
/// Declining to compare is not dropping the refusal: it is the ray
/// path's news, not the comparison's, and it is said in the ray path's
/// own words ([`crate::frame::pick_refusal`]) by whoever asked the ray
/// — the pick path when it asked at this cursor, the viewport's
/// comparison when the pick path skipped the frame. Said here as well,
/// as a disagreement, it would be one refusal announced twice and
/// named as something it is not.
///
/// # An id the index cannot name is not nothing
///
/// The id side is read as an [`IdAnswer`], not as a name, because the
/// id buffer can answer an id the picture has no name for: a patch
/// whose name the naming layer refused ([`IdAnswer::Unnamed`]), or an
/// id no patch of this picture is drawn under ([`IdAnswer::Undrawn`]:
/// never assigned, or assigned to a hidden placement's part, which the
/// index would otherwise name). `display` is the view the picture and
/// the ray were both drawn and asked under, while the view in hand is
/// the one the picture was drawn under
/// (`work/vnews/the-cursor-reads-the-display-view-in-hand-not-the-one-the-picture-was-drawn-under.md`
/// is where it is not). Neither AGREES with any
/// ray answer. Agreement is a shared name, and these have
/// none, so silence here would claim an agreement nobody can check.
/// Nor is either said anywhere else. This comparison is the id
/// answer's only reader, so the notice is where each is said, with
/// the id, which is what issue #1097 §4 asks an operator to record.
/// The ray path's own brush with an unnamed face is a refusal, and
/// beside a refusal only an undrawn id is said (above).
///
/// `answer` is the raw channel word (`serial << 32 | id`); `expected`
/// is [`IdQueryLog::outstanding`]. `None` means "nothing to say": no
/// query outstanding, a stale answer, a refused ray path beside
/// anything but an undrawn id, or the two agree.
pub fn compare(
    index: &PickIndex,
    display: &DisplayView,
    answer: u64,
    expected: Option<u32>,
    from_ray: Result<&[StableName], &PickError>,
) -> Option<IdNews> {
    if expected? != (answer >> 32) as u32 {
        return None;
    }
    let from_gpu = IdAnswer::of(index, display, answer as u32);
    let Ok(from_ray) = from_ray else {
        return beside_refusal(&from_gpu);
    };
    let agrees = match &from_gpu {
        IdAnswer::Nothing => from_ray.is_empty(),
        IdAnswer::Named(name) => from_ray.contains(name),
        IdAnswer::Unnamed { .. } | IdAnswer::Undrawn { .. } => false,
    };
    (!agrees).then(|| {
        IdNews::Disagreement(Disagreement {
            from_gpu,
            from_ray: from_ray.to_vec(),
        })
    })
}

/// What an id answer amounts to beside a ray path that refused: only
/// an undrawn id is news ([`compare`] says why each arm is what it is).
fn beside_refusal(from_gpu: &IdAnswer) -> Option<IdNews> {
    match *from_gpu {
        IdAnswer::Undrawn { id } => Some(IdNews::BesideRefusal { id }),
        IdAnswer::Nothing | IdAnswer::Named(_) | IdAnswer::Unnamed { .. } => None,
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::unwrap_used, clippy::expect_used, clippy::panic)]

    use super::*;
    use pncad::document::VarId;
    use pncad::prelude::{CapEnd, EntityKind, NameRef, RecipeNodeId, RoleSeg};

    /// A disagreement says each name in its words, and no role path, on
    /// the wasm32 build's stack however deep the name nests: a name's
    /// words walk its nesting from their own stack.
    #[test]
    fn a_disagreement_over_a_name_nested_past_every_stack_says_it_on_the_smallest_stack() {
        const DEEP: usize = 20_000;
        let shown = test_utils::own_thread::on_the_smallest_stack(|| {
            let leaf = StableName {
                kind: EntityKind::Face,
                node: RecipeNodeId::new(0, test_utils::refusal::tagged(1)),
                path: vec![RoleSeg::Cap(CapEnd::End)],
            };
            let deep = (0..DEEP).fold(leaf, |n, _| StableName {
                kind: EntityKind::Face,
                node: RecipeNodeId::new(0, test_utils::refusal::tagged(2)),
                path: vec![RoleSeg::From {
                    read: VarId::new(0, test_utils::refusal::tagged(3)),
                    of: NameRef::new(n),
                }],
            });
            Disagreement {
                from_gpu: IdAnswer::Named(deep.clone()),
                from_ray: vec![deep],
            }
            .to_string()
        });
        assert!(!shown.contains("From"), "no role path: {shown}");
        assert_eq!(shown.matches("the end cap of node 000000000001").count(), 2);
    }

    /// **An unnamed patch beside a refused ray is no verdict**, and an
    /// undrawn id beside one is. No document plants the naming layer's
    /// bug arm, so the answer is built as the index would have stored
    /// it; the refused-ray rows in `pane::viewport` drive the other
    /// arms through a real index.
    #[test]
    fn beside_a_refusal_an_unnamed_patch_is_no_verdict_and_an_undrawn_id_is_news() {
        let unnamed = IdAnswer::Unnamed {
            id: 9,
            error: UnnamedEntity {
                node: RecipeNodeId::new(0, test_utils::refusal::tagged(2)),
                entity: editor_core::names::EntityRef {
                    body: 0,
                    key: editor_core::names::EntityKey::Body,
                },
            },
        };
        assert_eq!(
            beside_refusal(&unnamed),
            None,
            "the refusal already carries the naming error"
        );
        assert_eq!(
            beside_refusal(&IdAnswer::Undrawn { id: 9 }),
            Some(IdNews::BesideRefusal { id: 9 }),
        );
    }
}
