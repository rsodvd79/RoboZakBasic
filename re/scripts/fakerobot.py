"""Robot finto su COM10: fa l'eco di ogni byte e logga (timestamp, hex).
Uso: python fakerobot.py [porta] [log] [secondi]
Con RESP (hex) opzionale si possono inviare byte extra dopo un byte dato: --after F0=0102030405 06
"""
import sys, time, serial

port = sys.argv[1] if len(sys.argv) > 1 else "COM10"
log = sys.argv[2] if len(sys.argv) > 2 else "fakerobot.log"
secs = float(sys.argv[3]) if len(sys.argv) > 3 else 120
extra = {}
direct = {}
for a in sys.argv[4:]:
    k, v = a.split("=")
    if v.startswith("!"):
        direct[int(k, 16)] = bytes.fromhex(v[1:])
    else:
        extra[int(k, 16)] = bytes.fromhex(v)

s = serial.Serial(port, 115200, timeout=0.05)
t0 = time.time()
queue = []
with open(log, "w") as f:
    while time.time() - t0 < secs:
        d = s.read(256)
        for b in d:
            f.write(f"{time.time()-t0:8.3f} {b:02X}\n")
            if queue:
                # dopo un comando con risposta, ogni byte ricevuto produce il byte successivo
                r = queue.pop(0)
                s.write(bytes([r]))
                f.write(f"{time.time()-t0:8.3f} -> {r:02X}\n")
                continue
            if b in direct:
                # byte non confermato con eco: il robot risponde direttamente
                for x in direct[b]:
                    time.sleep(0.3)
                    s.write(bytes([x]))
                f.write(f"{time.time()-t0:8.3f} => {direct[b].hex()}\n")
                continue
            s.write(bytes([b]))
            if b in extra:
                queue = list(extra[b])
        f.flush()
