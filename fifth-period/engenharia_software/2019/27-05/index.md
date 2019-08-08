---
title: Design Patterns
author: Pedro Correa
---

Um padrão de projeto descreve um solução que já foi utilizada para resolver problemas específicos, ou seja,
são soluções para problemas que alguém teve um dia e resolveu aplicando um modelo que foi documentado e que você pode adaptar integralmente na sua solução.

De maneira mais formal, podemos conceituar padrões para arquitetura de software são soluções de eficiência já comprovadas e amplamente utilizadas para a resolução de problemas comuns em projeto de software estas soluções são desenvolvidas e conhecidas por especialistas e tornam-se padrões por serem reutilizadas várias vezes em vários projetos e por terem eficácia comprovados.

Principais propriedades dos padrões de projeto:

- Capturam o conhecimento e a experiência de especialistas em projeto de software;
- Definem um vocabulário comum para a discussão de problemas e soluções de projeto;
- Facilitam a documentaçao e manutenção da arquitetura do software;

Principais benefícios:

- Fornecem soluções que já foram testadas e aprovadas;
- Tornam o sistema mais fácil de entender;
- A comunicação entre os participantes do projeto é mais eficiente;

**Importante**:
Padrões de projeto não são uma varinha mágica que vai tornar o seu projeto isento de falhas.
Se for mal implmentado eles podem até diminuir a compreensão do seu projeto e aumentar a quantidade de codigo.
Portanto padrões de projeto não resolvem todos os problemas.

# MVC (Model View Controller)

Tem por objetivo básico separar a lógica de negócio da apresentação.
Divide a funcionalidade envolvida em modelo de negócio e apresentação.

Na arquitetura MVC o "modelo" representa os dados da aplicação e as regras de negócio que governam o acesso e a modificação dos dados.
O "modelo" mantém o estado persistente do negócio e fornece ao controlador a capacidade de acessar as funcionalidades da aplicação.
Um componente de "visualização" exibe o conteúdo de uma parte particular do "modelo" e encaminha para o controlador as ações do usuário,
acessa também os dados do modelo via "controlador" e define como esses dados devem ser apresentados.

Um "controlador" define o comportamento da aplicação, é ele que interpreta as ações do usuário e as mapeia para chamadas do modelo.

Com base nas ações do usuário e no resultado do processamento do "modelo", o "controlador" seleciona uma visualização a ser exibida como parte da resposta a solicitação do usuário.
Há normalmente um controlador para cada conjunto de funcionalidades relacionadas.

Visualização   |  Controle       |  Modelo
-------------- | --------------- | --------------
web            |  controlador    |  negócio
desktop        |                 |  persistência

