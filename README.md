# continued-fraction

$$
e = 2 + \frac{1}{1 + \frac{1}{\displaystyle 2+\frac{1}{\displaystyle 1+\frac{1}{\displaystyle 1+\frac{1}{\displaystyle 4+\frac{1}{\displaystyle 1+\frac{1}{\displaystyle 1+\frac{1}{\displaystyle 6+\frac{1}{\ddots}}}}}}}}}
$$

## シャニマスの期間限定ガシャで天井する確率はだいたい2/9
シャニマスの限定ガシャで「ピックアップ」されるカードの、ガシャ1回あたりの当選確率は 0.5 % = $1/200$。
「天井」（一定回数ガシャを回すと、引換券と交換できて必ずカードを入手できる）は300回。
したがって、天井までピックアップされるカードを入手できない確率は、

$$
\begin{align}
\left(1-\frac{1}{200}\right)^{300} &= \frac{452817 \cdots (全690桁) \cdots 940001}{203703 \cdots (全691桁) \cdots 000000}\\
&= 0.22229219984074725\cdots.
\end{align}
$$

この値は $2/9 = 0.22222\cdots$ にかなり近い。実際、小数点以下第4位まで一致する。

実用的には「期間限定ガシャ9回中2回は天井する」と覚えておくと良い。

ちなみにこの確率は、「50回目までに少なくとも1回出る確率」におよそ等しい。「50連以内で出た！」と同じだけ「天井した！」がある。

## e

一般に、1回の試行で確率 $p$ で発生する事象が、$n$ 回の試行（各試行は独立）1回も発生しない確率は

$$
(1-p)^n
$$

である。$p$ が十分小さければ、

$$
\begin{align}
(1-p)^n 
&= \left((1-p)^{1/p}\right)^{pn}\\
&\simeq \left(\lim_{p \rightarrow 0}(1-p)^{1/p}\right)^{pn}\\
&= e^{-pn}
\end{align}
$$

と近似できる。

上の例では、$p = 1/200, n = 300$ であった。代入すると、

$$
e^{-3/2} = 0.2231301601\cdots
$$

となる。厳密な値よりずれが大きくなるが、やはり $2/9$ に近い。

ところで、$e^x$ のテイラー展開は、

$$
\begin{align}
e^x &= \sum_{n=0}^\infty \frac{x^n}{n!} \\
&= 1 + x + \frac{x^2}{2} + \frac{x^3}{6} +  \frac{x^4}{24} +\cdots
\end{align}
$$

である。$n$ 次の項までで打ち切った多項式 ($=f_n(x)$とする) に、$x = -3/2$ を代入した値を見よう。

| $n$ | $f_n(x)$ (分数) | $f_n(x)$ (小数) |
|:---:|:---:|:---|
| $0$ | $1$ | $1$ |
| $1$ | $-1/2$ | $-0.5$ |
| $2$ | $5/8$ | $0.625$ |
| $3$ | $1/16$ | $0.0625$ |
| $4$ | $35/128$ | $0.2734375$ |
| $5$ | $269/1280$ | $0.21015625$ |
| $6$ | $1157/5120$ | $0.2259765625$ |
| $7$ | $3191/14336$ | $0.2225864955357\cdots$ |
| $8$ | $256009/1146880$ | $0.2232221330915\cdots$ |
| $9$ | $102355/458752$ | $0.22311619349888392\cdots$ |
| $10$ | $10236229/45875200$ | $0.22313208443777902\cdots$ |

これを見ると、テイラー近似によって真の値に近づく様子が分かる。しかし、近似値がなす列の中に $2/9$ という「そこそこ良い近似」が現れない。$2/9$ という有理数を、$e^{-3/2}$ の近似値として何らかの方法で特徴付けられないだろうか？

## 連分数

答え：連分数

$$
e^{-3/2} = \frac{1}{\displaystyle 4+\frac{1}{\displaystyle 2+\frac{1}{\displaystyle 13+\frac{1}{\displaystyle 6+\frac{1}{\displaystyle 1+\frac{1}{\displaystyle 1+\frac{1}{\displaystyle 6+\frac{1}{\ddots}}}}}}}}
$$

