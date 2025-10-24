# Elements Working Together

Using multiple elements is not that much different from using a single element.  The elements interact through their mutual nodes. Each element needs to have its properties calculated.  As a review, the trail functions are defined for each node on the domain of the element, then the stiffness matrix is calculated for each node of the element.  You may notice that the stiffness matrices for some of the nodes have already been calculated from previous elements; however, they do not apply to the domain of any new element you add.  To get the full stiffness matrix you need to sum the matrices over the domains of all the elements.

<la-tex display="block">
  \mathbf{K}_{ji} =
  \iint_D \mathbf{B}^T_j\mathbf{E}\mathbf{B}_i \mathrm{dA} =
  \sum_k \iint_{D_k} \mathbf{B}^T_j\mathbf{E}\mathbf{B}_i \mathrm{dA}
</la-tex>

Where, <la-tex>D_k</la-tex> is the domain of the <la-tex>k^{th}</la-tex> element and <la-tex>D</la-tex> is the union of all the element domains.  The same applies for the mass.

> #### Example 4
> Expanding from the previous [example 1](basic_finite_element.md#example-1) of a single triangular linear element, we will add a second element to create a square.  The new element will share Nodes 2 and 3 and introduce Node 4 at (1, 1).  The domain of the old element will be denoted <la-tex>D_1</la-tex> and this new element will be denoted <la-tex>D_2</la-tex>.
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
>   <polyline points="50,25 125,25 125,100" class="element" />
>   <!-- text labelling the coordinates -->
>   <text x="40" y="25" text-anchor="end">
>     (0, 1)
>   </text>
>   <text x="135" y="25" text-anchor="start">
>     (1, 1)
>   </text>
>   <text x="100" y="50" text-anchor="middle" class="var">
>     D<tspan class="subscript">2</tspan>
>   </text>
>   <text x="75" y="80" text-anchor="middle" class="var">
>     D<tspan class="subscript">1</tspan>
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
>   D_2 = \{ (x, y) : x&lt;1,\,y&lt;1,\,1&lt;x+y\}
> </la-tex>
>
> <div style="max-width: fit-content; margin: auto;">
>   <la-tex>N_2 = 1-y</la-tex><br>
>   <la-tex>N_3 = 1-x</la-tex><br>
>   <la-tex>N_4 = x+y-1</la-tex>
> </div>
>
> Below is our new system of equations.
>
> <la-tex display="block">
>   \mathbf{K}_{11} \mathbf{u}_1 +
>   \mathbf{K}_{12} \mathbf{u}_2 +
>   \mathbf{K}_{13} \mathbf{u}_3 +
>   \mathbf{K}_{14} \mathbf{u}_4 =
>   \mathbf{f}_{s1}
> </la-tex>
> <br>
> <la-tex display="block">
>   \mathbf{K}_{21} \mathbf{u}_1 +
>   \mathbf{K}_{22} \mathbf{u}_2 +
>   \mathbf{K}_{23} \mathbf{u}_3 +
>   \mathbf{K}_{24} \mathbf{u}_4 =
>   \mathbf{f}_{s2}
> </la-tex>
> <br>
> <la-tex display="block">
>   \mathbf{K}_{31} \mathbf{u}_1 +
>   \mathbf{K}_{32} \mathbf{u}_2 +
>   \mathbf{K}_{33} \mathbf{u}_3 +
>   \mathbf{K}_{34} \mathbf{u}_4 =
>   \mathbf{f}_{s3}
> </la-tex>
> <br>
> <la-tex display="block">
>   \mathbf{K}_{41} \mathbf{u}_1 +
>   \mathbf{K}_{42} \mathbf{u}_2 +
>   \mathbf{K}_{43} \mathbf{u}_3 +
>   \mathbf{K}_{44} \mathbf{u}_4 =
>   \mathbf{f}_{s4}
> </la-tex>
>
> The only unknown Nodes are <la-tex>\mathbf {u}_3</la-tex> and <la-tex>\mathbf {u}_4</la-tex>, while <la-tex>\mathbf {u}_1 = \mathbf {u}_2 = \mathbf {0}</la-tex>.
>
> <la-tex display="block">
>   \mathbf{K}_{33} \mathbf{u}_3 +
>   \mathbf{K}_{34} \mathbf{u}_4 =
>   \mathbf{f}_{s3}
> </la-tex>
> <br>
> <la-tex display="block">
>   \mathbf{K}_{43} \mathbf{u}_3 +
>   \mathbf{K}_{44} \mathbf{u}_4 =
>   \mathbf{f}_{s4}
> </la-tex>
>
> Now we calculate the four stiffness matrices we need.  Note, we already calculated the stiffness matrix <la-tex>\mathbf{K}_33</la-tex>; however, we need to add the new domain to it.
>
> <la-tex display="block">
>   \mathbf{K}_33 =
>   \frac 1 2
>   \begin{bmatrix}
>     G & 0 \\
>     0 & E'
>   \end{bmatrix} +
>   \frac 1 2
>   \begin{bmatrix}
>     E' & 0 \\
>     0  & G
>   \end{bmatrix} =
>   \frac 1 2
>   \begin{bmatrix}
>     E' + G & 0 \\
>     0 & E' + G
>   \end{bmatrix}
> </la-tex>
> <br>
> <la-tex display="block">
>   \mathbf{K}_34 =
>   \mathbf{K}^T_43 =
>   \frac 1 2
>   \begin{bmatrix}
>     -E' &  -E'\nu \\
>     -G  &  -G
>   \end{bmatrix}
> </la-tex>
> <br>
> <la-tex display="block">
>   \mathbf{K}_44 =
>   \frac 1 2
>   \begin{bmatrix}
>     E' + G & E'\nu + G \\
>     E'\nu + G & E' + G
>   \end{bmatrix}
> </la-tex>
>
> Now we solve for our unknown displacements.
>
> <la-tex display="block">
>   \frac 1 2
>   \begin{bmatrix}
>     E'+ G  & 0    & -E'     & -E'\nu \\
>     0      & E'+G & -G      & -G \\
>     -E'    & -G   & E'+G    & E'\nu+G \\
>     -E'\nu & -G   & E'\nu+G & E'+G
>   \end{bmatrix}
>   \begin{pmatrix}
>     u_{x3} \\
>     u_{y3} \\
>     u_{x4} \\
>     u_{y4}
>   \end{pmatrix} =
>   \begin{pmatrix}
>     f_{sx3} \\
>     f_{sy3} \\
>     f_{sx4} \\
>     f_{sy4}
>   \end{pmatrix}
> </la-tex><br>
> <la-tex display="block">
>   \begin{bmatrix}
>     20000 &     0 & -15000 & -4500 \\
>         0 & 20000 &  -5000 & -5000 \\
>    -15000 & -5000 &  20000 &  9500 \\
>     -4500 & -5000 &   9500 & 20000
>   \end{bmatrix}
>   \begin{pmatrix}
>     u_{x3} \\
>     u_{y3} \\
>     u_{x4} \\
>     u_{y4}
>   \end{pmatrix} =
>   \begin{pmatrix}
>     15 \\
>     0 \\
>     0 \\
>     0
>   \end{pmatrix}
> </la-tex><br>
> <la-tex display="block">
>   \begin{pmatrix}
>     u_{x3} \\
>     u_{y3} \\
>     u_{x4} \\
>     u_{y4}
>   \end{pmatrix} =
>   \begin{pmatrix}
>      0.00195 \\
>      0.00035 \\
>      0.00168 \\
>     -0.00027
>   \end{pmatrix}
> </la-tex><br>
