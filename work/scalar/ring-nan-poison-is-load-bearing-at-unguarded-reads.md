---
id: ring-nan-poison-is-load-bearing-at-unguarded-reads
kind: issue
title: "The ring's NaN poison is load-bearing at 31 unguarded reads: RING-2's decoration swap launders a refusal into a pass there, and this row is the re-runnable register"
status: open
opened: 2026-09-21
priority: P2
cost: D
refs: [H5]
---


## What

RING-0's deliverable (c) swept every production read of one side of a
`RingInterval` bracket. Most are guarded — the site asks `is_poison()`
first, and `is_poison()` survives H5 ruling 1 cut (ii) as `dec < Def`.
**Thirty-one are not**, and each of them refuses today only because a
poisoned endpoint is NaN and every comparison against NaN is false. A
`DInterval` at `Trv` carries REAL endpoints
(`geom-core/tests/certified_door.rs`'s
`a_backend_refusal_can_carry_real_endpoints` is the witness), so at each
of these the same code takes the certifying branch on a value that may
not certify.

This row is a **register, not a prose list**: §Sweep below carries the
one command that regenerates it and the output that command produced,
so RING-2 re-runs a command rather than repeating a hand sweep. The
executable census — a lint or a test that fails when an unguarded read
is added — is RING-2's to write with the newtype, because it needs the
newtype's own accessor to be checkable.

## Three shapes

**(1) One side read without asking poison — 20 sites.** The refusal IS
the NaN comparison.

| site | expression | producer | can follow a ring division? |
| --- | --- | --- | --- |
| `geom-brep/offset_meters.rs` `mig` (`:261-263`) | `i.lo() > 0.0` / `i.hi() < 0.0`, documented as answering `0.0` for poison "whose comparisons are all false" | `PatchGrid::chan` coefficient hulls | no — sums and products only |
| `geom-brep/offset_meters.rs` `norm_sup` (`:318`) | `sqrt_up(norm_sq(v).hi())` | `norm_sq` of a hull row | no |
| `geom-brep/offset_meters.rs` (`:351`) | `sqrt_down(sq.lo())` | same | no |
| `geom-brep/offset_meters.rs` (`:369`) | `(proj / RingInterval::point(dn)).lo().max(0.0)` | **the ring division on the same line** | **yes** — immediately after one, and `.max(0.0)` is what absorbs today's NaN. Same shape as `mig`, same file |
| `geom-brep/offset_meters.rs` (`:609`) | `sqrt_up((h.sqr() - k).hi().max(0.0))` | `h`, `k` from the Weingarten hull | no |
| `geom-brep/props/quad.rs` area gauge fallback | `area.lo() > 0.0` | the quadrature area bracket | **yes** — the rational-patch weight division |
| `geom-brep/props/quad.rs` rational-weight gate | `g_w.lo() <= 0.0 \|\| !g_w.lo().is_finite()` | `PatchGrid::chan` | no. The `is_finite` arm catches NaN and ±inf, not a finite `Trv` bracket |
| `geom-brep/props/quad.rs` per-cell weight twin | same shape | `CellHulls::cell_hull` | no |
| `geom-brep/props/quad.rs` `norm_hi` (`:2120`) | `sqrt_enclosure(...).hi()` | any bracketed 3-vector | **yes** at a rational-patch caller |
| `geom-brep/props/quad.rs` `boundary_chord_perimeter_lo` (`:2229`) | `p.lo().max(0.0)` — `f64::max(NaN, 0.0)` is `0.0`, the refusing direction for a lower bound | a sum of `sqrt_enclosure` of `eval` differences | **yes** — the same rational-weight path as `area.lo()` |
| `geom-brep/offset_fit.rs` `cell_bound` | `w_lo` (`:2154`), `wt_lo` (`:2162`) bound to locals and tested `!(w_lo > 0.0) \|\| !w_lo.is_finite()` | coefficient hulls | no |
| `geom-brep/offset_fit.rs` `cell_bound` | the sign witness `dh.lo() > 0.0` / `dh.hi() < 0.0` (`:2171-2173`), no guard at all | same | no |
| `geom-brep/offset_fit.rs` (`:2183`) | `e_mig_iv.lo().max(e_proj_iv.lo())` | `e_mig_iv` is a re-mint **divided by `wt`** (see shape 2) | **yes** |
| `geom-brep/offset_fit.rs` (`:2204`) | `e_hull_lo.max(d.abs() - dist_iv.hi())` | `dist_iv` | **yes**, same chain |
| `geom-brep/offset_fit.rs` (`:2209`) | `bound.hi()` | same | **yes** |
| `geom-brep/ssi/certify.rs` (`:692`) | `(t.x.powi(2) + t.y.powi(2)).sqrt().hi()` | the transversality tangent | **yes** — `certify.rs` divides upstream |
| `mesh/nurbs_cert.rs` `cell_component` (`:497`) | `sq.hi()` consumed as `hi.sqrt().next_up()`, with "poison answers NaN, which every consumer treats as unbounded/poisoned" as the stated contract | cell hulls | no |
| `mesh/chords.rs` (`:286`, `:498`) | `sum_sq.hi().sqrt().next_up()` and `s.hi().sqrt().next_up()`, same contract | chord sums | no |

