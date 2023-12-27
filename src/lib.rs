use std::sync::Mutex;

use once_cell::sync::Lazy;
use pyo3::prelude::*;
use snowflake::SnowflakeIdGenerator;
use ulid::Ulid;
use uuid::Uuid;

use crate::objectid::ObjectId;

mod objectid;

static SNOWFLAKE_GENERATOR: Lazy<Mutex<SnowflakeIdGenerator>> = Lazy::new(|| {
    Mutex::new(SnowflakeIdGenerator::new(1, 1))
});

#[pyfunction]
#[pyo3(name = "ulid")]
fn get_ulid() -> PyResult<String> {
    Ok(Ulid::new().to_string())
}


fn get_snowflake(machine_id: Option<i32>, node_id: Option<i32>) -> i64 {
    let mut sg = SNOWFLAKE_GENERATOR.lock().unwrap();
    sg.node_id = node_id.unwrap_or(1);
    sg.machine_id = machine_id.unwrap_or(1);
    sg.real_time_generate()
}

#[pyfunction]
#[pyo3(name = "snowflake_str")]
fn get_snowflake_str(machine_id: Option<i32>, node_id: Option<i32>) -> PyResult<String> {
    Ok(format!("{:X}", get_snowflake(machine_id, node_id)))
}

#[pyfunction]
#[pyo3(name = "snowflake_int")]
fn get_snowflake_int(machine_id: Option<i32>, node_id: Option<i32>) -> PyResult<i64> {
    Ok(get_snowflake(machine_id, node_id))
}

#[pyfunction]
fn uuid_v7() -> PyResult<String> {
    Ok(Uuid::now_v7().to_string())
}

#[pyfunction]
fn object_id() -> PyResult<String> {
    Ok(ObjectId::new().to_hex_string())
}

#[pymodule]
fn fastid(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(get_ulid, m)?)?;

    m.add_function(wrap_pyfunction!(get_snowflake_int, m)?)?;
    m.add_function(wrap_pyfunction!(get_snowflake_str, m)?)?;

    m.add_function(wrap_pyfunction!(uuid_v7, m)?)?;

    m.add_function(wrap_pyfunction!(object_id, m)?)?;

    Ok(())
}
