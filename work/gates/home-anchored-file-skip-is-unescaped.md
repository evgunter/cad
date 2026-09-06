---
id: home-anchored-file-skip-is-unescaped
kind: issue
title: signed-zero-one-home.sh's home-anchored file skip interpolates the path unescaped, so the anchor is exact only by accident
status: open
opened: 2026-09-06
---

## Finding

Turned up by the sweep for `anchored-exact-text-skip-has-three-homes`,
which converted the two exact-text skips and left this NEIGHBOUR of the
class in place.

`scripts/gates/signed-zero-one-home.sh:109` drops every record at the
sanctioned home with

    | gate_grep -vE "^$HOME_FILE:" \

where `HOME_FILE=crates/step-import/src/signed_zero.rs` (`:66`) is
interpolated into an ERE **unescaped**. The `.` is a metacharacter, so
as a pattern the skip also names `crates/step-import/src/signed_zeroXrs`.

**THAT PATH IS UNREACHABLE BY CONSTRUCTION, and the row says so up
front**: the gate's scan set is `find "${SCAN_DIRS[@]}" -type f -name
'*.rs'` (`:101`), so no record it reads can carry a path whose
extension is not `.rs`, and the trailing `:` in the pattern stops a
longer path from matching. There is no live hole here and no red to
expect — the population is zero today and zero for any tree this gate
can scan.

What it is instead: **an anchor that is exact by accident rather than
by construction**, in a gate whose whole claim is "the flush lives in
THIS file and nowhere else". The pattern says something slightly wider
than the gate means, and the reason it does not matter is a property of
a DIFFERENT line (the `-name '*.rs'` in the scan) that nothing ties to
this one.

## Fix shape

`| gate_grep -vE "$(gate_record_anchor "$HOME_FILE")"` — `lib.sh`'s
anchor builder, which escapes the path and pins the `FILE:LINE:` shape
that every view emits. It is a change of CONSTRUCTION and not a hole
closed, so the fixture it owes says exactly that much: the home still
passes, and a record whose path differs from the home only where the
metacharacter sits is not exempt. The second half cannot be planted as
a `.rs` file in the scan (that is the point), so it is either a unit
assertion over hand-built records — `gate_exact_skip_escaping_case`'s
shape in `lib.sh` — or the case is not written and the change rests on
the builder's own fixtures.

## What was not measured

Whether any other gate reads a whole-file skip this way. The sweep that
found it (`grep -n '\-E "\^\$' scripts/gates/*.sh`) matched the three
exact-text skips, this one, and nothing else, but it cannot match a
skip built by a helper or spelled with the anchor in a variable.
