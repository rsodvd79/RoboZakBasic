import assert from "node:assert/strict";
import { readFileSync } from "node:fs";
import test from "node:test";
import vm from "node:vm";
import ts from "typescript";

const source = ts.transpileModule(
  readFileSync(new URL("./main.ts", import.meta.url), "utf8"),
  { compilerOptions: { module: ts.ModuleKind.CommonJS, target: ts.ScriptTarget.ES2022 } },
).outputText;

function ui(invoke, dialogs = {}, storage = new Map()) {
  const elements = new Map();
  const element = (tag = "div") => ({
    tagName: tag.toUpperCase(), disabled: false, value: "", textContent: "", children: [],
    dataset: {}, style: {}, attributes: {}, listeners: {},
    prepend(child) { this.children.unshift(child); },
    appendChild(child) {
      this.children.push(child);
      if (this.tagName === "SELECT" && this.children.length === 1) this.value = child.value;
    },
    replaceChildren() { this.children = []; if (this.tagName === "SELECT") this.value = ""; },
    setAttribute(name, value) { this.attributes[name] = value; },
    getAttribute(name) { return this.attributes[name]; },
    addEventListener(name, handler) { this.listeners[name] = handler; },
    focus() { this.focused = true; },
    showModal() { this.open = true; },
    close(value = "") { this.returnValue = value; this.open = false; this.listeners.close?.(); },
  });
  const document = {
    getElementById(id) {
      if (!elements.has(id)) elements.set(id, element(["port", "robot"].includes(id) ? "select" : "div"));
      return elements.get(id);
    },
    createElement: element,
  };
  document.getElementById("port").value = "COM1";
  document.getElementById("robot").value = "robonova";
  const context = vm.createContext({
    document, window: { addEventListener() {}, clearTimeout, setTimeout }, exports: {},
    localStorage: {
      getItem: (key) => storage.get(key) ?? null,
      setItem: (key, value) => storage.set(key, value),
    },
    require(name) {
      if (name === "@tauri-apps/api/core") return { invoke };
      if (name === "@tauri-apps/api/window") return { getCurrentWindow: () => ({
        onCloseRequested: async () => {},
        destroy: dialogs.destroy ?? (async () => {}),
      }) };
      if (name === "@tauri-apps/plugin-dialog") return {
        open: dialogs.open ?? (async () => null), save: dialogs.save ?? (async () => null),
      };
      if (name === "marked") return { marked: {} };
      throw new Error(`Unexpected import: ${name}`);
    },
  });
  vm.runInContext(source, context);
  return {
    context, get: (id) => document.getElementById(id), storage,
    messages: () => document.getElementById("console").children.map((e) => e.textContent),
  };
}

const tick = () => new Promise((resolve) => setImmediate(resolve));

const limits = () => ({ limits: Array.from({ length: 24 }, () => ({ min: 10, max: 190, declared: false })) });

const recentPaths = (get) => get("recent-sources-list").children.map((li) => li.children[0].title);

test("recent saves persist the latest 20 distinct Windows paths across UI sessions", async () => {
  let number = 0;
  const storage = new Map();
  const { context, get } = ui(async () => {}, {
    save: async () => `C:\\code\\${++number}.bas`,
  }, storage);
  get("code").value = "END";
  for (let n = 0; n < 21; n++) await context.saveSource(true);
  assert.equal(recentPaths(get).length, 20);
  assert.equal(recentPaths(get)[0], "C:\\code\\21.bas");
  assert.equal(recentPaths(get).at(-1), "C:\\code\\2.bas");
  context.rememberSource("c:/CODE/10.bas");
  assert.equal(recentPaths(get)[0], "c:\\CODE\\10.bas");
  assert.equal(recentPaths(get).filter((p) => p.toLowerCase() === "c:\\code\\10.bas").length, 1);
  const restored = ui(async () => {}, {}, storage);
  restored.context.loadRecentSources();
  assert.deepEqual(recentPaths(restored.get), recentPaths(get));
});

