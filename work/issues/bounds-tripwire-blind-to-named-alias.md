---
id: bounds-tripwire-blind-to-named-alias
kind: issue
title: Discipline tripwire — teach the compound-Bounds grep the ArcCarrierScalar alias (and named compound bounds generally)
status: open
opened: 2026-08-09
github: 279
refs: [273]
---

## From GitHub issue 279

Opened 2026-08-09; 0 comments.

## What

The compound-`Bounds` discipline tripwire (`.github/workflows/ci.yml`
"Bounds compound-bound allowlist", mirrored in `scripts/ci-local.sh`)
greps for the literal spelling `+ Bounds`. SWITCH-P (#273) introduced a
NAMED alias for that obligation:

```rust
// crates/profile/src/path/arc_fillet.rs  (an allowlisted file)
pub trait ArcCarrierScalar: Decide + Bounds {}
impl<T: Decide + Bounds> ArcCarrierScalar for T {}
```

so the replay driver can call the G2 arrival binders (`at_on`, `to_on` —
genuinely `Decide + Bounds`) while `crates/profile/src/path/program.rs`
stays free of the literal compound spelling and therefore stays
**un-allowlisted**, which is the strictly safer of the two routes: a
literal `+ Bounds` appearing in `program.rs` still trips the wire.

## The gap (probe-proven by the #273 reviewer)

The alias is an unmonitored second spelling. A bracket-reading function
written against `T: ArcCarrierScalar` **anywhere in the workspace**
compiles and passes the full tripwire pipeline:

```rust
fn f<T: ArcCarrierScalar>(x: T) -> f64 { use geom_core::Bounds; x.lo() }
```

The delivered driver does not do this (verified: `program.rs` contains
zero `Bounds` / `.lo()` / `.hi()` tokens), so this is *capability*, not
evasion — but the discipline should not depend on nobody using it.

## Asked for

1. Teach the tripwire the alias: treat `\bArcCarrierScalar\b` outside
   `crates/profile/src/path/{arc_fillet,program}.rs` as a compound
   spelling, failing the same way `+ Bounds` does. Both the hosted grep
   and the `ci-local.sh` mirror.
2. Note the alias in `geom-core/src/real.rs`'s Bounds scope rule, so the
   ratified text covers named forms of the compound bound and not only
   the literal one.
3. Consider generalising: any `trait X: … + Bounds` declared outside the
   allowlisted files is the same hazard, so a grep for `:\s*.*\+\s*Bounds`
   in trait DECLARATIONS may be the durable form.

## Not in scope for #273

CI edits and a `real.rs` ratification are both outside SWITCH-P's fence
(`docs/LIB-SWITCH-SPEC.md` §9: "no CI edits"), and the reviewer endorsed
the alias route explicitly ("keep the alias; do NOT allowlist
program.rs"). This is its own ratification PR.

Refs: PR #273; review verdict APPROVE-WITH-FIXES, MINOR-2.

## Home

`work/issues/`: the fix is a `.github/workflows/ci.yml` + `scripts/ci-local.sh` tripwire edit plus a `geom-core/src/real.rs` ratification — S-QA's old ground, and S-QA is closed.
