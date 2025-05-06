(function() {
    "use strict";

    
    function updateMathMLOutput(aElement) {
      var tex = aElement.textContent,
        display = aElement.getAttribute("display"),
        dir = aElement.getAttribute("dir");
      try {
        // Parse the LaTeX input and replace it with the MathML output.
        aElement.shadowRoot.innerHTML = TeXZilla.toMathMLString(
          tex,
          display === "block",
          dir === "rtl",
          true
        );
      } catch (e) {
        // Parsing failed: use an <merror> with the original LaTeX input.
        aElement.shadowRoot.innerHTML =
          "<math><merror><mtext>" + tex + "</mtext></merror></math>";
      }
      const SHEET = new CSSStyleSheet();
      SHEET.replaceSync("math{font-family: inherit;}");
      aElement.shadowRoot.adoptedStyleSheets = [SHEET];
    }

    class LaTeX_Element extends HTMLElement {
        constructor() {
          super()
          this.attachShadow({ mode: "open" });
          this.mo = new MutationObserver((recs) => {
            updateMathMLOutput(this);
          })
          this.mo.observe(this, { characterData: true, childList: true, attributes: true });
        }

        attributeChangedCallback(aName, aOld, aNew) {
          if (aName === "dir" || aName === "display") {
            if (aNew === null) {
              this.shadowRoot.firstElementChild.removeAttribute(aName);
            } else {
              this.shadowRoot.firstElementChild.setAttribute(aName, aNew);
            }
          }
        }
    } 

    customElements.define("la-tex", LaTeX_Element);
  })();