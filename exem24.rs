fn main(){
    
    let p1:u16 = 0x7f;
    let p2 = 5;
    let root:u16 = 0x6db7;
    let mut a:u16 = original;
    
    //Reproduzindo resultados do slide 39
    
    println!("\t\tOperadores Bitwise de Atribuição\n");
    println!("Expressão |  Equivalente | Valor Final\n");
    
    a = a & p1;
    println!("a &= {:#x} | a = a & {:#x} | {:#x}", p1, p1, a);
    
    a = root;
    a = a ^ p1;
    println!("a ^= {:#x} | a = a ^ {:#x} | {:#x}", p1, p1, a);
    
    a = root;
    a = a | p1;
    println!("a |= {:#x} | a = a | {:#x} | {:#x}", p1, p1, a);

    a = root;
    a = a << p2;
    println!("a <<= {}   | a = a << {}   | {:#x}", p2, p2, a);

    a = root;
    a = a >> p2;
    println!("a >>= {}   | a = a >> {}   | {:#x}", p2, p2, a);

}