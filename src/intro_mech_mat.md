# Intro to Mechanics of Materials

Starting with a 2D solid material. Let <la-tex>\mathbf{u}</la-tex> be the
displacement field of the material.  Let <la-tex>\mathbf{\epsilon}</la-tex> be the normal 
<la-tex>(\epsilon)</la-tex> and shear <la-tex>(\gamma)</la-tex> strains.  Let 
<la-tex>\mathbf{\sigma}</la-tex> be the normal <la-tex>(\sigma)</la-tex> and
shear <la-tex>(\tau)</la-tex> stresses.

<la-tex display="block">
\mathbf{u} = 
\begin{pmatrix}
  u_x \\ u_y
\end{pmatrix} \:\:\:\: displacement
</la-tex>

<la-tex display="block">
\mathbf{\epsilon} =
\begin{pmatrix}
  \epsilon_x \\ \epsilon_y \\ \gamma_{xy}
\end{pmatrix} =
\begin{pmatrix}
  \frac{\partial u_x}{\partial x} \\
  \frac{\partial u_y}{\partial y} \\
  \frac{\partial u_x}{\partial y} +
  \frac{\partial u_y}{\partial x}
\end{pmatrix} \:\:\:\: strain
</la-tex>

<la-tex display="block">
  \mathbf{\sigma} =
  \begin{pmatrix}
    \sigma_x \\ \sigma_y \\ \tau_{xy}
  \end{pmatrix} \:\:\:\: stress
</la-tex>

Let <la-tex>\mathbf{L}</la-tex> be a differential operator for converting 
<la-tex>\mathbf{u}</la-tex> to <la-tex>\mathbf{\epsilon}</la-tex>.

<la-tex display="block">
\mathbf{L(u)}=
\begin{bmatrix}
  \frac{\partial}{\partial x} & 0 \\
  0 & \frac{\partial}{\partial y} \\
  \frac{\partial}{\partial y} &
  \frac{\partial}{\partial x}
\end{bmatrix}
\begin{pmatrix}
  u_x \\ u_y
\end{pmatrix}
= \mathbf{\epsilon}
</la-tex>

Let <la-tex>\mathbf{E}</la-tex> be an elasticity matrix to convert strain into
stress, where <la-tex>E</la-tex> is Young’s modulus, <la-tex>G</la-tex> is the
modulus of rigidity, and <la-tex>\nu</la-tex> is Poisson’s ratio.  The matrix
shown is for the plane stress condition; that is <la-tex>\sigma_z = 0</la-tex>.
For convenience, we will apply the substitution <la-tex>E' = E/(1-\nu^2)</la-tex>.

<la-tex display="block">
  \mathbf{\sigma} =
  \mathbf{E\epsilon} =
  \begin{bmatrix}
    E' & E'\nu & 0 \\
    E'\nu & E' & 0 \\
    0 & 0 & G
  \end{bmatrix}
  \begin{pmatrix}
    \epsilon_x \\ \epsilon_y \\ \gamma_{xy}
  \end{pmatrix}
</la-tex>

Newton’s second law at any point in the material may be written as follows:

<la-tex display="block">
  \mathbf{f} = \rho
  \frac{\partial^2 \mathbf{u}}{\partial t^2}
</la-tex>

Where <la-tex>\mathbf{f}</la-tex> is the sum of forces per unit volume and
<la-tex>\rho</la-tex> is the density.  We are concerned with the
stress in the material, but other forces can be added, as needed.  Let
<la-tex>\mathbf{f}_s</la-tex> be the net force due to stress.
  
<svg width="100" height="100" style="display: block; margin: auto;">
  <style>
    line {
      stroke-width: 1.5px;
    }
    text {
      font-size: 11px;
    }
  </style>
  <defs>
    <marker id="arrow" markerWidth="6" markerHeight="4" refX="4" refY="2" orient="auto-start-reverse">
      <path d="M 0 0 L 6 2 L 0 4 z" style="fill: currentColor; stroke: none;" />
    </marker>
  </defs>
  <polygon points="37,37 63,37 63,63 37,63" />
  <text x="43" y="10" text-anchor="end" >
    σ<tspan class="subscript">y</tspan>
  </text>
  <text x="95" y="63" text-anchor="end" >
    σ<tspan class="subscript">x</tspan>
  </text>
  <text x="70" y="30" text-anchor="start" >
    τ<tspan class="subscript">xy</tspan>
  </text>
  <line x1="50" y1="30" x2="50" y2="10" stroke-width="2px" marker-end="url(#arrow)" />
  <line x1="50" y1="70" x2="50" y2="90" stroke-width="2px" marker-end="url(#arrow)" />
  <line x1="30" y1="50" x2="10" y2="50" stroke-width="2px" marker-end="url(#arrow)" />
  <line x1="70" y1="50" x2="90" y2="50" stroke-width="2px" marker-end="url(#arrow)" />
  <line x1="40" y1="32" x2="60" y2="32" stroke-width="2px" marker-end="url(#arrow)" />
  <line x1="60" y1="68" x2="40" y2="68" stroke-width="2px" marker-end="url(#arrow)" />
  <line x1="32" y1="40" x2="32" y2="60" stroke-width="2px" marker-end="url(#arrow)" />
  <line x1="68" y1="60" x2="68" y2="40" stroke-width="2px" marker-end="url(#arrow)" />
</svg>

<la-tex display="block">
  \mathbf{f}_s=
  \mathbf{L}^T(\mathbf{\sigma}) =
  \begin{bmatrix}
    \frac{\partial}{\partial x} & 0 & \frac{\partial}{\partial y} \\
    0 & \frac{\partial}{\partial y} & \frac{\partial}{\partial x}
  \end{bmatrix}
  \begin{pmatrix}
    \sigma_x \\
    \sigma_y \\
    \tau_{xy}
  \end{pmatrix}
</la-tex>

Let <la-tex>\mathbf{f}_b</la-tex> be the sum of other forces per unit volume,
such as body forces like  gravity and magnetism, reference-frame forces like
centrifugal force and coriolis force.

<la-tex display="block">
  \mathbf{f}_b + \mathbf{L}^T(\mathbf{\sigma}) =
  \rho \frac{\partial^2 \mathbf{u}}{\partial t^2}
</la-tex>
