#!/usr/bin/env python3
"""Publish one render pass into its committed lane directory, whole.

Usage: python publish_lane.py <stage> <lane> NAME [NAME ...]

Every render lane calls this once its pass has finished in an untracked
staging directory. NAME is each cell the pass was asked to draw.

THE LANE GOES FROM ONE WHOLE PASS TO THE NEXT. Every committed cell
file in <lane> (`*.png`, `*.svg`, `*.fail.txt`, `*.note.txt`, the
lane's sheet included) is removed, then this pass's cells are copied in.
A cell the pass did not produce is therefore a deletion in `git status`,
and `.github/actions/rebaseline-lane` refuses to commit a deletion: a
renamed or retired cell is retired by a person deleting the old file in
a commit, never by the re-baseline. The sheet is recomposed by the
caller after this returns.

A PARTIAL STAGE IS REFUSED, BEFORE THE LANE IS TOUCHED. The stage must
hold exactly the NAMEs: each one a frame (`NAME.png` or `NAME.svg`) or a
labeled failure (`NAME.fail.txt`), and no other cell. A name with
neither, or a cell no name asked for (a stale stage), publishes nothing
and exits 2, so publishing a subset of scenes cannot clear the rest.

  * `NAME.fail.txt` DISPLACES its frame: the scene produced no frame
    this pass, and the sheet shows the labeled failure, not a stale cell.
  * `NAME.note.txt` travels BESIDE its frame (a frame that rendered
    empty on purpose), so the sheet stamps the cell as declared.

Files in the stage with any other suffix are not cells and are not published.

PROVENANCE. When <lane> is one `check_render_provenance.py` governs
(its `LANE_DIRS`), the guard runs over the published directory before
this returns, and a violation exits 1.
"""

import shutil
import sys
from pathlib import Path

import check_render_provenance

FRAMES = (".png", ".svg")
FAIL = ".fail.txt"
NOTE = ".note.txt"
SIDES = (FAIL, NOTE)


def cell_name(path):
    """(name, suffix) for a cell file, or None for anything else."""
    for suffix in SIDES + FRAMES:
        if path.name.endswith(suffix):
            return path.name[: -len(suffix)], suffix
    return None


def cells_in(directory):
    out = {}
    for path in sorted(directory.iterdir()):
        parsed = path.is_file() and cell_name(path)
        if parsed:
            out.setdefault(parsed[0], {})[parsed[1]] = path
    return out


def plan(stage, names):
    """The files to publish, or SystemExit(2) naming what is missing/extra."""
    staged = cells_in(stage)
    wanted = set(names)
    missing = sorted(
        n for n in wanted
        if not any(s in staged.get(n, {}) for s in FRAMES + (FAIL,))
    )
    extra = sorted(set(staged) - wanted)
    if missing or extra:
        lines = [f"publish_lane: {stage} is not one whole pass; nothing was published."]
        lines += [f"  asked for, not drawn: {n}" for n in missing]
        lines += [f"  drawn, not asked for: {n}" for n in extra]
        print("\n".join(lines), file=sys.stderr)
        raise SystemExit(2)
    files_out = []
    for name in sorted(wanted):
        files = staged[name]
        if FAIL in files:
            files_out.append(files[FAIL])
            continue
        files_out += [files[s] for s in FRAMES if s in files]
        if NOTE in files:
            files_out.append(files[NOTE])
    return files_out


def publish(stage, lane, names):
    stage, lane = Path(stage), Path(lane)
    if not names:
        print(f"publish_lane: no cell names given for {lane}", file=sys.stderr)
        raise SystemExit(2)
    files_out = plan(stage, names)
    lane.mkdir(parents=True, exist_ok=True)
    for old in lane.iterdir():
        if old.is_file() and cell_name(old):
            old.unlink()
    for src in files_out:
        shutil.copyfile(src, lane / src.name)
    print(f"publish_lane: {len(names)} cell(s) -> {lane}")
    if lane.resolve().name in check_render_provenance.LANE_DIRS:
        check_render_provenance.certify([lane])


def main():
    args = sys.argv[1:]
    if len(args) < 3:
        print("usage: python publish_lane.py <stage> <lane> NAME [NAME ...]", file=sys.stderr)
        raise SystemExit(2)
    publish(args[0], args[1], args[2:])


if __name__ == "__main__":
    main()
