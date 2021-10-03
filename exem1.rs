fn main(){
    let valor = 0x7ffu16;
    let com = !valor;
    
    println!("Exemplo(considerando palavra de 16 bits): ");
    println!("Representação em bits é: {:#018b}", valor);    
    println!("Complemento é: {:b}", com);
    println!("Que corresponde a: {:#x}", com);
}
