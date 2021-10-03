  /*
    Suponha que a=0x6DB7 e que queiramos extrair os 6
bits mais a direita de a e assinalar à variável b. Como
fazer isso?
    */

fn main() {

    let a = 0x6DB7u16;
    let masc = 0x3fu16;
    let b = a & masc;
     
     println!("Máscara:");
     println!("Extrair os 6 bits mais a direita de A e assinalar a variavel A");
     println!("Representação de A: {:#x} em bits = {:016b}", a, a);
     println!("Máscara: {:#x} = {:016b}", masc, masc);
     println!("Representação de B: {:#x} em bits = {:016b}", b, b);
    
 }
