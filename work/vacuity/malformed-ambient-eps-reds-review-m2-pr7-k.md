---
id: malformed-ambient-eps-reds-review-m2-pr7-k
kind: issue
title: local cargo test — review_m2_pr7_k reds under a malformed ambient CAD_TOLERANCE_EPS
status: open
opened: 2026-08-15
github: 497
refs: [415, 448]
priority: P4
cost: E
---

## From GitHub issue 497

Opened 2026-08-15; 0 comments.

(M8 orchestrator) Adjacent finding from the #415 verification (out of that issue's scope, pre-existing): with a deliberately malformed `CAD_TOLERANCE_EPS=bogus` in the ambient environment, plain `cargo test -p geom-core --test all` reds on `review_m2_pr7_k::invalid_env_k_values_fall_back_and_record_typed` — the same ambient-env-sensitivity class #448 fixed for tolerance_init, in a different suite (tolerance_init itself stays green). Hosted CI never sees it (nextest forks per test); it only bites a local shell that exports a malformed value. Filed so the class has a name if an agent shell ever does that; the #448 self-re-exec probe pattern is the known fix shape. Low priority — no slate claim.

## Home

`work/issues/`: a test-integrity finding in `crates/geom-core/tests` — S-QA's ground, and S-QA is closed; it is not a cost lever, so it does not belong on S-TCOST's slate either.

## Re-homed (2026-09-04)

Moved from `work/issues/` to `work/tcost/` in the tracker-wide
re-home sweep of 2026-09-04 (Ev's direction, in-chat), which read every
open `work/issues/` file against every open program's `paths` and
against the code-quality K–X fences. Id, body and header are unchanged;
the directory is the claim (`work/README.md`). Any `## Home` section
above naming `work/issues/` is superseded by this line and is kept as
the record of why the file was parked there.

## Moved to S-TINT (2026-09-11)

Moved by `git mv` from `work/tcost/` at S-TINT's opening. Id, title and
body are unchanged; the directory is the claim (`work/README.md`).

**Why it moved.** S-TCOST's board was re-sorted on 2026-09-11 against the
repository going public on 2026-09-03 (`work/tcost/log.md`, the
2026-09-11 seam). That sort found this row is not a cost lever in either
currency — it neither shortens the gate's critical path nor saves a
billed minute, and it was never argued on one. It reached S-TCOST by the
tracker-wide re-home of 2026-09-04, which routed rows by PATH GLOB
(`crates/*/tests/*`, `crates/test-utils/*`) rather than by question.
This program is the question it was always about: whether the suite
asserts what it claims to assert.

## Re-derived (2026-09-15, lane B)

**VERDICT: REPRODUCES** — the mechanism is unchanged and the fix shape is
now in the tree at three named sites, none of which is this one. No test
was run.

### The subject, by name

`crates/geom-core/tests/review_m2_pr7_k.rs` still holds
`invalid_env_k_values_fall_back_and_record_typed`, and it still asserts

```
            out.contains("env_errors=1"),
            "invalid K {bad:?} must record a typed env error; got:\n{out}"
```

for each of `["0", "1", "-5", "NaN", "ten"]`.

### Why the ambient sensitivity survives

The file already uses the #448 self-re-exec *pattern* — its module doc
says so (*"re-exec pattern — the `Tolerance` OnceLock commits per
process"*) — but the pattern is not the fix. The fix is environment
SANITISATION, and `run_probe` does not do it:

```
fn run_probe(env: &[(&str, &str)]) -> String {
    let exe = std::env::current_exe().unwrap();
    let mut cmd = std::process::Command::new(&exe);
    …
    for (k, v) in env {
        cmd.env(k, v);
    }
```

`Command` inherits the parent environment and this only ADDS the named
vars, so an ambient `CAD_TOLERANCE_EPS=bogus` reaches the child, the child
records a typed env error for the malformed EPS *in addition to* the one
for the bad K, and `env_errors=1` is false. That is exactly the failure
the issue reports, and the arithmetic explains why the message is a count
rather than a predicate.

The sibling row in the same file, `margin_between_10_and_25_eps_flips_with_k`,
calls `run_probe(&[])` for its baseline and is exposed to the same
inheritance; its assertions are on `k=10` and the outcome string rather
than on the error count, so whether it also reds depends on what a
malformed EPS does to the 17ε margin. Not settled here.

### The fix shape EXISTS in this tree — three sites to copy

`grep -rn "env_remove\|env_clear" crates/*/tests/*.rs`:

- `geom-core/tests/eps_provenance.rs` — `cmd.env_remove(ENV_EPS);` and
  `cmd.env_remove("CAD_AMBIGUITY_K");`
- `geom-core/tests/review_m0_pr2.rs` — `.env_remove(ENV_EPS) // full
  control, even under the CI eps matrix`
- `editor-core/tests/wire_band_cause.rs` — `.env_remove("CAD_TOLERANCE_EPS")
  .env_remove("CAD_AMBIGUITY_K")`

The last is the exact two-line spelling this row needs, on both vars, and
`review_m0_pr2.rs`'s comment states the reason in the general form.

**A sibling with the same gap, found while re-deriving:**
`crates/geom-core/tests/ambiguity_k_env.rs` re-execs with the same
`current_exe` pattern and also has no `env_remove`. Not checked for
whether any of its assertions is count-shaped, so it is a candidate rather
than a finding.

### What the issue got right and what it understates

Right: *"Hosted CI never sees it (nextest forks per test)"* — still true;
the row is an agent-shell hazard, not a gate hazard. Understated: the
issue calls the #448 self-re-exec probe pattern *"the known fix shape"*,
and the pattern is present here while the defect persists — it is the
`env_remove` that is the fix, and saying "the re-exec pattern" is what
would let a reader look at this file, see the pattern, and conclude it was
already applied.

### Blind spot

Not reproduced — reproducing it means running `cargo test` under a
poisoned environment, which this lane did not do. What is established is
that the code path admits it: inheritance is unbroken and the assertion is
on a count that a second env error increments.

**Recommendation:** do not close. It is a two-line fix with three
in-tree models, and the row's "low priority — no slate claim" note is now
out of date: it has a slate.

## Sibling candidate, same re-exec pattern (S-TINT orchestrator, 2026-09-15)

Lane B, while re-deriving this row: `crates/geom-core/tests/ambiguity_k_env.rs`
re-execs through the same `current_exe` pattern as `review_m2_pr7_k.rs`
and likewise never `env_remove`s, so an ambient value reaches the child
the same way. **Not verified as a second instance** — the lane did not
check whether any of its assertions is count-shaped, which is what turns
the leak into a red. A taker on this row checks that file in the same
pass; if it is count-shaped, this row is a class of two and its title
should say so.
