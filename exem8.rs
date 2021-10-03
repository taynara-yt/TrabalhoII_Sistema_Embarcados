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
