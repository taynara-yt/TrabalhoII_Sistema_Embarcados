fn main(){
    
    let m:u16 = 0x4;
    let a:u16 = 0x6db7;
    let b:u16 = a ^ m;
    let c:u16 = b ^ m;
    
    //Exemplo na página 27

    println!("Máscara\n");
    println!("Suponha que a seja uma variável inteira sem sinal cujo valor é {:#x}.", a);
    println!("Qual o resultado da expressão a ^ {:#x} ?", m);
    println!("Se essa operação for executada repetidamente,\no valor de a será alternado entre {:#x} e {:#x}.\n", a, c);
    println!("a = {:016b} = {:#x}", a, a);
    println!("M = {:016b} = {:#x}", m, m);
    println!("b = {:016b} = {:#x}", b, b);
    println!("M = {:016b} = {:#x}", m, m);
    println!("c = {:016b} = {:#x}", c, c);
    
}