---
title: Pontos de Caso de Uso
author: Pedro Correa
---

Técnica utiliada para estimar tamanho de sistema.

- Calcular o *Unajusted Use Case Points* - UUCP;
- Definir o fator de ajuste TCF (*Technical Complexity Factor*) baseado no grau de importância de 13 temas que influenciam na complexidade técnica para desenvolvimento do sistema;
- Definir o fator e ajuste EF (*Environment Factor*) baseado no grau de importância de oito temas que definem o grau de eficiência do projeto;
- Calcular os *Use Case Points* ajustados;

## Passe 1: Cálculo do Peso dos Atores

Classificação dos atores envolvidos em cada caso de uso, obtendo um somatório de pontos não-ajustados;

Tipo de Ator     | Descrição               | Peso
---------------- | ----------------------- | ---------------------------------
Simples          | Outro sistema acessado através de um API de programação          | 1
Médio            | Outro sistema interagindo através de um protocolo de comunicação | 2
Complexto        | Um usuário interagindo através de um interface gráfica           | 3

O peso total dos atores do sistema é calculado pela soma dos produtos do número de atores de cada tipo pelo respectivo peso.

## Passo 2: Cálculo do Peso dos Casos de Uso

Para o cálculo inicial do peso bruto dos casos de uso, dividem-se os casos de uso em três níveis de complexidade, de acordo com o número de transações envolvidas em seu processament;

- Transação: é uma operação do sistema, que executa alguma ação (inclusão, alteração, exclusão, consulta) sobre as informações do bando de dados

Tipo de Caso de Uso   | Número de Transações  | Peso
--------------------- | --------------------- | ------
Simples               | Até 3                 | 1
Médio                 | 4 a 7                 | 2
Complexo              | 7 ou mais             | 3

- Cálculo do UUCW: somatória dos produtos da quantidade de casos de uso classificados em cada tipo pelo peso nominal do tipo em questão
O peso total não ajustado é calculado pelo somatório entre os pesos de atores e casos de uso:

`UUCP = UAW + UUCW`

## Passo 3: Cálculo dos Fatores de Ajuste

O ajuste é bastante similar ao adotado pela técnica de Pontod de Função, e é contituido de duas partes:

- Cálculo de fatores técnicos: cobrindo uma série de requisitos funcionais do sistema;
- Cálculo de fatores de ambiente: requisitos não-funcionais associados ao processo de desenvolvimento, tais como experiência da equipe e motivação;

Estes dois fatores geram multiplicados distinto que devem ser aplicados ao peso total não-ajustado, calculado anteriormente

### Passo 3.1: Fatores Técnicos

Fator    |  Requisito                 | Peso
-------- | -------------------------- | ------
T1       | Sistema Distribuído        | 2
T2       | Tempo de Resposta          | 2
T3       | Eficiência                 | 1
T4       | Processamento Complexo     | 1
T5       | Código Reutilizável        | 1
T6       | Facilidade de Instalação   | 0,5
T7       | Facilidade de Uso          | 0,5
T8       | Portabilidade              | 2
T9       | Facilidade de Mudança      | 1
T10      | Concorrência               | 1
T11      | Recursos de Segurança      | 1
T12      | Acessível por Terceiros    | 1
T13      | Requer Treinamento Especial| 1

Fator de complexidade técnica do sistema.

Para cada requisito, deve ser atribuído um valor que determina a influência do requisito no sistema, variando entre 0 e 5 - sendo que o valor 0 indica nenhuma influência, 3 indica influência moderada e 5 indica forte influência.

A somatória dos níveis de influência atribuídos a cada fator (T1 a T13) multiplicados pelo seu peso corresponde ao Tfactor

O cálculo do TCF é feito pela seguinte fórmula:
**0.6 + (0.01 x TFactor)**

### Passo 3.2: Fatores Ambientais

Fator   | Requisito                                      | Peso
------- | ---------------------------------------------- | -----------
E1      | Familiaridade com RUP ou outro processo formal | 1,5
E2      | Experiência com a Aplicação em Desenvolvimento | 0,5
E3      | Experiência em Orientação a Objetos            | 1
E4      | Presença de Analista Experiente                | 0,5
E5      | Motivação                                      | 1
E6      | Requisitos Estáveis                            | 2
E7      | Desenvolvedores em meio-expediente             | -1
E8      | Linguagem de Programação Difícil               | -1

O nível de influência indica o nível de disponibilidade de cada recurso no dcorrer do projeto, desta forma,
determinar que um dado fator tem nível de influência alta (isto é, atribuir a ele o valor 5) significa dizer que este fator está presente no projeto como um todo e influencia seu desenvolvimento.
Da mesma forma, atribuir um valor de influência zero (nenhuma influência) a um fator indica que o mesmo não está presente no processo de desenvolvimento.

A somatória dos níveis de influência atribuídos a cada fator (E1 a E8) multiplicados pelo seu peso corresponde ao EFactor.

O fator ambiental (EF) é calculado pela seguinte fórmula:
**1.4 + (-0.03 x EFactor)**

## Passo 4: Cálculo dos Pontos de Casos de Uso Ajustáveis

Esse cálculo é realizado com base na multiplcação dos pontos de casos de uso não ajustados, na complexidade técnica e na complexidade ambiental.
Pode-se calcular o valor total do sistema em *Use Case Points* ajustados utilizando-se da seguinte fórmula:
**UCP = UUCP x TCF x EF**
