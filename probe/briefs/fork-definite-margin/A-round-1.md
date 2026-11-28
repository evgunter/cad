Round 1 reply: fork-definite-margin

## For Ev

**Recommendation (unchanged, likely).** Where a margin's value reads no definite sign over the box at an exact witness (`Sym<Interval>`), the decision encloses the margin's plain normal form over the box's parameter brackets. It then applies the ordinary band test to the intersection of that enclosure and the value's. An in-band result is a measured zero, never a proven one, and it gets its own K outcome. One addition this round: a reader re-spelling that makes an identity fold to the zero form stays needed. A form whose atoms do not cancel has the same dependency inside it (`sqrt(r²) − r` encloses ± 0.1 over a ± 0.05 box). Only re-spellings made solely so that a non-zero margin reads its parameter once retire.

1. **Order.** Value; plain form (the zero form gives proven `Zero`); the box read decides if it can; PR 7712's point reading decides whether the ladder climbs; the ladder. The two proposed orders give the same answer on every ask, because they build the same plain form, enclose it the same way and apply the same band test. The point reading never changes an answer; it only gates the ladder, and in both orders the ladder climbs only where the point reading held 0. They differ in cost only: with the point reading first it runs on every undecided margin; with it after the box read it runs only where the box read did not decide. Since every undecided margin needs the plain form either way, the box read first is cheaper or equal.
   PR 7712's row `a_nonzero_margin_undecided_over_the_box_builds_no_form` becomes false as written. It should be re-cut as: "a margin the point reading refutes climbs no rung past the plain form" (no early walk, top reduction, door or retry).
2. **Which form: the plain form only.** It holds at every parameter value, it is the form the plain rung already builds, and its enclosure already enters `sqrt`/`abs`/`min`/`max`. The early form, with its decision-read arms, is sound at an exact witness, but it costs the early walk, whose deep reads at every `select` are an open cost row. To be added, the count would have to show a material share of margins whose plain form declines or straddles at a `select` atom, while the early form decides them and the sampled members agree.
3. **The decline list.** Where the read declines, the value's enclosure stands alone.
   - **Free** (`free_identity`): the node is an identity over a free variable, while the value is the caller's measured reading. They differ by design.
   - **Opaque**: the value was computed apart from the parameters, and the form holds an unknown with no bracket, so it cannot be enclosed.
   - **Hull** (`enclosure_hull`, keyed by its children's ids): it is a tie region between two values, not one real function of the parameters, so it has no real to enclose.
   - **Frozen**: a form past its budget, held as an atom with no bracket, so it cannot be enclosed.
   - **Poisoned**: there is no form.
   - **Any other atom** (`select`, the trig functions, an atom past `ENCLOSE_DEPTH`): `enclose_indet` has no enclosure for it. Trig could be added later with sound interval sin/cos.

   Registered identities are not on the list. The plain form never reads the door, so no sign can rest on an axiom.
   The structural question's closed spellings keep value and node in step (likely). The construction computes both channels from the spelling it chose, so they describe one real. E12's exception ("the value channel is the plain `T` run's except where…") is about the value differing from a plain run, not from its own node.
   Opaque and frozen atoms could later be enclosed by their own nodes' value enclosures, which is sound but loose. That is PR 7712's open difference; adopt it only if the count's decline breakdown is dominated by them.
4. **An empty intersection: agreed.** At an exact witness both enclosures contain the same real, so an empty meet is a kernel bug detectable only by re-derivation: DESIGN [failure.taxonomy] row 5, a `debug_assert!`. The release build then decides on the value's enclosure alone, which is today's answer and the conservative direction. It never applies the band test to an empty interval.
5. **The `f64` gap.** The sentence for E6: "Every decision of a certified leaf holds of the margin's real value, the construction's formula in exact arithmetic, at every member of the leaf, as a symbolic `Zero`'s already does." A member's rounded `f64` margin can differ from that only for a margin within rounding of a band edge. That gap already exists for every symbolic `Zero`.
6. **E12 text** (exact sentences).
   - Replace "Everything else decides numerically as today." with: "Every other margin decides on the intersection of its value's enclosure and its plain form's enclosure over the box, where the form can be enclosed; a zero read so is measured, not proven."
   - Soundness bullet: replace "the leaf replay's numeric channel is untouched for every non-identity margin, so E6's containment claims are unchanged" with: "every other margin is decided on certified enclosures of one real, so E6's containment claims hold of that real at every member of the leaf."
   - Affine rejection: replace "The symbolic tier subsumes it on every explicit identity; plain intervals suffice for the real margins." with: "The symbolic tier subsumes it: an identity by proof, any other margin by its form's enclosure."
7. **The re-spelled readers.**
   - **The side reader's ray**: still needed, for its own reason. The ray is a place the code chose, not a claim about the body (DESIGN [scalars], geom-core DL5.2). That holds whatever the decision reads.
   - **The one-plane reading** (`d₁ − (n₁·n₂)·d₂`): still needed. It makes two spellings of one plane an identity, so they are proven; a box read would give at best a measured zero, and only where the form happens to cancel.
   - **The wall's roots** (the radius recovered through a root): each such reading is still needed where it turns an identity into the zero form (the `sqrt(r²) − r` case above). Any part done only so that a non-zero margin reads its parameter once is a special case to retire. I did not trace which is which; each needs a per-site check (unsure).

**Confidence.** Items 1, 3 and 4: likely. Item 2: likely. Item 5: sure that this is E12's existing claim. Item 7: likely for the ray and the one-plane reading, unsure for the roots.

**What would change the recommendation:**
- **Any sampled `f64` member that disagrees** with a read decision and is not within rounding of a band edge. Stop: the form's real and the construction's real differ somewhere the decline list missed.
- **"Declines" dominating the undecided margins.** The design stands, but the next work is the decline list (opaque or frozen atoms enclosed by their own values, trig atoms), not the read.
- **"Encloses, straddles, members agree" dominating.** The form needs a centred or Horner enclosure before the band test.
- **Building the plain form costing a material share of leaf time on margins the read then fails to decide.** Then those margins should go through a gate (the point reading, or a centred form on the graph) before the form is built.

## For the orchestrator

- The other report's point that M15's `tilted` body is a fixture (a cap cut by a plane may fold to the zero form) is right to fold in. It is why the count includes a verb-built body. It does not change the mechanism.
- I checked on `boxfinish/refute-first` that `sym/point.rs`'s reading declines on `Opaque | Hull | Free`, so the two decline lists agree on those three.
- For item 7, "the wall's roots" is the likeliest reading: the three `…recovers-the-radius-through-a-root` / torus-reader rows. I did not open their fixes.
