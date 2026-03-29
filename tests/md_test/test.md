# CKKS基础
## 记录的零散知识
以下运算均在mod q下运行
$v$为随机向量，$ct = [c_{0},c_{1}]$，$sk=[1,s]$，$m$表示信息，公钥$pk=[-as+e, a]$，其中$b=-as +e$

$$
\begin{aligned}
c&=[v\cdot pk +(m+e_{0},e_{1})]_{q} \\
c_{0}&=[v\cdot b+m+e_{0}]_{q} \\
c_{1} &= [v\cdot a+e_{1}]_{q} \\
\end{aligned}
$$

解密：
$$
\begin{aligned}
m &\approx[<ct,sk>]_{q} \\
  &=[v(-a\cdot s + e)+m+e_{0}+v\cdot a\cdot s + e_{1}s]_{q} \\
  &=[m+ve+e_{0}+e_{1}s]_{q}
\end{aligned}
$$

我们令$ve+e_{0}+e_{1}s=e$
则$\langle ct, sk \rangle = m+e+qr$，根据mit讲义，r是非常小的
![[Pasted image 20251104221524.png]]
![[Pasted image 20251104221534.png]]
![[Pasted image 20251104221540.png]]


CKKS的主要流程
![[Pasted image 20251110131827.png]]
## Encoding & Decoding
[CKKS explained, Part 2: Full Encoding and Decoding – OpenMined](https://openmined.org/blog/ckks-explained-part-2-ckks-encoding-and-decoding/)

Encoding主要做的事情就是把message从vector的形式变为plaintext的形式，也就是
$$
C^{\frac{N}{2}}=>Z[X]/(X^{N}+1)
$$
其中，$X^{N}+1$是一个（2N次的）分圆多项式，也是一个不可约多项式

> [!summary] 
> ***最终的编码过程如下：***
>- $z \in C^{N/2}$
>- $\pi^{-1}(z) \in \mathbb{H}$
>- $\Delta \pi^{-1}(z)$
>- 投射到$\sigma(R)$中：$\lfloor \Delta \cdot \pi(z) \rceil_{\sigma(R)} \in \sigma(R)$
>- 用$\sigma$进行编码：$m(X)=\sigma^{-1}(\lfloor \Delta \cdot \pi(z) \rceil_{\sigma(R)}) \in R$
>
>***最终的解码过程如下：***
>$z=\pi \circ \sigma(\Delta^{-1}\cdot m)$

> [!warning] 注意
> $\sigma_{-1}(\lfloor \Delta \cdot \pi(z) \rceil_{\sigma(R)}) \in R$其实和$\lfloor \Delta\sigma^{-1}(\cdot \pi(z) )_{\sigma(R)}\rceil \in R$具有相同的意义，本质上是因为 ($\sigma$) 是欧氏空间上的等距变换（正交变换），它精确保持二范数与点间距离，因此最近点投影/取整算子满足$\sigma^{-1}\big(\lfloor y\rceil\big)=\lfloor \sigma^{-1}(y)\rceil$
> 
### canonical embedding 典范嵌入
典范嵌入是一类泛指的嵌入，指的是 ***原环元素在扩张结构中的”自然对应“***
简单的典范嵌入有$Z->Q$
我们处理的典范嵌入则是如下图所示：
$$
\sigma:C[X]/(X^{N}+1)=>C^{N}
$$
典范嵌入将分圆多项式$\Phi_{M}(X)=X^{N}+1$的各个根$\xi,\xi^{3},...,\xi^{2N-1}$带入目标多项式$C[X]/(X^{N}+1)$中逐个evaluate，然后得到的根组合成$C^{N}$
也即：
$$
\begin{aligned}
\sigma(m)&=(m(\xi),m(\xi^{3}),...,m(\xi^{2N-1}))\\
&= (z_{1},...,z_{N})
\end{aligned}
$$
注意，这里的根是从1到2N-1而不是N的

典范嵌入σ定义了一个同构（也就是说它定义了一个双射同态），在计算上它是同态的，在映射上是双射的

#### 分圆多项式的根
$$
\phi_{n}(x)=\prod\limits_{1\le k\le n,gcd(k,n)=1}(x-e^{2i\pi \frac{k}{n}})
$$

当$N=2^{k}$时，有
$$
\phi_{2N}(X)=X^{N}+1
$$
#### 双射的说明
已知：
$$m(X)=\sum\limits_{i=0}^{N-1}=\alpha_{i}X^{i} \in C[X]/(X^{N}+1)$$
评估是如下进行的：
$$\sum\limits_{j=0}^{N-1}\alpha(\xi^{2i-1})^{j}=z_{i},i=1,...,N$$

因此我们可以将其看作一个矩阵乘法：
$$A\alpha=z$$

由于A是一个范德蒙矩阵，且构成$x$的根各不相同，因此存在逆矩阵
![[Pasted image 20251110153907.png]]

所以典范嵌入及其逆变换都是一一对应的，也就是，它是一个双射

### $Z[X]/(X^N+1)$上的典范嵌入

![[Pasted image 20251111170719.png]]
由上图的N=4的简单情况可知，分圆多项式的根实际上是对称的。
在这个例子中，有$\omega_{1}=\overline{\omega_{7}}$，$\omega_{3}=\overline{\omega_{5}}$
考虑到总数=8，我们就有了$\omega_{j}=\overline{\omega_{-j}}$

由于在$m(x) \in Z[X]$中做评估，因此就有了$m(\xi^{j})=\overline{m(\xi^{-j})}=m(\overline{\xi^{-j}})$

由于$\sigma$映射中的每一个向量元素都是由多项式在单位根上评估而来，因此我们有：
$$
\begin{align}
Z_{N}&= (z_{1},...,z_{N})\\
     &= (m(\xi),m(\xi^{3}),...,m(\xi^{2N-1})) \\
     &= (m(\xi),m(\xi^{3}),...,m(\overline{\xi^{3}}),m(\overline{\xi})) \\
     &= (z_{1},z_{2},...,\overline{z_{2}},\overline{z_{1}})
\end{align}
$$

因此，需要在实数参数的$m(x)$的情况下，评估出来的$Z_{N}$实际上自由度只有$N/2$

从典范嵌入的正方向的例子可以说明（但是不是证明），如果我们想要保证典范嵌入的逆方向$\sigma^{-1}:C^{N} \to Z[X]/(X^N+1)$，复向量映射到$Z[X]/(X^N+1)$，我们至少要保证$C^{N}$的自由度减半，也就是变为$C^{N/2}$

从以下的代码以及输出也可以看出相关的结论：我们会发现，当M=8，N=4是，如果输入vector是形如上文的$Z_{N}$的形式，那么转换出来的多项式是实数的，如果输入的vector不是这样，那么转换的多项式$m(x)\notin Z[X]$

![[Pasted image 20251111200020.png]]

### $\pi$操作
$$
\begin{align}
&\pi: \mathbb{H}\to C^{\phi(M)/2} \\
&where~\mathbb{H}=\{(z_{j})_{j \in\mathbb{Z^{*}_{M}}}:z_{j}=\overline{z_{-j}},\forall j \in Z^{*}_{M}\} \in \mathbb{C}^{\phi(M)}
\end{align}
$$

其中$z_{j}$与$z_{-j}$可以参考上文的$\omega$，只不过这里指的是向量的元素
$Z^{*}_{M}$表示的是模$M$乘法群，包含了所有与$M$互素的元素

$\phi(M)$表示$M$的欧拉函数，也就是说，表示所有与$M$互素的函数个数，在$M$是2的幂次的情况下，$\phi(M)=\frac{M}{2}$

因此$\pi$操作实际上表示的是将原先的$C^{N}$映射到$C^{N/2}$中，其更加详细的数学表述如下：
$$
\begin{align}
&\pi(Z) = (z_j)_{j \in S}, \\
&where~S = \{ j \in \mathbb{Z}_M^* \mid 1 \le j < M/2 \}
\end{align}
$$

#### 示例： $M=8$ (对应 $\Phi_8(X)$)

用$M=8, N=\phi(M)=4$来演示这个流程：

1. **索引集：** $\mathbb{Z}_8^* = \{1, 3, 5, 7\}$。
2. **划分：**
    - $j=1 \implies -j=7 \pmod 8$。 配对：$\{1, 7\}$。
    - $j=3 \implies -j=5 \pmod 8$。 配对：$\{3, 5\}$。
3. **代表集：** 我们按规范选择 $S = \{ j \in \mathbb{Z}_8^* \mid 1 \le j < 8/2=4 \} = \{1, 3\}$。
    - $S$ 的大小为 $|S|=2$，这等于 $\phi(8)/2 = 4/2 = 2$。
4. **$\pi$ 操作：**
    - $\mathbb{H}$ 中的一个向量是 $Z = (z_1, z_3, z_5, z_7)$。
    - 它满足 $z_5 = \overline{z_{-5}} = \overline{z_3}$ 且 $z_7 = \overline{z_{-7}} = \overline{z_1}$。
    - 因此， $Z = (z_1, z_3, \overline{z_3}, \overline{z_1})$。
    - $\pi$ 操作提取由 $S=\{1, 3\}$ 索引的分量。

$$\pi(Z) = \pi( (z_1, z_3, z_5, z_7) ) = (z_1, z_3)$$

这个 $(z_1, z_3)$ 向量就在 $\mathbb{C}^{\phi(8)/2} = \mathbb{C}^2$ 空间中。


### $\pi^{-1}$
也就是$\pi$的逆操作，参考上文应该已经可以很好的理解了
***$\pi^{-1}$ 的工作是：***
1. 接收一个 $\phi(M)/2$ 维的复向量 $v$。
2. 用 $v$ 的分量来**填充** $\mathbb{H}$ 向量的“前半部分”（由代表集 $S$ 索引）。
3. 使用共轭对称性 $z_j = \overline{z_{-j}}$ 来**计算并填充** $\mathbb{H}$ 向量的“后半部分”。

#### $\pi^{-1}$ 示例操作
$M=8$ (对应 $\Phi_8(X)$)
- **输入空间：** $\mathbb{C}^{\phi(M)/2} = \mathbb{C}^2$
- **输出空间：** $\mathbb{H} \subset \mathbb{C}^4$
- **索引集：** $\mathbb{Z}_8^* = \{1, 3, 5, 7\}$
- **代表集 (S)：** $S = \{1, 3\}$
- **对称规则：** $z_5 = \overline{z_{-5}} = \overline{z_3}$ ； $z_7 = \overline{z_{-7}} = \overline{z_1}$

假设我们从 $\mathbb{C}^2$ 中选取一个任意的输入向量 $v$：
$$v = (v_1, v_2) = (1 + 2i, \quad 3 - 4i)$$
我们要计算 $Z = \pi^{-1}(v)$。$Z$ 是一个 4 维向量 $Z = (z_1, z_3, z_5, z_7)$。
1. 填充前半部分 (由 S={1, 3} 索引)：
    我们将 $v$ 的分量直接赋给 $Z$ 中由 $S$ 索引的位置。
    - $z_1 = v_1 = \mathbf{1 + 2i}$
    - $z_3 = v_2 = \mathbf{3 - 4i}$
2. 计算后半部分 (由 {5, 7} 索引)：
    我们使用共轭对称规则来计算 $z_5$ 和 $z_7$。
    - 计算 $z_5$：
        $z_5 = \overline{z_3} = \overline{(3 - 4i)} = \mathbf{3 + 4i}$
    - 计算 $z_7$：
        $z_7 = \overline{z_1} = \overline{(1 + 2i)} = \mathbf{1 - 2i}$
1. 组合结果：
    我们得到了完整的 $\mathbb{H}$ 空间向量 $Z$：
    $$Z = (z_1, z_3, z_5, z_7)$$
    $$Z = (\underbrace{1 + 2i}_{z_1}, \quad \underbrace{3 - 4i}_{z_3}, \quad \underbrace{3 + 4i}_{z_5 = \overline{z_3}}, \quad \underbrace{1 - 2i}_{z_7 = \overline{z_1}})$$

### 坐标随机舍入(coordinate-wise random rounding)
$R:Z[X]/(X^N+1)$

事实上，我们并不能直接使用$\sigma: Z[X]/(X^N+1) \to \sigma(R) \in \mathbb{H}$，这是因为$\mathbb{H}$中的元素并不一定在$\sigma(R)$中。
这一点其实可以很好的理解——$\mathbb{H}$是不可数的，而$\sigma(R)$是可数的（由于$R$是可数且与$\sigma(R)$是同态的）

因此需要找到一个方法，将$\mathbb{H}$ 映射到$\sigma(R)$

我们可以引入***坐标随机舍入***：这是一个将实数$x$随机舍入至$\lfloor x\rfloor$或者$\lfloor x \rfloor +1$，其概率取决于$x$与$\lfloor x\rfloor$或者$\lfloor x \rfloor +1$的接近程度，越接近，概率越高。

#### 正交基与Hermite内积
$$
Hermitian~product: \langle a,b\rangle = \sum\limits_{i=1}^{N}x_{i}\overline{y_{i}} = b^{H}a
$$

>[!note] 
>Hermite内积也可以放在x上，两种结果都是内积且呈复共轭关系，不同领域有不同的用法


选取整数多项式($R:Z[X]/(X^N+1)$)中的一组正交基$\{1,X,...,X^{N-1}\}$

由于我们在$R$上选取了一组“正交基”，同样的，我们在$\sigma(R)$上选取出了一组正交基$\beta = (b_{1},b_{2},...,b_{N})=(\sigma(1),\sigma(X),...,\sigma(X^{N-1}))$

因此，我们的
$$
\begin{align}
z&=(z_{1},z_{2},...,z_{n}) \\
 &=\sum\limits_{i=1}^{N}z_{i}b_{i} \\
~~&with~z_{i}=\frac{\langle z,b_{i} \rangle}{\| b_{i}\|^{2}}
\end{align}
$$

因此，我们对$z_{i}$执行***坐标随机舍入***即可

#### $X$是什么？为什么多项式上的内积定义是什么？为什么它们是正交的？
这里的一组基$\{1,X,...,X^{N-1}\}$在传统的定义中（使用积分定义的内积），应该是非正交的（不确定，没验证过），然而，它们能组合为$R$空间中的任意多项式，形如$\alpha_{0}+\alpha_{1}X+\dots+\alpha_{N-1}X^{N-1}$

然而，我们使用多项式系数来定义内积的话，我们会和轻易的发现，它们是正交的，比如
$[1,0,\dots,0]$表示的1，以及$[0,1,\dots,0]$表示的$X$，两者点积就等于0，而点积自身则不等于0

具体数学表述如下：
$$
\begin{align}
&\alpha_{i} = [\underbrace{0,\dots}_{i},1,\dots,0] \\
&\langle \alpha_{i},\alpha_{j} \rangle = 0,i\ne j \\
&\langle \alpha_{i},\alpha_{j} \rangle = 1,i = j
\end{align}
$$

***因此，通过多项式系数定义的点积上，它们是正交的***

而这么定义的原因，是为了保证典范嵌入后的基是正交的。我们会在后面的内容中分析.
#### 为什么典范嵌入后得到的向量也是正交的呢？
回顾先前的内容，我们容易得知，典范嵌入可以表示为如下公式：
$$A\alpha=z$$
> [[#双射的说明]]

$$
Y = \{\xi,\xi^{3},\dots,\xi^{2N-1}\}^{T},where~\xi~is~a~root~of~Cyclotomic~ Polynomial~X^{N}+1
$$
我们首先证明$A=NI$，是一个对角矩阵

***引理***
考虑多项式的内积

>[!note] 
>在这里的上下文中，$\mathbf{z}^{i}=\left[z_{1}^{i}~z_{2}^{i}~\dots ~z_{n}^{i}\right]^{T}$，$\mathbf{z}$表示任一向量

$$
\begin{align}
\langle Y^{i},Y^{j} \rangle &= \sum\limits_{n=1}^{N} (\xi^{2n-1})^{i}\overline{(\xi^{2n-1})^{j}} \\
&= \sum\limits_{n=1}^{N} (\xi^{2n-1})^{i}(\xi^{2n-1})^{-j} \\
&= \sum\limits_{n=1}^{N}\xi^{(2n-1)(i-j)} \\
\end{align}
$$

由于$\xi=e^{i\pi \frac{1}{N}}$，具有e指数的表示，因此我们可以安全的将共轭变为指数的取负号。另外，我们考虑到$\xi$为分圆多项式$X^{N}+1$的根：
$$
\begin{align}
&\xi^{N}=-1 \\
&\xi^{2N}=1
\end{align}
$$

注意到，在i不等于j时，原式是一个等比级数（几何级数）因此，我们有
$$
\begin{align}
if~i\ne j \\
&~~~~~\langle Y^{i},Y^{j} \rangle \\
&=\frac{\xi^{i-j}(1-\xi^{(i-j)2N})}{1-\xi^{2(i-j)}} \\
&=0 \\
if~i=j \\
&~~~~~\langle Y^{i},Y^{j} \rangle \\
&=N
\end{align}

$$

因此，我们可以将A的各种形态表示为如下形式
$A^{T}=\left[\vec{1},Y^{2},\dots,Y^{N-1}\right]^{T}$
$A=\left[\vec{1},Y^{2},\dots,Y^{N-1}\right]$
$A^{H} = \left[\vec{1},\overline{Y^{2}},\dots,\overline{Y^{N-1}}\right]^{T}$

因此
$$
\begin{align}
A^{H}A=\left[\begin{matrix}
\vec{1} \\
(Y^{2})^{H} \\
\dots \\
(Y^{N-1})^{H}
\end{matrix}\right] \left[\vec{1},Y^{2},\dots,Y^{N-1}\right]
\end{align}
$$
考虑到非对角部分，为$\langle Y^{i},Y^{j}\rangle=0,i\ne j$
对角部分则是$\langle Y^{i},Y^{j}\rangle=N,i=j$，因此$A^{H}A=NI$是一个对角矩阵

其中A是由根组成的矩阵，因此，典范嵌入后我们可以将内积表示为如下形式：
$$
\begin{align}
&~~~~~\langle \mathbf{z}_{i},\mathbf{z}_{j} \rangle \\
&=\mathbf{z_{j}}^{H}\mathbf{z_{i}} \\
&=(A\alpha_{j})^{H}A\alpha_{i} \\
&=\alpha_{j}^{H}A^{H}A\alpha_{i} \\
\end{align}
$$

因此，我们有

$$
\begin{align}
&~~~~~\langle \mathbf{z}_{i},\mathbf{z}_{j} \rangle \\
&=N\langle \alpha^{i},\alpha^{j}\rangle
\end{align}
$$
因此，在向量上，我们得到了正交的向量

### Delta
编码时用于保持精度
举一个例子，如果$\Delta=4$，$x=1.4$，那么
$$
x_{approximate}=\frac{\lfloor x \Delta \rfloor}{\Delta} = 1.5
$$
精度保持在$\frac{1}{\Delta}$

### 解编码过程总结
***最终的编码过程如下：***
- $z \in C^{N/2}$
- $\pi^{-1}(z) \in \mathbb{H}$
- $\Delta \pi^{-1}(z)$
- 投射到$\sigma(R)$中：$\lfloor \Delta \cdot \pi(z) \rceil_{\sigma(R)} \in \sigma(R)$
- 用$\sigma$进行编码：$m(X)=\sigma^{-1}(\lfloor \Delta \cdot \pi(z) \rceil_{\sigma(R)}) \in R$

***最终的解码过程如下：***
$z=\pi \circ \sigma(\Delta^{-1}\cdot m)$

## Encryption & Decryption
[mit CKKS](https://www.mit.edu/~linust/files/CKKS_Homomorphic_Encryption_Part_1.pdf#page=4.56)
### LWE 
[LWE加解密流程及实现](https://zhuanlan.zhihu.com/p/480326595)
LWE问题主要是将$(a_{i},b_{i})=(a_{i},\langle a_{i},s \rangle+e_{i})$从真随机的$\mathbb{Z}_{q}^{n}\times \mathbb{Z}_{q}$中区分开，其中$a_{i},s\in \mathbb{Z}_{q}^{n}$，$a_{i}$是均匀采样的，而$s$则是我们的秘密(secret，一般用作secret key)，$e_{i}\in \mathbb{Z}_{q}$而是随机的小噪声

这是一个困难问题，***如果没有噪声 ($e_i = 0$)： 问题会退化为一个标准的线性方程组***：
$$
\mathbf{a}_i \cdot \mathbf{s} = b_i
$$
在这种情况下，我们只需要足够多的 $(\mathbf{a}_i, b_i)$ 对，就可以使用高斯消元法 (Gaussian elimination) 等标准方法，轻松地求解出秘密向量 $\mathbf{s}$。


**有了噪声 ($e_i \neq 0$)：** 噪声使等式变成了**近似关系**，使得标准的线性代数方法失效。求解这个系统在计算上被认为是**困难的 (Hard)**，因为噪声有效地**混淆**了秘密 $s$ 的信息。


我们将$s$作为私钥，然后发布n对$(a_{i},\langle a_{i},s \rangle+e_{i})$，在这种情况下，这些对子可以被写为矩阵形式$(A, A\cdot s+e),A\in \mathbb{Z}_{q}^{n \times n}, e\in \mathbb{Z}_{q}^{n}$，由于我们很难获取到私钥，因此我们将其这些密钥对作为公钥p，而实际上用的公钥如下所示

$$
p = (-A\cdot s+e, A)
$$

我们的message $\mu\in \mathbb{Z}_{q}^{n}$在LWE的加密即是如下：
$$
c=(\mu,0)+p=(\mu-A\cdot s+e,A)=(c_{0},c_{1})
$$

而我们的解密则是如下所示：
$$
\tilde{\mu}=c_{0}+c_{1}\cdot s=\mu-A\cdot s+e+A\cdot s=\mu+e\approx \mu
$$

LWE最大的问题是效率低

### RLWE
与LWE在$\mathbb{Z}_{q}^{n}$上工作不同，RLWE在$\mathbb{Z}_{q}[X]/(X^{N}+1)$上工作
我们有$a,s,e\in \mathbb{Z}_{q}[X]/(X^{N}+1)$，其中，$a$是均匀采样，$s$是一个小的秘密多项式，$e$是一个小的噪声多项式
$v$为随机向量，$ct = [c_{0},c_{1}]$，$sk=[1,s]$，$m$表示信息，公钥$pk=[-as+e, a]$，其中$b=-as +e$

$$
\begin{aligned}
c&=[v\cdot pk +(m+e_{0},e_{1})]_{q} \\
c_{0}&=[v\cdot b+m+e_{0}]_{q} \\
c_{1} &= [v\cdot a+e_{1}]_{q} \\
\end{aligned}
$$

解密：
$$
\begin{aligned}
m &\approx[<ct,sk>]_{q} \\
  &=[v(-a\cdot s + e)+m+e_{0}+v\cdot a\cdot s + e_{1}s]_{q} \\
  &=[m+ve+e_{0}+e_{1}s]_{q}
\end{aligned}
$$

我们令$ve+e_{0}+e_{1}s=e$
则$\langle ct, sk \rangle = m+e+qr$，根据mit讲义，$r$是非常小的

### CKKS加密步骤
ckks的加密实际上就是上文提到的RLWE加密步骤，但是有一些细节需要表述清楚

$\mathcal{D}G(\sigma^{2})$：从$Z^{N}$空间中生成的向量随机采样，其每个坐标系数均从方差为$\sigma^{2}$的离散高斯分布中独立抽取

$\mathcal{H}WT(h)$：对于一个正整数$h$，$\mathcal{H}WT(h)$是$\{0,\pm 1\}^{N}$中汉明权重恰好为h的带符号二进制向量集合

> [!note] 什么是汉明权重(Hamming weight)？
> 指的是一个符号串中非零符号的个数，对于二进制数据位串，即串中1的个数

$\mathcal{Z}O(\rho)$：对于$\rho \in [0,1]$，$\mathcal{Z}O(\rho)$从集合$\{0,\pm 1\}^{N}$中抽取向量中的每一个元素，其中-1和+1的概率各为$\rho/2$，而取值为0的概率则是$1-\rho$

$q_{l}=q^{l}\cdot q_{0}$，$0<l\le L$

$\lambda$：安全参数，对于每一个$\lambda$，我们选择一个与$\lambda$和$q_{L}$相关的$M=M(\lambda,q_{L})$作为分圆多项式的$M$

> [!warning] 
> 注意，以上函数采样的是多项式的系数，但是生成结果均表示多项式


#### 密钥生成$KeyGen()$
生成一个私钥$sk$，一个公钥$pk$，以及一个评估密钥$evk$
> [!note] 评估密钥的作用
> - 执行重线性化
> - 执行密文旋转

第一步，给定$\lambda$，生成2的幂次$M=M(\lambda,q_{L})$，整数$h=h(\lambda,q_{L})$，整数$P=P(\lambda,q_{L})$以及一个实数$\sigma=\sigma(\lambda,q_{L})$

> [!question] $q_{L}$是什么？
> $$q_L = p_0 \cdot p_1 \cdot p_2 \cdots p_L$$
> **计算预算：** $q_L$ 的大小决定了密文在解密失败前可以进行多少次乘法运算（Rescaling 或 Modulus Switching）。随着同态计算的进行，模数会逐渐从 $q_L$ 减小到 $q_{L-1}, \dots, q_0$。

第二步，采样$s,a,e$，生成私钥与公钥。$s \leftarrow \mathcal{H}WT(h)$，$a \leftarrow \mathcal{R}_{q_{L}}$以及$e \leftarrow \mathcal{D}G(\sigma^{2})$。私钥$sk \leftarrow (1,s)$，公钥$pk \leftarrow (b,a)\in \mathcal{R}_{q_{L}}^{2}$，其中$b \leftarrow -as+e(mod q_{L})$

>[!question] $R_{q_{L}}$是什么？
>$$\mathcal{R}_{q_L} = (\mathbb{Z}_{q_L}[X]) / (X^N + 1)$$

第三步，采样$a^{\prime},e^{\prime}$，生成$evk$。$a^{\prime}\leftarrow \mathcal{R}_{P\cdot q_{L}}$，$e^{\prime}\leftarrow \mathcal{D}G(\sigma^{2})$。而$evk \leftarrow (b^{\prime},a^{\prime})\in \mathcal{R}_{P\cdot q_{L}}^{2}$，其中$b^{\prime}\leftarrow -a^{\prime}s+e^{\prime}+Ps^{2}(mod~P\cdot q_{L})$

#### 编码
见解码与编码部分

#### 加密$Enc_{pk}(m)$
采样$v \leftarrow \mathcal{Z}O(0.5)$与$e_{0},e_{1}\leftarrow \mathcal{D}G(\sigma^{2})$，输出$v\cdot pk+(m+e_{0},e_{1}) (mod~q_{L})$（注意，$v\cdot pk$输出的是类似$(a,b)$的形式）

#### 解密
见[[#RLWE]]部分

# CKKS各个计算的详细步骤
## Add

## Rescaling
[CKKS explained, Part 5: Rescaling – OpenMined](https://openmined.org/blog/ckks-explained-part-5-rescaling/)
![[Pasted image 20251107195021.png]]
### Rescale 是在做什么？

在 CKKS 里，密文大致可以理解成“它解密后会得到一个被放大了 `scale=Δ` 倍的明文”：

$$
\text{Dec}(c) \approx \Delta m + e \pmod q
$$

做一次乘法后，scale 会从 $Δ$ 变成 $Δ^2$：

$$
\text{Dec}(c_{\text{mul}}) \approx \Delta^2 m + e' \pmod q
$$

这时候数值“放大过头了”，所以要做 **rescale**：把密文系数大致除以 `Δ`，让 scale 从 `Δ^2` 降回接近 `Δ`，便于继续后面的计算。形式上可以理解为：

$$
c' \approx \left\lfloor \frac{c}{\Delta} \right\rceil,\qquad q' \approx \frac{q}{\Delta}
$$

也就是说，**rescale 不是只改 scale 的标签，而是真的把密文系数和它所在的模数层级一起缩小**。

### 为什么模数也要一起除以 $Δ$？

#### 1. 为了让结果“良定义”（well-defined）

密文本来是 **模 $q$ 的对象**。同一个元素可以写成很多不同的整数代表元，比如 $x$ 和 $x+q$ 表示的是同一个模 $q$ 元素。  
如果你只把“代表元”除以 $Δ$，那不同代表元除完、再四舍五入后，可能得到不同结果；这样“rescale”就不再是一个对模类本身良定义的操作。

但如果同时把模数也缩小到 $q/Δ$，那么这些不同代表元之间原本相差的一个 $q$，除以 $Δ$ 后就正好变成一个 $q/Δ$ 的倍数，于是它们在新模数下仍然代表同一个元素。  
**所以：同时缩小密文和模数，才能保证 rescale 的输出是确定的。**
![[Pasted image 20260318163807.png]]
![[Pasted image 20260318165053.png]]
rescale实际上是一个将一个商环变成另一个商环的映射
#### 2. 为了避免用模逆元时把误差放大

如果不降模数，而是在原模数 $q$ 下硬要“除以 $Δ$”，最自然的做法其实是乘上模逆元 $Δ^{-1} \bmod q$。  
但问题是：这个 $Δ^{-1}$ 在模 $q$ 里通常并不是一个很小的数，它可能非常大。这样一来，误差项也会被乘上它：

$$
e \mapsto e \cdot \Delta^{-1} \pmod q
$$

这不会像普通实数除法那样把误差缩小，反而可能让误差变得很大、很乱，影响后续解密精度。  
而 rescale 采用的是“**整数意义下缩小并舍入，同时切到更小模数**”的做法，这样误差通常是随整体一起按比例下降的，只额外引入一小部分舍入误差。

### 具体实现
参考[[Rescale 计算技术]]

## Rotaion
[Rotating CKKS Ciphertexts \| crysec.dev](https://crysec.dev/2023/01/24/rotating-ckks-ciphertexts.html)
Rotation主要做到是两个工作，一个是自同构(automorphism)，另外一个则是密钥切换(key switch)，key switch的详细内容将会在下一章介绍。

### 自同构基础知识
一个自同构通俗来说可以理解为一个映射，它将一个代数结构映射到自己，同时运算规则保持不变。具体来说，运算规则保持不变有如下性质（环同态）：
$$
\begin{align}
f(a+b) = f(a)+f(b) \\
f(ab) = f(a)f(b)
\end{align}
$$

在CKKS中，我们是在$\mathbb{Z}_{q}[X]/(X^{N}+1)$这个环下进行运算的，这个环下，我们可以定义一个同构：
$$
\kappa_{k}: m(X)\mapsto m(X^{k}),gcd(k,2N)=1
$$
其中，$m(X)$表示的是一个多项式

说明这是一个同构，需要从两个方面入手，一方面需要证明映射后的结果仍在同一个环中，另一方面需要证明映射本身需要满足同态性。更严格来说，还需要证明良定义和双射。

***同一个环证明***
 在商环 $\mathbb{Z}_{q}[X]/(X^N+1)$ 中，由于 $X^N+1\equiv 0$，所以有
$$
 X^N\equiv -1.
$$
因此任意次数大于等于 $N$ 的多项式，都可以递归地利用这个关系化简为次数小于 $N$ 的多项式。这正是商环的典型特征：我们把相差一个理想中元素的两个多项式视为同一个同余类。

因此，当我们将$m(X)=1+X+X^{2}\dots+X^{N-1}$中的$X$替代为$X^{k}$时，我们通过$X^{n}+1$的规约，可以将其重新变回到原先的多项式范围中，因此映射是在同一个环内发生的

> [!NOTE] 
>这和 $\mathbb{Z}/n\mathbb{Z}$ 的情形很类似。$\mathbb{Z}/n\mathbb{Z}$ 是把所有整数按“相差 $n$ 的倍数”分成同余类；也就是说，当$$a\equiv b\pmod n$$时，就把 $a$ 和 $b$ 看成商环中的同一个元素。对应地，在 $\mathbb{Z}[X]/(X^N+1)$ 中，如果两个多项式之差是 $(X^N+1)$ 的倍数，就把它们看成同一个元素。
这里的 $n\mathbb{Z}=(n)$ 和 $(X^N+1)$ 都不是单个数或单个多项式，而是理想（也是一种集合）。具体地，$$(n)=\{nk:k\in\mathbb{Z}\},$$ 
$$(X^N+1)=\{(X^N+1)q(X):q(X)\in\mathbb{Z}[X]\}$$
所以 $\mathbb{Z}/n\mathbb{Z}$ 和 $\mathbb{Z}[X]/(X^N+1)$ 都是“按某个理想取商”得到的商环。
不过要注意，$\mathbb{Z}[X]/(X^N+1)$ 一般只是环，不一定是域；同样，$\mathbb{Z}/n\mathbb{Z}$ 只有在 $n$ 为素数时才是域。

***同态性证明***
$\kappa_{k}(m+n)=\kappa_{k}(m)+\kappa_{k}(n)$不难证明，此处证明省略

$\kappa_{k}(mn)=\kappa_{k}(m)\kappa_{k}(n)$：
令$g(X)=m(X)n(X)$，$\kappa_{k}(g(X))=g(X^{k})=m(X^{k})n(X^{k})=\kappa_{k}(m)\kappa_{k}(n)$

### 良定义与双射证明
可以证明，在$gcd(k,2N)=1$等条件下，双射和良定义成立，证明过程略

## 旋转与自同构
为什么需要自同构呢？我研究了很久，发现主要原因是，论文需要一些高级的概念来吹水，所以才会引入自同构这个概念，实际上rotation做的事情很简单。

# 论文内容

附录B详细阐述了误差的估计
![[Pasted image 20251102162829.png]]
![[Pasted image 20251102162846.png]]

![[Pasted image 20251102162856.png]]

[CKKS Part3: CKKS的加密和解密 - PamShao - 博客园](https://www.cnblogs.com/pam-sh/p/15864191.html)

rescale：[CKKS Part5: CKKS的重缩放 - PamShao - 博客园](https://www.cnblogs.com/pam-sh/p/15865673.html)