test("successful loads and save-as enter recents but cancelled dialogs and failed writes do not", async () => {
  const selections = ["C:\\caricato.bas", null];
  const saves = [null, "C:\\non-salvato.bas", "C:\\copia.bas"];
  let fail = true;
  const { context, get } = ui(async (name) => {
    if (name === "read_source") return "END";
    if (name === "program_settings") return limits();
    if (name === "write_source" && fail) throw "Errore scrittura";
  }, { open: async () => selections.shift(), save: async () => saves.shift() });
  await context.openSource();
  assert.deepEqual(recentPaths(get), ["C:\\caricato.bas"]);
  await context.openSource();
  await context.sourceAction(() => context.saveSource(true));
  await context.sourceAction(() => context.saveSource(true));
  assert.deepEqual(recentPaths(get), ["C:\\caricato.bas"]);
  fail = false;
  await context.sourceAction(() => context.saveSource(true));
  assert.deepEqual(recentPaths(get), ["C:\\copia.bas", "C:\\caricato.bas"]);
});

test("opening a recent source protects unsaved changes and promotes only the loaded file", async () => {
  const calls = [];
  const { context, get } = ui(async (name) => {
    calls.push(name);
    if (name === "read_source") return "PRINT \"recente\"\nEND";
    if (name === "program_settings") return limits();
  }, { open: async () => { throw "Recenti must not open the file picker"; } });
  context.rememberSource("C:\\primo.bas");
  context.rememberSource("C:\\secondo.bas");
  get("code").value = "RUN\nEND";
  let action = context.openRecentSource("C:\\primo.bas");
  get("unsaved-dialog").close("cancel");
  await action;
  assert.equal(get("code").value, "RUN\nEND");
  assert.deepEqual(recentPaths(get), ["C:\\secondo.bas", "C:\\primo.bas"]);
  assert.deepEqual(calls, []);
  action = context.openRecentSource("C:\\primo.bas");
  get("unsaved-dialog").close("discard");
  await action;
  assert.equal(get("code").value, "PRINT \"recente\"\nEND");
  assert.deepEqual(recentPaths(get), ["C:\\primo.bas", "C:\\secondo.bas"]);
  assert.deepEqual(calls, ["read_source", "program_settings"]);
});

test("missing recent files leave the current source and history intact", async () => {
  const { context, get, messages } = ui(async () => { throw "File mancante"; });
  context.rememberSource("C:\\mancante.bas");
  get("code").value = "END";
  const action = context.openRecentSource("C:\\mancante.bas");
  get("unsaved-dialog").close("discard");
  await action;
  assert.equal(get("code").value, "END");
  assert.equal(context.sourceModified(), true);
  assert.deepEqual(recentPaths(get), ["C:\\mancante.bas"]);
  assert.ok(messages().includes("File mancante"));
});

test("invalid recent storage is recoverable and recent entries show safe text and paths", () => {
  const storage = new Map([["robozak.recentSources", "{invalid"]]);
  const { context, get, messages } = ui(async () => {}, {}, storage);
  context.loadRecentSources();
  context.showRecentSources();
  assert.equal(get("recent-sources-dialog").open, true);
  assert.equal(get("recent-sources-empty").hidden, false);
  assert.ok(messages().some((msg) => msg.includes("Impossibile leggere la cronologia")));
  context.rememberSource("C:\\cartella\\<file>.bas");
  const button = get("recent-sources-list").children[0].children[0];
  assert.equal(button.children[0].textContent, "<file>.bas");
  assert.equal(button.children[1].textContent, "C:\\cartella\\<file>.bas");
  assert.equal(get("recent-sources-empty").hidden, true);
  storage.set("robozak.recentSources", JSON.stringify([null, 1, "", "  ", "C:\\ok.bas", "c:/OK.bas"]));
  context.loadRecentSources();
  assert.deepEqual(recentPaths(get), ["C:\\ok.bas"]);
});

test("storage write failure does not turn a successful source save into an error", async () => {
  const { context, get, messages } = ui(async () => {}, {
    save: async () => "C:\\salvato.bas",
  }, { get: () => null, set: () => { throw "Storage unavailable"; } });
  get("code").value = "END";
  assert.equal(await context.saveSource(), true);
  assert.equal(get("source-state").textContent, "Salvato");
  assert.deepEqual(recentPaths(get), ["C:\\salvato.bas"]);
  assert.ok(messages().some((msg) => msg.includes("non salvata su disco")));
});

