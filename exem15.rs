fn main(){
    
    let a:u16 = 0x6db7;
    let m:u16 = 0xff;
    let b:u16 = a ^ m;
    
    //Exemplo na página 24

    println!("Máscara\n");
    println!("Suponha que a seja uma variável inteira sem sinal cujo valor é\n{:#x}. Vamos agora inverter os 8 bits mais à direita e preservar\nos 8 bits mais à esquerda. Essa nova representação binária\nserá assinalada à variável inteira sem sinal b.\n", a);
    println!("a = {:016b} = {:#x}", a, a);
    println!("M = {:016b} = {:#x}", m, m);
    println!("b = {:016b} = {:#x}", b, b);
    
}