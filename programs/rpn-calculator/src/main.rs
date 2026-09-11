enum Token {
    Number(f64),
    Operator(char),
}

#[derive(Debug)]
enum LogEntry {
    Success(String),
    Error(String),
}

struct Calculator {
    stack: Vec<f64>,
    log: Vec<LogEntry>,
    errored: bool,
}

impl Calculator {
    fn clear(&mut self) {
        self.stack.clear();
        self.log.clear();
        self.errored = false;
    }

    fn push(&mut self, n: f64) {
        self.stack.push(n);
    }

    fn add_log(&mut self, log: LogEntry) {
        self.log.push(log);
    }

    fn pop(&mut self) -> Option<f64> {
        self.stack.pop()
    }

    fn exec(&mut self, token: &Token) {
        // Once an error has occurred, ignore all further tokens.
        if self.errored {
            return;
        }

        match token {
            Token::Number(n) => self.push(*n),
            Token::Operator(opr) => {
                // Peek first: do not mutate the stack unless we know
                // the operation can succeed.
                if self.stack.len() < 2 {
                    self.add_log(LogEntry::Error(
                        "Not enough operands".to_string(),
                    ));
                    self.errored = true;
                    return;
                }

                // For division, also check for zero divisor before popping.
                // The top of the stack is the right operand (divisor).
                if *opr == '/' {
                    let divisor = self.stack[self.stack.len() - 1];
                    if divisor == 0.0 {
                        self.add_log(LogEntry::Error(
                            "Division by zero".to_string(),
                        ));
                        self.errored = true;
                        return;
                    }
                }

                // Safe to pop now: we've verified there are at least two values,
                // and (for `/`) that the divisor is non-zero.
                let v1 = self.pop().unwrap(); // right operand
                let v2 = self.pop().unwrap(); // left operand

                match opr {
                    '+' => {
                        self.push(v2 + v1);
                        self.add_log(LogEntry::Success(
                            "Addition operation successful".to_string(),
                        ));
                    }
                    '-' => {
                        self.push(v2 - v1);
                        self.add_log(LogEntry::Success(
                            "Subtraction operation successful".to_string(),
                        ));
                    }
                    '*' => {
                        self.push(v2 * v1);
                        self.add_log(LogEntry::Success(
                            "Product operation successful".to_string(),
                        ));
                    }
                    '/' => {
                        self.push(v2 / v1);
                        self.add_log(LogEntry::Success(
                            "Division operation successful".to_string(),
                        ));
                    }
                    _ => {
                        // Unknown operator: we already popped, so we've mutated.
                        // Log the error and mark the calculator as errored.
                        self.add_log(LogEntry::Error(format!(
                            "Unknown operator: {}",
                            opr
                        )));
                        self.errored = true;
                    }
                }
            }
        }
    }

    fn stack_state(&self) {
        println!("Current stack: {:?}", self.stack);
    }
}

fn compute(expr: &[Token], calc: &mut Calculator) {
    println!("\n --- Computation Stack ---");
    for token in expr {
        calc.exec(token);
        calc.stack_state();
        if calc.errored {
            println!("Halting: an error occurred.");
            break;
        }
    }
}

fn show_log(calc: &Calculator) {
    println!("\n --- Log ---");
    for log in &calc.log {
        match log {
            LogEntry::Success(msg) => println!("OK : {}", msg),
            LogEntry::Error(msg) => println!("ERR: {}", msg),
        }
    }
}

fn main() {
    // Test 1: a valid expression
    let expr_ok = vec![
        Token::Number(3.0),
        Token::Number(4.0),
        Token::Operator('*'),
        Token::Number(3.0),
        Token::Operator('-'),
    ];

    // Test 2: division by zero in the middle of an expression
    let expr_div_zero = vec![
        Token::Number(5.0),
        Token::Number(0.0),
        Token::Operator('/'),
        Token::Number(3.0),
        Token::Operator('+'),
    ];

    // Test 3: not enough operands
    let expr_missing = vec![
        Token::Number(1.0),
        Token::Operator('+'),
    ];

    let mut calc = Calculator {
        stack: Vec::new(),
        log: Vec::new(),
        errored: false,
    };

    println!("=== Valid expression: 3 4 * 3 - ===");
    compute(&expr_ok, &mut calc);
    show_log(&calc);

    calc.clear();

    println!("\n=== Division by zero: 5 0 / 3 + ===");
    compute(&expr_div_zero, &mut calc);
    show_log(&calc);

    calc.clear();

    println!("\n=== Missing operand: 1 + ===");
    compute(&expr_missing, &mut calc);
    show_log(&calc);
}