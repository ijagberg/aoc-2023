use std::collections::HashSet;

pub struct ValueHistory {
    values: Vec<i64>,
}

impl ValueHistory {
    pub fn new(values: Vec<i64>) -> Self {
        Self { values }
    }

    pub fn next_value(&self) -> i64 {
        let mut final_values = vec![self.values[self.values.len() - 1]];

        let mut values = self.values.clone();
        loop {
            let derived = derive(&values);
            final_values.push(derived[derived.len() - 1]);
            if derived[0] == 0 && derived[derived.len() - 1] == 0 {
                break;
            }
            values = derived;
        }

        final_values.into_iter().fold(0, |acc, curr| curr + acc)
    }

    pub fn prev_value(&self) -> i64 {
        let mut first_values = vec![self.values[0]];

        let mut values = self.values.clone();
        loop {
            let derived = derive(&values);
            first_values.push(derived[0]);
            if derived[0] == 0 && derived[derived.len() - 1] == 0 {
                break;
            }
            values = derived;
        }

        first_values
            .into_iter()
            .rev()
            .fold(0, |acc, curr| curr - acc)
    }
}

fn derive(values: &[i64]) -> Vec<i64> {
    let mut derived = Vec::with_capacity(values.len() - 1);
    for w in values.windows(2) {
        let diff = w[1] - w[0];
        derived.push(diff);
    }

    derived
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn prev_value_test() {
        let hist = ValueHistory::new(vec![0, 3, 6, 9, 12, 15]);
        assert_eq!(hist.prev_value(), -3);

        let hist = ValueHistory::new(vec![1, 3, 6, 10, 15, 21]);
        assert_eq!(hist.prev_value(), 0);

        let hist = ValueHistory::new(vec![10, 13, 16, 21, 30, 45]);
        assert_eq!(hist.prev_value(), 5);
    }
}
