document.addEventListener("DOMContentLoaded",addStyle);

function addStyle() {
  const sheet = new CSSStyleSheet();
  sheet.replaceSync("math{font-family: Fira Math;}");
  for(const elem of document.getElementsByTagName("la-tex")){
    elem.shadowRoot.adoptedStyleSheets = [sheet];
  }
}