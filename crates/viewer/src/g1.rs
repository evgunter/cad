//! **G1's preview/commit machine, held once** — the three rules every
//! gesture in this crate obeys, with each gesture's own refusal
//! vocabulary as a parameter.
//!
//! `crates/viewer/GUI-DESIGN.md` G1 ratifies the shape: *a gesture
//! emits preview edits against scratch state and exactly one committed
//! value on release*. Two gestures implement it — the value drag
//! ([`crate::session::DocSession`]'s, over a slot or a document
//! parameter) and the free-move probe ([`crate::display::DisplayState`]'s,
//! over an instance's frame) — and they own different value kinds,
//! different validation and different side effects. They are NOT one
//! type and a generic over their values would buy nothing; what they
//! share is the transition rules, which is what this module holds:
//!
//! - **a begin refuses when one is already in flight**, and the target
//!   is validated only after the slot is known free, so a refused
//!   begin never runs a check against a gesture it is not opening;
//! - **a preview REPLACES the value in flight** rather than composing
//!   with it, and refuses when nothing is in flight or when the
//!   operation names another gesture;
//! - **a commit lands exactly one value, and a gesture that never
//!   previewed lands nothing**, refusing by the same two names.
//!
//! # What holding them here makes impossible
//!
//! [`Slot`]'s in-flight state is private to this module, so a caller
//! cannot read it, take it or replace it except through the four doors
//! below. A rule about the transitions therefore cannot be spelled
//! anywhere else, and a rule changed here changes for both gestures in
//! the same edit — which is the property the chrome's own mapping
//! bought one layer up and this layer did not have.
//! `widgets::drag_ops` carries what that cost: two
//! hand-written copies of one mapping, one of them missing an arm.
//!
//! **What it does not do** is make a NEW shared rule land here rather
//! than in both callers. The closures each door takes are the caller's
//! own — the target's validation, the value's — and a rule written
//! inside one of those is written for one gesture. Holding the
//! transitions once is what is mechanical; the judgement about where a
//! new rule belongs is not.
//!
//! # The vocabularies stay apart
//!
//! [`Refusals`] is the three words one gesture speaks for the three
//! states, handed in per call. The value drag says `NoGesture` /
//! `GestureInFlight` / `WrongGesture`
//! ([`crate::session::refuse::Refusal`]) and the probe says
//! `NoFreeMove` / `FreeMoveInFlight` / `WrongFreeMove`
//! ([`crate::display::DisplayFault`]); they are different vocabularies
//! about different subjects and merging them is not what holding the
//! rules once requires.
//!
//! Module kind: **vocabulary** — it names no driver type and no
//! `app`-only crate (`crates/viewer/README.md`, Module boundaries).

/// **One gesture's words for the three states a G1 door can be in.**
///
/// A struct rather than three parameters because all three are the
/// same type and mean different things: positionally they sit one
/// transposition away from a door that says *finish the drag first*
/// where it means *no drag is in progress*, with nothing between the
/// mistake and the user to catch it. The same argument
/// `widgets::GestureVocabulary` makes about the operations,
/// one layer down about the refusals.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Refusals<Fault> {
    /// Nothing is in flight, and this operation drives a gesture.
    pub none: Fault,
    /// One is already in flight, and this operation opens a gesture.
    pub in_flight: Fault,
    /// One is in flight, but not the one this operation names.
    pub wrong: Fault,
}

/// A gesture in flight, or nothing: the state the three rules are
/// about.
///
/// `Held` is what the begin captured and the gesture is defined
/// against for its life — a value drag's target and base document, a
/// probe's instance. `Value` is what a preview replaces and a commit
/// lands.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Slot<Held, Value> {
    /// **Private, and that is the mechanism**: the doors below are the
    /// only way to observe or move this, so no caller can hold a
    /// second copy of a rule about it.
    open: Option<Open<Held, Value>>,
}

/// The open gesture's own state.
#[derive(Debug, Clone, PartialEq, Eq)]
struct Open<Held, Value> {
    held: Held,
    /// The last previewed value — `None` until the first preview,
    /// which is the whole of the no-move rule's state.
    value: Option<Value>,
}

impl<Held, Value> Default for Slot<Held, Value> {
    fn default() -> Self {
        Self::closed()
    }
}

impl<Held, Value> Slot<Held, Value> {
    /// No gesture in flight.
    #[must_use]
    pub const fn closed() -> Self {
        Self { open: None }
    }

    /// What the gesture in flight was opened on, if one is.
    #[must_use]
    pub const fn held(&self) -> Option<&Held> {
        match &self.open {
            Some(open) => Some(&open.held),
            None => None,
        }
    }

    /// The gesture in flight **and the value it is showing** — `None`
    /// both when nothing is in flight and when nothing has been
    /// previewed, because a gesture that has not moved shows nothing.
    #[must_use]
    pub fn previewing(&self) -> Option<(&Held, &Value)> {
        let open = self.open.as_ref()?;
        Some((&open.held, open.value.as_ref()?))
    }

