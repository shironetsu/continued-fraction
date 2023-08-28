# continued-fraction

$$
\begin{align*}
e = 2 + \frac{1}{\displaystyle 1 + \frac{1}{\displaystyle 2+\frac{1}{\displaystyle 1+\frac{1}{\displaystyle 1+\frac{1}{\displaystyle 4+\frac{1}{\displaystyle 1+\frac{1}{\displaystyle 1+\frac{1}{\displaystyle 6+\frac{1}{\ddots}}}}}}}}}
\end{align*}
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

ちなみにこの確率は、「50回目までに少なくとも1回出る確率」におよそ等しい。「50連以内で出た！」と同じだけ「天井した！」がある。かなり経験に合う感がある。

## e

一般に、1回の試行で確率 $p$ で発生する事象が、$n$ 回の試行（各試行は独立）の中で1回も発生しない確率は

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
| $7$ | $3191/14336$ | $0.2225864955\cdots$ |
| $8$ | $256009/1146880$ | $0.2232221330\cdots$ |
| $9$ | $102355/458752$ | $0.2231161934\cdots$ |
| $10$ | $10236229/45875200$ | $0.2231320844\cdots$ |

これを見ると、テイラー近似によって真の値に近づく様子が分かる。しかし、近似値がなす列の中に $2/9$ という「そこそこ良い近似」が現れない。$2/9$ という有理数を、$e^{-3/2}$ の近似値として何らかの方法で特徴付けられないだろうか？

## 連分数

答え：連分数

$$
e^{-3/2} = \frac{1}{\displaystyle 4+\frac{1}{\displaystyle 2+\frac{1}{\displaystyle 13+\frac{1}{\displaystyle 6+\frac{1}{\displaystyle 1+\frac{1}{\displaystyle 1+\frac{1}{\displaystyle 6+\frac{1}{\ddots}}}}}}}}
$$

| $n$ |第 $n$ 近似分数|小数|
|:---:|:---:|:---|
| $1$ |  $1/4$ | $0.25$ |
| $2$ |  $2/9$ | $0.2222222222\cdots$ |
| $3$ |  $27/121$ | $0.2231404958\cdots$ |
| $4$ |  $164/735$ | $0.2231292517\cdots$ |
| $5$ |  $191/856$ | $0.2231308411\cdots$ |
| $6$ |  $355/1591$ | $0.2231301068\cdots$ |
| $7$ |  $2321/10402$ | $0.2231301672\cdots$ |
| $8$ |  $2676/11993$ | $0.2231301592\cdots$ |
| $9$ |  $18377/82360$ | $0.2231301593\cdots$ |
| $10$ |  $21053/94353$ | $0.2231301601\cdots$ |

$e^{-3/2}$ に対して、2次の近似分数が $2/9$ であることが分かった。

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
&\hspace{30px}\times
\left( 1 - \frac{\alpha x^2}{3} +\cdots \right)
\left( 1 - \frac{\alpha x^3}{4} +\cdots \right) \cdots \\
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

になるということ。実際に代入すると、

$$
\begin{align}
(左辺) &= 0.2222921998 \cdots \\
(右辺) &= 0.2222934220 \cdots
\\
\end{align}
$$

となって、小数点以下第5位まで一致する。

## exp(L/M) の近似値

一般に、

