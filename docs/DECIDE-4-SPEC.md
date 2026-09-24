# DECIDE-4 — rule D past the unit bulge: what stands at a bulge that is not 1 on today's tree, the sign-free part, and the bulge's sign (spec)

**Program:** DECIDE (`work/decide/plan.md`). **Item:**
`work/decide/rule-d-reaches-the-unit-bulge-only.md` (P1, cost D;
M10-10's finding, SYM-3's render, SYM-5's re-measure). **Unit:**
`work/decide/DECIDE-4.md`. **Branch:** `decide/4-bulge-reach`, cut
from `props/sign-hull`'s head (`7f3c0cc3f`, SYM-9's merge); the PR
targets `props/sign-hull` and no `main` is merged into it (DECIDE-3's
and SYM-9's tree is where the tier's rules live until PROPS lands it).
**Implementer:** Opus. **Review tier:** DUAL (§Review).

**Read first, in full:** `docs/prompts/implementer-discipline.md`; the
item whole (the two measurements; "What stands (SYM-3)", including the
needle table and the two routes; the measurement patches; the corpus
sweep and its blind spot; "What SYM-5's rule E took");
`crates/editor-core/tests/m10_bulge_interval.rs` (the nominal pins and
their prose, DECIDE-3's paragraph included) and `m10_bulge_renders.txt`;
`m10_10_evidence_interval.rs` (`CAD_M10_10_DOC`, `CAD_M10_10_NEEDLES`,
the dyadic controls); `sym.rs`'s header and `SymRules` in full (rules C,
F, G and the decision read in particular — `signed_root` is the one
shipped-off rule, and `decision_read` is the shipped precedent for a
counted read); `sym/manifest.rs`; `sym/root.rs`; SYM-9's
`sym_9_retry_interval.rs` (the ladder, the leaf instrument).

## The claim

At the unit bulge the arc's two spellings — the carrier's
(`sweep::swept::arc_span = 4·atan|b|`, the profile's `seg.rs` radius
`abs(signed_radius)`) and the pushforward's (`SketchSegment::eval`'s
`4·atan b`) — meet at every sample, because `abs(1)` folds. At any
other bulge they meet only as far as the algebra reaches, and the class
is a third of the tour: every fillet (`tan(θ/4)`), every `Via` or
tangent arc, every `CircleSplit(n ≠ 2)`, and every sub-arc the kernel
mints itself through `SketchSegment::restrict`.

The record shows three residues, and two units have since moved them:

