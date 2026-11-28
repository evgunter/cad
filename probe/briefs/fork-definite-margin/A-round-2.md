Round 2 reply: fork-definite-margin

## For Ev

**Recommendation now (likely).** One unit, not two. The decision reads its form's enclosure over the box (as agreed), and in the same unit that enclosure learns to enter the atoms the drive declines on:
- trig functions, by the argument's enclosure and the interval type's own certified function;
- `Select`, by its condition's certified arm, else the hull of both arms;
- a hull, by the hull of its two children's enclosures;
- deep forms, with a per-node memo in place of the depth cap.

Opaque values stay declined. The leaf's claim is about the margin's real value, with no extra hold-back rule; why that is safe is item 1. What moved:
- The read is worth making only together with the atoms. On the tour's drive it decides 0 of 37,294 undecided asks, and every decline is an atom kind, not the idea (sure, from M16).
- "Plain form only" becomes "the form with the condition-certified `select` arms". 7,897 declines are at `Select`, the condition I set in round 1.
- Hull moves from the decline list to entered. Entering it reads no lane value, so round 1's reason for setting it aside does not apply.

**1. Members that contradict the reading.** This is the case I excepted, not a stop (likely).
- `pm_census_ef_residual_side` is a `prove`, read at the least band, whose zero and escalation are the two least positive `f64`s. The band's own contract is "a margin reads its sign alone, as an exact comparison would".
- The exact margin is +6.58e-19 at every member, so `Positive` is the exact comparison's answer. A member's `f64` reading (±5.55e-17) is rounding noise 80 times the margin. At the least band the band edge is 0, so "within rounding of a band edge" means "smaller than the rounding".
- No intersection was empty, and no decision at a band of real width differed.

What the box decision should be: the sign of the real margin. No box scalar can match a member's rounding: it varies member to member, and today's value channel can only straddle it and walk the leaf to `Budget`.

What a certified leaf then claims, one sentence for E6: "Every decision of a certified leaf holds of its margin's real value at every member, and the leaf takes the decisions the witness took." The second half is what protects a member's build. A decision in the verdict vector that the `f64` witness reads the other way by rounding gives a different verdict vector, so the leaf is refused `FlipCrossing`: a refusal, never a wrong certificate. Here the prove is not on the witness's path at all, because at the nominal the residual reads `Zero` in the band and the prove is never asked.

Where to hold back, as a rule: nowhere, for soundness. If Ev wants member fidelity on top, the one rule that needs only quantities already computed: where the read signs a margin at a band whose escalation is below the rounding of that margin's own DAG at one point of the box, the decision is undecided and marked terminal. That rounding is PR 7712's point reading: the DAG enclosed at one member in outward-rounded `Interval`, which contains that member's `f64` value. "Terminal" means no width decides it, so the driver refuses at the root rather than at its depth budget. It rests on one sampled member, not on all of them. I lean against it, because it refuses what is true of the reals to match noise.

**2. The tour.**
- **Does the recommendation stand?** Both in one unit (above). The round-1 text already covers an enclosure that enters more atoms, since it names the enclosure, not what the enclosure enters. It must drop "plain", because a `select` arm is the early form's.

Each declining kind, whether the enclosure may enter it, and what that rests on:
- **Trig** (`Sin`/`Cos`/`Atan`/`Atan2`): yes. Enclose the argument's form, then apply the always-compiled interval's certified function. It rests only on the transcendental enclosures the value channel already rests on. `Atan2` across its cut gives [−π, π], which is sound and simply loose.
- **`Select`**: yes. Where the condition's form is certified signed over the box, take that arm; this is the decision read, at an exact witness, which is the only place this runs. Otherwise take the hull of both arms' enclosures, which is sound at any witness. It rests on nothing new.
- **Hull**: yes, as the hull of its two children's enclosures. The construction's value lies between the two, so their hull contains it. It reads no lane value. Today a hull enters the form as an indeterminate keyed by its children's ids with no atom record. Likely (inferred, not traced) that is M16's 15,200 "no bracket, no atom record": 13,152 of the graphs hold a hull. So the unit needs a hull to carry its children, as other atoms already carry their arguments.
- **Depth** (192): yes. Memoize one enclosure per form node and drop the cap. No soundness question.
- **Opaque**: stays declined. M16 finds no opaque in the tour's graphs, so reading the lane value would buy nothing here, and the tier keeps reading only the box's brackets.

