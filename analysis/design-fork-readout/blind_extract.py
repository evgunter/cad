"""Write the blinded extract the coders read: rows.json minus the A/B
column, every model name redacted. Fails loudly if any survives."""
import json, re, sys

FIELDS = ["n", "date", "problem", "A", "B", "agree", "reconciliation",
          "to_ev", "ev", "match"]
NAMES = re.compile(r"opus|fable|sonnet|haiku|claude", re.I)


def main(src, dst, chunks):
    rows = json.load(open(src))
    out = []
    for r in rows:
        b = {"key": r["key"]}
        for f in FIELDS:
            b[f] = NAMES.sub("[model]", r[f])
        out.append(b)
    if NAMES.search(json.dumps(out)):
        sys.exit("model name survived redaction")
    k = int(chunks)
    size = -(-len(out) // k)
    for i in range(k):
        part = out[i * size:(i + 1) * size]
        with open(f"{dst}-{i + 1}.md", "w") as fh:
            for b in part:
                fh.write(f"## {b['key']} (row {b['n']}, {b['date']})\n\n")
                for f in FIELDS[2:]:
                    fh.write(f"**{f}**: {b[f]}\n\n")
        print(f"{dst}-{i + 1}.md", len(part), "rows",
              part[0]["key"], "…", part[-1]["key"])


if __name__ == "__main__":
    main(*sys.argv[1:])
