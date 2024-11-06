use pyo3::prelude::*;
use rayon::prelude::*;
use reqwest;

#[pyfunction]
fn load_all_images_from_urls(urls: Vec<String>) -> PyResult<Vec<Vec<u8>>> {
    let images: Vec<Vec<u8>> = urls.par_iter()
        .filter_map(|url| {
            let response = reqwest::blocking::get(url).ok()?;
            response.bytes().ok().map(|b| b.to_vec())
        })
        .collect();

    Ok(images)
}

/// A Python module implemented in Rust.
#[pymodule]
fn digicount_module(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(load_all_images_from_urls, m)?)?;
    Ok(())
}