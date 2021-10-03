fn main(){
    
    let x:u8 = 128;
    let x0:u8 = x >> 0;
    let x1:u8 = x >> 1;
    let x2:u8 = x >> 2;
    let x3:u8 = x >> 3;
    let x4:u8 = x >> 4;
    let x5:u8 = x >> 5;
    let x6:u8 = x >> 6;
    let x7:u8 = x >> 7;
    
    //Reproduzindo exemplo do slide 34
    
    println!("Operadores de Deslocamento\n");
    println!("Deslocando bits à direita\n");
    println!("{:08b}",x0);
    println!("{:08b}",x1);
    println!("{:08b}",x2);
    println!("{:08b}",x3);
    println!("{:08b}",x4);
    println!("{:08b}",x5);
    println!("{:08b}",x6);
    println!("{:08b}",x7);
}