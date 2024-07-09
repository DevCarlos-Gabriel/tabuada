use std::io;

fn main() {
    println!("Digite o número e a operação que você quer e eu vou gerá-la.\n\
    Para que a tabuada seja gerada corretamente você precisa informar:\n\
  - O número que você deseja saber a tabuada;\n\
  - A operação matemática que você deseja utilizar (+, -, *, / ou todas juntas);\n\
  - E até quando esse número será calculado.\n\
  \n");

  println!("Digite um número:\n");
  
    let mut num = String::new();
  
    io::stdin()
      .read_line(&mut num)
      .expect("Erro ao pegar número do usuário!");

    let num:i128= match num.trim().parse()
      {
        Ok(n)=>n,
        Err(_)=>
          {
            println!("Erro: Informe um número!"); return;
          }
      };

  println!("\nDigite a operação matemática que você quer usar (+, -, *, / ou todas):\n");

    let mut op = String::new();

    io::stdin()
      .read_line(&mut op)
      .expect("Erroa ao pegar a operação do usuário!");

  println!("\nDigite o fim da tabuada:\n");
  
    let mut end = String::new();

    io::stdin()
      .read_line(&mut end)
      .expect("Erro ao pegar o end do usuário!");

    let end: i128 = match end.trim().parse()
      {
        Ok(end)=>end,
        Err(_)=>
          {
            println!("Erro: Informe um número válido!");
            return;}
      };

  for iterator in 1..=end
    {
      match op.trim()
        {
          "+"=>
            {
              println!("{} + {} = {}", num, iterator, num+iterator);
            },
          "-"=>
            {
              println!("{} - {} = {}", num, iterator, num-iterator);
            },
          "*"=>
            {
              println!("{} * {} = {}", num, iterator, num*iterator);
            },
          "/"=>
            {
              if iterator != 0
              {
                println!("{} / {} = {}", num, iterator, num/iterator);
              }
              else
              {
                println!("Erro: Divisão por zero!");
              }
              
            },
          "todas"=>
            {
              let plus = format!("{:<8} + {:<8} = {:<12} ", num, iterator, num+iterator);

              let minus = format!("{:<8} - {:<8} = {:<12} ", num, iterator, num-iterator);

              let mult = format!("{:<8} * {:<8} = {:<12}", num, iterator, num*iterator);
              
              let div = 
              if iterator != 0
              {
                format!("{:<8} / {:<8} = {:<12}", num, iterator, num/iterator)  
              }
              else
              {
                "Erro: Divisão por zero!".to_string()
              };

              let concatena = format!("{}|{}|{}|{}", plus, minus, mult, div);

              println!("{}", concatena);
            }
          _=>
            {
              println!("Operação Inválida!");
            }
          
        };
    }
}
