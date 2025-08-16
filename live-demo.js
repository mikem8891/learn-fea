//@ts-check

import initWasm, {} from "./pkg/learn_fea.js";

import * as doc from "./modules/doc.js";

const wasm = await initWasm();


let model = {
  elasticity: 30_000
};

function setup() {
  let elasticity = doc.getInputElementById("elasticity");
  let poissonsRatio = doc.getInputElementById("poissons-ratio");
  let nodeIndex = doc.getInputElementById("node-index");
  let positionX = doc.getInputElementById("position-x");
  let positionY = doc.getInputElementById("position-y");
  let forceX = doc.getInputElementById("force-x");
  let forceY = doc.getInputElementById("force-y");
  let displacementX = doc.getInputElementById("displacement-x");
  let displacementY = doc.getInputElementById("displacement-y");
}

if (document.readyState === 'loading') {
  document.addEventListener('DOMContentLoaded', setup);
} else {
  setup();
}

//wasm.greet();

//wasm.main();