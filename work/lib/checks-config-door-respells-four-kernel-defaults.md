---
id: checks-config-door-respells-four-kernel-defaults
kind: issue
title: The Python ChecksConfig door re-spells four kernel defaults, and neither config door has a census anchor
status: open
opened: 2026-09-15
refs: [1493, 1495]
---

## From PORT's `python-cannot-set-options-structs` sweep (PR for
`port/pyopts-four-doors`)

That unit swept for the class *a kernel options struct crossing the
FFI with no per-field exposure and no census anchor* and fixed the four
the row named (`ImportOptions`, `AsciiOptions`, `BinaryOptions`,
`EvalOptions`, joining `StepOptions`). Two more configuration structs
reach a Python door and are **not** those four, because they cross as
value classes with their own constructors rather than as keywords.
They are LIB's, and neither is in the fixed set.

### `ChecksConfig` — the defaults are typed twice

`ChecksConfig::new` in `crates/pncad-py/src/py/checks.rs` declares

```
#[pyo3(signature = (connectedness = Severity::Warn, expected_components = None,
                    separation = Advisory::Warn, chart_coherence = Advisory::Off))]
```

and `editor-core`'s `impl Default for ChecksConfig`
(`crates/editor-core/src/checks.rs`) independently declares
`connectedness: Severity::Warn`, `expected_components:
BTreeMap::new()`, `separation: Advisory::Warn`, `chart_coherence:
Advisory::Off`.

The two agree today and nothing holds them to each other. This is
exactly the defect PR 1493's `unwrap_or(defaults.X)` spelling exists to
prevent — the export door's `step_string` writes
`product_name.unwrap_or(defaults.product_name)` precisely so the
Python default cannot drift from the Rust one — and the neighbouring
`McConfig` door already uses that spelling
(`crates/pncad-py/src/py/analysis.rs`, `let base = a::McConfig::default()`).

The one field that would be caught is `chart_coherence`, whose
kernel-side doc comment argues at length for `Off` being the one
default on the type that is not `Warn`: a kernel decision to change it
would leave the Python door silently handing out `Off`.

The fix is the established one: make every parameter `None`-defaulting
and forward `d::ChecksConfig::default()` field by field. `Severity` and
`Advisory` are already Python-visible, so the shape carries.

### Both config doors lack a census anchor

`surface_census.rs`'s `options_doors` now holds five options structs to
their doors field by field (a destructure with no `..` plus a roster
whose `NotBound` entries decay against the door's keywords).
`ChecksConfig` and `McConfig` are outside it. Both build their kernel
struct with an exhaustive literal, so a field added kernel-side does
red `E0063` at the constructor and cannot be *silent* — that is why
this is a row of its own rather than part of the four — but the
decision that follows the red is not recorded anywhere, and a
constructor that answers it with a hardcoded value passes every test in
the tree.

`AnalysisPolicy` was checked and is **not** in this class: its field is
private and its only constructor validates, so growth is the kernel's
own problem.

## Home

`work/lib/` — `crates/pncad-py/*` is LIB's territory
(`work/lib/program.md`'s `paths`). Filed by a PORT lane under
`work/README.md`'s rule that a finding goes on the slate of the program
whose ground it lands on.
