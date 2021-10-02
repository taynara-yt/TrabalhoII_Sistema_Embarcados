fn main(){
    
    let original:u16 = 0x7ff;
    let complementar = !original;
    
    println!("Operador Complementar (16 bits)\n");
    println!("Qual será o complemento de {:#x} ?", original);
    println!("Representação em bits é: {:016b}", original);
    println!("Complemento é: {:016b}", complementar);
    println!("Que corresponde à: {:#x}", complementar);
    println!("Ou seja: ~{:#x} = {:#x}", original, complementar);
    
}