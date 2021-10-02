fn main(){
    
    let a:u16 = 0x6db7;
    let complementar = !a;
    
    println!("Operador Bitwise Lógicos\n");
    println!("Suponha que a seja variável inteira sem sinal de valores {:#x}.", a);
    println!("Qual o valor de ~a ?");
    println!(" a = {:016b} = {:#x}", a, a);
    println!("~a = {:016b} = {:#x}", complementar, complementar);
    
}