# BLIND — the kernel-discipline gates' matchers (plan)

Opened 2026-10-26 by GUARD's split (`work/guard/log.md`, that date).
Live state is `work/blind/log.md`'s tail and the item files beside this
plan, never this file.

Branch prefix: **`blind/`** — unit branches `blind/<unit>-<slug>`.
Away-channel tag `(BLIND orchestrator)`. A/B ordinal band
**BLIND = 15600–15699**, claimed on this program for bookkeeping.

## Charter

A discipline gate is a regex or awk matcher standing in for a rule the
type system does not hold: no compound bracket bound outside the
allowlist, no interval squared by multiplying two copies, no production
bit-identity consumer, no decision on a reporting margin, no
`register_equal` outside its constructor sites, no ambient environment
read in kernel source. **Each row is a spelling of the policed thing
that the gate's matcher cannot see**, or one it refuses that is not the
policed thing. The fix is almost always the same shape: widen the
matcher or its census register, plant the escaped spelling as a
selftest case, give every new hit a disposition, and state in the gate's
KNOWN GAPS what the widened matcher still cannot match.

The shared reader and the selftest harness these gates run on
(`scripts/gates/lib.sh`) are GUARD's, not this program's. A row here that
finds the harness is the obstacle files on GUARD.

## The slate

| item | pri | cost | what it is |
| --- | --- | --- | --- |
| `reporting-margin-gate-misses-comparisons-of-two-read-readings` | P2 | E | `reporting-margin-door.sh` counts mint sites; a production `==` of two classifier-minted readings passes |
| `self-dot-has-no-gate-the-interval-square-one-cannot-see-it` | P2 | E | `interval-square-allowlist.sh` sees `x * x`, not `x.dot(x)` or `dot(a, a)` |
| `the-bit-identity-consumer-gate-cannot-see-a-to-bits-read-of-bounds` | P2 | E | `bit-identity-consumer.sh` misses `.to_bits()` on `lo()`/`hi()` |
| `G4` | P3 | M | retire `profile`'s `ArcCarrierScalar`; reds `bounds-allowlist.sh` by design |
| `D212` | P3 | D | the gate-side half of `G4` (GAP 3 text, roster entry, `arc_fillet.rs` filter); rides with `G4` |
| `S210` | P3 | D | sole `T: Bounds` / `T: Enclosure` doors have a rule and no instrument; #701 is its twin |
| `S41` | P3 | E | the poisoned-enclosure clamping idiom has three spellings and no lint |
| `census-bracket-read-gate-is-blind-to-validate-child-modules` | P3 | E | the bracket-read census counts `validate.rs` alone, not its `validate/` children |
| `register-equal-allowlist-exempts-a-whole-file-and-hides-test-calls` | P3 | E | a whole-file definition home hides thirteen test calls in `sym.rs` |
| `no-ambient-env-gate-cannot-see-temp-dir` | P3 | E | `no-ambient-env.sh` misses `env::temp_dir(`, `current_dir(`, `home_dir(`, `set_current_dir(` |
| `bounds-allowlist-fires-on-a-binding-pattern-over-an-enclosure-variant` | P4 | E | the `Enclosure` suffix matches a variant name in a match pattern |
| `tour-scalar-trait-bundles-certifiedbounds-outside-the-allowlist-scan` | P4 | E | `demos/tour`'s compound bound sits outside the gate's `crates/*/src` root |

## Order

The three P2 rows first: each is cheap, has its widened pattern written
in its body, and closes a hole through which a wrong answer can land
unseen. Then the cheap P3 widenings (`no-ambient-env`, `register-equal`,
the bracket-read census, `S41`'s clamping lint). `G4` with `D212` riding
it is one unit that crosses into `profile` by announced seam. `S210` is
the tail: its own body says the real instrument may be a rustc or
rust-analyzer driver, which is a design weighing before it is a unit.

## Review posture

Infra-only, as GUARD: one style review per unit against
`docs/prompts/reviewer-style-lane.md`, no A/B row. A unit that changes
what a gate ACCEPTS takes a second correctness reviewer, because a
widened gate that is wrong is silent — and most units here change what
a gate accepts.
