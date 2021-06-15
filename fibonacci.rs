use std::io;

fn main() {
    println!("henlo giv noombre");
    let mut a = String::new(); 
    io::stdin().read_line(&mut a).expect("sususs amogus"); 
    a.pop(); 
    let count = a.parse::<i32>().unwrap();
    println!("printing {} values", count);
    let mut c: i128 = 0; 
    let mut d: i128= 1; 
    print!("1 "); 
    for n in 1..count {
        let mut result: i128 = d + c; 
        c = d; 
        d = result; 
        print!("{} ", result); 
    }
    println!("done"); 
    
}
