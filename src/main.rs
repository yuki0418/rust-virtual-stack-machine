use std::{
    collections::HashMap,
    io::{BufRead, BufReader},
};

fn main() {
    if let Some(f) = std::env::args()
        .nth(1)
        .and_then(|f| std::fs::File::open(f).ok())
    {
        parse_batch(BufReader::new(f));
    } else {
        parse_interactive();
    }
}

struct Vm {
    stack: Vec<Value>,
    vars: Vec<HashMap<String, Value>>,
    blocks: Vec<Vec<Value>>,
}

impl Vm {
    fn new() -> Self {
        let functions: [(&str, fn(&mut Vm)); 11] = [
            ("+", add),
            ("-", sub),
            ("*", mul),
            ("/", div),
            ("<", lt),
            (">", rt),
            ("if", op_if),
            ("def", op_def),
            ("puts", puts),
            ("dup", dup),
            ("exch", exch),
        ];
        Self {
            stack: vec![],
            vars: vec![functions
                .into_iter()
                .map(|(name, fun)| (name.to_owned(), Value::Native(NativeOp(fun))))
                .collect()],
            blocks: vec![],
        }
    }

    fn find_var(&self, name: &str) -> Option<Value> {
        self.vars
            .iter()
            .rev()
            .find_map(|vars| vars.get(name).map(|var| var.to_owned()))
    }
}

#[derive(Clone)]
struct NativeOp(fn(&mut Vm));

impl PartialEq for NativeOp {
    fn eq(&self, other: &NativeOp) -> bool {
        self.0 as *const fn() == other.0 as *const fn()
    }
}

impl Eq for NativeOp {}

impl std::fmt::Debug for NativeOp {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "<NativeOp>")
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum Value {
    Num(i32),
    Op(String),
    Sym(String),
    Block(Vec<Value>),
    Native(NativeOp),
}

impl Value {
    fn as_num(&self) -> i32 {
        match self {
            Self::Num(val) => *val,
            _ => panic!("Value is not a number"),
        }
    }

    fn block(self) -> Vec<Value> {
        match self {
            Self::Block(val) => val,
            _ => panic!("Value is not a block"),
        }
    }

    fn as_sym(&self) -> &str {
        if let Self::Sym(sym) = self {
            sym
        } else {
            panic!("Value is not a symbol");
        }
    }

    fn to_string(&self) -> String {
        match self {
            Self::Num(i) => i.to_string(),
            Self::Op(ref s) | Self::Sym(ref s) => s.clone(),
            Self::Block(_) => "<Block>".to_string(),
            Self::Native(_) => "Native".to_string(),
        }
    }
}

fn parse_batch(source: impl BufRead) -> Vec<Value> {
    let mut vm = Vm::new();
    for line in source.lines().flatten() {
        for word in line.split(" ") {
            parse_word(word, &mut vm);
        }
    }
    vm.stack
}

fn parse_interactive() {
    let mut vm = Vm::new();
    for line in std::io::stdin().lines().flatten() {
        for word in line.split(" ") {
            parse_word(word, &mut vm);
        }
        println!("stack: {:?}", vm.stack);
    }
}

fn parse_word(word: &str, vm: &mut Vm) {
    if word.is_empty() {
        return;
    }

    if word == "{" {
        vm.blocks.push(vec![]);
    } else if word == "}" {
        let top_block = vm.blocks.pop().expect("Block stack underrun!");
        eval(Value::Block(top_block), vm);
    } else {
        let code = if let Ok(num) = word.parse::<i32>() {
            Value::Num(num)
        } else if word.starts_with("/") {
            Value::Sym(word[1..].to_string())
        } else {
            Value::Op(word.to_string())
        };
        eval(code, vm)
    }
}

fn eval(code: Value, vm: &mut Vm) {
    if let Some(top_block) = vm.blocks.last_mut() {
        top_block.push(code);
        return;
    }

    if let Value::Op(ref op) = code {
        let val = vm
            .find_var(op)
            .expect(&format!("{op:?} is not a defined operation"));

        match val {
            Value::Block(block) => {
                vm.vars.push(HashMap::new());
                for code in block {
                    eval(code, vm);
                }
                vm.vars.pop();
            }
            Value::Native(op) => op.0(vm),
            _ => vm.stack.push(val),
        }
    } else {
        vm.stack.push(code.clone());
    }
}

/// ----------------- Operations -----------------------
macro_rules! impl_op {
    {$name:ident, $op:tt} => {
        fn $name(vm: &mut Vm) {
            let rhs = vm.stack.pop().unwrap().as_num();
            let lhs = vm.stack.pop().unwrap().as_num();
            vm.stack.push(Value::Num((lhs $op rhs) as i32));
        }
    }
}
impl_op!(add, +);
impl_op!(sub, -);
impl_op!(mul, *);
impl_op!(div, /);
impl_op!(lt, <);
impl_op!(rt, >);

/// ----------------- Functions -----------------------

fn puts(vm: &mut Vm) {
    let value = vm.stack.pop().unwrap();
    println!("{}", value.to_string());
}

fn op_if(vm: &mut Vm) {
    let false_branch = vm.stack.pop().unwrap().block();
    let true_branch = vm.stack.pop().unwrap().block();
    let condition = vm.stack.pop().unwrap().block();

    for code in condition {
        eval(code, vm);
    }

    let cond_result = vm.stack.pop().unwrap().as_num();

    if cond_result != 0 {
        for code in true_branch {
            eval(code, vm);
        }
    } else {
        for code in false_branch {
            eval(code, vm);
        }
    }
}

fn op_def(vm: &mut Vm) {
    let value = vm.stack.pop().unwrap();
    eval(value, vm);
    let value = vm.stack.pop().unwrap();
    let sym = vm.stack.pop().unwrap().as_sym().to_string();

    vm.vars.last_mut().unwrap().insert(sym, value);
}

fn dup(vm: &mut Vm) {
    let value = vm.stack.last().unwrap();
    vm.stack.push(value.clone());
}

fn exch(vm: &mut Vm) {
    let last = vm.stack.pop().unwrap();
    let second = vm.stack.pop().unwrap();
    vm.stack.push(last);
    vm.stack.push(second);
}
