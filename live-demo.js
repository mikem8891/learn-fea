//@ts-check
"use strict";

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
  let rigidity = doc.getInputElementById("rigidity");
  let addNodeBtn = doc.getElementById("add-node");
  let nodeIndex = doc.getInputElementById("node-index");
  let positionX = doc.getInputElementById("position-x");
  let positionY = doc.getInputElementById("position-y");
  let forceX = doc.getInputElementById("force-x");
  let forceY = doc.getInputElementById("force-y");
  let displacementX = doc.getInputElementById("displacement-x");
  let displacementY = doc.getInputElementById("displacement-y");

  elasticity.addEventListener("change", changeMaterials);
  poissonsRatio.addEventListener("change", changeMaterials);
  rigidity.addEventListener("change", changeMaterials);
  
  /**
   * @param {Event} [_evt]
   */
  function changeMaterials(_evt) {
    let e = parseFloat(elasticity.value);
    let nu = parseFloat(poissonsRatio.value);
    let g = parseFloat(rigidity.value);
    model = init_fea(e, nu, g);
  }

  addNodeBtn.addEventListener("click", (_evt) => {
    nodeIndex.value = model.nodes_len().toString();
    /** @todo add node to model */
    const ZERO = "0";
    positionX.value = ZERO;
    positionY.value = ZERO;
    displacementX.value = ZERO;
    displacementY.value = ZERO;
    forceX.value = ZERO;
    forceY.value = ZERO;

    positionX.focus();
  });



  changeMaterials();
}

if (document.readyState === 'loading') {
  document.addEventListener('DOMContentLoaded', setup);
} else {
  setup();
}