fn main() {

   let a = 0x6DB7u16;
   let b = 0xA726u16;
    
    println!("Representação em bits é: {:#018b}", a);
    println!("Representação em bits é: {:#018b}", b);
    
    println!("A&B é: {:#018b}", (a | b));
    println!("Que corresponde a: {:x}", (a | b));
}

