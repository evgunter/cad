## For Ev

**Recommendation (likely): D, the door through the at-rest policy, with the other reply's admission test written into its ledger entry.** Point 1 goes partly against D, points 2 and 3 go for it, and what point 1 leaves is closed by making the door's one product a *line's* place, not a number's.

**Point 1. Does the door limit anything? Within `topo`, no; I concede that.** Read: `ShellDoor::certified` (constructor bounded `CertifiedPolicy + CertifiedBounds`), `AtRestPolicy::shell_door() -> Option<Self::ShellDoor>`, and the `f64` and probe impls returning `Some(ShellDoor::certified())`. The bracket bound is spelled once, on the constructor. But the policy's method hands the made door to every `T: AtRestPolicy` body, with no further bound. So any policy-bounded function, which is most of `topo`, could obtain the door and make a place. The other reply is right about that, and I was wrong in round 3 to say "the bound is what stops code that does not hold the door".

What D still adds over P beyond the home:
- *Reach by crate.* The policy lives in `topo`. Deciding code beneath it (`geom`, `geom-brep`, `geom-core`) cannot name the door. `Decide::place` would be callable from every crate.
- *What can be made.* D's constructor makes a line's place from a point and a direction, and the only evaluator that takes it is the ray. P's `place(x)` turns any scalar into a place, for any evaluator, present or future. A midpoint decision through D has to be a ray reading.
- *Where the read is spelled.* `Bounds`, on one constructor, in a file with a ledger entry: what the existing gate reads.

What P's allowlist adds over D's bound: a list of call sites with a stated argument. That is worth having under D too, and costs nothing there: the ledger entry is that list (one site today) and should carry the place-invariance argument.

**Point 2. Does #990's letter forbid a second projection? Not under D; under P one sentence becomes false on its face.** Read: the passage in `real.rs` from "The two shapes that are not decisions" to the #701 sentence. It opens "A door **outside the seams** has ... no `f64` out of a `T: Decide` for a refusal payload". "The one projection is `Real::told_reading`" follows a colon after "A derived quantity leaves as text and only as text". So it speaks of how a derived quantity leaves a door outside the seams. It cannot be universal as written: `Bounds::lo`/`hi` is itself a projection of a value at `T` to a number, it exists, and the passage sets it apart as the seams.
- *After D:* no sentence is false. The read is `Bounds`, inside a ledgered seam; nothing leaves a door; no method is added to the scalar.
- *After P:* the scalar has two projection methods, both callable outside the seams. "The one projection is `Real::told_reading`" is then false read alone, and true only by the context argument above. P has to amend it, as the other reply says.
The word "one" forbids a second projection *on the scalar, outside the seams*. D adds none.

**Point 3. A dual under one reader: the same under both, and it removes P's reason to reach a dual.** Read: `geom/src/places.rs` (the private step is bounded `T: Real<Value = T>`, so no place evaluator exists at a dual; "it is first read without its tangent, through its `value_view()`, and evaluated there"). Under D the reader runs on the body's values, at the value scalar (`f64` for `Dual<f64>`, `Interval` for `Dual<Interval>`), and that scalar holds the door. Under P, `place()` at a dual returns the base value's place, but the ray made from it still cannot be a ray of duals (no step exists, and DL5.2 forbids one), so P also has to read the body on its values. Once it does, the place is taken at the value scalar there too, and "a projection on every deciding scalar including a dual" has no caller at a dual. Both need a value view of a whole body, which neither of us has confirmed exists.

**If we still differ, the choice in two sentences.**
- **D:** the place of a line is made by one constructor in `geom`'s place module, bounded on the bracket read alone, and reaches the side reader as a tenth door of the at-rest policy; text changed: one sentence in the README's DL5.2 place row and one `Bounds` ledger entry stating the invariance argument; #990 untouched.
- **P:** every deciding scalar gains `place()` beside `told_reading`, gated by an allowlist of call sites; text changed: the same DL5.2 sentence, plus #990's "the one projection" amended to two and its #701 sentence extended.

D is the narrower grant (one product, one crate upward, no new scalar method) and changes less ratified text. P is the more uniform, and is the one to take if you expect places drawn from values to be wanted below `topo`.

**Confidence.** Point 1 as stated (the door is open to all policy-bounded code): sure. Point 2 (no sentence false after D; one after P): likely, it is a reading of a passage whose words are yours. Point 3: likely; the body's value view is unconfirmed. D over P: likely.

## For the orchestrator

- The DL5.2 sentence both replies now share needs one word changed under D: not "by `Decide::place`" but "by the place's own constructor, bounded on the bracket read alone".
- Under D the ledger entry should say three things: necessity (one line for every member; only values at `T` say where the face is), that it decides nothing (no accessor; every branch a named predicate), and the admission test (any line from outside a region reads the same side; the first-ray rule of `[ev]` PR 6592 where no region is bounded).
- Not checked: that every scalar's `Value` type implements the at-rest policy (needed for the reader to run on values under D); whether `Body` has a value view; whether the policy doc's count of "nine injected doors" is pinned by a row.
- The other reply's note on row 292 is right and applies to both mechanisms: Ev accepted the number-to-scalar step for places and held back "convert to f64 and back". A place drawn from a bracket is the other direction; say so in the PR.
