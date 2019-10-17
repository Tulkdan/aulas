---
title: Linguagens Regulares
---

O estudo das linguagen regulares é abordado usando os seguintes formalistmos:

- Autômato finito: é um sistema de estados finitos
- Expressão regular: é definida a partir de uma linguagem básca e das operações de concatenação e união
- Gramática regular: representa uma gramática com restrições na formaçõa das regras de produção

### Sistema de Estados Finitos

É a definição de um modelo matemático de sistema com entradas e saídas.
Somente pode assumir um número finito e predefinidio de estados.
Cada estado resume as informações do passado, neessárias para determinar as ações para a pŕoxima entrada.

### Autômatos Finitos

É um sistema de estados finitos, o qual contitui um modelo computacional do tipo sequencial muito comum em diversas áreas da computação, destacando-se em:

* linguagens formais
* compiladores
* semântica formal
* modelos para concorrência

Autômatos finitos são elementos essenciais para o estudo da computação e constituem um modelo útil na elaboração de varios tipos de software.
As principais aplicações dos Autômatos Finitos são:

- Controle de comportamento de circuitos digitais
- Libras - estruturar a língua portuguesa para a língua utilizada pelos surdos

Os autômatos finitos apresentam um formalismo operacional ou reconhecedor conforme apresentado abaixo:

* **determinístico**: para o estado corrente e o símbolo de entrada lido, o sistema assume um único estado válido
* **não determinístico**: para o estado corrente e o símbolo de entrada lido, o sistema assume um estado pertencente a um conjunto de estados alternativos
* **com movimentos vazios**: é um tipo de AFND onde pode haver uma transição de estados sem que haja leitura de símbolos

Em termos de poder computacional, podemos afirmar que o três modelos apresentados são equivalentes.

##### Autômatos finitos Determinísticos

É uma máquina contituida, basicamenete, de trẽs partes:

1. Fita: Dispositivo de entrada, qual contém a informação a ser processada
2. Unidade de Controle: reflete o estado corrente da máquina
3. Programa: função de transição

$$ M = (\Sigma, Q, \delta, q, F) $$

**Estados** são nodos, respresentados por círculos, lembrando que por convenção, utilizaremos o nome dos estados sempre representado pela letra q, e serão diferenciados por um índice.

**Transições** são arestas, ligando os correspondentes nodos.
Seão representados por uma seta com um único elemento do alfabeto

**Estado Inicial** denota o ponto de partida do autômato.
Será representado por uma seta que denota o ponto inicial, e que não representa uma transição de estados.

**Transições paralelas** representam duas transições diferentes, partindo de uma mesma origem e chegando a um mesmo destino

Um autômato finito sempre para ao processar qualquer entrada pois, como qualquer palavra é finita, e como um novo símbolo de entrada é lido a cada aplicação da função programa, não existe possibilidade de loop.

A parada de processamento de autômato finito para uma entrada w pode ser de duas maneiras:

1. **Aceita a entrada w**, após processa o último símbolo da fita, o autômato finito assume um estado final.
2. **Rejeita a entrada w**, são duas possibilidades:
    + 2.1 após processa o último símbolo da fita, o autômato finito assume um estado não final
    + 2.2 Em algum momento, ao longo do processamento de w, a função programa é indefinida para o argumento

$\delta$ | a  | b
-------- | -- | -
q0       | q1 | q2