Is the class mostly fixtures, as one round-1 condition put it? Not yet shown. The verb-built body's residual does fold to the zero form, but over a box that body is refused at margins with `Sin`/`Cos` atoms (`bool_join_nearest`), and the tour's top predicates decline the same way. The condition "decides little elsewhere" is a statement about today's enclosure's reach, which the further count tests.

**What the further count must show for the extended unit to be right:**
- (i) a material share of the 37,294 asks decide;
- (ii) the tour driven with the read on certifies leaves it refuses today. This is the program's own test; asks decided without verdicts moving prove nothing;
- (iii) no empty intersection, and no member contradiction outside the sub-rounding least-band class of item 1;
- (iv) cost per ask stays in µs against the ladder's 28 ms.

**What would show the next wall is somewhere else:**
- asks decide but leaves still refuse, at a refusal that is not a margin: a value that is not a decision, too wide to use (a span location, a `Size` mint over a straddle, a normalize whose length straddles 0);
- most undecided asks are `Invalid` margins (a domain violation), which `decide_sign` keeps from the tier entirely (its `domain_violation` arm). The count should report those as their own column;
- the asks enter every atom and still straddle while members agree: the form wants a centred enclosure;
- the 812 `assert_bound` straddles: if members disagree there, those are real flips, which is correct and leaves nothing to do.

**3. The re-spelled readers.**
- **PR 7724 (the wall's roots):** it continues. I take the other reply's reading over my round-1 "per-site check": its roots are values a construction consumes, its unbounded `select` arm is wrong at plain `Interval`, and it changed `f64` too. None of that is dependency.
- **Dispatch rule until the extended count is in:** hold any site re-spelling whose only stated reason is "a margin reads its parameter through two stored values". Continue site work that fixes a value, makes a true identity provable, or applies DESIGN `[scalars]`.
- **After the count:** if the extended read decides `curve_carrier_clear`, `azimuth_near_cut_side`, `period_near_mark`, `curve_carrier_gap` and `curve_face_pose_disc`, their open re-spelling rows close as covered, unless one has one of the three reasons above.

**E12 text** (changed from round 1 only in "plain"):
- Decision, replacing "Everything else decides numerically as today.": "Every other margin decides on the intersection of its value's enclosure and its form's enclosure over the box, where the form can be enclosed; a zero read so is measured, not proven."
- Soundness: replacing "the leaf replay's numeric channel is untouched for every non-identity margin, so E6's containment claims are unchanged" with: "every other margin is decided on certified enclosures of one real, so E6's containment claims hold of that real at every member of the leaf."
- Affine rejection: strike "; plain intervals suffice for the real margins".

**Confidence.** Recommendation: likely. Item 1 (sub-rounding at the least band, safe through the witness match): likely. Each atom's soundness: sure for trig, select and hull, as argued. The 15,200 being hulls: unsure, inferred. Item 3: likely.

## For the orchestrator

- **The hull indeterminate**: on main it is `Poly::indet(indet_hull(kid0, kid1))` with no `mint_atom` record (`sym.rs`, the form's `Hull` arm), so `enclose_indet` finds neither a bracket nor an atom. Check that this is the 15,200 before the extended count closes. If it is, "Hull by its children" needs the hull minted as an atom carrying both children's forms. That changes the atom's key, so check the content-hash/determinism consequences.
- **`signed.rs`'s module doc** ("the one door a value comes through"; enclosure "only in the EARLY memo") becomes stale once the decision itself reads the enclosure. It is code text, so it lands with the unit.
- **PR 7712's sentence** in E12 ("…so the tier is asked nothing and the decision stays undecided"): as the other reply said in round 1, it becomes "…so the ladder's remaining rungs are not climbed". It lands with whichever of the two PRs merges second.
- **M16's `false_premise` pin** going red (`Escalated` → `OffSurface`) is the change working, so re-cut it, do not repair it. That also closes `work/sym/a-margin-whose-form-is-a-nonzero-constant-escalates-over-a-box`, except its nominal-only rim, which still straddles, correctly.
