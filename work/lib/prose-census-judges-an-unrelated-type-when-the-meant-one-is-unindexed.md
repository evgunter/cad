---
id: prose-census-judges-an-unrelated-type-when-the-meant-one-is-unindexed
kind: issue
title: the prose census's bare-name fallback judges an unrelated same-name type when the meant one is declared through a macro metavariable, so 'never soundness' is false
status: open
opened: 2026-09-26
priority: P3
cost: D
refs: [prose-census-undecided-residue]
---

(GATHER lane, from PR 3259's review.) `crates/pncad-py/src/prose_census.rs`
claims, in the doc on `struct Types` (~509-511), that falling back to
the bare name *"costs precision and never soundness: rivals that
disagree still answer Undecided"*. That holds only when the meant type
is IN the index as one of the rivals. It fails when the meant type is
not indexed at all.

**How a type goes unindexed.** `type_table`'s declaration walk (~680-690)
reads the name after `struct `/`enum ` with `ident_at`, which takes a
run of identifier characters. A declaration written inside a
`macro_rules!` body as `pub enum $tag { .. }` yields no name (`$` is not
an identifier character), and the walk `continue`s. The item is skipped
outright; it is not even recorded as an `Unreadable` rival.
`Types::lookup` (~640-656) then fails `declaring_path` for the written
path and falls back to `self.named(bare_name)`. That judges every OTHER
type of that bare name in the tree, none of which the site meant.

**Repro, loud direction (observed).** PR 3259's commit `01b42413d`
declared `profile::Verb` as `pub enum $tag { .. }` inside
`tag_projections!`. Both `prose_census` tests went red on hosted CI
(run 36189598086, job `test (eps = 1e-12, 1/2)`):
`no_display_impl_renders_a_brace_shaped_payload_through_debug` reported
`crates/editor-core/src/persist/check.rs` / `ProgramFault` / `verb` as
BRACED. That field is `Option<profile::Verb>`, a fieldless tag, but the
fallback judged `crates/verbs/src/verb.rs`'s unrelated
`Verb<T> { Fillet { .. }, .. }`. The PR worked around it by spelling the
enum name literally at each call site (`0a912b360`), and the site went
back to Undecided.

**Repro, silent direction (never observed, because nothing would show
it).** Swap the shapes: the meant type is braced and declared through a
metavariable, and an unrelated same-name type elsewhere is fieldless.
The census then answers PROSE for a site that renders `A { x: .. }`
through `Debug`, and nothing reds. There is no roster row for this
direction. A minimal tree, in the style of the module's own
`rival_tree` tests:

```text
crates/a/src/lib.rs:  macro_rules! m { ($t:ident) => { pub enum $t { A { x: u32 } } } } m!(Kind);
crates/b/src/lib.rs:  pub enum Kind { A }
crates/c/src/lib.rs:  pub struct E { k: a::Kind }
                      impl Display for E { .. write!(f, "{:?}", self.k) .. }
```

Expected: Undecided (or Braced). The census answers from `b::Kind`, so
the site reads as prose.

**The blind-spot list omits it too.** The module docs' `macro_rules!`
entry (~102-105) says a macro-declared type *"parses to an item with no
fields"*. That is true of a literal name inside a macro body. It is not
true of a metavariable name, which is not indexed at all.

**Shape of a fix, for the taker:** either index `enum $x`/`struct $x` as
an `Unreadable` declaration at its module path, so that a qualified
written path resolves to it and answers Undecided; or refuse the
bare-name fallback when the written path is qualified and its leading
segment names an indexed crate. Either makes the "never soundness"
sentence true, and the silent-direction tree above is the test that
should go red first.
