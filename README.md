# Análise de Frequência (Cifra de César)

Este projeto implementa uma análise de frequência para tentar quebrar a Cifra de César. A Cifra de César é um método simples de criptografia de substituição, onde cada letra do texto é deslocada um certo número de posições para frente no alfabeto. A análise de frequência explora a ocorrência estatística de letras em um idioma para deduzir a chave de criptografia.

## Funcionalidades

*   Análise de Frequência
*   Plotagem de Distribuição
*   Quebra da Cifra de César

## Como funciona

A análise de frequência se baseia no fato de que, em muitos idiomas, algumas letras aparecem com mais frequência do que outras. Em inglês, por exemplo, a letra 'E' é a mais comum. Ao analisar a frequência das letras em um texto cifrado com a Cifra de César, podemos assumir que a letra mais frequente no texto cifrado provavelmente corresponde à letra 'E' no texto original. A diferença entre as posições dessas letras no alfabeto nos dá uma possível chave para a decriptação.

O código realiza os seguintes passos:

1.  Calcula a frequência de cada letra no texto cifrado.
2.  Encontra a letra mais frequente.
3.  Calcula a diferença entre a posição da letra mais frequente e a posição da letra 'E' no alfabeto. Essa diferença é considerada a chave potencial.

## Código (Pseudo-código)

```
funcao analise_frequencia(texto):
  inicializa um mapa de frequencias para cada letra do alfabeto com 0
  para cada letra no texto:
    se a letra estiver no alfabeto:
      incrementa a frequencia da letra no mapa
  retorna o mapa de frequencias

funcao quebrar_cifra_cesar(texto_cifrado):
  frequencias = analise_frequencia(texto_cifrado)
  letra_mais_frequente = encontra a letra com maior frequencia em frequencias
  chave = (indice da letra_mais_frequente - indice de 'E') modulo tamanho do alfabeto
  imprime a letra mais frequente e a chave potencial
```

## Execução

```bash
cargo run
```
