# CLEARTEXT — the clearance refusals say what they found

Four P1 rows, one unit. Program: CLEAR. Branch: `clear/cleartext`.
Review: single STYLE review. Read each row in full; its body carries
the evidence and the sites:
- `work/clear/clearance-refusal-names-one-face-twice-across-bodies.md`
- `work/clear/clearance-reports-a-no-bodies-payload-as-a-bad-body-index.md`
- `work/clear/named-face-scope-refuses-not-a-face-without-naming-what-it-found.md`
- `work/clear/clearance-window-selection-asks-how-many-before-what.md`

The rows predate SHELL-3 (#5909), SYMCLEAR (#5928) and SELFX (#5955),
so find each site in today's code:
- The engine's arms are `topo::clearance::EngineRefusal`, and SELFX
  added `SharedCell` and `CertificateArm` payloads to it.
- `editor_core::clearance::ClearanceRefusal` wraps the engine's arms.
- The measure door raises `topo::clearance::MinSeparationRefusal`.
- `NoAdmittedPair` is gone.
- pncad no longer re-exports `ClearanceRefusal`/`SelectionRefusal`.

If a row's site no longer exists, say so and close that part.

## What is true when this lands

1. **A refusal naming two faces names which body each is on** wherever
   the pair crosses bodies (`PoisonEnclosure`, and every other arm or
   payload that prints a pair). Two bare arena keys never print as the
   same subject.
   - `topo` has no recipe address. `EngineRefusal` therefore names the
     pair by side (the first or second `Side`), and `editor-core`
     renders the side as its selection.
   - Decide the shape. The renders stay one function per type.
2. **`EngineRefusal` and `ClearanceRefusal` implement `Display`.** Check
   by reading the impls, not by grep: the row explains why
   (`impl core::fmt::Display for` spellings). The rendered sentences
   reuse the `spoken` machinery editor-core already has, and do not
   start a second vocabulary.
3. **No typed cause is destroyed.**
   - The `NoSuchBody` relabel of `InterrogateError::NoBodies` carries
     the inner error, or gets an arm in its own words. Read PR 2378's
     argument first.
   - `ToleranceHasNoBand` carries its `BandError`, whose two arms want
     opposite repairs.
   - Sweep `clearance.rs` (both halves) and `measure.rs` for every
     spelling of a destroyed cause: `map_err(|_| …)`, `let … else`,
     `.ok()?` on a typed `Result`, and `unwrap_or` over an error. List
     every hit and what you did with it.
4. **The named scope asks WHAT before HOW MANY**, as `resolve_face` and
   `entity_of` do.
   - Look the name up, refuse an absent name, ask the name's kind, and
     only then split `Unique` from `Tied`.
   - A tie is not `Unresolved`: give it its own arm naming the
     candidates.
   - `NotAFace` names the kind it found, and a wrong body is its own
     refusal, not `NotAFace`.
5. Every changed rendering is pinned by a row that goes red on the old
   text. Re-pin goldens that move, and say in the PR what moved and why.

## Ground

- Both halves of the clearance engine (`editor-core` and `topo`),
  `measure.rs` and `names/interrogate.rs` if item 3 reaches the error
  type there.
- Announce any seam with `python3 scripts/work.py territory --base origin/main`.
- Read `docs/prompts/implementer-discipline.md` in full first.
- Never arm a waiter or a background chain. Anything that outlives
  600 s runs `setsid`-detached and is polled in the foreground.
- CI is the verification of record.

## Report

≤150 lines: per row, what changed; the item-3 hit list; the goldens
moved; the PR and its CI.
