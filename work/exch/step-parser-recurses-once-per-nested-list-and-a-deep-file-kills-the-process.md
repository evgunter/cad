---
id: step-parser-recurses-once-per-nested-list-and-a-deep-file-kills-the-process
kind: issue
title: step-import: the parser recurses once per nested list with no bound, so a deeply nested STEP file kills the process with SIGSEGV
status: open
opened: 2026-09-30
priority: P1
---

(Filed by EDIT's `edit/part-depth-bound` fix pass, from its review's
probe and its mutual-recursion sweep.)

## What

`crates/step-import/src/parse.rs`'s `Parser::value` and
`Parser::value_list` are mutually recursive with no nesting bound: a
`(` in a parameter enters `value_list`, which calls `value` for each
member, which enters `value_list` again (a typed parameter,
`KEYWORD(…)`, takes the same path). `Value` (`List(Vec<Value>)`,
`Typed(String, Vec<Value>)`) is as deep as the text, and its derived
`Drop`, `Clone`, `PartialEq` and `Debug` recurse once per level too.

So a STEP file whose parameter nests deep enough kills the process
with a stack overflow instead of refusing typed. A file is user input:
`pncad.import_step` and `step_import::import_step` read it as given.

## Evidence

The review's probe (`review-probes/depth-rev/step_probe.py` on
`review/depth-rev`, not merged), run here on the dev wheel CI builds
(`maturin build`, no `--release`), CPython 3.12, main thread:

```python
import sys, pncad
n = int(sys.argv[1])
nested = "(" * n + "1" + ")" * n
text = (
    "ISO-10303-21;\nHEADER;\nFILE_DESCRIPTION((''),'2;1');\nFILE_NAME('','',(''),(''),'','','');\n"
    "FILE_SCHEMA(('AUTOMOTIVE_DESIGN'));\nENDSEC;\nDATA;\n"
    f"#1=CARTESIAN_POINT('',{nested});\nENDSEC;\nEND-ISO-10303-21;\n"
)
pncad.import_step(text)
```

| n | outcome |
|---|---|
| 500 | refuses typed (no solid in the file) |
| 1000 | refuses typed |
| 2000 | SIGSEGV, exit 139 |

## What would close it

A nesting bound in the parser (a depth counter on `value_list`),
refused typed as a syntax error with a recourse, set so the parse and
every walk over `Value` fit the smallest stack an import door runs on;
or a parser and a `Value` whose walks do not recurse. A row that
imports a file one past the bound, on that stack, and reads the
refusal back.
