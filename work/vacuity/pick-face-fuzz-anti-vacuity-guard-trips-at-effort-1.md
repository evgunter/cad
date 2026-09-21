---
id: pick-face-fuzz-anti-vacuity-guard-trips-at-effort-1
kind: issue
title: review_gui1_r1's random-ray oracle sweep asserts "some draw hit the cube" over 120 integer rays at effort 1 — a probabilistic guard that reds CI on a bad seed
status: open
opened: 2026-09-08
refs: [1617]
priority: P3
cost: E
---

Filed by the S-MESH orchestrator from MESH-12's landing run (PR 1617, run
34171106411, job `test (interval, eps = 1e-6, 1/2)`), on DOCM's slate
because the file is `crates/editor-core/tests/review_gui1_r1.rs`.

`random_integer_rays_match_the_exact_oracle` draws `fuzz::scaled(120)`
integer rays (origins in [-3, 3]³, directions in [-2, 2]³) against a
unit cube and, after the sweep, asserts `hits_seen > 0` with the message
"no draw hit the cube — generator shape broke". At effort 1 that is a
probabilistic claim, and seed `0x2870e278e5a1ef24` falsifies it:

```
CAD_FUZZ_SEED=0x2870e278e5a1ef24 CAD_TOLERANCE_EPS=1e-6 \
  cargo nextest run -p editor-core --features interval \
  -E 'test(random_integer_rays_match_the_exact_oracle)'
# → panicked at review_gui1_r1.rs:498: no draw hit the cube
```

Reproduces at default eps with the same seed; passes at
`CAD_FUZZ_EFFORT=2`; passes for 24 other seeds. The row's own comment
says anti-vacuity is "structural, not searched (the battery row covers
guaranteed hits)", which is the right rule — the searched guard
contradicts it. The fix is the comment's: drop the searched assertion,
or make the sweep's first draw a known hit so the guard is structural.
`test-utils::fuzz`'s header says effort "can never make the test fail";
here a LOWER effort can, which is the same contract broken from below.

Not MESH-12's fence (the unit touches no editor-core file); disclosed on
PR 1617 and left to its owner.

## Second instance, a second program, a different seed (2026-09-13, WIRE)

Added as evidence rather than a second row. This guard reddened **WIRE
PR 2518's state-sync run** — run `34783289811`, job
`test (eps = 1e-6, 2/2)`, `review_gui1_r1.rs:498`, same message:

```
no draw hit the cube — generator shape broke;
reproduce with CAD_FUZZ_SEED=0x1a9e0f26198e881b CAD_FUZZ_EFFORT=1
```

**A different seed from the one this row was filed on** — `0x1a9e0f26…`
against the original `0x2870e278…` — and a different eps row, which is
what a probabilistic guard looks like from the outside: it is not one bad
seed, it is a distribution.

**The commit it reddened was two markdown files.** The previous run on
the same code (`34782064566`, head `01d8dbca8`) was green; the only diff
to the red head was two `work/wire/*.md` closing sections. **Re-running
the failed job on identical code passed**, which is the decisive
instrument: the variable is the seed, not the tree.

What that cost, and why it is worth recording on the row rather than
just in a log: the red arrived on a **docs-only state-sync commit at the
end of a unit**, which is the point where an orchestrator is deciding
whether to merge. A guard that reds there spends a reviewer's attention
on an already-answered question, and the honest reading of a red on a
two-markdown-file diff is "something is wrong with the gate", which is
exactly the wrong lesson to teach.

The tree already carries the rule this row asks for:
`crates/test-utils/src/vacuity.rs` says an anti-vacuity claim is
**"stated against the floor of the dial, never"** above it — so the fix
has a written convention to land on, not just a judgement call.

## Re-homed (2026-09-13)

Moved from `work/docm/` to `work/tint/` at DOCM's exit sweep (`docs/DOC-LEDGER.md`,
sweep 14): a probabilistic or once-flaky test guard is S-TINT's (test-suite integrity). Id, body and header are unchanged; the directory is the
claim (`work/README.md`). Any `## Home` section above is superseded by
this line and is kept as the record of why the file was where it was.

## Re-derived (2026-09-15, lane B)

**VERDICT: REPRODUCES** — the searched guard, its message, and the comment
that contradicts it are all intact and unedited. No test was run.

Re-derived by name in `crates/editor-core/tests/review_gui1_r1.rs`,
`random_integer_rays_match_the_exact_oracle`:

