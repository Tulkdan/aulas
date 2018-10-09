---
title: Triggers
author: Pedro Correa
output: html_document
---

Eventos que podem automaticamente ativar uma função, ela monitora toda vez que certa ação definida for ativada.
Também deve-se definir se a trigger deve ser definida para ser executada para cada linha, dependendo da quantidade de registrodos que são desejados afetar, ou seja, se for necessário modificar apenas uma linha, deverá ser definido apenas essa linha para modificar, caso for *x* linhas, deverá ser definido essas *x* linhas para serem modificadas.

Existem dois modos que uma trigger pode ser realizada, antes (*before*) e depois (*after*) dos dados serem modificados na tabela.

Caso uma trigger for *before*, ela será executada antes dos dados serem modificados na tabela. 

##### Benefícios: 

* Você consegue manipular anteriormente o dado antes de ser modificado dentro da tabela.

Caso uma trigger for *after*, ela será executada antes dos dados serem modificados na tabela.

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