---
title: Camada de rede
---

Os protocolos da camada de rede especificam o **endereçamento** e **processos** que possibilitam que os dados da camada de transporte sejam empacotados e transportados.

O **encapsulamento** permite que os dados passem nas redes com o mínimo de overhead.

## Processos básicos da camada de rede

#### Endereçamento

Fragmentos individuais precissam ser direcionados a um dispositivo final, precisam de endereços

#### Encapsulamento

Adição de um cabeçalho oou rótulo (IP destino e origem)

#### Roteamento

Os hosts nem sempre estão conectado à mesma rede

#### Decapsulamento

No destino, o host examina o endereço de destino.
Se estiver correto o pacote é descapsulado

## Protocolos da camada de rede

1. Física
2. Enlace de dados
3. Rede
4. Transporte
5. Sessão
6. Apresentação
7. Aplicação

- IPv4
- IPv6
- ARP - Address Resolution Protocol

Utilizado para encontrar o media access control address da rede vizinha dada um endereço de IPv4

- RARP - Reverse Address Resolution Protocol
- ICMP - Internet Control message Protocol

## IPv4

**Sem conexão** - nenhuma conexão é estabelecida antes do envio dos pacotes de dados.

**Melhor esforço** - nenhum cabeçalho é usado para garantir a entrega dos pacotes.
Ele não se preocupa e nem deve se preocupar se os pacotes enviados foram recebidos.
Outros protocolos gerenciam o processo de rastreamento de pacotes e garantem sua entrega.

**Independente de meios físicos** - opera independemente do meio que transporta os dados.
A camada de rede não fica sobrecarregada com as características do meio físico em que os pacotes serão transportados.
É responsabilidade da camada de enlace. Apenas o tamanho máximo da PDU é considerado


## Classes de endereços IP

Classe de Endereços | Faixa do primeiro octeto | Rede (N) e Host (H) | Máscara de sub-rede padrão
------------------- | ------------------------ | ------------------- | --------------------------
A                   | 1-127                    | N.H.H.H             | 255.0.0.0
B                   | 128-191                  | N.N.H.H             | 255.255.0.0
C                   | 192-223                  | N.N.N.H             | 255.255.255.0
