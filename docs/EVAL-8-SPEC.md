# EVAL-8 — under the pinned lift the op reuses the pre-pass's validated form: a decision is made once on a node's behalf (spec)

**Program:** EVAL (`work/eval/plan.md`, unit 8). **Item:**
`work/eval/profile-node-log-holds-the-f64-validation-twice-under-the-pinned-lift.md`
(EVAL-7's residue).
**Track:** E with a **correctness arm**: the unit changes what a Profile
node's log holds (and therefore the certification keys and the eps-audit
populations) a second time after EVAL-7; one style review and one
correctness review, fix pass, record-at-merge. No A/B draw.
**Branch:** `eval/8-validate-once`. **Difficulty:** M (a small diff behind
one new door in another program's glob, and a re-baseline).

## The claim

**Under `ProfileLift::Pinned` the op re-decides what the pre-pass already
decided.** `prepare_profile` (`crates/editor-core/src/eval/wire.rs`)
validates the authored `Profile<f64>` at f64 and keeps `validated_f64`;
the `Pinned` arm of the op (`wire.rs`, `ProfileLift::Pinned =>`) then
embeds the SAME `profile_f64` at the lane scalar (`anchor::embed_profile`,
every coordinate `T::from_f64`, exact) and calls `validate(tol)` again.
Since EVAL-7 both runs land on the node's log, so under the pinned lift
at the build scalar the log holds each validation decision twice
(`kstats_bracket_rows::…pre_pass_and_the_ops_decisions_follow` pins
`pre[6..] == op`), `resolve::vdiff`'s per-predicate populations are
doubled, and `m4_pr6_eps_diff.rs` reports one physical flip as `count:
4`. EVAL-7 stated this honestly at the Pinned arm and in `vdiff`'s doc;
this unit removes the second decision.

The op's validation under `Pinned` is REDUNDANT, not merely repeated: the
lift is exact (`from_f64` on every coordinate), the validation is 2-D and
reads nothing of the lane but point values, so at every scalar the
verdict sequence equals the f64 one (D9). What the op needs from
`validate` is the VALUE — a `ValidatedProfile<T>` — and that is a lift of
`validated_f64`, not a re-derivation. Under `Guided` the op resolves the
lane program to different values and its validation is genuinely its
own; nothing changes there.

## What lands

1. **One door in `crates/profile`, by announced seam to S-BOOL** (their
   glob; the orchestrator's log line and the PR body announce it):
   `ValidatedProfile<T>::map<U: Real>(self, f: impl Fn(T) -> U) ->
   ValidatedProfile<U>` — the per-coordinate lift of the canonical form
   (plane, loops, segments, every stored scalar) with the canonical-form
   invariants preserved by construction (a lift maps coordinates; it
   reorders nothing). Its doc says exactly what it does NOT re-check and
   why a lift needs no re-validation (the invariants are combinatorial
   and order-preserving under any injective coordinate map; the
   classification predicates were decided at f64 and the lift is exact
   under `from_f64`). If `ValidatedLoop`/`ValidatedSegment` need their
   own `map`, they are private to the crate and follow. A second door,
   `with_plane(self, plane: SketchPlane<T>) -> Self`, because the plane
   is "passed through from the input (validation is 2-D; the plane is
   conventional data)" per its own doc and the op's plane may be the
   lane's derived frame.
2. **The `Pinned` arm reuses the pre-pass's form:**
   `pre.validated_f64.clone().map(T::from_f64).with_plane(plane)` where
   `plane` is what the arm computes today (`placement.map(T::from_f64)`
   or `frame_plane_lane(..)`). `embed_profile` loses its one production
   caller (the `Pinned` arm) — check `rg embed_profile` and retire it if
   nothing else reads it, in this unit (the profile-embed-twice issue in
   `work/issues/` narrows accordingly; say so there by orchestrator, not
   by the lane).
3. **The log holds each decision once.** The Profile node's log under
   `Pinned` at every scalar is the pre-pass's 75 (the part fixture);
   `kstats_bracket_rows`' count map moves (`profile 144 → 75`, the sum
   accordingly), the order row's `pre[6..] == op` pin GOES (it pinned
   "today's double run pending this unit"), `m4_pr6_eps_diff.rs`'s
   populations return to their pre-EVAL-7 values (`count: 2`, `2/0`,
   `6/0`, …) and the golden rule sentence names this as the ratified
   re-pin it is, `m10_6_certifying_keys.txt` re-blesses once more
   (`M10_6_BLESS_CERT_KEYS=1`), and `vdiff`'s module doc drops the
   doubling sentence EVAL-7 added. The PR body tables EVERY moved value
   with before/after, as EVAL-7 did.
4. **Nothing else moves**: a base-vs-head dump (EVAL-7's shape: per node
   verdict count, escalation count, `VerdictVectorKey`; every corpus
   document at f64 and interval, `Pinned` and `Guided`) shows only
   Profile rows under `Pinned` moving, every `Guided` row byte-identical.
   Geometry is untouched by construction (the validated form's VALUES
   are the same bits either way — assert it: the lifted form equals the
   re-validated form field for field, as a row that runs both on the
   corpus and compares).

## Correctness claims (the correctness arm)

1. **Value identity**: for every corpus document under `Pinned` at f64,
   `Dual64` and interval, the lifted validated form equals the
   re-validated form (plane, loop count, per-loop segment count, every
   coordinate's bits) — a row that computes both.
2. **Log identity**: the Profile node's log under `Pinned` equals the
   pre-pass's log exactly (length and sequence), at every lane/eps point.
3. **`Guided` untouched**: every `Guided` row of the dump is byte-identical
   base to head.
4. **The re-pins are exactly the inverse of EVAL-7's doubling** for the
   eps-audit populations (each halves back to its pre-EVAL-7 value — the
   reviewer diffs against the value in `git show <pre-EVAL-7-sha>:crates/editor-core/tests/m4_pr6_eps_diff.rs`).
5. **The door is total**: `map` on a validated profile with holes, with
   arcs, with tangent joints, and with the canonical winding preserves
   the canonical-form accessors' answers (outer first, holes in input
   order, segment kinds) — a row over the profile crate's own fixtures,
   living in `crates/profile/tests` by the same seam.

## Sweep

`rg -n 'validate\(tol\)|\.validate\(' crates/editor-core/src` — every
validation call in EVAL's files with a disposition (the op's `Guided`
arm and any other are genuine decisions; a second re-validation of an
already-validated value is this unit's class). Blind spot: a validation
reached through a helper not named `validate`.

## Review

Style lane per `docs/prompts/reviewer-style-lane.md`; correctness lane
takes claims 1–5 as MAJOR-class. Emphasis: the door's doc argues for a
lift needing no re-check — read it adversarially (Q2: a justification
for skipping a check is exactly where a hidden precondition hides; name
any coordinate map under which the canonical form's invariants could
fail and whether `from_f64` is one).

## Records at merge

`work/eval/log.md` entry with the S-BOOL seam and the M10 re-baseline
announcements; the item `closed` with `pr:`; this spec deleted per
`docs/DOC-LEDGER.md`.
