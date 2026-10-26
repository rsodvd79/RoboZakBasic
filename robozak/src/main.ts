import { invoke } from "@tauri-apps/api/core";
import { marked } from "marked";

interface Motor { no: number; group: number; used: boolean; left: number; top: number; zero_left: number; zero_top: number }
interface Robot { image: string; zero_image: string; motors: Motor[] }
interface Sim { robot: { servos: number[]; output: string[]; log: string[]; elapsed_ms: number }; stop: string }

const $ = <T extends HTMLElement>(id: string) => document.getElementById(id) as T;
let keywords = new Set<string>();
let connected = false;
let sliders: HTMLInputElement[] = [];

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
  $("hl").scrollTop = code.scrollTop;
}

function log(msg: string, cls = "") {
  const d = document.createElement("div");
  d.className = cls;
  d.textContent = msg;
  $("console").prepend(d);
}

async function loadRobot() {
  const name = $<HTMLSelectElement>("robot").value;
  const r = await invoke<Robot>("robot", { name });
  const stage = $("stage");
  stage.innerHTML = `<img src="/robots/${r.image.replace(".bmp", ".png")}" alt="${name}" />`;
  sliders = [];
  for (const m of r.motors.filter((m) => m.used)) {
    const s = document.createElement("input");
    s.type = "range";
    s.min = "10";
    s.max = "190";
    s.value = "100";
    s.title = `Motore ${m.no}: 100`;
    s.className = "servo";
    s.style.left = `${m.left}px`;
    s.style.top = `${m.top}px`;
    s.addEventListener("input", () => {
      s.title = `Motore ${m.no}: ${s.value}`;
      onServo(m.no, Number(s.value));
    });
    sliders[m.no] = s;
    stage.appendChild(s);
  }
}

function onServo(motor: number, angle: number) {
  if (connected) log(`Motore ${motor} = ${angle} (solo vista: protocollo seriale live non ancora implementato)`, "w");
}

async function loadPorts() {
  const ports = await invoke<string[]>("serial_ports");
  $("port").innerHTML = ports.map((p) => `<option>${p}</option>`).join("");
}

async function toggleConnect() {
  const btn = $("connect");
  try {
    if (connected) {
      await invoke("serial_close");
      connected = false;
      btn.textContent = "Connetti";
    } else {
      await invoke("serial_open", { name: $<HTMLSelectElement>("port").value });
      connected = true;
      btn.textContent = "Disconnetti";
      log("Porta aperta a 9600,n,8,1");
    }
  } catch (e) {
    log(String(e), "e");
  }
}

async function download() {
  if (!connected) return log("Connetti prima la porta seriale", "e");
  const d = new Date();
  const stamp = [d.getDate(), d.getMonth() + 1, d.getFullYear() % 100, d.getHours(), d.getMinutes(), d.getSeconds()];
  try {
    log("Compilazione e download in corso...");
    const n = await invoke<number>("download_program", {
      src: $<HTMLTextAreaElement>("code").value,
      name: "ROBOZAK1",
      stamp,
    });
    log(`Download completato (${n} byte)`, "ok");
  } catch (e: any) {
    log(e?.line ? `Riga ${e.line}: ${e.msg}` : String(e), "e");
  }
}

async function check() {
  try {
    await invoke("check_program", { src: $<HTMLTextAreaElement>("code").value });
    log("Programma corretto", "ok");
    return true;
  } catch (e: any) {
    log(`Riga ${e.line}: ${e.msg}`, "e");
    return false;
  }
}

async function run() {
  try {
    const r = await invoke<Sim>("simulate", { src: $<HTMLTextAreaElement>("code").value, maxSteps: 200000 });
    r.robot.servos.forEach((v, i) => {
      if (sliders[i]) {
        sliders[i].value = String(v);
        sliders[i].title = `Motore ${i}: ${v}`;
      }
    });
    r.robot.output.forEach((l) => log(l));
    log(`Simulazione ${r.stop === "End" ? "terminata" : "interrotta (limite passi)"}, tempo simulato ${r.robot.elapsed_ms} ms`, "ok");
  } catch (e) {
    log(String(e), "e");
  }
}

interface HelpItem { file: string; title: string }
let helpItems: HelpItem[] = [];

async function showHelp(file: string) {
  const md = await (await fetch(`/help/${encodeURIComponent(file)}`)).text();
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
}

function listHelp(q: string) {
  const f = q.trim().toLowerCase();
  $("help-list").innerHTML = helpItems
    .filter((i) => !f || i.file.toLowerCase().includes(f) || i.title.toLowerCase().includes(f))
    .map((i) => `<li data-f="${esc(i.file)}">${esc(i.title)}</li>`)
    .join("");
}

async function initHelp() {
  helpItems = await (await fetch("/help/index.json")).json();
  listHelp("");
  $("help-q").addEventListener("input", (e) => listHelp((e.target as HTMLInputElement).value));
  $("help-list").addEventListener("click", (e) => {
    const f = (e.target as HTMLElement).dataset.f;
    if (f) void showHelp(f);
  });
  $("tab-help").addEventListener("click", () => {
    const h = $("help");
    h.hidden = !h.hidden;
    $("console").hidden = !h.hidden;
  });
}

window.addEventListener("DOMContentLoaded", async () => {
  keywords = new Set(await invoke<string[]>("keywords"));
  const code = $<HTMLTextAreaElement>("code");
  code.value =
    "DIM A AS BYTE\n\nPTP SETON\nPTP ALLON\nMOTOR G6A\nSPEED 5\n\nMAIN:\n    MOVE G6A,100,76,145,93,100,100\n    DELAY 500\n    MOVE G6A,100,100,100,100,100,100\n    PRINT \"fatto\"\n    END\n";
  code.addEventListener("input", render);
  code.addEventListener("scroll", () => ($("hl").scrollTop = code.scrollTop));
  $("robot").addEventListener("change", loadRobot);
  $("refresh").addEventListener("click", loadPorts);
  $("connect").addEventListener("click", toggleConnect);
  $("check").addEventListener("click", check);
  $("download").addEventListener("click", download);
  $("run").addEventListener("click", async () => {
    if (await check()) await run();
  });
  render();
  await Promise.all([loadRobot(), loadPorts(), initHelp()]);
});
