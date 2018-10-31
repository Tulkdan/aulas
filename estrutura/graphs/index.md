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

## Algoritmos Gulosos (Greedy Algorithms)

O algoritmo de Prim é também conhecido como **Algoritmos Gulosos**, que são algoritmos conhecidos por fazerem a melhor escolha no momento.
Essa estratégia não oferece a garantia de encontrar uma solução globalmente perfeitas para o problema.
Mas com ela conseguimos provar uma MST.

Em seguida temos um exemplo de um lógica genérica para se encontrar a MST.

```
void GenericoMST() { 
    S = ∅;
    while(S não constitui uma árvore geradora mínima) { 
        (u, v) = seleciona(A);
        if (aresta (u, v) é segura para) 
            S = S + {(u, v )};
    }
    return S ;
}
```

Antes de cada iteração, S é um subconjunto de uma MST.
A cada aresta que verificamos, adicionamos a S a aresta (u,v) que não viola a variante.
Essa aresta é chamada de *aresta segura*.

Dentro do *while*, S tem que fazer parte do subconjunto prórpio da MST T, e quando existir uma aresta (u,v) que pertence a T e que não pertence a S e é considerado uma *aresta segura*, ele é adicionado ao conjunto de S.

## Algoritmo de Prim

Este algoritmo pode ser derivado do algoritmo anterior e a operação é muito semelhante ao algoritmo de Dijkstra para localizar o caminho mais curto em um grafo.
Neste algoritmo, o MST é gerada em um ponto escolhido arbitrariamente, e a partir desde ponto, começamos adicionar as arestas que possuem o valor mínimo que ligam para algum ponto que não esteja conectada a subárvore, ou seja, apenas liga *arestas seguras*.


Os algorítmos de Prim de Kruskal possuem similaridade no tempo de execução que as pontas estão pré-ordenadas pelo peso.

### Links

[Visão rápida do que é MST](https://www.youtube.com/watch?v=eHC2tjQPX3A)

[Apresentação que não tive paciência pra ver](https://www.youtube.com/watch?v=tCheLd4H-nM&t=304s)

[Aula do MIT (em inglês)](https://www.youtube.com/watch?v=tKwnms5iRBU)

[Apresentação de MST](http://www.dt.fee.unicamp.br/~ricfow/IA881/arvoreGeradora.pdf)

[MST segundo a USP](https://www.ime.usp.br/~pf/algoritmos_para_grafos/aulas/spanningtrees.html#exchange1)

[Apresentação de MST da USP](https://www.ime.usp.br/~coelho/mac0328-2011/aulas/aula20.pdf)

[Conceitos básicos de grafo](https://www.youtube.com/watch?v=MC0u4f334mI)

[Livro para suporte](http://www2.dcc.ufmg.br/livros/algoritmos/cap7/slides/c/completo1/cap7.pdf)

[Livro para suporte 2](http://www.inf.ufrgs.br/~tsrodrigues/utilidades/cormem.pdf)

