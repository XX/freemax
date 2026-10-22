import init, * as wasm from "./dist/client.js";

await init();

// Ответы с ошибкой (404, 403, 500) тоже подставляются: сервер отдаёт для них страницу в общем layout.
htmx.config.responseHandling = [
    { code: "204", swap: false },
    { code: "[2345]..", swap: true, error: false },
    { code: "...", swap: false },
];

wasm.init();
wasm.reinit();
document.body.style.visibility = "visible";

document.body.addEventListener("htmx:afterSettle", () => wasm.reinit());
