# シャニマスの期間限定ガシャで天井する確率がおよそ2/9であること、あるいは連分数について

## 確率論

シャニマスの期間限定ガシャでピックアップされるアイドルの提供割合は通例 $0.500\% = 1/200 $で、「天井」は300回。

したがって、天井まで目当てのアイドルを入手できない確率は、

$$
\begin{align}
\left(1-\frac{1}{200}\right)^{300} &= \frac{\overbrace{452817 \cdots 940001}^{\text 690桁}}{\underbrace{203703  \cdots 000000}_{\text 691桁}}\\
&= 0.2222921998\cdots
\end{align}
$$

と計算できる。

厳密には「10回引く」でしかスタンプがたまらないため、295回目で出たとしてもそれは既に天井に達しているのだが、話を単純にするためその効果は無視する。

さて、この値は $2/9 = 0.22222\cdots$ にかなり近い。実際、小数点以下第4位まで一致する。

実用的には「期間限定ガシャ9回中2回は天井する」と考えておくと覚えやすい。放クラ（5人）とノクチル（4人）のpSSRは絶対に引くと決めている場合、杜野凛世と福丸小糸で天井するということになる、というのは不正確な表現だが、まあ割合としてはそうなる。

ちなみにこの確率は、期間限定ガシャで「50回目までに少なくとも1回出る確率」におよそ等しい:

$$
1-\left(1-\frac{1}{200}\right)^{50} = 0.2216874429\cdots
$$

「50連以内に出た！」と同じだけ「天井した！」がある。何となく経験に合う感がある。

## 自然対数

一般に、1回の試行で確率 $p$ で発生する事象が、$n$ 回の試行（各試行は独立）の中で1回も発生しない確率は

$$
(1-p)^n
$$

である。この値は $p$ が十分小さければ、

$$
\begin{align}
(1-p)^n 
&= \left((1-p)^{1/p}\right)^{pn}\\
&\simeq \left(\lim_{p \rightarrow 0}(1-p)^{1/p}\right)^{pn}\\
&= e^{-pn}
\end{align}
$$

と近似できる。

上述のシャニマスの期間限定ガシャで天井する確率の例では、$p = 1/200,\ n = 300$ であった。代入すると、

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

で、収束半径は $\infty$ である。$n$ 次の項までで打ち切った多項式 ($=f_n(x)$とする) に、$x = -3/2$ を代入した値を見よう。

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

これを見ると、テイラー近似によって真の値 $e^{-3/2}$ に近づく様子が分かる。しかし、近似値がなす列の中に $2/9$ という「そこそこ良い近似」が現れない。$2/9$ という有理数を、$e^{-3/2}$ の近似値として何らかの方法で特徴付けられないだろうか？

## 連分数

答え：連分数。

$e^{-3/2}$ の（正則）連分数展開は、以下のように与えられる：

$$
\begin{align}
e^{-3/2} &= \frac{1}{\displaystyle 4+\frac{1}{\displaystyle 2+\frac{1}{\displaystyle 13+\frac{1}{\displaystyle 6+\frac{1}{\displaystyle 1+\frac{1}{\displaystyle 1+\frac{1}{\displaystyle 6+\frac{1}{\ddots}}}}}}}} \\
&=\lbrack 0; 4, 2,13,6,1,1,6, \cdots \rbrack
\end{align}
$$

`;` 以下 $n$ 項目までで打ち切った第 $n$ 次近似分数を順に計算すると、以下の表が得られる：


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

つまり、$2/9$ は $e^{-3/2}$ の連分数展開の第2近似分数だということが分かった。

一般に、実数 $\alpha$ の連分数展開が

$$
\begin{align}
\alpha
  &= a_0 + \frac{1}{\displaystyle a_1+\frac{1}{\displaystyle a_2+\frac{1}{\ddots}}} \\
  &= \lbrack a_0; a_1, a_2, \cdots \rbrack
\end{align}
$$

で与えられるとする。$a_i$ を部分商と呼ぶ。このとき（すでに使ったが）、第 $n$ 近似分数 $p_n/q_n$ （既約で $p_n$ と $q_n$ は互いに素）は

$$
\frac{p_n}{q_n} = \lbrack a_0; a_1, a_2, \cdots a_n \rbrack
$$

で定義される。

この連分数から得られる近似分数はある意味で $\alpha$ に対する最良の近似を与える。次の事実が知られている(\[Kida\] 命題4.8.)：

任意の整数 $p$、$0<q\leq q_n$ を満たす 任意の整数 $q$ に対して、

$$
\left|\alpha - \frac{p_n}{q_n} \right| \leq
\left|\alpha - \frac{p}{q} \right|.
$$

等号は $p_n/q_n = p/q$ の場合にだけ成立。

つまり、$q_n$ 以下の分母を持つ有理数の中で最も $\alpha$ に近づけるのが $p_n/q_n$ であるということ。

また、このときの差は、

$$
\left|\alpha - \frac{p_n}{q_n} \right|
< \frac{1}{a_{n+1}q_n^2}
$$

によって上から抑えられる(\[Kida\]補注4.10.)。

上の例で言うと、

$$
\left|e^{-3/2} - \frac{2}{9} \right|
< \frac{1}{13\cdot 9^2} = 0.000949\cdots < 10^{-3}
$$