| $n$ |第 $n$ 近似分数|小数|
|:---:|:---:|:---|
| $1$ |  $1/4$ | $0.25$ |
| $2$ |  $2/9$ | $0.2222222222222222$ |
| $3$ |  $27/121$ | $0.22314049586776857$ |
| $4$ |  $164/735$ | $0.22312925170068026$ |
| $5$ |  $191/856$ | $0.2231308411214953$ |
| $6$ |  $355/1591$ | $0.22313010685103707$ |
| $7$ |  $2321/10402$ | $0.22313016727552393$ |
| $8$ |  $2676/11993$ | $0.22313015925956806$ |
| $9$ |  $18377/82360$ | $0.22313016027197669$ |
| $10$ |  $21053/94353$ | $0.22313016014329168$ |


## 再びテイラー展開

次の関数を考える:

$$
f(x) := \left( 1 - x \right) ^ {\alpha / x}.
$$

上のケースは $x = 1/200, \alpha = 3/2$。

$$
\begin{align}
f(x) &= \exp\left( \frac{\alpha}{x} \log \left( 1 - x \right) \right) \\
&= \exp\left( 
  \frac{\alpha}{x} 
  \left(
    -x
    -\frac{x^2}{2}
    -\frac{x^3}{3}
    -\cdots
  \right) 
\right)\\
&= \exp(-\alpha)
\exp\left(-\frac{\alpha x}{2}\right)
\exp\left(-\frac{\alpha x^2}{3}\right)
\cdots\\
&= e^{-\alpha}
\left( 1 - \frac{\alpha x}{2} + \frac{\alpha^2 x^2}{8} - \frac{\alpha^3x^3}{48} + \cdots\right)\\
&\phantom{=e^{-\alpha}}
\left( 1 - \frac{\alpha x^2}{3} +\cdots \right) \\
&\phantom{=e^{-\alpha}}
\left( 1 - \frac{\alpha x^3}{4} +\cdots \right) \\
&= e^{-\alpha} \left(1 
-\frac{\alpha}{2} x
+\frac{\alpha(3\alpha-8)}{24} x^2
-\frac{\alpha(\alpha^2-8\alpha+12)}{48} x^3
+\cdots\right)
\end{align}
$$

だいたい、
$$
\left(1-\frac{1}{200}\right) ^ {300}
\simeq \left( 1 - \frac{3}{800} \right) e^{-3/2}.
$$

$$
\begin{align}
(左辺) &= 0.2222921998 \cdots \\
(右辺) &= 0.2222934220 \cdots
\\
\end{align}
$$

## Links
- Wikipedia https://en.wikipedia.org/wiki/Continued_fraction
- OEIS A001203 https://oeis.org/A001203
  - $\pi$ の連分数展開
- OEIS A003417 https://oeis.org/A003417
  - リンクに有用な資料多数あり。
- Finding the continued fraction of e^(l/m) http://www.numbertheory.org/php/davison.html
  - C言語によるプログラムが公開されている。
- ArXiv
  - "Variations on an error sum function for the convergents of some powers of $e$" https://arxiv.org/abs/1408.2206
  - "How to obtain the continued fraction convergents of the number $e$ by neglecting integrals" https://arxiv.org/abs/1005.2951
  - "A short proof of the simple continued fraction expansion of $e$" https://arxiv.org/abs/math/0601660
  - "Rational approximations of the exponential function at rational points" https://arxiv.org/abs/1609.07076
  - "Another Proof of $e^{x/y}$ being irrational" https://arxiv.org/abs/2104.06263
  - "A simple continued fraction expansion for $e^n$" https://arxiv.org/abs/1909.13597
- "Continued Fractions : Old and New (Natural extension of arithmetic algorithms and S-adic system)" https://repository.kulib.kyoto-u.ac.jp/dspace/handle/2433/243575
- "ネイピア数eの連分数展開について" http://ja9nfo.web.fc2.com/math/202108ContiFracExpOfE.pdf
 
- 未閲覧
  - J. H. McCabe, ‘On the Pad´e table for $e^x$ and the simple continued fractions for $e$ and $e^{L/M}$ ’, Ramanujan J. 19 (2009) 95–105.
    - https://www.researchgate.net/publication/237531137_On_the_Pad_table_for_e_x_and_the_simple_continued_fractions_for_e_and_e_LM
