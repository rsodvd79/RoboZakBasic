import ghidra.app.script.GhidraScript;
import ghidra.app.decompiler.*;
import ghidra.program.model.listing.*;
import java.io.*;

public class DecompileAll extends GhidraScript {
    @Override
    public void run() throws Exception {
        // Args: outFile [startAddr endAddr]
        String[] a = getScriptArgs();
        String out = a.length > 0 ? a[0] : "/tmp/decomp.c";
        FunctionIterator it = a.length > 2
            ? currentProgram.getFunctionManager().getFunctions(new ghidra.program.model.address.AddressSet(toAddr(a[1]), toAddr(a[2])), true)
            : currentProgram.getFunctionManager().getFunctions(true);
        DecompInterface di = new DecompInterface();
        di.openProgram(currentProgram);
        try (PrintWriter w = new PrintWriter(new FileWriter(out))) {
            for (Function f : it) {
                DecompileResults r = di.decompileFunction(f, 30, monitor);
                w.println("// " + f.getName() + " @ " + f.getEntryPoint());
                if (r.decompileCompleted()) w.println(r.getDecompiledFunction().getC());
            }
        }
    }
}
