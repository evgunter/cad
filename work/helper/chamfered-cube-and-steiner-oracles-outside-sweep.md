---
id: chamfered-cube-and-steiner-oracles-outside-sweep
kind: issue
title: The chamfered-cube and Steiner closed forms are restated outside crates/sweep too
status: open
opened: 2026-09-03
priority: P3
cost: E
---


Found by TCOST-10's style review, by a sweep over the CONSTANTS rather
than the names. TCOST-10 homed the two closed forms for `crates/sweep`
in `crates/sweep/tests/common/oracles.rs` and wrote a
"deliberately not absorbed, and the whole of it" list; that list was
complete only for `crates/sweep`, because the census that cut the unit
grepped for helper NAMES (`fn brick`, `fn ..._volume`) and every site
below is either an inline expression or a differently-named helper.

**The chamfered cube, `a³ − 6ad² + (16/3)d³`** — the same association
as `oracles::chamfered_cube_volume`:

- `crates/editor-core/tests/lib_g16_chamfer_node.rs:47` (inline
  `let volume = …`; the module also carries a surface-area form)
- `crates/pncad-py/tests/test_north_star.py:1193` (and the derivation
  restated in prose at `:1165`), which cites `lib_g16_chamfer_node.rs`
  as where the derivation lives

**Its complement, `6ad² − (16/3)d³`** — the same association as
`oracles::chamfered_cube_removed`:

- `demos/tour/src/diechamfer.rs:149` (`edge_material`)

**The Steiner rounded box in the DIE association** (`core³ + 6R·core² +
12(πR²/4)·core + (4/3)πR³`) — a sixth and seventh member of the
five-file class `oracles.rs` names inside `crates/sweep`:

- `crates/editor-core/tests/m5_pr12_fillet_node.rs:33-40` (`rounded_box`,
  volume and area)
- `demos/tour/src/diefillet.rs:449-455` (`blank_volume`)

**The sweep obligation for anyone taking this up is the CONSTANTS, not
the names.** `16.0 / 3.0`, `(4.0 / 3.0) * PI`, `12.0 * (PI * r * r /
4.0)` and their Python spellings, over `crates/` and `demos/`, then
read each hit. A name-based grep is a receipt for nothing here.

Not obviously a fix-by-sharing: a cross-crate test-support home is
LIB-U6's territory per `crates/sweep/tests/common/mod.rs`'s routing
rule, and `demos/` deliberately reaches the kernel from an outside
consumer's seat (`docs/prompts/implementer-discipline.md` §3), so a
demo computing its own expectation may be the right shape. What is
NOT right is the present state: five spellings of two formulas with no
single place that says which is the derivation, and one of them
(`test_north_star.py`) citing a Rust file for a derivation that has
since moved.

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

## Re-derived (2026-09-15, lane D)

**VERDICT: REPRODUCES**, with the floor raised from five sites to seven;
one sub-claim in the closing paragraph is itself now stale.

**Command — the CONSTANTS, as the row demands, not the names:**

```
grep -rnE '16\.0 */ *3\.0|16 */ *3|16\.0/3\.0'  --include=*.rs --include=*.py crates/ demos/ docs/ tools/
grep -rnE '4\.0 */ *3\.0'                        --include=*.rs --include=*.py crates/ demos/ docs/
grep -rnE '12\.0 *\* *\(?PI|12 *\* *\(?math\.pi' --include=*.rs --include=*.py crates/ demos/
```

**Chamfered cube `a³ − 6ad² + (16/3)d³`** — both sites LIVE:
`crates/editor-core/tests/lib_g16_chamfer_node.rs:47` (`let volume = …`,
with the derivation in prose at `:20` and the surface-area form still
beside it) and `crates/pncad-py/tests/test_north_star.py:1305` (prose
derivation now at `:1275`).

**Complement `6ad² − (16/3)d³`** — LIVE: `demos/tour/src/diechamfer.rs:149`
(`edge_material`), and it now has a **second spelling in the same file**,
the printed form at `:281`.

**Steiner rounded box in the DIE association** — both sites LIVE:
`crates/editor-core/tests/m5_pr12_fillet_node.rs:37-39` (`rounded_box`,
volume and area) and `demos/tour/src/diefillet.rs:450` (`blank_volume`).

**Two members the list did not carry**, both in Python, both the Steiner
association the row says `oracles.rs` names as a five-file class:

- `crates/pncad-py/tests/test_north_star.py:1206`
  (`test_diefillet_matches_the_scene_oracle`), `core ** 3 + 6.0 * R *
  core ** 2 + 12.0 * (math.pi * R * R / 4.0) * core + (4.0/3.0) *
  math.pi * R ** 3`;
- `crates/pncad-py/tests/test_north_star.py:1543`, the same form at
  `DIE_R`.

So `test_north_star.py` carries **three** of the two formulas, not one,
and it is the only file that spells both.

**The sub-claim that does NOT reproduce.** The row closes with *"one of
them (`test_north_star.py`) citing a Rust file for a derivation that has
since moved"*. Today `test_north_star.py:1277` cites
`crates/editor-core/tests/lib_g16_chamfer_node.rs` and that file's own
header at `:20` still carries the derivation (`V = L³ − 6Ld² +
(16/3)d³`) and still meters the surface area. The citation resolves.

**A stale pointer this row causes.**
`crates/sweep/tests/common/oracles.rs:60` tracks the row at
`work/tcost/chamfered-cube-and-steiner-oracles-outside-sweep.md`, a path
that has not existed since the 2026-09-11 move to `work/tint/`.
`oracles.rs`'s outside-crate list itself is otherwise accurate — it names
the same four Rust/demo sites and the Python file.

**Blind spot.** The constant sweep matches a literal division; a form
that pre-computes `16/3` into a named constant, or writes `5.3333…`, or
factors the cube differently, would not appear. `docs/` was searched but
carries no member of either association.

**Recommend: keep open, extend the list to the two Python Steiner
sites, and retire the "derivation has since moved" sentence.**
