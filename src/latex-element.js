// @ts-check

const SHEET = new CSSStyleSheet();
SHEET.replaceSync("math{font-family: inherit; overflow-x: auto; overflow-y: hidden;}");

/**
 * @param {LatexElement} aElement
 */
function updateMathMLOutput(aElement) {
  var tex = aElement.textContent,
    display = aElement.getAttribute("display"),
    dir = aElement.getAttribute("dir");
  const shadowRoot = /** @type {ShadowRoot} */(aElement.shadowRoot);
  try {
    // Parse the LaTeX input and replace it with the MathML output.
    shadowRoot.innerHTML = TeXZilla.toMathMLString(
      tex,
      display === "block",
      dir === "rtl",
      true
    );
  } catch (e) {
    // Parsing failed: use an <merror> with the original LaTeX input.
    shadowRoot.innerHTML =
      "<math><merror><mtext>" + tex + "</mtext></merror></math>";
  }
}

class LatexElement extends HTMLElement {
    constructor() {
      super()
      const shadowRoot = this.attachShadow({ mode: "open" });
      shadowRoot.adoptedStyleSheets = [SHEET];
      this.mo = new MutationObserver((_recs) => {
        updateMathMLOutput(this);
      })
      this.mo.observe(this, { characterData: true, childList: true, attributes: true });
      updateMathMLOutput(this);
    }
    
    /**
   * @param {string} aName
   * @param {string | null} _aOld
   * @param {string | null} aNew
   */
    attributeChangedCallback(aName, _aOld, aNew) {
      if (aName === "dir" || aName === "display") {
        const shadowRoot = /** @type {ShadowRoot} */(this.shadowRoot);
        if (aNew === null) {
          shadowRoot.firstElementChild?.removeAttribute(aName);
        } else {
          shadowRoot.firstElementChild?.setAttribute(aName, aNew);
        }
      }
    }
} 

customElements.define("la-tex", LatexElement);
