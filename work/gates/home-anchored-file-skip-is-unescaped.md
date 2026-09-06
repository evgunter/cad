---
id: home-anchored-file-skip-is-unescaped
kind: issue
title: signed-zero-one-home.sh's home-anchored file skip interpolates the path unescaped, so a sibling path exempts itself
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
the skip also exempts `crates/step-import/src/signed_zeroXrs`, and
any other path this tree could hold that differs from the home only
where a metacharacter sits. The trailing `:` is the only thing keeping
it from exempting a longer path as well, so the anchor is doing half
its job by accident rather than by construction.

Population today: zero. The defect is a widening, so it is the
direction that never cries wolf — a file added at such a path is
exempt from the negative-zero-flush rule with nothing red anywhere,
which is what `bounds-allowlist.sh`'s and `viewer-module-kinds.sh`'s
own anchors were changed away from in that unit.

Not fixed there because it is a different mechanism: this skip exempts
a WHOLE FILE, not one ratified text, so it takes neither
`gate_exact_skip` nor its fixtures. Its subject check is already the
rule that unit landed — `gate_require_file "$HOME_FILE"` at `:100`
reds when the home is gone.

## Fix shape

`| gate_grep -vE "$(gate_record_anchor "$HOME_FILE")"` — `lib.sh`'s
anchor builder, which escapes the path and pins the `FILE:LINE:` shape
— plus the fixture that holds it: a flush planted at a sibling path
that differs from the home only where the metacharacter sits must FIRE
(`gate_selftest_case`), and the home itself must still pass.

## What was not measured

Whether any other gate reads a whole-file skip this way. The sweep that
found it (`grep -n '\-E "\^\$' scripts/gates/*.sh`) matched the three
exact-text skips, this one, and nothing else, but it cannot match a
skip built by a helper or spelled with the anchor in a variable.
