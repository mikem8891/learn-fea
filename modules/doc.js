// @ts-check

/**
 * @param {string} id 
  */
export function getElementById(id) {
  let elem = document.getElementById(id);
  if (elem === null) {
    throw new Error(`Element id = "${id}" not found`);
  }
  return elem;
}

/**
 * @param {string} id 
 * @returns {HTMLInputElement}
 */
export function getInputElementById(id) {
  let elem = getElementById(id);
  let tag = elem.tagName.toLowerCase(); 
  if (tag != "input") {
    throw new Error(`Element id = "${id}" is a ${tag}, not an input`);
  }
  return /** @type {HTMLInputElement} */ (elem)
}

/**
 * @param {HTMLInputElement} elem 
 */
export function getFloatFrom(elem) {
  let val = parseFloat(elem.value);
  if (isNaN(val)) {
    throw new Error(`"${elem.value}" is not a number`);
  }
  return val 
}