---
title: Autômatos Finito com Movimentos Vazios
---

Um autômato finito com movimento vazio é uma transição de estados sem a leitura de um símbolo da fita.
Para facilitar vamos utilizar a seguinte abreviação:

> Autômato finito não determinístico + vazio AFN$\varepsilon$

A facilidade de movimentos vazios não aumenta o poder de reconhecimento de linguagens, qualquer AFN$\varepsilon$ pode ser simulado por um AFN

> **AFN$\varepsilon$ pode ser simulado por um AFN**

Um AFN$\varepsilon$ é um quíntupla ordenada

$$M = (\Sigma, Q, \delta, q0, F)$$

* $\Sigma$ é um alfabeto (finito e não-vazio) de entrada;
* Q é um conjunto finito de estados possíveis;
* $\delta$ função programa ou função de transição;
* q0 é o elemento que representa o estado inicial;
* F é um subconjunto de Q, conjunto de estados finais

### Equivalência entre AFD's e AFN's

Para cada AFN existe um AFD que lhe é equivalente, isto é, que reconhece a mesma linguagem.
Contudo, se o AFN tiver n estados, o AFD equivalenete pode ter até **$2^n$ estados**

Mas na maioria dos casos, é possível a construção de um AFD com um número de estados próximo do AFN original.

