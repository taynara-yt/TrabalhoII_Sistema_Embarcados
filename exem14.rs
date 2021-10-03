  /*
Suponha, agora, que desejamos
transformar a representação binária de
a em outra representação binária, na
qual os 8 bits mais à esquerda são
todos 1s e os 8 bits mais à direita
permanecem com os seus valores
originais.
    */

    fn main() {

        let a = 0x6DB7u16;
        let masc = 0xff00u16;
        let b = a | masc;
         
         println!("Máscara:");
         println!("Representação de A: {:#x} = {:016b}", a, a);
         println!("Máscara: {:#x} = {:016b}", masc, masc);
         println!("Representação de B: {:#x} = {:016b}", b, b);
     }
