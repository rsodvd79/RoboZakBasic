import { invoke } from "@tauri-apps/api/core";
import { getCurrentWindow } from "@tauri-apps/api/window";
import { open as openFileDialog, save as saveFileDialog } from "@tauri-apps/plugin-dialog";
import { marked } from "marked";

interface Motor { no: number; group: number; used: boolean; left: number; top: number; zero_left: number; zero_top: number }
interface Robot { image: string; zero_image: string; motors: Motor[] }
interface Sim { robot: { servos: number[]; output: string[]; log: string[]; elapsed_ms: number }; stop: string }
interface ServoLimit { min: number; max: number; declared: boolean }

const $ = <T extends HTMLElement>(id: string) => document.getElementById(id) as T;
let keywords = new Set<string>();
let connected = false;
let sliders: HTMLInputElement[] = [];
let motorSliders: HTMLInputElement[] = [];
let motorValues: HTMLOutputElement[] = [];
let showingSimulation = false;
let serialBusy = false;
let servoTask: Promise<void> | null = null;
const pendingServos = new Map<number, number>();
let servoLimits: ServoLimit[] = Array.from({ length: 24 }, () => ({ min: 10, max: 190, declared: false }));
let settingsReady = true;
let settingsTimer: number | undefined;
let sourcePath: string | null = null;
let savedSource = "";
let fileBusy = false;
const sourceFilters = [{ name: "Sorgente BASIC", extensions: ["bas"] }, { name: "File di testo", extensions: ["txt"] }];
const recentSourcesKey = "robozak.recentSources";
let recentSources: string[] = [];