test("saving an untitled source chooses a path once and tracks subsequent edits", async () => {
  const writes = [];
  let dialogs = 0;
  const { context, get } = ui(async (name, args) => {
    assert.equal(name, "write_source");
    writes.push(args);
  }, { save: async () => { dialogs++; return "C:\\programmi\\prova.bas"; } });
  get("code").value = "PRINT \"OK\"\nEND";
  context.render();
  assert.equal(get("source-state").textContent, "Modifiche non salvate");
  await context.sourceAction(() => context.saveSource());
  assert.equal(get("source-name").textContent, "prova.bas");
  assert.equal(get("source-name").title, "C:\\programmi\\prova.bas");
  assert.equal(get("source-state").textContent, "Salvato");
  get("code").value += "\n";
  context.render();
  assert.equal(get("source-name").textContent, "* prova.bas");
  await context.sourceAction(() => context.saveSource());
  assert.equal(dialogs, 1);
  assert.equal(writes.length, 2);
  assert.equal(writes[1].src, get("code").value);
});

test("save-as cancellation and write failure preserve the original document identity", async () => {
  let fail = false;
  const paths = ["C:\\originale.bas", null, "C:\\copia.bas"];
  const { context, get, messages } = ui(async () => {
    if (fail) throw "Disco non scrivibile";
  }, { save: async () => paths.shift() });
  get("code").value = "END";
  assert.equal(await context.saveSource(), true);
  get("code").value = "RUN\nEND";
  assert.equal(await context.saveSource(true), false);
  fail = true;
  await context.sourceAction(() => context.saveSource(true));
  context.render();
  assert.equal(get("source-name").textContent, "* originale.bas");
  assert.equal(get("source-name").title, "C:\\originale.bas");
  assert.equal(get("code").value, "RUN\nEND");
  assert.ok(messages().includes("Disco non scrivibile"));
  assert.equal(get("source-save").disabled, false);
});

test("new source cancellation protects unsaved code and discard resets editor and settings", async () => {
  const calls = [];
  const { context, get } = ui(async (name) => { calls.push(name); return limits(); });
  get("code").value = "'$LIMIT 0,50,90\nEND";
  const cancelled = context.newSource();
  assert.equal(get("unsaved-dialog").open, true);
  assert.equal(get("code").readOnly, true);
  await context.newSource();
  get("unsaved-dialog").close("cancel");
  await cancelled;
  assert.equal(get("code").value, "'$LIMIT 0,50,90\nEND");
  assert.equal(get("code").readOnly, false);
  assert.deepEqual(calls, []);
  const discarded = context.newSource();
  get("unsaved-dialog").close("discard");
  await discarded;
  assert.equal(get("code").value, "");
  assert.equal(get("source-name").textContent, "Senza titolo.bas");
  assert.equal(get("source-state").textContent, "Non salvato");
  assert.deepEqual(calls, ["program_settings"]);
});

test("choosing save before replacement cancels replacement if the save dialog is cancelled", async () => {
  const calls = [];
  const { context, get } = ui(async (name) => { calls.push(name); });
  get("code").value = "RUN\nEND";
  const action = context.newSource();
  get("unsaved-dialog").close("save");
  await action;
  assert.equal(get("code").value, "RUN\nEND");
  assert.deepEqual(calls, []);
});

test("load saves pending edits before reading the selected file and refreshes source limits", async () => {
  const calls = [];
  let saved = "";
  const { context, get } = ui(async (name, args) => {
    calls.push(name);
    if (name === "write_source") saved = args.src;
    if (name === "read_source") return saved;
    if (name === "program_settings") return limits();
  }, { open: async () => "C:\\stesso.bas", save: async () => "C:\\stesso.bas" });
  get("code").value = "PRINT \"nuovo\"\nEND";
  const action = context.openSource();
  await tick();
  get("unsaved-dialog").close("save");
  await action;
  assert.deepEqual(calls, ["write_source", "read_source", "program_settings"]);
  assert.equal(get("code").value, "PRINT \"nuovo\"\nEND");
  assert.equal(get("source-name").textContent, "stesso.bas");
  assert.equal(get("source-state").textContent, "Salvato");
});

test("cancelled file selection and load failure never erase the current source", async () => {
  const selections = [null, "C:\\mancante.bas"];
  const { context, get, messages } = ui(async () => { throw "File non disponibile"; },
    { open: async () => selections.shift() });
  get("code").value = "END";
  await context.openSource();
  assert.equal(get("unsaved-dialog").open, undefined);
  const action = context.openSource();
  await tick();
  get("unsaved-dialog").close("discard");
  await action;
  assert.equal(get("code").value, "END");
  assert.equal(context.sourceModified(), true);
  assert.ok(messages().includes("File non disponibile"));
});

