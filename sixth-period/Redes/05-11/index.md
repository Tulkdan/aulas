---
title: Roteamento IP
author: Pedro Correa
---

Tipos de roteamento:

### Estático

Quando o admin da infra defini a rota.

#### Vantagens

* Redução do overhead na CPU do router
* Não há utilização de largura de banda entre os routers
* Segurança (uma vez que o administrador possui total controle do processo de roteamento)

#### Desvantagens

* O administrador precisa, efetivamente, possuir um profundo conhecimento global da rede
* Se uma nova rede for adicionada, o administrador deve, manualmente, adicionar a rota
* Não é viável para redes de grande porte

### Dinâmico

Existem aplicações/protocolo que fazem isso (a.k.a Protocolos de Roteamento Dinâmico).
Eles definem a rota com o ip mais próximo.

### Default

É o IP default que caso não for encontrado o IP localmente, deverá ser enviado para outro responsável sobre o IP a ser pesquisável.

## Comando

```{bash}
Router(config)# ip route network-address subnet-mask (ip-address | exit-interfaces)
```

Parâmetro       | Descrição
--------------- | ------------------------------
network-address | Endereço da rede (ID da subrede) de destino da rede remota a ser adicionado a tabela de roteamento
subnet-mask     | Máscara de sub-rede da rede remota a ser adicionada a tabela de roteamento. A máscara de sub-rede pode ser modificada para sumarizar um grupo de redes
ip-address      | Normalmente conhecido como endereço IP do roteador do próximo salto (IP da roteador onde deverá fazer o salto)
exit-interface  | Interface de saída usada no encaminhamento de pacotes para a
