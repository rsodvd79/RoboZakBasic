import glob, os, re, shutil, sys
from markdownify import markdownify as md

src, dst = sys.argv[1], sys.argv[2]
os.makedirs(dst, exist_ok=True)

def read(p):
    b = open(p, "rb").read()
    for enc in ("utf-8", "cp949"):
        try:
            return b.decode(enc)
        except UnicodeDecodeError:
            pass
    return b.decode("cp949", "replace")

from urllib.parse import unquote

def slug(rel):
    return re.sub(r"[^\w.-]+", "_", rel.replace("/", "__"), flags=re.U)

out = []
for p in sorted(glob.glob(src + "/**/*.htm", recursive=True)):
    html = read(p)
    html = re.sub(r"<style.*?</style>|<!--.*?-->|<script.*?</script>", "", html, flags=re.S)
    m = re.search(r"<body[^>]*>(.*)</body>", html, re.S | re.I)
    text = md(m.group(1) if m else html, heading_style="ATX")
    text = re.sub(r"\n{3,}", "\n\n", text).strip()
    rel = os.path.relpath(p, src)
    name = slug(rel)[:-4] + ".md"
    for d in glob.glob(os.path.dirname(p) + "/*.files"):
        shutil.copytree(d, dst + "/" + os.path.basename(d), dirs_exist_ok=True)
    text = re.sub(r"\]\(([^)]*?)\.htm\)", lambda m: "](" + slug(unquote(m.group(1))) + ".md)", text)
    open(f"{dst}/{name}", "w", encoding="utf-8").write(text + "\n")
    out.append(name)
print(len(out), "files")