test("closing a modified source can be cancelled or discarded without serial commands", async () => {
  let destroyed = 0;
  let prevented = 0;
  const { context, get } = ui(async () => { throw "Unexpected IPC"; },
    { destroy: async () => { destroyed++; } });
  const event = { preventDefault() { prevented++; } };
  get("code").value = "END";
  let closing = context.closeSourceWindow(event);
  get("unsaved-dialog").close("cancel");
  await closing;
  assert.equal(destroyed, 0);
  closing = context.closeSourceWindow(event);
  get("unsaved-dialog").close("discard");
  await closing;
  assert.equal(destroyed, 1);
  assert.equal(prevented, 2);
});

test("source shortcuts save and save-as while source actions stay disabled during download", async () => {
  let dialogs = 0;
  let writes = 0;
  const { context, get } = ui(async (name) => {
    if (name === "write_source") writes++;
  }, { save: async () => { dialogs++; return "C:\\test.bas"; } });
  get("code").value = "END";
  for (const shiftKey of [false, false, true]) {
    let prevented = false;
    context.sourceShortcut({ key: "s", ctrlKey: true, shiftKey, preventDefault() { prevented = true; } });
    await tick();
    assert.equal(prevented, true);
  }
  assert.equal(dialogs, 2);
  assert.equal(writes, 3);
  vm.runInContext("serialBusy = true; updateControls()", context);
  assert.equal(get("source-new").disabled, true);
  assert.equal(get("source-open").disabled, true);
  assert.equal(get("source-save").disabled, true);
  assert.equal(get("source-recent").disabled, true);
  context.showRecentSources();
  assert.equal(get("recent-sources-dialog").open, undefined);
  await context.newSource();
  context.sourceShortcut({ key: "s", ctrlKey: true, preventDefault() {} });
  await tick();
  assert.equal(writes, 3);
  assert.equal(get("code").value, "END");
});

test("servo updates are serialized and coalesced to the latest position", async () => {
  const calls = [];
  let release;
  const first = new Promise((resolve) => { release = resolve; });
  const { context } = ui(async (name, args) => {
    assert.equal(name, "servo_set");
    calls.push(args.pos);
    if (calls.length === 1) await first;
  });
  context.setConnected(true);
  context.onServo(0, 10);
  context.onServo(0, 20);
  context.onServo(0, 30);
  assert.deepEqual(calls, [10]);
  release();
  await tick();
  assert.deepEqual(calls, [10, 30]);
});

test("download drains the active servo, drops queued updates, and closes UI session", async () => {
  const calls = [];
  let release;
  const active = new Promise((resolve) => { release = resolve; });
  const { context, get, messages } = ui(async (name, args) => {
    calls.push(name);
    if (name === "servo_set") await active;
    if (name === "download_program") {
      assert.equal(args.name, "ROBOZAK1");
      return 24;
    }
  });
  context.setConnected(true);
  context.onServo(0, 10);
  context.onServo(0, 30);
  const downloading = context.download();
  await context.download();
  context.onServo(1, 40);
  assert.equal(get("connect").disabled, true);
  assert.deepEqual(calls, ["servo_set"]);
  release();
  await downloading;
  assert.deepEqual(calls, ["servo_set", "download_program"]);
  assert.equal(get("connect").textContent, "Connetti");
  assert.equal(get("connect").disabled, false);
  assert.equal(get("download").disabled, true);
  assert.ok(messages().some((m) => m.includes("Sessione chiusa")));
});

test("servo failures invalidate UI state and discard queued movements", async () => {
  const calls = [];
  const { context, get } = ui(async (name) => {
    calls.push(name);
    if (name === "servo_set") throw "Eco servo errato";
    if (name === "serial_is_open") return false;
  });
  context.setConnected(true);
  context.onServo(0, 100);
  context.onServo(1, 100);
  await tick();
  assert.deepEqual(calls, ["servo_set", "serial_is_open"]);
  assert.equal(get("download").disabled, true);
  assert.equal(get("connect").textContent, "Connetti");
});

test("compile diagnostics retain the live connection and display the original line", async () => {
  const { context, get, messages } = ui(async (name) => {
    if (name === "download_program") throw { line: 2, msg: "Sintassi non valida per MR-C3024" };
    if (name === "serial_is_open") return true;
  });
  context.setConnected(true);
  await context.download();
  assert.equal(get("download").disabled, false);
  assert.equal(get("connect").textContent, "Disconnetti");
  assert.ok(messages().includes("Riga 2: Sintassi non valida per MR-C3024"));
});

