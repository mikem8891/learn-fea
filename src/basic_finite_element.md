# Basics of Finite Element

Finite element method is used to solve partial differential equations on domains with complex geometry.  This is in contrast to finite difference method which usually  requires a domain to be easily divided into equal sized tiles or blocks.  Finite element method uses elements made up of nodes.  Adjacent elements share the nodes at their common boundary.  The solution is approximated as the sum below.

<la-tex display="block">
  \mathbf{u}(\mathbf{x},t)\approx\sum_i{N_i(\mathbf{x}) \mathbf{u}_i(t)}
</la-tex>

Where <la-tex>N_i</la-tex> are trail functions and <la-tex>\mathbf u_i</la-tex> are the values of the solution at each node.  The trail functions are determined by the domain and the type of elements used.  A trial function for node i will have a value of 1 at node i and 0 at all other nodes.  In addition, the trail function will be 0 outside of any elements it is a part of.  Depending on the type of element, the trail function may be linear, quadratic, or some other distribution. When all the trail functions in an element are summed, the value across the entire element is 1.

> #### Example 1
> 
> Lets look at an example of trail functions.  The simplest 2D element is a linear triangular element. This element is not accurate for structural finite element, so we will look at better > elements later.  Let <la-tex>D</la-tex> be the triangular domain of an element between nodes at (0, 0), (1, 0), and (0, 1).
> 
> <svg height="125" width="175">
>   <defs>
>     <!-- marker for the nodes -->
>     <marker id="node" markerWidth="4" markerHeight="4" refX="2" refY="2" orient="0">
>       <circle cx="2" cy="2" r="2"/>
>     </marker>
>   </defs>
>   <!-- element -->
>   <polygon points="50,25 125,100 50,100" class="element" />
>   <!-- text labelling the coordinates -->
>   <text x="40" y="25" text-anchor="end">
>     (0, 1)
>   </text>
>   <text x="75" y="80" text-anchor="middle" class="var">
>     D
>   </text>
>   <text x="40" y="100" text-anchor="end">
>     (0, 0)
>   </text>
>   <text x="135" y="100" text-anchor="start">
>     (1, 0)
>   </text>
>   <!-- label of the element domain -->
> </svg>
> <la-tex display="block">
>   D = \{ (x, y) : 0&lt;x,\,0&lt;y,\,x+y&lt;1\}
> </la-tex>
> 
> Each node has its own trail function.  The trail function is 1 at its node and zero at the other nodes.
> 
> <div style="max-width: fit-content; margin: auto;">
>   <la-tex>N_1 = 1-x-y</la-tex><br>
>   <la-tex>N_2 = x</la-tex><br>
>   <la-tex>N_3 = y</la-tex><br>
> </div>  
> <br>

Since the approximate solution is a product of a space-only dependent function (<la-tex>N_i(\mathbf{x})</la-tex>) and a time-only dependent value (<la-tex>\mathbf{u}_i(t)</la-tex>), the stress and strain only depend on the derivatives of the trail functions and only the value at the node. For this reason, we let <la-tex>\mathbf{B}</la-tex> be the derivative of the trail function used to calculate stress and strain.

<la-tex display="block">
  \mathbf{B} = \mathbf{L}(N) = 
  \begin{bmatrix}
    \frac{\partial N}{\partial x} & 0 \\
    0 & \frac{\partial N}{\partial y} \\
    \frac{\partial N}{\partial y} &
    \frac{\partial N}{\partial x}
  \end{bmatrix}
</la-tex>

