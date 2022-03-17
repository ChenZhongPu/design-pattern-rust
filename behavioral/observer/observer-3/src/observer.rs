pub trait Observer {
    fn update(&self, tmp: f64);
 }
 
pub struct CurrentConditionDisplay;
impl Observer for CurrentConditionDisplay {
    fn update(&self, tmp: f64) {
        println!("CurrentConditionDisplay gets temperature = {}", tmp);
    }
}

pub struct PredictDisplay;
impl Observer for PredictDisplay {
    fn update(&self, tmp: f64) {
        // a fake predication
        println!("PredictDisplay predicts temperature = {}", tmp + 0.1);
    }
}