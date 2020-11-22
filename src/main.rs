fn vectors_and_loops() {
    // declare mutable vector with prefilled values
    let mut v: Vec<i32> = vec![1,2,3,4];
    v.push(5);
    v.pop();
    v[0] = 0;

    println!("v: {:?}", v);


    // declare immutable variable
    let vec_length = 10;

    // declare new mutable vector
    let mut v2: Vec<i32> = Vec::new();
    // resize with fixed size and capacity to avoid reallocating memory
    v2.resize(vec_length, 0); // vec.resize(vec_length, inital_value);
    for i in 0..v2.len() {
        v2[i] = i as i32;
    }
    println!("v2: {:?}", v2);

    // declare with fixed capcity to avoid reallocating memory
    let mut v3: Vec<i32> = Vec::with_capacity(vec_length);
    for i in 0..vec_length {
        v3.push(i as i32); // cast from usize to i32
    }
    println!("v3: {:?}", v3);

    // range loop
    for _x in v3.iter() {
        // pass
    }
    // mutable range loop
    for x in v3.iter_mut() {
        *x *= 2; 
    }
    println!("v3=v3*2: {:?}", v3);

    // slice by indices
    let slice: &[i32] = &v3[1..5];
    println!("v3[1..5]: {:?}", slice);





}

fn main() {
    println!("Hello, world!");

    vectors_and_loops();
}
