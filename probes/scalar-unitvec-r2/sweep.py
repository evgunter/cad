import os, re, sys
root = sys.argv[1]
roots = [os.path.join(root,'crates'), os.path.join(root,'demos'), os.path.join(root,'tools')]
hits=[]
for base in roots:
    for dp,dn,fn in os.walk(base):
        if '/tests' in dp or '/target' in dp or '/examples' in dp: continue
        for f in fn:
            if not f.endswith('.rs'): continue
            p=os.path.join(dp,f)
            lines=open(p,encoding='utf-8',errors='replace').read().split('\n')
            # mark cfg(test) regions crudely: from a line '#[cfg(test)]' followed by mod, to EOF-ish
            cut=len(lines)
            for i,l in enumerate(lines):
                if l.strip()=='#[cfg(test)]' and i+2<len(lines) and ('mod ' in lines[i+1] or 'mod ' in lines[i+2]):
                    cut=min(cut,i); break
            for i,l in enumerate(lines[:cut]):
                m=re.match(r'\s*(pub(\(.*?\))?\s+)?(unsafe\s+)?(async\s+)?fn\s+(\w+)', l)
                if not m: continue
                name=m.group(5)
                # gather doc block immediately above
                j=i-1; doc=[]
                while j>=0 and (lines[j].strip().startswith('///') or lines[j].strip().startswith('#[')):
                    if lines[j].strip().startswith('///'): doc.append(lines[j])
                    j-=1
                doc='\n'.join(reversed(doc))
                # gather signature until the line whose brace balance closes params
                sig=[]; depth=0; k=i
                while k<len(lines) and k<i+25:
                    sig.append(lines[k]); depth+=lines[k].count('(')-lines[k].count(')')
                    if depth<=0 and k>i-1 and '(' in '\n'.join(sig): break
                    k+=1
                sig='\n'.join(sig)
                dl=doc.lower()
                if not re.search(r'\bunit\b|normali[sz]ed|\bdirection\b|\bnormal\b|\baxis\b', dl): continue
                if not re.search(r'Vec3<|Vec2<|UnitVec3|Dir<|SpanBox<|\[f64; 3\]', sig): continue
                hits.append((p.replace(root+'/',''), i+1, name, re.search(r'\bunit\b', dl) is not None))
print(len(hits),"hits")
for h in sorted(hits):
    print(f"{h[0]}:{h[1]} {h[2]}{'  [unit]' if h[3] else ''}")
