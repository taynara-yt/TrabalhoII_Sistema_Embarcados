fn main(){
    
    let a:u16 = 0x6db7;
    let m:u16 = 0xff00;
    let b:u16 = a ^ m;
    
    println!("Máscara\n");
    println!("Se quisermos inverter os 8 bits mais à esquerda de a e manter os\n8 bits mais à direita originais:\n");
    println!("a = {:016b} = {:#x}", a, a);
    println!("M = {:016b} = {:#x}", m, m);
    println!("b = {:016b} = {:#x}", b, b);
    
}