"""Registra WriteFile/ReadFile dell'IDE su una porta COM (bytes + timestamp).
Uso: python frida_serial.py <pid> <secondi> > log.txt"""
import frida, sys, time

pid = int(sys.argv[1])
dur = float(sys.argv[2])
s = frida.attach(pid)
js = r"""
const k = Process.getModuleByName('kernel32.dll');
function hex(p, n) { return n > 0 && n < 4096 ? Array.from(new Uint8Array(p.readByteArray(n))).map(b => ('0' + b.toString(16)).slice(-2)).join(' ') : ''; }
Interceptor.attach(k.getExportByName('WriteFile'), {
  onEnter(a) { this.h = a[0]; this.p = a[1]; this.n = a[2].toInt32(); },
  onLeave(r) { send('TX h=' + this.h + ' n=' + this.n + ' ' + hex(this.p, this.n)); } });
Interceptor.attach(k.getExportByName('ReadFile'), {
  onEnter(a) { this.h = a[0]; this.p = a[1]; this.np = a[3]; },
  onLeave(r) { const n = this.np.isNull() ? 0 : this.np.readU32(); if (n) send('RX h=' + this.h + ' n=' + n + ' ' + hex(this.p, n)); } });
"""
sc = s.create_script(js)
sc.on('message', lambda m, d: print('%.3f' % time.time(), m.get('payload', m), flush=True))
sc.load()
time.sleep(dur)
