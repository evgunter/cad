# FIX-ERRKINDS — one declaration for an error enum and its fieldless kind

Spec for `work/fix/kind-mirrors-have-no-single-declaration.md`. Binds one
implementer for one unit; deleted at merge per `docs/DOC-LEDGER.md`, with
the item file and the PR body as the record.

**This spec wants Ev's eyes before a lane takes it.** It changes the
declaration of three public error types and their three public kind
enums, moves ~87 public variant declarations out of rustfmt's care, and
needs a `#[macro_export]` in a kernel crate for which the tree has one
precedent, in a dev-only crate. The scale check below is measured; the
five choices in §6 are not measurements.

---

## 1. The question the scale check answered

`transition_table!` (`crates/profile/src/path/program.rs`, the
`Step`/`Verb`/`verb()` half) already generates a payload-carrying enum,
a fieldless kind enum and the projection between them from one
declaration. Its grammar accepts named-field and tuple arms with
`#[doc]` and nothing else. **Whether a general `macro_rules!` can carry
what the four in-tree pairs actually need was the first thing owed, on
the largest pair, before migrating four.**

Answered by prototype: a standalone `macro_rules!` type-checks under
the pinned toolchain (rustc 1.97.0, edition 2024) against every shape
the four pairs use. The prototype was run and discarded; nothing
prototype-shaped is committed. What it established, and what it could
not, is §2 and §5.

## 2. Measured: what the four pairs are, at `0aa691375`

Re-derived at this spec's merge base. **The item's counts are stale**
(it says 41 / 28 / 10, from 2026-09-11):

| pair | error | kind | variants | payload shapes | generics |
|---|---|---|---|---|---|
| boolean | `BooleanError` (`crates/topo/src/boolean/mod.rs`, ~:620) | `BooleanErrorKind` (~:1249) | **43** | 35 named, 7 tuple, 1 unit | none |
| path | `PathError` (`crates/profile/src/path.rs`, ~:783) | `PathErrorKind` (~:1274) | **31** | 26 named, 2 tuple, 3 unit | `<T: Real>` |
| product | `ProductError` (`crates/editor-core/src/product.rs`, ~:103) | `ProductErrorKind` (~:290) | **10** | 9 named, 1 unit | none |
| attr | `Attr` (`crates/editor-core/src/appearance.rs`, ~:127) | `AttrKind` (~:99) | **3** | 3 tuple | none |

Attributes, measured on each of the eight enums:

- **No `#[non_exhaustive]` anywhere**, on any of the eight. No `cfg`.
- **No non-doc attribute on any variant**, in any of the eight.
- Per-enum derives, and the two halves of a pair never share a list:
  `BooleanError` `#[derive(Debug)]` against its kind's
  `#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]`; `PathError`
  `#[derive(Clone, Debug)]`; `ProductError` `#[derive(Debug)]`.
- **`Attr`/`AttrKind` carry `serde::Serialize`, `serde::Deserialize`,
  `PartialOrd`, `Ord`, and `#[serde(deny_unknown_fields)]` on both
  halves.** They are a persisted key/value pair, not an error pair.
  §4 is about them.
- Per-field doc comments are used throughout the named-field arms.

So the specific worry the item raised — `#[non_exhaustive]`, `cfg` and
derive attributes needing `$(#[$m:meta])*` passthrough — resolves as:
`cfg` and `#[non_exhaustive]` are not used by any pair today, and
passthrough is needed anyway for the derives, which differ per enum.
Generic passthrough is needed by exactly one pair.

## 3. The grammar, forced by execution

Two spellings are not free choices; each was a compile error first.

**Generics are bracketed, not angle-bracketed.**
`$(< $($g:tt)* >)?` is rejected: *`error: local ambiguity when calling
macro`* — a `tt` repetition cannot tell which `>` closes the group.
`transition_table!` already brackets its generics (`on [ $($gen:tt)* ]`)
for this reason.

**Bounds go in a bracketed `where` group, not inline.** With
`enum PathError[T: Real]` the generated `impl` self-type reds with
E0229 (*associated item constraints are not allowed here*), because an
impl self-type takes parameter names without bounds and `macro_rules!`
cannot strip them. Restating the names (`[T: Real][T]`) works and was
rejected as the worse of two: it declares the parameter list twice,
which is the defect this row exists to remove. The spelling that
declares each thing once and type-checks clean is:

```rust
error_kinds! {
    /// Typed failure of the PATHS algebra.
    #[derive(Clone, Debug)]
    pub enum PathError[T] where [T: Real];

    /// Which arm of [`PathError`] refused.
    #[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
    pub enum PathErrorKind;

    /// A tolerance was not positive.
    NonPositiveTolerance {
        /// The offending value, in the caller's scalar.
        tol: T,
    },
    /// A degenerate segment.
    Degenerate,
    /// A nested band failure.
    Band(BandError),
}
```

The matcher, verified to accept every shape in §2:

