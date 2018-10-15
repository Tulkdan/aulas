---
title: Triggers
author: Pedro Correa
output: html_document
---

Eventos que podem automaticamente ativar uma função, ela monitora toda vez que certa ação definida for ativada.
As Triggers não podem afetar as tabelas nas quais ele esta associado e também não devem ser muito complexos.

Também deve-se definir se a trigger deve ser definida para ser executada para cada linha, dependendo da quantidade de registros que são desejados afetar, ou seja, se for necessário modificar apenas uma linha, deverá ser definido apenas essa linha para modificar, caso for *x* linhas, deverá ser definido essas *x* linhas para serem modificadas.


### Sintaxe

```
CREATE TRIGGER <nome>
    AFTER|BEFORE INSERT|DELETE|UPDATE
    ON <tabela>
    [FOR EACH ROW]
    BEGIN
        <comandos SQL>
    END;
```

`[FOR EACH ROW]` é opcional, indicando que a trigger deve ser executada para cada registro.

Existem dois modos que uma trigger pode ser realizada, antes (*before*) e depois (*after*) dos dados serem modificados na tabela.

### Before

Neste modo de trigger os dados ainda não foram inseridos dentro da tabela, mas já são conhecidos pelo programa, mas, as regras de integridade que a tabela possui ainda não foram verificados.

#### Vantagens

* Setar valores para algumas colunas do registro que está sendo inserido
* Guardar os valores que serão atualizados/removidos em uma tabela de histórico

### After

Neste modo de trigger, os dados já foram inseridos dentro da tabela, contudo, as regras de intregridade já foram passadas e não se pode mudar os valores.

#### Vantagens

* A trigger só será ativada após ter certeza que todas as regras de integridades foram aceitas.

### :NEW

Só é aceito para caso a trigger receber modo de *INSERT* e *UPDATE* na tabela que a trigger esta associada.

### :OLD

Só é aceito para caso a trigger receber modo de *DELETE* e *UPDATE* na tabela que a trigger esta associada.