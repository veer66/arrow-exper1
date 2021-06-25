use std::fs::File;
use arrow::csv;
use arrow::array::{Array, Float64Array};

const FRAME_SIZE: usize = 300;
const RATIO: f64 = 11.1;

fn is_fake_max_age(age: &Float64Array, i: usize) -> bool {
    let mut is_max = true;
    for j in (i + 1)..(i + 1 + FRAME_SIZE).min(age.len()) {
	if !age.is_null(j) && !age.is_null(i) &&
	    age.value(j) > age.value(i) * RATIO {
		is_max = false;
		break
	}
    }
    is_max
}

fn main() {
    let input_path = "../confirmed-cases.csv";
    let input_file = File::open(input_path).unwrap();
    let builder = csv::ReaderBuilder::new()
	.infer_schema(Some(134000))
	.has_header(true)
	.with_batch_size(1000000);
    let mut reader = builder.build(input_file).unwrap();
    let batch = reader.next().unwrap().unwrap();
    let age_i = batch.schema().index_of("age").unwrap();
    let age = batch.column(age_i).as_any().downcast_ref::<Float64Array>().unwrap();
    let max_age: Vec<_> = (0..age.len()).map(|i| is_fake_max_age(age, i)).collect();
    println!("{} {}", max_age.len(), max_age[10000]);
}