> #### Example 2
>
> Lets calculate <la-tex>\mathbf{B}</la-tex> for the trail functions from [example 1](#example-1) above.
> 
> <la-tex display="block">
>   \mathbf{B}_1 = 
>   \begin{bmatrix}
>     -1 &  0 \\
>      0 & -1 \\
>     -1 & -1
>   \end{bmatrix} ,\:
>   \mathbf{B}_2 = 
>   \begin{bmatrix}
>     1 & 0 \\
>     0 & 0 \\
>     0 & 1
>   \end{bmatrix},\:
>   \mathbf{B}_3 = 
>   \begin{bmatrix}
>     0 & 0 \\
>     0 & 1 \\
>     1 & 0
>   \end{bmatrix}
> </la-tex><br>

The strain and stress can be approximated using the equations below.

<la-tex display="block">
  \mathbf{\epsilon}\approx\sum_i{\mathbf{B}_i \mathbf{u}_i(t)} 
  ,\:
  \mathbf{\sigma}\approx\mathbf{E}\sum_i{\mathbf{B}_i \mathbf{u}_i(t)}
</la-tex>

Now lets rewrite Newton's second law from above in terms of the approximate
solution.

<la-tex display="block">
  \mathbf{f}_b + \mathbf{L}^T(\mathbf{\sigma}) =
  \sum_i{\rho N_i \frac{\mathrm{d} \mathbf{u}_i}{\mathrm{d} t^2}}
</la-tex>

<p>We will multiply Newton's second law above by <la-tex>N_j</la-tex>, the nodes' trail functions, to generate a system of ordinary differential equations that can be solved.

<la-tex display="block">
  N_j \mathbf{f}_b + N_j \mathbf{L}^T(\mathbf{\sigma}) =
  \sum_i{\rho N_i N_j \frac{\mathrm{d} \mathbf{u}_i}{\mathrm{d} t^2}}
</la-tex>

We apply the product rule to the stress term.  This will allow us to break up the term into an internal stress force and an external stress force applied at the boundary.

<la-tex display="block">
  N_j \mathbf{f}_b + 
    \mathbf{L}^T(N_j  \mathbf{\sigma}) - 
    \mathbf{L}^T(N_j) \mathbf{\sigma}  =
  \sum_i{\rho N_i N_j \frac{\mathrm{d} \mathbf{u}_i}{\mathrm{d} t^2}}
</la-tex>

<p>We will give each term its own variable and integrate over the domain of
the element.</p>

<la-tex display="block">
  \mathbf{f}_{bj} + \mathbf{f}_{sj} - \sum_i \mathbf{K}_{ji}\mathbf{u}_i = 
  \sum_i M_{ji} \frac{\mathrm{d} \mathbf{u}_i}{\mathrm{d} t^2}
</la-tex>
<br>
<la-tex display="block">
  \mathbf{f}_{bj} = \iint_D N_j \mathbf{f}_b \, \mathrm{dA}
</la-tex>
<br>
<la-tex display="block">
  \mathbf{f}_{sj} = \iint_D \mathbf{L}^T(N_j \mathbf{\sigma}) \, \mathrm{dA}
</la-tex>
<br>
<la-tex display="block">
  \mathbf{K}_{ji}\mathbf{u}_i = 
    \iint_D \mathbf{B}^T_j \mathbf{\sigma} \, \mathrm{dA} =
    \left( \iint_D \mathbf{B}^T_j\mathbf{E}\mathbf{B}_i \mathrm{dA} \right) \mathbf{u}_i
</la-tex>
<br>
<la-tex display="block">
  M_{ji} = \iint_D \rho N_i N_j \mathrm{dA}
</la-tex>

Where <la-tex>\mathbf{f}_{s}</la-tex> is the force due to the stress, <la-tex>\mathbf{K}</la-tex> is the stiffness, and <la-tex>M</la-tex> is the mass.

The term inside the integral for the stiffness matrix evaluate to the following matrix.
<la-tex display="block">
  \mathbf{B}^T_j\mathbf{E}\mathbf{B}_i = 
</la-tex><br>
<la-tex display="block">
  \begin{bmatrix}
    E' \frac{\partial N_i}{\partial x} \frac{\partial N_j}{\partial x} + 
    G  \frac{\partial N_i}{\partial y} \frac{\partial N_j}{\partial y} & 
    E'\nu \frac{\partial N_i}{\partial y} \frac{\partial N_j}{\partial x} + 
    G     \frac{\partial N_i}{\partial x} \frac{\partial N_j}{\partial y} \\\\
    E'\nu \frac{\partial N_i}{\partial x} \frac{\partial N_j}{\partial y}+ 
    G     \frac{\partial N_i}{\partial y} \frac{\partial N_j}{\partial x} & 
    E' \frac{\partial N_i}{\partial y} \frac{\partial N_j}{\partial y} + 
    G  \frac{\partial N_i}{\partial x} \frac{\partial N_j}{\partial x}
  \end{bmatrix}
</la-tex>

> #### Example 3
>  Now lets calculate mass values and stiffness matrices using the trails 
>  functions from [example](#example-1) above.  We will start with the mass value for <la-tex>i=1</la-tex> and 
>  <la-tex>j=1</la-tex>.  For convenience, we will apply
>  the substitution <la-tex>v = 1-x</la-tex> and <la-tex>w = v-y</la-tex>.
>  <la-tex display="block">
>    M_{11} = \rho \iint_D {N_1}^2 \mathrm{dA}
>  </la-tex><br>
>  <la-tex display="block">
>    = \rho \int_0^1 \int_0^{1-x} (1-x-y)^2 \text{ dy dx}
>  </la-tex><br>
>  <la-tex display="block">
>    = \rho \int_0^1 \int_0^{v} w^2 \text{ dw dv}
>    = \frac{\rho}{3} \int_0^1 v^3 \text{ dv}
>    = \frac{\rho}{12}
>  </la-tex>
>  
>  <p>The result is the same for <la-tex>M_{22}</la-tex> and 
>  <la-tex>M_{33}</la-tex>. Now we look at <la-tex>i=1</la-tex> and 
>  <la-tex>j=2</la-tex>.</p>
>  <la-tex display="block">
>    M_{21} = \rho \iint_D N_1 N_2 \text{ dA}
>  </la-tex><br>
>  <la-tex display="block">
>    = \rho \int_0^1 \int_0^{1-x} (1-x-y)x \text{ dy dx}
>  </la-tex><br>
>  <la-tex display="block">
>    = \rho \int_0^1 (1-v) \int_0^{v} w \text{ dw dv}
>  </la-tex><br>
>  <la-tex display="block">
>    = \frac{\rho}{2} \int_0^1 (1-v)v^2 \text{ dv}
>    = \frac{\rho}{24}
>  </la-tex><br>
>  The result is the same for <la-tex>M_{13}</la-tex>, <la-tex>M_{21}</la-tex>, <la-tex>M_{23}</la-tex>, <la-tex>M_{31}</la-tex>, and <la-tex>M_{32}</la-tex>.  Written as a matrix, the result looks like this.
>
>  <la-tex display="block">
>  \mathbf{M} = \frac{\rho}{24}
>  \begin{bmatrix}
>    2 & 1 & 1 \\
>    1 & 2 & 1 \\
>    1 & 1 & 2
>  \end{bmatrix}
>  </la-tex>
>  <p id="2d-tri-lin-stiffness-ex">Now we will look at the stiffness matrices. 
>  </p>
>  <la-tex display="block">
>  \mathbf K_11 = \frac{1}{2}
>  \begin{bmatrix}
>    E'+G & E'\nu+G \\ 
>    E'\nu+G & E'+G
>  \end{bmatrix}
>  </la-tex><br>
>  <la-tex display="block">
>    \mathbf K_21 = \mathbf K_12^T =
>    \frac 1 2
>    \begin{bmatrix}
>      -E' & -E'\nu \\ 
>      -G  & -G
>    \end{bmatrix}
>  </la-tex><br>
>  <la-tex display="block">
>    \mathbf K_31 = \mathbf K_13^T =
>    \frac 1 2
>    \begin{bmatrix}
>      -G      & -G \\ 
>      -E'\nu  & -E'
>    \end{bmatrix}
>  </la-tex><br>
>  <la-tex display="block">
>    \mathbf K_32 = \mathbf K_23^T =
>    \frac 1 2
>    \begin{bmatrix}
>      0      & G \\ 
>      E'\nu  & 0
>    \end{bmatrix}
>  </la-tex><br>
>  <la-tex display="block">
>    \mathbf K_22 = 
>    \frac 1 2
>    \begin{bmatrix}
>      E' & 0 \\ 
>      0  & G
>    \end{bmatrix}, \:
>    \mathbf K_33 = 
>    \frac 1 2
>    \begin{bmatrix}
>      G & 0 \\ 
>      0 & E'
>    \end{bmatrix}
>  </la-tex><br>

<p>It is relatively straight forward to calculate these values, except 
<la-tex>\mathbf{f}_{sj}</la-tex>.  Here we need to use Green's theorem.  This shows
us that <la-tex>\mathbf{f}_{sj}</la-tex> is only a result of the stress at the 
boundary.  <la-tex>\partial D</la-tex> is the boundary of domain <la-tex>D</la-tex>.</p>

<la-tex display="block">
  \mathbf{f}_{sj} = \iint_D \mathbf{L}^T(N_j \mathbf{\sigma}) \, \mathrm{dA}
</la-tex>
<br>
<la-tex display="block">
  =\iint_D 
    \begin{pmatrix}
      \frac{\partial}{\partial x}(N_j\sigma_x) +
      \frac{\partial}{\partial y}(N_j\tau_{xy})\\
      \frac{\partial}{\partial x}(N_j\tau_{xy}) +
      \frac{\partial}{\partial y}(N_j\sigma_y)
    \end{pmatrix}
    \mathrm{dA}
</la-tex>
<br>
<la-tex display="block">
  =\oint_{\partial D}
    N_j
    \begin{pmatrix}
      \sigma_x dy -
      \tau_{xy} dx \\
      \tau_{xy} dy -
      \sigma_y dx
    \end{pmatrix}
</la-tex>

It is more useful to apply boundary conditions in terms of the bearing stress <la-tex>\sigma_b</la-tex> and the shear friction stress <la-tex>\tau_f</la-tex> on the surface.

<la-tex display="block">
  \mathbf{f}_{sj} =
  \oint_{\partial D}
    N_j
    \left(
      \sigma_b \, \hat{\mathbf{N}} +
      \tau_f   \, \hat{\mathbf{T}}
    \right)
    ds
</la-tex>
<p>Where <la-tex>\hat{\mathbf{N}}</la-tex> is the normal unit vector,
<la-tex>\hat{\mathbf{T}}</la-tex> is the tangent unit vector, and
<la-tex>ds</la-tex> is the differential length.</p>

<la-tex display="block">
  \hat{\mathbf{N}} = 
  \begin{pmatrix}
    \frac{dy}{ds} \\
    -\frac{dx}{ds}
  \end{pmatrix},\:
  \hat{\mathbf{T}} = 
  \begin{pmatrix}
    \frac{dx}{ds} \\
    \frac{dy}{ds}
  \end{pmatrix}
</la-tex>
<br>
<la-tex display="block">
  ds = \sqrt{dx^2 + dy^2}
</la-tex>
<p>Note <la-tex>\sigma_b</la-tex> is negative for a compressive stress.  
<la-tex>\tau_f</la-tex>, <la-tex>\hat\mathbf T</la-tex> and <la-tex>\oint_{\partial D}</la-tex> go 
counterclockwise around <la-tex>D</la-tex>.  <la-tex>\hat\mathbf N</la-tex> 
points outward.</p>

>  #### Example 4
>  Lets calculate <la-tex>\mathbf f_s</la-tex>.  We will consider a bearing stress
>  <la-tex>\sigma_b</la-tex> on the left side of the element.
>
>  let,
>  <div style="max-width: fit-content; margin: auto;">
>    <la-tex>x = 0</la-tex><br>
>    <la-tex>y = 1-t</la-tex><br>
>  </div><br>
>  <la-tex display="block">N_1 = 1-x-y = t , \: \tau_f = 0</la-tex><br>
>  <la-tex display="block">
>  \hat{\mathbf{N}} \, ds=
>  \begin{pmatrix}
>    \frac{dy}{dt} \\
>    -\frac{dx}{dt}
>  \end{pmatrix} \, dt=
>  \begin{pmatrix}
>    -1 \\ 0
>  \end{pmatrix} \, dt
>  </la-tex><br>
>  <la-tex display="block">
>  \mathbf{f}_{s1} =
>  \int_{\partial D}
>    N_1
>    \left(
>      \sigma_b \, \hat{\mathbf{N}} +
>      \tau_f   \, \hat{\mathbf{T}}
>    \right)
>    ds
>  </la-tex><br>
>  <la-tex display="block">
>  = \sigma_b
>  \begin{pmatrix}
>    -1 \\ 0
>  \end{pmatrix}
>  \int_0^1 t \, dt
>  = \frac{\sigma_b}{2}
>  \begin{pmatrix}
>    -1 \\ 0
>  \end{pmatrix}
>  </la-tex><br>
>  <p><la-tex>N_2</la-tex> is zero on the entire left side.</p>
>  <la-tex display="block">
>    N_2 = x = 0 , \: 
>    \mathbf{f}_{s2} = 
>    \begin{pmatrix}
>      0 \\ 0
>    \end{pmatrix}
>  </la-tex><br>
>  <la-tex display="block">
>    N_3 = y = 1 - t
>  </la-tex><br>
>  <la-tex display="block">
>    \mathbf{f}_{s3} =
>    \sigma_b
>    \begin{pmatrix}
>      -1 \\ 0
>    \end{pmatrix}
>    \int^{1}_{0} 1 - t \, dt
>    = \frac{\sigma_b}{2}
>    \begin{pmatrix}
>      -1 \\ 0
>    \end{pmatrix}
>  </la-tex><br>

Finally, lets solve for the displacement, strain, and stress of element.  
For now, we will not consider any body forces, just the external forces. 
Additionally, we will only consider static equilibrium.
These two constraints can be summarized by the two equations below.

<la-tex display="block">
  \mathbf{f}_{bj} =
  \begin{pmatrix}
    0 \\ 0
  \end{pmatrix} ,\:
  \frac{\mathrm{d} \mathbf{u}_i}{\mathrm{d} t^2} =
  \begin{pmatrix}
    0 \\ 0
  \end{pmatrix} 
</la-tex>

With these constraints, Newton's second law on the element simplifies as 
follows.

<la-tex display="block">
  \sum_i \mathbf{K}_{ji}\mathbf{u}_i = 
  \mathbf{f}_{sj}
</la-tex>

This is a set of linear equations, two equations for each node.  Unfortunately this system of equations cannot be solved as-is.  Right now the system has infinite solutions.  If you imagine a force applied to the element, there is nothing keeping it from flying off.  Some of the nodes need to be restrained.  The combination of restrained nodes and applied loads provides our boundary conditions. Nodes are restrained by setting their displacement.  Now we will rearrange the equations by the equations with known forces <la-tex>(\mathbf{f}_k)</la-tex>, and the equations with unknown forces <la-tex>(\mathbf{f}_u)</la-tex>. The terms of the equations are rearranged in terms of known displacements <la-tex>(\mathbf{u}_k)</la-tex> and unknown displacements <la-tex>(\mathbf{u}_u)</la-tex>

<la-tex display="block">
  \mathbf{K}_{kk}\mathbf{u}_k +
  \mathbf{K}_{ku}\mathbf{u}_u= 
  \mathbf{f}_k
</la-tex><br>
<la-tex display="block">
  \mathbf{K}_{uk}\mathbf{u}_k +
  \mathbf{K}_{uu}\mathbf{u}_u= 
  \mathbf{f}_u
</la-tex>

First solve for <la-tex>(\mathbf{u}_u)</la-tex> in the top equation.  Then solve for <la-tex>(\mathbf{f}_u)</la-tex> using the bottom equation.

<la-tex display="block">
  \mathbf{K}_{ku}\mathbf{u}_u= 
  \mathbf{f}_k -
  \mathbf{K}_{kk}\mathbf{u}_k
</la-tex><br>
<la-tex display="block">
  \mathbf{f}_u =
  \mathbf{K}_{uk}\mathbf{u}_k +
  \mathbf{K}_{uu}\mathbf{u}_u
</la-tex>

The top equation can be solved directly using 
Gaussian elimination or approximated using Jacobi or Gauss-Seidel iterative 
methods.  Gaussian elimination is slow, while Jacobi or Gauss-Seidel iterative 
methods are faster and easier to implement but less accurate.  Gauss-Seidel 
method is faster and uses less memory than Jacobi method; however, Jacobi 
method lends itself well to massively parallel computing that can be run on GPUs. 
There is a risk that Gauss-Seidel or Jacobi method will not converge on a 
solution; however, static structural FEA has a good chance of converging with 
these methods.

>  #### Example 5
>  Using the element from the previous examples we will calculate the the displacement, strain, and stress.  The element has three nodes, for a total of six equations to be solved.
>  
>  <la-tex display="block">
>    \mathbf{K}_{11} \mathbf{u}_1 +
>    \mathbf{K}_{12} \mathbf{u}_2 +
>    \mathbf{K}_{13} \mathbf{u}_3 =
>    \mathbf{f}_{s1}
>  </la-tex><br>
>  <la-tex display="block">
>    \mathbf{K}_{21} \mathbf{u}_1 +
>    \mathbf{K}_{22} \mathbf{u}_2 +
>    \mathbf{K}_{23} \mathbf{u}_3 =
>    \mathbf{f}_{s2}
>  </la-tex><br>
>  <la-tex display="block">
>    \mathbf{K}_{31} \mathbf{u}_1 +
>    \mathbf{K}_{32} \mathbf{u}_2 +
>    \mathbf{K}_{33} \mathbf{u}_3 =
>    \mathbf{f}_{s3}
>  </la-tex>
>  <p>We will use the following values to get a concrete solution.  Nodes 1 and 2 
>    will be restrained to have no displacement.</p>
>  <la-tex display="block">
>    E' = 30000 , \:\:
>    \nu = 0.3 , \:\:
>    G  = 10000 , \:\:
>    \sigma_b = -30
>  </la-tex><br>
>  <la-tex display="block">
>    \mathbf{u}_1 =
>    \mathbf{u}_2 =
>    \begin{pmatrix}
>      0 \\ 
>      0 
>    \end{pmatrix}
>  </la-tex>
>  To solve for <la-tex>\mathbf{u}_3</la-tex>, we use the third equation from above.  The equation has been simplified based on the constraints on Nodes 1 and 2.
>
>  <la-tex display="block">
>    \mathbf{K}_{33} \mathbf{u}_3 =
>    \mathbf{f}_{s3}
>  </la-tex><br>
>  <la-tex display="block">
>    \frac 1 2
>    \begin{bmatrix}
>      G & 0_{} \\ 
>      0_{} & E'
>    \end{bmatrix}
>    \begin{pmatrix}
>      u_{x3} \\ 
>      u_{y3}
>    \end{pmatrix} =
>    \begin{pmatrix}
>      f_{sx3} \\ 
>      f_{sy3}
>    \end{pmatrix}
>  </la-tex><br>
>  <la-tex display="block">
>    \begin{bmatrix}
>      5000 & 0_{} \\ 
>      0_{} & 15000
>    \end{bmatrix}
>    \begin{pmatrix}
>      u_{x3} \\ 
>      u_{y3}
>    \end{pmatrix} =
>    \begin{pmatrix}
>      15_{} \\ 
>      0_{}
>    \end{pmatrix}
>  </la-tex><br>
>  <la-tex display="block">
>    u_{x3} = 0.003, \:
>    u_{y3} = 0
>  </la-tex>
>  <p>Now we can calculate the forces at Nodes 1 and 2.</p>
>  <la-tex display="block">
>    \mathbf{f}_{s1} =
>    \mathbf{K}_{13} \mathbf{u}_3
>  </la-tex><br>
>  <la-tex display="block">
>    \begin{pmatrix}
>      f_{sx1} \\ 
>      f_{sy1}
>    \end{pmatrix} =
>    \frac 1 2
>    \begin{bmatrix}
>      -G & -E'\nu \\ 
>      -G & -E'
>    \end{bmatrix}
>    \begin{pmatrix}
>      u_{x3} \\ 
>      u_{y3}
>    \end{pmatrix}
>  </la-tex><br>
>  <la-tex display="block">
>    \begin{pmatrix}
>      f_{sx1} \\ 
>      f_{sy1}
>    \end{pmatrix} =
>    \begin{bmatrix}
>      -5000 & -4500 \\ 
>      -5000 & -15000
>    \end{bmatrix}
>    \begin{pmatrix}
>      0.003 \\ 
>      0
>    \end{pmatrix}
>  </la-tex><br> 
>  <la-tex display="block">
>    f_{sx1} = -15, \:
>    f_{sy1} = -15
>  </la-tex><br>
>  <la-tex display="block">
>    \mathbf{f}_{s2} =
>    \mathbf{K}_{23} \mathbf{u}_3
>  </la-tex><br>
>
>  <la-tex display="block">
>    f_{sx2} = 0, \:
>    f_{sy2} = 15
>  </la-tex>
>  <p>Now strains and stresses.</p>
>  <la-tex display="block">
>    \mathbf{\epsilon} = \sum_i{\mathbf{B}_i \mathbf{u}_i} = 
>    \mathbf{B}_3 \mathbf{u}_3
>  </la-tex><br>
>  <la-tex display="block">
>    \begin{pmatrix}
>      \epsilon_x \\ \epsilon_y \\ \gamma_{xy}
>    \end{pmatrix} =
>    \begin{bmatrix}
>      0 & 0 \\
>      0 & 1 \\
>      1 & 0
>    \end{bmatrix}
>    \begin{pmatrix}
>      0.003 \\ 
>      0
>    \end{pmatrix} =
>    \begin{pmatrix}
>      0 \\ 0 \\ 0.003
>    \end{pmatrix}
>  </la-tex><br>
>  <la-tex display="block">
>    \mathbf{\sigma} =
>    \mathbf{E\epsilon} 
>  </la-tex><br>
>  <la-tex display="block">
>    \begin{pmatrix}
>      \sigma_x \\ \sigma_y \\ \tau_{xy}
>    \end{pmatrix} =
>    \begin{bmatrix}
>      E' & E'\nu & 0 \\ 
>      E'\nu & E' & 0 \\
>      0 & 0 & G
>    \end{bmatrix}
>    \begin{pmatrix}
>      \epsilon_x \\ \epsilon_y \\ \gamma_{xy}
>    \end{pmatrix}
>  </la-tex><br>
>  <la-tex display="block">
>    =
>    \begin{bmatrix}
>      30000 & 9000 & 0 \\ 
>      9000 & 30000 & 0 \\
>      0 & 0 & 10000
>    \end{bmatrix}
>    \begin{pmatrix}
>      0 \\ 0 \\ 0.003
>    \end{pmatrix}
>  </la-tex><br>
>  <la-tex display="block">
>    \begin{pmatrix}
>      \sigma_x \\ \sigma_y \\ \tau_{xy}
>    \end{pmatrix} =
>    \begin{pmatrix}
>      0 \\ 0 \\ 30
>    \end{pmatrix}
>  </la-tex>
>  <p>Notice these stresses and strains are uniform throughout the entire element.
>  This makes for a poor approximation of the actual stresses and strains.  This is 
>  why linear elements are bad for modeling material mechanics.  An actual part 
>  with these loads would have non-uniform stresses, including bending.  A single
>  linear element cannot not model bending or transverse shear stress.  To 
>  properly model these stresses we would need many linear elements or a few more
>  sophisticated elements, like quadratic elements.</p>
