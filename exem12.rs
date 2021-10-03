fn main(){
    
    let a:u16 = 0x6db7;
    let m:u16 = 0xfc00;
    let b:u16 = a & m;
    
    //Exemplo na página 17

    println!("Máscara\n");
    println!("Suponha, novamente, que a seja uma variável inteira se,\nsinal de valor {:#x}. Agora, extraia os 6 bits mais à\nesquerda desse valor e assinale à variável inteira sem sinal b.\nAssinale 0s aos 10 bits mais à direita de b.\n", a);
    println!("a = {:016b} = {:#x}", a, a);
    println!("M = {:016b} = {:#x}", m, m);
    println!("b = {:016b} = {:#x}", b, b);
    
}