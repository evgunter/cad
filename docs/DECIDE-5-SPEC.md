# DECIDE-5 — the arc's span from the turn the profile decided (spec)

**Program:** DECIDE (`work/decide/plan.md`). **Item:**
`work/decide/rule-d-reaches-the-unit-bulge-only.md`, the section "The
bulge's sign: how the tier learns it", ruled by Ev on `[ev]` #3186
(2026-09-25, Decision 1). **Unit:** `work/decide/DECIDE-5.md`.
**Branch:** `decide/5-span-from-the-turn`, cut from `props/sign-hull`'s
head after DECIDE-4's merge (`bd2bf0c85`). The PR targets
`props/sign-hull`, and no `main` is merged into it. **Implementer:**
Opus. **Review tier:** single FULL review (§Review).

**Read first, in full:**
- `docs/prompts/implementer-discipline.md`;
- the item's DECIDE-4 section: route B's local patch, its counts and its
  costs, which are this unit's before-numbers, and the ruling section
  after it;
- `crates/sweep/src/swept.rs`: `placed_segment_spec`, `arc_span`,
  `turn_axis`, `register_span_identity` and `register_rim_identity`, and
  the crate docs on the turn;
- `crates/profile/src/path/family.rs`'s `bulge_carrier`: the decision
  `path_arc_bulge` and where `SegmentKind::Arc { turn }` is minted;
- `crates/sweep/src/revolve/axis.rs:470`, the other `arc_span` caller;
- `crates/editor-core/tests/m10_bulge_interval.rs` and the link's pins
  (`decide_3_split_rows_interval.rs`, `m10_10_pins_interval.rs`).

## The claim

The profile program decides the arc's turn `σ` from the bulge's sign
(`path_arc_bulge`, a certified `Sign` over the box) and hands it to the
sweep. The sweep uses it for the carrier's axis (`turn_axis`), and then
re-inspects the bulge through `abs` for the span, `4·atan|b|`. The
pushforward spells the same span `4·atan b`. So at a parameter bulge the
tier sees two atoms for one quantity (`sqrt(1 + |b|²)` against
`sqrt(1 + b²)`), related only through the sign the profile already
decided.

**The change (Ev's ruling):** the sweep spells the span `4·atan(σ·b)`
from the decided turn. Where `σ = sign(b)` over the box, `σ·b` IS `|b|`:
negation is exact, so the value channel is the same bits. `atan(−b)`
meets `atan(b)` as forms through `(−b)² = b²`. What the constructor
states changes; what it computes does not.

**Ratified and not re-litigated:** Ev's Decision 1 on #3186 (the
spelling, not the registration variant); Decision 2 (the apothem's sign
is `the-apothems-sign-is-a-value-read`, not this unit's); rule C stays
shipped off; DECIDE-4's exact quotient.

## Phase 1 — before touching anything

1. **Bit-identity.** Apply the change and run the value-channel pins,
   the sweep crate's tests and the digests that hash values, not forms.
   Show that no value moves. If a value moves, stop and report: `σ·b ≠
   |b|` somewhere means the turn and the bulge disagree, and that is a
   finding in its own right.
2. **`Sign::Zero`.** `turn_axis` takes the positive arm for a `Zero` turn
   ("unreachable for classified arcs … kept total"). The span must use
   the same convention, in ONE helper beside `turn_axis`, or both must
   fail loud if `Zero` is truly unreachable. Choose, and state why. No
   second spelling of the convention.
3. **The other caller.** `revolve/axis.rs:470` uses `arc_span` for a
   margin `π − arc_span(b)`. That is a magnitude question, and it keeps
   `abs` unless reading it shows otherwise. Say which. If `arc_span`
   ends with one caller, it stays, named for what it now is.

## Phase 2 — the change and the re-baselines

- The change in `placed_segment_spec`, through the helper from item 2.
  `register_span_identity` states its identity about the new span node;
  check that its proof comment still reads true, and update it if it
  names `|b|`.
- **Re-take every split and ceiling the change moves**, and re-baseline
  and say each. The before-numbers are route B's, measured by DECIDE-4
  with a local patch:
  - the `0.5` parameter control: `carrier_matches_mapped_source`
    126/0/38/16 → 126/0/48/6, and `carrier_on_surface_2` 138/0/0/6 →
    140/0/0/4;
  - its ceiling: 4.3375e2 → 5.1078e2·ε;
  - R2's link: `carrier_matches_mapped_source` 108/0/40/32 → 108/0/62/10,
    and `carrier_on_surface_2` 92/0/6/10 → 88/0/8/12.

  If they differ now that DECIDE-4's exact quotient is in, say so.
- **The link's four lost theorems.** Render the four decisions that go
  from theorem to door or numeric, and say why. If a value-free rule
  would keep them, file it as an issue; this unit does not build it.
  They are re-baselined, and said, per Ev's standing words: "never skip
  out on a change that would make the code better because it would
  require rebaselining".
- **Leaf cost:** best of 3, release, `ON + the ladder`, the instrument
  documents, before and after. DECIDE-4 measured the link at 19.8 →
  10.7 s and the pad at 151 → 91 s.
- **A row** that fails if the span is spelled through `abs` again. At
  the scalar door, the carrier's `4·atan(σ·b)` and the pushforward's
  `4·atan b` must meet as a theorem at a parameter bulge of either sign.

## Scope

- **Files:** `crates/sweep/src/swept.rs`; `crates/sweep/src/revolve/axis.rs`
  only if item 3 says so; tests under `crates/*/tests`; the unit and item
  files.
- **Territory:** `crates/sweep` is CARVE's and BAND's ground. Run
  `work.py territory --base origin/props/sign-hull` and announce the seam
  in the PR body.
- **Not in scope:** no change to the profile program's decision, to rule
  C, or to any tier rule.

## Review

**Single FULL review**, recorded in `work/decide/log.md` at spec time.
The design is ruled. The change is a few lines at one constructor site,
and whether it is right can be settled by reading it plus executing a
handful of probes:
- the value channel's bit-identity;
- the `Zero` convention;
- the proof comment of the span identity;
- the re-baselines, the link's four lost theorems especially.

The review falsifies those claims, plus
`docs/prompts/reviewer-style-lane.md` in full.

## Landing

- When the PR opens, the unit's status becomes `review`.
- At merge the orchestrator closes the unit, deletes this spec (with a
  note under `docs/doc-ledger/`), and closes
  `rule-d-reaches-the-unit-bulge-only`.
- The item's remaining residue, the ring at `fl(0.4)`, is re-stated at
  the ring class's row. If none is open, it is filed.
