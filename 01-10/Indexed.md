---
title: Indexes
author: Pedro Correa
output: html_document
---

Existem 2 tipo:

## Primárias

***

Onde são criadas automáticas pelo banco, não é possível ser modificado

## Secundária

***

Onde podem ser criadas pelo desenvolvedor, exitem 2 tipo: normal e únicos

Para se criar um índice normal -> `CREATE INDEX nome_Indice ON Tabela(campo);`

Para se criar um índice único -> `CREATE UNIQUE INDEX nome_Indice ON Tabela(campo);`
