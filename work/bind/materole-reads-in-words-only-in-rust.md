---
id: materole-reads-in-words-only-in-rust
kind: issue
title: Python's MateRole has no __str__, so a script cannot read the kernel's sentence for what a mate did
status: open
opened: 2026-10-02
priority: P3
cost: E
refs: [materole-has-no-display]
---

Found by AUTH-16 (PR 3769), which gave the viewer's mate row the
kernel's sentence for `MateRole`.

`impl Display for MateRole` (`crates/editor-core/src/mate/solve.rs`)
is the kernel's one sentence for what a mate did in the solve:
whether it placed its child, only declares a contact, or was refused.
The viewer now draws it under a mate's tree row. The Python mirror,
`crates/pncad-py/src/py/mate.rs`'s `MateRole` pyclass (~:536), is a
bare `eq_int` enum with no `__str__`, so `str(solve.role(mate))` gives
pyo3's default, and a script has to write its own words for the role.
That is the gap MSOLVE's `materole-has-no-display` closed for Rust
surfaces.

**What would close it.** Keep the kernel value beside the mirror (or
map back with a `to_kernel`), and give the pyclass a `__str__` that
returns the kernel's `Display`, the way `assembly.rs`'s pyclasses do
(`fn __str__(&self) -> String { self.0.to_string() }`, ~:267). Add a
stub line and a test that pins `str(role)` against the kernel's
sentence for each variant.
