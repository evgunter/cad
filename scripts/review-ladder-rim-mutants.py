#!/usr/bin/env python3
"""Plant one source-argument mutant in `names::emit_blend::name_blend`.

A review probe for PR #2794 (`crates/editor-core/tests/edit_ladder_rim.rs`).
The suite's claim is that each of its four rows reads the ARGUMENT a blend
name carries, not merely that the name exists; the only way to measure that
is to make an argument wrong while leaving the minted keys and the table's
totality untouched, which is what this does.

It rewrites the SOURCE half of one `BlendNaming` channel's rows in place --
`rim_feet`, `meridian_splits`, `bands` or `slits` -- and leaves the minted
key half alone, so `check_total` and every count stay green and only a row
that reads the argument can see the change.

    python3 scripts/review-ladder-rim-mutants.py <channel> <op>
    python3 scripts/review-ladder-rim-mutants.py revert

Ops:
    rotate    rotate the channel's source arguments left by one
    swaprims  swap sources 0<->2 and 1<->3 (the two rims' feet, across rims)
    swapin    swap sources 0<->1 (a permutation inside ONE rim)
    drop1     drop one member of each source SET (`bands` only)
    droprow   drop a whole record, so the mint loses its name entirely

Then run: `cargo test -p editor-core --test all`.
"""

import pathlib
import subprocess
import sys

ROOT = pathlib.Path(__file__).resolve().parent.parent
SRC = ROOT / "crates/editor-core/src/names/emit_blend.rs"
ANCHOR = "    let up = |key: EntityKey| upstream_name(target, target_node, ent(0, key));"

BODIES = {
    "rotate": (
        "let mut s: Vec<_> = __m.{c}.iter().map(|x| x.1.clone()).collect();"
        " if s.len() > 1 {{ s.rotate_left(1); }}"
        " for (i, r) in __m.{c}.iter_mut().enumerate() {{ r.1 = s[i].clone(); }}"
    ),
    "swaprims": (
        "if __m.{c}.len() == 4 {{"
        " let mut s: Vec<_> = __m.{c}.iter().map(|x| x.1.clone()).collect();"
        " s.swap(0, 2); s.swap(1, 3);"
        " for (i, r) in __m.{c}.iter_mut().enumerate() {{ r.1 = s[i].clone(); }} }}"
    ),
    "swapin": (
        "if __m.{c}.len() >= 2 {{"
        " let a = __m.{c}[0].1.clone(); let b = __m.{c}[1].1.clone();"
        " __m.{c}[0].1 = b; __m.{c}[1].1 = a; }}"
    ),
    "drop1": "for r in __m.{c}.iter_mut() {{ if r.1.len() > 1 {{ r.1.pop(); }} }}",
    "droprow": "if __m.{c}.len() == 4 {{ __m.{c}.remove(0); }}",
}


def pristine() -> str:
    out = subprocess.run(
        ["git", "-C", str(ROOT), "show", "HEAD:crates/editor-core/src/names/emit_blend.rs"],
        check=True,
        capture_output=True,
        text=True,
    )
    return out.stdout


def main() -> None:
    if len(sys.argv) == 2 and sys.argv[1] == "revert":
        SRC.write_text(pristine())
        print("reverted")
        return
    if len(sys.argv) != 3 or sys.argv[2] not in BODIES:
        print(__doc__)
        raise SystemExit(2)
    chan, op = sys.argv[1], sys.argv[2]
    text = pristine()
    if ANCHOR not in text:
        raise SystemExit("anchor line not found; emit_blend.rs has moved")
    block = (
        "    #[allow(unused_mut, unused_variables)]\n"
        "    let mut __m = rec.clone();\n"
        "    { " + BODIES[op].format(c=chan) + " }\n"
        "    let rec = &__m;\n"
    )
    SRC.write_text(text.replace(ANCHOR, block + ANCHOR, 1))
    print("planted", chan, op)


if __name__ == "__main__":
    main()
