 /*
Suponha que a seja uma variável inteira
sem sinal cujo valor é 0x6DB7, como nos
exemplos anteriores. Vamos agora
inverter os 8 bits mais à direita e preservar
os 8 bits mais à esquerda. Essa nova
representação binária será assinalada à
variável inteira sem sinal b. 
    */

    fn main() {

        let a = 0x6DB7u16;
        let masc = 0xffu16;
        let b = a ^ masc;
         
         println!("Máscara:");
         println!("Vamos agora inverter os 8 bits mais à direita e preservar os 8 bits mais à esquerda");
         println!("Representação de A: {:#x}  = {:016b}", a, a);
         println!("Máscara: {:#x} = {:016b}", masc, masc);
         println!("Representação de B: {:#x} = {:016b}", b, b);
     }
