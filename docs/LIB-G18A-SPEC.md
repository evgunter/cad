# LIB-G18a — the resolver and memo parameters of Python's `evaluate`

**Status: RATIFIED (Evan, in-chat, 2026-08-29 — "that looks right").
Binding at dispatch. Full model-A/B protocol unit (NOT mechanical):
the flagship signature changes, so this is the judgment-checked kind.**

## The ratified shape

```python
evaluate(doc, *, resolver: Workspace | None = None,
              prior: Evaluation | None = None)
```

- **`resolver=`** carries a `Workspace` into the evaluation as the
  document seam: Rust's `evaluate(doc, prior, cancel, opts, tol)`
  already takes `opts.resolver: Option<Arc<dyn PartResolver>>`
  (`crates/editor-core/src/eval/mod.rs:976`), and the G15-bound
  `Workspace` IS the curated `PartResolver` impl — the binding passes
  it through; nothing kernel-side moves. The parameter keeps Rust's
  ROLE vocabulary (`resolver`, not `workspace`) so a second resolver
  type could ever join without a rename. `None` — the default — stays
  today's kernel-only evaluation: every `InstantiatePart` node refuses
  typed, exactly as now, and existing callers are untouched.
- **`prior=`** is the memo (spec D4): nodes whose content key matches
  reuse the prior value; only the changed cone re-runs. This closes
  PYPU's banked finding ("memoized recompute is unobservable from
  Python") in the same signature change rather than a second one.
- **`cancel` deliberately stays out** — the census's `B-CANCEL` family
  is its own unit.

## Deliverables

1. The two keyword parameters on `pncad-py`'s `evaluate`, faithful to
   the Rust door. Refusal vocabulary crosses per the crate's errors.rs
   convention (typed exceptions carrying the structured error, never
   strings): measure what `ResolveFailure`/`ResolveFault`/`PartFault`
   already have in `tags.rs` and extend exhaustively where they do not.
2. **Memo observability, honestly**: the unit must make reuse
   OBSERVABLE from Python or report precisely why it cannot. Prefer an
   existing curated observable on `Evaluation`; a timing side channel
   is not acceptable evidence. If the Rust value records nothing a
   binding may honestly expose, the unit delivers the faithful `prior=`
   pass-through plus a stated gap with the Rust-side door it would
   need — that report shapes a follow-up, and inventing an observable
   kernel-side is out of fence.
3. Stubs (`pncad.pyi`, ty fixtures legal + illegal), census (the
   `gap: G18` entries this closes move; G18 itself STAYS OPEN — the
   node/edit half is G18b), audit page re-cut honest (rows 46/47 move
   only as far as the truth: an `InstantiatePart` node still cannot be
   AUTHORED from Python, but a document that already carries one — the
   tour's assembly documents, loaded through `Workspace` — can now
   EVALUATE; whether that flips a row is measured against each row's
   own claim, not asserted).
4. Python tests with the tour's assembly corpus as oracle
   (`demos/tour/src/assembly.rs`, per the ASM deposit): load the bench
   documents from a workspace directory, evaluate with `resolver=`,
   compare against the Rust evaluation's own recorded expectations;
   the no-resolver typed refusal stays pinned; the resolution refusal
   family (cycle, pin mismatch at the seam, unknown id) each exercised
   through the new parameter where reachable from Python.

## Fences

No node/edit/refactoring bindings (G18b, a later mechanical unit). No
assembly-gate bindings (`assemble` etc. — G18b's tail). No kernel,
schema, or Rust-façade changes; the Rust `evaluate` signature is
untouched. No `cancel` binding. No changes to `Workspace`'s own
surface beyond what passing it as a resolver strictly requires.

## Protocol

Full A/B: implementer arm = block LIB-12 slot 2, read back from the
redacted block record (git history) at dispatch. Pre-draw fields
logged at this spec's ratification: **M / STRUCTURAL**. v6 dual at
review time (ordinal claimed from the LIB band on main at dispatch);
the dual carries the standing LIB-12 CONTAMINATION FLAG
(MODEL-AB-LOG, 2026-08-29 redaction entry). Blinding: no
Co-Authored-By trailer in lane commits; reviewer-visible surfaces
never name the arm.
