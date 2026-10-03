"""Execute every Python block in the guide (LIB-U10).

The guide's prose lives in Markdown under `docs/`, and its Rust blocks
are doctests because `crates/pncad/src/guide.rs` pulls the same files
into rustdoc with `include_str!`. This module is the other half of
that arrangement: it reads those same Markdown files and runs every
```python block in them.

Nothing in the guide is transcribed into a test. The Markdown IS the
test input, so a guide example cannot rot without turning a build red.

The one invariant a guide author must respect: **each ```python block
is a complete, self-contained program** — its own imports, its own
setup — and it must run to completion without raising. A block that
demonstrates a refusal catches that refusal itself, which is what a
reader would have to do anyway.
"""

import re
import unittest
from pathlib import Path

ROOT = Path(__file__).resolve().parents[3]

PAGES = [
    ROOT / "docs" / "GUIDE.md",
    ROOT / "docs" / "guide" / "examples.md",
    ROOT / "docs" / "guide" / "fail-loud.md",
    # REGISTERED, not covered: `selecting.md` is the Rust half of the
    # guide and has no ```python blocks today, so its only assertion
    # here is that the file exists. It is listed so a Python block
    # added to it is executed from the first commit rather than from
    # whenever someone remembers this list.
    ROOT / "docs" / "guide" / "selecting.md",
    ROOT / "docs" / "guide" / "meshing.md",
    ROOT / "docs" / "guide" / "assembly.md",
    ROOT / "docs" / "guide" / "north-star-audit.md",
    # The crate front door: its example is held to the same standard.
    ROOT / "crates" / "pncad-py" / "README.md",
]

# ```python ... ``` at the start of a line, non-greedy, capturing the body
# and the line number of the opening fence.
FENCE = re.compile(r"^```python\n(.*?)^```$", re.MULTILINE | re.DOTALL)


def blocks(page):
    """Yield (line_number, source) for each python block in `page`."""
    text = page.read_text(encoding="utf-8")
    for match in FENCE.finditer(text):
        line = text.count("\n", 0, match.start()) + 1
        yield line, match.group(1)


class TestGuideBlocksExecute(unittest.TestCase):
    def test_every_python_block_in_the_guide_runs(self):
        ran = 0
        for page in PAGES:
            self.assertTrue(page.is_file(), f"missing guide page: {page}")
            for line, source in blocks(page):
                where = f"{page.relative_to(ROOT)}:{line}"
                with self.subTest(block=where):
                    code = compile(source, where, "exec")
                    # A fresh namespace per block: blocks are independent
                    # programs, never a chain that silently shares state.
                    # The S102 marker below: running the guide's own code
                    # blocks IS this test — the source is a committed docs
                    # page, never input.
                    exec(code, {"__name__": "__guide__"})  # noqa: S102
                ran += 1
        self.assertGreater(ran, 0, "the guide has no python blocks to run")

    def test_the_check_is_not_vacuous(self):
        """The guide really does carry Python, and we really do find it."""
        found = list(blocks(ROOT / "docs" / "GUIDE.md"))
        self.assertGreaterEqual(
            len(found), 3, "GUIDE.md should carry the Python journey"
        )
        self.assertTrue(
            any("import_step" in source for _, source in found),
            "the Python journey should reach the export round-trip",
        )


if __name__ == "__main__":
    unittest.main()
