---
id: chamfered-cube-and-steiner-oracles-outside-sweep
kind: issue
title: The chamfered-cube and Steiner closed forms are restated outside crates/sweep too
status: open
opened: 2026-09-03
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
