// Fibonaccin ensimmäiset 10 lukua:  0, 1, 1, 2, 3, 5, 8, 13, 21, 34

fn fibonacci(n: u32) -> u32 {

    if n == 0 {
        return 0
    } else if n == 1 {
        return 1
    } else {
        return fibonacci(n-1) + fibonacci(n-2)
    } 
}

fn main() {
    for i  in 0..10 {
        println!("Fibonacci luku nro {}", i + 1);
        println!("{}", fibonacci(i));  
    }
}