// clippy2.rs
// Make me compile! Execute `rustlings hint clippy2` for hints :)



fn main() {
    let mut res = 42;
    let option = Some(12);
    while let Some(x)=option {  //if let
        res += x;
    }
    println!("{}", res);
}
