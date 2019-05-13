---
title: Diagrama de Sequência
author: Pedro Correa
---

Determinar a sequência de eventos que ocorrem em um determinado processo, ou seja, quais condições devem ser satisfeitas e quais métodos devem ser disparados entre os objetos envolvidos e e, que ordem durante um processo específico.

Assim, determinar a ordem em que os eventos ocorre,, as mensagens que são enviadas, os métodos que são chamados e como os objetso interagem entre si dentro de um determinado processo é o objetivo principal deste diagrama.

Um diagrama de sequência na marioria das vezes se identifica com um caso de uso específico, porque um caso de uso, em geral, repere-se a um processo disparado por um usuário.

# Atores

Entidades externas que interagem com o sistema e que disparam serviços bonecos. Magros identicos aos usados no diagrama de casos de uso, porém contendo uma linha de vida.

# Objetos

Representam as instâncias das classes envolvidas no processo tratado no diagrama de sequência. Os objetos são apresentados como retângulos contendo um texto que identifica primeiramente o nome do objeto, em minúsculo, e depois o nome da classe, com as letras inicias maiúsculas, a qual o objeto pertence essas duas informações são separadas por um símbolo de dois pontos. Possuem uma linha de vida representada por uma linha vertival tracejada.

# Mensagens ou estímulos

Demonstrar a ocorrência de eventos, que normalmente forçam a chamada de um método em algum dos objetos envolvidos no processo de um diagrama de sequência, em geral, é iniciado por algum evento externo, causado por algum ator, o que acarreta o disparo de um método em um dos objetos.

A ocorrência deste evento inicia o processo. As mensagens podem ser disparadas entre:

- um ator e outro ator
- Um ator e um objeto, onde o ator produz um evento que força o disparo de um método em um objeto
- um objeto e um objeto, onde um objeto transmite uma mesnagem para outro objeto solicitando a execução de um método
- um objeto a um autor, ocorre quando possui mensagem de retorno
