---
title: Autômatos Finito Não Determinístico (AFN)
---

O que é:

* não há transição $\epsilon$
* para cada estado "s", dado um símbolo de entrada "a", existe no máximo, um destino de alcance após a transição

### Conceito Inicial

O conceito de não determinismo de um autômato significa que

* a função de transição pode conduzir a múltiplos estados, ou mesmo a nenhum estado
* podem ocorrer transições por $\epsilon$

### Definição

Um AFN é um quíntupla ordenada

$$M = (\sigma, Q, \delta, q0, F)$$

* $\sigma$ é um alfabeto (finito e não vazio) de entrada
* Q é um conjunto finito de estados possíveis
* $\delta$ é uma função de transição
* q0 é o elemento que representa o estado inicial
* F é um subconjunto de Q, denominado conjunto de estados finais

### Linguagem de um AFN

A linguagem de um AFN $M = (\sigma, Q, \delta, q0, F)$ é definida por L(M):

$$L(M) = {w | {\delta * (q0, w)} \cap f \ne \phi}$$

No caso de um AFN, uma palavra w é reconhecida quando o conjunto $\delta(q0, w)$ contém pelo menos um dos estados de aceitação

### Estado de parada

A parada de um AFN para uma entrada w pode ser de duas maneiras:

1. Aceita a entrada w. Após processar o último símbolo da fita, existe pelo mens um estado final pertencente ao conjunto de estados alternativos atingidos
2. Rejeita a entrada w. São duas alternativas
  + após processar o último símbolo da fita, todos os estados alternativs atingidos são não finais
  + em algum momento, ao longo do processamento de w, o conjunto de estados alternativos é vazio. Neste caso o autômato para por indefinição

##### Aceitação

A linguagem ACEITA ou LINGUAGEM RECONHECIDA POR M, é denotada por:

> ACEITA(M) ou L(M)

É o conjunto de todas as palavras pertencentes a $\sigma$ tais que existe pelo menos um caminho alternativo que aceita a palavra, a partir de ${q0}$, ou seja:

$$L(M) = {w | \delta(q0, w) \cap F \phi}$$

##### Rejeição

A linguagem REJEITA, é denotada por:

> REJEITA(M)

É o conjunto e todas as palavras pertencentes a $\sigma$ rejitadas por todos os caminhos alternativos de M, a partir de ${q0}$, ou seja:

$$L(M) = {w | \delta(q0, w) \cap F \phi}$$

### Equivalência entre AFD e AFN

Para cada AFN existe um AFD que lhe é equivalente, isto é, que reconhece a mesma linguagem.
Contudo, se o AFN tiver n estados, o AFD equivalente pode ter até $2^n$ estados no pior caso.

Mas na maioria dos casos, é possível aconstrução de um AFD com um número de estados próximo do AFN original, no entanto com uma quantidade maior de transições

###### Exemplo

$$L5 = {w | w possui aa ou bb como subpalavra}$$

Portanto podemos afirmar que a palavra **abaa** é ACEITA conforme demonstrado abaixo:

$${q0, q1, q2} \cap F = {q1} \ne \phi$$
