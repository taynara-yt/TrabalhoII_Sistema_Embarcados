fn main(){
    
    let a:u16 = 0x6db7;
    let b:u16 = 0xa726;
    let ab = a|b;

    //Exemplo na página 12
    
    println!("Operador Bitwise Lógicos\n");
    println!("Suponha que a e b sejam variáveis inteiras\nsem sinal de valores {:#x} e {:#x}.", a, b);
    println!("Qual o valor de a | b ?");
    println!("  a = {:016b} = {:#x}", a, a);
    println!("  b = {:016b} = {:#x}", b, b);
    println!("a|b = {:016b} = {:#x}", ab, ab);
    
}