function normalizeRecentSources(value: unknown): string[] {
  if (!Array.isArray(value)) return [];
  const paths: string[] = [];
  const seen = new Set<string>();
  for (const entry of value) {
    if (typeof entry !== "string" || !entry.trim()) continue;
    const path = entry.replace(/\//g, "\\");
    const key = path.toLowerCase();
    if (seen.has(key)) continue;
    seen.add(key);
    paths.push(path);
    if (paths.length === 20) break;
  }
  return paths;
}

function renderRecentSources() {
  const list = $("recent-sources-list");
  list.replaceChildren();
  $("recent-sources-empty").hidden = recentSources.length > 0;
  for (const path of recentSources) {
    const li = document.createElement("li");
    const button = document.createElement("button");
    button.type = "button";
    button.title = path;
    const name = document.createElement("strong");
    name.textContent = path.split("\\").pop() ?? path;
    const location = document.createElement("span");
    location.textContent = path;
    button.appendChild(name);
    button.appendChild(location);
    button.addEventListener("click", () => {
      $<HTMLDialogElement>("recent-sources-dialog").close();
      void openRecentSource(path);
    });
    li.appendChild(button);
    list.appendChild(li);
  }
}

function loadRecentSources() {
  try {
    recentSources = normalizeRecentSources(JSON.parse(localStorage.getItem(recentSourcesKey) ?? "[]"));
  } catch {
    recentSources = [];
    log("Impossibile leggere la cronologia dei sorgenti recenti.", "w");
  }
  renderRecentSources();
}

function rememberSource(path: string) {
  recentSources = normalizeRecentSources([path, ...recentSources]);
  renderRecentSources();
  try {
    localStorage.setItem(recentSourcesKey, JSON.stringify(recentSources));
  } catch {
    log("Cronologia disponibile per questa sessione, ma non salvata su disco.", "w");
  }
}

function showRecentSources() {
  if (fileBusy || serialBusy) return;
  renderRecentSources();
  $<HTMLDialogElement>("recent-sources-dialog").showModal();
}

function sourceModified() {
  return $<HTMLTextAreaElement>("code").value !== savedSource;
}

function updateSourceState() {
  const dirty = sourceModified();
  const name = sourcePath?.split(/[\\/]/).pop() ?? "Senza titolo.bas";
  $("source-name").textContent = `${dirty ? "* " : ""}${name}`;
  $("source-name").title = sourcePath ?? "Sorgente non ancora salvato";
  $("source-state").textContent = dirty ? "Modifiche non salvate" : sourcePath ? "Salvato" : "Non salvato";
  $("source-state").dataset.state = dirty ? "dirty" : "idle";
}

async function sourceAction(action: () => Promise<unknown>) {
  if (fileBusy || serialBusy) return;
  fileBusy = true;
  updateControls();
  try {
    await action();
  } catch (e) {
    log(errorMessage(e), "e");
  } finally {
    fileBusy = false;
    updateControls();
  }
}

async function saveSource(saveAs = false): Promise<boolean> {
  let path = sourcePath;
  if (!path || saveAs) {
    path = await saveFileDialog({
      title: "Salva sorgente BASIC",
      defaultPath: path ?? "Senza titolo.bas",
      filters: sourceFilters,
    });
    if (!path) return false;
  }
  const src = $<HTMLTextAreaElement>("code").value;
  await invoke("write_source", { path, src });
  sourcePath = path;
  savedSource = src;
  updateSourceState();
  rememberSource(path);
  log(`Sorgente salvato: ${path}`, "ok");
  return true;
}

async function confirmSourceReplacement(): Promise<boolean> {
  if (!sourceModified()) return true;
  const dialog = $<HTMLDialogElement>("unsaved-dialog");
  const choice = await new Promise<string>((resolve) => {
    dialog.returnValue = "cancel";
    dialog.addEventListener("close", () => resolve(dialog.returnValue), { once: true });
    dialog.showModal();
  });
  if (choice === "save") return saveSource();
  return choice === "discard";
}

async function replaceSource(src: string, path: string | null) {
  const code = $<HTMLTextAreaElement>("code");
  code.value = src;
  code.selectionStart = code.selectionEnd = 0;
  code.scrollTop = code.scrollLeft = 0;
  sourcePath = path;
  savedSource = code.value;
  showingSimulation = false;
  render();
  window.clearTimeout(settingsTimer);
  await refreshSettings();
}

async function newSource() {
  await sourceAction(async () => {
    if (!await confirmSourceReplacement()) return;
    await replaceSource("", null);
    log("Nuovo sorgente BASIC");
  });
}

async function openSource() {
  await sourceAction(async () => {
    const path = await openFileDialog({
      title: "Carica sorgente BASIC",
      defaultPath: sourcePath ?? undefined,
      filters: sourceFilters,
      multiple: false,
      directory: false,
    });
    if (!path || Array.isArray(path)) return;
    await loadSourcePath(path);
  });
}

async function loadSourcePath(path: string) {
  if (!await confirmSourceReplacement()) return;
  const src = await invoke<string>("read_source", { path });
  await replaceSource(src, path);
  rememberSource(path);
  log(`Sorgente caricato: ${path}`, "ok");
}

async function openRecentSource(path: string) {
  await sourceAction(() => loadSourcePath(path));
}

async function closeSourceWindow(event: { preventDefault(): void }) {
  if (!sourceModified() && !fileBusy && !serialBusy) return;
  event.preventDefault();
  if (fileBusy) return;
  if (serialBusy) {
    log("Attendi la fine dell'operazione seriale prima di chiudere.", "w");
    return;
  }
  await sourceAction(async () => {
    if (await confirmSourceReplacement()) await getCurrentWindow().destroy();
  });
}

function sourceShortcut(event: KeyboardEvent) {
  if (!(event.ctrlKey || event.metaKey) || event.altKey) return;
  const key = event.key.toLowerCase();
  if (key === "s") {
    event.preventDefault();
    void sourceAction(() => saveSource(event.shiftKey));
  } else if (!event.shiftKey && (key === "n" || key === "o")) {
    event.preventDefault();
    if (key === "n") void newSource();
    else void openSource();
  }
}

function updateControls() {
  for (const id of ["source-new", "source-open", "source-save", "source-save-as", "source-recent"]) {
    $<HTMLButtonElement>(id).disabled = fileBusy || serialBusy;
  }
  $<HTMLTextAreaElement>("code").readOnly = fileBusy;
  $<HTMLButtonElement>("check").disabled = fileBusy;
  $<HTMLButtonElement>("connect").disabled = serialBusy || (!connected && !$<HTMLSelectElement>("port").value);
  $("connect").textContent = connected ? "Disconnetti" : "Connetti";
  $<HTMLButtonElement>("download").disabled = serialBusy || fileBusy || !connected || !settingsReady;
  $<HTMLButtonElement>("run").disabled = serialBusy || fileBusy;
  $<HTMLSelectElement>("robot").disabled = serialBusy;
  $<HTMLSelectElement>("port").disabled = serialBusy || connected;
  $<HTMLButtonElement>("refresh").disabled = serialBusy || connected;
  sliders.forEach((s) => { s.disabled = serialBusy || fileBusy || !settingsReady; });
  motorSliders.forEach((s) => { s.disabled = serialBusy || fileBusy || !settingsReady; });
  const state = $("connection-state");
  state.textContent = serialBusy ? "Operazione in corso" : connected ? "Robot connesso" : "Non connesso";
  state.dataset.state = serialBusy ? "busy" : connected ? "live" : "offline";
  $("link-status").textContent = serialBusy ? "Operazione seriale in corso" :
    connected ? `Collegato a ${$<HTMLSelectElement>("port").value} / 9600 baud` : "Robot non connesso";
  $("robot-mode").textContent = showingSimulation ? "Simulato" : connected ? "Controllo live" : "Locale";
  $("robot-mode").dataset.state = connected && !showingSimulation ? "live" : "idle";
  $("robot-hint").textContent = !settingsReady ? "Direttive da correggere o verificare: controlli servo sospesi." :
    serialBusy ? "Controlli sospesi durante l'operazione seriale." :
    connected ? `${showingSimulation ? "Posizioni simulate. " : ""}Modificare uno slider muove il servo reale.` :
    "Modalit\u00e0 locale: gli slider non muovono il robot.";
}

function setConnected(value: boolean) {
  connected = value;
  if (!value) pendingServos.clear();
  updateControls();
}

async function syncConnection() {
  try {
    setConnected(await invoke<boolean>("serial_is_open"));
  } catch {
    setConnected(false);
  }
}

function errorMessage(e: unknown): string {
  if (typeof e === "object" && e !== null && "msg" in e) {
    const d = e as { line?: number; msg: string };
    return d.line ? `Riga ${d.line}: ${d.msg}` : d.msg;
  }
  return String(e);
}

const esc = (s: string) => s.replace(/&/g, "&amp;").replace(/</g, "&lt;");

function highlight(src: string): string {
  return src
    .split("\n")
    .map((line) => {
      const i = line.indexOf("'");
      const code = i < 0 ? line : line.slice(0, i);
      const comment = i < 0 ? "" : `<span class="c">${esc(line.slice(i))}</span>`;
      return (
        esc(code).replace(/[A-Za-z_][A-Za-z0-9_]*/g, (w) =>
          keywords.has(w.toUpperCase()) ? `<span class="k">${w}</span>` : w,
        ) + comment
      );
    })
    .join("\n");
}

function render() {
  const code = $<HTMLTextAreaElement>("code");
  $("hl").innerHTML = highlight(code.value) + "\n";
  const lines = code.value.split("\n").length;
  $("line-numbers").textContent = Array.from({ length: lines }, (_, i) => i + 1).join("\n");
  $("editor-lines").textContent = `${lines} ${lines === 1 ? "riga" : "righe"}`;
  $("editor-state").textContent = "Da verificare";
  $("editor-state").dataset.state = "idle";
  updateSourceState();
  syncEditorScroll();
  updateCursor();
}

function syncEditorScroll() {
  const code = $<HTMLTextAreaElement>("code");
  $("hl").scrollTop = code.scrollTop;
  $("hl").scrollLeft = code.scrollLeft;
  $("line-numbers").scrollTop = code.scrollTop;
}

function updateCursor() {
  const code = $<HTMLTextAreaElement>("code");
  const before = code.value.slice(0, code.selectionStart);
  const lines = before.split("\n");
  $("cursor-position").textContent = `Riga ${lines.length}, colonna ${lines[lines.length - 1].length + 1}`;
}

function setActivityTab(help: boolean) {
  $("help").hidden = !help;
  $("console").hidden = help;
  $("console-toolbar").hidden = help;
  $("console-empty").hidden = help || $("console").children.length > 0;
  for (const [id, selected] of [["tab-console", !help], ["tab-help", help]] as const) {
    $(id).setAttribute("aria-selected", String(selected));
    $(id).tabIndex = selected ? 0 : -1;
  }
}

function setRobotTab(list: boolean) {
  $("diagram-view").hidden = list;
  $("motor-list").hidden = !list;
  for (const [id, selected] of [["view-diagram", !list], ["view-motors", list]] as const) {
    $(id).setAttribute("aria-selected", String(selected));
    $(id).tabIndex = selected ? 0 : -1;
  }
  if (!list) fitRobot();
}

function wireTabs(first: string, second: string, select: (second: boolean) => void) {
  [first, second].forEach((id, index) => {
    $(id).addEventListener("click", () => select(index === 1));
    $(id).addEventListener("keydown", (e) => {
      if (!["ArrowLeft", "ArrowRight", "Home", "End"].includes(e.key)) return;
      e.preventDefault();
      const next = e.key === "Home" ? false : e.key === "End" ? true : index === 0;
      select(next);
      $(next ? second : first).focus();
    });
  });
}

function log(msg: string, cls = "") {
  const d = document.createElement("div");
  d.className = cls;
  d.textContent = msg;
  $("console").prepend(d);
  $("console-empty").hidden = true;
  $("task-status").textContent = msg;
  if (cls === "e") setActivityTab(false);
}

function clearConsole() {
  $("console").replaceChildren();
  setActivityTab(false);
  $("task-status").textContent = "Console pulita";
}

function fitRobot() {
  const view = $("diagram-view");
  if (!view.clientWidth || !view.clientHeight) return;
  const scale = Math.min(1, view.clientWidth / 338, view.clientHeight / 436);
  $("stage-frame").style.width = `${338 * scale}px`;
  $("stage-frame").style.height = `${436 * scale}px`;
  $("stage").style.transform = `scale(${scale})`;
}

function setServoView(motor: number, value: number) {
  const requested = value;
  const limit = servoLimits[motor];
  if (limit) value = Math.max(limit.min, Math.min(limit.max, value));
  for (const s of [sliders[motor], motorSliders[motor]]) {
    if (!s) continue;
    s.value = String(value);
    s.title = `Motore ${motor}: ${value}`;
    s.setAttribute("aria-valuetext", `Posizione ${value}`);
  }
  if (motorValues[motor]) motorValues[motor].value = String(value);
  $("servo-info").textContent = `Motore ${motor} / posizione ${value}`;
  return value !== requested;
}

async function refreshSettings() {
  const src = $<HTMLTextAreaElement>("code").value;
  settingsReady = false;
  pendingServos.clear();
  updateControls();
  try {
    const settings = await invoke<{ limits: ServoLimit[] }>("program_settings", { src });
    if (src !== $<HTMLTextAreaElement>("code").value) return;
    servoLimits = settings.limits;
    let clipped = false;
    motorSliders.forEach((s, motor) => {
      const limit = servoLimits[motor];
      if (!limit) return;
      const before = Number(s.value);
      const value = Math.max(limit.min, Math.min(limit.max, before));
      for (const control of [s, sliders[motor]]) {
        if (!control) continue;
        control.min = String(limit.min);
        control.max = String(limit.max);
      }
      setServoView(motor, value);
      clipped ||= value !== before;
    });
    if (clipped) {
      showingSimulation = true;
      log("Anteprima adattata ai limiti del sorgente: nessun comando servo inviato.", "w");
    }
    settingsReady = true;
  } catch (e) {
    if (src === $<HTMLTextAreaElement>("code").value) log(errorMessage(e), "e");
  } finally {
    updateControls();
  }
}

function scheduleSettings() {
  settingsReady = false;
  pendingServos.clear();
  updateControls();
  window.clearTimeout(settingsTimer);
  settingsTimer = window.setTimeout(() => { void refreshSettings(); }, 250);
}

async function loadRobot() {
  const name = $<HTMLSelectElement>("robot").value;
  const r = await invoke<Robot>("robot", { name });
  const stage = $("stage");
  stage.innerHTML = `<img src="/robots/${r.image.replace(".bmp", ".png")}" alt="${name}" />`;
  sliders = [];
  motorSliders = [];
  motorValues = [];
  $("motor-list").replaceChildren();
  for (const m of r.motors.filter((m) => m.used)) {
    const limit = servoLimits[m.no] ?? { min: 10, max: 190 };
    const initial = Math.max(limit.min, Math.min(limit.max, 100));
    const s = document.createElement("input");
    s.type = "range";
    s.min = String(limit.min);
    s.max = String(limit.max);
    s.value = String(initial);
    s.title = `Motore ${m.no}: ${initial}`;
    s.className = "servo";
    s.setAttribute("aria-label", `Posizione motore ${m.no}`);
    s.style.left = `${m.left}px`;
    s.style.top = `${m.top}px`;
    s.addEventListener("input", () => {
      onServo(m.no, Number(s.value));
    });
    s.addEventListener("focus", () => { $("servo-info").textContent = s.title; });
    sliders[m.no] = s;
    stage.appendChild(s);
    const card = document.createElement("div");
    card.className = "motor-card";
    const label = document.createElement("label");
    label.htmlFor = `motor-${m.no}`;
    label.textContent = `Motore ${String(m.no).padStart(2, "0")}`;
    const value = document.createElement("output");
    value.value = String(initial);
    value.setAttribute("aria-label", `Posizione motore ${m.no}`);
    label.appendChild(value);
    const control = document.createElement("input");
    control.id = `motor-${m.no}`;
    control.type = "range";
    control.min = s.min;
    control.max = s.max;
    control.value = s.value;
    control.title = s.title;
    control.addEventListener("input", () => onServo(m.no, Number(control.value)));
    control.addEventListener("focus", () => { $("servo-info").textContent = control.title; });
    card.appendChild(label);
    card.appendChild(control);
    $("motor-list").appendChild(card);
    motorSliders[m.no] = control;
    motorValues[m.no] = value;
  }
  $("motor-count").textContent = `${motorSliders.filter(Boolean).length} servo`;
  showingSimulation = false;
  $("servo-info").textContent = "Seleziona un servo per vedere la posizione";
  fitRobot();
  updateControls();
}

function onServo(motor: number, angle: number) {
  if (serialBusy || !settingsReady) return;
  const limit = servoLimits[motor];
  if (limit) angle = Math.max(limit.min, Math.min(limit.max, angle));
  showingSimulation = false;
  setServoView(motor, angle);
  updateControls();
  if (!connected) return;
  pendingServos.set(motor, angle);
  if (!servoTask) servoTask = flushServos().finally(() => { servoTask = null; });
}

async function flushServos() {
  while (connected && !serialBusy && settingsReady && pendingServos.size) {
    const [motor, pos] = pendingServos.entries().next().value!;
    pendingServos.delete(motor);
    try {
      await invoke("servo_set", { motor, pos, src: $<HTMLTextAreaElement>("code").value });
    } catch (e) {
      pendingServos.clear();
      log(errorMessage(e), "e");
      await syncConnection();
      break;
    }
  }
}

async function loadPorts() {
  try {
    const ports = await invoke<string[]>("serial_ports");
    const previous = $<HTMLSelectElement>("port").value;
    const select = $<HTMLSelectElement>("port");
    select.replaceChildren();
    for (const p of ports) {
      const option = document.createElement("option");
      option.value = p;
      option.textContent = p;
      select.appendChild(option);
    }
    if (!ports.length) {
      const option = document.createElement("option");
      option.value = "";
      option.textContent = "Nessuna porta disponibile";
      select.appendChild(option);
    } else if (ports.includes(previous)) select.value = previous;
    updateControls();
  } catch (e) {
    log(`Ricerca porte: ${errorMessage(e)}`, "e");
  }
}

async function toggleConnect() {
  if (serialBusy) return;
  serialBusy = true;
  pendingServos.clear();
  updateControls();
  try {
    await servoTask;
    if (connected) {
      await invoke("serial_close");
      setConnected(false);
    } else {
      await invoke("serial_open", { name: $<HTMLSelectElement>("port").value });
      setConnected(true);
      log("MR-C3024 riconosciuto, porta aperta a 9600,n,8,1");
    }
  } catch (e) {
    log(errorMessage(e), "e");
    await syncConnection();
  } finally {
    serialBusy = false;
    updateControls();
  }
}

async function download() {
  if (serialBusy) return;
  if (!connected) return log("Connetti prima la porta seriale", "e");
  serialBusy = true;
  pendingServos.clear();
  updateControls();
  const d = new Date();
  const stamp = [d.getDate(), d.getMonth() + 1, d.getFullYear() % 100, d.getHours(), d.getMinutes(), d.getSeconds()];
  try {
    await servoTask;
    if (!connected) return log("Connessione persa: riconnetti prima del download", "e");
    log("Compilazione e download in corso...");
    const n = await invoke<number>("download_program", {
      src: $<HTMLTextAreaElement>("code").value,
      name: "ROBOZAK1",
      stamp,
    });
    setConnected(false);
    log(`Programma inviato (${n} byte). Sessione chiusa: riconnetti prima del controllo live.`, "ok");
    log("Il protocollo non fornisce una conferma finale; il baud successivo del robot non e verificato.", "w");
  } catch (e) {
    log(errorMessage(e), "e");
    await syncConnection();
  } finally {
    serialBusy = false;
    updateControls();
  }
}

async function check() {
  const src = $<HTMLTextAreaElement>("code").value;
  setActivityTab(false);
  $("editor-state").textContent = "Verifica...";
  try {
    await invoke("check_program", { src });
    if (src !== $<HTMLTextAreaElement>("code").value) {
      log("Sorgente modificato: ripetere la verifica.", "w");
      return false;
    }
    log("Programma compilabile per MR-C3024", "ok");
    $("editor-state").textContent = "Validato";
    $("editor-state").dataset.state = "ok";
    return true;
  } catch (e) {
    if (src !== $<HTMLTextAreaElement>("code").value) return false;
    log(errorMessage(e), "e");
    $("editor-state").textContent = "Da correggere";
    $("editor-state").dataset.state = "error";
    return false;
  }
}

async function run() {
  const src = $<HTMLTextAreaElement>("code").value;
  setActivityTab(false);
  try {
    const r = await invoke<Sim>("simulate", { src, maxSteps: 200000 });
    if (src !== $<HTMLTextAreaElement>("code").value) {
      log("Sorgente modificato: risultato della simulazione precedente ignorato.", "w");
      return;
    }
    let clipped = false;
    r.robot.servos.forEach((v, i) => {
      if (sliders[i]) clipped = setServoView(i, v) || clipped;
    });
    showingSimulation = true;
    updateControls();
    if (clipped) log("Anteprima delle posizioni simulate adattata ai limiti; lo stato logico del simulatore non e' stato modificato.", "w");
    r.robot.output.forEach((l) => log(l));
    r.robot.log.forEach((l) => log(l, "w"));
    const stop = r.stop === "End" ? "terminata" : r.stop === "Unsupported" ? "interrotta (operazione non simulata)" : "interrotta (limite passi)";
    log(`Simulazione ${stop}, tempo DELAY ${r.robot.elapsed_ms} ms`, r.stop === "End" && !r.robot.log.length && !clipped ? "ok" : "w");
    log("Simulazione logica: movimenti istantanei, nessuna misura di tempi fisici o sensori.", "w");
  } catch (e) {
    if (src !== $<HTMLTextAreaElement>("code").value) return;
    log(errorMessage(e), "e");
  }
}

interface HelpItem { file: string; title: string }
let helpItems: HelpItem[] = [];

async function showHelp(file: string) {
  const response = await fetch(`/help/${encodeURIComponent(file)}`);
  if (!response.ok) throw new Error("Impossibile caricare la pagina della guida");
  const md = await response.text();
  const body = $("help-body");
  body.innerHTML = await marked.parse(md.replace(/\]\(([^)]+?)\)/g, (_, u: string) => `](${u.replace(/ /g, "%20")})`));
  body.querySelectorAll("img").forEach((img) => {
    const src = img.getAttribute("src") ?? "";
    img.setAttribute("src", `/help/${src}`);
  });
  body.querySelectorAll("a").forEach((a) => {
    const href = decodeURIComponent(a.getAttribute("href") ?? "");
    if (href.endsWith(".md")) {
      a.addEventListener("click", (e) => {
        e.preventDefault();
        void showHelp(href);
      });
    }
  });
  body.scrollTop = 0;
  $("help-title").textContent = helpItems.find((i) => i.file === file)?.title ?? file.replace(/\.md$/, "");
  $("help-list").querySelectorAll<HTMLButtonElement>("button").forEach((b) => {
    b.setAttribute("aria-current", String(b.dataset.f === file));
  });
}

