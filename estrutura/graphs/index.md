---
title: Grafos
author: Pedro Correa
output: html_document
---

# Motivação

Ideal para modelar fatos do mundo real em que os relacionamentos possuem maior importância ou aparecem em grande quantidade, podendo até representar cenários dinâmicos.

***

# Árvore Geradora Mínima (MST - Minimum Spanning Tree)

É uma árvore que mora dentro do grafo, sendo que esta árvore deverá atingir todos os vértices da árvore.
Essa árvore deverá conter o caminho que possui o menor peso da árvore.
Diferente do modo de busca de largura da árvore, o foco é no peso das arestas, algo que não é utilizado na busca da largura.

Mas a MST tem algumas propriedades que devem ser prestadas atenção para saber se um grafo pode ser MST:

* O problema possui solução apenas se o grafo for **conexo**.
* Não possui nenhum ciclo.
* Grafo não-direcionado
* Grafo ponderado

*Obs*: se todas as arestas tiverem o mesmo custo então toda árvore geradora será MST.

***

Bom, mas como podemos encontrá-la?

Podemos fazer por força bruta, assim testaremos todas as combinações possíveis e escolhemos a que der a menor soma.
Mas acabamos encontrando problemas pois a implementação deste modo é difícil e muito custosa, pois o grafo pode ter *n* árvores geradores dentro de si.

Podemos encontrar o MST de maneira linear utilizando as propriedades. 

### Links

[Visão rápida do que é MST](https://www.youtube.com/watch?v=eHC2tjQPX3A)

[Apresentação que não tive paciência pra ver](https://www.youtube.com/watch?v=tCheLd4H-nM&t=304s)

[Aula do MIT (em inglês)](https://www.youtube.com/watch?v=tKwnms5iRBU)

[Apresentação de MST](http://www.dt.fee.unicamp.br/~ricfow/IA881/arvoreGeradora.pdf)

[MST segundo a USP](https://www.ime.usp.br/~pf/algoritmos_para_grafos/aulas/spanningtrees.html#exchange1)

[Apresentação de MST da USP](https://www.ime.usp.br/~coelho/mac0328-2011/aulas/aula20.pdf)

[Conceitos básicos de grafo](https://www.youtube.com/watch?v=MC0u4f334mI)

Algorítimos para pesquisar: *Prim* e *Kruskal*