test("simulation shows unsupported effects instead of an unqualified success", async () => {
  const { context, get, messages } = ui(async (name) => {
    assert.equal(name, "simulate");
    return {
      robot: { servos: [], output: ["ok"], log: ["Non simulato: STOP"], elapsed_ms: 10 },
      stop: "Unsupported",
    };
  });
  await context.run();
  assert.ok(messages().includes("Non simulato: STOP"));
  assert.ok(messages().some((m) => m.includes("interrotta (operazione non simulata)")));
  assert.ok(get("console").children.filter((e) => e.textContent.startsWith("Simulazione")).every((e) => e.className === "w"));
});

test("offline servo changes synchronize diagram, list and numeric value without serial writes", async () => {
  const calls = [];
  const { context, get } = ui(async (name) => {
    calls.push(name);
    assert.equal(name, "robot");
    return { image: "robonova.bmp", motors: [{ no: 0, used: true, left: 10, top: 10 }] };
  });
  await context.loadRobot();
  context.onServo(0, 125);
  const diagram = get("stage").children[0];
  const card = get("motor-list").children[0];
  const control = card.children[1];
  const output = card.children[0].children[0];
  assert.equal(diagram.value, "125");
  assert.equal(control.value, "125");
  assert.equal(output.value, "125");
  assert.ok(get("robot-hint").textContent.includes("non muovono"));
  assert.deepEqual(calls, ["robot"]);
});

test("errors reveal the console from help and clearing restores its empty state", () => {
  const { context, get, messages } = ui(async () => {});
  context.setActivityTab(true);
  assert.equal(get("console").hidden, true);
  context.log("Riga 2: errore", "e");
  assert.equal(get("console").hidden, false);
  assert.equal(get("help").hidden, true);
  assert.equal(get("tab-console").getAttribute("aria-selected"), "true");
  assert.equal(get("console-empty").hidden, true);
  assert.deepEqual(messages(), ["Riga 2: errore"]);
  context.clearConsole();
  assert.deepEqual(messages(), []);
  assert.equal(get("console-empty").hidden, false);
});

test("no serial ports leaves a readable placeholder and disables connection", async () => {
  const calls = [];
  const { context, get } = ui(async (name) => { calls.push(name); return []; });
  await context.loadPorts();
  assert.equal(get("port").children[0].textContent, "Nessuna porta disponibile");
  assert.equal(get("connect").disabled, true);
  assert.equal(get("download").disabled, true);
  assert.deepEqual(calls, ["serial_ports"]);
});

test("editor line gutter, caret status and highlighting follow both scroll axes", () => {
  const { context, get } = ui(async () => {});
  const code = get("code");
  code.value = "ABC\nDEF";
  code.selectionStart = 5;
  code.scrollTop = 48;
  code.scrollLeft = 120;
  context.render();
  assert.equal(get("line-numbers").textContent, "1\n2");
  assert.equal(get("cursor-position").textContent, "Riga 2, colonna 2");
  assert.equal(get("hl").scrollTop, 48);
  assert.equal(get("hl").scrollLeft, 120);
  assert.equal(get("line-numbers").scrollTop, 48);
});

test("source limits update both controls and preview without issuing servo commands", async () => {
  const calls = [];
  const { context, get, messages } = ui(async (name) => {
    calls.push(name);
    if (name === "robot") return { image: "robonova.bmp", motors: [{ no: 0, used: true, left: 10, top: 10 }] };
    if (name === "program_settings") return { limits: [{ min: 50, max: 90, declared: true }] };
  });
  await context.loadRobot();
  get("code").value = "'$LIMIT 0,50,90";
  context.setConnected(true);
  await context.refreshSettings();
  const diagram = get("stage").children[0];
  const card = get("motor-list").children[0];
  assert.equal(diagram.min, "50");
  assert.equal(diagram.max, "90");
  assert.equal(card.children[1].value, "90");
  assert.equal(card.children[0].children[0].value, "90");
  assert.deepEqual(calls, ["robot", "program_settings"]);
  assert.ok(messages().some((m) => m.includes("nessun comando servo")));
});

