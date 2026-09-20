---
id: checks-config-door-respells-four-kernel-defaults
kind: issue
title: The Python ChecksConfig door re-spells three kernel defaults; the two config doors are a deliberate half-fix of the options class
status: open
opened: 2026-09-15
refs: [1493, 1495, 2678]
priority: P4
cost: E
---

## From PORT's `python-cannot-set-options-structs` sweep (PR #2678)

That unit swept for the class *a kernel options struct crossing the
FFI with no per-field exposure and no census anchor* and closed it for
five structs — `StepOptions` (PR 1493), then `ImportOptions`,
`AsciiOptions`, `BinaryOptions` and `EvalOptions`. Two more
configuration structs reach a Python door and were left, because they
cross as value classes with their own constructors rather than as
keywords, and the spec's fence was four doors.

**This row is that deliberate half-fix, and the class it leaves open is
named here rather than left to be rediscovered.** The reason it matters
is the unit's own finding: `AsciiOptions` and `BinaryOptions` also
"already looked right" — they exposed their field as a keyword and
looked in sync with the kernel — and they had been writing `solid ` and
eighty zero bytes for their whole existence, because the Python default
was a hand-typed value rather than a forwarded one, and nothing held
the two together. `ChecksConfig` agrees with the kernel *today* by the
same kind of coincidence.

### `ChecksConfig` — the defaults are typed twice

`ChecksConfig::new` in `crates/pncad-py/src/py/checks.rs` declares

```
#[pyo3(signature = (connectedness = Severity::Warn, expected_components = None,
                    separation = Advisory::Warn, chart_coherence = Advisory::Off))]
```

and `editor-core`'s `impl Default for ChecksConfig`
(`crates/editor-core/src/checks.rs`) independently declares
`connectedness: Severity::Warn`, `expected_components: BTreeMap::new()`,
`separation: Advisory::Warn`, `chart_coherence: Advisory::Off`.

**Three of the four are re-spellings** — `connectedness`, `separation`
and `chart_coherence`, each a Python enum value typed in the signature
and mapped to the kernel's by `to_kernel()`. The fourth,
`expected_components`, is not: `None` becomes an empty map through
`unwrap_or_default()`, so it coincides with the kernel's default
through a *different* `Default` impl rather than through a copied
constant. It is the one field of the four that is already safe, and for
a reason that does not generalise to the others.

The three agree today and nothing holds them there. This is exactly the
defect PR 1493's `unwrap_or(defaults.X)` spelling exists to prevent —
`step_string` writes `product_name.unwrap_or(defaults.product_name)`
precisely so the Python default cannot drift — and the neighbouring
`McConfig` door already uses that spelling
(`crates/pncad-py/src/py/analysis.rs`, `let base = a::McConfig::default()`).

The sharpest of the three is `chart_coherence`, whose kernel-side doc
comment argues at length for `Off` being the one default on the type
that is not `Warn`, with two measured reasons. A kernel decision to
change it would leave the Python door silently handing out `Off`.

The fix is the established one: every parameter `None`-defaulting,
forwarding `d::ChecksConfig::default()` field by field. `Severity` and
`Advisory` are already Python-visible, so the shape carries.

### Both config doors are outside the census's membership rule

`surface_census.rs` holds five options structs to their doors field by
field, and `every_options_type_in_py_is_rostered` enforces the
membership of that list — but the rule it enforces is a NAME rule:
a type called `*Options` constructed under `src/py/`. `ChecksConfig`
and `McConfig` are named `*Config` and are outside it by construction.
That test's doc names them as its stated blind spot and points here.

Both build their kernel struct with an exhaustive literal, so a field
added kernel-side does red `E0063` at the constructor and cannot be
*silent* — which is why this is a row of its own rather than part of
the four. What is unanchored is the decision that follows the red, and
a constructor answering it with a hardcoded value passes every test in
the tree.

`AnalysisPolicy` was checked and is **not** in this class: its field is
private and its only constructor validates, so growth is the kernel's
own problem.

## Home

`work/lib/` — `crates/pncad-py/*` is LIB's territory
(`work/lib/program.md`'s `paths`). Filed by a PORT lane under
`work/README.md`'s rule that a finding goes on the slate of the program
whose ground it lands on.
