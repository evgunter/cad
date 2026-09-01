# MATE-8 — issue 1435: interior_witness's candidate schedule completed

**Binding at dispatch** (S-MATE program, `docs/S-MATE-PLAN.md`;
difficulty pre-logged at this spec: **M**). Read
`docs/prompts/implementer-discipline.md` in full before starting.
The primary specification is issue 1435, funded as the 4b design's
stage 0 (Evan, in-chat, 2026-09-01: "doing it now sounds good");
`docs/CENSUS-REST-CLOSURE-DESIGN.md`'s U-R2 section is the ratified
contract the rung lives under.

## Situation

`chart_region::interior_witness` — #1063's rescue rung for declared
pairs whose region walk refuses `TouchingBoundary` — declines
whenever its FIXED D9 candidate schedule (each outer trim's vertex
centroid + ear midpoints) misses the overlap. Measured (the MATE-4a
dual, both arms): a ~7.5e-3 m² overlap seven orders above ε missed
by all 14 candidates on a non-convex trim, while a geometrically
equivalent seat certifies on its first candidate — legal declared
seats bifurcate per-fixture on where fixed candidates land. The
rung's ratified three-outcome contract (a proof, a decline, or the
refusal it was carrying) is honest; the DECLINE branch fires on
decidable geometry.

## Deliverables

1. **A complete-or-honest schedule, D9-deterministic.** Preferred
   shape: seed candidates from the OVERLAP ITSELF — the planar F5
   machinery can clip the two trims' regions exactly (`contfp`'s
   own parity walk), so candidates drawn from the clipped
   intersection (its vertices' centroid, per-piece) land in the
   overlap by construction whenever a decidable overlap exists.
   Alternative if clipping is unavailable at this site: a
   deterministic refinement ladder with a stated budget whose
   exhaustion is a DECLINE THAT SAYS SO (schedule exhausted at
   depth N), never a silent miss. Either way: the decline stops
   firing on the demonstrated class, and any remaining decline
   states its cause honestly.
2. **Red-first from the dual's own demonstrations**: the spike-seat
   PAIR (adopted rows `a_spike_overhang_certifies_outright` /
   MATE-4a's overhang seat) — on main one certifies and one parks
   `Uncertified`; after, BOTH certify. The MATE-4a probe
   (`the_lemma_probe_declared`'s re-blessed outcome) flips again —
   re-bless it onto the certified outcome with the reasoning at the
   site (it is this program's own pin; note the flip in the PR).
3. **No regression**: the #969/#1063/MATE-4a/MATE-5 suites are the
   oracles — every currently-certifying seat still certifies,
   every ratified refusal unmoved. The flush seat's first-candidate
   fast path stays (do not slow the common case to fix the rare
   one — say what the schedule costs on the flush seat).
4. **The frame-invariance obligation**: candidates derived from the
   clip are symmetric in the pair by construction — pin it (run the
   bifurcation pair both ways).
5. **Class sweep** (discipline §5): the genus is "a fixed sampling
   schedule deciding a class outcome" — sweep chart_region.rs
   (MATE-5's cylinder arm disclosed its own schedule posture with a
   1435 cross-reference: revisit that disclosure and update it to
   cite this unit's landed shape); hit list, dispositions, blind
   spots.

## Acceptance

- The bifurcation pair certifies both ways; the re-blessed probes
  green with their new outcomes stated; all oracle suites green.
- Refusal-reach moves classified against the D2 addendum (row 2:
  the schedule-miss over-refusal retires).
- ε posture (issue-1356): the witness decides under the band —
  argue the three-outcome band story; pin lanes as the change
  warrants.

## Hard rules

- NO `Co-Authored-By` trailer and no model names in lane commits.
- Keyword hygiene: write "issue 1435" spelled out; never a closing
  keyword before a `#`-reference. The orchestrator closes the
  issue after merge.
- Scope fence: `crates/topo/src/chart_region.rs` and
  `crates/topo/tests/` only. No census.rs, no boolean/, no docs
  beyond the code's own comments, no `docs/MODEL-AB-LOG.md`, no
  `docs/S-MATE-*.md`.
- Work only inside your worktree (the shared session scratchpad is
  off-limits); merge main before opening the PR and whenever it
  moves.
- Commit and push after every coherent unit of work (branch
  `mate/8-witness-schedule`).
