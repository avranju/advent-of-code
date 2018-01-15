use std::io::{self, BufRead};
use std::rc::Rc;

mod parser;
mod types;
mod vars;

use types::{Instruction, Source, ValueSource};
use vars::VarTable;

fn main() {
    let stdin = io::stdin();
    let instructions = stdin
        .lock()
        .lines()
        .map(|l| l.unwrap())
        .filter(|l| l.len() > 0)
        .map(|line| parser::parse(&line))
        .collect::<Vec<Instruction>>();

    let mut var_table = VarTable::new();

    let value_sources = instructions.iter().flat_map(|inst| match inst.src {
        Source::Value(ref v) => vec![v],
        Source::Not(ref v) => vec![v],
        Source::And(ref l, ref r) => vec![l, r],
        Source::Or(ref l, ref r) => vec![l, r],
        Source::LeftShift(ref l, ref r) => vec![l, r],
        Source::RightShift(ref l, ref r) => vec![l, r],
    });

    for src in value_sources {
        if let ValueSource::Variable(ref name) = src {
            var_table.add(&name);
        }
    }
    for inst in instructions.iter() {
        var_table.add(&inst.dest);
    }

    let var_table = Rc::new(var_table);

    for instruction in instructions.into_iter() {
        print!("Processing instruction: {}\r", instruction);
        match instruction.src {
            Source::Value(ref value_src) => {
                value_expr(value_src, var_table.clone(), &instruction.dest)
            }
            Source::Not(ref value_src) => not_expr(value_src, var_table.clone(), &instruction.dest),
            Source::And(left, right) => binary_expr(
                &left,
                &right,
                var_table.clone(),
                &instruction.dest,
                |left, right| left & right,
            ),
            Source::Or(left, right) => binary_expr(
                &left,
                &right,
                var_table.clone(),
                &instruction.dest,
                |left, right| left | right,
            ),
            Source::LeftShift(left, right) => binary_expr(
                &left,
                &right,
                var_table.clone(),
                &instruction.dest,
                |left, right| left << right,
            ),
            Source::RightShift(left, right) => binary_expr(
                &left,
                &right,
                var_table.clone(),
                &instruction.dest,
                |left, right| left >> right,
            ),
        };
    }

    println!("\n\n>>> Results:");
    var_table.print_entries();
}

fn not_expr(value_src: &ValueSource, var_table: Rc<VarTable>, dest: &str) {
    match value_src {
        ValueSource::Constant(val) => {
            var_table.set(dest, Some(!(*val)));
        }
        ValueSource::Variable(name) => {
            var_table.subscribe(
                &name,
                {
                    let var_table = var_table.clone();
                    let dest = dest.to_owned();
                    move |_: &str, val: u16| {
                        var_table.set(&dest, Some(!val));
                    }
                },
                Some(dest.to_owned()),
            );
        }
    }
}

fn value_expr(value_src: &ValueSource, var_table: Rc<VarTable>, dest: &str) {
    match value_src {
        ValueSource::Constant(val) => {
            var_table.set(dest, Some(*val));
        }
        ValueSource::Variable(name) => {
            var_table.subscribe(
                &name,
                {
                    let var_table = var_table.clone();
                    let dest = dest.to_owned();
                    move |_: &str, val| {
                        var_table.set(&dest, Some(val));
                    }
                },
                Some(dest.to_owned()),
            );
        }
    }
}

fn binary_expr<F>(
    left: &ValueSource,
    right: &ValueSource,
    var_table: Rc<VarTable>,
    dest: &str,
    op: F,
) where
    F: Fn(u16, u16) -> u16 + 'static,
{
    match (left, right) {
        (ValueSource::Constant(left), ValueSource::Constant(right)) => {
            var_table.set(dest, Some(op(*left, *right)));
        }
        (ValueSource::Constant(left), ValueSource::Variable(name)) => {
            var_table.subscribe(
                &name,
                {
                    let var_table = var_table.clone();
                    let dest = dest.to_owned();
                    let left = *left;
                    move |_: &str, val: u16| {
                        var_table.set(&dest, Some(op(left, val)));
                    }
                },
                Some(dest.to_owned()),
            );
        }
        (ValueSource::Variable(name), ValueSource::Constant(right)) => {
            var_table.subscribe(
                &name,
                {
                    let var_table = var_table.clone();
                    let dest = dest.to_owned();
                    let right = *right;
                    move |_: &str, val: u16| {
                        var_table.set(&dest, Some(op(val, right)));
                    }
                },
                Some(dest.to_owned()),
            );
        }
        (ValueSource::Variable(left_name), ValueSource::Variable(right_name)) => {
            on_vars_change(
                var_table.clone(),
                left_name,
                right_name,
                {
                    let var_table = var_table.clone();
                    let dest = dest.to_owned();
                    move |val1, val2| {
                        var_table.set(&dest, Some(op(val1, val2)));
                    }
                },
                dest,
            );
        }
    }
}

fn make_subscriber<F>(
    var_table: Rc<VarTable>,
    var1: &str,
    var2: &str,
    on_change: F,
) -> impl Fn(&str, u16)
where
    F: Fn(u16, u16) + 'static,
{
    let var1 = var1.to_string();
    let var2 = var2.to_string();

    move |_: &str, _: u16| {
        if let (Some(val1), Some(val2)) = (var_table.get_var(&var1), var_table.get_var(&var2)) {
            on_change(val1, val2);
        }
    }
}

fn on_vars_change<F>(var_table: Rc<VarTable>, var1: &str, var2: &str, on_change: F, dest: &str)
where
    F: Fn(u16, u16) + 'static,
{
    let on_change = Rc::new(on_change);

    let subscriber = make_subscriber(var_table.clone(), var1, var2, {
        let on_change = on_change.clone();
        move |v1, v2| on_change(v1, v2)
    });
    var_table.subscribe(var1, subscriber, Some(dest.to_owned()));

    let subscriber = make_subscriber(var_table.clone(), var1, var2, {
        let on_change = on_change.clone();
        move |v1, v2| on_change(v1, v2)
    });
    var_table.subscribe(var2, subscriber, Some(dest.to_owned()));
}
