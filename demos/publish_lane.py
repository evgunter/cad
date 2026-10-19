#!/usr/bin/env python3
"""Publish one render pass into its committed lane directory, whole.

Usage: python publish_lane.py [--frame=.png|.svg] [--sidecars] [--no-provenance]
                              <stage> <lane> NAME [NAME ...]
       python publish_lane.py --selftest

Every render lane calls this once its pass has finished in an untracked
staging directory (`render.sh`: `out/stage/<lane>/`; `render-wild.sh`:
`out/stage/renders-wild/`; `render-uv.sh`: `out/stage/renders-uv/`;
`render_gui.py`: `out/gui/`; `render-mc.sh`: `out/mc/`, which the tour
writes). NAME is each cell the pass was asked to draw; a name given twice
counts once. The cell file vocabulary is `lane_cells.py`'s.

THE LANE GOES FROM ONE WHOLE PASS TO THE NEXT. Every committed cell
file in <lane> (any frame or sidecar suffix, the lane's sheet included)
is removed, then this pass's cells are copied in. A cell the pass did
not produce is therefore a deletion in `git status`, and
`.github/actions/rebaseline-lane` refuses to commit a deletion: a
renamed or retired cell is retired by a person deleting the old file in
a commit, never by the re-baseline. A sheet composed from the published
cells is recomposed by the caller after this returns.

THE NAMES ARE A CENSUS, CHECKED IN BOTH DIRECTIONS, BEFORE THE LANE IS
TOUCHED. Each NAME must be in the stage as exactly `NAME<frame>` (the
lane's one frame suffix, `--frame`, default `.png`) or, on a lane that
takes `--sidecars`, as `NAME.fail.txt`. Any other cell file in the stage
(a name nobody asked for, the other frame suffix, a sidecar on a lane
that takes none) is one the census does not name. Either mismatch
publishes nothing and exits 2. A name missing from the stage means the
pass did not finish, or a scene subset was asked to publish over the
whole lane; a cell the list does not name would otherwise never be
published, with nothing downstream able to tell.

SIDECARS (`--sidecars`, the two `render.sh` lanes):
  * `NAME.fail.txt` DISPLACES its frame and its note: the scene produced
    no frame this pass, and the sheet shows the labeled failure, not a
    stale cell.
  * `NAME.note.txt` travels BESIDE its frame (a frame that is missing
    something on purpose), so the sheet stamps the cell as declared.

Files in the stage with no cell suffix are not cells and are not published.

PROVENANCE. A lane `check_render_provenance.py` governs (its `LANE_DIRS`)
is certified by that guard after publishing, and a violation exits 1.
Any other lane must say `--no-provenance`; a lane that omits it, or a
governed lane that passes it, is refused with exit 2 before anything is
touched, so a lane cannot publish uncertified by being left off a list.
"""

import shutil
import sys
import tempfile
from pathlib import Path

import check_render_provenance
from lane_cells import FAIL, FRAMES, NOTE, PNG, SIDECARS, split


def refuse(lines):
    print("\n".join(lines), file=sys.stderr)
    raise SystemExit(2)


def plan(stage, names, frame, sidecars):
    """The stage files to publish, or exit 2 naming what does not match."""
    allowed = {frame, *SIDECARS} if sidecars else {frame}
    staged = {}
    for path in sorted(stage.iterdir()):
        parsed = path.is_file() and split(path.name)
        if parsed:
            staged.setdefault(parsed[0], {})[parsed[1]] = path
    # A name is drawn by its frame, or (on a sidecar lane) by its failure.
    drawn_by = {frame, FAIL} & allowed
    missing = [n for n in names if not drawn_by & staged.get(n, {}).keys()]
    extra = sorted(
        path.name
        for name, files in staged.items()
        for suffix, path in files.items()
        if name not in names or suffix not in allowed
    )
    if missing or extra:
        refuse(
            [f"publish_lane: {stage} is not the pass the census names; nothing was published."]
            + [f"  named, not drawn: {n}{frame}" for n in missing]
            + [f"  drawn, not named: {f}" for f in extra]
        )
    out = []
    for name in names:
        files = staged[name]
        if FAIL in files:
            out.append(files[FAIL])
            continue
        out.append(files[frame])
        if NOTE in files:
            out.append(files[NOTE])
    return out


