---
title: Procedure
author: Pedro Correa
output: html_document
---

São idênticos as functions, mas não é necessário retornar algum valor.
Eles podem ser definidos como subprogramas que ficam armazenados na base de dados em sua forma executável, podendo receber valores para trabalhar.

Um procedimento **NÃO** pode ser chamado como parte de uma expressão em comandos DML (*SELECT*, *INSERT*, *DELETE* e *UPDATE*)

### Sintaxe

```
CREATE OR REPLACE PROCEDURE <nome_procedure> (
    <parametro1> IN | OUT | IN OUT <Tipo>,
    <parametro2> IN | OUT | IN OUT <Tipo>) AS
    BEGIN

    EXCEPTION

    END;
```

## Tipos de Parâmetros

Os parâmetros podem ser classificados como parâmetros de entrada, saída ou entrada e saída.

```
CREATE OR UPDATE PROCEDURE <teste_parametro> (
    p_parametroIN   IN Number,
    p_parametroOUT  OUT Number,
    p_parametroIN_OUT IN OUT Number) AS
    BEGIN

    EXCEPTION

    END;
```

### Tipo IN

O parâmetro é recebido pelo procedimento dentro d argumento do tipo **IN**.
Este parâmetro não poderá ser modificado, pois ele pode ser usado apenas para a leitura.

### Tipo OUT

Qualquer valor passado na chamada no procedimento para argumentos do tipo OUT é ignorado.
Este parâmetro é somente de escrita, ou seja, não é possivel atribuir valor a outras variáveis a partir dela.

Quando o procedimento finaliza, retorna ao bloco que o chamou atualizando a variável do bloco com o valor setado pelo procedimento.

### Tipo IN OUT

É uma combinação do **IN** e **OUT**, ou seja, o parâmetro recebido servirá para escrita ou leitura.

***

# Functions

São muito similares as procedures, possuindo os mesmos tipos de restrições e argumentos.

Normalmente são utilizadas para cálculos, podendo ser chamada como parte de uma expressão em comandos DML, mas não é aconselhável fazer alterações de registros.
Geralmente sempre retornam algum valor.

#### Sintaxe
```
CREATE OR REPLACE FUNCTION <name_function> (
    parametro1 IN | OUT | IN OUT Tipo,
    parametroN IN | OUT | IN OUT Tipo)
RETURN <tipo_retorno> AS
BEGIN

EXCEPTION

END;
```