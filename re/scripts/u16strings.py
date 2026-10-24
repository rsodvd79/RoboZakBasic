import re, sys
b = open(sys.argv[1], "rb").read()
for m in re.finditer(rb"(?:[\x20-\x7e]\x00){3,}", b):
    print(hex(m.start()), m.group().decode("utf-16le"))
for m in re.finditer(rb"(?:(?:[\x00-\xff][\xac-\xd7]|[\x20-\x7e]\x00)){3,}", b):
    s = m.group()
    try:
        t = s.decode("utf-16le")
        if re.search(r"[\uac00-\ud7a3]", t): print(hex(m.start()), "KO", t)
    except Exception: pass
