---
id: python-has-no-door-to-declare-a-seam
kind: issue
title: Python declares only flush findings, and no finding is ever a seam: a Seam cannot be declared from Python
status: open
opened: 2026-10-02
---


## What

`topo::BooleanCoincidence::Seam` (TANG, `tang/pi-seam`) crosses to
Python as `pncad.BooleanCoincidence.Seam` (`crates/pncad-py/src/py/flush.rs`,
`boolean_coincidence`), and a `Declare` node carrying one round-trips on
the wire. But Python builds a `Declare` node only from flush findings
(`Node.declare`, `Doc.declare`, `Doc.declare_all` in
`crates/pncad-py/pncad.pyi`). The flush detector reports `Rest` or
`Continuation` (`topo::flush`, `BooleanCoincidence::of_senses`), never a
seam: a G1 rim is not flush. So a Python author cannot state the one
declaration a capped tube needs. Rust callers can, through
`Node::Declare { pairs }`.

## The question

The door is either a seam finding (a detector for shared G1 rims, the
detect/declare protocol's shape) or a `Node.declare` that takes face
pairs and a class directly. SELECT-DESIGN §3's no-fusion boundary bears
on which.
