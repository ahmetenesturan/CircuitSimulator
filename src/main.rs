mod circuit;
use circuit::Resistor;
use circuit::Branch;



fn main() {
    let b = Branch::new(0,1);
    let r = Resistor::new(37e3, b);
    println!("{}", r.resistance);
    println!("{}", b.first_node);
}

//Branch icin copy trait gerek