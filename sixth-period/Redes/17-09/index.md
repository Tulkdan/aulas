---
title: Camada de Transporte
---

# Multiplexação e Demultiplexação

A camada de rede, especificamente o protocolo IP, oferece uma comunicação lógica entre hospedeiros.
A camada de transporte desempenha o papel fundamental de fornecer serviços de comunicação diretamente aos processos de aplicação que rodam em hospedeiros diferentes, ou seja, a chamada comunicação lógica entre processos de aplicação.
A responsabilidade dos protocolos da camada de transporte, UDP e TCP é ampliar o serviço de entrega do IP.

### Funcionamento

1. Nos hospedeiros, a camada de transporte recebe ou envia segmentos ao processo de aplicação apropriado
2. Um processo pode ter um ou mais **sockets**, portas pelas quais dados passam da rede para o processo e do processo para a rede.
3. Cada segmento de camada de transporte tem um conjunto de campos para tal finalidade
4. Os hospedeiros examinam os campos para identificar a porta receptora ou destinatária e direciona o segmento ao socket
5. A tarefa de entregar os dados contidos em um segmento ao socket correto é denominada **demultiplexação**
6. O trabalho dee reunir, no hospedeiro origem, partes de dados provenientes de diferentes sockets, encapsular e passar para a camada de rede é denominada **multiplexação**

## TCP

1. Causa sobrecarga adicional para adicionar funções
2. Cada segmento TCP tem 2 bytes de overhead
3. Orientado a conexão

## UDP

1. Fornece entrega de dados de baixa sobrecarga
2. Melhor esforço
3. Sem conexão

# Sockets

Uma interface local, criada por aplicações e controlada pelo SO, na qual os processos de aplicação podem tanto enviar quanto receber mensagens entre processos de aplicações (podendo ser local ou remoto).
Aplicações de redes consistem em um par de programas que residem em dois sistemas finais diferentes.
Ao serem executados, criam-se um processo cliente e servidor que se comunicam através de sockets.

