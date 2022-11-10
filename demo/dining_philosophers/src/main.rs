
struct Philosopher{
    name:String,
}
impl Philosopher{
    fn new(name: &str)->Philosopher{
        Philosopher{
            name: name.to_string(),
        }
    }
}
fn main() {
    println!("Hello, world!");
    let p1 = Philosopher::new("孔子");
}
