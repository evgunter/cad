## For Ev

**Where this stands.** The coordinator's list of agreements is right, with one factual correction: `chart_region` is not the only production band built from the band's own thresholds (item 4). Of the five differences, I concede two (the README clause and what else is owed), two turn out not to be differences (the join slack and `resolved_as`), and one remains a split: the count. My lean there is the existing integer rule. Nothing was built or measured.

**1. `fate::tip_wraps`, the count: I hold. Split, lean `Period::decide`. (likely)**
- *What I checked.* `Period::decide(name, gap, meter, band)` returns the whole number `k` with `|gap − k·period| < period/2`. It never rounds. It decides signs at the half-period marks: the two either side of zero, then outward (1, 2, 4, … and a bisection). Each mark goes through the ordinary `decide` funnel under the caller's name.
- *Does it fit as it is?* Yes. The period is a half-turn, the gap is `whole turn − turned`, and the meter is `|g| Margin::levered(g, lever)`. `topo::pcurves::decision` already calls it in exactly this shape (`"pcurve_singular_turn"`, an angle gap levered at a lever).
- *What its refusals map to.* `BranchMiss::Undecided(cause)` is a mark read in band: the corner escalation `tip_wraps` already returns. `BranchMiss::OnMark` is a gap half-way between two counts: `TipUnread`. A decided `k` equal to minus the corner count is a tip, plus the count is a spur, anything else is `TipUnread`. The corner count becomes an integer; today it is summed as a scalar.
- *How the other lanes read it.* Each mark is a normal sign decision, so no lane needs a new rule. At `Interval`, an enclosure wholly past a mark decides and one astride it escalates. At `Sym`, the decision is the witness scalar's unless the form is identically zero, as for every other decision in this function. A dual decides on its value.
- *Is my earlier argument answered?* No. The other report did not check how a rounding door reads on `Sym` and `Interval`, and offered a second form for that reason. Either form adds a door, plus a fixed fraction or a new constant band. `period.rs` already rules on this: "No whole-period integer is read off a value". Its reason: the floor of an enclosure near a half-integer is a whole unit wide.
- *What the other report has right.* A count has no length, and my meter gives the turning sum a lever. That lever is a choice: the least of the corners' reaches, read from below. I think it is the honest choice. The sum is off an integer only because each corner counted as a half-turn was decided Zero at its own reach, so the guard should scale with those reaches. A fixed fraction does not.
- *If you prefer no lever at all,* the smaller answer is still on `Period`: the same marks read at the least band (sign only), as `Period::near` does internally. I would take that over a rounding door.

**2. `join::off_conic_slack`: same destination; the difference is sequencing.**
- *What I checked.* `decide_across(names, datum, swing, band)` reads Zero where `|datum| + swing` is within ε. It reads a definite sign where the datum shrunk by the swing is past K·ε. It escalates otherwise. `travel` computes the same definite test by hand. For a tie it decides the reading and the slack separately, which `decide_across`'s doc warns against (two terms each just inside the band sum to nearly twice it).
- *The other report's fallback is answered.* It called `Band::padded` the weaker answer because the pad is not an `f64`. `decide_across` takes a scalar swing, so nothing needs to become an `f64`.
- *The other report's claim holds.* The slack's tie check is `2·cot ψ ≤ 1` with the tolerance cancelling; the code's own doc says "from about `cot ψ = 0.5` on".
- *Where I stand.* Reading each site on the conic is the right final state; I named it in my first report as a separate row. I do not put it in this unit, for three reasons. `Turn` holds an axis, a point and a direction, with no conic and no parameter. Neither report verified that a parameter is in hand where a `Turn` is made. And it changes a record, which is your condition from PR 7141 and which this unit's row excludes. So: `decide_across` now, and a row for the read on the conic, which then deletes the slack.

**3. The README clause: I concede to the two sentences.** My other two sentences are not needed. "A length scaled by a pure number is still that length" is already said by "A dimensionless factor belongs to the reading". The swing sentence would describe a state item 2 intends to delete. The two sentences are also neutral on item 1.

**4. What else is owed: I concede, and both proposals stand together.**
- *What moved me.* The brief said `chart_region` is the one production band built from the band's thresholds. It is not. `topo::census::span_clear_as` builds `Band::new(escalate − zero, escalate)`. `step_import::interpret::file_band` builds `Band::new(e, k·e)` at a file's ε: the hidden `Band::linear_at`, written by hand in production. So hand-built bands are already a route for a site's own number, as the other report says.
- *Where I stand.* The two new bands get named constructors in geom-core: the offered band, and the band one K-step finer. Production references to `Band::new` are pinned with reasons, as the lever doors are. That is a handful of sites; my rough count found four or five once test modules are excluded.
- *What I keep.* The from-f64 gate already lists the sites that lift a band threshold into a scalar (27 rows, 4 of them ours), and the lever gate's "what review holds" line should name that shape. The other report's grep for `Margin::levered(..).value()` used as multiplication is worth doing too.

