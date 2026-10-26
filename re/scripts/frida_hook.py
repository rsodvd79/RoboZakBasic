import frida, sys, time
pid = int(sys.argv[1])
dur = float(sys.argv[2])
s = frida.attach(pid)
js = r"""
const err = ptr(0x67437a);
function h(addr, name) {
  Interceptor.attach(ptr(addr), { onEnter(a) {
    send(name + ' f340=' + ptr(0x674340).readU16() + ' f370=' + ptr(0x674370).readU16() + ' f372=' + ptr(0x674372).readU16() + ' b374=' + ptr(0x674374).readU8() + ' err=' + err.readS16() + ' al=' + this.context.eax.toString(16)); } });
}
Interceptor.attach(Process.getModuleByName('kernel32.dll').getExportByName('WriteFile'),{onEnter(a){this.n=a[2].toInt32();this.h=a[0];},onLeave(r){send('WriteFile h='+this.h+' n='+this.n+' ret='+r);}}); h(0x58F600,'OnComm'); h(0x58F795,'OnComm_skip'); h(0x623690,'match21b'); Interceptor.attach(ptr(0x623B44),{onEnter(a){var sp=this.context.esp;send('Output stack '+hexdump(sp,{length:48,header:false,ansi:false}));var vp=sp.add(16).readPointer(); send('inner '+hexdump(vp,{length:16,header:false,ansi:false})); var sa=vp.add(8).readPointer(); send('sa '+hexdump(sa,{length:40,header:false,ansi:false})); var pd=sa.add(12).readPointer(); send('data '+hexdump(pd,{length:129,header:false,ansi:false}));}}); h(0x623F30,'errpath_b'); h(0x6241A0,'overflow_b'); h(0x623C44,'exit_b'); h(0x62378C,'fill_done'); h(0x6239EE,'sum_done'); h(0x61E695,'read_loopexit'); Interceptor.attach(ptr(0x61E510),{onEnter(a){send('read_from '+this.returnAddress);},onLeave(r){send('read_ret al='+r.toString(16)+' b374='+ptr(0x674374).readU8()+' err='+err.readS16());}}); h(0x61DD40,'send_noecho'); h(0x61D9B0,'send_echo'); h(0x61E510,'read');
h(0x62505E,'after_read'); h(0x6250E0,'match21'); h(0x625694,'exitE'); h(0x6250CB,'err26');
h(0x625980,'errpath'); h(0x625594,'Output_set'); h(0x625033,'send3F');
"""
sc = s.create_script(js)
sc.on('message', lambda m, d: print(time.strftime('%H:%M:%S'), m.get('payload', m), flush=True))
sc.load()
time.sleep(dur)









