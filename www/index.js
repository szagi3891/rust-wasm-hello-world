//@ts-check

import * as wasm from "wasm-game-of-life";

window.alertConsole = (message) => {
    console.info(`alert console - ${message}`);
};

window.callFromWasm = (message) => {
    console.info(`Wiadomosc z wasm -> ${message}`);
}

console.info('aaa');

const aa = wasm.greet("dupa bladaaaaaa333", "66");
console.info(`AA = ${aa}`);