function listHelp(q: string) {
  const f = q.trim().toLowerCase();
  const filtered = helpItems.filter((i) => !f || i.file.toLowerCase().includes(f) || i.title.toLowerCase().includes(f));
  $("help-list").replaceChildren();
  for (const item of filtered) {
    const li = document.createElement("li");
    li.dataset.f = item.file;
    const button = document.createElement("button");
    button.type = "button";
    button.dataset.f = item.file;
    button.textContent = item.title;
    li.appendChild(button);
    $("help-list").appendChild(li);
  }
  $("help-count").textContent = `${filtered.length} comandi`;
  $("help-no-results").hidden = filtered.length > 0;
}

async function initHelp() {
  helpItems = await (await fetch("/help/index.json")).json();
  listHelp("");
  $("help-q").addEventListener("input", (e) => listHelp((e.target as HTMLInputElement).value));
  $("help-list").addEventListener("click", (e) => {
    const f = (e.target as HTMLElement).closest<HTMLElement>("[data-f]")?.dataset.f;
    if (f) void showHelp(f).catch((err) => log(errorMessage(err), "e"));
  });
}

window.addEventListener("DOMContentLoaded", async () => {
  keywords = new Set(await invoke<string[]>("keywords"));
  const code = $<HTMLTextAreaElement>("code");
  code.value =
    "DIM A AS BYTE\n\nPTP SETON\nPTP ALLON\nMOTOR G6A\nSPEED 5\n\nMAIN:\n    MOVE G6A,100,76,145,93,100,100\n    DELAY 500\n    MOVE G6A,100,100,100,100,100,100\n    PRINT \"fatto\"\n    END\n";
  code.addEventListener("input", () => { render(); scheduleSettings(); });
  $("source-new").addEventListener("click", newSource);
  $("source-open").addEventListener("click", openSource);
  $("source-save").addEventListener("click", () => { void sourceAction(() => saveSource()); });
  $("source-save-as").addEventListener("click", () => { void sourceAction(() => saveSource(true)); });
  $("source-recent").addEventListener("click", showRecentSources);
  loadRecentSources();
  window.addEventListener("keydown", sourceShortcut);
  code.addEventListener("scroll", syncEditorScroll);
  for (const event of ["click", "keyup", "select"]) code.addEventListener(event, updateCursor);
  $("robot").addEventListener("change", () => { void loadRobot().catch((e) => log(errorMessage(e), "e")); });
  $("refresh").addEventListener("click", loadPorts);
  $("port").addEventListener("change", updateControls);
  $("connect").addEventListener("click", toggleConnect);
  $("check").addEventListener("click", check);
  $("download").addEventListener("click", download);
  $("run").addEventListener("click", async () => {
    if (await check()) await run();
  });
  wireTabs("tab-console", "tab-help", setActivityTab);
  wireTabs("view-diagram", "view-motors", setRobotTab);
  $("clear-console").addEventListener("click", clearConsole);
  new ResizeObserver(fitRobot).observe($("diagram-view"));
  render();
  updateControls();
  const results = await Promise.allSettled([
    loadRobot(), loadPorts(), initHelp(), refreshSettings(),
    getCurrentWindow().onCloseRequested(closeSourceWindow),
  ]);
  for (const result of results) if (result.status === "rejected") log(errorMessage(result.reason), "e");
});