(The three `offset_fit.rs` rows at `:2183`, `:2204`, `:2209` and the two
`chords.rs` rows are counted individually above: 20 sites over 14 rows.)

**(2) A ring value re-minted from its own endpoints — 8 sites.**
`RingInterval::from_bounds(x.lo(), x.hi())` is poison-preserving only
because the endpoints are NaN; under cut (ii) it mints a `Com` bracket
out of a `Trv` one, which is laundering of exactly the kind
`from_certified` exists to prevent.

- `geom-brep/props/quad.rs`, three: `cos_step`'s and `sin_step`'s
  `from_bounds(lo.lo(), hi.hi())`, and `sin_step`'s sign mirror
  `from_bounds(-unit.hi(), -unit.lo())`.
- `geom-brep/ssi/enclose.rs`, two: `Box3::split`'s `half` closure,
  `from_bounds(i.lo(), m)` and `from_bounds(m, i.hi())`.
- `topo/src/props.rs` (`:2505`), one: `trig_at_start`'s `clamp`
  closure, `from_bounds(x.lo() - pad, x.hi() + pad)`. Divides upstream.
- `geom-brep/offset_fit.rs` (`:2131`), one:
  `RingInterval::point(sqrt_down(e_mig_sq.lo())) / wt` — a re-mint that
  is then **divided**, so it both launders and produces.
- `geom-core/src/spline/compose/tensor.rs` (`:572`→`:578`), one:
  `carrier.coords[d][0].lo()` bound to `center` and fed to
  `RingInterval::point(shift)`. Poison is load-bearing here —
  `point(NaN)` poisons — and a `Trv` mints a clean `Com`. A file no
  earlier sweep visited.

`quad.rs`'s `widen` has the same shape and is **not** in the list: it
asks `is_poison()` first, which is what every site above should do.

**(3) Two sides read and consumed arithmetically — 3 sites.** Outside
every pattern the first sweep used: the read is not a comparison and
not a re-mint, it is a midpoint. NaN today, a finite number under
`Trv`.

- `topo/src/props.rs` (`:2491`) `mid_pad`, `((x.lo() + x.hi()) * 0.5, (x.hi() - x.lo()) * 0.5)`,
  re-minted through `T::from_f64`. Its input is the quad lane's
  `bounds` whether or not `refusal` is `Some`.
- `geom-brep/ssi/enclose.rs` (`:186-190`) `Box3::center`.
- `geom-brep/offset_meters.rs` (`:356`) the `mid` closure,
  `(i.lo() + i.hi()) * 0.5`.

## Two sites that are conditional, not hazards

`topo/src/props.rs` (`:2731`, the exact-structure read
`x.lo() == x.hi() && x.lo().is_finite()`) and its twin at `:2966` were
both on the first list. **Neither can launder as the dry run spells cut
(ii)**: their only producer is `RingInterval::from_certified`
(`:2769-2776` and the `ring` closure at `:2927`), which refuses an
uncertified scalar as `poison()` = NaI under the newtype, so the value
reaching the comparison is Com or NaN in both arithmetics.

They become hazards **only if RING-2 chooses a decorated bracket for
`from_certified` instead of NaI** — a choice neither the H5 ruling nor
RING-0's spec puts as a question, and one that silently decides whether
a third of this list is a hazard at all. RING-2's spec carries it.

## Three sites listed as safe, with the reason

- `geom-brep/props/quad.rs` (`:460`) and (`:524`): `.lo().max(-1.0)`
  re-minted, argued safe in the comment above `:456` — the operand
  reaching them is poison-free by the `span`/`pieces` guard and the
  only poisonable operand goes through `clamped_to`. Listed because a
  reader sweeping for the shape will find them and should not have to
  re-derive the argument.
- `geom-brep/props/quad.rs` (`:1158`) `sqrt_enclosure`: reads both
  endpoints, and opens with `if x.is_poison() { return x; }`. The guard
  every site in shape 1 is missing.

## What a `Trv` with real endpoints can come from

