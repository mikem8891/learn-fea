//@ts-check

import initWasm, {Lin2DStaticModel, init_fea} from "./pkg/learn_fea.js";

import * as doc from "./modules/doc.js";

const wasm = await initWasm();

/** @type {Lin2DStaticModel} */
let model;

/** @todo set 'change' events  */
function setup() {
  
  wasm.main();

  let elasticity = doc.getInputElementById("elasticity");
  let poissonsRatio = doc.getInputElementById("poissons-ratio");
  let nodeIndex = doc.getInputElementById("node-index");
  let positionX = doc.getInputElementById("position-x");
  let positionY = doc.getInputElementById("position-y");
  let forceX = doc.getInputElementById("force-x");
  let forceY = doc.getInputElementById("force-y");
  let displacementX = doc.getInputElementById("displacement-x");
  let displacementY = doc.getInputElementById("displacement-y");

  elasticity.addEventListener("change", changeMaterials);
  poissonsRatio.addEventListener("change", changeMaterials);
  
  function changeMaterials(evt) {
    let e = parseFloat(elasticity.value);
    let nu = parseFloat(poissonsRatio.value);
    let g = e / (2 * (1 + nu));
    model = init_fea(e, nu, g);
  }
}

if (document.readyState === 'loading') {
  document.addEventListener('DOMContentLoaded', setup);
} else {
  setup();
}

