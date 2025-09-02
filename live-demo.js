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
  let known = {
    displacement: {
      x: doc.getInputElementById("known-displacement-x"),
      y: doc.getInputElementById("known-displacement-y")
    },
    force: {
      x: doc.getInputElementById("known-force-x"),
      y: doc.getInputElementById("known-force-y")
    },
  };
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
    const e = parseFloat(elasticity.value);
    const nu = parseFloat(poissonsRatio.value);
    const g = parseFloat(rigidity.value);
    model = init_fea(e, nu, g);
  }

  addNodeBtn.addEventListener("click", (evt) => {
    const lastNode = model.nodes_len();
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
    const index = parseInt(nodeIndex.value);
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
    const node = model.get_node(index);
    
    setNodeInputsTo(node);
  }
  
  function setNodeInputsTo(node) {
    const known = {
      x: node.get_known_x(),
      y: node.get_known_y()
    };
    
    known.force.x.checked = known.x == KnownType.FORCE;
    known.force.y.checked = known.y == KnownType.FORCE;
    known.displacement.x.checked = known.x == KnownType.DISPLACEMENT;
    known.displacement.y.checked = known.x == KnownType.DISPLACEMENT;

    positionX.value = node.get_pos_x().toString();
    positionY.value = node.get_pos_y().toString();
    displacementX.value = node.get_disp_x().toString();
    displacementY.value = node.get_disp_y().toString();
    forceX.value = node.get_force_x().toString();
    forceY.value = node.get_force_y().toString();
  }

  nodeIndex.addEventListener("change", changeNodeIndex);

  function changeNodeProp(evt) {
    let index = parseInt(nodeIndex.value);
    let node = model.get_node(index);
    
    if (evt.target == known.force.x) {
      node.set_known_x(KnownType.FORCE);
    } else if (evt.target == known.force.y) {
      node.set_known_y(KnownType.FORCE);
    } else if (evt.target == known.displacement.x) {
      node.set_known_x(KnownType.DISPLACEMENT);
    } else if (evt.target == known.displacement.y) {
      node.set_known_y(KnownType.DISPLACEMENT);
    } 

    node.set_pos_x(parseFloat(positionX.value));
    node.set_pos_y(parseFloat(positionY.value));
    node.set_disp_x(parseFloat(displacementX.value));
    node.set_disp_y(parseFloat(displacementY.value));
    node.set_force_x(parseFloat(forceX.value));
    node.set_force_y(parseFloat(forceY.value));

    model.set_node(index, node);
    
    setNodeInputsTo(node);
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