fn main() {

    let valor = 0x5b3cu32;
    let com = !valor;
     
     println!("Representação em bits é: {:#032b}", valor);
     println!("Complemento é: {:b}", com);
     println!("Que corresponde a: {:#x}", com);
 }
 