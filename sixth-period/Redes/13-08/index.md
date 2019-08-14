---
title: DNS
---

Surgiu diante de uma demanda que funcionasse de um forma distribuida e hierárquica.
Ele venho para facilitar a mente humana de guardar nomes e intermediar o nome para a linguagem de máquina.
É um dos serviços na camada de comunicação mais **importante** pela sua facilidade para o usuário.

É serviços que mais sofre ataque. Possui todas as informações do domínio, e quando atacado, pode informar ao atacante todos os endereços dos usuários conectados a rede.
Possui o protocolo de comunicação UDP.
Para se criar um endereço, é necessário para que se tenha dois servidores, o master e slave, que se mantenham em sincronia para quando o master cara fora do ar, o slave possa assumir sem que o usuário sinta a diferença.
Essa sincronização entre servidores é utilizada o TCP.

## Implementação

#### User

É o que solicita a informação para o servidor mandando o nome do DNS.
A mapeação esta no diretório *hosts*, onde podemos mapear o IP com o nome do DNS desejado, assim, toda vez que for fazer alguma conexão via DNS, ele irá verificar primeiro neste diretório para ver se está mapeado localmente.

#### Resolver

É a entidade cliente do DNS.

#### Name Server

Entidade servidora que responde as requisições.
A maioria dos servidores existentes utilizando o [BIND](https://www.isc.org/bind/) nos ambientes UNIX.

***

Possui 3 tipos de servidores DNS:

#### Raiz

São o servidores que gerenciam os endereços de DNS mapeados e acessados pelo local,
assim muitos servidores que não sabem os DNS procurado,
envia para o servidor raiz que possa saber o endereço para ser tratado a requisição da chamada.

#### Autoridade

Servidor no qual foi configurado o DNS.
