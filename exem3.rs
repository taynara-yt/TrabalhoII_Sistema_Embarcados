fn main(){
    
    let original:u32 = 0x1111;
    let complementar = !original;
    
    println!("Operador Complementar (32 bits)\n");
    println!("Qual será o complemento de {:#x} ?", original);
    println!("Representação em bits é: {:032b}", original);
    println!("Complemento é: {:032b}", complementar);
    println!("Que corresponde à: {:#x}", complementar);
    println!("Ou seja: ~{:#x} = {:#x}", original, complementar);
    
}
