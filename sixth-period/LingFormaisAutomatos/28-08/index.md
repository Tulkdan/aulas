---
title: Linguagem e Gramática
---

## Linguagens Livres do Contexto (LLC)

O estudo das linguagens livres do contexto é de fundamental importância, pois compreende a sintaze da maioria das linguagens de programações de propósitos gerais como:

- Pascar;
- C;
- Java;

- **Árvore de derivação**
> - Representa a derivação de uma palavra na forma de árvore
> - Inicia do símbolo inicial como o raiz
> - Termina em símbolos terminais como folhas

- **Gramática ambigua**: se existir pelo menos uma palavra com duas ou mais árvores de derivação

- **Simplificação de gramática**: simplifica as produções sem reduzir o poder de geração da gramática

- **Forma normal**: Restrições rígidas na forma das produções, sem reduzir o poder de geração da gramática

Uma gramática $$G = (V, T, P, S)$$ é dita ser uma GLC se todas as suas produções, em P, são de forma:

* V - Variáveis gramaticais
* T - Alfabeto terminal
* P - Regras de produção
* S - Simbolo inicial

> A -> a

onde:

- A é uma variável de v
- a, uma palavra de (V u T)

## GLC - Duplo balanceamento

Os exemplos anteriores, representam um caso clássico de fundamental importância no estudo de computação e informática,
pois permite estabelecer analogia com estruturas de duplo balanceamento em linguagens de programação

Uma GLC é uma gramática ambigua se existe pelo menos uma palavra com *duas ou mais derivações, a esquerda ou mais a direita*
