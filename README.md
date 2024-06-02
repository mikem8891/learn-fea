# learn-fea
Learning finite element analysis

Lower-case bold variables are vectors.
Upper-case bold variables are matrices.

Staring with a 2D solid material. Let \\(\mathbf{u}\\) be the displacement field.  Let \\(\mathbf{\epsilon}\\) be the strains in the xy-plane.

\\[\mathbf{u} = \begin{pmatrix}
u_x \\\\ u_y
\end{pmatrix} \\]

\\[\mathbf{\epsilon} =
\begin{pmatrix}
\epsilon_x \\\\ \epsilon_y \\\\ \gamma_{xy}
\end{pmatrix} =
\begin{pmatrix}
\frac{\partial u_x}{\partial x}
\\\\ \frac{\partial u_y}{\partial y} 
\\\\ \frac{\partial u_x}{\partial y} +
\frac{\partial u_y}{\partial x}
\end{pmatrix}
\\]

Let L be a differential operator for converting u to \epsilon.
\\[
\mathbf{L(u)}=
\begin{pmatrix}
\frac{\partial}{\partial x}
\\\\ \frac{\partial}{\partial y} 
\\\\ \frac{\partial}{\partial y} +
\frac{\partial}{\partial x}
\end{pmatrix}
\begin{pmatrix}
u_x \\\\ u_y
\end{pmatrix}
=
\mathbf{\epsilon}
\\]

<script src="https://polyfill.io/v3/polyfill.min.js?features=es6"></script>
<script id="MathJax-script" async src="https://cdn.jsdelivr.net/npm/mathjax@3/es5/tex-mml-chtml.js">

