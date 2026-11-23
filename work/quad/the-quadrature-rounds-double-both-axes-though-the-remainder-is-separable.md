---
id: the-quadrature-rounds-double-both-axes-though-the-remainder-is-separable
kind: issue
title: A quadrature round doubles the pieces along both axes, though the remainder bound is a sum of a u term and a v term
status: open
opened: 2026-11-22
---


Found by BOXFINISH's sweep for the shape its split unit fixed in `editor_core::drive` (`work/boxfinish/a-drive-splits-a-box-along-a-variable-its-undecided-node-does-not-read.md`): a refinement step chosen without asking what the undecided thing depends on. **Read, not measured**: the choice rule below is quoted from the code; that splits are wasted is an argument from what the test reads, and no cell count was taken. (BOXFINISH split-reads lane)

- **`props::quad`'s rounds** (`run::Run`, the rectangle lanes, `SPAN_INIT_PIECES`): each round doubles
  the pieces per span along u and v. The remainder is `hull(f_uu)·h_u³h_v/24 + hull(f_vv)·h_u h_v³/24`;
  where one term dominates, doubling the other axis quadruples the work for little of the bound.
  Uniform refinement, not a per-cell choice.
