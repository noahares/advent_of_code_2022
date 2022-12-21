use std::collections::HashMap;

use anyhow::Result;

#[derive(Debug, Clone)]
enum Expression {
    Value(i64),
    Expression((String, char, String))
}

impl Expression {
    fn eval(self: &Self, map: &HashMap<String, Expression>) -> i64 {
        match self {
            Expression::Value(i) => *i,
            Expression::Expression(e) => {
                let op_1 = map[&e.0].clone();
                let op_2 = map[&e.2].clone();
                let result = match e.1 {
                    '+' => op_1.eval(map) + op_2.eval(map),
                    '-' => op_1.eval(map) - op_2.eval(map),
                    '*' => op_1.eval(map) * op_2.eval(map),
                    '/' => op_1.eval(map) / op_2.eval(map),
                    _ => 0
                };
                result
            }
        }
    }
    fn eval_with_stupid_human(self: &Self, map: &mut HashMap<String, Expression>, human: i64) -> i64 {
        map.insert("humn".to_string(), Expression::Value(human));
        self.eval(map)
    }
}


fn part01() -> Result<i64> {
    let expressions: HashMap<String, Expression> = std::fs::read_to_string("./data/day21.input")?
        .trim()
        .lines()
        .map(|line| match sscanf::scanf!(line,
                                   "{String}: {String} {char} {String}") {
            Ok(tuple) => (tuple.0, Expression::Expression((tuple.1, tuple.2, tuple.3))),
            Err(_) => {
                let expr = sscanf::scanf!(line,
                                  "{String}: {i64}").unwrap();
                (expr.0, Expression::Value(expr.1))
                }
        })
        .collect();
        let root_expr = expressions["root"].clone();
        let result = root_expr.eval(&expressions);
    Ok(result)
}

fn part02() -> Result<i64> {
    let mut human_range = (i32::MIN as i64 / 2, i32::MAX as i64 * 2048);
    let mut expressions: HashMap<String, Expression> = std::fs::read_to_string("./data/day21.input")?
        .trim()
        .lines()
        .map(|line| match sscanf::scanf!(line,
                                   "{String}: {String} {char} {String}") {
            Ok(tuple) => (tuple.0, Expression::Expression((tuple.1, tuple.2, tuple.3))),
            Err(_) => {
                let expr = sscanf::scanf!(line,
                                  "{String}: {i64}").unwrap();
                (expr.0, Expression::Value(expr.1))
                }
        })
        .collect();
        let (left_result, right_result) = match expressions["root"].clone() {
            Expression::Expression(e) => (e.0, e.2),
            _ => unreachable!()
        };
        expressions.insert("root".to_string(), Expression::Expression((left_result, '-', right_result)));
        let root_expr = expressions["root"].clone();
        let result = root_expr.eval_with_stupid_human(&mut expressions, human_range.0);
        if result > 0 { (human_range.0, human_range.1) = (human_range.1, human_range.0); }
        let mut human: i64 = 0;
        for _ in 0..100 {
            human = (human_range.0 + human_range.1) / 2;
            let result = root_expr.eval_with_stupid_human(&mut expressions, human);
            if result == 0 {
                // println!("good job human!");
                return Ok(human);
            }
            if result < 0 { human_range.0 = human; } else { human_range.1 = human; }
        }
    Ok(human)
}

fn main() -> Result<()> {
    println!("{:?}", part01().unwrap());
    println!("{:?}", part02().unwrap());
    Ok(())
}
