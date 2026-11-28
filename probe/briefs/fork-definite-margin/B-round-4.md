# Round 4 reply: what a certified leaf promises

## For Ev

**Where I land (likely): no hold-back, and the reason is now the promise, not the rounding.** The ratified text promises (iii): the real geometry at every member, plus the `f64` build at the witness alone. No sentence promises anything about an `f64` build at another member, and today's kernel does not keep such a promise either. If you want it, it is a new promise and a much larger piece of work than this rule.

**The question beneath: which promise.** I find no sentence that states (ii), the `f64` build at every member. The sentences that bear on it all speak of the real value:
- E12, soundness: "A symbolic `Zero` is a theorem about real arithmetic ... so the identity holds at every real parameter point in the box", and then "The f64 witness pass still evaluates every residual numerically at a point". That is (iii) in so many words: reals everywhere, `f64` at one point.
- E6: a leaf is certified where every predicate is definite "AND the verdict vector matching the witness branch's"; "Measure enclosures are containment-true over the whole leaf". DESIGN `[scalars]` says what containment is: "containment of the true value is the interval contract".
- DESIGN `[simple.conditioned]`: at `f64` values "are then treated as real numbers: nothing charges, pads or estimates float rounding ... the `Interval` lane exists for what must account for it".
- The consumer's sentence, "N% of the tolerance mass has no valid build", is about builds, and a user's build is `f64`. It reaches (ii) only through `[simple.conditioned]`'s standing assumption that an `f64` build is the real one. Nothing certifies that assumption per member.

So: the text settles (iii) for what is certified (likely). Whether you also intend (ii) is yours; nothing ratified says so.

**1. Does today's kernel already keep (ii)?** Only partly, so the hold-back would be adding something the kernel never had, not restoring it. (likely; the last two bullets are read from docs, not traced)
- Kept: through ring arithmetic. The member's `f64` run and the leaf's interval run do the same operations in the same order, and outward rounding contains each rounded result, so a definite V does enclose every member's rounded margin there.
- Broken by a proven `Zero`: the leaf answers `Zero` where a member's `f64` reads a rounding-size sign. E12 ratified that.
- Broken by a closed spelling at `Sym`: E12 says the value channel is the plain run's "except where a construction chose a closed spelling". The leaf holds an exact `1` where a plain `f64` member computes a quotient that may round to its neighbour.
- Broken by proposals: DESIGN `[scalars]` has the lane take solved parameters and fitted images from `f64` at the witness and certify them at the lane scalar. A member's own `f64` solve is not in the leaf's computation at all.

**2. The twin, on that footing.** "A theorem whatever the literals' rounding" against "a fact about how the inputs were rounded" is a difference in how the answer was reached, not in what a member's build is promised. In both, the member's `f64` sign is noise beside a true statement about the real margin. And the document's literals are its numbers: 6.58e-19 is a theorem about the document as written. Whether the author meant the vertex to be on the plane is a question of intent, which the `unproven-coincidence` lint answers, not certification. One rule for both: neither is held back.

**3. The terminal marking and the child.** This is the hold-back's real defect, and I missed it in round 2. With the plain rule, a decision is monotone under inclusion: a sub-box has tighter enclosures, so what is decided stays decided, and "terminal" (E6: refuse, never refine) is a proof that no sub-box decides. With a sampled member it is neither. A box refused terminal at its root can contain a sub-box that certifies, so the report for one region depends on which box the study happened to start from. Each leaf is still a function of its own box, but "terminal" would assert something one sample does not show.

**4. The split, if you want to choose it.**
- **A (mine).** "A certified leaf is a statement about the real geometry at every member, checked against the `f64` build at the witness." It gives up: a member's own `f64` build may decide differently where its rounding decides, namely within rounding of a tolerance band's edge (new with this change, thin) or at an exact comparison finer than rounding (true today of every proven `Zero`).
- **B.** "A certified leaf also vouches for the `f64` build at each of its members." It gives up: the reading's decisions wherever a sampled member's rounding disagrees, monotone reports, and it is still not delivered, since one sampled member is not every member and the three breaks in item 1 remain. Delivering it would mean a certified rounding bound over the box at every decision, and revisiting proven `Zero`, closed spellings and proposals.
- I would choose A. Likely, not sure: the choice turns on what you mean a certificate to say, and the text only leans.

**What M16b would have to show to move me.**
- At the least band, any share of disagreement only lists callers to inspect. It moves me only if one of them builds structure on the sign and cannot be re-posed.
- At tolerance bands, disagreement on more than about 1% of decided asks would move me. It would mean rounding is not small beside ε on those margins, which is a formula owed a re-posing under `[simple.conditioned]`; I would then hold back at tolerance bands until those are fixed.
- Held-back terminal leaves: any number that includes a leaf with a certifying sub-box confirms item 3.

**E6 sentence (A):** "Every decision of a certified leaf holds of its margin's real value at every member, and the leaf takes the decisions the witness took."

## For the orchestrator

- **The one check worth making before this goes to the owner:** whether the witness is a plain `f64` build or a `Sym<f64>` one. If the latter, the closed-spelling break in item 1 is between the witness and a user's plain build as well, which strengthens (iii) as the only promise the kernel can be keeping. I did not trace it.
- **Item 1's "kept through ring arithmetic"** assumes the interval transcendentals contain the `f64` lane's own results, not only the true value. The contract in `[scalars]` is the true value; I did not check the backend for more. If they do not, (ii) is broken there too.
- **Item 3 is checkable by M16b cheaply:** for each leaf the hold-back would refuse as terminal, run the rule on its two halves. One certifying half is the counterexample.
- **I would put the split to the owner even though I think the text leans A**, since both designers have now held both positions; the question is short, and it is about meaning, as the brief says.
- Round 3's test-only instrument still stands under A, unchanged.
