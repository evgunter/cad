---
id: fused-two-shell-body-doc-predates-movefac
kind: issue
title: fused_two_shell_body says its shape is not publicly constructible, but movefac builds it
status: open
opened: 2026-09-06
track: P
refs: [S69, 2014]
---

## What

`crates/topo/src/euler_ring.rs:2588-2592` — the `fused_two_shell_body`
fixture's doc says:

> A same-solid two-shell body: the shape `kfmrh`'s fusion form exists
> for. It is **not constructible through the public operators**
> (`mvfs` mints one solid per shell), so the second shell is re-homed
> by raw in-crate write — the same adversarial posture as the rest of
> this module's corruption rows.

That is no longer true. `Body::movefac`
(`crates/topo/src/movefac.rs:66`) is a public operator that mints a
new shell **into an existing solid**, and it is the door the
distribution step of `splitfinish`/`setopfinish` goes through. A
same-solid two-shell body is exactly what it produces:
`crates/topo/src/movefac.rs:333`'s `cross_shell_kfmrh_fuses_shells`
builds one through the public doors and fuses it, and since `S69`
(PR 2014) the seqgen walk reaches the same shape by rolling
`OpChoice::Movefac`.

The consequence is not a wrong test — the fixture's three rows
(`kfmrh_refuses_a_dangling_face_in_f2s_shell`,
`kfmrh_refuses_a_dead_shared_solid`, and the corruption they plant on
top) still test what they say. It is that a reader is told the shape
is unreachable, and that a raw in-crate write is being paid for a
posture the ops no longer require. The two guard rows corrupt the
fixture deliberately and would keep their raw writes; the fixture
UNDER them can be built by `mfkrh` + `movefac`.

## Why it is not folded into S69

S69's fence was the generator, the ledger and `kfmrh`'s postcondition
site; rewriting a test fixture in `euler_ring.rs`'s corruption block
is a different change with its own blast radius (three rows read it).
Disclosed there, filed here.

## What closing it looks like

Rebuild `fused_two_shell_body` through `mfkrh` + `movefac` (or state
in-file why the raw write is still wanted — e.g. that the corruption
rows want a shell whose provenance is NOT `Provenance::Movefac`), and
delete the sentence that says the shape is not publicly constructible.
