# Árvore AVL

Árvores AVLs vem para arrumar a árvore para balancear, caso a árvore tenha mais filhos do lado direito que o esquerdo ou vice versa.

Uma árvore AVL balanceada é uma árvore em que as alturar de suas subárvores nunca difere maior que 1, ou seja, a diferença de filhos deve ter uma diferença entre 0 e 1. Para calcular $| He - Hd | < 2$

## Caso A

Quando a inserção ocorre na sub-árvore esquerda do nó à esquerda N (Ne), sendo N o nó que está sempre desbalanceado.

## Caso C

Caso simétrico ao Caso A, a diferença é pelo lado que irá virar, que será realizado uma rotação parar a esquerda

```{c}
void rotacaoEsq (p_no n) {
  p_no nd;
  nd = n->dir;
  n->dir = nd->esq;
  nd->esq = n;
}
```

### Passos
* Ne é colocado na raiz
* EE permanece a sub-árvore esquerda de Ne
* N torna-se a raiz da sub-árvore direita de Ne
* ED torna-se sub-árvore esquerda de N
* D permanece a sub-árvore direita de N

## Caso B

Ele é necessário fazer duas rotações para a direita, podendo desmembrar nas duas rotações simples já conhecidas, uma no filho para a esquerda, outra no pai para a direita

## Caso D

Mesma coisa para para o caso B mas com as rotações invertidas, a primara para o filho na direita e outra para o pai para a esquerda.

## Remoção

Para a árvore AVL, deverá verificar se a remoção acarretou em desbalanceamento, realizando as rotações necessárias para o balanceamento. Para remover na árvore AVL, deverá seguir esses passos:

* Buscar a nó que deverá ser removido
* Se é folha, remover
* Se possui apenas um filho, coloca este filho no seu lugar
* Se tem dois filhos, busca o **antecessor** (maior valor na árvore da esquerda), copia os valores do nó pelos valores do sucessor e remove o antecessor