def publish(stage, lane, names, frame=PNG, sidecars=False, provenance=True):
    stage, lane = Path(stage), Path(lane)
    names = list(dict.fromkeys(names))
    if not names:
        refuse([f"publish_lane: no cell names given for {lane}"])
    if frame not in FRAMES:
        refuse([f"publish_lane: --frame must be one of {', '.join(FRAMES)}, got {frame!r}"])
    governed = lane.resolve().name in check_render_provenance.LANE_DIRS
    if governed != provenance:
        refuse([
            f"publish_lane: {lane.name} is "
            + ("" if governed else "not ")
            + "a lane check_render_provenance.py governs (LANE_DIRS), so it "
            + ("cannot take --no-provenance." if governed else "must say --no-provenance.")
        ])
    files = plan(stage, names, frame, sidecars)
    lane.mkdir(parents=True, exist_ok=True)
    for old in lane.iterdir():
        if old.is_file() and split(old.name):
            old.unlink()
    for src in files:
        shutil.copyfile(src, lane / src.name)
    print(f"publish_lane: {len(names)} cell(s) -> {lane}")
    if governed:
        check_render_provenance.certify([lane])
    return [f.name for f in files]


# ---- self-test -----------------------------------------------------


def selftest():
    def tree(d):
        return {p.name: p.read_text() for p in sorted(d.iterdir())}

    def run(case, stage_files, lane_files, names, want, copied=None, **kw):
        with tempfile.TemporaryDirectory() as tmp:
            stage, lane = Path(tmp, "stage"), Path(tmp, "lane")
            for d, files in ((stage, stage_files), (lane, lane_files)):
                d.mkdir()
                for f, text in files.items():
                    (d / f).write_text(text)
            before = tree(lane)
            try:
                done = publish(stage, lane, names, provenance=False, **kw)
                got = tree(lane)
                assert copied is None or done == copied, f"{case}: copied {done}"
            except SystemExit as e:
                got = e.code
                assert tree(lane) == before, f"{case}: a refusal touched the lane"
            assert got == want, f"{case}: want {want!r}, got {got!r}"

    run("full pass", {"a.png": "new", "b.png": "new"}, {"a.png": "old"},
        ["a", "b"], {"a.png": "new", "b.png": "new"})
    run("missing name", {"a.png": "new"}, {"a.png": "old", "b.png": "old"}, ["a", "b"], 2)
    run("extra cell", {"a.png": "n", "c.png": "n"}, {"a.png": "old"}, ["a"], 2)
    run("fail displaces its frame; note beside its frame",
        {"a.png": "f", "a.fail.txt": "why", "b.png": "f", "b.note.txt": "nb"}, {"a.png": "old"},
        ["a", "b"], {"a.fail.txt": "why", "b.note.txt": "nb", "b.png": "f"}, sidecars=True)
    run("fail plus note: the fail alone", {"a.fail.txt": "why", "a.note.txt": "nb"}, {},
        ["a"], {"a.fail.txt": "why"}, sidecars=True)
    run("note alone is not a cell", {"a.note.txt": "nb"}, {}, ["a"], 2, sidecars=True)
    run("sidecar on a lane without sidecars", {"a.png": "f", "a.note.txt": "nb"}, {}, ["a"], 2)
    run("wholesale clear of a stale cell and sidecar",
        {"a.png": "f"}, {"a.png": "old", "gone.png": "x", "gone.note.txt": "x", "keep.md": "k"},
        ["a"], {"a.png": "f", "keep.md": "k"})
    run("svg frame", {"a.svg": "s"}, {"old.svg": "x"}, ["a"], {"a.svg": "s"}, frame=".svg")
    run("png beside the declared svg", {"a.svg": "s", "a.png": "p"}, {}, ["a"], 2, frame=".svg")
    run("svg where png is declared", {"a.svg": "s"}, {}, ["a"], 2)
    run("repeated names count once", {"a.png": "n"}, {}, ["a", "a"], {"a.png": "n"},
        copied=["a.png"])
    run("an unknown frame suffix", {"a.png": "n"}, {}, ["a"], 2, frame=".jpg")
    with tempfile.TemporaryDirectory() as tmp:
        stage = Path(tmp, "stage")
        stage.mkdir()
        (stage / "a.svg").write_text("s")
        for lane, provenance in ((Path(tmp, "renders-mc"), True), (Path(tmp, "renders"), False)):
            try:
                publish(stage, lane, ["a"], frame=".svg", provenance=provenance)
                raise AssertionError(f"provenance={provenance} into {lane.name} published")
            except SystemExit as e:
                assert e.code == 2 and not lane.exists(), (lane.name, e.code)
    print("publish_lane selftest: ok (15 cases)")


def main():
    args = sys.argv[1:]
    if args == ["--selftest"]:
        selftest()
        return
    kw = {}
    while args and args[0].startswith("--"):
        flag = args.pop(0)
        if flag.startswith("--frame="):
            kw["frame"] = flag.split("=", 1)[1]
        elif flag == "--sidecars":
            kw["sidecars"] = True
        elif flag == "--no-provenance":
            kw["provenance"] = False
        else:
            refuse([f"publish_lane: unknown flag {flag}", __doc__])
    if len(args) < 3:
        refuse([__doc__])
    publish(args[0], args[1], args[2:], **kw)


if __name__ == "__main__":
    main()
