fn main() {
    println!("just estudy");
    let t: i32 = int32(2);
    println!("aqui => {}", t);
    let s = String::from("pedro");
    let a = sten(&s);
    println!("aa => {}", a);
}

fn int32(i: i32) -> i32 {
    i * 2
}
fn sten(nome: &str) -> &str{
    nome
}