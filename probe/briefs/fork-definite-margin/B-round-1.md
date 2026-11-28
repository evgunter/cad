# Round 1 reply: a margin definite over a box

## For Ev

**Recommendation, unchanged (likely):** at the symbolic box scalar, a decision the value leaves undecided reads the intersection of the value's enclosure and the enclosure of the margin's plain normal form over the box's brackets, through the ordinary band test. Two things moved: I take the other report's order (item 1), and its empty-intersection check (item 4). One thing I hold against it: none of the re-spelled readers retires (item 7).

**1. Order.** Value; plain form; a zero form gives `Zero`; the box read (the form's enclosure through the band test) decides if it can; then the point reading (PR 7712: the expression read at one point of the box, which proves "not an identity" when it excludes 0); then the remaining rungs only where the point reading held 0.
- The two orders give the same answer on every ask: a point reading that excludes 0 implies the plain form is not zero, so no step's outcome depends on which ran first. (sure)
- They differ in cost only. The box read needs the plain form whether or not the point reading refutes, so the point reading can save nothing before it. Put first, it is paid on every identity the plain form proves and every margin the box read decides. Put after, it is paid only where it can still save something: the later rungs. (sure)
- PR 7712's row "a non-zero margin undecided over the box builds NO form" is re-cut to "builds the plain form and no other: no early walk, no reduction, no retry". Its saving on the later rungs stands.

**2. Which form.** The plain form only. It equals the margin at every parameter value, with no condition. The early form (the normal form after the tier has picked `select` arms it certified over the box) is admissible at an exact witness, and would be added only if the count shows a material share of asks that decline on a `select` atom, or that straddle while members agree, which the early form decides with no contradiction of the members.

**3. The decline list.** The box read declines, and the value's enclosure stands, on:
- **Free** (`free_identity`): the value is a measured bound the caller supplies, not the expression's value. The one case where the two reals differ by design.
- **Opaque** (a value with no expression): no bracket to enclose it by; the tier reads no lane value.
- **Hull** (a set-valued join of two values): a set, not a function of the parameters.
- **Frozen** (a form cut off at the budget): an unknown function; cancellation past it is lost.
- **Poisoned form**: the form says of itself that it is not the node's.
- **An atom the enclosure cannot enter**: `select` (in the plain form), trig, anything past the enclosure's depth of 8, a denominator whose enclosure holds 0, a parameter with no finite bracket.
- **Registered identities** (axioms a construction records that two nodes are one function): nothing to decline. The plain form never reads the registry, so no sign rests on one.
- The structural question's closed spellings (the exact axis `(0, ±1)`) keep value and node in step: the literal is both. The posed readings (`difference_dot`) store the same real function with a different rounding, which an enclosure of the real does not see. (likely; read, not run)

**4. An empty intersection.** Agreed: two certified enclosures of one real cannot be disjoint, so it is a kernel bug only a re-derivation detects, and a `debug_assert!` beside the existing `Zero` one. In a release build the value's reading stands (undecided), so nothing is ever certified on a contradiction.

**5. The leaf's claim, in one sentence.** A certified leaf says that at every real parameter point of its box, every decision's margin, taken as the real number its recorded computation denotes, has the leaf's verdict; it says nothing new about a member's rounded `f64` margin, which is as E12's symbolic `Zero` already stands and differs only within rounding of a band edge.

**6. Text for E12.**
- Decision, replacing "Everything else decides numerically as today.": *"Every other margin decides on the intersection of its value's enclosure and its plain normal form's enclosure over the box's brackets; where the form cannot be enclosed, on the value's."*
- Soundness, replacing "the leaf replay's numeric channel is untouched for every non-identity margin, so E6's containment claims are unchanged": *"both are certified enclosures of one real, so their intersection is, and E6's containment claims are unchanged; an empty intersection at an exact witness is a kernel bug, asserted in debug builds."*
- Affine rejection: strike "; plain intervals suffice for the real margins". Nothing replaces it.

**7. The re-spelled readers.** All three stay, each for a reason that is not the box read's:
- **The wall's roots** (PR 7724): still needed in every lane. Its roots are values a construction consumes, not margins, and the unbounded `select` arm is wrong at plain `Interval` too; holding `b` at 0 states a true identity and changed `f64` as well.
- **The side reader's ray**: still needed. The ray is a place the code chose, and DESIGN `[scalars]` says a choice is never built as a boxed value; that was the defect, with the wide readings its symptom.
- **The one-plane reading** (PR 7710): still needed. It makes a true identity provable; the box read would at best call it a measured zero, and proven against measured is what the `unproven-coincidence` lint reads.

**What the count must show.** I keep the recommendation if, among undecided decisions whose sampled members agree, the box read decides a material share (on M14's bodies and the verb-built tilted body, most), with no empty intersection and no decision a member contradicts. I change it as follows:
- Any contradiction of a member, or any empty intersection: stop; the soundness argument has a hole and nothing ships until it is found.
- Declines dominate (hull, opaque, frozen): the mechanism stands, but the unit that matters is giving those atoms an enclosure, and this one alone will not move the tour.
- "Enclosable, straddles, members agree" dominates: the form must be enclosed about the box's centre rather than term by term, in the same unit.
- Building plain forms for the refuted asks gives back more than about a quarter of PR 7712's saving: a cheaper first read on the expression graph (the point reading plus an interval gradient over the box) goes ahead of the form for those asks.
- The verb-built tilted body folds to the zero form and the box read decides little elsewhere: the class is mostly fixtures, and the E12 change is not worth making yet.

## For the orchestrator

- **E12's 7698 sentence needs a touch too.** "an enclosure that excludes 0 there proves the margin is no identity over the box, so the tier is asked nothing and the decision stays undecided" becomes false under the new order: the plain form was already built and the box read already tried. Suggested: "...so the ladder's remaining rungs are not climbed and the decision stays undecided."
- **Item 1's cost claim rests on one assumption I did not measure:** that the plain rung, not a later one, proves most identities. If most identities are proven only at the early rung or the door, the point reading after the plain form is still right (it gates exactly those rungs), but the plain form's cost on the refuted class is then the whole of the new cost. That is count column (e).
- **Opaque and hull as an extension, not now.** An opaque or hull atom is one node, so enclosing it by that node's own value enclosure would be sound; it needs the tier to read a lane value, which `sym::signed` says it never does. It is PR 7712's open difference again, and the count's decline buckets size it.
- **The already-filed rows** (`work/tier/real-margin-dependency-widening`, `work/sym/a-margin-whose-form-is-a-nonzero-constant-escalates-over-a-box`): I did not find them in round 0; agreed they and this item are one, and I did not re-read them for this reply.
- **Item 7 against the other report.** It calls the landed re-spellings special cases whose only reason was dependency. I read each row's stated final state and disagree for the three named; I did not check the other re-spellings listed in the item (`an-aimed-rays-other-readings…`, the radial readers), which should be weighed at their sites on the same test: does it fix a value, state an identity, or apply `[scalars]`? If none, it retires.
- **K outcome.** Agreed with the other report: the box read's `Zero` is not `Symbolic` and not plain numeric; it wants its own tag, and no `as_proven`.
