---
title: Sequence
author: Pedro Correa
output: html_document
---

Sequences são contadores automáticos incrementados toda vez que são acessados.
Permitem a geração de valores inteiros sem repetição e, dessa forma, 
facilitam a criação de valores para identificadores únicos.

### Sintaxe:
```
CREATE SEQUENCE nome_sequence
    [INCREMENT BY integer]
    [START WITH integer]
    [MAXVALUE integer]
    [MINVALUE integer]
    [CYCLE | NOCYCLE]
    [CACHE integer | NOCACHE]
    [ORDER | NOORDER]
```

nome_sequence    | Significado
---------------- | ------------------------------------------------------------
INCREMENT BY     | Indica o intervalo entre os números gerados
MIN VALUE        | Valor mínimo que a sequence pode assumir
MAX VALUE        | Valor máximo que a sequence pode assumir
START WITH       | O primeiro valor gerado pela sequence
CYCLE OU NOCYCLE | **CYCLE** reinicia após atingir o extremo e **NOCYCLE** incrementa até atingir o extremo estipulado
CACHE ou NOCACHE | **CACHE** armazena alguns valores em cache e **NOCACHE** não armazena cache
ORDER ou NOORDER | **ORDER** indica que os valores gerados estarão em Ordem e **NOORDER**  não garante ordenação

### Exemplos

Comando para criar uma sequence com valor mínimo 1, máximo quase infinito e incremento 1

```
CREATE SEQUENCE Exemplo_1
```

***

Comando seguinte cria uma sequence com incremento de 10 começando do 1

```
CREATE SEQUENCE Exemplo_2 INCREMENT BY 10
```

***

É possível acessar os valores nos comandos SQL através das "pseudo-colunas"

* **NEXTVAL**: Incrementa a sequence e retorna o novo valor
* **CURRVAL**: Retorna o último valor da sequence gerado para a sessão atual

**CURRVAL** só funciona quando utilizando após o **NEXTVAL**, caso contrário daria **ERROR**.