1. **The ring** (the boss's `carrier_matches_mapped_source` 6/54, the
   D-tab's `fl(0.4)` mantissa under every sagitta coefficient). SYM-5's
   rule E took the boss's six. The D-tab at `0.4` still stands on it.
2. **An `abs` over a non-constant argument** (the carrier's radius
   against its own square spelled without the `abs`). DECIDE-3's
   canonical root spells `sqrt(L²)` as `|L|`, the same atom the
   carrier's `abs` mints, and its companion `abs(X)² = X²`
   (`abs_square`) is shipped. Together they took all of the boss's
   on-surface residue (`m10_bulge_interval`'s DECIDE-3 paragraph). The
   D-tab's on-surface rows (27/9/3/1) are unmoved on both spellings,
   and why is not yet rendered.
3. **The sign of a parameter bulge** (`atan|b|` against `atan b`,
   `sqrt(1 + abs(b)²)` against `sqrt(1 + b²)`). No value-free rule
   reads it. It is the one residue that is a design question: route A
   (rule C's clause 3 over `abs`, a counted read) or route B (the
   authoring door's decided turn `σ` handed to the tier).

So this unit measures first what stands on today's tree, then takes
what a value-free rule can take. The sign goes to Ev as a fork only if
the measurement says it is still what blocks.

**Ratified and not re-litigated:** E12; the three rungs; rule G and the
two certified reads (DECIDE-3, Ev's shape 1 on #2970); SYM-9's ladder
and its shipped default; rule F's strict positivity (SYM-8: a
`copysign` at a real zero denotes no function of the value);
`COEFF_BITS = 256` for the first attempt; the ring retry measured and
not shipped (SYM-9).

## Phase 1 — before touching anything (the measurement)

All numbers on `props/sign-hull` at the branch point, with the shipped
set and the drive's default retry (`DEFAULT_SYM_RETRY`), at ε = 1e-9
unless a row says otherwise.

1. **The five bulge documents, rendered.** These are the boss at `2`,
   the D-tab literal and parameter at `0.4`, and the two dyadic
   controls at `0.5`. For each, give:
   - the nominal split, per predicate, for the arc-family predicates
     (`carrier_matches_mapped_source`, `carrier_on_surface_{1,2}`,
     `witness_on_surface_{1,2}`, `carrier_endpoint_{start,end}`,
     `arc_span`, `line_span`, `contact_at_shared_vertex`);
   - the whole-certifying ceiling, shipped and `without_the_algebra`;
   - for **every** still-numeric decision on those predicates, its
     cause, as one of four: **(i)** a freeze (name the node and the
     cause from `sym::profile`: `Terms` / `Degree` / the ring);
     **(ii)** an `abs` or `sqrt` atom whose argument is not a
     constant, standing against a spelling of the same quantity
     without it (sign-free: identically zero for every real value of
     the parameters given `abs(X)² = X²` or a manifest-sign fact);
     **(iii)** the sign of `b` (two atoms for one quantity that only
     the sign relates); **(iv)** other, rendered.

   Present it as one table per document, with the renders trimmed into
   `m10_bulge_renders.txt` as SYM-3 did (this re-takes that file on
   today's tree; its old contents are recoverable from git and need not
   stay).
2. **The needle table re-taken** (`CAD_M10_10_NEEDLES`: `atan(1·abs(`
   and any `abs(`, plain / early / numeric) on the dyadic parameter
   control. Beside it, the two routes counted on today's tree:
   - **Route A**: `signed_root` on, as a local measurement, reverted.
     Give decisions moved per predicate, as `sign_gated`.
   - **Route B**: a local patch spelling the carrier's span
     `4·atan(σ·b)` from the decided `path_arc_bulge` sign, reverted.
     Give decisions moved per predicate.

   Also give each route's cost on the leaf instrument (release).
3. **The class, sampled.** The same per-cause attribution on R2's
   filleted bracket (a measured document whose `.fillet(0.5)` is a
   `tan(π/8)` bulge), restricted to decisions on its fillet arcs. Also
   one count: how many arc-family decisions on the two-hole plate are
   asked of a sub-arc `restrict` minted (bulge `≠ 1`). Report zero if
   it is zero. This answers the item's blind spot (i) on a document
   already measured.

**Stop rules.** If Phase 1 finds no decision in cause (ii) on any
document, and no cause-(iv) decision a value-free rewrite takes, Phase 2a
is empty. If it finds no decision in cause (iii), Phase 2b is empty. If
both are empty, the unit closes at Phase 1: the PR is the measurement,
the item re-states what stands (every cause-(i) freeze filed or pointed
at its existing row), and the review tier drops to a single STYLE
review (§Review).

## Phase 2a — the sign-free part

Take the cause-(ii) and value-free cause-(iv) decisions Phase 1 found,
by the narrowest rewrite that takes them. It must:

- be an equality of reals at every point clause 1 admits, with no value
  read, so a zero through it is a THEOREM;
- sit in the early walk behind the rule it extends (G's companion, F's
  manifest sign, or A's per-node walk — whichever the renders show is
  missing), gated by that rule's dial or a new named one;
- carry a soundness argument in the module header of the file it
  lives in.

The item's Patch C (`abs(X) = X` on a non-negative `X`) was narrowed
to strict positivity by SYM-8 for `copysign`'s sake. A rewrite over
`abs` alone, where `abs(0) = 0` makes non-negativity sound, must argue
that the `copysign` hazard does not reach it, not assume it.

Acceptance:

- Every pinned split and ceiling that moves is re-baselined and said,
  in the row's prose and the PR body. This covers `m10_bulge_interval`,
  the six measured documents' pins, `decide_3_split_rows_interval` and
  SYM-9's rows.
- `numeric` falls or holds on every measured document. A decision
  that moves from `symbolic_zero`/`registered` to `numeric` is a
  finding and goes in the PR body with its render. It is not a reason
  to withhold the rule if the rule is right: Ev's standing words are
  "never skip out on a change that would make the code better because
  it would require rebaselining", and the code that goes in makes no
  concession to skipping one.
- A negative row: an `abs` the rewrite must NOT fold (a sign-carrying
  argument), pinned opaque.
- The cost on SYM-9's leaf instrument (release), with and without the
  rewrite, disclosed against the 1.6 s line. This is a disclosure, not
  a gate.

## Phase 2b — the bulge's sign: Ev's fork

If Phase 1 counts cause-(iii) decisions, **stop before implementing
either route.** Report both routes' counts and costs (item 2) and the
changes each makes:

- **Route A** turns on a shipped-off dial whose zeros are `sign_gated`.
  It is the decision read's shape at `sqrt`/`abs`.
- **Route B** changes what the arc constructor states. Either the span
  spelled from `σ`, or a registration `register_equal(abs(b), σ·b)` at
  `path_arc_bulge`'s decision, so its zeros are `registered`.

Give a recommendation with its reason. The orchestrator writes the fork
into the item and opens an `[ev]` PR (`needs_ev: true`). **Phase 2a
does not wait on it:** when 2a is green it goes to review. If the ruling
has not arrived by then, 2b becomes its own unit on the ruling, and this
unit closes on 2a.

## Scope

- Files: `crates/geom-core/src/sym.rs` and `sym/*.rs` (the rewrite and
  its header section), tests under `crates/editor-core/tests/` and
  `crates/geom-core`'s own, the item and unit files, and
  `m10_bulge_renders.txt`.
- Measurement patches (Phase 1 item 2) are local and reverted. Their
  diffs go in the item, as SYM-3's did.
- No change to a constructor (`sweep`, `profile`, `sketch`) before Ev's
  ruling. No change to `COEFF_BITS`, to the retry ladder's default, or
  to `signed_root`'s shipped value.

## Review

**DUAL**, recorded at spec time in `work/decide/log.md`. The unit
changes what the tier decides across a family of documents (a third of
the tour), and a value-free rewrite over `abs` is exactly where the
record has found unsound folds before (SYM-8's `copysign` narrowing;
DECIDE-3's side-condition source). Class for the log: **M / NUMERIC**.

The dual runs under `docs/DUAL-REVIEW-PROTOCOL.md` at the commit that
last touched it on `main`. That means:

- two Opus reviewers, concurrently, on one frozen head, with identical
  briefs;
- the fix pass off the adjudicated union;
- the row recorded at merge, numbered in `main`'s merge order.

`docs/DUAL-REVIEW-LOG.md` lives on `main` and not on `props/sign-hull`,
so the row cannot ride this PR as its last commit (rule 8). It rides
the orchestrator's carry PR to `main` instead: the PR that brings the
unit's tracker state across at merge, as SYM-9's A/B row did (#3181).
It is that PR's last commit, and it takes its DR number when that PR
merges.

Claims to falsify:

1. the rewrite's soundness as an equality of reals on clause 1's
   domain, including at a real zero of its argument;
2. no plain theorem re-labelled, no `sign_gated` counted as a theorem;
3. the numbers, meaning Phase 1's attribution and every re-baseline;
4. the cost;
5. plus `docs/prompts/reviewer-style-lane.md` in full.

If the unit closes at Phase 1 (both 2a and 2b empty, or 2a empty and 2b
at Ev), the tier is a single STYLE review instead, and the log says so.

## Landing

Status `review` on `work/decide/DECIDE-4.md` when the PR opens. At
merge the orchestrator closes the unit, deletes this spec (note under
`docs/doc-ledger/`), and closes the item or re-states what stands in
it.
