/*
E se eu quiser alterar alguns bits para um
valor pré-estabelecido?
• Por exemplo, eu quero trocar os dois bits
menos significativos para o valor.*/


fn main() {
    
    let b = 0xafu8;
    let masc1 = 0xfcu8;
    let a = b & masc1;
    let masc2 = 0x02u8;
    let c = a | masc2;
     
     println!("Máscara");
     println!("Vamos verificar a validade:");
     println!("Representação de B: {:#x}  = {:#08b}", b, b);
     println!("Masc1: {:#x} = {:#08b}", masc1, masc1);
     println!("B & Masc1: {:#x} = {:#08b}", a, a);
     println!("Masc2: {:#x} = {:#08b}", masc2, masc2);
     println!("((B & M1) | M2): {:#x} = {:#08b}", c, c);
 }