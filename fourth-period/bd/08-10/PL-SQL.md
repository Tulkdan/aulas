---
title: PL/SQL
output: html_document
---

PL/SQL significa *Procedural Language/SQL*, ele estende o SQL adicionando construções encontradas em linguagens procedurais. Todos os programas el PL/SQL são compostos por blocos, esses blocos podem estar um dentro do outro permitindo os comandos de *SELECT*, *INSERT*, *DELETE* e *UPDATE*.

### Variáveis

As variáveis em PL/SQL podem ser compostas por 30 caracteres não podendo possuir espaços em braco, o primeiro char deve ser uma letra e não podem ser usados caracteres especiais.

Os tipos de variáveis disponíveis são as mesmos tipos de valor por coluna existentes em SQL (*NUMBER*, *VARCHAR2(N)*, *CHAR(N)*, etc), mas também pode-se declaram uma variável sendo igual a uma linha de uma tabela, pode-se ser também tipo *BOOLEAN* e pode armazenar o tipo que representa um registro de uma tabela.

### Sintaxe
```
DECLARE
/* Seção declarativa: variáveis, tipos e subprogramas locais */
    v_numero        NUMBER := 3
    v_text          VARCHAR2(20);
    v_tipo_coluna   tabela.coluna%TYPE
    v_tipo_registro tabela%ROWTYPE
    v_boleano       BOOLEAN

BEGIN
/* Seção onde vão as instruções procedurais e SQL. */
    v_numero := v_numero + 1;
    v_texto := 'TESTE'
    v_boleano :=true;

EXCEPTION
/* Seção de tratamento de exceção */

END;
```

### Controle de fluxo
```
IF <condicao_1> THEN
    <lista de comandos>
ELSIF <condicao_2> THEN
    <lista de comandos>
ELSE
    <lista de comandos>
END IF;
```

### Repetição
`DO...WHILE()`
```
LOOP
    <lista de comandos>
    EXIT WHEN <condicao>
END LOOP;
```
`WHILE()`
```
WHILE <condicao> LOOP
    <lista de comandos>
END LOOP;
```
`FOR()`
```
FOR <var> IN [REVERSE] <inicio>..<fim> LOOP
    <lista de comandos>
END LOOP;
```

## Cursor

É igual a um for normal, mas a navegação é pelo registros das linhas retornadas por um comando *SELECT*, finalizando o loop quando não tem mas registros retornados.

```
BEGIN
    FOR <variavel> IN (SELECT id_produto
                        FROM produto)
    LOOP
        UPDATE produto SET valor = valor * 1.1 WHERE id_produto = variavel.id_produto;
    END LOOP;
END;
```

## Exception

Local onde o programador podem tratar os erros que ocorrerem de forma personalizada ou os erros podem ser associadas a erros da própria base Oracle.

Todo o tratamento de uma exceção consiste em uma cláusula *WHEN* e de comandos a serem executados quando a exceção é chamada

```
EXCEPTION
WHEN <nome_da_exception_1> THEN
    <corpor para tratamento da exception>
WHEN <nome_da_exception_2> THEN
    <corpor para tratamento da exception>
WHEN <nome_da_exception_3> THEN
    <corpor para tratamento da exception>
END;
```

Tipo de error     | Significado
----------------- | -----------------------------------------------------------
DUP_VAL_ON_INDEX  | Restrição de chave primária violada. Usada em operações *INSERT*
NO_DATA_FOUND     | *SELECT* executado sem retornar data
TOO_MANY_ROWS     | Quando um *SELECT INTO* retorna mais de um registro
INVALID_NUMBER    | Erro de conversão para o tipo *NUMBER*
OTHER             | Qualquer erro que pode ocorrer, bom deixá-la como última opção para garantir que todos os erros sejam detectados