---
id: python-param-classes-name-a-variable
kind: issue
title: Python's ParamName, DocParam and DocParamValue name a variable by the retired word
status: closed
opened: 2026-10-04
closed: 2026-10-05
branch: intent/literals-b
---

INTENT-VARS-1 PR 3 gave Python a `Var` handle and the variable doors
(`Doc.vars`, `Doc.var`, `Doc.var_name`, `Doc.rename_var`,
`Doc.delete_var`), but the classes beside them still say "param":

- `ParamName` (`crates/pncad-py/src/py/doc.rs:3245`,
  `crates/pncad-py/pncad.pyi:2856`), a variable's name;
- `DocParam` (`crates/pncad-py/src/py/doc.rs:3393`, `pncad.pyi:3201`),
  a free variable's definition;
- `DocParamValue` (`crates/pncad-py/src/py/doc.rs:3633`,
  `pncad.pyi:3264`), its value.

A caller now holds a `Var` whose name is a `ParamName` and whose
definition is a `DocParam`. Rust's `VarName`, `FreeVar` and
`FreeValue` are the words to follow. The rename is a breaking change to
the binding's surface (class names, the `.pyi`, the binding census in
`crates/pncad-py/tests/test_binding_census.py`), so it wants its own PR
with the stub and the census moving together. The error tag words
(`unknown_param`, `count_param`, …) are part of the same surface and
should be decided in the same pass.

Raised by PR 3's dual review (r2 S4).

Closed by INTENT-LITERALS PR B, which renamed the three classes to
`VarName`, `FreeVar` and `FreeValue` with the stub and the census in the
same pass. The tag words keep their spelling: they are a separate
public contract (`crates/pncad-py/src/tests.rs`'s `TAG_INVENTORY`), and
spec §3 names only the classes.
