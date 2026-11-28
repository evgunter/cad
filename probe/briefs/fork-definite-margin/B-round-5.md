# Round 5 reply: the tour does not move

## For Ev

**Where I land (likely on the diagnosis, unsure on the cure until it is counted).** The round-4 mechanism is right for the bodies it certifies and is not what the tour needs. The principle underneath it stands and should be what E12 says: a decision may read any certified enclosure of its margin's real value taken from the expression. The form's enclosure is one such reading, good for margins that cancel to nothing or nearly nothing. The tour's margins need a second one, centred on a point of the box, and that one touches a decision you made in your own words (DUAL-DESIGN DL1: "a Dual never certifies"). I would put nothing to you until the count of the centred reading is in, and then one PR, not two.

**1. The defect, restated.**
- **What the tour's margins are.** Each is a definite function of the box whose true range is about a hundredth of its enclosure, under every evaluation the kernel has. `curve_carrier_clear` is 3.87e-3 to 3.93e-3 at every member sampled and encloses [0, 1.39e-2].
- **Why.** Evaluating term by term in intervals over-states a range in proportion to the box's width whenever a parameter occurs many times. The value does it op by op. The form does it term by term, and an expanded polynomial has more occurrences of each parameter than the computation it came from, not fewer. Rounds 1 to 4 described the special case where the occurrences cancel outright; there the form wins. Here nothing cancels outright.
- **Where it starts.** At how a decision's range is enclosed, which is still "the reading of a decision", but not at the form. A normal form is the right object for proving zero and the wrong one for enclosing a range: it is expanded, it grows until the budget cuts it off (the frozen nodes), and opening the frozen nodes costs 39 ms an ask to recover the value's own enclosure. A form that needs freezing should not be enclosed at all.
- **The readers.** The re-spellings worked (PR 7724: 1,496 undecided to 212) because each removes repeated occurrences by hand. That is effective and per-site. It is not where the defect starts, but it is the only thing measured to move the tour.

**2. Does the round-4 agreement stand, change or shrink?** It shrinks as a mechanism and its sentence becomes more general.
- **What stays.** The form's enclosure with trig and `select` entered, at the shipped depth: about 19 to 46 µs an ask. It builds the verb-made tilted body at ± 0.05 (refused at every width today), takes the extrude door's slab from four refused wide boxes to one, and clears the fixtures. Opening frozen nodes is dropped: 39 ms an ask and no leaf moves.
- **The centred reading, as a design.**
  - *What is evaluated:* the expression graph, not the form and not the stored value.
  - *How:* the margin at one member c of the box, plus, for each parameter, its derivative enclosed over the whole box times that parameter's distance from c. The margin at c is PR 7712's point reading, already built for another purpose. The decision is the band test on the intersection with the value's enclosure (and the form's, where taken).
  - *What soundness rests on:* the mean value theorem, with the derivative enclosed over the box in outward-rounded intervals. At a kink (`abs`, `min`, `max`) the derivative's enclosure is the hull of both sides, which is the generalised-gradient form of the theorem. A `select` certified over the box takes its arm; one that straddles, and a hull, take the hull of each arm's own centred enclosure, which needs no continuity.
  - *Where it declines:* a root whose argument reaches zero, a quotient whose denominator holds zero, `atan2` across its cut, a value with no expression.
  - *What it cannot do:* its excess shrinks with the square of the box's width. That is ample for a margin with room to spare and useless for showing a margin is inside the band, which is why E12 rejected its relatives for identities ("a ceiling at about √ε"). So it does not replace the form.
  - *One mechanism or two:* two readings, one rule. The form decides zeros and near-zero constants; the centred reading decides signs. The rule is the claim they share.
  - *Cost and order:* not measured. My estimate is two to three point-reading passes, 1 to 3 ms an ask, against the ladder's 29 ms. Order: value; plain form (zero gives `Zero`; its cheap enclosure); point reading; centred reading built on it; the ladder's later rungs only where the point reading held zero.
