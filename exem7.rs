fn main(){
    
    let b:u16 = 0xa726;
    let complementar = !b;
    
    //Exemplo na página 9

    println!("Operador Bitwise Lógicos\n");
    println!("Suponha que b seja variável inteira sem sinal de valores {:#x}.", b);
    println!("Qual o valor de ~b ?");
    println!(" b = {:016b} = {:#x}", b, b);
    println!("~b = {:016b} = {:#x}", complementar, complementar);
    
}