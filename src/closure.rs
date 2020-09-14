// 缓存第一次传入的值
struct Cache<T>
where
    T: Fn(u32) -> u32,
{
    caculation: T,
    value: Option<u32>,
}

impl<T> Cache<T>
where
    T: Fn(u32) -> u32,
{
    fn new(caculation: T) -> Cache<T> {
        Cache {
            caculation,
            value: None,
        }
    }

    fn value(&mut self, arg: u32) -> u32 {
        match self.value {
            Some(v) => v,
            None => {
                let v = (self.caculation)(arg);
                self.value = Some(v);
                v
            }
        }
    }
}

pub fn main() {
    let mut c = Cache::new(|x| x + 1);
    let res = c.value(1);
    println!("res: {}", res);

    let res = c.value(2);
    println!("res: {}", res);
}
