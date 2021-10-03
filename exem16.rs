fn main() {

    let a = 0x6DB7u16;
    let masc = 0xff00u16;
    let b = a ^ masc;
     
     println!("Máscara:");
     println!("Se quisermos inverter os 8 bits mais à esquerda de a e manter os 8 bits mais à direita originais:");
     println!("Representação de A: {:#x}  = {:#018b}", a, a);
     println!("Máscara: {:#x} = {:#018b}", masc, masc);
     println!("Representação de B: {:#x} = {:#018b}", b, b);
 }