$$
\begin{align*}
e^{2/k} &= 
\left\{
\begin{array}{ll}
\left\lbrack \overline{1, \frac{k-1}{2}+3\lambda k, 6 + 12\lambda k, \frac{5k-1}{2}+3\lambda k, 1} \right\rbrack_{\lambda\geq 0} & \text{if } k > 1,\  k \text{ is odd. }\\
\left\lbrack 7, \overline{2+3\lambda, 1, 1, 3 + 3\lambda,  18 + 12\lambda } \right\rbrack_{\lambda\geq 0} & \text{if } k = 1.
\end{array}
\right.,\\
e^{1/k} &= 
\left\{
\begin{array}{ll}
\left\lbrack \overline{1, (1+2\lambda) k - 1, 1} \right\rbrack_{\lambda\geq 0} & \text{if } k > 1,\\
\left\lbrack 1, \overline{1, 2+2\lambda, 1} \right\rbrack_{\lambda\geq 0} & \text{if } k = 1.
\end{array}
\right.
\end{align*}
$$

となることが知られている \[[numbertheory.org](http://www.numbertheory.org/php/davison.html)\]。

- $e^1$: https://oeis.org/A003417
- $e^2$: https://oeis.org/A001204
- $e^{1/2}$: https://oeis.org/A058281
- TODO

ところが、$e^3, e^4, e^5 \cdots$ については同様の式が知られていない。

```
e^3 = [20; 11, 1, 2, 4, 3, 1, 5, 1, 2, 16, 1, 1, 16, 2, 13, 14, 4, 6, 2, 1, 1, 2, 2, 2, 3, 5, 1, 3, 1, 1, 68, 7, 5, 1, 4, 2, 1, 1, 1, 1, 1, 1, 7, 3, 1, 6, 1, 2, 5, 4, 7, 2, 1, 3, 2, 2, 1, 2, 1, 4, 1, 1, 13, 1, 1, 2, 1, 1, 1, 1, 3, 7, 11, 18, 54, 1, 2, 2, 2, 1, 1, 6, 2, 2, 46, 2, 189, 1, 24, 1, 8, 13, 4, 1, 1, 1, 3, 1, 1, 1, 1, 1, 2, 7, 5, 4, 3, 8, 1, 1, 1, 2, 8, 1, 1, 1, 1, 3, 1, 3, 2, 1, 2, 6, 3, 15, 4, 1, 1, 77, 1, 2, 492, 1, 1, 48, 4, 6, 2, 7, 1, 9, 2, 1, 3, 6, 1, 15, 1, 4, 7, 1, 1, 1, 4, 1, 3, 1, 34, 3, 5, 16, 4, 1, 6, 4, 1, 1, 9, 2, 67, 3, 3, 3, 110, 1, 1, 1, 2, 1, 1, 4, 27, 6, 2, 11, 1, 3, 4, 1, 8, 4, 1, 1, 13, 9, 1, 2, 1, 2, 1, 19, 2, 4, 3, 24, 1, 5, 3, 1, 739, 1, 62, 1, 1, 2, 1, 5, 1, 2, 5, 1, 1, 6, 1, 1, 23, 1, 3, 1, 2, 3, 35, 1, 2, 2, 1, 3, 2, 3, 4, 1, 9, 2566, 12, 8, 3, 9, 1, 2, 2, 1, 1, 1, 13, 5, 1, 1, 1, 3, 1, 1, 1, 423, 1, 3, 1, 1, 2, 4, 1, 1, 1, 1, 11, 9, 1, 1, 71, 1, 1, 2, 1, 1, 3, 2, 1, 1, 3, 694, 1, 1, 3, 1, 1, 21, 3, 3, 1, 3, 1, 51, 1, 17, 1, 4, 21, 1, 66, 1, 17, 6, 3, 1, 1, 3, 3, 1, 1, 1, ...]

e^(-3/2) = [4, 2, 13, 6, 1, 1, 6, 1, 6, 1, 20, 1, 87, 1, 1, 2, 1, 5, 1, 2, 9, 1, 7, 1, 1, 61, 1, 9, 4, 2, 2, 1, 2, 2, 5, 2, 1, 2, 1, 1, 30, 2, 78, 5, 1, 1, 5, 1, 12, 2, 1, 4, 2, 3, 1, 2, 1, 1, 1, 4, 9, 1, 1, 3, 2, 1, 24, 1, 5, 1, 3, 5, 6, 3, 9, 4, 8, 1, 2, 1, 2, 2, 1, 2, 3, 1, 1, 3, 1, 8, 4, 1, 41, 1, 2, 13, 5, 1, 55, 2, 1, 1, 7, 1, 13, 6, 12, 1, 2, 3, 2, 2, 6, 3, 2, 11, 7, 11, 4, 1, 1, 7423, 1, 2, 1, 1, 3, 1, 1, 10, 1, 1, 6, 4, 3, 1, 1, 1, 2, 2, 3, 7, 1, 2, 2, 1, 2, 1, 2, 1, 1, 2, 1, 169, 1, 1, 1, 1, 1, 10, 1, 2, 1, 11, 3, 12, 68, 1, 2, 2, 8, 5, 8, 2, 1, 3, 1, 1, 3, 2, 1, 1, 6, 1, 2, 1, 2, 21, 1, 2, 6, 1, 1, 1, 1, 15, 2, 1, 1, 18, 1, 1, 1, 37, 1, 1, 1, 2, 6, 7, 1, 1, 1, 9, 3, 5, 9, 2, 2, 10, 1, 3, 1, 2, 1, 1, 2, 3, 6, 1, 24, 2, 1, 1, 5, 1, 40, 1, 2, 4, 1, 2, 1, 1, 4, 2, 3, 21, 2, 4, 1, 11, 1, 2, 2, 8, 31, 1, 1, 3, 5, 1, 3, 1, 1, 2, 25, 1, 5, 19, 1, 4, 3, 3, 2, 1, 2, 1, 7, 2, 1, 7, 1, 3, 1, 21, 1, 1, 2, 2, 9, 1, 6, 1, 2, 1, 3, 1, 9, 1, 4, 2, 1, 1, 7, 1, 4, 2, 1, 3, 5, 2, 3, 4, 1, 1, 1, 2, 1, 3, 5, 3, 1, 3, 1, 2, 1, 1, 31, 2, 3, 2, 118, 2, 1, 8, 3, 2, 1, 3, 1, 2, 4, 2, 1, 2, 1, 244, 2, 25, 1, 2, 4, 2, 3, 4, 1, 2, 6, 2, 1, 2, 1, 2, 3, 1, 4, 3, 2, 2, 2, 1, 1, 47, 1, 3, 17, 12, 1, 5, 6368, 5, 3, 4, 1, 2, 19, 1, 3, 1, 3, 37, 35, 1, 1, 3, 2, 1, 1, 2, 1, 1, 1, 1, 1, 8, 1, ...]
```

```
e^4 = [54; 1, 1, 2, 21, 4, 1, 1, 57, 2, 1, 9, 1, 1, 13, 6, 2, 20, 2, 4, 1, 17, 1, 3, 2, 7, 2, 1, 1, 1, 1, 3, 2, 1, 11, 4, 2, 1, 3, 2, 1, 7, 1, 7, 1, 12, 2, 8, 7, 6, 1, 2, 1, 7, 2, 37, 2, 3, 66, 1, 1, 1, 2, 1, 2, 1, 1, 2, 1, 1, 1, 1, 2, 3, 1, 1, 2, 10, 18, 2, 19, 4, 2, 1, 9, 1, 9, 2, 1, 1, 1, 1, 4, 5, 1, 125, 1, 4, 1, 12, 10, 1, 1, 1, 15, 15, 11, 1, 40, 1, 1, 1, 4, 11, 1, 26, 1, 14, 12, 1, 1, 2, 1, 2, 1, 1, 72, 1, 1, 19, 1, 758, 1, 11, 8, 3, 18, 53, 1, 10, 1, 3, 2, 1, 3, 1, 3, 1, 6, 5, 1, 1, 9, 2, 1, 3, 2, 1, 3, 1, 164, 1, 3, 2, 6, 2, 1, 1, 4, 1, 63, 1, 54, 1, 1, 3, 1, 112, 1, 1, 1, 1, 8, 76, 1, 4, 5, 1, 3, 37, 1, 8, 1, 1, 22, 1, 3, 1, 1, 1, 40, 1, 28, 2, 2, 25, 11, 2, 33, 1, 1, 1, 1, 57, 8, 15, 1, 4, 4, 4, 3, 1, 4, 1, 1, 18, 2, 3, 2, 3, 1, 1, 1, 1, 5, 1, 1, 394, 31, 1, 13, 7, 10, 3, 3, 5, 3, 22, 1, 1, 1, 3, 14, 1, 6, 6, 1, 4, 1, 1, 1, 3, 1, 5, 5, 1, 29, 1, 1, 1, 1, 1, 2, 8, 1, 1, 1, 1, 6, 1, 5, 2, 2, 1, 23, 1, 5, 2, 3, 1, 2, 1, 1, 1, 1, 4, 1, 5, 19, 1, 2, 1, 2, 1, 3, 1, 2, ...]
```

## log で見る

$$
\begin{align*}
\log 9 &= 2.197224577\cdots, \\
\log 2 &= 0.6931471806\cdots.
\end{align*}
$$

差が大体 1.5 。

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

# 一鉢二鉢一鉢二鉢の謎

[exponential function \- 2\.71828\. And then another 1828\. \- Mathematics Stack Exchange](https://math.stackexchange.com/questions/764135/2-71828-and-then-another-1828)

$$
e = 2.7\underline{1828}\underline{1828}45904523\cdots
$$

[A001113 \- OEIS](https://oeis.org/A001113)

なぜ `1828` が2回繰り返して現れるか？

`1828` が循環する場合: 

$$
q = 2.7182818281828 \cdots = 2.7\dot{1}82\dot{8}
= \frac{271801}{9999}
$$

連分数展開を比較する:

$$
\begin{align*}
q &= \lbrack\ 2; 1, 2, 1, 1, 4, 1, 1, 6, 1, 1, 8, 1, 1, 5\ \rbrack,\\
e &= \lbrack\  2; 1, 2, 1, 1, 4, 1, 1, 6, 1, 1, 8, 1, 1, 10, 1, 1, 12, \cdots\ \rbrack,\\
\end{align*}
$$

近似分数:

$$
\begin{gather}
\lbrack\ 2; 1, 2, 1, 1, 4, 1, 1, 6, 1, 1\ \rbrack = \frac{2721}{1001},\\
\lbrack\ 2; 1, 2, 1, 1, 4, 1, 1, 6, 1, 1, 8, 1, 1\ \rbrack = \frac{49171}{18089},\\
\lbrack\ 2; 1, 2, 1, 1, 4, 1, 1, 6, 1, 1, 8, 1, 1, 10\ \rbrack = \frac{517656}{190435}
\end{gather}
$$

どんな「説明」に納得できるか？