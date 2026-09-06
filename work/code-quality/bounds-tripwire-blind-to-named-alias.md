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

## Re-homed to code-quality (2026-09-04, by the CIW orchestrator), and mostly answered

CIW picked this up as a slate item and is handing it back, because both
the file it targets and the answer to it moved.

**The tripwire is no longer in `ci.yml`.** It is
`scripts/gates/bounds-allowlist.sh`, invoked by the workflow at
`.github/workflows/ci.yml:979` and by the local half's gate-directory
loop. `scripts/gates/*` is Track K's ground and is named in CIW's
`keep_out`, so CIW cannot take this.

**Ask 1 is now argued against, in ratified gate prose.** That script's
header carries the alias question as **KNOWN GAP 3**, names
`ArcCarrierScalar` explicitly, writes out the exact roster-keyed matcher
this issue asks for —

```
gate_grep -E "[A-Za-z0-9_][[:space:]]*:[[:space:]]*(ArcCarrierScalar)\b\
             |\+[[:space:]]*(ArcCarrierScalar)\b"
```

— and then declines it, because on this tree it reds `family.rs` and
`program.rs`, whose every use is `T: ArcCarrierScalar`, character for
character the sole bracket bound `plant_sole_bracket_bounds` pins as
must-NOT-fire. The only close available is redding those two files and
allowlisting them, which is the cry-wolf-then-allowlist outcome S63
forbids. The invisibility is pinned by a fixture,
`plant_alias_uses_invisible`, which requires the gate to **pass** on
exactly these shapes and would red the day the matcher changed. So ask
1 is not an open question here; it is a decided one, and reopening it
means arguing with S63, not adding a regex.

**Ask 2 is still owed and is the live residue.** `geom-core/src/real.rs`'s
`Bounds` scope rule still covers only the literal compound spelling. The
ratified text should say that a NAMED compound bound is the same
obligation — one paragraph, in the crate that owns the rule.

**Ask 3 is the real subject and already has ids.** The gate's header
records it as `S158` / `D102`: this matcher anchors on `+`, and `+` is
one of several ways Rust spells a compound bound (`where T: Decide,
T: Bounds` is silent, as is the multi-line form rustfmt converges on
from a caught spelling). KNOWN GAP 4 states it with no mitigation and
says why a bigger regex is not the answer. Closing that is a redesign of
what the gate matches.

The uses this issue worried about remain capability rather than
evasion: `program.rs` still contains zero `Bounds` / `.lo()` / `.hi()`
tokens (re-checked 2026-09-04), while the alias has spread since filing
— it is now used in `crates/profile/src/path/family.rs` and re-exported
as far as `crates/pncad/src/profile.rs:54`.
