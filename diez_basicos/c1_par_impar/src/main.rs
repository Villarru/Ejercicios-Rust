use std::io;

fn main() {
    println!("Introduce un numero: ");
    let mut num = String::new();

    io::stdin().read_line(&mut num).
        expect(" no se ha podido recibir dato");
    let num: f64 = num.trim().parse().expect("");

    if num%2.0 == 0.0{
        println!("{} es par", num);
    }else if num%1.0 == 0.0 {
        println!("{} es impar", num);
    }else{
        println!("{} no es par", num);
    }
    
    
}
