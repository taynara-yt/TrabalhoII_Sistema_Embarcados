fn main(){
    
    let b:u8 = 0xaf;
    let m1:u8 = 0xfc;
    let a:u8 = b & m1;
    let m2:u8 = 0x02;    
    let c:u8 = a|m2;
    
    //Exemplo na página 29

    println!("Máscara\n");
    println!("E se eu quiser alterar alguns bits para um valor pré-estabelecido?");
    println!("Por exemplo, eu quero trocar os dois bits menos significativos\npara o valor 10.\n");
    println!("{:#x} = {:08b} = B", b, b);
    println!("{:#x} = {:08b} = M1", m1, m1);
    println!("{:#x} = {:08b} = B & M1", a, a);
    println!(" {:#x} = {:08b} = M2", m2, m2);
    println!("{:#x} = {:08b} = ((B & M1)|M2)", c, c);
    
}