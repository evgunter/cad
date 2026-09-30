---
id: r1-dual-digest-ladder-collides-and-has-no-keeper
kind: issue
title: The R1 dual digest's hand-numbered tag ladder collides at 24, beside a comment claiming the opposite
status: open
opened: 2026-09-12
priority: P3
cost: E
---


Filed 2026-09-12 by the S-TCOST orchestrator out of the style review of
PR 2433 — specifically out of the reviewer brief's Q8, the once-per-review
end-to-end read of the largest file touched. Neither defect below is in
that PR's diff; both were invisible to every per-unit review of the file
because no process here ever reads a whole file. Routed to S-TINT: a
hand-kept enumeration that nothing keeps, and a one-shot artefact with no
schedule, are that program's two named shapes and neither is a cost
argument.

## Defect 1 — the tag ladder collides at 24, and the comment denies it

`crates/editor-core/tests/r1_dual_probes.rs` carries a hand-numbered tag
ladder: every `ValuePayload` / `DatumValue` variant digests a `u64` tag
before its fields, so two structurally different values cannot collide in
the digest. **Verified directly on PR 2433's head — two variants both
write `d.u64(24)`:**

- `ValuePayload::Datum(DatumValue::AxisInPlane { .. })`, whose own
  comment introduces it as *"Tag 24, appended"*;
- `ValuePayload::MeasureUnavailable { .. }`.

And the second carries a comment asserting exactly what is false:

> *A measure with no value at this scalar digests as the ABSENCE, **at
> its own tag**: two passes that both failed to measure agree, and
> neither agrees with a pass that measured something.*

It is not at its own tag. It shares one with `AxisInPlane`. This is
`docs/prompts/reviewer-style-lane.md`'s Q2 in its sharpest form — a
comment asserting an invariant that nothing enforces, and that the code
five lines away already violates.

**The impact is small today and that is not the finding.** The digest is
an f64-vs-`Dual64` differential over one document, so both lanes produce
the same node kinds and a collision cannot separate them. The finding is
that a ladder with no keeper has now collided once, silently, and the
next collision is between variants that a real divergence CAN separate.
The ladder is the guard; nothing guards the ladder.

**What it asks for:** give the tag ladder a keeper — a test that asserts
the tags are distinct, which is a dozen lines and is the executable form
of the comment that is currently wrong — then fix the collision and the
comment. `work/tint/plan.md`'s standing rule applies: *a census has one
executable home and every other site points at it.*

## Defect 2 — a one-shot instrument re-homed without the schedule it owes

The same file carries the deep-digest instrument (`D`, `body_deep`,
`eval_deep` — roughly 200 lines), re-homed here when the one-shot
merge-base differential that carried it expired.

`memories/test-suite-cost.md` requires a one-shot comparison artefact to
be **deleted, or to name in-file the future comparison that schedules
it**. The note at the top of the instrument names neither: it explains
where the code came from, not what will read it next.

So ~200 lines of test-only machinery sit in the tree with no stated
consumer. It may well have one — the file's own rows use it — in which
case the fix is the sentence saying so, and the sentence is the whole
deliverable. What is not acceptable is the current state, where a reader
cannot tell a live instrument from an expired one.

## The class

Both defects share a shape worth sweeping for rather than fixing twice:
**a hand-kept ladder, census or enumeration inside a test file, where the
only statement of the invariant is a comment.** S-TINT's charter already
names the family (*"a loud-skip marker whose row list is hand-kept in
eight files, each copy admitting in its own rustdoc that it goes stale
silently"*), and a digest tag ladder is the same object one crate over.
Where to look: any `d.u64(<literal>)` / `hasher.write_u64(<literal>)`
ladder over an enum's variants, anywhere in `crates/*/tests/`. That
census does not exist; neither this row nor the review ran it.

## Re-derived (2026-09-15, lane D)

**VERDICT: REPRODUCES** — both defects, verbatim — and the class the row
says was never censused turns out to have **three members, all three
carrying the same collision and the same false comment**.

### Defect 1 — still collides at 24, comment still denies it

**Command.**
`grep -noE 'd\.u64\([0-9]+\)' crates/editor-core/tests/r1_dual_probes.rs | sort -t'(' -k2 -n | uniq -c`

The ladder runs `0,1,2,3,10..24`. Exactly one duplicate: **tag 24 twice**.

- `:217` — `ValuePayload::Datum(DatumValue::AxisInPlane { … })`,
  introduced at `:208` by the comment *"Tag 24, appended: both spellings
  of an in-plane axis, so a drift in the numbers a revolve actually
  consumes cannot hide behind the lift."*
- `:276` — `ValuePayload::MeasureUnavailable { .. } => d.u64(24)`,
  under the comment that is false: *"A measure with no value at this
  scalar digests as the ABSENCE, at its own tag: two passes that both
  failed to measure agree, and neither agrees with a pass that measured
  something."*

(Tags 0 and 1 also appear three times each, at `:153`/`:176`/`:249` and
`:155`/`:178`/`:251`. Those are a different, nested enumeration, not the
`ValuePayload` ladder — they are not the collision.)

**No keeper.** The file's six `#[test]` rows are
`r1_dual_value_channel_matches_f64_including_carriers`,
`r1_dual_product_matches_f64_including_carriers`,
`r1_two_seeds_over_a_shared_subgraph_separate_exactly_on_the_cone`,
`r1_no_value_only_key_collision_search`,
`r1_e2e_consumer_drive_at_dual64`,
`r1_is_dl3s_measured_problem_reproducible`. None asserts tag
distinctness.

### Defect 2 — still no scheduled consumer

The instrument (`struct D` at `:83`, `body_deep` at `:133`, `eval_deep`
at `:168`) is intact, and the module header at `:32-35` still explains
only provenance: *"The deep-digest instrument (`D`, `body_deep`,
`eval_deep`) lives here since the one-shot merge-base differential that
first carried it (`r1_mb_diff`) expired with its comparison, per its own
in-file note."* No sentence names a future comparison that schedules it.

### The census the row says does not exist — run here

**Command.**
`grep -rnoE '\b(d\.u64|write_u64|h\.u64|hasher\.write_u64|\.write_tag)\(([0-9]+)\)' crates/*/tests/`

Three files in `crates/*/tests/` carry a literal-tag ladder, and they
are **the same ladder three times**:

| file | literal tags | duplicate |
|---|---|---|
| `crates/editor-core/tests/r1_dual_probes.rs` | 24 | tag 24 x2 |
| `crates/editor-core/tests/r2_m10_di_probes.rs` | 22 | tag 24 x2 (`:288`, `:347`) |
| `crates/editor-core/tests/m10_di_dual_corpus.rs` | 22 | tag 24 x2 (`:202`, `:261`) |

All three have the identical `AxisInPlane` / `MeasureUnavailable` pair on
24, and `grep -rln 'at its own tag' crates/` returns exactly those three
files — **the false comment is copied too**. So the row's "what it asks
for" is three edits, not one, and its own class paragraph is answered:
the ladder is not a one-file object, it is a hand-copied one, which is
why nothing keeps it.

**Blind spot.** The census matched five literal spellings of a tag
write; a ladder that writes its tag through a named constant, an
`as u64` cast of a discriminant, or a differently-named hasher method
would not appear. `crates/*/src` was not swept — the row scopes the
class to `crates/*/tests/`.

**Nothing here needed a run**; the collision is a reading fact.

**Recommend: keep open, widen to the three files**, and note that a
keeper asserting distinctness has to be written once and shared, or the
fix reproduces the defect it closes.
