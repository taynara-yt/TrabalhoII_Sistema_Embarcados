
/*
Suponha que a seja uma variável inteira sem sinal
cujo valor é 0x6DB7, como nos exemplos
anteriores.
• Qual o resultado da expressão a ^ 0x4?
   – inverterá o valor do bit número 2 (o terceiro bit a partir da
    direita) de a.
• Se essa operação for executada repetidamente, o
valor de a será alternado entre 0x6DB7 e 0x6DB3
• Nesse caso, o terceiro bit será “ligado” e
“desligado” alternadamente.*/


fn main() {
    
    let a = 0x6DB7u16;
    let masc = 0x4u16;
    let b = a ^ masc;
    let c = b ^ masc;
     
     println!("Máscara");
     println!("Vamos verificar a validade:");
     println!("Representação de A: {:#x}  = {:#018b}", a, a);
     println!("Máscara: {:#x} = {:#018b}", masc, masc);
     println!("Representação de B: {:#x} = {:#018b}", b, b);
     println!("Máscara: {:#x} = {:#018b}", masc, masc);
     println!("Representação de C: {:#x} = {:#018b}", c, c);
 }
