use rand::prelude::*;

struct MonadWrapper<T> {
    base: T,
    logs: Vec<String>,
    trace: Vec<T>,
}

impl<T> MonadWrapper<T>
where
    T: Copy,
{
    fn new(base: T) -> Self {
        Self {
            base,
            logs: vec![],
            trace: vec![],
        }
    }

    fn unwrap(&self) -> T {
        self.base
    }

    fn add_logs(&mut self, opp_desc: &str, curr_num: T) {
        self.logs.push(opp_desc.to_string());
        self.trace.push(curr_num);
    }
}

fn square<T: std::marker::Copy>(i: &T) -> MonadWrapper<T>
where
    for<'a> &'a T: std::ops::Mul<&'a T, Output = T>,
{
    let new_num = i * i;
    let mut seg = MonadWrapper::new(new_num);
    seg.add_logs("Squared the number", new_num);
    seg
}

fn times_two<T: std::marker::Copy>(i: &T) -> MonadWrapper<T>
where
    for<'a> &'a T: std::ops::Add<&'a T, Output = T>,
{
    let new_num = i + i;
    let mut seg = MonadWrapper::new(new_num);
    seg.add_logs("Multplied the number by two", new_num);
    seg
}

fn add_one<T: std::marker::Copy>(i: &T) -> MonadWrapper<T>
where
    for<'a> &'a T: std::ops::Add<u128, Output = T>,
{
    let new_num = i + 1;
    let mut seg = MonadWrapper::new(new_num);
    seg.add_logs("Added one to the number", new_num);
    seg
}

fn divide_by_two<T: std::marker::Copy>(i: &T) -> MonadWrapper<T>
where
    for<'a> &'a T: std::ops::Div<u128, Output = T>,
{
    let new_num = i / 2;
    let mut seg = MonadWrapper::new(new_num);
    seg.add_logs("Devided the number by two", new_num);
    seg
}

fn transform<T, F>(mut base: MonadWrapper<T>, trans_func: F) -> MonadWrapper<T>
where
    F: Fn(&T) -> MonadWrapper<T>,
    T: Copy,
{
    let mut new_num = trans_func(&base.unwrap());
    new_num.logs.append(&mut base.logs);
    new_num.trace.append(&mut base.trace);
    MonadWrapper {
        base: new_num.unwrap(),
        logs: new_num.logs,
        trace: new_num.trace,
    }
}

fn main() {
    let number = MonadWrapper::new(42);
    let mut first = transform(number, add_one);
    let mut rng = rand::thread_rng();
    for _ in 1..10 {
        let x = rng.gen_range(1..=4);
        match x {
            1 => first = transform(first, add_one),
            2 => first = transform(first, times_two),
            3 => first = transform(first, square),
            4 => first = transform(first, divide_by_two),
            _ => panic!(),
        }
    }

    println!(
        "Answer is {}\nLogs are: {:?}\nTrace is: {:?}",
        first.unwrap(),
        first.logs,
        first.trace
    );
}
