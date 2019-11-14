---
title: Autômato com pilha
---

Os autômatos com pilha têm o seu poder de reconhecimento estendido, quando comparado ao dos autômatos finitos, justamente pela disponibilidade e pela utilização de uma memória auxiliar organizada na forma de uma pilha.

A máquina de estados é capaz de armazenar, consultar e remover símbolos de um alfabeto próprio, denominado alfabeto de pilha, segundo a convenção usual para estruturas deste tipo (LIFO - "last in first out")

Quanto aos seus demais componentes e características, o autômato de pilha se assemelha ao autômato finito

* A pilha é independente da fita de entrada
* A pilha não possui limite máximo de tamanho
* Estruturalmente, sua principal característica é que o último símbolo gravado é o primeiro a ser lido
* "Push": introduzer um caractere adicional
* "Pop": apagar um caractere
* Substituição de um caractere
* Inicialização com qualquer símbolo, normalmente com o caractere vazio

#### AF

* Não admite transições vazias
* $\delta$ é uma função total
* Há equivalência entre os determinísticos e os não determinísticos

#### AP

* Admite transições vazias
* $\delta$ não é necessariamente uma função total
* Não há equivalência entre os determinísticos os não determinísticos

---

O modelo autômato com pilha possui duas definições universalmente aceitas que diferem no critério de parada do autômato:

1. o valor inicial da pilha é vazio e o autômato para aceitando ao atingir um estado final;
2. a pilha contém, inicialmente, um símbolo especial denominado símbolo inicial da pilha. Nã existem estados finais, e o autômato para aceitando quando a pilha estiver vazia

As duas definições são equivalentes, ou seja, possuem o mesmo poder computacional

Um autômato com pilha é composto basicamente por quatro partes:

1. **Fita**: análoga à do autômato finito
2. **Pilha**: memória auxiliar do tipo pilha que pode ser usada para leitura e gravação
3. **Unidade de controle**: reflete o estado corrente da máquina. Possui uma cabeça de fita e uma de pilha
4. **Programa**: função programa ou função de transição. Comanda a leitura da fita, leitura e gravação da fita e define o estado da máquina

Um AP é uma 6-upla:

$$M = (\Sigma, Q, \delta, q0, F, V)$$

* $\Sigma$ é um alfabeto de entrada
* Q é um conjunto finito de estados possíveis
* $\delta$ é um função de transição ou função programa
* q0 é o elemento que representa o estado inicial
* F é um subconjunto de Q, denominado conjunto de estados finais
* V é um alfabeto auxiliar ou alfabeto de pilha


As seguintes características da função programa devem ser consideradas:

* Trata-se de uma funçaõ parcial e, portant, pode ser definida para alguns argumentos do conjunto de partida
* A omissão do parâmetro de leitura, representado por "?", indica o teste de pilha vazia ou toda palavra de entrada lida
* O símbolo $\varepsilon$ na leitura indica a facilidade de movimento vazio da fita ou da pilha, neste caso o autômato não lê, nem move a cabeça
* O símbolo $\varepsilon$ na gravação indica que nenhuma gravação é realizada na pilha e não move a cabeça

### Formato de transição de um AP

$$\delta (q1, a, A) = {(q2, B)}$$

# Máquina de Turing

Uma das abordagens do estudo das Máquinas de Turing pe como reconhecedores de linguagens, ou seja, dispositivos capazes de determinar se uma dad palavra sobre o alfabeto de entrada pertence ou não a uma certa linguagem.

Formaliza a ideia de uma pessoa que realiza cálculos, sendo que o modelo possui no mínimo o mesmo poder computacional de qualquer cmputador de propósito geral.
O ponto de partida de Turing foi analisar a situação na qual uma pessoa, equipada com um instrumento de escrita e um apagador, realiza cálculos em uma folha de papel organizada em quadrados.

O trabalho da pessoa pode ser resumido em sequência de operações simples como segue:

* Ler um símbolo de um quadrado
* Alterar um símbolo em um quadrado
* Mover os olhos para outro quadrado
* Quando é encontrada alguma representação satifatória para a resposta desejada, a pessoa termina seus cálculos
* Possui memória ilimitada

Uma máquina de Turing possui:

* uma fita infinita para representar a sua memória ilimitada
* um cabeçote para ler e escrever na memória/fila
* um controle
* capacidade de movimentar-se para a direita ou esquerda
