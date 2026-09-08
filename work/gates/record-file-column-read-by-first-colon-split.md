---
id: record-file-column-read-by-first-colon-split
kind: issue
title: two record readers outside the skips read the FILE column by splitting at the first colon
status: open
opened: 2026-09-08
refs: [bounds-allowlist-select-cuts-at-the-first-colon]
---

## Finding

Turned up by `bounds-allowlist-select-cuts-at-the-first-colon`'s sweep.
That row's class is the whole-file SKIP; this is the same misreading of
the FILE column at two sites that are not skips, so neither its grep nor
the sibling row's could see them. The sweep pattern was

    grep -rn 'cut -d:\|%%:\*\|s/:\.\*//\|IFS=:\|-F: ' scripts/gates/*.sh

and what it cannot match is a column read done inside an `awk` body
without `-F:`, or one done by `sed` with a differently spelled address —
so this hit list is the sweep's, not a census.

Its other hits are `probe-suite-census.sh:492-494,602,797,811,970,976,
1173` and `:539,687,1178`, which split that gate's OWN colon-joined
entry strings (`mode:crate:module:want`) and not records; they are
not-this-class.

**1. `scripts/gates/viewer-module-kinds.sh:469`** — the union of the
line arm and the window arm is deduplicated on a site key built as

    awk -F: '{ k = $1 ":" $2 } !(k in seen) { seen[k] = 1; print }'

`$1 ":" $2` is `FILE:LINE` only while the FILE column carries no colon
of its own. For `vocab/forms.rs:x.rs` the key is the path's own two
halves, so every record from that file shares ONE key whatever line it
is on and all but the first are dropped from the union:

    printf 'a/b.rs:x.rs:3: mod one\na/b.rs:x.rs:9: mod two\n' \
      | awk -F: '{ k = $1 ":" $2 } !(k in seen) { seen[k] = 1; print }'
    a/b.rs:x.rs:3: mod one

The direction is BLIND, not exemption: sites vanish from the count the
exception entries are compared against, so a `VOCAB_EXCEPTIONS` count
reads low and the gate can go green over a site nobody argued.

**2. `scripts/gates/lib.sh:1195`, consumed at `:1150`** —
`gate_test_only_mounts` narrows to `mod` declarations with

    gate_grep -oE "$GATE_RECORD_PREFIX_RE.*[[:space:]]mod [a-z_][a-z0-9_]*\$"

`GATE_RECORD_PREFIX_RE` is `^[^:]*:[0-9]+:`, which is the first-colon
reading spelled as a pattern: a record from a colon-carrying path
matches it NOWHERE, so `-oE` emits nothing and the declaration is not
registered as a test-only mount at all —

    printf 'a/b.rs:x.rs:3: #[cfg(test)] mod one\n' \
      | grep -oE '^[^:]*:[0-9]+:.*[[:space:]]mod [a-z_][a-z0-9_]*$'   # no output

The mounted subtree is then read as PRODUCTION by every gate that calls
`gate_production_sources`. `${decl%%:*}` at `:1150` is the same reading
again, and is consistent with the pattern above rather than a second
defect. The lib.sh header at `GATE_RECORD_PREFIX_RE` already names the
shape `…/real.rs:x.rs` — as one of the three things a gate's ANCHOR
rules out, which is the intended direction there. Used as a PARSER the
same expression is a silent drop, and that use is not argued at the
header.

**Population is zero today**: `find crates/*/src -name '*:*'` returns
nothing. A `:` is legal in a path here and in git, which is the whole
premise the sibling rows are built on.

## Why it is worth a row

Same construction value as the row that disclosed it, in the direction
that never cries wolf for site 1 and cries wolf for site 2, and both
want a fixture rather than a claim. Site 2 is `lib.sh`'s: the repair is
either a parser that reads the column the way a record is actually
shaped (up to the first `:LINE:`) or an explicit refusal, and whichever
it is, it is one decision made once for every gate that reads records —
so it is not a rider on a single gate's unit.
