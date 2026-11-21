# Review: PR 7227 (`oneround/unsourced-bits`) at `a686ebb5`

Full review: the claims C1–C10 and the style lane. Reviewed at the frozen head
`a686ebb564286ce76cb2e5bc70cd0bfc740eaea5`; merge base `888cbdb1`. The PR's live
head has moved on since (`f930f0d6`), and nothing here reads it.

## Verdict: APPROVE-WITH-FIXES

The differential agrees with the author's main claim, and covers more than the
author's did: **no verdict moved** in the fast set at either ε, in the slow rows
that reach the sites, or in the tour (counts below). The pins, the fixture
counts and the gate all check out. The fixes are about honesty and scope, not
about numbers:

- **MAJOR 1.** The comment `corner_arms` gains says the verdicts levered at
  its arms refuse on Zero. They do not: on Zero the corner solve answers the
  tangent foot. The PR moves the circle arm to the lower bound on that false
  premise. I demonstrated the effect at the solve: an escalation becomes an
  answer that sits 4ε off the true roots.
- **MINOR 2–8.** `corner_arms` is not reached only from the tour: 193 sweep
  and topo rows reach it. Rows already exist that catch a chord-only read. The
  fallback arm is reached. The circle arm is not a bit-only mover, yet it was
  taken while its twin in `ring_path` was left. The census wedge is now read at
  two arms inside one function. The `intersect` quotients go through a door
  whose dimensional contract they do not meet, and they are family 2, which the
  unit row puts out of scope. The certify test twin is not "a test's stated
  scale". The PR body is stale or wrong in several places.

## What I executed and what I only read

**Executed**
- **A decision-hash differential over the K funnel.** The probe is
  `decision-hash-probe.patch`, applied byte-identically to both trees. It
  hashes **every** classified decision on every thread from the thread's first
  decision, not from first reaching a site. Each record holds the predicate,
  the scalar type and the sign (or "escalated"), plus a second hash with the
  margin's bits. They are aggregated per test: as a multiset, in sequence on
  the test thread, and per predicate.
  - Merge base against the frozen head, with `CAD_FUZZ_SEED=0x5eed` pinned on
    both, nextest `ci` profile.
  - Fast set of geom-brep, topo and sweep (9404 tests) at ε = 1e-9 and 1e-12.
  - 108 slow rows of geom-brep, topo and sweep chosen by module (the sites'
    modules, shell/sf2b/torax, offer rows, square search, the corner rows) at
    both ε.
  - demos/tour torus and teapot tests at both ε.
  - The comparison script is `cmp.py`, the runner `run.sh`, and the slow
    selection `slow.expr`.
- **A reach and kind log at `corner_arms`**, plus env-switched mutants
  (`CAD_ARM_MODE=chord|arc`), in a third worktree. Then the corner-solve probe
  test. All of it is in `corner-arms-probe-and-mutants.patch`; the reach census
  is `corner-arms-reach.txt`.
- `scripts/gates/lever-sources.sh` on the head (green, 47). I then planted a
  new `Lever::unsourced` in `topo/src/census.rs` and it went red, as it
  should: `ROSE crates/topo/src/census.rs: 0 -> 1`.
- Base-vs-base reruns of the 4 rows whose verdict hashes differed. They were
  noise.
- Resolved CI run 44849941554 to step level (the jobs API).

**Only read**
- Every converted site and its callers.
- `edge_extent_inf` and `edge_extent_sup`, and `Margin::levered_inv`.
- The README "Float rounding" section, both rows, and the gate and its counts.
- Sampled markers: 18 of the 47 left, listed under C7.
- The ONEHOME re-citation.

**Not done**
- I did not run the whole slow set: 1598 rows, against 108 selected.
- I did not run the Interval CI lane (`interval.yml`) or the 1e-6 ε row.

### Probe caveat (the first default-ε fast run)
My first probe named its files `pid-threadid`. Over about 10k processes, pids
wrapped (`pid_max` is 32768) and overwrote some records: roughly 450 tests per
side were lost at the default ε. I fixed the name and re-ran the default-ε fast
set on both sides (`fast2` below). The 1e-12 runs and the slow and tour runs
used the fixed name, except the head's 1e-12 fast run, which lost only 1–3
tests.

