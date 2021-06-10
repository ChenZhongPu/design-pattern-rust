pub trait Observer {
    fn update(&mut self, tmp: f64);
    fn get_id(&self) -> usize;
}

pub struct CurrentConditionDisplay {
    id: usize,
}
impl CurrentConditionDisplay {
    pub fn new(id: usize) -> Self {
        CurrentConditionDisplay { id }
    }
}
impl Observer for CurrentConditionDisplay {
    fn update(&mut self, tmp: f64) {
        println!(
            "CurrentConditionDisplay ({}) gets temperature = {}",
            self.id, tmp
        );
    }
    fn get_id(&self) -> usize {
        self.id
    }
}

pub struct PredictDisplay {
    id: usize,
    last_temp: f64,
}

impl PredictDisplay {
    pub fn new(id: usize) -> Self {
        PredictDisplay {
            id,
            last_temp: f64::NAN,
        }
    }
}

impl Observer for PredictDisplay {
    fn update(&mut self, tmp: f64) {
        // a fake predication
        if self.last_temp.is_nan() {
            println!("PredicatiDisplay ({}) cannot predict now", self.id);
        } else {
            println!(
                "PredicatDisplay ({}) predicts temperature = {}",
                self.id,
                (tmp + self.last_temp) / 2.0
            );
        }
        self.last_temp = tmp;
    }

    fn get_id(&self) -> usize {
        self.id
    }
}
