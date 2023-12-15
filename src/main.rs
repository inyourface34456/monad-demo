struct MonadWrapper<T> {
    base: T,
    logs: Vec<String>,
}

impl<T> MonadWrapper<T>
where
    T: Copy,
{
    fn new(base: T) -> Self {
        Self { base, logs: vec![] }
    }

    fn unwrap(&self) -> T {
        self.base
    }

    fn add_logs(&mut self, opp_desc: String) {
        self.logs.push(opp_desc)
    }
}

fn square<T: std::ops::Mul<Output = T> + std::marker::Copy>(i: &T) -> MonadWrapper<T>
where
    for<'a> &'a T: std::ops::Mul<&'a T, Output = T>,
{
    let mut seg = MonadWrapper::new(i * i);
    seg.add_logs("Squared the number".to_string());
    seg
}

fn add_one<T: std::ops::Add<Output = T> + std::marker::Copy>(i: &T) -> MonadWrapper<T>
where
    for<'a> &'a T: std::ops::Add<&'a T, Output = T>,
{
    let mut seg = MonadWrapper::new(i + i);
    seg.add_logs("Added one to the number".to_string());
    seg
}

fn transform<T, F>(mut base: MonadWrapper<T>, trans_func: F) -> MonadWrapper<T>
where
    F: Fn(&T) -> MonadWrapper<T>,
    T: Copy,
{
    let mut new_num = trans_func(&base.unwrap());
    new_num.logs.append(&mut base.logs);
    MonadWrapper {
        base: new_num.unwrap(),
        logs: new_num.logs,
    }
}

fn main() {
    let number = MonadWrapper::new(42);
    let answer = transform(number, square);
    let second_answer = transform(answer, add_one);

    println!(
        "Answer is {}\nLogs are: {:?}",
        second_answer.unwrap(),
        second_answer.logs
    );
}
