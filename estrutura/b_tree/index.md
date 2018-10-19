---
title: Árvore B
author: Pedro Correa
output: html_document
---

# Introdução

Foi criada de para poder armazenar grande quantidade de dados muito grandes para fazer a pesquisa dos dados com muita mais eficiência. 
Tendo menos leitura no HD, criando pequenos pedaços que serão utilizados para a memória RAM, de acordo com a necessidade, irá trocar as informações do HD para a RAM 

Uma solução é aumentar a quantidade de filhos que o nó pode ter, por exemplo, na árvore binária, para percorrer 1.000.000 de dados levaríamos $log2(1000000) = 20$, assim, iríamos percorrer 20 nós para encontrar o valor.

Se a gente aumentar em 3 nós, poderíamos armazenar mais nós e mais dados que o nó poderá armazenar, contudo, começamos a chamar as árvores de *m-árias*, que serão de base M, alargando mais a árvore e tendo mais informações trazidas do Disco.

***

Uma árvore B de parâmetro *t* é uma árvore *m-ária* em que:
* todas as filhas estão no mesmo nível
* cada nó interno tem pelo menos *t* filhos
* todo nó tem no máximo **2t** filhos
* a árvore satisfaz a propriedade de busca

#### Observações

Como o nó interno pode ter mais chaves dentro de si, para cada chave, temos chave+1 filhos.

O mínimo de dados que é possível ter é t filho, sendo a chave possuindo t-1 registros
O máximo de dados que é possível ter é 2t filho, sendo a chave possuindo 2t-1 registros

**A raiz sempre terá no mínimo 2 filho**

# Escolhendo *t*

Escolha *t* máximo que um nó com 2t filhos caiba na página, se $t = 1001$ com $h = 2$ armazenamos até $10^9$ chaves, sou seja, armazenanos duas vezes o disco

# Busca na Árvore B

Para procurar a chave *k* no nó *x*
* Verificar se a chave está no nó X
* Case não, entre no filhos de acordo

# Inserção

A inserção sempre ocorre em um nó folha, porém, o nó pode estar cheia