---
title: Resumo
---

# Acesso direto à memória - DMA

[fonte](https://pt.m.wikipedia.org/wiki/Acesso_direto_%C3%A0_mem%C3%B3ria)

O DMA permite que certos dispositivos de hardware num computador acessem a memória do sistema para leitura e escrita independentemente da CPU.
Muitos sistemas utilizam DMA, incluindo controladores de disco, placas gráficas, de rede ou de som.

O acesso direto da memória é usado igualmente para transferência de dados de núcleos em processadores multicore,
em especial nos sistema-em-microplaquetas do processador,
onde seu elemento de processamento é equipado com uma memória local,
e o acesso direto da memória é usado para transferir dados entre a memória local e a memória principal.
Os computadores que têm os canais de acesso direto à memória podem transferir dados aos dispositivos com muito menos perdas gerais de processamento do que computadores sem uma via de acesso direto à memória.

Uma transferência por DMA essencialmente copia um bloco de memória de um dispositivo para outro.
A CPU inicia a transferência, mas não executa a transferência.
Para os chamados third party DMA, como é utilizado normalmente dos barramentos ISA, a transferência é realizada pelos controladores DMA que são tipicamente parte do chipset da placa mãe.

Um uso típico do DMA ocorre na cópia de blocs de memória da RAM do sistema para um buffer de dispositivo.
Estas operações não bloqueiam o processador que fica livra para realizar outras tarefas.
Transferências DMA são essenciais para sistemas embarcados de alto desempenho.

Justamente por serem **muito lentos**, os canais de DMA são utilizados apenas por periféricos lentos, comom drives de disquete, placas de som.
Periféricos mais rápidos, como discos rígidos, utilizam o Bus Mastering, uma espécia de DMA melhorado.

# Interrupção

[fonte](https://pt.m.wikipedia.org/wiki/Interrup%C3%A7%C3%A3o)

Processadores atuais incluem mecanismos para o tratamento de situações especiais, conhecidas como interrupções.
Em uma interrupção, o fluxo normal de instruções é interrompido para que a causa da interrupção seja tratada.

O processador pode auto-interromper-se para tratar exceções de execução, tais como um erro em uma operação aritmética, uma tentativa de execução de instrução ilegal ou uma falha de página em memória virtual.

Interrupções podem ser categorizadas em: *maskable interrupt*, *non-maskable interrupt*, *inter-processor interrupt*, *software interrupt* e *spurios interrupt*.

* **Maskable interrupt (IRQ)**: é uma interrupção de hardware que pode ser ignorada por configurar um bit em um registro da máscara de interrupção bit-mask.
* **Non-maskable interrupt(NMI)**: é uma interrupção de hardware que carece um bit-mask associado, então isto nunca pode ser ignorado. São frequentemento usados por timers.
* **Inter-processor interrupt (IPI)**: é uma caso especial que é gerado por um processador para interromper outro procssador em um sistema de multiprocessadores.
* **Software interrupt**: é uma interrupção gerada dentro de um processador pela execução de uma instrução.
* **Spurious interrupt**: é uma interrupção de hardware que é indesejável. Elas são tipicamente geradas por condições do sistema, tais como interferência elétrica.

Os processadores têm tipicamente uma máscara interna de interrupção com o que permite o software ignorar toda interrupção de hardware externo ao mesmo tempo que é definido.
Esta máscara pode oferecer acesso mais rápido do que acessar o registro da máscara de interrupção em um PIC, ou desabilitar as interrupções do próprio dispositivo.

Uma interrupção que deixa a máquina em um estado bem-definido é chamada de **interrupção precisa**. Como uma interrupção, tem 4 propriedades:

1. O PC (Program Counter) é salvo em um lugar conhecido.
2. Todas as intruções antes apontadas pelo PC tem total execução.
3. Sem instruções além do apontado e que está sendo executado pelo PC.
4. O estado de execção da instrução apontada pelo PC é conhecida.

Uma interrupção que não atenda a esse requerimento é chamada e **interrução imprecisa**.

# Memória

Memória são todos os dispositivos que permitem a um computador guardar dados, temporária ou permanentemente.
A unidade básica de memória é o dígito binário, ou bit.

Basicamente existem dois tipos de memórias: *memórias voláteis*, que perdem seus dados com ausência de energia como a memória cache, registradora, memória de acesso aleatória (RAM).
As memórias flash, disco rígido (HD), são memórias não voláteis, que não perdem seus dados na ausência de energia.

### Memória principal

São memórias que o processador pode endereçar diretamente, sem as quais o computador não pode funcionar.
Fornecem uma ponte para as secundárias, mas a sua função principal é a de conter a informação necessárias para o processador num determinado momento.

Nesta categoria insere-se a RAM, que é uma memória de semicondutores, volátil, com acesso aleatório, isto é, palavras individuais de memória são acessadas diretamente, utilizando uma lógica de endereçãmento implementada em hardware.

### Memória secundária

São memórias utilizadas para armazenamento permanente de dados.
Não podem ser endereçadas diretamente, a infomação precisa ser carregada em memória principal antes de poder ser tratada pelo processador.
Não são estritamente necessárias para a operação do computador.

# RAID (Redundant Array of Independent Disks)

RAID é um memio de se criar um subsistema de armazenamento composto por vários discos individuais, com a finalidade de ganhar segurança (por meio da redundância de dados) e desempenho.
Popularmente, RAID seriam dois ou mais discos  trabalhando simultaneamente para um mesmo fim, por exemplo, citando o exemplo de RAID 1, serviria como um espelhamento simples, rápido e confiável entre dois discos, para se fazer uma cópia idêntiva de um disco em outro.

O RAID oferece segurança e confiabilidade por meio da adição de redundãncia. Se um disco falhar, o outro continua funcionando normalmente e o usuário nem percebe diferença.
O administrador é avisado pelo sistema e substitui o disco que falhou.
Apesar disso, o RAID não protege contra falhas de energia ou erros de operação ou contra a falha simultânea dos dois discos. Falhas de energia, código errado de núcleo ou error operacionais podem danificar os dados de forma irrecuperável. Por este motivo, mesmo usando-se o RAID não se disponse a tradicional cópia de backup.

### Implementação via software

A configuração do RAID via software é feita pelo sistema operacional, que precisa ser implementado no próprio núcleo a utilização de RAIDs via software.
Esta alternativa acaba tornando-se barata.

Todo o processamento necessário para o gerenciamento do RAID é feito pela CPU.
Toda movimentação de dados (leitura e escrita) é feita por uma camada de software que faz a abstração entre a operação lógica e os discos físicos.

### Implementação via hardware

Controladoras RAID em hardware usam layouts de disco proprietários (e diferentes). Por isso, normalmente não é possível misturar controladoras de fabricantes diferentes.

Uma implementação de RAID em hardware requer pelo menos uma controladora especialmente dedicada para isso. Em um pc comum isso pode ser uma placa de expansão PCI, PCI-e ou uma placa integrada à placa-mãe.

A maioria das implementações em hardware proveem cache de leitura e escrita, o que melhora a perfomance.
Na maioria dos casos, o cache de escrita é não volátil, e portanto, escritas pendentes não são perdidas no caso de uma falha no suprimento de energia.
Implementações em hardware promovem performance garantida, não sobrecarregam o processador e podem suportar vários sistemas operacionais, já que a controladora apresentará ao sistema operacional um disco simples.

### Níveis padrão de RAID

#### RAID 0 (striping)

No striping, ou distribuição, os dados são subdivididos em segmentos consecutivo que são escritos sequencialmente através de cada um dos discos de um array.
A distribuição oferece melhor desempenho comparado a discos individuais, se o tamanho de cada segmento for ajustado de acordo com a aplicação que utilizará o conjunto.

###### Vantagens

* Acesso rápido as informações
* Custo baixo para expansão de memória

###### Desvantagens

* Caso algum dos setores de agum dos HDs venha a apresentar perda de informações, o mesmo arquivos que está dividido entre os mesmos setores dos demais HDs não terão mais sentido existir, pois uma parte do arquivo foi corrompida, ou seja, caso algum disco falhe, não tem como recuperar.
* Não tem espelhamento
* Não é usada paridade

#### RAID 1 (mirroring)

RAID-1 é o nível que implementa o espelhamento de disco. Para esta implementação são necessários dois discos ou mais.
O funcionamento deste nível é simples: todos os dados são gravados em discos diferentes, se um disco falhar ou for removido, os dados preservados no outro disco permitem a não descontinuidade da operação do sistema.

###### Vantagens

* Caso algum setor de um dos discos venha a falhar, basta recuperar o setor defeituoso copiando os arquivos contidos do segundo disco
* Segurança nos dados

###### Desvantagens

* Custo relativamente alto se comparado ao RAID-0
* Ocorre aumento no tempo de escrita
* Tem espelhamento
* Não é usada paridade

#### RAID 2

Surgiu no final dos anos 80, quando os HDs ainda não possuíam checagem de erros.
Assim, pode-se dizer que o RAID-2 é similar ao RAID-0, mas possuindo algoritmos de Hamming ECC (Error Correcting Code).

###### Vantagens

* Usa ECC, diminuindo a quase zero as taxas de erro, mesmo com falhas de energia

###### Desvantagens

* Há tecnologias melhores para o mesmo fim.
* Dependendo da configuração há desperdício de espaço que poderia ser usado para dados.

#### RAID 3

É uma versão simplificada do RAID-2.
Nesse arranjo, um único bit de paridade é computado para cada palabra de dados e escrito em um drive de paridade.
Pode parecer que um único bit de paridade dá somente detecção e não correção de erro.
Todavia, para o caso de uma falha de drive, ela provê correção total de errors de um bit, uma vez que a posição do bit defeituoso é conhecida.
Se um drive falhar, o controlador apenas finge que todos os seus bits são "zeros".

A fim de evitar o atraso em razão da latência rotacional, o RAID-3 exige que todos os eixos das unidades de disco estejam sincronizados.
A maioria das unidades de disco mas recentes não possuem a op~ao de sincronização do eixo, ou se são capazers disto, faltam os conectores necessários, cabos e documentação do fabricante.

###### Vantagens

* Leitura rápida
* Escrita rápida
* Possui controle de erros

###### Desvantagens

* Montagem difícil via software
