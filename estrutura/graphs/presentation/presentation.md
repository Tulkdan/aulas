---
title: Árvore Geradora Mínima
author: Felipe Theodoro | Juan Domingos | Pedro Correa | Ygor Gazola
output:
    ioslides_presentation:
        widescreen: true
        bigger: true
        transition: 0.5
toc: yes
---

## O que é

- É um subconjunto do grafo
- Deverá conectar todos os vértices do grafo
- Deverá conter o caminho que possui o menor peso.

<div class="notes">
- Árvore dentro do grafo
- Possui o caminho de menor peso
</div>


## Para ser considerada AGM

<p align="center"> 
<img src="../img/mst.png">
</p>

- Grafo deve ser conexo
- Não deve possuir nenhum ciclo
- Deve ser em grafos não-direcionados
- Deve ser em grafos ponderados (as arestas possuem pesos)

## Algoritmos Gulosos

- Não oferece garantia
- Mas é possível provar uma MST

```{c}
void GenericoMST() { 
    S = {};
    while(S não constitui uma árvore geradora mínima) { 
        (u, v) = seleciona(A);
        if (aresta (u, v) é segura para) 
            S = S + {(u, v)};
    }
    return S;
}
```

<div class="notes">
Antes de cada iteração, S é um subconjunto de uma MST.
A cada aresta que verificamos, adicionamos a S a aresta (u,v) que não viola a variante.
Essa aresta é chamada de *aresta segura*.

Dentro do *while*, S tem que fazer parte do subconjunto prórpio da MST T, e quando existir uma aresta (u,v) que pertence a T e que não pertence a S e é considerado uma *aresta segura*, ele é adicionado ao conjunto de S.
</div>

## Corte | Aresta segura

<p align="center"> 
<img src="../img/mst-greedy.png" style="width: 70%;">
</p>

<div class="notes">
- Um corte (V′, V−V′) de um grafo não direcionado G = (V, A) é uma partição de V.
- Uma aresta (u, v) ∈ A cruza o corte (V′, V−V′) se um de seus vértices  pertence a V′ e o outro vértice pertence a V−V′.
- Um corte respeita um conjunto S de arestas se não existirem arestas em S que o cruzem.
- Uma aresta cruzando o corte que tenha custo mínimo sobre todas as
arestas cruzando o corte é uma aresta leve.
</div>

## Algoritmo de Prim

<p align="center"> 
<img src="../img/250px-Prim.PNG" style="width: 35%;">
</p>

<div class="notes">
- Escolhido a partir de um ponto
- Seleciona as arestas seguras de menor peso
- Repita o passo anterior até todos os vértices estarem conectados
</div>

---

<p align="center">
<img src="../img/PrimAlgDemo.gif" style="width: 50%;">
</p>

## Algoritmo de Kruskal

<p align="center">
<img src="../img/kruskal.png" style="width: 60%;">
</p>

<div class="notes">
- *S* é uma floresta
- Ordena as arestas por peso
- Seleciona a aresta de menor peso da floresta
- Seleciona outra aresta segura de menor peso
- Repita até todos os vértices estarem conectados
</div>

---

<p align="center">
<img src="../img/KruskalDemo.gif" style="width: 50%;">
</p>

## Prim *vs* Kruskal

- **Prim** : Melhor quando se tem um grafo muito denso com mais pontos que vértices
- **Kruskal** : Melhor quando se tem um grafo esparço

<div class="notes">
Use o algoritmo de Prim quando se tem um grafo com mais pontos que vértices, se não use o algortimos de Kruskal
</div>