---
id: loft-path-loses-nine-predicate-families-from-the-probe-stream
kind: issue
title: Retiring the T-validation drops nine profile-validation predicate families from the loft path's Probe sample stream, and the k-lint gate cannot observe a shrunken population
status: open
opened: 2026-09-12
refs: [2409]
---


## Finding

Measured by the full review of PR 2409, which instrumented it rather
than arguing it. Confidence `sure`, the reviewer's.

One minimal two-section loft at `Probe` (`loft_body::<Probe>`,
two-vertex circles r=1.0/0.625), sink installed, both heads:

| head | samples | distinct predicate names |
| --- | --- | --- |
| `main` | 455 | 33 |
| PR 2409 | 397 | **24** |

The 58 that leave are **nine whole families**: `vertex_separation` (8),
`segment_straightness` (8), `arc_diameter_clearance` (8), `arc_span`
(8), `contact_at_shared_vertex` (12), `carrier_circles_identity` (6),
`canonical_order_x` (4), `arc_apex_identity` (2), `loop_orientation`
(2) — i.e. **every profile-validation predicate, gone from this path**,
because the `f64` validation in `skin::validate_sections` records
nothing where the retired `T`-validation did.

## Why the gate cannot catch this, which is the point

`tools/k-lint` flags **margins per row**, so a *shrunken* population can
only ever produce weakly **fewer** flags — a narrowing is invisible to a
threshold instrument by construction. And nothing re-counts predicate
names off the fresh CSV: `tools/k-lint/tests/predicate_roster.rs` reads
kernel **source**, not the sweep. So "k-lint is green" is true and says
nothing about this.

This is the third face of the silent-coverage class that
`memories/agent-lane-operations.md` already collects (after
CONFLICTING-no-run, queued-with-zero-jobs, and green-name-over-skipped-
step): **a green gate over a population that got smaller.**

## Narrowing, not a hole

The nine families survive in the linted CSV through other tour scenes —
`demos/tour/src/bossplate.rs:40` and siblings validate at `S`. So the
distribution K is derived from still sees them; what changed is that the
**loft path** no longer contributes. That is why this is a row and not a
blocker on PR 2409.

## What is owed, and why it is filed rather than fixed in place

`docs/prompts/reviewer-style-lane.md` Q6: a disclosed narrowing owes a
**concretely scheduled followup**, and PR 2409 gave none. This row is
that schedule.

Three dispositions a taker should weigh; the row does not pick one
because the instrument and the cause sit in different programs:

1. **Record the `f64` validation's margins too**, so the loft path keeps
   contributing. Lands in `crates/sweep/` — **S-BOOL's**.
2. **Teach the instrument to count families**, so a population that
   shrinks reddens instead of quietly passing. Lands in `tools/k-lint`
   — **INSTR's** (`work/guard/program.md`'s `keep_out` routes `tools/*`
   there).
3. **Argue it needs neither** — the families are covered elsewhere and
   the loft path was never their only source. This is a real answer and
   it is the cheapest, but it has to be *written*, because right now the
   coverage change is recorded nowhere a future reader would find it.

WIRE holds the row because WIRE's unit caused the narrowing; the fix
almost certainly does not land here.
