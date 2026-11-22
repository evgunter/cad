## For Ev

**The README clause I sign, word for word.** It goes in the lever bullet of `crates/geom-core/README.md`, "Float rounding", after "…belongs to the reading, not the arm.":

> A count is an integer and is read as one. A reading asked at a tolerance other than the run's (the one a refusal offers) is decided in that tolerance's band, never levered by it.

What each sentence prevents:
- **The count sentence.** Without it, `fate::tip_wraps` is spelled wrongly: today it holds a number of half-turns against ε at a unit arm. So would any later whole-number reading.
- **The offered-tolerance sentence.** Without it, both `dihedral` sites are spelled wrongly (today they lever by `band.escalate()`), and so is `validate::resolved_as`'s interim spelling (today it levers K by a rounding).

I agree to drop the other two sentences from the four-sentence version.
- "A length scaled by a pure number is still that length" adds nothing to "a dimensionless factor belongs to the reading". The apex reaches are spelled rightly without it, and the `Margin::of` rustdoc fix carries the operational point.
- The swing sentence describes `join`'s interim, which its own row deletes. `decide_across`'s doc is already that door's home.

**Final state per site, as agreed:**
- `dihedral::wedge_reads_zero_at_the_arm`: `Margin::levered(sin θ, arm)` at its real arm, decided in the arm refusal's `offered_band()`.
- `dihedral::wedge_reads_zero_at_the_offer`: the same wedge in the pairing refusal's `offered_band()`; the `÷|cos θ|` goes.
- `fate::tip_wraps`: `Period::new(half turn).decide(..)` on `whole − turned`, metered at the corners' least reach from below; `k` is compared with ± the corner count as integers.
- `join::off_conic_slack`: the swing of `decide_across` inside `travel`; `bool_join_arc_slack` goes. A separate row reads each site on the conic, under your records condition.
- `intersect` `pn_apex_tangent_reach` and `pn_apex_point_reach`: `Margin::of` of one named apex-start function, a plain division on the decided arm.
- `intersect` `pair_bound` far, near and offset: plain arithmetic. Far and near call the same function.
- `validate::resolved_as`: `Margin::of(ρ)` in the named one-K-step-finer band `(ε/K, ε)`, until the re-posing row deletes it.

**What is asked of you:**
- **The clause above.** It is the only decision. The `Margin::of` rustdoc fix rides the same PR so you can see it, but it decides nothing new.
- **The count's lever, shown for visibility, not as a question.** The count's marks are metered at the corners' least reach, read from below. That is because what admitted each corner as a half-turn was a Zero at its own reach. The no-lever alternative is a `Period` method in geom-core that reads the same marks for their sign alone at `Band::least`, as `Period::near` does. The two differ only on loops whose edges are within the band, and those refuse earlier anyway. I lean levered.

## For the orchestrator

- **Pinning `Band::new`: part of what I recommend, and an agent's call.**
  - *Checked.* `census::span_clear_as` (`census.rs:4227`) is confirmed in production. I did not open `step_import::interpret::file_band`.
  - *Most references are tests.* The bulk of `Band::new` references in `src/` are in test-only modules (`pierce_rows`, `r1_probes`, `*_probes`, `tier3_tests`). The pin must use the gate's statement view, which drops `cfg(test)` and test-only modules, not a line grep.
  - *What lands.* Named geom-core constructors for the offered band and the finer band, plus a per-file pin on the remaining production `Band::new` references, each with its reason: `chart_region` halved zero, `span_clear_as`, `file_band` (which is `linear_at` written by hand, and should probably call it or a named import door).
  - *Sequencing.* Land the pin no later than the unit that deletes `Lever::unsourced`, since that deletion is what makes `Band::new` the open route.
  - *Why it is not Ev's.* A gate script is enforcement of the ratified "a tolerance is the band's". It is not on CLAUDE.md's sign-off list and decides no design.
- **Anything else for Ev beyond the clause?** No. The `join` read-on-the-conic row goes to Ev only through its own PR, under the PR 7141 records condition.
- **Unverified by either of us:**
  - The `LeverBounds` fold that gives "least reach from below" across the loop's corners.
  - A conic parameter in hand at `Turn::facing`.
  - How `offered_band` reads on the Interval lane.
  - `file_band` (not opened this round).
- **Gate interaction.** `least-band-on-a-door.sh` forbids `Band::least` at a production call site. If Ev prefers the no-lever count, it must be a `Period` method in geom-core.
- **Roster.** Dropping `bool_join_arc_slack` touches `crates/topo/tests/recourse_roster.rs` and the K-name register, per the other report.
