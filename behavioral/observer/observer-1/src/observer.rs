pub trait Observer {
    fn update(&self, tmp: f64);
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
    fn update(&self, tmp: f64) {
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
}

impl PredictDisplay {
    pub fn new(id: usize) -> Self {
        PredictDisplay { id }
    }
}

impl Observer for PredictDisplay {
    fn update(&self, tmp: f64) {
        // a fake predication
        println!(
            "PredictDisplay ({}) predicts temperature = {}",
            self.id,
            tmp + 0.1
        );
    }
    fn get_id(&self) -> usize {
        self.id
    }
}
