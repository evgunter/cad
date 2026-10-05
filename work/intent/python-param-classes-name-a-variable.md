---
id: python-param-classes-name-a-variable
kind: issue
title: Python's ParamName, DocParam and DocParamValue name a variable by the retired word
status: open
opened: 2026-10-04
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
