fn main(){
    
    let b:u8 = 0xaf;
    let m1:u8 = 0xe3;
    let a:u8 = b & m1;
    let m2:u8 = 0x14;
    let c:u8 = a|m2;
    
    //Exemplo na página 31

    println!("Máscara\n");
    println!("Outro exemplo, eu quero trocar os bits 5, 4 e 3 para o valor 101\ne mantendo os outros bits intactos.\n");
    println!("{:#x} = {:08b} = B", b, b);
    println!("{:#x} = {:08b} = M1", m1, m1);
    println!("{:#x} = {:08b} = B & M1", a, a);
    println!("{:#x} = {:08b} = M2", m2, m2);
    println!("{:#x} = {:08b} = ((B & M1)|M2)", c, c);
    
}