    /// **Rule 1. Open a gesture, refusing one that is already open.**
    ///
    /// `held` runs only once the slot is known free, so a begin that is
    /// refused runs no validation and mints no state: the target's
    /// check answers for a gesture that is actually being opened. Its
    /// refusal is the caller's own — a slot that is not there, an
    /// instance a mate names — and is returned unchanged.
    ///
    /// # Errors
    ///
    /// `words.in_flight` when a gesture is already open, or whatever
    /// `held` refuses.
    pub fn begin<Fault>(
        &mut self,
        words: Refusals<Fault>,
        held: impl FnOnce() -> Result<Held, Fault>,
    ) -> Result<(), Fault> {
        if self.open.is_some() {
            return Err(words.in_flight);
        }
        self.open = Some(Open {
            held: held()?,
            value: None,
        });
        Ok(())
    }

    /// **Rule 2. Replace the value in flight**, refusing when there is
    /// no gesture or when `names` does not answer for the open one.
    ///
    /// The name is checked before `value` runs, so an operation that
    /// belongs to another gesture computes nothing and previews
    /// nothing. `value` is the caller's step — the edit and its
    /// application for a value drag, the rigid-motion check for a probe
    /// — and it REPLACES what was there rather than composing with it,
    /// which is why it is handed the begin's `Held` and never the last
    /// preview. Its second component is returned to the caller
    /// untouched, for the side products a preview has (the edit a panel
    /// reports, the scratch document it shows).
    ///
    /// A refused `value` leaves the gesture open and its previous
    /// preview standing.
    ///
    /// # Errors
    ///
    /// `words.none` when nothing is in flight, `words.wrong` when
    /// another gesture is, or whatever `value` refuses.
    pub fn preview<Fault, Extra>(
        &mut self,
        words: Refusals<Fault>,
        names: impl Fn(&Held) -> bool,
        value: impl FnOnce(&Held) -> Result<(Value, Extra), Fault>,
    ) -> Result<Extra, Fault> {
        let Some(open) = self.open.as_mut() else {
            return Err(words.none);
        };
        if !names(&open.held) {
            return Err(words.wrong);
        }
        let (previewed, extra) = value(&open.held)?;
        open.value = Some(previewed);
        Ok(extra)
    }

    /// **Rule 3. End the gesture and hand back the one value it
    /// landed** — `None` when it never previewed, which is the no-move
    /// rule: a click that happened to land on a field commits nothing,
    /// costs no undo step and asks for no re-evaluation.
    ///
    /// The name is checked **before** the gesture is taken, so a
    /// refused commit leaves the gesture it does not name open: the
    /// release event of one field is not a release of another, and a
    /// gesture that ended here would end with nobody having let go of
    /// it.
    ///
    /// # Errors
    ///
    /// `words.none` when nothing is in flight, `words.wrong` when the
    /// gesture in flight is not the one `names` answers for.
    pub fn commit<Fault>(
        &mut self,
        words: Refusals<Fault>,
        names: impl Fn(&Held) -> bool,
    ) -> Result<Option<(Held, Value)>, Fault> {
        let Some(open) = self.open.take_if(|open| names(&open.held)) else {
            return Err(match self.open {
                Some(_) => words.wrong,
                None => words.none,
            });
        };
        Ok(open.value.map(|value| (open.held, value)))
    }

    /// **Abandon the gesture**, answering whether it had previewed —
    /// the same question the commit answers, because the caller owes
    /// the same work: a gesture that put nothing on screen has nothing
    /// to take off it.
    ///
    /// It names no gesture: the state a cancel exists for is a drag
    /// whose field is no longer drawn, so there is nothing left to
    /// name it with (`crates/viewer/README.md`, Every gesture has a
    /// cancel door).
    ///
    /// # Errors
    ///
    /// `words.none` when nothing is in flight.
    pub fn cancel<Fault>(&mut self, words: Refusals<Fault>) -> Result<bool, Fault> {
        match self.open.take() {
            None => Err(words.none),
            Some(open) => Ok(open.value.is_some()),
        }
    }

    /// **Forget the gesture without refusing**, answering whether one
    /// was in flight.
    ///
    /// Not a fourth rule: this is the door for a holder that is
    /// discarding its whole state rather than ending a gesture — a
    /// document replaced under the display state, an instance the
    /// document no longer admits. A user's release is [`Self::commit`]
    /// and a user's abandonment is [`Self::cancel`]; neither is this.
    pub fn discard(&mut self) -> bool {
        self.open.take().is_some()
    }
}

#[cfg(test)]
mod tests {
    #![allow(clippy::expect_used)]

    use super::{Refusals, Slot};

    /// A toy vocabulary: the rules are held once, so they are asserted
    /// once, over words that belong to no gesture.
    #[derive(Debug, PartialEq, Eq)]
    enum Word {
        None,
        InFlight,
        Wrong,
        Target,
        Value,
    }

