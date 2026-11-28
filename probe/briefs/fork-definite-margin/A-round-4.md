Round 4 reply: fork-definite-margin

## For Ev

**The ratified text settles the promise, and it settles on (iii): the real geometry at every member, plus the `f64` witness's decisions.** It makes no promise of its own about an `f64` build at any other member. Under (iii) the two positions meet at **no hold-back**, so I withdraw my round-3 rule. What stays is the other reply's test-only instrument, which flags any decision where the form's answer and a member's rounded reading differ.

The sentences that settle it:
- **E6, the leaf protocol:** "Every predicate definite AND the verdict vector matching the witness branch's → leaf certified … Measure enclosures are containment-true over the whole leaf."
- **What "containment-true" means**, from DESIGN `[scalars]`: "containment of the true value is the interval contract." The true value is the real one.
- **E12, soundness:** "the identity holds at every real parameter point in the box."
- **What protects a member's `f64` build** is not the leaf but DESIGN `[simple.conditioned]`: "Every computation is posed so that it is well-conditioned where its result is decided, and at `f64` its values are then treated as real numbers … the `Interval` lane exists for what must account for it." `[verdicts]`: "K's headroom absorbs f64 rounding … a formula whose error is amplified … is a defect, re-posed, never covered by K."

So a member's build agrees with a certified leaf by the posing rule, a property of the code, wherever that rule holds. Where rounding decides a decision, the defect is the pose, at its caller. "N% of the mass has a valid build" is a statement about the real geometry, which is what an `f64` build is treated as. (Likely: no sentence says "not the member's build" in so many words, but every sentence that speaks of containment speaks of the true value.)

1. **Did today's kernel keep promise (ii), the `f64` build at every member? Only in part, and E12 already gave it up in three places.**
   - **Plain numeric decisions, kept:** the replay runs the `f64` build's ops in the same order with no fused operations (`[scalars]`). Outward rounding is monotone, so a definite V contains each member's rounded result at each op (likely). For transcendentals this needs the interval backend to pad by at least the `f64` function's own error; I have not checked that.
   - **A proven `Zero`, broken:** E12's own example, `(x + w) − w − x`, reads `Zero` over the box while a member rounds it to 2.8e-17.
   - **Closed spellings, broken:** a numeric scalar "carries no expression … there the general spelling is taken". A member's `f64` build computes the chord over its length while the leaf computes the exact axis, so V does not contain the member's value.
   - **Decision-read `select` arms, broken:** the tier picks an arm from the real sign; a member's `f64` build picks it by rounding.

   So the hold-back would add a promise the kernel has not had since E12; it would not restore one.
2. **The twin.** In what is *promised to a member's build*, there is no difference. Both answers are about the real margin, and in both a member's `f64` may read a rounding-size sign. My round-3 distinction (a theorem whatever the literals' rounding, against a fact about how they were rounded) is about design intent, not about the promise. And 6.58e-19 is a theorem about its literals. One rule, therefore: no hold-back for either. (likely)
3. **The terminal marking and the child that samples another member.**
   - With terminal marking, the parent is refused at the root and its children are never run. The report is a function of the box in E6's literal sense, because the sampled member is a fixed rule of the brackets. But it is not a geometric fact: the same sub-box, studied on its own, could certify.
   - Without terminal marking, the driver bisects, and whether a child certifies depends on which member each split happens to sample.

   Either way a sampled-member rule turns rounding noise into verdict structure. That is a further reason against it, beyond the cost.
4. **Where this lands.** It is settled, not split. E12's decision text is the other reply's round-2 sentence without the hold-back clause. The E6 sentence: "Every decision of a certified leaf holds of its margin's real value at every member, and the leaf takes the decisions the witness took; a member's own `f64` build agrees wherever its decisions are posed as `[simple.conditioned]` requires." What you give up, stated plainly: a member's `f64` build can differ from the leaf within rounding of a band edge. That is new and as thin as rounding; at an exact comparison it is already true of every proven `Zero`.

**If you read the promise as (ii) instead**, these are the two sentences to choose between:
- **(iii):** "A certified leaf certifies the real geometry and the witness's build." It gives up member builds that are decided by rounding, which the posing rule calls defects.
- **(ii):** "A certified leaf certifies every member's `f64` build." It gives up the proven `Zero` at exact comparisons, closed spellings and decision-read arms, all of which would have to be held back as well, and a sampled member only approximates it anyway.

I would choose (iii) (likely).

**What M16b would have to show to change this.**
- Disagreements between the form's answer and P (one member's rounded reading) at a tolerance band that are more than a handful, each a pose defect. Or, at the least band, more than about 1% of the decided asks on the tour. Either would mean `[simple.conditioned]` is not true of the kernel's poses broadly, so the promise as I read it diverges from the builds users get, and it becomes your question.
- Any disagreement on a certified leaf's witness path, and its share of the refused mass as `FlipCrossing`. Each is a caller to re-pose.

## For the orchestrator

- **I reverse my round-3 position**, on the ratified text above. Both replies should now read "no hold-back; test-only instrument".
- **Not checked:** whether `interval-transcendentals` pads enough to contain the `f64` libm-or-in-house result, which is what (ii)'s "kept for plain decisions" rests on for trig. If it does not, (ii) was never kept even there.
- **The closed-spelling break of (ii)** is read from E12's text ("there the general spelling is taken"), not traced in code.
- **The instrument's rows** are M16b's P-disagreement column. Each disagreement it finds files as a pose defect at its caller (`[simple.conditioned]`), not as a tier defect.
