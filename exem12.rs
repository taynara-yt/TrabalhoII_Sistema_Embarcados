  /*
Suponha, novamente, que a seja uma variável inteira
sem sinal de valor 0x6DB7. Agora, extraia os 6 bits mais
à esquerda desse valor e assinale à variável inteira sem
sinal b. Assinale 0s aos 10 bits mais à direita de b. • Para executar essa operação podemos escrever a
expressão bitwise b=a & 0xFC00

 Assim, a constante 0xFC00 servirá como máscara

 O valor de b será 0x6C00.
    */

fn main() {

    let a = 0x6DB7u16;
    let masc = 0xfc00u16;
    let b = a & masc;
     
     println!("Máscara:");
     println!("Extrair os 6 bits mais a esquerda de 0x6DB7 e assinale à variável inteira sem sinal b. Assinale 0s aos 10 bits mais à direita de b." );
     println!("Representação de A: {:#x} em bits = {:016b}", a, a);
     println!("Máscara: {:#x} = {:016b}", masc, masc);
     println!("Representação de B: {:#x} em bits = {:016b}", b, b);
 }
