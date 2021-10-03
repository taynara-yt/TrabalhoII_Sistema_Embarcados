fn main(){
    let a:u16 = 0x6DB7;
    let b:u16 = a << 6;
    
    //Reproduzindo exemplo do slide 36
    
    println!("\t\tOperadores de Deslocamento\n");
    println!("Deslocando bits à esquerda\n");
    println!("a = {:#x}",a);
    println!("a (bin) = {:016b}",a);
    println!("b (bin) = {:016b}",b);
    println!("b = {:#x}",b);
}