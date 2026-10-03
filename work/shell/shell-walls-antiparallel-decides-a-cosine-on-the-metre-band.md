---
id: shell-walls-antiparallel-decides-a-cosine-on-the-metre-band
kind: issue
title: shell_walls_antiparallel decides a bare cosine against the metre band, and the dimension audit has no row for it
status: open
opened: 2026-10-01
---


## What

`crates/topo/src/shell.rs`, the wall-pair gate in the thicken path
(`shell_walls_antiparallel`, near :2379), decides

    Margin::of(-(a.normal.dot(b.normal)) - T::one())

against the metre band. Both normals are unit vectors, so the margin is
`−cos θ − 1`, dimensionless: whether two faces "face each other" is
read at an angle of about `√(2ε)`, the same at every model size. The
ray caster's skip test had this shape (ledger row F2, fixed on
`reach/opensign-red`); there a non-length comparand let a wide rod's
wall go unseen at ε = 1e-6.

`docs/predicate-dimension-audit.md` has no row for this predicate, and
`work/tier/symbolic-tier-census.md` lists it as "not in the M10-8
documents".

## Found by

The REACH `opensign-red` lane's sweep for bare-cosine margins
(`Margin::of(… .dot(…))` outside tests), 2026-10-01. Not measured on a
fixture: no row here shows it answering wrong.

## Final state

The gate's margin is a length (the angle levered by the extent over
which the two walls are compared, as the other parallel gates lever
theirs), or the audit carries a row saying why a cosine is the right
comparand here.
