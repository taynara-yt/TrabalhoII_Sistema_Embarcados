/*
Outro exemplo, eu quero trocar os bits 5,4
e 3 para o valor “101” e mantendo os outros bits intactos.
– B = (( B & M1) | M2) – Onde M1=11100011 e M2=00010100.*/


fn main() {
    
    let b = 0xafu8;
    let masc1 = 0xe3u8;
    let a = b & masc1;
    let masc2 = 0x14u8;
    let c = a | masc2;
     
     println!("Máscara");
     println!("Vamos verificar a validade:");
     println!("Representação de B: {:#x}  = {:08b}", b, b);
     println!("Masc1: {:#x} = {:08b}", masc1, masc1);
     println!("B & Masc1: {:#x} = {:08b}", a, a);
     println!("Masc2: {:#x} = {:08b}", masc2, masc2);
     println!("((B & M1) | M2): {:#x} = {:08b}", c, c);
 }
