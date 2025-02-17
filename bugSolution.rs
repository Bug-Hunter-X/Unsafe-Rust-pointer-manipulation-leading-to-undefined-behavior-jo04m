fn main() {
    let mut v = vec![1, 2, 3];
    v[0] = 10; // Safe way to modify vector element
    println!("v: {:?}", v);
    
    //Alternative using iterators:
    let mut v2 = vec![1, 2, 3];
    v2.iter_mut().for_each(|x| *x *= 2);
    println!("v2: {:?}", v2);
} 