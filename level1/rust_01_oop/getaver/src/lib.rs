pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[derive(Debug)]
pub struct AveCollect {
    list: Vec<i32>,
    ave: f64,
}

impl AveCollect {
    pub fn new() -> Self {
        Self {
            list: vec![],
            ave: 0.0,
        }
    }
    pub fn add(&mut self, value: i32) {
        self.list.push(value);
        self.update_average();
    }
    pub fn delete(&mut self) -> Option<i32> {
        let res = self.list.pop();
        match res {
            Some(v) => {
                self.update_average();
                Some(v)
            }
            None => None,
        }
    }
    pub fn update_average(&mut self) -> f64 {
        let total: i32 = self.list.iter().sum();
        self.ave = total as f64 / self.list.len() as f64;
        self.ave
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