## CI run 44849941554, step level (frozen head)

**Ran, all green**
- *change filter*: classify, base and direct-path rows. Its step "the merge
  this dispatch gates" was skipped.
- *lint*: the work tracker; gates (7 min); rustfmt; clippy `--all-features`;
  rustdoc. The change-filter self-test step was **skipped**.
- *test (kernel)*:
  - build;
  - default eps (2 min);
  - **"eps 1e-6, 1e-12 — the eps-sensitive crates and test files this diff
    touches"** (1.5 min);
  - "the slow-set tests this diff adds or edits" (1 s: it added none);
  - doc-tests.
- *test (app)*: the same steps (the eps step took 1 s).
- *mesh budget*: ran.
- *corrupt input (release)*: ran.

**Skipped**
- Jobs: *prime the build cache*, *viewer app*, **demos (tour + wild)**, *step
  import*, **interval**, *tools*, *python suite*.
- Steps: "the slow set did not run" (it is conditional).

**What CI never saw**
- No slow row ran on CI.
- The tour did not run on CI. The PR body calls the tour the only reacher of
  `corner_arms`. That is wrong: 193 rows in the fast set reach it (C3).
- The interval lane did not run.
- CI's ε rows are 1e-6 and 1e-12, not 1e-9. The default step is 1e-9.

## Findings

### MAJOR 1. The `corner_arms` verdicts act on their Zero. The new comment says they refuse on it, and the circle arm was moved to the lower bound on that premise.
DEMONSTRATED at the solve level, BY-INSPECTION at the body level · `likely`.

**What the new comment claims.** `crates/topo/src/offset_axial.rs:1224-1229`
now says: "Each reading levered at them is a transversality whose Zero
refuses, so an arm past an edge's diameter would certify a corner too shallow
to pin within that edge." The PR body says the same ("the longest arm a
refuse-on-Zero transversality may take").

**What the code does.**
- In `solve_pairs` (`offset_axial.rs:1966-1988`) and `cap_pair_corner`
  (`:1785-1805`), a Zero at `offset_axial_corner` sets `resolvable = false`.
  The solve then answers with `tangent_foot`, wherever that function's own
  tangency gap reads Zero.
- `tangent_foot`'s doc (`:1900-1914`) says the foot sits `√(2r·gap)` off a
  root "wherever a root is determined".
- The module's own row `the_corner_pairs_climb_the_ladder` pins the foot
  answering.

So this is a verdict that acts on every sign. Positive serves the branch root;
Zero serves the foot.

**What the README rule asks.**
- `crates/geom-core/README.md`, "Float rounding": a verdict that acts on its
  Zero reads the extent from above. One that acts on every sign reads both
  bounds and acts only where they agree.
- `edge_extent_inf`'s own doc lists "a root a corner accepts as transversal"
  among its from-below consumers, under the premise that "a Zero is a refusal,
  never a served class". That premise is false here.
- The author's sentence is the README's rule for refuse-on-Zero verdicts,
  applied to a verdict that does not refuse on Zero.
- The old arm (the arc length, longer than D) broke the rule on the Positive
  side. The new one (≤ D) breaks it on the Zero side. Neither reads both
  bounds.

**The probe.** `review_probe_the_shorter_arm_answers_a_foot_where_the_arc_length_refused`
(in the mutants patch).
- Setup: a station line ρ = 11 − 8e-12 against the profile circle (r = 1,
  centre (10, 0)), band (1e-6, 1e-5), and the arm a half-turn edge of radius
  0.1 has.
- **Old arm** (π·0.1, the arc length): `Err(Escalated { margin 1.2567e-6,
  predicate "offset_axial_corner" })`. The solve refuses.
- **New arm** (2·0.1, the diameter): `Ok(Some((10.999999999992, 0.0)))`. The
  solve answers the foot. The true roots are (11 − δ, ±4.000e-6), so the
  answer sits **4.0e-6 = 4 zero-bands off either root**.

