fn main(){
    
    let a:u16 = 0x6db7;
    let m:u16 = 0xff;
    let b:u16 = a | m;
    
    println!("Máscara\n");
    println!("Suponha que a variável a seja uma inteira sem sinal de valor\n{:#x}, como antes.Transforme a sua correspondente representação\nbinária em uma outra representação binária na qual os 8 bits mais\nà direita são todos 1s e os 8 bits mais à esquerda permanecem\ncom seus valores originais. Assinale essa representação binária\nà variável inteira sem sinal b.\n", a);
    println!("a = {:016b} = {:#x}", a, a);
    println!("M = {:016b} = {:#x}", m, m);
    println!("b = {:016b} = {:#x}", b, b);
    
}