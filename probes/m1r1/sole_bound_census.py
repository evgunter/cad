import re, sys, pathlib
root = pathlib.Path('/home/user/cad-wt-m1r1/crates')
def blank(src):
    out=[]; i=0; n=len(src); mode=None
    while i<n:
        c=src[i]
        if mode is None:
            if src.startswith('//',i):
                j=src.find('\n',i); j = n if j<0 else j
                out.append(' '*(j-i)); i=j; continue
            if src.startswith('/*',i):
                j=src.find('*/',i); j = n if j<0 else j+2
                out.append(''.join(ch if ch=='\n' else ' ' for ch in src[i:j])); i=j; continue
            if c=='"':
                j=i+1
                while j<n:
                    if src[j]=='\\': j+=2; continue
                    if src[j]=='"': j+=1; break
                    j+=1
                out.append(''.join(ch if ch=='\n' else ' ' for ch in src[i:j])); i=j; continue
            out.append(c); i+=1
        else: i+=1
    return ''.join(out)
hits=[]
for p in sorted(root.glob('*/src/**/*.rs')):
    src=p.read_text()
    b=blank(src)
    # find generic lists <...> after fn/impl/struct/trait/enum/type
    for m in re.finditer(r'\b(fn|impl|struct|trait|enum|type)\b(\s+[A-Za-z_][A-Za-z0-9_]*)?\s*<', b):
        start=m.end()-1
        depth=0; j=start
        while j<len(b):
            if b[j]=='<': depth+=1
            elif b[j]=='>':
                depth-=1
                if depth==0: break
            j+=1
        inner=b[start+1:j]
        # split top-level commas
        parts=[]; d=0; cur=''
        for ch in inner:
            if ch in '<([': d+=1
            elif ch in '>)]': d-=1
            if ch==',' and d==0: parts.append(cur); cur=''
            else: cur+=ch
        parts.append(cur)
        for part in parts:
            pm=re.match(r"^\s*([A-Za-z_][A-Za-z0-9_]*)\s*:\s*(.+)$", part.strip(), re.S)
            if not pm: continue
            bound=' '.join(pm.group(2).split())
            if bound in ('Bounds','Enclosure','geom_core::Bounds','crate::Bounds','geom_core::Enclosure'):
                line=b[:m.start()].count('\n')+1
                hits.append((str(p.relative_to('/home/user/cad-wt-m1r1')), line, m.group(1), bound))
for h in sorted(set(hits)):
    print(f"{h[0]}:{h[1]} [{h[2]}] {h[3]}")
print("TOTAL", len(set(hits)))