```rust
$(#[$emeta:meta])*
$evis:vis enum $err:ident $([ $($g:tt)* ] $(where [ $($w:tt)* ])?)? ;
$(#[$kmeta:meta])*
$kvis:vis enum $kind:ident ;
$(
    $(#[$vmeta:meta])*
    $name:ident
        $({ $($(#[$fmeta:meta])* $f:ident : $ft:ty),* $(,)? })?
        $(( $($tt:ty),* $(,)? ))?
),* $(,)?
```

Four things this establishes, each by compiling rather than by argument:

1. **`Self::$name { .. }` matches all three arm shapes** — named-field,
   tuple of any arity, and unit. One projection body covers the whole
   table, `BooleanError::UnrepresentableResult` included.
2. **Payloads nesting other crates' error types are `$ft:ty` / `$tt:ty`
   and need nothing.** The item flags these as the scale risk; they are
   not one. `Band(BandError)` and `Euler(EulerOpError)` compile as
   written.
3. **A derive passed through a `meta` fragment runs.** The kind enum's
   `PartialEq` and `Debug`, used by the prototype's `assert_eq!`,
   arrive only through `$(#[$kmeta:meta])*`.
4. **The kind enum's per-variant rustdoc can be generated**, not lost:
   `#[doc = concat!("[`", stringify!($err), "::", stringify!($name), "`].")]`
   reproduces exactly the `/// [`BooleanError::Band`].` cross-links the
   three kind enums carry by hand today.

A wrong pairing is then not expressible: `$name` declares the arm and
projects it, so `Self::Merge(_) => Kind::Join` has no spelling. The
phantom direction closes with it — there is no second declaration to
add a variant to.

## 4. `Attr`/`AttrKind` is NOT part of this migration

The item lists it as the fourth pair and as *"the one that gains a
guarantee it has never had"*. Three findings against that, all measured
at this merge base:

1. **It is not an error pair.** It is the appearance store's key/value
   pair, `serde`-derived on both halves with
   `#[serde(deny_unknown_fields)]`, `Ord` on both (`AttrKind` is a
   `BTreeMap` key), and it has a persisted wire format. Nothing about
   the migration is mechanical there: it puts a schema behind a macro.
2. **The item's measurement that nothing matches on it exhaustively is
   false, in both halves.** Its own cited grep,
   `grep -rn "AttrKind::[A-Za-z]* *=>" --include=*.rs crates/`, returns
   **3** hits, not zero (`crates/pncad-py/src/tags.rs`, ~:300-302, an
   exhaustive tag map). And it would have missed a second consumer even
   at zero: `AttrKind::noun()`
   (`crates/editor-core/src/appearance.rs`, ~:111) is an exhaustive
   match with no `_` arm written in the `Self::` spelling, which that
   pattern cannot match by construction. Both predate the item's
   2026-09-11 text. `AttrKind`'s phantom direction is guarded twice
   over; only its pairing direction is open, across 3 arms.
3. **The payoff is 3 arms.** The one thing the grammar cannot be
   executed against here is serde: `#[serde(deny_unknown_fields)]` is
   an inert helper attribute the derive reads off the item, and it is
   the one passthrough this scale check could NOT run (§5). Buying an
   unverified serde passthrough and a wire-format pin for 3 arms is the
   wrong trade in the same unit that migrates 84.

**Disposition.** Migrate the three error pairs. `Attr`/`AttrKind` gets
its own row if anyone still wants it after the three land, and that row
owes a serde round-trip pin proving the representation did not move.

## 5. What this could not establish