Inside the ring, **division and a negative `powi` (which is a division)
are the only producers** of a sub-`Def` bracket with real endpoints:
`from_certified` gives NaI, `from_bounds` mints Com or Dac, `+ − ×`
propagate the minimum decoration of their operands, `clamped_to` caps
at the operand's own decoration and `hull` takes the minimum. So a
`Trv` reaching any site above has a division behind it, and the
"division-reachable?" column is the whole question for RING-2.

That is narrower than this row first claimed. The sentence it used to
carry — *"the laundering does not need a division, and after cut (ii) a
domain clamp anywhere upstream produces one"* — describes **cut (iii)**,
where the ring dissolves into `Interval` and the domain-clamping
transcendentals (`acos`, `sqrt` of a straddling bracket) become
producers in their own right. It belongs to RING-3, not here.

## Sweep, and its blind spot

One command, from the repo root, run at the head this row is filed on:

```
python3 - <<'PY'
import re, subprocess
FILES = subprocess.run(["git", "grep", "-l", "RingInterval", "--", "crates/*/src/**"],
                       capture_output=True, text=True, check=True).stdout.split()
READ = re.compile(r"\.(lo|hi)\(\)")
CMP = re.compile(r"(==|!=|<=|>=|<|>)")

def production(text):
    """(lineno, line) for every line outside a #[cfg(test)] mod BLOCK."""
    lines = text.split("\n"); out = []; i = 0; n = len(lines)
    while i < n:
        if lines[i].strip().startswith("#[cfg(test)]"):
            j = i + 1
            while j < n and (lines[j].strip().startswith("#[") or not lines[j].strip()):
                j += 1
            if j < n and re.match(r"\s*(pub\s+)?mod\s", lines[j]):
                depth = 0; started = False; k = j
                while k < n:
                    depth += lines[k].count("{") - lines[k].count("}")
                    started |= "{" in lines[k]
                    if started and depth <= 0:
                        break
                    k += 1
                i = k + 1
                continue
        out.append((i + 1, lines[i])); i += 1
    return out

total = 0
for f in FILES:
    for ln, line in production(open(f).read()):
        s = line.strip()
        if s.startswith("//") or not READ.search(line):
            continue
        tag = ["CMP"] if CMP.search(line) else []
        if "from_bounds" in line or "::point(" in line:
            tag.append("REMINT")
        for kw in ("is_finite", "is_nan", "sqrt", ".max(", ".min(", "abs()",
                   "next_up", "next_down", "/"):
            if kw in line:
                tag.append(kw)
        print("%s:%d: [%s] %s" % (f, ln, "/".join(tag) or "bare", s[:120]))
        total += 1
print("TOTAL production lines with an endpoint read:", total)
PY
```

It visits 26 files and prints **121 lines**. A site spanning several
lines prints once per line, so lines are not sites; the classification
above is of those lines by hand. Eighteen of them are not ring reads at
all — 16 inside `geom-core/src/interval.rs`, where the `Interval`
wrapper reads its own `DInterval`, and 2 inside `ring_interval.rs`
itself. The other 103 are ring reads, and they collapse to the 31
hazard sites, the 2 conditional and the 3 argued-safe named above plus
a remainder that is guarded by an `is_poison()` on an enclosing branch,
already holds an `f64`, or feeds a bracket a certified door re-checks.
Dispositioning that remainder by reading is exactly the step that let
five sites go missing the first time, which is why the executable
census belongs in RING-2 rather than in another hand pass.

**What the earlier sweep actually did, corrected.** The first version of
this row described its cut as "above each file's `#[cfg(test)]`". Taken
literally that truncates `topo/src/props.rs` at `:954` and
`mesh/nurbs_cert.rs` at `:887`, yet sites past both cuts were listed —
so that is not what ran. The command above states the real rule: skip
each `#[cfg(test)] mod` BLOCK by brace matching and carry on past it,
which reaches `quad.rs:5158` and `topo/props.rs:2966`.

**Blind spots, unchanged.** It cannot match a ring endpoint handed to a
helper that takes `f64` — the value is no longer a ring value where the
comparison happens — nor a read inside a macro body, nor a read split
across two lines by rustfmt.

## Disposition

Not RING-0's to fix: that unit changes no `src`, and the repair is a
decision about each door's contract that belongs with the newtype
itself. The fix at every site is one shape — ask the refusal first
(`is_poison()`, or `from_certified` where a value is being re-minted)
instead of relying on NaN. The RING-0 dry run
(`scalar/ring-0-dry-run`) reached **none** of these with a red row,
which is why they need naming rather than reading off a failure list.
