## For Ev

**Recommendation (likely).** Fix this at the symbolic scalar's decision (`Sym::decide_sign`), once, and not at each reader. When a margin's value reads no definite sign over the box at an exact witness (`Sym<Interval>`), enclose the margin's normal form over the box and decide on the **meet**, meaning the intersection, of that enclosure and the value's enclosure. Both are certified enclosures of one real number, so their intersection is too. The decision rule (the band) stays the same, applied to a tighter interval. In practice:
- a form that is a rounding-size constant inside the zero band decides `Zero`;
- a form that keeps one sign across the box decides that sign;
- a form inside the sliver band (between ε and Kε) is refused at the root as a terminal sliver, not walked down to `Budget`;
- a form that straddles zero bisects as it does today, and bisection now narrows it.

The tier already computes this enclosure: the decision read (DECIDE-3, `sym/signed.rs`) encloses forms over the box brackets to pick a `select`/`min`/`max` arm. It is never used for the decision itself. E12's "Everything else decides numerically as today" is the sentence to change.

Terms used below:
- **Value channel**: the interval the replay computes, which counts each stored copy of a parameter as an independent number.
- **Expression / form**: the hash-consed DAG beside each value, and its polynomial normal form with exact rational coefficients, where each parameter is one symbol.
- **Zero band / escalate**: the interval decision's `|m| ≤ ε` reads `Zero`; `m ≥ Kε` (or `≤ −Kε`) reads a sign; in between is a sliver.
- **Exact witness**: a lane whose comparisons are certified signs of the real value (`Sym<Interval>`), not rounded readings (`f64`).

**Premise check.**
1. **This class is not new** (sure). `work/tier/real-margin-dependency-widening` (P2, filed 2026-09-04, the day after E12) names it: dependency widening of margins that are not identities. It calls it "the next ceiling" and asks for this same count. `work/sym/a-margin-whose-form-is-a-nonzero-constant-escalates-over-a-box` (P3) is the constant case. Neither row's candidate fixes includes reading the form the tier already holds. The item should be merged with them, not counted separately.
2. **The defect is not at the readers** (likely). E12 rejected per-site re-spelling for identities because "the sites receive VALUES, and the dependence was lost upstream". The argument holds just as well for definite margins. The three re-spellings this program has done (the side reader's ray, a wall's roots, the one-plane reading) are that rejected remedy, applied one unit at a time.
3. **The body's construction is a separate, smaller defect** (likely). The cap vertex lies on its plane only to rounding, because the plane is stored as an `f64`-normalized normal plus an origin taken from another vertex. That is a true identity that is not stored as one. It is already filed (`a-point-built-on-a-frame-of-normalized-directions-…`). Fixing it turns one constant into a proven zero. It cannot help `curve_carrier_gap` (form `0.912·|top|`), `pm_census_ee_parallel` (constant 0.41) or `pierce_lever` (zero outside the box). Those margins are really non-zero, and only reading the form decides them.

**What each kind is owed over a box.** The leaf protocol (E6) needs every member of the box to take the leaf's decision. Any certified enclosure of the margin's real value gives that, the form's included.
- **A rounding-size constant that is not an identity** (6.58e-19). It reads `Zero` in the band, which is what the `f64` build reads at every member. It is a measured zero and must not be marked proven: no `as_proven`, and it gets its own K tag so E12's evidence ratio stays honest.
- **A definite function** (`0.912·|top|` on [0.75, 1.25], enclosure [0.68, 1.14]). It reads `Positive`.
- **A coincidence that holds across the box.** This is the constant case if it lies in the band, and a terminal sliver if it lies in (ε, Kε). The sliver case is the P3 row's honest verdict, given at the root.
- **A form that straddles zero over the box.** This is a real flip or a near-flip, and the driver refines toward it as E6 intends.

**Final state, and what it makes true.**
- A leaf's decision no longer depends on how many stored copies a parameter passed through.
- The reader spellings stop mattering on the symbolic lane.
- Soundness is the obligation E12 already carries for `Zero`: the form's real value equals the construction's real value. The read therefore **declines**, and the value's enclosure stands, on anything that breaks that equality:
  - a `Free` node (`free_identity`, the one the existing assert already exempts),
  - an opaque value, a frozen node, a poisoned form,
  - registered identities (the door is an axiom store; a sign must not rest on an axiom),
  - any atom the enclosure cannot enter. Today that means trig; adding sound interval trig is a later extension.
