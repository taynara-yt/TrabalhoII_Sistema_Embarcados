fn main() {

    let valor = 0xC5u32;
    let com = !valor;
     
     println!("Representação em bits é: {:#032b}", valor);
     println!("Complemento é: {:b}", com);
     println!("Que corresponde a: {:#x}", com);
 }
 