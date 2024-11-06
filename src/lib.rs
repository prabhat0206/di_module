use pyo3::prelude::*;
use rayon::prelude::*;
use reqwest;

#[pyclass]
struct ImageResult {
    #[pyo3(get)]
    url: String,
    #[pyo3(get)]
    image_binary: Vec<u8>,
}

#[pyfunction]
fn load_all_images_from_urls(urls: Vec<String>) -> PyResult<Vec<ImageResult>> {
    let images: Vec<ImageResult> = urls.par_iter()
        .filter_map(|url| {
            let response = reqwest::blocking::get(url).ok()?;
            let bytes = response.bytes().ok()?.to_vec();
            Some(ImageResult {
                url: url.clone(),
                image_binary: bytes,
            })
        })
        .collect();

    Ok(images)
}

/// A Python module implemented in Rust.
#[pymodule]
fn digicount_module(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<ImageResult>()?;
    m.add_function(wrap_pyfunction!(load_all_images_from_urls, m)?)?;
    Ok(())
}