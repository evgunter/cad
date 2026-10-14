---
id: loop-boundary-discards-loses-an-fn-whose-signature-holds-a-semicolon
kind: issue
title: loop-boundary-discards drops the enclosing fn when its signature holds a ';' (an array type such as [T; 2]), so its sites read as '(no enclosing fn)'
status: open
opened: 2026-10-13
priority: P3
cost: E
---


## What

`scripts/gates/loop-boundary-discards.sh` tracks the enclosing `fn` with a `pending` name. It sets the name at `fn <name>`, attaches it at the next `{`, and clears it at every `;`.

A `;` inside the signature, such as an array type in a parameter or return type (`-> Result<[ChartCuts; 2], …> {`), clears the name before the body's `{`. That `fn` is then never pushed, and every discard inside it reads as `(no enclosing fn)`: an `UNREG` line for each site, plus `MISCOUNT … matched 0` for its register entries.

**Reached:** PR 5039's `topo::boolean::ops::apply_cut_ins`. EMIT worked around it with a type alias (`CutsPerOperand`).

**Owed:** clear `pending` only on a `;` at the depth where the `fn` was declared, outside its `<…>`/`(…)`/`[…]` nesting. Add a gate selftest row with an array type in a signature.
