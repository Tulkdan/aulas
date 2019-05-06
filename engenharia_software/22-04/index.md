---
title: Diagramas
author: Pedro Correa
---

# Diagrama de máquina de estados

Especifica a sequência de estados que um objeto atravessa durante sua vida, mediante a eventos (estímulos externos).

Estados por que passa uma instância de uma classe.

Utilizado quando existir um certo grau de complexidade referente à transição de estados de um objeto.

Um estado representa a situação que um objeto se encontra em um determinado momento durante o período em que este participa de um processo.

Um estado é representado por um retângulo com as pontas arredondadas, apresentado duas ou três divisões, sendo que a primeira divisão armazena a descrição do estado e a segunda as possíveis ações ou atividades, executadas pelo objeto quando em um estado.
É possível existir uma terceira divisão para definir possíveis transições internas, que são transições que não causam mudanças no estado do objeto.

A segunda divisão do retângulo pode armazenar três cláusulas:

1. Entry - ações realizadas no momento em que o objeto assume o estado em questão;

2. Exit - ações executadas antes do objeto mudar de estado;

3. Do - ações executadas enquanto o objeto se encontra em um determinado estado;

* Estado inicial - momento que iniciará a análise de um objeto

* Estado final - final da análise de estados de um objeto

* Transições internas - ocorrem durante o estado de um objeto sem modificá-lo

---

# Diagrama de Componentes

Diagrama de componentes que determina como os componentes se relacionam.

Tudo o que for componente de Software, estará mapeado dentro deste diagrama.
Algo mais limitado pois não se é muito utilizado.

---

# Diagrama de Implantação

Documentação de servidores, firewall implementados, impressoras... Todos os componentes físicos utilizados pelo sistema.

São utilizados para representar a arquitetura física de um sistema.

Mostra os vários componentes de Software de um sistema e suas dependências.

Para sistemas simples, a arquitetura física não tem tanta importância.
No entanto, na modelargem de sistemas complexos é fundamental reconhecer quais são os componentes físicos do sistema e quais são as interdependências.

Os elementos são os **nós** e as **conexões**.

Um nó é uma unidade física que representa um recuso computacional, é um equipamento de importância para o sistema.
É representado graficamente por um cubo.
O nome e o tipo do nó são definidos no interior do cubo, ambos são sublinhados e separados por um sinal de dois pontos.

Os nós são ligados uns aos outros através de conexões, estas mostram mecanismos de comunicação entre os nós.
É representado graficamente por uma linha ligando dois nós, assim, também pode ser descrito o tipo de conexão.
