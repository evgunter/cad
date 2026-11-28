# Round 3 reply: whether the reading holds back

## For Ev

**I withdraw the hold-back.** The two designers no longer differ. The box decision is the band test on the intersection of the value's enclosure and the form's enclosure, with no extra rule. What I keep from my round-2 proposal is a test-only instrument (item 4), not a rule in the build.

**1. The witness-match argument.** It is complete for soundness, and my rule did not improve on it where I thought it did. Take an exact comparison (the *least band*: a band so narrow that any non-zero value reads its sign) on the witness's path, of a margin smaller than a member's rounding.
- *Witness and reading disagree.* The leaf is refused as a flip at every width. Under my hold-back the same leaf is also refused at every width, because the value's enclosure straddles at the least band and stays undecided. So the hold-back certified nothing more here; it only changed the label. And the flip label is the more useful one: it names the decision whose `f64` answer is rounding.
- *Witness and reading agree.* The leaf certifies. Another member's `f64` build may read the other sign. E6 does not promise anything about that build: its claim is about the replay at the box scalar and the witness's decisions, and "a function of its box alone" is about the leaf's verdict. What protects that member is not the leaf but the standing rule that an `f64` decision is posed so rounding does not decide it (DESIGN `[simple.conditioned]`). If rounding does decide it, two `f64` builds one unit-in-the-last-place apart already differ today, with no box anywhere: the defect is the caller's, and it predates this change. (likely)
- *Is "refused everywhere" the right report for a margin that is zero to rounding?* Yes, for an exact comparison: no build can take that decision reliably, at any scalar. It is no regression either: today the same leaf runs to its depth budget.

**2. The hold-back's costs, which I under-weighed.**
- It is a function of the box and deterministic (the sampled member is a fixed rule of the brackets), so it would not have broken E6 or the store.
- But it is not monotone: a child box samples another member, so a decision held back in a parent can stand in a child, or the reverse, for no geometric reason. Bisection is assumed to cure an undecided leaf, and at the least band no width would; it would need its own "terminal" marking.
- It costs a second walk of the expression on every decided ask, and must enter every atom the form's enclosure enters.
- A margin whose small exact sign is meaningful is refused by it. That alone is weak (such a margin belongs at a tolerance band, where 1e-17 reads `Zero`), but it adds to the above.

**3. The twin decides it.** A proven symbolic `Zero` at the least band already answers where a member's `f64` reads a rounding-size sign, and E12 ratified that. I can find no principled difference between "exactly zero, and the member's sign is noise" and "6.6e-19, and the member's sign is noise". One rule therefore means either no hold-back for both, or holding back proven identities at the least band too, which would undo part of what E12 exists for (parameter orders and same-foot tests that are identities). The first is right. (likely)

**4. What an exact comparison is for.** The least band's callers are `prove` (a bound against a fixed level; two parameters compared), `choose` (a heuristic's pick between sound candidates) and the census's side reading.
- For `choose`, either sign is sound, so an unresolvable sign harms nothing.
- For the census, the side is asked only where the residual's tolerance-band read is undecided, which at `f64` means the residual is beyond ε, far above rounding. Well posed.
- A caller that asks an exact sign of a quantity that is zero by construction, and builds on the answer, is the defect. The fix is at that caller, not at the reading.
- To find such callers: a test-only instrument (in no shipped build, as PR 7712's is) that reports every decision where the form's answer and the one-member rounded reading differ. That is my round-2 rule, kept as a defect-finder.

**5. Recommendation now, as one rule.** At an exact witness, where the value's enclosure V gives no answer, the decision is `band(V ∩ F)`, F being the form's enclosure over the box's brackets with each atom enclosed from its arguments; where F cannot be taken, `band(V)`. Nothing else.

**The sentence for E6:** "Every decision of a certified leaf holds of its margin's real value at every member, and the leaf takes the decisions the witness took."

**What it does not promise, stated so you can accept or reject it:** a member's own `f64` build can decide differently from the leaf in two places: a tolerance-band margin within rounding of the band's edge, and an exact comparison of a quantity smaller than its rounding. The first is new with this change and as thin as rounding. The second exists today for every proven `Zero`.

**E12 text:** the other reply's round-2 sentences. I drop my clause "or the margin read at one member with its rounding answers differently".

**Confidence.** No hold-back: likely. That my rule certified nothing more in the disagreeing case: sure (it follows from the value's enclosure straddling). That no least-band caller builds structure on a rounding-size sign: unsure; I read the three kinds named, not every `prove` site, and the instrument is how to find out.

## For the orchestrator

- **One thing to carry into the unit's brief:** the instrument in item 4, wired where PR 7712's `point::check` is, with a row asserting it asked. Its first run over the tour is also the "members contradict" column of the extended count, taken properly.
- **M16's 17 asks** should be re-read under that instrument once the reading is on: I still expect them not to occur (the residual reads `Zero` first), inferred from `census::undecided_signs`, not run.
- **The new thin gap at tolerance-band edges** (a member's `f64` margin at ε + rounding while the form reads ≤ ε) is new: today's value channel contains every member's `f64` margin, so it never certified there. I put it in the For Ev section on purpose; it is the one thing the owner gives up.
- **If a reviewer wants member fidelity later**, the other reply's rule (undecided and terminal where the band's escalation is below the margin's rounding at one point) is the right shape, better than mine: it fires only at bands narrower than rounding and carries the terminal marking mine lacked.
- **Not re-checked this round:** `Hull`'s mint sites, `atan`/`atan2` in the interval backend, the 15,200.