- **serde attribute passthrough was not executed.** The prototype ran
  standalone under `rustc`, and the box's root filesystem was at 100%
  (1.7 MB free of 252 GB, ~30 GB of it other lanes' `CARGO_TARGET_DIR`s),
  so no `cargo` build and no linking was possible; the prototype was
  type-checked with `--emit=metadata`. `#[derive(Debug)]` through a
  `meta` fragment is proven; `#[serde(...)]` through one is not. It is
  only needed by §4's pair, which this migration does not take.
- **The prototype was type-checked, not run.** Every claim above is a
  compile-time claim — whether a pattern matches a shape, whether an
  impl header is legal — and the generated `kind()` body is a `match`
  with no runtime content. But it is type-checking, not execution, and
  saying otherwise would overstate it.
- **`Display` is untouched and its pairing stays open.** The macro
  generates `kind()` only. `BooleanError`'s hand-written `Display`
  (~:1413, ~400 lines) can still render a `Merge` arm with join prose,
  and nothing objects. The item's claim that the migration closes "the
  pairing direction" is true of `kind()` and false of `Display`.
- **A payload declared with the right name and the wrong type** is
  still unchecked. Out of scope.
- **One point of failure replaces four.** A bug in ~40 lines of macro
  is a bug in three pairs at once. The trade is ~40 reviewed lines
  against 84 hand-written rows and three hand-copied guards.

## 6. The five choices this spec does not get to make alone

Each is a real decision, and none of them is settled by the feasibility
answer:

1. **Where the macro lives.** `geom-core` is the only common ancestor of
   `topo`, `profile` and `editor-core`, so a `#[macro_export]` there
   reaches all three. It is also the kernel's numeric root, and hosting
   an error-declaration macro there is a layering choice. The
   alternative is a new leaf workspace member, which costs a crate.
2. **The tree has one `#[macro_export]`**, `crates/test-utils/src/lib.rs`
   (~:81) — a dev-only crate. A cross-crate `macro_rules!` export in a
   shipped kernel crate has no precedent here.
3. **87 public variant declarations leave rustfmt's care.** Measured:
   `rustfmt` reformats around a macro invocation and does not touch its
   body. `cargo fmt --check` stays green either way — rustfmt is a
   no-op on the body, not an error — so nothing reds; the discipline is
   simply gone for the largest declarations in three crates. This is
   already the status quo for `Step`'s table.
4. **`enum PathError[T] where [T: Real];` is not Rust's spelling**, and
   a reader of `path.rs` meets it before they meet the macro.
5. **Public rustdoc on the three kind enums changes.** Their hand-written
   enum-level docs describe guards that this migration deletes and a
   drift that it makes unrepresentable; the text has to be rewritten,
   not carried over.

## 7. Migration order and what each one deletes

Biggest pair first, per the item. **One PR per pair**, each on top of
the macro's own PR, so a pair that turns out to need something the
macro lacks is one revert rather than three.

**PR 1 — the macro.** `error_kinds!` in its chosen home, with its own
doc block stating what the grammar accepts and what it does not
(`cfg`, `#[non_exhaustive]` and variant-level attributes are untested
because no pair uses them — say so at the macro rather than discovering
it at the fifth caller). No migration in this PR.

**PR 2 — `BooleanError` (43 arms).** Deletes, in
`crates/topo/src/boolean/mod.rs`:

- `pub enum BooleanErrorKind` and its doc block (~:1220-1336);
- `impl BooleanError { fn kind() }` (~:1338-1400), 43 hand-written arms;
- the guard `each_kind_has_an_arm_and_each_built_arm_projects_to_its_own_kind`
  and the `label()` table inside it (~:2851-2940), and `sample_errors()`
  (~:2711-2849), whose only caller it is;

≈ 400 lines, against a macro invocation carrying the same 43 rows.

**PR 3 — `PathError` (31 arms).** Deletes `pub enum PathErrorKind`
(`crates/profile/src/path.rs`, ~:1274-1337) and
`impl<T: Real> PathError<T> { fn kind() }` (~:1339). **`path_error_tag`
(`crates/pncad-py/src/tags.rs`, ~:113) does NOT go** — it is a real
consumer producing the stable Python-facing tag strings, and its
exhaustiveness over the kind enum is a bonus, not its purpose. The item
names it as this pair's guard; it is a guard that also does a job.

**PR 4 — `ProductError` (10 arms).** Deletes `pub enum ProductErrorKind`
(`crates/editor-core/src/product.rs`, ~:290-311),
`fn kind()` (~:319), and the guard plus its `label()` table (~:963-1000)
with its `sample_errors()`.

Each PR re-derives its own counts and spans at its own merge base; the
numbers here are this spec's, at `0aa691375`, and they rot.

## 8. Verification each migration owes

- The three kind enums are public API. The migration must not change a
  single variant name, a single arm's payload, or the `Debug` spelling
  anything pins. `crates/pncad-py/src/tests.rs` pins
  `BooleanErrorKind::ClassificationInvariant` and
  `ProductErrorKind::NoBodyRoots`, `crates/editor-core/tests/switch_slots.rs`
  pins `PathErrorKind::NonpositiveCircleRadius`, and `path_error_tag`
  pins all 31 path tags. **A pin that moves is a migration that changed
  behaviour**, not a baseline to restore.
- A code-tier run, full matrix. The diff is three kernel crates.
- The `panic-free-macro-bodies.sh` gate reaches inside `macro_rules!`
  bodies because clippy does not; `error_kinds!`'s body must stay clean
  under it, which a `match` with no calls is.

## 9. The proc-macro precedent, read at the clause

The item asserts *"No design question for Ev"* and cites
`scripts/gates/README.md` (~:49) as Ev rejecting a proc-macro
(ratified 2026-09-06, PR 2067). **The clause was read.** It says a
proc-macro *"sees only the token stream of the item it is attached to,
so enforcing anything with it means annotating every generic item in
the kernel. A rule that holds only where someone remembered to opt in
is not a rule, and the omission is invisible."*

**The distinction holds, on the clause's own terms.** The rejection is
of a mechanism for *enforcing an invariant everywhere*, where opting in
is optional and forgetting is invisible. Generation is the opposite
case on exactly that axis: forgetting the macro leaves no kind enum, no
`kind()`, and a red build at the first consumer. And the clause is
about a proc-macro; `macro_rules!` is a different mechanism, with 39
instances in non-test source today.

**The conclusion drawn from it does not follow.** "The proc-macro
objection does not reach this" is true and is not the same claim as
"no design question for Ev". §6's five choices are design choices, and
they arrive whichever way that clause reads.
