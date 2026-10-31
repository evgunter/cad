"""rows.json -> key.csv: the only file that knows the models.

a_model is read from the row's A/B column; a row with a urandom byte
whose parity disagrees with the stated mapping fails loudly.
deviation marks rows whose labels or blinding departed from protocol
(no byte drawn, or the log's notes say blinding broke)."""
import csv, json, re, sys

BROKEN = {"2"}  # row 2's note: chat updates named the models before Ev decided


def a_model(ab):
    t = ab.lower()
    for pat in (r"\ba\s*(?:=|was|is|:)?\s*(opus|fable)\b",
                r"(opus|fable)\s*(?:=|was|is)?\s*a\b"):
        m = re.search(pat, t)
        if m:
            return m.group(1)
    return ""


def main(src, dst):
    rows = json.load(open(src))
    with open(dst, "w", newline="") as fh:
        w = csv.writer(fh)
        w.writerow(["key", "n", "date", "a_model", "byte", "deviation"])
        for r in rows:
            a = a_model(r["ab"])
            m = re.search(r"byte\s*(\d+)", r["ab"])
            byte = m.group(1) if m else ""
            if byte and a and (int(byte) % 2 == 0) != (a == "opus"):
                sys.exit(f"{r['key']}: byte {byte} contradicts A = {a}")
            dev = []
            if a and not byte and "no byte" in r["ab"].lower():
                dev.append("no-byte")
            if r["n"] in BROKEN and r["line"] == 28:
                dev.append("blinding-broken")
            w.writerow([r["key"], r["n"], r["date"], a, byte, ";".join(dev)])


if __name__ == "__main__":
    main(*sys.argv[1:])
