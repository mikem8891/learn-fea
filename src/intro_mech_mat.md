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
  
<div style="mask: url(stress.svg) no-repeat center; background-color: currentColor; height: 100px;"></div>

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
