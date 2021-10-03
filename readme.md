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

**Exemplo 11**
~~~c
fn main(){
    
    let a:u16 = 0x6db7;
    let m:u16 = 0x3f;
    let b:u16 = a & m;

    //Exemplo na página 15

    println!("Máscara\n");
    println!("Suponha que a = {:#x} e que queiramos extrair os 6 bits mais\na direita de a e assinalar à variável b. Como fazer isso ?\n", a);
    println!("a = {:016b} = {:#x}", a, a);
    println!("M = {:016b} = {:#x}", m, m);
    println!("b = {:016b} = {:#x}", b, b);
    
}
~~~


**Exemplo 12**
~~~c
fn main(){
    
    let a:u16 = 0x6db7;
    let m:u16 = 0xfc00;
    let b:u16 = a & m;
    
    //Exemplo na página 17

    println!("Máscara\n");
    println!("Suponha, novamente, que a seja uma variável inteira se,\nsinal de valor {:#x}. Agora, extraia os 6 bits mais à\nesquerda desse valor e assinale à variável inteira sem sinal b.\nAssinale 0s aos 10 bits mais à direita de b.\n", a);
    println!("a = {:016b} = {:#x}", a, a);
    println!("M = {:016b} = {:#x}", m, m);
    println!("b = {:016b} = {:#x}", b, b);
    
}
~~~


**Exemplo 13**
~~~c
fn main(){
    
    let a:u16 = 0x6db7;
    let m:u16 = 0xff;
    let b:u16 = a | m;
    
    //Exemplo na página 20

    println!("Máscara\n");
    println!("Suponha que a variável a seja uma inteira sem sinal de valor\n{:#x}, como antes.Transforme a sua correspondente representação\nbinária em uma outra representação binária na qual os 8 bits mais\nà direita são todos 1s e os 8 bits mais à esquerda permanecem\ncom seus valores originais. Assinale essa representação binária\nà variável inteira sem sinal b.\n", a);
    println!("a = {:016b} = {:#x}", a, a);
    println!("M = {:016b} = {:#x}", m, m);
    println!("b = {:016b} = {:#x}", b, b);
    
}
~~~


**Exemplo 14**
~~~c
fn main(){
    
    let a:u16 = 0x6db7;
    let m:u16 = 0xff00;
    let b:u16 = a | m;
    
    //Exemplo na página 22

    println!("Máscara\n");
    println!("Suponha, agora, que desejamos transformar a representação binária\nde a em outra representação binária, na qual os 8 bits mais\nà esquerda são todos 1s e os 8 bits mais à direita permanecem\ncom os seus valores originais.\n");
    println!("a = {:016b} = {:#x}", a, a);
    println!("M = {:016b} = {:#x}", m, m);
    println!("b = {:016b} = {:#x}", b, b);
    
}
~~~


**Exemplo 15**
~~~c
fn main(){
    
    let a:u16 = 0x6db7;
    let m:u16 = 0xff;
    let b:u16 = a ^ m;
    
    //Exemplo na página 24

    println!("Máscara\n");
    println!("Suponha que a seja uma variável inteira sem sinal cujo valor é\n{:#x}. Vamos agora inverter os 8 bits mais à direita e preservar\nos 8 bits mais à esquerda. Essa nova representação binária\nserá assinalada à variável inteira sem sinal b.\n", a);
    println!("a = {:016b} = {:#x}", a, a);
    println!("M = {:016b} = {:#x}", m, m);
    println!("b = {:016b} = {:#x}", b, b);
    
}
~~~


**Exemplo 16**
~~~c
fn main(){
    
    let a:u16 = 0x6db7;
    let m:u16 = 0xff00;
    let b:u16 = a ^ m;
    
    //Exemplo na página 25

    println!("Máscara\n");
    println!("Se quisermos inverter os 8 bits mais à esquerda de a e manter os\n8 bits mais à direita originais:\n");
    println!("a = {:016b} = {:#x}", a, a);
    println!("M = {:016b} = {:#x}", m, m);
    println!("b = {:016b} = {:#x}", b, b);
    
}
~~~


**Exemplo 17**
~~~c
fn main(){
    
    let m:u16 = 0x4;
    let a:u16 = 0x6db7;
    let b:u16 = a ^ m;
    let c:u16 = b ^ m;
    
    //Exemplo na página 27

    println!("Máscara\n");
    println!("Suponha que a seja uma variável inteira sem sinal cujo valor é {:#x}.", a);
    println!("Qual o resultado da expressão a ^ {:#x} ?", m);
    println!("Se essa operação for executada repetidamente,\no valor de a será alternado entre {:#x} e {:#x}.\n", a, c);
    println!("a = {:016b} = {:#x}", a, a);
    println!("M = {:016b} = {:#x}", m, m);
    println!("b = {:016b} = {:#x}", b, b);
    println!("M = {:016b} = {:#x}", m, m);
    println!("c = {:016b} = {:#x}", c, c);
    
}
~~~


**Exemplo 18**
~~~c
fn main(){
    
    let b:u8 = 0xaf;
    let m1:u8 = 0xfc;
    let a:u8 = b & m1;
    let m2:u8 = 0x02;    
    let c:u8 = a|m2;
    
    //Exemplo na página 29

    println!("Máscara\n");
    println!("E se eu quiser alterar alguns bits para um valor pré-estabelecido?");
    println!("Por exemplo, eu quero trocar os dois bits menos significativos\npara o valor 10.\n");
    println!("{:#x} = {:08b} = B", b, b);
    println!("{:#x} = {:08b} = M1", m1, m1);
    println!("{:#x} = {:08b} = B & M1", a, a);
    println!(" {:#x} = {:08b} = M2", m2, m2);
    println!("{:#x} = {:08b} = ((B & M1)|M2)", c, c);
    
}
~~~


**Exemplo 19**
~~~c
fn main(){
    
    let b:u8 = 0xaf;
    let m1:u8 = 0xe3;
    let a:u8 = b & m1;
    let m2:u8 = 0x14;
    let c:u8 = a|m2;
    
    //Exemplo na página 31

    println!("Máscara\n");
    println!("Outro exemplo, eu quero trocar os bits 5, 4 e 3 para o valor 101\ne mantendo os outros bits intactos.\n");
    println!("{:#x} = {:08b} = B", b, b);
    println!("{:#x} = {:08b} = M1", m1, m1);
    println!("{:#x} = {:08b} = B & M1", a, a);
    println!("{:#x} = {:08b} = M2", m2, m2);
    println!("{:#x} = {:08b} = ((B & M1)|M2)", c, c);
    
}
~~~








