fn main(){
    
    let original:u32 = 0xc5;
    let complementar = !original;
    
    //Exemplo na página 5

    println!("Operador Complementar (32 bits)\n");
    println!("Qual será o complemento de {:#x} ?", original);
    println!("Representação em bits é: {:032b}", original);
    println!("Complemento é: {:032b}", complementar);
    println!("Que corresponde à: {:#x}", complementar);
    println!("Ou seja: ~{:#x} = {:#x}", original, complementar);
    
}