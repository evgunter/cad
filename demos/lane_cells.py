"""The file vocabulary of a committed render lane's cells.

A cell named NAME is a frame, `NAME.png` or `NAME.svg`, optionally with a
sidecar: `NAME.fail.txt` (the scene drew nothing; the reason, shown as a
labeled placeholder in place of the frame) or `NAME.note.txt` (the frame
is missing something on purpose; stamped neutrally over it). The writers
(`render.py`, `render_freecad.py`, `render_gui.py`), the composer
(`compose_montage.py`) and the publisher (`publish_lane.py`) spell these
suffixes from here; `render.sh`'s one `.fail.txt` write is shell and
spells it itself, and a misspelling there is not a cell, so the
publisher's census refuses the pass.
"""

PNG = ".png"
SVG = ".svg"
FRAMES = (PNG, SVG)
FAIL = ".fail.txt"
NOTE = ".note.txt"
SIDECARS = (FAIL, NOTE)


def frame_path(directory, name, suffix=PNG):
    return directory / f"{name}{suffix}"


def fail_path(directory, name):
    return directory / f"{name}{FAIL}"


def note_path(directory, name):
    return directory / f"{name}{NOTE}"


def split(filename):
    """(NAME, suffix) for a cell file's name, or None for any other file."""
    for suffix in (*SIDECARS, *FRAMES):
        if filename.endswith(suffix) and len(filename) > len(suffix):
            return filename[: -len(suffix)], suffix
    return None
