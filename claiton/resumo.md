---
output: pdf_document
---

# Tipo de Mídias

## Discreta

Também conhecida como **estática** ou em bloco, pois é composta por itens que são independentes do tempo

* textos
* imagens
* gráficos

## Contínua

Também conhecida como **dinâmicas** ou dependentes do tempo, pois apresentam uma dependência temporal entre os itens de informação

* áudio
* vídeo
* animações

## Capturadas

São informações capturadas do mundo real

* imagens
* vídeo
* áudio

## Sintetizada

São informações criadas de uma meneira não natural

# Sinais analógicos e digitais

A informação analógica corresponde a uma onda eletromagnética gerada que pode assumir infinitos valores no tempo. Um bom exemplo é a voz humana.

## Analógico

O sinal analógico pode ser representado por uma onda senoidal com as seguintes características:

* **Amplitude**: representa o valor de intensidade mais alta. Para sinais elétricos ela é medida em volts
* **Frequência**: quantidade de cicles em um intervalo de tempo
* **Fase**: descreve a posição da forma de onda em relação ao tempo zero.

# Conversão Analógica Digital

Mesmo apresentando várias vantagens, a digitalização de informações multimídia apresenta algumas deficiências:

* **Distorção**: O maior problema da utilização de informações multimídia na forma digital é a distorção de codificação. O sinal gerado após a conversão D/A não é idêntico ao original.

Para a conversão de sinais analógicos em digital é necessário a realização de três passo:

* **Amostragem**: etapa em que o sinal analógico será retido num valor fixo por um período de tempo;
* **Quantificação**: os pulsos do sinal serão arredondados para níveis de quantização;
* **Codificação**: cada nível receberá um código binário;

# Técnicas de Codificação

## NRZ - Non-Return to Zero

É o tipo de codificação mais simples.
eio dela, nós apenas representamos um 1 por meio de um sinal alto e um 0 por meio de um sinal baixo.

Existem três tipo de codificação NRZ

### NRZ Unipolar

Este tipo de codificação é a mais simples.
Os limites da onda estão sempre entre 0 e 1 e tomam o valor 1 quando o bit a codificar é 1 e 0 quando o bit a codificar é 0.

### NRZ Polar

Os limites da onda neste tipo de codificação estão entre -1 e 1.
A onda codificada toma o valor 1 quando o bit a codificar é 1 e toma o valor -1 quando o bit a codificar é 0.


### NRZ Bipolar

Os limites da onda neste tipo de codificação estão entre -1, 0 e 1.
Toma o valor 0 quando o bit a codificar é 0 e toma o valor 1 e -1 **alternadamente** quando o bit a codificar é 1.

## RZ - Return to Zero

Na codificação RZ o nível de tensão ou corrente retorna sempre ao nível zero após uma transição.

Diz-se por isso que tem uma Duty cicle de 50% e utiliza o dobro da largura de banda em relação aos códigos NRZ.

Existem três tipo de codificação RZ:

### RZ Unipolar

Os limites da onda estão sempre entre 0 e 1 e toma o valor 1 quando o bit a codificar é 1 e 0 quando o bit acodificar é 0.

### RZ Polar

Os limites da onda neste tipo de codificação estão entre -1 e 1.
A onda toma o valor 1 quando o bit a codificar é 1 e toma o valor -1 quando o bit a codificar é 0.

### RZ Bipolar

Os limites da onda neste tipo de codificação estão entre -1, 0 e 1.
Toma o valor 0 quando o bit a codificar é 0 e toma o valor 1 e -1 alternadamente quando o bit a codificar é 1

## Manchester

Neste tipo de codificação, representamos um 1 por um sinal baixo que sobre e 0 por um sinal alto qe desce.
A sua principal vantagem é a **facilidade de se recuperar erros**.
Mesmo que parte da transmissão se perca, ainda assim é fácil detectar qual foi o sinal enviado.

### Manchester Normal

Os limites da onda neste tipo de codificação está entre 1 e -1.
Neste código de linha, as decisões sã sempre tomadas a meio de cada bit.
Assim, as transições entre 0 -> 1 e 1 -> 0 ocupam a largura de um bit desde o meio do bit anterior até ao meio do bit seguinte.
As restantes transições 0 -> 0 e 1 -> 1 ocupam apenas meio bit.

### Manchester Diferencial

Ao contrário da Manchester Normal que nos permite saber qual é o sinal enviado simplesmente acompanhando uma transiçã, a Diferencial é um pouco mais complexa.

Um 1 é representado fazendo a primeira metade do sinal igual a última metade do sinal anterior e um 0 é representado fazendo a primeira metade do sinal ser diferente da segunda metade do sinal anterior.
Ou, em outras palavras, se no começo do sinal houve mudança de sinal, é 0 e se não houve é 1.
