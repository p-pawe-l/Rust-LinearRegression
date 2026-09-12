#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub struct Vector2D {
    x: f64,
    y: f64,
}

#[allow(dead_code)]
impl Vector2D {
    pub fn new(px: f64, py: f64) -> Self {
        Self { x: px, y: py }
    }
    pub fn get_x(&self) -> f64 {
        self.x
    }
    pub fn get_y(&self) -> f64 {
        self.y
    }
    pub fn scale(&mut self, k: &f64) {
        self.x *= k;
        self.y *= k;
    }
    pub fn add(&mut self, v: &Vector2D) {
        self.x += v.get_x();
        self.y += v.get_y();
    }
    pub fn add_k(&mut self, k: &f64) {
        self.x *= k;
        self.y *= k;
    }
    pub fn dot(&self, v: &Vector2D) -> f64 {
        self.x * v.get_x() + self.y * v.get_y()
    }
}

#[allow(dead_code)]
pub struct DataSet {
    data_points: Vec<Vector2D>,
}

#[allow(dead_code)]
impl DataSet {
    pub fn new(dataset: Vec<Vector2D>) -> Result<Self, String> {
        if dataset.is_empty() {
            return Err::<Self, String>(String::new());
        }
        Ok(Self {
            data_points: dataset,
        })
    }
    fn get_all_x(&self) -> Vec<f64> {
        let mut x_vec: Vec<f64> = Vec::new();
        for dp in &self.data_points {
            x_vec.push(dp.get_x());
        }
        x_vec
    }
    fn get_all_y(&self) -> Vec<f64> {
        let mut y_vec: Vec<f64> = Vec::new();
        for dp in &self.data_points {
            y_vec.push(dp.get_y());
        }
        y_vec
    }
    pub fn calc_mean(&self) -> Vector2D {
        let mut mean_vec: Vector2D = Vector2D::new(0.0, 0.0);
        mean_vec.x = self.get_all_x().iter().sum::<f64>() / (self.data_points.len() as f64);
        mean_vec.y = self.get_all_y().iter().sum::<f64>() / (self.data_points.len() as f64);
        mean_vec
    }
    pub fn transform<F>(&mut self, f: F)
    where
        F: FnMut(&mut Vector2D),
    {
        self.data_points.iter_mut().for_each(f);
    }
}

#[allow(dead_code)]
#[derive(Debug, Clone, Copy)]
pub struct LinearRegressionModel {
    coefficients: Vector2D,
}

#[allow(dead_code)]
impl LinearRegressionModel {
    pub fn new() -> Self {
        Self {
            coefficients: Vector2D::new(0.0, 0.0),
        }
    }
    pub fn get_a(&self) -> f64 {
        self.coefficients.get_x()
    }
    pub fn get_b(&self) -> f64 {
        self.coefficients.get_y()
    }
    pub fn fit(&mut self, dataset: &DataSet) {
        let mean_vec = dataset.calc_mean();

        let mut yx: f64 = 0.0;
        let mut x_squared: f64 = 0.0;

        for dp in &dataset.data_points {
            yx += (dp.get_y() - mean_vec.get_y()) * (dp.get_x() - mean_vec.get_x());
            x_squared += dp.get_x() - mean_vec.get_x();
        }

        self.coefficients.y = yx / x_squared;
        self.coefficients.x = mean_vec.get_y() - self.coefficients.y * mean_vec.get_x();
    }
    pub fn predict(&self, x: f64) -> f64 {
        self.get_a() * x + self.get_b()
    }
}
