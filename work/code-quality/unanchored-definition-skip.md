---
id: unanchored-definition-skip
kind: issue
title: bounds-allowlist.sh's CertifiedBounds definition skip is unanchored, so the moved lines stay exempt while the subject check goes silent
status: open
opened: 2026-09-03
track: K
refs: [D106]
---


## Was

unrowed. Raised by the style review of PR #1639 (`D106`/`D208`/`D209`,
the Track K gate-prose unit) and measured by this lane; it is a
gate-MECHANISM change, so it was not fixed in that prose PR.

## Finding

### The definition skip is not anchored to its file, and the check that was supposed to cover that goes silent in exactly the case that needs it

**The mechanism.** `scripts/gates/bounds-allowlist.sh:224-225` (all `bounds-allowlist.sh`
line numbers here are relative to `c4a7ea11`, this branch's head) drops
two
lines from the scan by exact text and **nowhere else in the pipeline is
the path constrained**:

```
gate_grep -vE ':[0-9]+:pub trait CertifiedBounds: Bounds \+ CertifiedEnclosure \{\}$'
gate_grep -vE ':[0-9]+:impl<T: Bounds \+ CertifiedEnclosure> CertifiedBounds for T \{\}$'
```

The exempted text is `crates/geom-core/src/real.rs`'s alias definition.
The skip exempts it **anywhere under `crates/*/src`**.

`gate_definition_skip_subject` (`scripts/gates/bounds-allowlist.sh:204-213`)
is the file's answer to exact-text brittleness — it reds when the two
lines are no longer in `real.rs` verbatim — but it opens
`[ -f "$f" ] || return 0` (`:206`). So a **renamed or moved** `real.rs`
turns the subject check into a silent no-op *while the unanchored skip
keeps exempting the two lines at their new path*. The guarantee item 3
of the header claims — that reverting the skip to a name anchor is
refused one step earlier — evaporates with no red anywhere.

**The measurement** (this lane, at `c4a7ea11`, via the gate's own
`--root` fixture harness; a clean tree plus one planted crate):

| Fixture | `crates/planted/src/lib.rs` | `real.rs` | Result |
|---|---|---|---|
| A | both `CertifiedBounds` definition lines | absent (renamed) | `OK … 2 source files scanned`, exit 0 |
| B | both `CertifiedBounds` definition lines | present, verbatim | `OK … 3 source files scanned`, exit 0 |

Fixture A is the rename case: subject check no-ops on the missing file,
skip still fires on the moved text, gate green. Fixture B shows the skip
is not conditioned on `real.rs` at all — the same two lines are free
text in any crate in the tree.

**The twin, and it is strictly better.** `scripts/gates/no-extra-real-bounds.sh:83-90`'s
`gate_sealed_skip_subject` is the same mechanism for `SpanLocate`, and
it carries the anchor this one lacks: `SEALED_HOME_RE`
(`no-extra-real-bounds.sh:79`) is joined to the declaration in the
filter at `:133`, so the skip applies to that text **in that file and
nowhere else**. Its header reasons the failure out at `:63-69` —
*"An unanchored skip would exempt the same declaration copied anywhere
in the tree, and would turn `locate.rs` being renamed into a quiet
no-op"* — and it holds the property with a planted case
(`plant_sealed_decl_elsewhere`, `:295-298`). Measured the same way:

| Fixture | file | Result |
|---|---|---|
| C | `crates/planted/src/lib.rs` carries `pub trait SpanLocate: sealed::Sealed + Real {` | **RED**, `found extra bound(s) on Real above`, exit 1 |
| D | the same declaration at `crates/geom-core/src/spline/locate.rs` | OK, exit 0 |

**Neither file cites the other**, so the reasoning at
`no-extra-real-bounds.sh:63-69` — written about precisely this failure —
never reached its sibling one directory over.

**The fix shape**, not prescribed here: give the definition skip a
`DEFINITION_HOME_RE` anchor in the `SEALED_HOME_RE` shape, keep the
subject check as the other half, and plant the moved-definition case the
way `plant_sealed_decl_elsewhere` does. `[ -f "$f" ] || return 0` can
then stay, because an anchored skip makes the rename loud on its own.

**Plausibly a class.** The reviewer's judgement, and this lane did not
sweep it: the other exact-text skips under `scripts/gates/` want the
same read — is the skip joined to a path, or is it free text anywhere
the scan reaches? Two instances are known (this one unanchored, the
`SpanLocate` one anchored); a taker should enumerate the rest rather
than assume two is the population.

**Not fixed in PR #1639**, deliberately: that PR is a prose unit on this
file's header, and the change here is to the gate's matcher pipeline and
its self-test fixtures — a file a concurrent Track K lane was converting
at the time.
