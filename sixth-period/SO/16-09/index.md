---
title: Sistemas de arquivos
author: Pedro Correa
---

[Referência do resumo foi tirado deste artigo](https://pt.m.wikibooks.org/wiki/Sistemas_operacionais/Sistemas_de_arquivos)

Os sistemas de arquivos estruturam a informação guardada em uma unidade de armazenamento, podendo ser representada de forma textual ou graficamente.
O habitual é utilizar dispositivos de armazenamento de dados que permitem o acesso aos dados como uma corrente de blocos de um mesmo tamanho, usualmente de 512 bytes de largura.
O software do sistema de arquivos é responsável pela organização destes setores em arquivos e diretórios e matém um registro de que setores pertencem a que arquivos e quais não têm sido utilizados.

Em geral os sistemas de arquivos proveem métodos essenciais para criar, mover, renomear e eliminar tanto arquivos como diretórios. Outros permitem a criação de enlaces (links) adicionais a um diretório ou arquivo.

## Características dos sistemas de arquivos

* Segurança ou permissões
* Listas de controle de acesso (ACLs)
* Mecanismo para evitar a fragmentação
* Capacidade de enlaces simbólicos (symbolic link) ou duros (hard links)
* Integridade do sistema de arquivos
* Suporte para arquivos dispersos
* Suporte para quotas de discos
* Suporte de crescimento do sistema de arquivos nativo

## Conceitos de arquivos

* Arquivos de texto Windows por padrão são codificados em ASCII e não em Unicode.
* Arquivos de texto Unix por padrão são codificados em Unicode e não em ASCII 
* O termo "arquivo de repositório" não é uma terminologia reconhecida, tudo indica que se quis dizer "arquivos de dados".

## Tipos de Arquivos

O Unix e Windows suportam diretórios e arquivos organizados em sequência de bytes, sequência de registros e árvores, conhecidos como arquivos regulares.
O Unix, em especial, suporte além dos tipos regulares, arquivos de dispositvo de caracteres e de blocos.

* Os **arquivos de dispositivo de caracteres** são utilizados para modelar dispositivos de E/S, tais como terminais, redes, impressoras.

* Já os **arquivos de dispositivo de blocos** dão suporte aos dispositivos de armazenamento em massa tais como discos e pen-drives.

## Implementação de arquivos

A criação de arquivos exige que o sistema operacional tenha controle de quais áreas ou blocos no disco estão livres.
É importante o controle de quais blocos de discos estão relacionados a quais arquivos.
Este gerenciamento pode ser feito, principalmente de várias formas:

* **Alocação contígua**
* **Alocação por lista encadeada**
* **Alocação Indexada**
* **Alocação Combinada**

### Alocação Contígua

É o esquema mais simples de alocar e armazenar os arquivos no disco.
Consiste em armazenar um arquivo em blocos sequenciamente dispostos.
Neste tipo, o sistema localiza um arquivo através do endereço do primeiro bloco e da sua extensão em blocos.

Este tipo de alocação apresent duas vantagens significativas:
* É bastatne simples de implementar e de realizar o controle sobre os onde os blocos estão
* O desempenho de leitura é excelente, pois todo o arquivo pode ser lido em uma única operação a partir do primeiro bloco de dados

No entanto, este tipo de implementação apresenta um grande problema: **Fragmentação do disco**
Como os arquivos podem ser criados e eliminados frequentemente, os segmentos livres vão se fragmentando em pequenos pedaços por todo o disco.

O problema da fragmentação pode ser contornado através de rotinas que reorganizem todos os arquivos no disco de maneira que só exista um único segmento de blocos livres.

A alocação contígua é amplamente utilizada em CD-ROMS e em discos apenas para leitura.
Neles o tamanho do arquivo é conhecido anteriormente e nunca vai ser alterado.

### Alocação por Lista Encadeada

Consiste em manter os arquivos como uma lista encadeada de blocos de disco.
Dessa forma uma parte de cada bloco é usada como ponteiro para o próximo bloco.
O restante do bloco é usado para dados.

Uma **vantagem** desse tipo é que o tamanho do arquivo não precisa ser conhecido antes de sua criação, já que cada bloco terá um ponteiro para o próximo bloco.

A **desvantagem** deste tipo de alocação é o tempo de leitura extremamente lento.
O acesso deverá ser sempre sequencial.
Assim para acessar um bloc intermediário será necessário percorrer o arquivo desde o início.

### Alocação Indexada

É uma forma para resolver o problema do tempo de leitura da aloação por lista encadeada.
Isso é feito por meio de uma tabela de endereços dos blocos ocupados pelo arquivo.
Para cada novo bloco alocado, inclui-se mais um novo item na tabela.

Usando alocação indexada, o acesso aleatório fica mais fácil, pois não é necessário percorrer todos os blocos de forma sequencial, o endereço de cada bloco fica armazenado na tabela de alocação.

A **desvantagem** desta abordagem é que a tabela alocação deverá ser mantida na memória principal e dependendo do tamanho dos blocos do disco, ocupará muito espaço de memória.

### Alocação Combinada

Esta abordagem envolve o uso de blocos de índices e de encadeamento dos mesmos.
Ela combina a baixa ocupação de espaço em memória da técnica de **lista encadeada** e o bom desempenho da técnica de **tabela de índices**.

É usado uma técnica chamada de níveis de indireção na indexação.
Nesta técnica parte de um bloco é utilizado para apontar diretamente para blocos de dados, chamado de **apontadores diretos**,
e outra parte pode ser utilizada para apontadores de blocos, chamado de **apontadores indiretos**.

Esta abordagem é tipicamente utilizada na implementação do sistema de arquivos Unix, chamada de **I-nodes** (index-node).
Um I-node são os metadados, armazenado em estrutura de dados própria, que relaciona atributos e os endereços dos blocos de um arquivo.

## Cache de Sistemas de Arquivos

Uma cache de disco pode ser definida como parte da memória RAM ou memória buffer cache, utilizada para acelerar o acesso aos dados que estão sendo mais frequentemente requeridos.
Para isso a cache de disco pode ser implementado de duas formas principais:

1. Através da memória RAM inserida no próprio disco rígido
2. Utilização de parte da memória RAM

Caches de disco rígido são mais eficientes, mas são também muito mais caras.
Todos os discos rígidos modernos possuem uma cache interna.
Para complementar a cache interno, os SOs criam um segundo cache usando a memória RAM.

Caches de disco funcionam armazenando os dados mais acessados.
Quando um programa precisa acessar um novo dados. o SO primeiramente verifica se os dados estão na cache antes de lê-lo do disco.
Isso poque o acesso à memória RAM é muito mais rápido do que o acesso ao disco.

## Gerência de Espaço Livre

O monitoramento de espaço livre em um disco é realizado principalmente através de dois diferentes métodos: lista encadeada e mapa de bits

### Lista Encadeada

Nesta forma de gerenciamento, primeiramente é preciso entender que os blocos livres são blocos que não contém arquivos.
Entretanto os mesmos não ficam em branco, pois enquanto não estão sendo utilizados, eles contém informações que o SO armazena para mapear o espaço livre.
Ou seja, estes blocos estão livres porém não estão vazios.

Desta forma, cada bloco livre no disco possui ponteiros para os seguintes espaços livres em disco.
Se considerarmos que um bloco possui um tamanho de 1 KBytes, e um ponteiro tem tamanho de 32 bits, dentro de um bloco livre temos 255 ponteiros para os próximos blocos livres do disco.

### Mapa de Bits

Nesta forma de gerenciamento, é utilizado um espaço fixo adicional em disco especificamente para o mapeamento do espaço livre, de forma que para cada bloco em disco é utilizado um bit no mapa.
Sendo assim, um disco com n blocos requer um mapa de bits com n bits.
Não surpreende que os mapas de bits requeiram menos espaço, já que ele usa 1 bit por bloco, contra 32 bits no modelo de lista encadeada.

Uma vantagem dessa estratégia é que as alocações em um bloco único de mapa de bits faz com que o blocos de disco fiquem próximos uns dos outros, minimizando assim os ovimentos dos braços de leitura do disco.
