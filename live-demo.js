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
  let knownXDisplacement = doc.getInputElementById("known-displacement-x");
  let knownYDisplacement = doc.getInputElementById("known-displacement-y");
  let knownXForce = doc.getInputElementById("known-force-x");
  let knownYForce = doc.getInputElementById("known-force-y");
  let displacementX = doc.getInputElementById("displacement-x");
  let displacementY = doc.getInputElementById("displacement-y");
  let forceX = doc.getInputElementById("force-x");
  let forceY = doc.getInputElementById("force-y");

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

  addNodeBtn.addEventListener("click", (evt) => {
    let lastNode = model.nodes_len();
    nodeIndex.value = lastNode.toString();

    model.add_node();
    
    changeNodeIndex(evt);

    positionX.focus();
  });

  function changeNodeIndex(evt) {
    if (model.nodes_len() <= 0) {
      nodeIndex.value = "";
      return;
    }
    let index = parseInt(nodeIndex.value);
    if (index < 0 || isNaN(index)) {
      nodeIndex.value = "0";
      changeNodeIndex(evt);
      return;
    }
    if (index >= model.nodes_len()) {
      nodeIndex.value = (model.nodes_len() - 1).toString();
      changeNodeIndex(evt);
      return;
    }
    let node = model.get_node(index);

    positionX.value = node.get_pos_x().toString();
    positionY.value = node.get_pos_y().toString();
    displacementX.value = node.get_disp_x().toString();
    displacementY.value = node.get_disp_y().toString();
    forceX.value = node.get_force_x().toString();
    forceY.value = node.get_force_y().toString();
  }

  nodeIndex.addEventListener("change", changeNodeIndex);

  function changeNodeProp(_evt) {
    let index = parseInt(nodeIndex.value);
    let node = model.get_node(index);

    node.set_pos_x(parseFloat(positionX.value));
    node.set_pos_y(parseFloat(positionY.value));
    node.set_disp_x(parseFloat(displacementX.value));
    node.set_disp_y(parseFloat(displacementY.value));
    node.set_force_x(parseFloat(forceX.value));
    node.set_force_y(parseFloat(forceY.value));

    model.set_node(index, node);
  }

  positionX.addEventListener("change", changeNodeProp);
  positionY.addEventListener("change", changeNodeProp);
  displacementX.addEventListener("change", changeNodeProp);
  displacementY.addEventListener("change", changeNodeProp);
  forceX.addEventListener("change", changeNodeProp);
  forceY.addEventListener("change", changeNodeProp);

  changeMaterials();
}

if (document.readyState === 'loading') {
  document.addEventListener('DOMContentLoaded', setup);
} else {
  setup();
}