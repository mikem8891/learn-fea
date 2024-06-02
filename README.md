# learn-fea
Learning finite element analysis

Lower-case bold variables are vectors.
Upper-case bold variables are matrices.

Staring with a 2D solid material. Let \\(\mathbf{u}\\) be the displacement
field.  Let \\(\mathbf{\epsilon}\\) be the normal \\(\epsilon\\) and shear
\\(\gamma\\) strains.  Let \\(\mathbf{\sigma}\\) be the normal \\(\sigma\\) and
shear \\(\tau\\) stresses.

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

\\[\mathbf{\sigma} =
\begin{pmatrix}
\sigma_x \\\\ \sigma_y \\\\ \tau_{xy}
\end{pmatrix}
\\]

Let \\(\mathbf{L}\\) be a differential operator for converting \\(\mathbf{u}\\)
to \\(\mathbf{\epsilon}\\).

\\[
\mathbf{L(u)}=
\begin{bmatrix}
\frac{\partial}{\partial x} & 0
\\\\ 0 & \frac{\partial}{\partial y} 
\\\\ \frac{\partial}{\partial y} &
\frac{\partial}{\partial x}
\end{bmatrix}
\begin{pmatrix}
u_x \\\\ u_y
\end{pmatrix}
=
\mathbf{\epsilon}
\\]

Let \\(\mathbf{E}\\) be a elasticity matrix to convert strain to stress, where E is Young's modulus, G is the modulus of rigidity, and \\(\nu\\) is Poisson's ratio.

\\[\mathbf{E} =
\begin{bmatrix}
\frac{E}{1-\nu^2} & \frac{E\nu}{1-\nu^2} & 0
\\\\ \frac{E\nu}{1-\nu^2} & \frac{E}{1-\nu^2} & 0 
\\\\ 0 & 0 & G
\end{bmatrix}
\\]

<script src="https://polyfill.io/v3/polyfill.min.js?features=es6"></script>
<script id="MathJax-script" async src="https://cdn.jsdelivr.net/npm/mathjax@3/es5/tex-mml-chtml.js">
