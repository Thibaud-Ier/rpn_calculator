type Number = f64;

pub fn calcul_rpn(_str: &str) -> String 
{
    let vec = str_tab(_str);
    let mut engine = RpnEngine::new(); 

    return engine.execute(&vec);
}

fn str_tab(_str: &str) -> Vec<&str>
{
    return _str.trim().split(" ").collect();
}

struct RpnEngine
{
    list: Vec<Number>
}

impl RpnEngine {
    fn new() -> RpnEngine
    {
        RpnEngine {
            list: Vec::new() 
        }
    }

    fn execute(&mut self, vec: &Vec<&str>) -> String
    {
        for item in vec {
            match *item {
                "+" => self.operation(add),
                "-" => self.operation(minus),
                "*" => self.operation(multiply),
                "/" => self.operation(divide),
                "SQRT" => self.sqrt(),
                x if x.parse::<Number>().is_ok() => self.push_number(x.parse::<Number>().unwrap()),
                _ => { 
                    println!("Invalid string : not an operation or number {}", *item);
                    return "".to_string();
                 },
            }
        }

        if self.list.len() == 1
        {
            println!("{}", self.list[0].to_string());
            return self.list[0].to_string();
        }
        else  
        {
            println!("Invalid string : calcul is not finished");
            return "".to_string();
        }  
    }

    fn push_number(&mut self, number: Number)
    {
        self.list.push(number);
    }

    fn pop_number(&mut self) -> Number
    {
        let number = self.list.pop();

        match number {
            Some(n) => return n,
            None => { panic!("Invalid string : it missed an operator"); },
        }
    }

    fn operation(&mut self, op: fn(Number, Number) -> Number) {
        let op2 = self.pop_number();
        let op1 = self.pop_number();

        self.push_number(op(op1,op2));
    }

    fn sqrt(&mut self)
    {
        let op1 = self.pop_number();
        
        self.push_number(op1.sqrt());
    }
}

fn add(op1: Number, op2: Number) -> Number
{
    return op1 + op2;
}

fn minus(op1: Number, op2: Number) -> Number
{
    return op1 - op2;
}

fn multiply(op1: Number, op2: Number) -> Number
{
    return op1 * op2;
}

fn divide(op1: Number, op2: Number) -> Number
{
    return op1 / op2;
}