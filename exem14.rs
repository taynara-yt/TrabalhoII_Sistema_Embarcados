fn main(){
    
    let a:u16 = 0x6db7;
    let m:u16 = 0xff00;
    let b:u16 = a | m;
    
    //Exemplo na página 22

    println!("Máscara\n");
    println!("Suponha, agora, que desejamos transformar a representação binária\nde a em outra representação binária, na qual os 8 bits mais\nà esquerda são todos 1s e os 8 bits mais à direita permanecem\ncom os seus valores originais.\n");
    println!("a = {:016b} = {:#x}", a, a);
    println!("M = {:016b} = {:#x}", m, m);
    println!("b = {:016b} = {:#x}", b, b);
    
}