# Trabalho Prático II - Rust
## Sobre este repositório

Repositório destinado a disciplina Sistemas Embarcados, ministrado pelo **Prof. Vandermi Joao da Silva**.
Aqui estará todas as implementações dos exemplos do slide "Programação de Baixo Nível em C" feitas em Rust.


### Discentes

- Adriano Gomes, 21751229;
- Rafael Guedes, 21542175;
- Taynara Silva, 21751227

### Resolução
**Exemplo 1**
~~~c
fn main(){
    let valor = 0x7ffu16;
    let com = !valor;
    
    println!("Exemplo(considerando palavra de 16 bits): ");
    println!("Representação em bits é: {:016b}", valor);    
    println!("Complemento é: {:b}", com);
    println!("Que corresponde a: {:#x}", com);
}
~~~

**Exemplo 2**
~~~c
fn main() {

    let valor = 0xC5u32;
    let com = !valor;

     println!("Exemplo(considerando palavra de 32 bits): ");
     println!("Representação em bits é: {:032b}", valor);     
     println!("Complemento é: {:b}", com);
     println!("Que corresponde a: {:#x}", com);
 }
~~~

**Exemplo 3**
~~~c
fn main() {

    let valor = 0x1111u32;
    let com = !valor;
     
     println!("Exemplo(considerando palavra de 32 bits): ");
     println!("Representação em bits é: {:032b}", valor);
     println!("Complemento é: {:b}", com);
     println!("Que corresponde a: {:#x}", com);
 }
 
~~~

**Exemplo 4**
~~~c
fn main() {

    let valor = 0xffffu32;
    let com = !valor;
     
     println!("Exemplo(considerando palavra de 32 bits): ");
     println!("Representação em bits é: {:032b}", valor);
     println!("Complemento é: {:b}", com);
     println!("Que corresponde a: {:#x}", com);
 }
~~~

**Exemplo 5**
~~~c
fn main() {

    let valor = 0x5b3cu32;
    let com = !valor;
     
     println!("Exemplo(considerando palavra de 32 bits): ");
     println!("Representação em bits é: {:032b}", valor);
     println!("Complemento é: {:b}", com);
     println!("Que corresponde a: {:#x}", com);
 }
~~~

**Exemplo 6**
~~~c
fn main() {

    let a = 0x6DB7u16;
    
     println!("Operadores Bitwise Lógicos:");
     println!("Qual o valor de ~a ?");
     println!("Representação em bits é: {:016b}", a);
     println!("a~: {:b}", !a);
     println!("Que corresponde à: {:#x}", !a);
         
 }
~~~

**Exemplo 7**
~~~c
fn main() {

    let b = 0xA726u16;
    
     println!("Operadores Bitwise Lógicos:");
     println!("Qual o valor de ~b ?");
     println!("Representação em bits é: {:016b}", b);
     println!("b~: {:b}", !b);
     println!("Que corresponde à: {:#x}", !b);
     
 }
~~~

**Exemplo 8**
~~~c
fn main() {

    let a = 0x6DB7u16;
    let b = 0xA726u16;
     
     println!("Operadores Bitwise Lógicos:");
     println!("Qual o valor de A & B ?");
     println!("Representação em bits é: {:016b}", a);
     println!("Representação em bits é: {:016b}", b);
     println!("A&B é: {:016b}", (a & b));
     println!("Que corresponde à: {:#x}", (a & b));
 }
~~~

**Exemplo 9**
~~~c
fn main() {

    let a = 0x6DB7u16;
    let b = 0xA726u16;
     
     println!("Operadores Bitwise Lógicos:");
     println!("Qual o valor de A ^ B ?");
     println!("Representação em bits é: {:016b}", a);
     println!("Representação em bits é: {:016b}", b);
     println!("A^B é: {:016b}", (a ^ b));
     println!("Que corresponde à: {:#x}", (a ^ b));
 }
~~~

**Exemplo 10**
~~~c
fn main() {

    let a = 0x6DB7u16;
    let b = 0xA726u16;
     
     println!("Operadores Bitwise Lógicos:");
     println!("Qual o valor de A | B ?");
     println!("Representação em bits é: {:016b}", a);
     println!("Representação em bits é: {:016b}", b);     
     println!("A|B é: {:016b}", (a | b));
     println!("Que corresponde à: {:#x}", (a | b));
 }

~~~
