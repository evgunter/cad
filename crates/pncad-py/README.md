# pncad — Python bindings

Python bindings for the `pncad` B-rep CAD kernel: author exact
solids, validate them, measure them, tessellate and cross-check the
mesh, export STEP and STL — headless, from a script.

`pncad` is a **placeholder name**. The project has not been named yet
(design question Q9), and the crate, the module and the docs will all
be renamed together when it is. Nothing is published to PyPI, because
there is nothing to publish it under.

## Install

With `maturin` in a virtualenv:

```console
$ python3 -m venv .venv && . .venv/bin/activate
$ pip install maturin
$ maturin develop -m crates/pncad-py/Cargo.toml --features extension-module
```

Without `pip` — build the cdylib and stage it, which is what this
repo's own runner does:

```console
$ ./crates/pncad-py/run-python-tests.sh          # builds, stages, runs the tests
$ PYTHONPATH=target/python-stage python3 crates/pncad-py/examples/bracket.py
```

Wheels are abi3 (`py38`), so one wheel per platform serves every
CPython from 3.8. The bindings are f64-only; the kernel's certified
interval and dual-number lanes are not bound yet.

## A first model

```python
from pncad import Doc, Node, evaluate, mm

doc = Doc()
profile = doc.insert(
    Node.polygon([(0 * mm, 0 * mm), (80 * mm, 0 * mm), (80 * mm, 40 * mm), (0 * mm, 40 * mm)])
)
plate = doc.insert(Node.extrude(profile, 8 * mm))

body = evaluate(doc).value(plate).body()
body.validate()
print(f"{body.mass_properties().volume:.3e} m^3")      # 2.560e-05 m^3
```

Rounds and arcs are the PATHS lattice, where each state of the tip is
its own class exposing only its legal continuations:

```python
from pncad import Doc, Node, Open, Start, evaluate, mm

rounded = (
    Open.at((0 * mm, 0 * mm))
    .line_to((40 * mm, 0 * mm))   # sharp corner here, arriving east
    .toward(0.0, 1.0)             # departure ray: north
    .fillet(6 * mm)               # round where that ray meets the next
    .toward(-1.0, 0.0)            # arrival ray: west
    .to((0 * mm, 30 * mm))        # the authored far vertex
    .line_to(Start)
)
doc = Doc()
plate = doc.insert(Node.extrude(doc.insert(Node.profile(rounded)), 8 * mm))
assert evaluate(doc).succeeded(plate)
```

The rounded corner is never authored: you give the two rays that would
have met there and the arc is fitted to their virtual intersection,
trimming both. Off-lattice moves — a second director, `.tangent()` on
a point with no incoming leg, a leading `.fillet` — are not methods
that refuse; they are methods that do not exist.

`crates/pncad-py/examples/bracket.py` is the full journey — build,
evaluate, validate, measure, export STEP, and re-import the result to
prove it round-trips.

## What you are talking to

Python speaks the kernel's **document layer**, not a second
kernel-bypassing API. You insert nodes describing what to build,
`evaluate` the document, and read typed values out. That is
deliberate: the document layer is the single API surface shared with
the future GUI, macro recording and headless tests, so a Python script
is a recipe that persists, replays and undoes — not a pile of opaque
calls. `Doc.save()` / `load()` round-trip bit-identically.

Three things to expect, all of which have longer treatments in the
guide:

- **Typed quantities.** `25 * mm` is a `Length`, not a float.
  Mixing dimensions raises `DimensionError` with `op`, `left`,
  `right`.
- **`evaluate` is total** — it never raises. Every node either has a
  value or has failed; ask with `ev.succeeded(node)`. Reading a
  failed node raises `EvaluationError`, and a node poisoned by an
  upstream failure names the culprit in `through`.
- **The kernel refuses rather than guesses.** Two faces that merely
  touch are not silently welded; a boolean over an undeclared
  coincidence fails loudly — and the refusal carries its own
  recourse: the candidate declaration rides the exception as a typed
  `finding` (`Evaluation.find_flush_candidates` → `Node.declare` is
  the protocol behind it). Refusals are exceptions carrying
  attributes, never prose to parse — all of them subclass
  `PncadError`.

## Documentation

- `docs/GUIDE.md` — the guide. §1.3 is the Python quickstart, §2.8 the
  canonical journey.
- `docs/guide/fail-loud.md` — the refusal vocabulary, with executed
  Python examples.
- `docs/guide/meshing.md` — the ladder's tessellate and cross-check
  rungs: what a `Mesh` carries across, and how a caller re-derives
  closure and volume from it.
- `docs/guide/north-star-audit.md` — **what is not bound yet**, gap by
  gap. Read this before assuming a feature exists.
- `pncad.pyi` — the stubs, checked against the compiled module by
  `tests/test_stubs.py` name for name, and by `ty` for signatures in
  `tests/test_ty.py` — where the lattice's illegal states are pinned
  as type errors. The runtime checks live once at the Rust boundary.

## Tests

```console
$ ./crates/pncad-py/run-python-tests.sh [python-binary]
```

Covers the document surface, quantities, stub drift, D9 bit-identical
replay, persistence and STEP round-trips, the mesh door and its STL
exports, plus every Python block in
the guide (`tests/test_guide.py` reads the Markdown directly, so the
documentation cannot rot).

These tests are **not** in CI yet — building the extension module
requires the `extension-module` feature, and the wheel job is a
recorded follow-up. Run them by hand when you touch the bindings.

## License

MIT OR Apache-2.0.
