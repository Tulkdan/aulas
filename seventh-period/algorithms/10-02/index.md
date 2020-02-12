---
title: Projeto e Analise de Algoritmos
---

### O tempo de execucao depende da grandeza da entrada

Eh necessario definir uma dimensao para a entrada, pois os problemas numericos podem depender, ou da quantidade de elmentos numericos ou no numero de bits que reprsenta o valor numerico da entrada.

Problemas baseados em estrutura de dados podem depender da quantidade de elementos da estrutura, como vertices e/ou arestas em um grafo ou arvore.
Ja problemas baseados em textos podem depender do numero de caracteres ou numeros de palavras.

Crescimento  |   $10^2$   |   $10^3$   |   $10^5$   |
-------------|------------|------------|------------|
*logn*       |      2     |      3     |      5     |
*n*          |   $10^2$   |   $10^3$   |   $10^5$   |
*nlogn*      | $2 * 10^2$ | $3 * 10^3$ | $5 * 10^5$ |
*$n^2$*      |   $10^4$   |   $10^6$   |   $10^14$  |
*$2^n$*      |$1.2 * 10^2$|  $10^301$  | $10^30103$ |


### Classe O

$O(g(n))$ = { $f(n)$: existem constantes positivas c e $n_{0}$ tais que $0 \le f(n) \le cg(n)$, para todo $n \ge n0$ }

Ou seja, se $f(n) \in O(g(n))$ significa que $f(n)$ cresce no maximo tao rapidamente quanto $g(n)$

### Classe $\Omega$ 

$\Omega (g(n))$ = { f(n): existem constantes ponsitivas c e $n_{0}$ tais que $f(n) \ge cg(n)$, para todo $n \ge n_{0}$ }

Ou seja, se $f(n) \in \Omega(g(n))$ significa que $f(n)$ cresce no munimo tao lentamente quanto $g(n)$

### Classe $\Theta$

$\Theta(g(n))$ = { $f(n)$: existem constantes positivas $c_{1}$, $c_{2}$ e $n_{0}$ tais que $0 \le c_{1}g(n) \le f(n) \le c_{2}g(n)$ para todo $n \ge n_{0}$ }

Ou seja, se $f(n) \in \Theta(g(n))$ significa que $f(n)$ crescre tao rapidamente quanto $g(n)$
