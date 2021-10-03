  /*
    Suponha que a variável a seja uma inteira sem sinal de valor 0x6DB7, como antes.
    Transforme a sua correspondente representação binária em uma outra representação
    binária na qual os 8 bits mais à direita são todos 1s e os 8 bits mais à esquerda 
    permanecem com seus valores originais. Assinale essa representação binária 
    à variável inteira sem sinal b
    */

fn main() {
    
        let a = 0x6DB7u16;
        let masc = 0xffu16;
        let b = a | masc;
         
         println!("Máscara:");
         println!("Transformação da Variavel {:#x} para nova representação binária na qual os 8 bits mais a direita são 1s e os 8 bits a esquerda permanecem orginais.",a );
         println!("Representação de A: {:#x} em bits = {:016b}", a, a);
         println!("Máscara: {:#x} = {:016b}", masc, masc);
         println!("Representação de B: {:#x} em bits = {:016b}", b, b);
     }
