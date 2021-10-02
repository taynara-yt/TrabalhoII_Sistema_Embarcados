fn main(){
    
    let a:u16 = 0x6db7;
    let m:u16 = 0x3f;
    let b:u16 = a & m;

    println!("Máscara\n");
    println!("Suponha que a = {:#x} e que queiramos extrair os 6 bits mais\na direita de a e assinalar à variável b. Como fazer isso ?\n", a);
    println!("a = {:016b} = {:#x}", a, a);
    println!("M = {:016b} = {:#x}", m, m);
    println!("b = {:016b} = {:#x}", b, b);
    
}