なので、小数点以下第2位まで一致する事実に整合する。

## exp(p/q) の連分数展開

上で降って湧いてきたかのように出てきた $e^{-3/2}$ の連分数展開だが、任意の精度で求めるアルゴリズムがある。

\[Matthews\]で解説されている[アルゴリズム](http://www.numbertheory.org/php/davison.html)がそれで、
[BCMATHでの実装](http://www.numbertheory.org/gnubc/davison) を Rust で実装し直したものを以下のページで公開している。上で示した連分数展開はこれによって得たもの。

実は $e^{1/k}$ と $e^{2/k}$ ($k$ は整数) の連分数展開は規則的な表式が知られていて、同 \[Matthews\] からの引用だが、

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

となる。特に、$e$ の連分数展開は

$$
e = \lbrack 2; 1, 2, 1, 1, 4, 1, 1, 6, 1, 1, 8,  1,1, 10,\cdots \rbrack
$$

と簡潔に表せる。

一方、その他の $e^{p/q}$ 型の数、特に $e^{3/k}$ 型については類似の表式が知られておらず、実際、今我々が注目している $e^{-3/2}$ も、100項目まで見ても周期のようなものは見えない。

$$
\begin{align*}
    e^{-3/2} = \lbrack
    0;\ & 4,2,13,6,1,1,6,1,6, 1,20,1,87,1,1,2,1,5,1, 2,\\
        & 9,1,7,1,1,61,1,9,4, 2,2,1,2,2,5,2,1,2,1, 1,\\
        & 30,2,78,5,1,1,5,1,12, 2,1,4,2,3,1,2,1,1,1, 4,\\
        & 9,1,1,3,2,1,24,1,5, 1,3,5,6,3,9,4,8,1,2, 1,\\
        & 2,2,1,2,3,1,1,3,1, 8,4,1,41,1,2,13,5,1,55,2\rbrack
\end{align*}
$$





## 期待値
話をガシャに戻して、「天井」があるとき、目当てのアイドルを入手するまでにガシャを引く回数の期待値を考える。

$k$ 回目で初めて出る確率 $P(k)$ は、1回あたりの提供確率 $p$ を使って

$$
P(k) = 
    \left\{
    \begin{array}{cc}
        p(1-p)^{k-1}    & 1 \leq k \leq n-1,   \\
        (1-p)^{n}       & k = n,              \\
        0               & k > n
    \end{array}
    \right.
$$

と表せる。回数の期待値は、

$$
\begin{align}
E   &= \sum_{k=1}^\infty k P(k) \\
    &= \frac{1}{p} \left(1-\left(1-p\right)^n\right) \\
    &= \frac{1}{p} \left(1-P_{\text 天}\right)
\end{align}
$$

と綺麗な形になる。ここで天井に達する確率を$P(n) = P_{\text 天}$と置いた。

ちなみに $n \rightarrow n+1$ と変えたときの差分を考えるとこの和は計算しやすいのだが、より簡明な説明は知らない。

シャニマスの期間限定ガシャの場合、$P_{\text 天}\simeq 2/9$ だから、

$$
E \simeq 200 (1 - 2/9) = 155.55\cdots
$$

から、約156回が期待値となる。上の式で $n\rightarrow \infty$ （天井がない場合）とすると $E=1/p=200$ となるので、天井があることによって「天井に達する確率」分だけ、比で回数の期待値が抑えられると見ることができる。

定性的には納得できるが、厳密な結果としては、$P(k)$ が不連続であることを考えると結構不思議な気がする。

## 再びテイラー展開
$e^{-3/2}$ と $(1-1/200)^{300}$ を同一視しつつ考えてきたが、両者はどれくらい異なるだろうか。

次の関数を考える:

$$
f(x) := \left( 1 - x \right) ^ {\alpha / x}.
$$

今考えているケースでは $\alpha=3/2,\ x=1/200$。

$f(x)$ の $x=0$ の周りでのテイラー展開は、

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

と計算できる。1次の項で打ち切って、だいたい、

$$
\left(1-\frac{1}{200}\right) ^ {300}
\simeq \left( 1 - \frac{3}{800} \right) e^{-3/2}.
$$

になるということ。実際に代入すると、

$$
\begin{align}
({\text 左辺}) &= 0.2222921998 \cdots \\
({\text 右辺}) &= 0.2222934220 \cdots
\\
\end{align}
$$

となって、小数点以下第5位まで一致する。



## まとめ
- シャニマスの期間限定ガシャで天井する確率は大体 $e^{-3/2} \simeq 2/9$。
- ある実数がある有理数に近いとき、両者は連分数展開によって結び付けられる場合がある。
- $\exp(p/q)$ の連分数展開は Davison のアルゴリズムによって与えられる。



## 参考文献
- \[Matthews\] Keith Matthews, "Finding the continued fraction of $e^{p/q}$", [http://www.numbertheory.org/php/davison.html](http://www.numbertheory.org/php/davison.html)
- \[Kida\] 木田雅成、『大学数学 スポットライト・シリーズ9 連分数』、近代科学社、2023年、[https://www.kindaikagaku.co.jp/book_list/detail/9784764906433/](https://www.kindaikagaku.co.jp/book_list/detail/9784764906433/)