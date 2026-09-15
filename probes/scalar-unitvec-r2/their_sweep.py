import os, re, sys
root = sys.argv[1]
hits=[]
for base in ['crates','demos','tools']:
    b=os.path.join(root,base)
    if not os.path.isdir(b): continue
    for dp,dn,fn in os.walk(b):
        if os.sep+'tests' in dp or os.sep+'target' in dp: continue
        for f in fn:
            if not f.endswith('.rs'): continue
            p=os.path.join(dp,f)
            lines=open(p,encoding='utf-8',errors='replace').read().split('\n')
            cut=len(lines)
            for i,l in enumerate(lines):
                if l.strip()=='#[cfg(test)]' and i+2<len(lines) and ('mod ' in lines[i+1] or 'mod ' in lines[i+2]):
                    cut=i; break
            for i,l in enumerate(lines[:cut]):
                if not re.search(r'\bfn\s+\w+', l): continue
                if 'Vec3<' not in l: continue
                above=lines[max(0,i-8):i]
                if any('///' in a and re.search(r'unit', a, re.I) for a in above):
                    name=re.search(r'\bfn\s+(\w+)', l).group(1)
                    hits.append((p.replace(root+'/',''), i+1, name))
print(len(hits),"hits")
for h in sorted(hits): print(f"{h[0]}:{h[1]} {h[2]}")
