mod csv_manager;
mod linear_regression;

fn main() -> Result<(), csv_manager::CsvError> {
    let data = csv_manager::read_csv("data/points.csv")?;
    let x_col = data.column_f64("x")?;
    let y_col = data.column_f64("y")?;

    let mut vector2d_vec: Vec<linear_regression::Vector2D> = Vec::new();
    assert_eq!(x_col.len(), y_col.len());
    for i in 0..x_col.len() {
        let new_vector: linear_regression::Vector2D =
            linear_regression::Vector2D::new(x_col[i], y_col[i]);
        vector2d_vec.push(new_vector);
    }

    let dataset = linear_regression::DataSet::new(vector2d_vec);
    let mut model: linear_regression::LinearRegressionModel =
        linear_regression::LinearRegressionModel::new();
    model.fit(&dataset.unwrap());
    model.save("models/model_50")?;

    Ok(())
}
