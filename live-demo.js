//@ts-check

import initWasm, {Matrix} from "./pkg/learn_fea.js";

const wasm = await initWasm();

const matrix = Matrix.identity(3);

alert(`${matrix.get(1, 1)}`);


wasm.greet();