**5. `validate::resolved_as`: no difference in final state.** It is both things. It is a rounding charge the README forbids, which goes when the re-posing row lands. Its threshold is `ε/K` and not the headroom because `ε/K` is the tightest tolerance an in-band refusal offers; that row says as much. The interim is the same in both reports: `Margin::of(ρ)` in the band `(ε/K, ε)`, through the named constructor of item 4. I withdraw my suggestion to read `ρ` in the particular refusal's offered band at `chord_join`. It moves verdicts on a site that is going away.

**The README clause I sign** (added to the lever bullet of "Float rounding", through an `[ev]` PR):

> A count is an integer and is read as one. A reading asked at a tolerance other than the run's (the one a refusal offers) is decided in that tolerance's band, never levered by it.

**Final state per site.**
- `dihedral::wedge_reads_zero_at_the_arm`: the wedge `Margin::levered(sin θ, arm)` at its real arm, decided in the arm refusal's `offered_band()`. Zero means the arm binds; `None` means no offer.
- `dihedral::wedge_reads_zero_at_the_offer`: the same wedge in the pairing refusal's `offered_band()`. The `sin θ / |cos θ|` quotient goes.
- `fate::tip_wraps`: `Period::new(half turn).decide(..)` on `whole − turned`, metered at the corners' least reach from below, with `k` compared to ± the corner count as integers. (The split of item 1.)
- `join::off_conic_slack`: the swing of `decide_across` inside `travel`. `bool_join_arc_slack` goes. A row is filed to read each site on the conic.
- `intersect` `pn_apex_tangent_reach` and `pn_apex_point_reach`: `Margin::of` of one named function for the apex start, `(|δ| + rounding)/sine`.
- `intersect` `pair_bound` far, near, offset: plain arithmetic. Far and near call that same function.
- `validate::resolved_as`: `Margin::of(ρ)` in the named finer band `(ε/K, ε)`, until the re-posing row deletes it.

**Confidence.** Items 2 to 5: likely. Item 1: likely on fit, with the lever's fold the one judgement in it.

## For the orchestrator

- **Checked this round** (worktree `/root/.local/share/cad-work/designer-dd-b`, read-only):
  - `crates/geom-core/src/period.rs`: `Period::decide`, `BranchMiss`, `WalkStop`, and `Period::near`. `near` reads the same marks with `Margin::of` at `Band::least()`, off the log under ledger row F31.
  - `crates/topo/src/pcurves/decision.rs`, the `"pcurve_singular_turn"` call: the precedent for an angle gap levered through `Period::decide`, with `Undecided` mapped to an escalation and `OnMark` to its own case.
  - `crates/geom-core/src/sym.rs`, `impl Decide for Sym`: "Everything else is `T::sign_within` verbatim".
  - `crates/geom-core/src/k_stats.rs` `decide_across_reported`, and `crates/topo/src/boolean/join.rs` `travel` and `turn::Turn`.
  - `crates/topo/src/census.rs` `span_clear_as` and `crates/step-import/src/interpret.rs` `file_band` / `band_at`.
- **Brief error.** "The one production precedent for a band built from the band's own thresholds" is wrong: see `span_clear_as` and `file_band` above. My count of production `Band::new` references came from a crude script that cuts each file at its first test module. Treat the figure as approximate; the gate's own counter should produce the pin.
- **`scripts/gates/least-band-on-a-door.sh`** forbids `decide(.., Band::least())` at a production call site. If Ev takes the no-lever form for the count, it has to be a `Period` method in geom-core, or go through `k_stats::prove`; it cannot be spelled at the site.
- **Still unverified by either report:**
  - That a conic parameter is in hand where `Turn::facing` is called. The frame there is `germ_section_frame`'s axis line.
  - Which fold of the corners' `LeverBounds` gives "from below" for the count's meter.
  - The offered band on the Interval lane.
- **`Real::floor` exists** on every lane, so a rounding door is buildable. My objection is the period module's stated rule and the width of an enclosure's floor, not feasibility.
- **Names in `travel`.** `decide_across` takes two names. `travel`'s caller name is the far row and `"bool_join_arc_clear"` the floor row. `"bool_join_arc_slack"` disappears from the K-telemetry roster, so `crates/topo/tests/recourse_roster.rs` and any K-name register will need the row dropped.
- **The other report's `.value()` claim.** I confirmed it at `ssi/certify.rs` (`Margin::levered(margin, Lever::unsourced(domain.extent)).value()`). I did not confirm the `ssi/march.rs` site.
- **Scratch.** One throwaway script at `/root/.local/share/cad-work/designer-dd-b-scratch/bn.py`; an empty file was also created at `/tmp/claude-0/-home-user-cad/55faae27-5a2c-5e42-af60-1b9ce7691468/scratchpad/bn.py` by a mistyped command. Nothing was written in the worktree.