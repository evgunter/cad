---
id: bounds-allowlist-select-cuts-at-the-first-colon
kind: issue
title: bounds-allowlist.sh's per-file SELECT cuts the FILE column at the first colon, so a colon-carrying path rides an allowlist entry
status: open
opened: 2026-09-06
refs: [whole-file-skips-are-hand-spelled-not-anchored]
---

## Finding

Turned up by `whole-file-skips-are-hand-spelled-not-anchored`'s sweep,
which asked for the whole CLASS — a whole-file exemption that is not
pinned to the `FILE:LINE:` shape — rather than for the one spelling that
row's grep matches. This is the same defect in a spelling the anchor
does not fit, and it is the LAST one in this directory.

`scripts/gates/bounds-allowlist.sh:940`:

    hits=$(printf '%s\n' "$records" | gate_grep -v '^$' |
      cut -d: -f1 | sort -u |
      gate_grep -vxF -f <(gate_allowlist_paths))

`cut -d: -f1` takes the FILE column as "everything before the first
colon". A record from a file whose own path carries a colon —
`crates/geom-core/src/real.rs:x.rs:12:…`, legal on this filesystem and
in git — therefore arrives as `crates/geom-core/src/real.rs`, matches
that allowlist entry exactly under `-vxF`, and is dropped. A compound
`Bounds` bound written there is exempt on a ratification that was
granted to a different file.

**Measured, not read.** The same file, twice, over the live tree:

    printf 'pub fn f<T: Decide + Bounds>(_t: T) {}\n' \
      > 'crates/topo/src/boolean/boxes.rs:x.rs'
    scripts/gates/bounds-allowlist.sh          # OK, 183 occurrences

    printf 'pub fn f<T: Decide + Bounds>(_t: T) {}\n' \
      > 'crates/topo/src/boolean/zzz_probe.rs'
    scripts/gates/bounds-allowlist.sh          # ERROR, names the file

`crates/topo/src/boolean/boxes.rs` is an allowlist entry pinned at 4;
the colon-carrying sibling rode its ratification and the pinned count
did not move.

**The count check does not catch it.** `gate_allowlist_counts` at
`:553` selects the same entry's records with
`gate_grep -E "$(gate_record_anchor "$path")"`, which is anchored and
does NOT match the colon path — so the pinned count for `real.rs` is
unmoved and the gate stays green. The two halves of the same gate read
the file column two different ways, and only one of them is pinned.

## Why it is worth a row

Population zero and the direction that never cries wolf, exactly as the
sibling row argued for the six literal skips — the value is
construction. What makes it worth its own row rather than a line in
that PR is that the repair is NOT the sibling's: `gate_record_anchor`
builds a record pattern, and this select works over a de-duplicated
column of paths, not over records. The two candidate repairs are to
drop the `cut` and select with the anchor alternation the counts
already build, or to read the FILE column the way `lib.sh`'s
`GATE_RECORD_PREFIX_RE` defines it. Either is a change to how the scan
picks its hits, in the file D102 and D103 both landed in, and it wants
its own fixture: a compound bound planted at `<entry>.rs:x.rs` must
fire while the entry itself stays exempt at its pinned count.
