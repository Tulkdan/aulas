---
title: Sistemas de Arquivos
---

Arquivo: abstração criada pelo SO para gerenciar e representar os dados.

Gerencia de arquivos: são criados diretórios para facilitar a manipulação de dados através de:

* criar, deletar;
* abrir, fechar
* ler, escrever
* posicionar

Na ling. "C" temos a I/O ANSI com um conjunto de funções para controlar o sistema de arquivos do SO.

Principais funções:

1. fopen()
2. fclose()
3. fprintc()
4. fgetc()
5. fwrite()
6. fread()
7. fscanf()

### Passos para uso:
```c
// cria ponteiro do tipo arquivo
FILE *arquivo;
// abrir/criar arquivo
arquivo = fopen("caminho-do-arquivo", "w");
// ler arquivo
aux = fscanf(arquivo, "%d %f", &cod, &salario);
// fechar arquivo
fclose(arquivo);
```

