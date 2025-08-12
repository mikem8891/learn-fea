//@ts-check

import initWasm, {} from "./pkg/learn_fea.js";

const wasm = await initWasm();


wasm.greet();

wasm.main();