- **Ratified text it touches, and its provenance.** DL1 says "A Dual never certifies — permanently", closing your 2026-08-19 ruling "a Dual may not certify — at least for now", and adds that "a future need would be a new design conversation on its own evidence". The centred reading does not make the `Dual` scalar certify; the symbolic tier walks its own graph. But it does use a derivative's enclosure to certify a value, which is the thing DL1 was about, and I would not slide it past that sentence. M16b and the count under way are the evidence DL1 asks for. E6's "duals as pruning only" needs the same look.

**3. The site work.** Release it, sorted by what the count showed for each predicate, not as one class.
- `curve_carrier_clear`: members agree, pure dependency. Owed the centred reading; hold its re-spelling until that count.
- `curve_face_pose_disc`: the same, and PR 7724 already took most of it. Its remainder is the ray's start, which the fixed-ray unit owes regardless.
- `period_near_mark`: 70 of 240 are dependency (hold, as above). The other 170 pass through a hull: a tie between two values, wide before any margin is formed. That is a value, not a decision, and is site work now.
- `azimuth_near_cut_side`, `curve_carrier_gap`: members could not be evaluated (a hull or a measured bound in the graph), so the count determines nothing for them. Each is owed a trace of what that value is, then site work on it. Release.
- Anything reading an angle across `atan2`'s cut (2,942 declines) is owed a re-posing away from the cut at its site. No reading fixes that.

**4. What the further count must show.**
- *Asks:* the centred reading decides most of the 22,812 that straddle while members agree, with no empty intersection and no member contradicting.
- *Leaves:* more than 211 certified, or materially fewer than 511 splits. This is the only test that counts; M16b showed asks can be decided with no leaf moving.
- *Cost:* a few milliseconds an ask, and total leaf time no higher than today's 1,546 s.
- *If asks decide and the tour still does not move:* I would conclude that the reading of a decision is not the tour's wall, and stop designing general mechanisms against the tour. The next measurement is then one row per refused leaf: its first undecided node and why (a value with no expression, a refusal that is not a margin, an angle across its cut, or a margin that really crosses, like the 67 `assert_bound`s). Some of the 301 refused leaves may be refused correctly, and nobody has yet said how many should certify.
- In that case I would still put the form reading to you on its own, as a smaller change, for the bodies it does build; and release all site work.

**E12 text, if the centred reading earns its place:** "Every other margin decides on the intersection of its value's enclosure and a certified enclosure of its real value over the box taken from its expression; where none can be taken, on the value's." If it does not: round 4's sentence.

## For the orchestrator

- **This was foreseeable and I under-weighted it.** My first report named the risk (an expanded form encloses worse than the factored original) and the fallback (the point reading plus an interval gradient on the graph), then treated it as a cost fallback and not as the likely main mechanism. My round-1 condition "enclosable, straddles, members agree: enclose about the centre" is the one M16b met.
- **For the count now running:** have it report the derivative enclosures' widths beside the result. If a derivative's enclosure is itself as loose as the value (a small true slope from two large terms, each as wide as the box), the centred reading fails for a nameable reason and a second-order form will not be worth trying.
- **Reverse or forward.** One reverse sweep gives every parameter's derivative; with one or two box parameters per leaf a forward sweep per parameter is as cheap and simpler. The implementer's choice, on the count.
- **The tier keeps no node values**, so the derivative pass needs each node's enclosure over the box: one forward pass over the brackets, remembered per leaf. M16b's side-table alternative (180 of 1,465 decided) suggests the stored lane values add little; I would not add a store for them.
- **`Free` in a graph** means the margin is a measured bound (`free_identity`). No reading from the expression applies; those asks should be counted out of the denominator.
- **Not checked:** the generalised-gradient argument at each kink the tier has (I argued it for `abs`/`min`/`max`; `copysign`, `signum` and the period readers need their own line); my cost estimate; whether DL1's author would read the centred reading as covered. The last is the owner's.
