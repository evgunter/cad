## For Ev

**Recommendation (likely): D, the door through the at-rest policy. I move back to it.** The three facts below settle it, and they settle it for the reason the other reply gives (where the bracket read is spelled), not for its claim that the bound limits who can make a place. I moved in round 3 on the letter of #990, and I misread that letter. With that corrected, the two of us should end on one answer: D. #990 is not amended.

The two mechanisms in this round's brief:
- **D**: a door value the scalar supplies through the at-rest policy (`AtRestPolicy`).
- **P**: a projection `Decide::place` on every deciding scalar.

**1. Does the door limit anything? Only by layer, not within `topo`.** The item I read is `crates/topo/src/props.rs`. The right is spelled on the constructor: `impl<T: CertifiedPolicy + geom_core::CertifiedBounds> ShellDoor<T>`, whose `certified()` is the only constructor. But any `T: AtRestPolicy` code can call `T::shell_door()` and receive the door at every certifying scalar without spelling anything. A place door would be the same: every function bounded on the at-rest policy could make a place.

What D adds over P beyond the home:
- Code bounded on `Decide` alone (`geom-core`, `geom`, `geom-brep`, the walk's own private functions) cannot make one.
- The one bracket read sits where `Bounds` is spelled, which the policy's own doc requires ("every door that reads a bracket spells `Bounds` where the reader can see it").

What neither design gives is a type-level guard inside `topo`. So D still needs P's safeguard: the `Bounds` ledger entry states the place-invariance argument for the side reader, and a gate or reviewer reads the door's call sites. That safeguard is an entry in the existing ledger, not a new allowlist.

**2. Does #990's letter forbid a second projection? No; I withdraw my round-3 claim.** The item I read is `crates/geom-core/src/real.rs`, "The two shapes that are not decisions". The passage's opening scopes it: "A door outside the seams has … no `f64` out of a `T: Decide` for a refusal payload." Its "the one projection is `Real::told_reading`" sits inside bullet 2, on derived quantities leaving a door, and reads "A derived quantity leaves as text and only as text: the one projection is …". So it states the one way a derived quantity leaves a door, not every way a value at `T` yields something.
- **After D**, no sentence is false. The place is made inside a ledgered seam, is no `f64`, and never leaves a door.
- **After P**, the #990 sentence also stays literally true, as long as no place enters a payload. But the next paragraph is strained: "a bracket reader under a name the gate does not read … stays out". `Decide::place` is a bracket reader on every deciding scalar, so it would need that sentence amended to admit a gated name.

P therefore changes ratified text and D does not.

**3. A dual: the same problem under both, and the same answer.** The place's number is not the issue. P takes it from the dual's base value. Under D, the dual's door could be built on its value scalar; DL5.1 allows a bracket read at a dual that returns the base bracket. The issue is the ray. The pinned private step is bounded `Value = T`, and DL5.2 says a place is never a `T`. So a ray at `T = Dual` built from a place needs one of two things, under either mechanism:
- a body read on its values, so the walk runs at `T::Value`. That is DL5.2's own "result comes back on the value channel", and what the place module says of any geometry with a tangent;
- or a sentence admitting a zero-tangent `T`.

I checked `topo` for a whole-body value view and there is none: no `Body<T>` to `Body<T::Value>` map, and topology holds no scalar.

My recommendation for the dual is the value-view body, read at `T::Value`. That gives one reader and one ray construction. The cost is a value view of `Body`, which does not exist yet. If you would rather not build it, the alternative final state is that at a dual the door has no values and the ray keeps riding the body. That loses nothing at `Dual<f64>`, where the value channel is a point, and keeps today's widths only at a dual over an enclosure. Confidence in the value view over riding: unsure. Whether any path runs the reader at a dual over an enclosure is unmeasured.

**Text under D.**
- **README DL5.2, place row, append:** "A place may also be read off a value held at `T`, by the place's own constructor, bounded on the bracket read alone and reached as a door its scalar supplies; it is admitted where what is decided at the place is the same at every place it could have been."
- **`Bounds` ledger, one entry:** the other reply's text, which I accept as written. Its argument: necessary, because the ray must be one line for every member and only values at `T` say where the face is; and it decides nothing, because the place has no accessor, every branch is a named predicate, and any line from outside a region reads the same side.
- **#990: unchanged.**

**Confidence.**
- The door limits by layer only: sure.
- "The one projection" lies within the payload bullet: sure.
- D changes less ratified text than P: sure.
- No body value view exists: likely (searched, not proven absent).
- The dual's value view as the final state: unsure.

## For the orchestrator

- **The bound goes on the door type's constructor, not on a policy method**, as `ShellDoor::certified` does. The unit's brief should say so.
- **The policy doc's "nine injected doors" count** is re-worded when the code lands.
- **Read the reach numbers with the real step at `Sym`.** The other reply's point stands: the private step at `Sym` mints unknowns, and M14's hand switch may have used literal points. Re-take one row with the real step before pinning widths.
- **If the value view is not built now:** file it as its own row, with the dual riding in the meantime. Don't build it inside the side-reading unit.
