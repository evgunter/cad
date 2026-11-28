# Round 2 reply: a margin definite over a box

## For Ev

**Recommendation now (likely).** The same decision at the same place, but as one unit of three parts, because the count shows the first part alone does nothing on the tour:
1. an undecided decision at the symbolic box scalar reads the intersection of its value's enclosure and its normal form's enclosure over the box (as agreed in round 1);
2. that enclosure enters every atom it soundly can: trig, `select` and hull as well as `sqrt`/`abs`/`min`/`max`, with no depth limit;
3. a new hold-back: the form's answer stands only where the same margin, read at one member with its rounding, gives the same answer.

What moved: part 3 is new (item 1); part 2 moved from "later extension" into the unit (item 2); the cost worry is gone (12 to 62 µs an ask against the ladder's 28 ms). Not ratifying the E12 sentence until the extended count is in is the right sequencing.

**1. The 17 members that contradict.**
- **Which it is.** It is the gap round 1 named (a margin within rounding of a band edge), not a hole in the argument that the form and the construction denote one real. The evidence: the form is an exact constant, no intersection is empty anywhere, and the value's enclosure at each member holds both the 6.58e-19 and the ± 5.55e-17. My round-1 stop condition ("any contradiction of a member") was worded too bluntly and fired on it; its other half (an empty intersection) did not. (likely)
- **But the gap is not rare where round 1 implied it was.** These asks are at the *least band*: the band an exact comparison uses, whose edge is zero itself. There every margin of rounding size is within rounding of the edge. The reading says `Positive`, which is true of the real number and is not what a member's `f64` arithmetic computes.
- **What the box scalar's decision should be.** Undecided, as today: the sign is finer than a member's arithmetic resolves, and the plain interval lane at a box of no width already says so (it straddles).
- **The rule, on enclosures and bands.** Let F be the form's enclosure over the box, V the value's, and P the *point reading*: the same expression enclosed op by op, outward-rounded, with each parameter at one member (PR 7712 builds it; its width is that member's rounding). The answer `band(F ∩ V)` stands only if `band(P)` is the same answer; otherwise `band(V)` stands. Worked: least band, F = 6.58e-19, P = [−2.2e-16, 3.6e-16] straddles, held back. ε band, same margin: P is inside ± ε, `Zero` (measured) stands. A margin of 0.41 at any band: P agrees, the sign stands. It only ever holds back, so it cannot make the reading less sound. (likely)
- **What a certified leaf claims.** At every member, each decision's margin, as the real number its computation denotes, has the leaf's verdict; and at the leaf's sampled member the rounded computation reads the same verdict. Beyond that member, a member's `f64` build agrees to the extent DESIGN `[simple.conditioned]` already assumes of every `f64` build: that rounding does not decide.
- **In the real flow these 17 asks do not occur** (inferred from `census::undecided_signs`, not run): the side is asked only where the residual's own band read is undecided, and with the reading on that residual reads `Zero` first. The rule is still owed for any exact comparison.

**2. The tour.**
- **Stands, or a different unit?** Both in one. The count's own words are right: the atom kinds are the limit, not the idea. It decides 315 of 342 on the fixtures and clears the extrude door's slab (a kernel-built body) at wide boxes.
- **My round-1 condition** ("the verb-built body folds to the zero form and the reading decides little elsewhere: not worth making yet") is half met: the Boolean-built body does fold, and the tour gets nothing. So the E12 change is worth making only together with the atoms.
- **Each declining kind:**
  - *Trig (`sin`, `cos`, `atan`, `atan2`)*: enclose the argument's form, apply the interval lane's certified function. Sound. It rests on nothing new: the value channel already rests on those functions. Loose where `sin` and `cos` of one angle are enclosed separately.
  - *`Select`*: enclose its condition. A certified sign takes that arm (this is the existing decision read); a straddle takes the hull of both arms' enclosures, since the construction takes one of them at every member. Sound at an exact witness, on the argument `sym::signed` already makes. This replaces "the early form" as a separate question.
  - *Hull* (a join of two values at a tie): the hull of its children's enclosures. Sound, because the value it stands for lies between its children at every member. Loose by the width of the tie.
  - *Depth*: no soundness content; remember each atom's enclosure for the leaf and drop the limit.
  - *Opaque, free, frozen*: still decline. The tour's graphs hold none, so the reason round 1 gave (the tier reads no lane value) costs nothing here and I would keep it. All four entries above read only the form and the brackets.
- **Does the round-1 text cover it?** Nearly. "Plain" must go (a `select` entered by its condition is no longer the plain form), and the text should say atoms are entered. Sentences below.
- **What the extended count must show.** Right if: (a) a material share of the tour's 36,482 declines now decide, with the item-1 rule on and no member contradicting; and (b) leaves change: fewer than 511 splits, or refused leaves certifying. The next wall is elsewhere if:
  - asks decide but each leaf's first undecided node is unchanged, or the body is refused before any margin (the Boolean body's `bool_join_nearest`): a refusal upstream;
  - most asks become "enclosable, straddles, members agree" and their forms hold hulls or roots as wide as the value: the looseness is in a quantity that is not a decision (a ray, a root, a chart point), which no reading at the decision can tighten;
  - most straddle and members disagree: those are real flips, and the drive is at its geometry.
  - The 15,200 untraced are probably the hulls (13,152 graphs hold one, and a hull has no bracket and no atom record); if they are something else, that kind needs its own line before anything is built. (unsure)

**3. The re-spelled readers.** Sort site work by what it fixes:
- *A value a construction consumes* (PR 7724's roots, which read `[-inf, inf]` and feed crossing points): continue and land. No reading at a decision tightens a value, and an unbounded root inside a hull encloses unbounded under the new reading too.
- *An identity, or a place the code chose* (PR 7724's `b = 0`, the one-plane reading, the fixed ray): continue and land.
- *A non-zero margin re-spelled only so that a parameter enters once*: stop dispatching these until the extended count is in. If it decides the tour's large margins, this kind of unit is over.

**E12 text, as it changes.**
- Decision: *"Every other margin decides on the intersection of its value's enclosure and its normal form's enclosure over the box's brackets, each atom enclosed from its own arguments; where the form cannot be enclosed, or the margin read at one member with its rounding answers differently, on the value's."*
- Soundness and the affine rejection: as agreed in round 1.

## For the orchestrator

- **Why P and not a constant.** I looked for a rounding bound already in the kernel: the admission floor (`geom-core` README, "Float rounding") is the one statement, but it is a length, "no site charges it", and scaled to a band's headroom it vanishes at the least band, which is the case that needs it. P is dimension-free and is this margin's own rounding. It is one member, not a bound over the box; I say so in the claim.
- **P declining.** PR 7712's reading declines on hull, opaque and free. With hulls entered by the form's enclosure, P must enter them the same way (hull of children at the point), or every hull-carrying ask is held back. Ask the extended count to report "held back because P declined" as its own column.
- **Order, restated.** Value; plain form, zero gives `Zero`; box read; P (confirms a box-read answer, and where the box read gave none, gates the later rungs as in 7712); remaining rungs. P is now paid on decided asks too: 0.2 to 1 ms warm by 7712's numbers, against asks that otherwise cost 28 ms.
- **A pre-existing twin, not mine to settle.** A proven symbolic `Zero` at the least band has the same gap today: the tier says `Zero` where a member's `f64` reads a rounding-size sign. Worth a row if none exists.
- **Not checked.** That `Hull`'s value lies between its children at every member (I read the interval hull's doc, not every mint site); that the interval backend has certified `atan`/`atan2` with the branch handling the tier's atoms assume; that the 17 asks vanish with the reading on.
- **The 812 that straddle are all `assert_bound`**: worth one look at whether those are bounds on non-decision values (the second "next wall" bullet) before the extended count is read.
- **M16's caveat matters for item 1.** Its "members" are the expression evaluated in `f64`, not decisions an `f64` member build took. A stronger sample would replay the body at `f64` members and compare decision logs; that is what E6's "takes the witness's decisions" is about.