The π/2 drop therefore moves a verdict in the unsafe direction, from a refusal
to an answer. It happens on the window `det·2r_edge ≤ ε < det·π·r_edge`, where
the foot's error is about `r_profile·ε/(2 r_edge)`, which exceeds ε once the
profile circle is wider than the edge's diameter.

**What I did not show.** I did not build a door-level body that lands in that
window. The suites never landed there: there are no verdict moves at either ε
over 193 reaching rows. Whether such a body is reachable is unshown.

**Severity.** MAJOR for the false premise in the new comment, and for the
bound being chosen by it. The numeric exposure is band-scale and unreached by
any suite.

### MINOR 2. `corner_arms` is reached by 193 rows in the fast set, not only the tour, and the fallback arm is reached. The brief's premise and the PR body are wrong.
DEMONSTRATED · `sure`.

**The reach log.** Over the sweep and topo `ci` profile it records 8169 calls
from 193 tests (`corner-arms-reach.txt`):
- Largest reachers: `verbs_shell` 1062 calls, `spiric_rim` 738,
  `shell_curved_mouth` 642, `shell8_r2_probes` 538, `shell7_seam_corner` 492,
  `torax_axial` 283, and more.
- Carrier kinds: 5232 circle (3137 half turns, 769 full turns, and 80 on the
  Interval lane), 2923 line, **12 spiric**.

**Where the PR body is wrong.**
- It says: "demos/tour's torus-vessel and teapot tests (36), the only suites
  that reach `offset_axial::corner_arms`".
- It says the fallback is "**not measured** … the tour tests that reach
  `corner_arms` record only those two kinds".

**The fallback.** It is reached, by the spiric edges in
`verbs_shell::the_klein_elbow_opens_at_one_cap_to_its_measured_volume` and
`…the_klein_wall_pair_seals…`. At every one of its 12 reaches the new read
equals the old chord bit for bit (`new=0.4504054321550891
chord=0.4504054321550891`). So C4's content holds where it is reached, but
the PR's "not measured" was answerable from the fast set.

### MINOR 3. Existing rows already catch the chord-only read that the PR says no fixture catches.
DEMONSTRATED · `sure`.

**The mutant.** `CAD_ARM_MODE=chord` reads every arm as its chord again.
- It reddens about 20 of the 192 reaching rows (the list was cut at 20). Among
  them: `shell7_seam_corner::a_sphere_from_an_arc_run_hollows_to_its_closed_form`,
  `shell_cavity_embedded::the_tube_holds`,
  `shell_curved_mouth::a_tubes_wall_opens_to_two_seamed_bands`,
  `verbs_shell::an_annular_cap_opens_to_two_disjoint_rims`,
  `lamina_annulus::shell_and_offset_of_unslit_annuli_match_closed_forms`,
  `shell10_r2_dump::shell10_r2_dump_corpus`,
  `verbs_shell::the_record_is_a_function_of_the_construction`, and others.
- These are the closed one-seam circles: 769 full-turn reaches.

**What this contradicts.** The PR's "A closed one-seam circle would catch it,
and no fixture builds one at `corner_arms`" is false.

**The new row.** It goes red on the old arc read (`CAD_ARM_MODE=arc`:
FAIL). Under arc mode it is the **only** red among the 192 reaching rows, so
the old arm was never pinned by a door-built row. That is consistent with "no
verdict moved".

**The midpoint mutant** I did not run; it is credible by inspection.

**Should the row catch the chord-only read?** It needn't: the closed-circle
rows already do. But the PR should cite them, rather than claim there are
none.

### MINOR 4. The circle arm is not a bit-only mover, yet the unit took it; its identical twin is left, so the family is now read two ways.
BY-INSPECTION · `sure`.

- **The unit's own scope.** The unit row's title, "arms whose honest read
  moves only last bits", fences the circle arm out. The PR itself says it is
  "the one converted production read that changed by more than last bits" (up
  to π/2).
