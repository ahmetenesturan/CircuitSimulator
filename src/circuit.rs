pub struct Resistor{
    resistance:f64,
    branch:Branch
}

pub struct Branch{
    first_node:u16,
    second_node:u16
}

impl Branch {
    pub fn new(first:u16, second:u16) -> Self{
        let b = Branch{first_node:first, second_node:second};
        return b;
    }
}

impl Resistor{
    pub fn new(res:f64, b:Branch) -> Self{
        let r = Resistor{resistance:res, branch:b};
        return r;
    }
}