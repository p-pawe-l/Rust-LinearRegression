pub struct Vector2D { x: f64, y: f64 }
impl Vector2D {
    pub fn new(px: f64, py: f64) -> Self { Self { x: px, y: py} }
    pub fn get_x(&self) -> f64 { return self.x; }
    pub fn get_y(&self) -> f64 { return self.y; }
}

pub struct DataSet { data_points: Vec<Vector2D> }
impl DataSet {
    pub fn new(dataset: Vec<Vector2D>) -> Result<Self, String> { 
        if dataset.len() == 0 {
            Err::<Self, String>(format!("Dataset cannot be empty."));
        }
        return Ok(Self {data_points: dataset});
    }
    fn get_all_x(&self) -> Vec<f64> {
        let mut x_vec: Vec<f64> = Vec::new();
        for dp in &self.data_points {
            x_vec.push(dp.get_x());
        }
        return x_vec;
    }
    fn get_all_y(&self) -> Vec<f64> {
        let mut y_vec: Vec<f64> = Vec::new();
        for dp in &self.data_points {
            y_vec.push(dp.get_y());
        }
        return y_vec;
    }
    pub fn calc_mean(&self) -> Vector2D {
        let mut mean_vec: Vector2D = Vector2D::new(0.0, 0.0);
        mean_vec.x = self.get_all_x().iter().sum::<f64>() / (self.data_points.len() as f64);
        mean_vec.y = self.get_all_y().iter().sum::<f64>() / (self.data_points.len() as f64);
        return mean_vec;
    }
}

pub struct LinearRegression { coefficients: Vector2D }
impl LinearRegression {
    pub fn new() -> Self { return Self {coefficients: Vector2D::new(0.0, 0.0)} }
    pub fn get_a(&self) -> f64 { return self.coefficients.get_x(); }
    pub fn get_b(&self) -> f64 { return self.coefficients.get_y();}
    pub fn fit(&mut self, dataset: &DataSet) {
        let mean_vec = dataset.calc_mean();
        
        let mut yx = 0.0;
        let mut x_squared = 0.0;
    
        for dp in &dataset.data_points {
            yx = yx + (dp.get_y() - mean_vec.get_y()) * (dp.get_x() - mean_vec.get_x());
            x_squared = x_squared + (dp.get_x() - mean_vec.get_x());
        }

        self.coefficients.y = yx as f64 / x_squared as f64;
        self.coefficients.x = mean_vec.get_y() - self.coefficients.y * mean_vec.get_x();
    }
    pub fn predict(&self, x: f64) -> f64 { return self.get_a() * x + self.get_b(); }
}