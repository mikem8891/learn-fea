//@ts-check

import * as initWasm from "./pkg/learn_fea.js";

const wasm = await initWasm.default();

wasm.greet();