test("invalid or pending directives suspend live movements", async () => {
  const calls = [];
  let reject;
  const pending = new Promise((_, r) => { reject = r; });
  const { context, get } = ui(async (name) => {
    calls.push(name);
    if (name === "program_settings") return pending;
  });
  context.setConnected(true);
  const refreshing = context.refreshSettings();
  context.onServo(0, 100);
  assert.equal(get("download").disabled, true);
  reject({ line: 1, msg: "$DEVICE: supportato solo MR-C3024" });
  await refreshing;
  context.onServo(0, 100);
  assert.deepEqual(calls, ["program_settings"]);
  assert.ok(get("robot-hint").textContent.includes("sospesi"));
});

test("live servo requests carry the current source and respect its bounds", async () => {
  const calls = [];
  const { context, get } = ui(async (name, args) => {
    if (name === "program_settings") return { limits: [{ min: 50, max: 100, declared: true }] };
    calls.push(args);
    assert.equal(name, "servo_set");
  });
  const src = "'$LIMIT 0,50,100\nEND";
  get("code").value = src;
  await context.refreshSettings();
  context.setConnected(true);
  context.onServo(0, 150);
  await tick();
  assert.equal(calls.length, 1);
  assert.equal(calls[0].pos, 100);
  assert.equal(calls[0].src, src);
});

test("a stale settings response cannot replace newer source limits", async () => {
  let release;
  const first = new Promise((resolve) => { release = resolve; });
  const { context, get } = ui(async (name, args) => {
    if (name === "robot") return { image: "robonova.bmp", motors: [{ no: 0, used: true, left: 10, top: 10 }] };
    assert.equal(name, "program_settings");
    if (args.src === "OLD") return first;
    return { limits: [{ min: 80, max: 100, declared: true }] };
  });
  await context.loadRobot();
  get("code").value = "OLD";
  const old = context.refreshSettings();
  get("code").value = "NEW";
  await context.refreshSettings();
  release({ limits: [{ min: 10, max: 190, declared: false }] });
  await old;
  const control = get("motor-list").children[0].children[1];
  assert.equal(control.min, "80");
  assert.equal(control.max, "100");
});

test("simulation previews stay synchronized even when initial positions exceed source limits", async () => {
  const { context, get, messages } = ui(async (name) => {
    if (name === "robot") return { image: "robonova.bmp", motors: [{ no: 0, used: true, left: 10, top: 10 }] };
    if (name === "program_settings") return { limits: [{ min: 50, max: 90, declared: true }] };
    if (name === "simulate") return { robot: { servos: [100], output: [], log: [], elapsed_ms: 0 }, stop: "End" };
  });
  await context.loadRobot();
  get("code").value = "'$LIMIT 0,50,90\nEND";
  await context.refreshSettings();
  await context.run();
  const card = get("motor-list").children[0];
  assert.equal(get("stage").children[0].value, "90");
  assert.equal(card.children[1].value, "90");
  assert.equal(card.children[0].children[0].value, "90");
  assert.ok(messages().some((m) => m.includes("stato logico del simulatore")));
  assert.equal(get("console").children.find((e) => e.textContent.startsWith("Simulazione terminata")).className, "w");
});

test("simulation of an older source cannot overwrite a newly limited preview", async () => {
  let release;
  const pending = new Promise((resolve) => { release = resolve; });
  const { context, get, messages } = ui(async (name) => {
    if (name === "robot") return { image: "robonova.bmp", motors: [{ no: 0, used: true, left: 10, top: 10 }] };
    if (name === "program_settings") return { limits: [{ min: 50, max: 90, declared: true }] };
    if (name === "simulate") return pending;
  });
  await context.loadRobot();
  get("code").value = "OLD";
  const running = context.run();
  get("code").value = "'$LIMIT 0,50,90\nEND";
  await context.refreshSettings();
  release({ robot: { servos: [150], output: ["OLD OUTPUT"], log: [], elapsed_ms: 0 }, stop: "End" });
  await running;
  assert.equal(get("motor-list").children[0].children[0].children[0].value, "90");
  assert.ok(!messages().includes("OLD OUTPUT"));
  assert.ok(messages().some((m) => m.includes("precedente ignorato")));
});

test("validation of an older source cannot mark edited code as valid", async () => {
  let release;
  const pending = new Promise((resolve) => { release = resolve; });
  const { context, get } = ui(async (name) => { assert.equal(name, "check_program"); return pending; });
  get("code").value = "OLD";
  const checking = context.check();
  get("code").value = "NEW";
  context.render();
  release();
  assert.equal(await checking, false);
  assert.equal(get("editor-state").textContent, "Da verificare");
});
