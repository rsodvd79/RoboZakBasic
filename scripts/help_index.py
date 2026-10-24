import glob, json, os, re, sys

d = sys.argv[1]
items = []
for f in sorted(glob.glob(d + "/*.md")):
    first = next((l for l in open(f, encoding="utf-8") if l.strip()), "")
    title = re.sub(r"[*#_`]|\s{2,}", " ", first).strip()[:60]
    items.append({"file": os.path.basename(f), "title": title or os.path.basename(f)})
json.dump(items, open(d + "/index.json", "w", encoding="utf-8"), ensure_ascii=False, indent=1)
print(len(items))
