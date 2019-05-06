---
title: Codificacao
author: Pedro Correa
output: html_document
---

# Codificação de Linha

## Tipos de codificação de linha

* Unipolar
* Polar
* Bipolar

A transformação de uma sequência binária na sua representação elétrica é feita através da codificação de linha.

Como representar a sequência de bits `1 0 1 1 0 1 1 0 1 0 1 0 0 0 1`, em que cada bit tem a duração de T segundos?

* Recuperação de Erro: informações suficientes de **temporização para a recuperação** de relógio no receptor

* Espectro de frequência: com pequena largura de banda **situada a baixas frequências**. Para minimizar a influência de ruído térmico, ruído impulsibo entre outros.

* Transparência: para **todos os tipos de mensagens**

* Decodificação univoca


No que diz respeito às principais técnicas de codifição, podemos dividi-las em trẽs

* NRZ
* RZ
* Manchester

## NRZ - Non-Return to Zero

É o tipo de codificação mais simples. Por meio dela, nós apenas representamos um 1 por meio de um sinal alto e um 0 por meio de um sinal baixo

## NZ - Return to Zero

Na codigição RZ, o nível de tensão ou corrente retorna sempre ao nível zero após uma transição

Diz-se por isso que tem uma *Duty cicle* de 50% e utiliza o dobro da largura de banda em relação aos códigos NRZ

## Mancheste Normal

Os limites da onda neste tipo de codigicação está entre `1` e `-1`. Neste código de linha, as decisões são sempre tomadas a meio de cada bit.
Assim, as transições  entre `0 -> 1` e `1 -> 0` ocupam a largura de um bit desde o meio do bit anterior até ao meio do bit seguinte.
As restantes transições, `0 -> 0` e `1 -> 1`, ocupam apenas meio bit.

## Manchester Diferencial

Um "1" é representado fazendo a primeira metade do sinal igual a última metade do sinal anterior e um "0" é representado fazendo a primeira metade do sinal ser diferente da segunda metade do sinal anterior.
Ou em outras palavras, se no começo do sinal houve mudança de sinal é 0, e se não houver é 1.