- **The twin.** `crates/topo/src/ring_path.rs:1803` keeps the same arm: "the
  arc's length, its angular span times its radius, longer than the arc's
  extent, which `edge_extent_inf` reads as its chord or fold". The comment is
  word for word the one deleted from `offset_axial.rs`, and the twin stays on
  `Lever::unsourced`.
- **The parent row.** It groups both under family 1, "arc lengths, where the
  extent reads are diameters". After this PR, one is read as a diameter and
  one as an arc length.
- **The class.** The same class probably includes `curve_pair::pose`
  `line_line` and `line_conic`.

### MINOR 5. The census wedge is read at two arms inside one function (C7).
BY-INSPECTION · `sure`.

- `edge_in_curved_face` (`crates/topo/src/census.rs:4715-4720`) now levers its
  two wall sides at `e.span().extent()`.
- Its own `strictly_inside` calls, a few lines below (`:4724`, via
  `census.rs:2798-2806`), still lever at `w.arm`, which is
  `SectorShape::arm`, the `metering_arm` family left whole by the
  orchestrator's ruling.

So the family the brief keeps whole for the records unit is now read two ways
in one function: the very split that ruling was meant to avoid.

**The new comment.** "At the line edge's own extent, which each sheet is
scaled to" is inaccurate. `Wedge::own` and `Wedge::next` are unit
(`shape.unit_own`, `census.rs:2785`). Only the inputs to `edge_wedge_shape`
were scaled by `e.len`.

### MINOR 6. `intersect`'s quotients spell a stand-off over a sine through `levered_inv`, whose contract is a reading over a per-length; and they are family 2, which the unit row fences out (C1).
BY-INSPECTION · `sure` on the door, `likely` on the scope.

**The door's contract.** `Margin::levered_inv(x, per_length)`
(`crates/geom-core/src/predicate.rs:980-992`) is documented for a
dimensionless x over a relative curvature (1/m), or a jet residual over a
speed.

**What the sites pass.**
- `crates/geom-brep/src/intersect.rs:2448-2449` and `:2661-2662` pass
  `size = |apex_gap| + gap_err`, a length: the plane's stand-off **plus a
  hand-written rounding term**.
- The divisor is `steep`, `|discr|`, `d` or `m`, each dimensionless.
- The offset site (`:2685-2688`) passes `2·size` over a sum of square roots.
- Each result is `.value()`'d at once.