- the draw is still `let total = fuzz::scaled(120);` over
  `let mut rng = fuzz::start("review r1: pick_face exact-oracle sweep");`
  — 120 at the floor of the dial, as filed;
- origins are still `rng.below(7) as i128 - 3`, i.e. integers in [-3, 3]³;
- the guard is still

```
    // Anti-vacuity is structural, not searched (the battery row covers
    // guaranteed hits); still, a sweep where nothing ever hit would be
    // a broken generator worth hearing about.
    assert!(
        hits_seen > 0,
        "no draw hit the cube — generator shape broke; {}",
        fuzz::replay()
    );
```

So the row still states the rule and then breaks it in the next statement,
and the failure message the two CI reds quote is unchanged verbatim.

**The written convention the row lands on is also unchanged.**
`crates/test-utils/src/vacuity.rs`'s module doc still says an anti-vacuity
claim under `CAD_FUZZ_EFFORT=1` is *"stated against the floor of the dial,
never"* above it. Nothing has been added at the `review_gui1_r1.rs` site to
reconcile with it.

**What did NOT move.** The fix the row proposes — either deletion of the
searched assertion, or a first draw that is a known hit — has not landed in
either shape. `hits_seen` is incremented at exactly one place, inside the
sweep loop; there is no seeded structural hit before it.

**Sibling row.** `work/tint/random-integer-rays-search-trips-at-eps-1e-6-on-one-run.md`
is about the same `#[test]`. This lane could not establish whether that
flake was THIS assertion — see that row's own re-derivation — so the two
should not be merged on a reading alone.

**Blind spot.** Reproduction is established by reading the source, not by
re-running the two seeds. Whether `0x2870e278e5a1ef24` and
`0x1a9e0f26198e881b` still falsify the guard on today's tree is unverified;
the generator's shape is byte-identical to the filing, so they very likely
do, but nothing here measures it.

**Recommendation:** do not close; this is the cheapest live row on the
slate and it has now reddened two programs' gates on unrelated diffs.

## Third instance, a third seed, a third eps row (2026-09-15, PORT)

Added as evidence rather than a second row, and it is the instance the
sibling row's re-derivation asked for: **this one carries the message.**

PORT's `msrv-floor-equality-gate` run `34997969247`, job
`test (eps = 1e-12, 1/2)`, `crates/editor-core/tests/review_gui1_r1.rs:498`:

```
no draw hit the cube — generator shape broke;
reproduce with CAD_FUZZ_SEED=0xf30b717118986019 CAD_FUZZ_EFFORT=1
```

Three seeds now — `0x2870e278…`, `0x1a9e0f26…`, `0xf30b7171…` — across
**three different eps rows**: the filing was `(interval, eps = 1e-6, 1/2)`,
WIRE's was `(eps = 1e-6, 2/2)`, this is `(eps = 1e-12, 1/2)`. The guard is
not correlated with a lane, an eps row or a shard, which is what the row
already says a distribution looks like from outside and is now the third
reading of it.

**The diff it reddened was a shell script and a YAML step.** PORT's change
is `scripts/gates/msrv-floor-equals-channel.sh` and its two calls in
`.github/workflows/ci.yml` — it compiles nothing, links nothing and cannot
reach `editor-core`. The other eleven `test (…)` jobs of the same run were
green on the same tree, including `test (eps = 1e-12, 2/2)`, the other
shard of the same eps row. This is WIRE's "reddened a docs-only commit"
one step further out: the tree under test was not merely unrelated to the
failure, it was **not Rust at all**.

**Re-run by another name.** The instrument WIRE used — re-run the failed
job on identical code — was not available to this lane (the API returned
403 for `rerun-failed-jobs`), so it was bought the other way: the next
commit on the branch added **one markdown file and nothing else**, and its
run `35001465730` was green in all 39 jobs, `test (eps = 1e-12, 1/2)`
included. Same Rust tree, new seed, green. The variable is the seed.

**For the sibling row.** `work/tint/random-integer-rays-search-trips-at-eps-1e-6-on-one-run.md`
asks which of two assertions its 2026-09-09 flake was, and says nothing in
the tree distinguishes them. This instance does not answer that question
for that run — a message this row's instance carries is not a message that
one's had — but it does raise the prior: three of three fully-messaged
reds of this `#[test]` are the searched anti-vacuity guard, and zero are
oracle disagreements.
