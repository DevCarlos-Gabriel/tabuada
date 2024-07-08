# Calculadora de Tabuadas 🧮

Este projeto é uma implementação simples de um programa em Rust que gera a tabuada de um número com base na operação matemática escolhida pelo usuário. O programa pode calcular a tabuada de um número utilizando adição, subtração, multiplicação, divisão ou todas as operações juntas.

## Funcionalidades ✨

- **Tabuada de Adição**: Calcula a tabuada de adição para um número especificado.
- **Tabuada de Subtração**: Calcula a tabuada de subtração para um número especificado.
- **Tabuada de Multiplicação**: Calcula a tabuada de multiplicação para um número especificado.
- **Tabuada de Divisão**: Calcula a tabuada de divisão para um número especificado.
- **Todas as Operações**: Calcula a tabuada utilizando todas as operações matemáticas.

## Como Usar 🚀

1. **Clone o repositório:**
    ```sh
    git clone https://github.com/DevCarlos-Gabriel/tabuada.git tabuada
    cd tabuada
    ```

2. **Compile o programa:**
    ```sh
    cargo build
    ```

3. **Execute o programa:**
    ```sh
    cargo run
    ```

## Exemplos de Uso 📋

Ao iniciar o programa, você verá o seguinte prompt:

```sh
Digite o número e a operação que você quer e eu vou gerá-la.
Para que a tabuada seja gerada corretamente você precisa informar:
- O número que você deseja saber a tabuada;
- A operação matemática que você deseja utilizar (+, -, *, / ou todas juntas);
- E até quando esse número será calculado.

Digite um número:
```

### Gerando a Tabuada 🔢

1. Digite o número para o qual deseja gerar a tabuada e pressione Enter;
2. Insira a operação matemática desejada (+, -, *, / ou todas) e pressione Enter;
3. Insira o valor final até onde a tabuada será calculada e pressione Enter.

Exemplo de entrada:

```sh
Digite um número:
5

Digite a operação matemática que você quer usar (+, -, *, / ou todas):
todas

Digite o fim da tabuada:
10
```

Saída esperada:

```sh
5        + 1        = 6            |5        - 1        = 4            |5        * 1        = 5           |5        / 1        = 5           
5        + 2        = 7            |5        - 2        = 3            |5        * 2        = 10          |5        / 2        = 2           
5        + 3        = 8            |5        - 3        = 2            |5        * 3        = 15          |5        / 3        = 1           
5        + 4        = 9            |5        - 4        = 1            |5        * 4        = 20          |5        / 4        = 1           
5        + 5        = 10           |5        - 5        = 0            |5        * 5        = 25          |5        / 5        = 1           
5        + 6        = 11           |5        - 6        = -1           |5        * 6        = 30          |5        / 6        = 0           
5        + 7        = 12           |5        - 7        = -2           |5        * 7        = 35          |5        / 7        = 0           
5        + 8        = 13           |5        - 8        = -3           |5        * 8        = 40          |5        / 8        = 0           
5        + 9        = 14           |5        - 9        = -4           |5        * 9        = 45          |5        / 9        = 0           
5        + 10       = 15           |5        - 10       = -5           |5        * 10       = 50          |5        / 10       = 0
```

### Estrutura do Código 📂

O código é estruturado da seguinte forma:

1. Entrada do Usuário: Captura o número, a operação e o limite da tabuada.
2. Cálculo da Tabuada: Realiza o cálculo da tabuada com base na operação escolhida.
3. Saída: Exibe a tabuada calculada.

## Considerações Finais 📝

Contribuições são bem-vindas! Sinta-se à vontade para abrir um PR ou relatar problemas.

## Licença 📄

Este projeto está licenciado sob a Licença MIT - veja o arquivo [LICENSE](https://github.com/DevCarlos-Gabriel/tabuada/blob/main/LICENSE) para mais detalhes.