- It runs only at exact witnesses, as the decision read does: at `f64` the construction compares rounded values.
- It is deterministic (D9): a function of the form and the box's `f64` brackets, and the brackets are already the only values the tier reads.
- **The check comes free.** At an exact witness, an empty meet (the value's and the form's enclosures disjoint) proves a kernel bug (DESIGN [failure.taxonomy] row 5). It becomes a `debug_assert!` next to the existing `Zero` one, which gives `a-retrys-zero-has-no-cross-check…`'s class a check at every undecided decision.
- **Left possible:** margins whose forms freeze or reach an opaque value still widen. `bool_contact_vertex`'s `dz²` freezes on the degree budget. It disappears here only because it is downstream of the residual. The count says how large that remainder is.
- **Reversible:** one switch in `SymRules` gives the differential with the read off, as every other rule has.

**Alternatives, as final states.**
- **(B) Keep re-spelling readers** (today). One unit per reader, it never closes, it cannot reach readers that build through ratios (`pierce_span`), and it is the remedy E12 rejected. No.
- **(C) Fix constructions only.** This is right for true identities, as a matter of what is true, but it does nothing for definite margins. Do it on its own merits (P2/P3), not as the cure.
- **(D) An affine or centred value channel.** That would be a second representation of what the form already holds exactly: one thing represented twice. No.
- **Interaction with PR 7712's point reading** (refute an identity before the ladder). Compatible. The order becomes: value; plain form; zero form gives `Zero`; the box read decides if it can; the point reading refutes; the ladder. The box read takes the definite and in-band margins away before 7712's refuter ever sees them.

**Ratified text to change** (provenance: E12 is agent-written text Ev approved as "lgtm" on #1712. Ev's own words are "make this parameter-aware so it's usable". The shallow checkout cannot reach the commit, and #1712 shows no comments or reviews through the API). Proposed, short:
- In **Decision**, replace "Everything else decides numerically as today." with: "Every other margin decides on the meet of its value's enclosure and its form's enclosure over the box; where the form cannot be enclosed, on the value's."
- In **Soundness**, replace "the leaf replay's numeric channel is untouched for every non-identity margin, so E6's containment claims are unchanged" with: "both are certified enclosures of one real, so their meet is; at an exact witness an empty meet is a kernel bug, asserted."
- In the affine rejection, strike "plain intervals suffice for the real margins". It has been measured false since 2026-09-04.

**What a count must show for this to be right, and how to take it.** At every undecided decision, at `Sym<Interval>` over boxes of width, classify the form's enclosure as: zero form / definite / in band / sliver / straddles / declines (and why). Populations:
- M11's tour drive with the box arm off;
- M13's 11,028 builds at their study boxes;
- the M14/M15 bricks.

Then take the differential, verdicts and leaf counts with the read on and off. It is right if all of these hold:
- (i) definite + in-band + sliver is a material share of undecided decisions and of refused mass;
- (ii) leaves flip from refused to certified, with zero empty-meet asserts;
- (iii) `f64` members sampled in each newly certified leaf take the read's decision at every member;
- (iv) "declines" is small. If it is large, the next wall is the freeze/opaque budget, not this.

## For the orchestrator

- **Brief error:** "nobody has counted" is true, but the count is the one `work/tier/real-margin-dependency-widening` § "The next thing to measure" already asks for. Fold the item, that row and the P3 constant row into one, or link them.
- **Assumed, not checked:**
  - That nothing besides `Free`, opaque, frozen and the door makes the value channel differ from the form's real. Check whether closed spellings (the structural question) and hull nodes keep the value and the node in step.
  - That the early form, with decision-read arms, is admissible at the top decision under the same exact-witness argument as `signed.rs`'s module doc. I believe so, since the arms are certified over the box.
- **`signed::enclose_indet`** enters only `sqrt`/`abs`/`min`/`max`. M15's forms need only `sqrt` and `abs`. The tour's `curve_carrier_clear`/`curve_face_pose_disc` may need trig; the count's "declines: trig" bucket says so.
- **The read-Zero must not use `margin.as_proven()`**, and it needs a K outcome of its own (not `Symbolic`, not plain numeric). Otherwise E12's symbolic/numeric ratio is mislabeled.
- **Landed re-spellings** (PR 7724's pose, the side reader's ray, the one-plane reading): after this lands, each is a special case whose only reason was dependency on the symbolic lane. Keep those the plain `Interval` lane needs, if that lane certifies anything shipped. I could not settle whether it does (unsure); weigh each at its site.
- **Cost** is not weighed (out of brief). The enclosure is one polynomial pass over a form the plain rung already builds. The DECIDE-3 cost row (`decision-read-triples-the-plate-pin-suites-wall-time`) is the precedent to watch.
- **PR 7712's two-designer difference** (whether an opaque may be read at a point of its own enclosure) recurs here as the "declines" bucket. The same count settles both.
