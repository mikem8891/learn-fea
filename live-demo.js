//@ts-check
"use strict";

import initWasm, {Lin2DStaticModel, Node2D, KnownType, init_fea} from "./pkg/learn_fea.js";

import * as doc from "./modules/doc.js";

const wasm = await initWasm();

/** @type {Lin2DStaticModel} */
let model;

/** @todo set 'change' events  */
function setup() {
  
  wasm.main();

  const elasticity = doc.getInputElementById("elasticity");
  const poissonsRatio = doc.getInputElementById("poissons-ratio");
  const rigidity = doc.getInputElementById("rigidity");
  const addNodeBtn = doc.getElementById("add-node");
  const nodeIndex = doc.getInputElementById("node-index");
  const positionX = doc.getInputElementById("position-x");
  const positionY = doc.getInputElementById("position-y");
  const known = {
    displacement: {
      x: doc.getInputElementById("known-displacement-x"),
      y: doc.getInputElementById("known-displacement-y")
    },
    force: {
      x: doc.getInputElementById("known-force-x"),
      y: doc.getInputElementById("known-force-y")
    },
  };
  const displacementX = doc.getInputElementById("displacement-x");
  const displacementY = doc.getInputElementById("displacement-y");
  const forceX = doc.getInputElementById("force-x");
  const forceY = doc.getInputElementById("force-y");
  const addElementBtn = doc.getElementById("add-element");
  const elementIndex = doc.getInputElementById("element-index");
  const elementNodes = [
    doc.getInputElementById("element-node-0"),
    doc.getInputElementById("element-node-1"), 
    doc.getInputElementById("element-node-2")
  ];
  const stepBtn = doc.getElementById("step-button");

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
    positionX.select();
  });

  function changeNodeIndex(evt) {
    let index = parseInt(nodeIndex.value);
    if (model.nodes_len() <= 0 || isNaN(index)) {
      nodeIndex.value = "";
      clearNodeInputs();
      return;
    }
    if (index < 0) {
      index = 0;
      nodeIndex.select();
    } else if (index >= model.nodes_len()) {
      index = model.nodes_len() - 1;
      nodeIndex.select();
    }
    nodeIndex.value = index.toString();
    const node = model.get_node(index);
    setNodeInputsTo(node);
    node.free();
  }

  /**
   * 
   * @param {Node2D} node 
   */
  function setNodeInputsTo(node) {
    const nodeKnown = {
      x: node.knownX,
      y: node.knownY
    };
    
    known.force.x.checked = nodeKnown.x == KnownType.Force;
    known.displacement.x.checked = nodeKnown.x == KnownType.Displacement;
    known.force.y.checked = nodeKnown.y == KnownType.Force;
    known.displacement.y.checked = nodeKnown.y == KnownType.Displacement;

    positionX.value = node.posX.toString();
    positionY.value = node.posY.toString();
    displacementX.value = node.dispX.toString();
    displacementY.value = node.dispY.toString();
    forceX.value = node.forceX.toString();
    forceY.value = node.forceY.toString();
  }

  function clearNodeInputs() {
    known.force.x.checked = false;
    known.displacement.x.checked = false;
    known.force.y.checked = false;
    known.displacement.y.checked = false;

    positionX.value = "";
    positionY.value = "";
    displacementX.value = "";
    displacementY.value = "";
    forceX.value = "";
    forceY.value = "";
  }

  nodeIndex.addEventListener("change", changeNodeIndex);

  function changeNodeProp(evt) {
    const index = parseInt(nodeIndex.value);
    if (model.nodes_len() <= 0 || isNaN(index)) {
      nodeIndex.value = "";
      clearNodeInputs();
      return;
    }
    const node = model.get_node(index);
    
    if (known.force.x.checked) {
      node.knownX = KnownType.Force;
    } else {
      node.knownX = KnownType.Displacement;
    }
    if (known.force.y.checked) {
      node.knownY = KnownType.Force;
    } else {
      node.knownY = KnownType.Displacement;
    } 

    node.posX = parseFloat(positionX.value);
    node.posY = parseFloat(positionY.value);
    node.dispX = parseFloat(displacementX.value);
    node.dispY = parseFloat(displacementY.value);
    node.forceX = parseFloat(forceX.value);
    node.forceY = parseFloat(forceY.value);

    model.set_node(index, node);
    
    setNodeInputsTo(node);
  }

  known.force.x.addEventListener("click", changeNodeProp);
  known.force.y.addEventListener("click", changeNodeProp);
  known.displacement.x.addEventListener("click", changeNodeProp);
  known.displacement.y.addEventListener("click", changeNodeProp);

  positionX.addEventListener("change", changeNodeProp);
  positionY.addEventListener("change", changeNodeProp);
  displacementX.addEventListener("change", changeNodeProp);
  displacementY.addEventListener("change", changeNodeProp);
  forceX.addEventListener("change", changeNodeProp);
  forceY.addEventListener("change", changeNodeProp);

  addElementBtn.addEventListener("click", (_evt) => {
    const nodeLen = model.nodes_len();
    if (nodeLen <= 3) {
      alert("too few nodes to make an element");
      elementIndex.value = "";
      return;
    }
    const elementLen = model.elements_len();
    elementIndex.value = elementLen.toString();
    model.add_elem();

    const indices = model.get_element_indices(elementLen);
    elementNodes[0].value = indices[0].toString();
    elementNodes[1].value = indices[1].toString();
    elementNodes[2].value = indices[2].toString();
  });

  function changeElementIndex() {
    let index = parseInt(elementIndex.value);
    if (model.elements_len() <= 0 || isNaN(index)) {
      elementIndex.value = "";
      elementNodes[0].value = "";
      elementNodes[1].value = "";
      elementNodes[2].value = "";
      return;
    }
    if (index < 0) {
      index = 0;
      elementIndex.select();
    } else if (index >= model.elements_len()) {
      index = model.elements_len() - 1;
      elementIndex.select();
    }
    elementIndex.value = index.toString();

    const indices = model.get_element_indices(index);
    elementNodes[0].value = indices[0].toString();
    elementNodes[1].value = indices[1].toString();
    elementNodes[2].value = indices[2].toString();
  }

  elementIndex.addEventListener("change", changeElementIndex);

  function changeElementIndices(_evt) {
    const index = parseInt(elementIndex.value);
    const indices = new Uint32Array(3);

    for (let i = 0; i < 3; i++){
      indices[i] = parseInt(elementNodes[i].value);
    }

    model.set_element_indices(index, indices);
  }

  elementNodes[0].addEventListener("change", changeElementIndices);
  elementNodes[1].addEventListener("change", changeElementIndices);
  elementNodes[2].addEventListener("change", changeElementIndices);
    
  stepBtn.addEventListener("click", (evt) => {
    model.step();
    changeNodeIndex(evt);
  });
  /// TODO: 
  /// 1. add elements
  /// 2. step results
  /// 3. delete nodes and elements
  /// 4. graphical output
  /// 5. add error messages

  changeMaterials();
}

if (document.readyState === 'loading') {
  document.addEventListener('DOMContentLoaded', setup);
} else {
  setup();
}