    fn words() -> Refusals<Word> {
        Refusals {
            none: Word::None,
            in_flight: Word::InFlight,
            wrong: Word::Wrong,
        }
    }

    /// `Held` is a name; `Value` is a number.
    fn slot() -> Slot<&'static str, i32> {
        Slot::closed()
    }

    fn open(slot: &mut Slot<&'static str, i32>, held: &'static str) -> Result<(), Word> {
        slot.begin(words(), || Ok(held))
    }

    fn preview(
        slot: &mut Slot<&'static str, i32>,
        named: &'static str,
        value: i32,
    ) -> Result<(), Word> {
        slot.preview(words(), |held| *held == named, |_| Ok((value, ())))
    }

    #[test]
    fn a_begin_under_an_open_gesture_is_refused_and_never_validates() {
        let mut slot = slot();
        open(&mut slot, "a").expect("the door answers");
        let mut validated = false;
        let refusal = slot
            .begin(words(), || {
                validated = true;
                Err(Word::Target)
            })
            .expect_err("the door refuses");
        assert_eq!(refusal, Word::InFlight);
        assert!(
            !validated,
            "a refused begin ran its target check anyway, so the check answered about a gesture nothing was opening"
        );
        assert_eq!(slot.held(), Some(&"a"), "the open gesture was disturbed");
    }

    #[test]
    fn a_refused_target_opens_nothing() {
        let mut slot = slot();
        assert_eq!(
            slot.begin(words(), || Err(Word::Target))
                .expect_err("the door refuses"),
            Word::Target
        );
        assert_eq!(slot.held(), None);
    }

    #[test]
    fn a_preview_replaces_the_last_and_shows_the_latest() {
        let mut slot = slot();
        open(&mut slot, "a").expect("the door answers");
        preview(&mut slot, "a", 1).expect("the door answers");
        preview(&mut slot, "a", 2).expect("the door answers");
        assert_eq!(slot.previewing(), Some((&"a", &2)));
        assert_eq!(
            slot.commit(words(), |held| *held == "a")
                .expect("the door answers"),
            Some(("a", 2))
        );
    }

    #[test]
    fn nothing_drives_a_gesture_that_is_not_there() {
        let mut slot = slot();
        assert_eq!(
            preview(&mut slot, "a", 1).expect_err("the door refuses"),
            Word::None
        );
        assert_eq!(
            slot.commit(words(), |held| *held == "a")
                .expect_err("the door refuses"),
            Word::None
        );
        assert_eq!(
            slot.cancel(words()).expect_err("the door refuses"),
            Word::None
        );
    }

    #[test]
    fn another_gestures_preview_and_commit_are_refused_and_change_nothing() {
        let mut slot = slot();
        open(&mut slot, "a").expect("the door answers");
        preview(&mut slot, "a", 1).expect("the door answers");
        assert_eq!(
            preview(&mut slot, "b", 9).expect_err("the door refuses"),
            Word::Wrong
        );
        assert_eq!(
            slot.commit(words(), |held| *held == "b")
                .expect_err("the door refuses"),
            Word::Wrong
        );
        assert_eq!(
            slot.previewing(),
            Some((&"a", &1)),
            "another field's drag steered the open gesture"
        );
    }

    #[test]
    fn a_gesture_that_never_previewed_commits_nothing_and_cancels_nothing() {
        let mut slot = slot();
        open(&mut slot, "a").expect("the door answers");
        assert_eq!(
            slot.commit(words(), |held| *held == "a")
                .expect("the door answers"),
            None
        );
        assert_eq!(slot.held(), None, "the commit left the gesture open");

        open(&mut slot, "a").expect("the door answers");
        assert!(!slot.cancel(words()).expect("the door answers"));
    }

    #[test]
    fn a_cancel_says_it_had_previewed() {
        let mut slot = slot();
        open(&mut slot, "a").expect("the door answers");
        preview(&mut slot, "a", 3).expect("the door answers");
        assert!(slot.cancel(words()).expect("the door answers"));
        assert_eq!(slot.held(), None);
    }

    #[test]
    fn a_refused_preview_leaves_the_last_one_standing() {
        let mut slot = slot();
        open(&mut slot, "a").expect("the door answers");
        preview(&mut slot, "a", 1).expect("the door answers");
        let refusal = slot
            .preview(
                words(),
                |held| *held == "a",
                |_| Err::<(i32, ()), Word>(Word::Value),
            )
            .expect_err("the door refuses");
        assert_eq!(refusal, Word::Value);
        assert_eq!(slot.previewing(), Some((&"a", &1)));
    }

    #[test]
    fn discard_forgets_without_refusing() {
        let mut slot = slot();
        assert!(!slot.discard());
        open(&mut slot, "a").expect("the door answers");
        assert!(slot.discard());
        assert_eq!(slot.held(), None);
    }
}
