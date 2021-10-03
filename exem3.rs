fn main() {

    let valor = 0x1111u32;
    let com = !valor;
     
     println!("Exemplo(considerando palavra de 32 bits): ");
     println!("Representação em bits é: {:#032b}", valor);
     println!("Complemento é: {:b}", com);
     println!("Que corresponde a: {:#x}", com);
 }
