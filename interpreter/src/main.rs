#[derive(Copy, Clone)]
enum Primitive {
    Add,
    Multiply,
    Subtract,
    Number(i32),
}

fn evaluate(primitives: Vec<Primitive>) -> i32 {
    let firstElement = &primitives[0];
    let mut iter = primitives.iter();
    iter.next();
    match firstElement {
        Primitive::Add => iter.fold(0, |t, n| t + evaluate(vec![*n])),
        Primitive::Multiply => iter.fold(1, |p, n| p * evaluate(vec![*n])),
        //two's complement seems to work maybe? hopefully
        Primitive::Subtract => {
            let fnum = evaluate(vec![*iter.next().unwrap()]);
            iter.fold(fnum, |t,n| t + (!evaluate(vec![*n])+ 1))
        } 
        Primitive::Number(val) => *val,
    }
}

fn main() {
    let mut primitives = Vec::<Primitive>::new();
    primitives.push(Primitive::Subtract);
    primitives.push(Primitive::Number(5));
    primitives.push(Primitive::Number(5));
    primitives.push(Primitive::Number(5));
    let result = evaluate(primitives);
    println!("The result is {result}");
}
