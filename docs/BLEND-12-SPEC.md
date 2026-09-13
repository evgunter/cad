# BLEND-12 — the six fillet recourse sentences reach the caller the gates were written for (spec)

**Program:** BLEND (`work/blend/plan.md`, unit 12). **Item:**
`work/blend/fillet-escalation-site-has-no-producer.md`.
**Track:** kernel change — the standard v6 unit (binding spec, drawn implementer
arm, cross-model dual review, union fix pass, record-at-merge; §Review).
**Pre-draw fields, logged before the draw:** difficulty **S**, task-class
**STRUCTURAL**.

- **S** — one Display arm keyed on names the tree already dispatches on,
  one dead arm retired, the followability rows flipped from
  characterizations to pins. The decision is taken here.
- **STRUCTURAL** — which of three dispositions; no numeric question.

**Territory note.** `crates/profile/*` is S-BOOL's glob; the profile fillet
door's arms are edited by announced seam (`work/bool/log.md`, at dispatch).
Nothing outside `crates/profile/src/{validate.rs, path.rs, lib.rs}`,
`crates/profile/tests/**` and the FFI tag census (`crates/pncad-py/**`, only
if a `PathError` shape moves) changes.

## The claim

**All six `FILLET_*_RECOURSE` sentences are dead: nothing constructs the one
value that renders them.** `crates/profile/src/validate.rs` renders them from
exactly one arm — `ProfileError::Escalated { site: EscalationSite::Fillet, .. }`,
dispatched on the escalation's predicate name (`:719`–`:755`) — and no
`crates/*/src` site constructs that value: every `ProfileError::Escalated` mint
carries a `Segment`, `SegmentPair` or `Loop` site, and `EscalationSite::Fillet`
appears in `src` only at the Display arm (the other two mentions are
hand-built test values). The nine `fillet_*` predicates ARE decided
(`crates/profile/src/sugar.rs`: `fillet_corner_turn`, `fillet_corner_arm`,
`fillet_enclosing_carrier`, `fillet_leg_fit`, `fillet_leg_reach`,
`fillet_offset_lever`, …), but an in-band verdict leaves the door as
`PathError::Escalated { source }`, whose Display has no fillet arm and appends
the shared `COINCIDENCE_RECOURSE` ("declare the coincidence, move the
geometry, or lower the tolerance") — sentences written for a joint the author
never authored, at a fillet the author asked for. `fillet_recourse_followability.rs`
(PR 1753) pins today's state as a characterization: the door's refusal carries
NONE of the six.

**The decision, taken by the orchestrator (2026-09-13): disposition (1).**
`PathError::Escalated`'s Display gains a fillet arm keyed on the predicate
name, exactly the shape BLEND-10 (PR #2497) gave the eight stored-form
predicates (`path.rs`, the arm naming the site and appending
`FILLET_STORED_FORM_INBAND_RECOURSE` with no coincidence tail): an in-band
`fillet_*` verdict renders the site ("resolving the fillet at this corner")
and the tailored sentence, and never the coincidence recourse. The six
constants stay where they live (`validate.rs`, exported from `lib.rs`, beside
BLEND-10's three); the name → sentence map becomes ONE function there
(`fillet_recourse_for(predicate: &str) -> Option<&'static str>`, or the
crate's spelling), read by `path.rs`'s Display — one home for the dispatch,
not a second `match` copied from `:719`. **The `ProfileError` arm at
`EscalationSite::Fillet` is then machinery with no producer and no reader:
retire it** — the variant, the arm, and the two hand-built test values —
unless Phase 1 finds a producer (then say so and keep it). Disposition (2)
(routing `sugar`'s escalations through `ProfileError`) is not taken: the
door's refusal type is `PathError` and D4 ¶1 says one discrimination point
per layer; (3) (retiring the sentences) is not taken: the gates fire and
the sentences are the tailored answer.

**Ratified and not re-litigated:** the nine `fillet_*` predicate names (a
K-corpus family); D4 ¶1 (one variant per situation); the rule that a
recourse must be true at every site its tag can fire (README A3-2); the
BLEND-10 arm's shape.

## Phase 1 — measure before touching anything

Enumerate every `decide("fillet_…")` site in `crates/profile/src` with the
`PathError` path its `Indeterminate` takes to the door (through
`TrimRefusal::Escalated`, `ArcTrimRefusal::Escalated`, the direct mints), and
for each of the six sentences the predicate(s) `validate.rs:719`–`:755` maps
to it. For each predicate, construct an in-band fixture through the PUBLIC
door (the item's siblings: the tilted-turn corner for `fillet_corner_turn`,
a degenerate leg for `fillet_corner_arm`, the enclosing radius at the
crossover for `fillet_enclosing_carrier`, the exact fit for
`fillet_leg_fit`/`fillet_leg_reach`, the offset lever) and record the
sentence rendered TODAY (expected: the coincidence recourse). Predicates that
cannot be driven in band through the door are named with the reason. Then
confirm `EscalationSite::Fillet` has no producer: every constructor of
`ProfileError::Escalated` in `crates/*/src` with its site. One table in the
PR body BEFORE Phase 2 begins.

## Phase 2 — the change

1. `validate.rs`: `fillet_recourse_for(name) -> Option<&'static str>` — the
   one map from the nine names to the six sentences (some names share a
   sentence; the map says which); the Display arm at `:719` rewritten to
   call it if the arm survives, deleted with the variant if it does not.
2. `path.rs`: `PathError::Escalated`'s Display dispatches `source.predicate`
   through `fillet_recourse_for` FIRST, then the stored-form eight, then
   the junction keys with the shared recourse (the order stated at the
   site); the fillet arm names the site and appends the tailored sentence,
   no coincidence tail.
3. Retire `EscalationSite::Fillet` (variant, arm, `tests/common/mod.rs:697`'s
   hand-built value, `rejections.rs:501`'s note) if Phase 1 confirms no
   producer; every citation follows.
4. **Rows**, in `crates/profile/tests/fillet_recourse_followability.rs` (the
   existing home — its module doc promises every fillet recourse followed):
   the characterization rows flip to pins — for each sentence a fixture
   driven in band through the public door, the refusal's rendered text
   asserted to carry the tailored sentence and NOT `COINCIDENCE_RECOURSE`,
   then the sentence's own request built (the rows already build the
   endorsed request; keep that half). A row that the shared recourse never
   renders for a `fillet_*` name (a census over the nine).
5. **The mutant**, in the PR body: drop the fillet arm → the sentence rows
   red; swap two sentences in the map → the two rows red.
6. **The doc trail**: each constant's "Unreachable as rendered prose" doc
   sentence (`validate.rs:366, :391, :423, :444, :460`) goes; the crate
   README's V6 or the refusal-envelope section gains one sentence; the
   instance recorded on
   `work/blend/every-escalation-carries-the-coincidence-recourse-first.md`.

## Constraints, binding

- **No behaviour change at the door beyond the rendered text**: every fillet
  that builds today builds bit-identically; every refusal keeps its variant
  and payload; only `PathError::Escalated`'s Display for `fillet_*`
  predicates changes. Dump the profile suites' fillet outputs and refusal
  variants at both SHAs and diff.
- **No new predicate name; the K stream is unchanged** (a Display arm meters
  nothing).
- **One map** for name → sentence; `path.rs` does not copy `validate.rs:719`.
- **Every sentence true at every site its predicate fires** (the A3-2 rule):
  Phase 1's fixtures are the evidence; a predicate whose sentence is false at
  one of its sites gets the sentence corrected, not the site excluded.

## Acceptance

- The Phase 1 table (nine predicates → doors → sentences → today's text).
- The fillet arm; the one map; the site arm retired (or its producer named);
  the followability rows flipped to pins with the census row; the mutant
  table; both differentials clean; hosted CI green.

## Out of scope

The nine predicates' bands and lever arms; the shared
`COINCIDENCE_RECOURSE`'s wording at junction keys (the class item stays
open for the non-fillet instances); the enclosing class (ratified).

## Review

v6 dual on the frozen head; claims to falsify (verbatim to both reviewers,
with `docs/prompts/reviewer-style-lane.md` by path):

- **C1** Every in-band `fillet_*` verdict reachable through the public door
  renders its tailored sentence and never the coincidence recourse: drive
  each of the nine in band yourself (the row's fixtures are the
  implementer's, not your oracle) and read the text off the error.
- **C2** Every rendered sentence is followable at the site it fired: follow
  each and show the request it endorses builds and validates.
- **C3** `EscalationSite::Fillet` had no producer (or the one Phase 1 named
  is real): grep every constructor; and nothing else read the retired arm.
- **C4** Every fillet that built at the merge base builds bit-identically
  and every refusal keeps its variant and payload (re-run both
  differentials); the K count per fillet is unchanged.
- **C5** There is one map (find a second spelling of name → sentence
  anywhere in `crates/profile`), and the Display's dispatch order cannot
  hand a `fillet_*` name to the stored-form or junction arms.
