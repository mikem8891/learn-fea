// @ts-check

function getElementById(id) {
  let elem = document.getElementById(id);
  if (elem === null) {
    throw new Error(`element with id = "${id}" not found`);
  }
  return elem;
}