**So, to C1's question.** `size` is not a rate's denominator, and not an arm
hidden in a door either. It is the margin itself, a length over a sine, which
the README does allow ("a length that is the reading itself … is the
margin"). But the door's dimensional argument does not cover it, and the code
is a bare division wearing a door's name.

**The scope.** The PR's own row correction says these sites are family 2 ("a
tolerance or a rounding standing in for an arm", "re-pose, not a lever"). The
unit row says "Not this unit: families 2 to 5". The re-posing family 2 asks
for is not done: `gap_err`, a rounding charge, stays in the margin (README:
"Re-posing, never charging"). The unit row also lists "`intersect`'s quotient
forms" in scope, so the row contradicts itself, and the PR resolves the
contradiction silently.

**The parent row.** It already listed these under family 2. The correction
"they were family 2's, not family 1's" corrects something only the unit row
and "Reconciled for Ev" implied.

### MINOR 7. The certify test twin is not "a test's stated scale" (C5).
BY-INSPECTION · `sure`.

- **The site.** `crates/geom-brep/src/ssi/certify.rs:3890-3894` reads
  `fixture_lever(domain.extent)`. That is the production row's own
  `SsiDomain::extent`, mirrored so that `to_bits()` matches the production
  `Lever::unsourced(domain.extent)` at `certify.rs:2747`.
- **The door's stated reason.** `lever-fixtures.counts` and the gate header
  say the fixture doors are "a test's stated scale, with no geometry behind
  it".
- **Why that does not fit.** This site is an oracle bound by hand to a
  production read. When the family-3 unit converts `certify.rs:2747`, this
  twin must change in step, and nothing says so at either site (Q2).

**The rest of C5 holds.**
- The other six sites are test code under `#[cfg(test)]`: `fan.rs:649`,
  `join.rs:8596`, `link.rs:5660`, and the `tests` modules of `star/cut.rs` and
  `star/order.rs`.
- The counts move by exactly these seven: geom-brep +1; topo +6 (fan 2, join
  `ExtentBall::fixture` 1, `chord_rows` 1, cut 1, order 1).

### MINOR 8. The PR body and the unit row are stale or wrong in places (C10, C9).
BY-INSPECTION · `sure`.

- **Stale "stacked" text.** The title and first line say "stacked on #7143 …
  cannot merge before that PR"; #7143 merged 2026-11-21 00:08. The unit row
  still says "Stacked on `oneround/lever-doors` (PR 7143)".
- **Reach claims.** "the only suites that reach `corner_arms`" and the
  fallback "not measured" (MINOR 2). "no fixture builds one at
  `corner_arms`" (MINOR 3).
- **Wrong base line numbers.** The marker list cites `boolean/join.rs` 8489
  and 2160; at the base they are **8622** and **2271**.
- **The seen-on-base list is incomplete.** At ε = 1e-9 a **fifth** slow row
  is red in my runs: sweep
  `offer_rows::every_sphere_offer_passes_just_below_it`. It is red on both
  sides, so it is not this PR's (C10 below).
- **"About 870 tests reach a site."** I cannot reconstruct this. My bits-only
  differences alone touch 445–512 tests, and `corner_arms` alone is reached
  by 193.

### NOTE 9. `leaned`'s arm is still the `metering_arm` family's unsourced arm (C1).
BY-INSPECTION · `sure`.

- **What the change does.** `star::leaned(x, leans) = x·min(lean)` moves the
  lean's sine into the reading, as the README asks ("a dimensionless factor
  belongs to the reading, not the arm"). That part is right.
- **Where the arm comes from.** It is `BoolSector::arm` or the star's
  `Sector.arm`, read from `metering_arm`'s single `Lever::unsourced` pin
  (`crates/geom-brep/src/sector_shape.rs:380`;
  `crates/topo/src/star/walk.rs:251,260,284`).
- **The consequence.** The site now carries no marker of its own, but its arm
  still has no source. Removing the double conversion is correct. "Levered at
  the arm itself" should not be read as "sourced".

### NOTE 10. `census::Span::extent` rebuilds a line from the span's own record.
BY-INSPECTION · `likely`.

- **The read.** `crates/topo/src/census.rs:391-397` reads
  `edge_extent_inf(Line(p0, dir), at.0, at.1)`.
- **What the span already holds.** `ends`, the edge's stored vertices (or a
  stretch's evaluated ends).
- **The consequence.** With `dir` unit, the result is `len` reproduced through
  a carrier the census minted, not the edge's own points or carrier. It is
  equal to rounding, so this is a source in form more than in substance.
- **Bounds.** For a line, inf (the chord) and sup (`|dir|·|Δt|`, the old
  read's exact formula) coincide to rounding. So `parallel_reading`, which
  acts on every sign, is not hurt by reading one bound.

### NOTE 11. Docs that still describe the lean as part of the lever.
BY-INSPECTION · `likely`.

- `crates/topo/src/boolean/insert.rs:1905-1906`: "It is levered at the
  sector's arm and the two directions' leans (`sectors::leaned`)".
- `crates/topo/src/boolean/sectors.rs:1225-1226`: "the bounds' reaches its
  lever as [`leaned`] reads them". `leaned` reads no reach any more.

## Claims

- **C1 · partly holds.**
  - `line_cone_roots`, `corner_arms` (line, circle and fallback) and
    `parallel_reading` read an edge's extent through its door.
  - `parallel_reading` and the census wedge rebuild a line from the record
    (NOTE 10).
  - `leaned`'s arm is still unsourced upstream (NOTE 9).
  - The `intersect` five are a margin, not an arm, spelled through a door
    whose contract they do not meet (MINOR 6). `Span::len` is gone.
- **C2 · fails at `corner_arms` (MAJOR 1). Per site:**
  - `line_quadric_cone_lead`: claims a sign; Zero → `Unsettled::Degenerate`;
    from below is right.
  - `pm_census_ef_curved_wall_side`: claims a sign; Zero refuses; from below
    is right.
  - `pm_census_ee_parallel`: acts on every sign; for lines inf = sup to
    rounding; acceptable.
  - `germ_side`, `germ_order` and `bool_shared_cut_order`: arm unchanged.
  - `pn_apex_*`: act on Zero; not a lever at all.
  - `offset_axial_corner`, `offset_axial_cap_pair` and
    `offset_together_corner`: act on Zero. The axial pair serves the tangent
    foot; the planar door moves to the next triple. From below alone is not
    the README's rule.
  - The author's sentence is the README's rule for a verdict this is not.
- **C3 · holds on what I ran.** It is better covered than the author's
  design, since every decision is hashed from the thread's first, not from
  the first site reached. Results are in the table under "The differential".
- **C4 · holds in content, with a fact wrong.**
  - The fallback is reached by 12 spiric arms in the fast set, and reads the
    chord there bit for bit (MINOR 2).
  - `edge_extent_inf` ≥ chord and ≤ D, so a longer arm on a claim-a-sign
    reading is within the rule. For the Zero arm, see MAJOR 1.
  - Collapsing the match IS the gate's intent. `cover-kind-match.sh` cites
    `[one-way.readings]`, "the kind matches in the readings … never at a
    reader": the per-kind logic belongs inside `edge_extent_inf`, and the
    reader should not repeat it. The `offset_axial` pin stayed at 9, so
    nothing was cleared that counted.
- **C5 · holds, except that the certify twin's door reason is wrong
  (MINOR 7).**
- **C6 · holds.**
  - 65 → 47 markers, and the per-file pins total 47.
  - The removed pins are line_quadric 1, intersect 5, certify 1, fan 2,
    join 1, census 2, chord_rows 1, offset_axial 2, cut 1 and order 2: 18 in
    all, with none added.
  - The gate is green at the head and red on a plant.
- **C7 · holds for the sample. The family is now read two ways (MINOR 5), and
  so are the arc-length twins (MINOR 4).**
  - 18 of the 47 checked against the tree: march ×2, certify 2747, boundary,
    section, pose ×3, `chord_join`, `rim_wedge`, `ring_path` 329 and 1803,
    `chart_region` ×2, `vtxfac`, `dihedral` ×2, `pcurves::decision`,
    `validate`, `fate`, `join` 2271, `trace` ×2, `overlap`, `replay`,
    `pcurve_cache` ×5, `props::curved`, and profile ×6.
  - Each matches its family as the PR body files it (with join's line number
    wrong).
- **C8 · the row goes red on the arc-length mutant (run).** The midpoint
  mutant was not run (credible by inspection). On the chord-only gap see
  MINOR 3. "Bit-only sites: no rows" is fair: at the bits-moved sites my
  differential found no verdict to pin. The circle arm is not bit-only, and
  it has a row.
- **C9 · holds.**
  - `boolean::sectors::metering_arm` and `departure_arm_lever` exist nowhere
    in the base tree.
  - `metering_arm`'s nine callers are as listed: geom-brep `sector_shape`
    321 and 329; `star/walk.rs` 251, 260 and 284; `topo/sector_shape.rs` 171
    (`ruling_chord`) and 291 (`apex_extrema_inside`); `splitting/insert.rs`
    139 and 304.
  - `SectorShape::arm`, `FullTurn::arm` and `ApexShape::arm` are bare `T`
    fields.
  - `line_cone_roots` is in `line_quadric`, and `leaned_lever` was
    `star::order`'s.
  - The ONEHOME re-citation is a pure rename, `star::leaned_lever` →
    `star::leaned`, and accurate. Its "By `star::leaned`'s derivation" now
    names a function that derives the foreshortening rather than a lever,
    which is harmless.
  - One quibble is in MINOR 6.
- **C10 · see the slow-set table.** The title is stale (MINOR 8).

## The differential

Base `888cbdb1` against head `a686ebb5`, seeds pinned, nextest `ci` profile.
"Verdict" means the per-predicate multiset of (predicate, scalar, sign or
escalation). "Bits" means the same with the margin's bits.

| suite | ε | rows run (base / head) | rows with decisions compared | decisions compared | verdict moves | rows with bit moves | outcomes |
|---|---|---|---|---|---|---|---|
| fast set, geom-brep + topo + sweep (`fast2`) | 1e-9 | 9403 / 9404 | 7966 | 198,746,595 | **0** (+4 noise rows) | 510 | all pass, both sides |
| fast set (`fast`) | 1e-12 | 9403 / 9404 | 7944 | 199,573,713 | **0** (+4 noise rows) | 512 | all pass, both sides |
| 108 slow rows (`slow.expr`) | 1e-9 | 108 / 108 | 100 | 184,274,120 | **0** | 40 | 5 red on both, panic text identical |
| 108 slow rows | 1e-12 | 108 / 108 | 100 | 192,841,511 | **0** | 40 | 4 red on both, panic text identical |
| demos/tour `torus|teapot` (38 rows) | 1e-9 | 38 / 38 | 38 | 13,643,113 | **0** | 26 | all pass, both sides |
| demos/tour `torus|teapot` | 1e-12 | 38 / 38 | 38 | 18,556,371 | **0** | 25 | `lily::review_probes::the_weld_refuses_at_the_cone_torus_pair` red on both, panic identical |

The new row (`a_half_turn_rims_corner_arms_are_its_diameter`) exists only on the head, so it is in no comparison.

**Verdict differences: only the 4 nondeterministic rows.** They are
`topo validate::tests::{ngon_pillows_validate_cleanly, prisms_validate_cleanly,
random_op_sequences_stay_tier1_valid_at_every_step,
random_sequences_then_teardown_validate_vacuously}`, and they differ between
two runs of the base alone. They touch none of the sites' predicates.

**Bits moved only at the sites' own predicates.** Counted in tests at 1e-12:

| predicate | tests |
|---|---|
| `line_quadric_cone_lead` | 230 |
| `offset_axial_corner` | 170 |
| `pm_census_ee_parallel` | 109, across f64, `Interval` and `Sym<Interval>` |
| `germ_side` | 26 |
| `germ_order` | 19 |
| `pn_apex_{pair,point,tangent}_reach` | 21 |
| `offset_axial_cap_pair` | 9 |
| `offset_together_corner` | 5 |

`pm_census_ef_curved_wall_side` and `bool_shared_cut_order` moved no bits
anywhere.

**Slow rows.** All slow and tour outcomes are the same on both sides. The
failing set and its panic text are listed per run in the C10 table below.

### C10: the slow rows said to be red without this unit

| row | 1e-9 base | 1e-9 head | 1e-12 base | 1e-12 head |
|---|---|---|---|---|
| topo `offer_rows::every_offered_tolerance_passes_just_below_it` | red | red | red | red |
| topo `section_cert::square_search::the_square_wall_arm_agrees_with_the_traced_section` (fuzz seed pinned) | red | red | red | red |
| topo `a_corner_reads_only_its_own_sectors::the_corners_offer_executes` | red | red | red | red |
| topo `a_corner_reads_only_its_own_sectors::an_unglued_pair_parallel_at_the_arm_refuses_on_the_arms_tilt` | red | red | green | green |
| sweep `offer_rows::every_sphere_offer_passes_just_below_it`, **not in the PR's list** | red | red | red | red |

**C10 holds for the four rows the PR names.** Each is red on the merge base at
1e-9, and this unit does not change how it fails. The panic output is
byte-identical base against head, once thread ids and backtraces are stripped,
at both ε. Each failing row's whole decision stream is identical too: it is
among the 100 rows with zero verdict and zero bit moves except those listed.

**A fifth row is red on the base and missing from the PR's list.** It is
sweep's sphere offer row. The PR body also says nothing of 1e-12, where the
fourth row turns green on both sides.

The failures themselves are offer-band rows (offers 10× below the quoted
margin), the square search's three tube-top degeneracies, and the corner
offer's quoted-versus-offered tolerance. All are base defects and none are
this unit's.

## Style

**Questions exercised:** Q1, Q2, Q3, Q4, Q5, Q6, Q7. Q8 was partial: I read
`offset_axial.rs`'s module header (1–300) and its solve section (1220–2060),
not the whole 4067-line file, and not `census.rs` (23k lines).

- **Q1, near-parallel roles: two arc-length twins** (MINOR 4):
  `offset_axial.rs` (converted) and `ring_path.rs:1803` (left), with the same
  sentence. **Two arms for one wedge** (MINOR 5). `sure`.
- **Q2, comment doing the code's work.**
  - The new `corner_arms` doc asserts an invariant ("whose Zero refuses")
    that the code under it contradicts (MAJOR 1).
  - "read as it reads it" at the certify twin is a by-hand coupling that
    nothing enforces (MINOR 7).
  - `sure`.
- **Q2, the deleted text.**
  - The deleted comment carried a measured fact: "Measured on the revolved
    TUBE, which refused that way before this. The arc length is the length
    that was always meant." It recorded why the chord was rejected.
  - The new text keeps the closed-circle argument, but drops the measurement
    and the statement that the arc length was a deliberate choice. That is a
    design decision reversed by comment edit, not argued.
  - `likely`.
- **Q3, can this test fail?**
  - The new row pins the value 2 at a pose where the line arm is also 2. The
    corner's verdicts read `min` over arms (every arm must read Positive), so
    the circle arm's change never governs that corner.
  - The row pins `corner_arms`' output, not any verdict that consumes it.
  - `sure`.
- **Q4, invalidated premises.**
  - `edge_extent_inf`'s doc (`crates/geom-brep/src/extent.rs:466-467`) names
    "a root a corner accepts as transversal" as a from-below consumer whose
    Zero refuses. `offset_axial`'s corner is a consumer that serves on Zero.
    That is a sentence the code drifted from; it is not a doc to fix.
  - The PR leaned on that sentence instead of checking it.
  - `likely`.
- **Q5, promises the code doesn't keep.**
  - `offset_axial`'s Conditioning header says the meter "refuses a TANGENT
    junction". The tangent foot answers tangent junctions in band. This
    predates the PR, but the PR's new comment builds on it.
  - `likely`.
- **Q6, deviations.**
  - Taking family-2 sites and a more-than-bits mover, against the unit row's
    fences, is disclosed in the row correction but not scheduled or
    justified as an improvement (MINOR 4, MINOR 6). `likely`.
  - The "no verdict moved" claim rests on a measurement with no guard. Ev
    accepted moved verdicts (PR 7141), so it needs none, but the PR body
    should say that the differential was not committed. `unsure`.
- **Q7, how I'd do it.**
  - `Margin::levered_inv(…).value()` used as a division helper reads as
    ceremony. `Margin::of(size / steep)`, or a door for "a length over a
    sine", would say what it is. `likely`.
  - Also: the `cover-kind-match` fix line in a lever PR (`a686ebb5`,
    "curvature_arm's pin stands on its own") is an unrelated gate repair
    riding along. It is disclosed, and harmless. `sure`.
- **Class sweeps owed.**
  - Arc-length arms: `ring_path:1803`, `curve_pair::pose` `line_line` and
    `line_conic`.
  - Readers of `edge_extent_inf` whose Zero is served: grep for
    `edge_extent_inf` callers whose verdict feeds a fallback ladder
    (`offset_together`'s triple loop is one).

### Ratings (1–5)

| | rating | evidence |
|---|---|---|
| Idiom and structure | 3 | `leaned` and the match collapse are clean. `levered_inv` is misused as a divider, and the wedge reads two arms in one fn. |
| Test quality | 2 | The one new row cannot see the circle arm govern a verdict (the min over arms is the line's 2). Rows that catch a chord-only read exist but go uncited. No row pins the Zero-served foot against the arm. |
| Doc and comment honesty | 2 | The new `corner_arms` premise is false. The census comment says unit sheets are scaled. The PR body is wrong about reach, the fallback, fixtures and line numbers, and its "stacked" text is stale. |
