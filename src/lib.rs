use pyo3::exceptions::PyIOError;
use pyo3::prelude::*;
use std::fs::File;
use std::io::{Read, Write};
use zip::unstable::write::FileOptionsExt;
use zip::{write::FileOptions, CompressionMethod};

/// A Python wrapper for Rust's zip crate with ZipCrypto support
#[pyclass(name = "ZipWriter")]
struct PyZipWriter {
    writer: Option<zip::ZipWriter<std::fs::File>>,
    password: Option<Vec<u8>>,
}

#[pymethods]
impl PyZipWriter {
    /// Create a new ZIP file with optional password for ZipCrypto encryption
    #[new]
    #[pyo3(signature = (path, password = None))]
    fn new(path: &str, password: Option<&[u8]>) -> PyResult<Self> {
        let file = File::create(path).map_err(|e| PyIOError::new_err(e.to_string()))?;
        let writer = zip::ZipWriter::new(file);

        Ok(PyZipWriter {
            writer: Some(writer),
            password: password.map(|p| p.to_vec()),
        })
    }

    /// Add a file from disk to the ZIP archive
    fn write_file(&mut self, source_path: &str, entry_name: &str) -> PyResult<()> {
        let mut writer = self
            .writer
            .take()
            .ok_or_else(|| PyIOError::new_err("ZipWriter is closed"))?;

        let mut file = File::open(source_path)
            .map_err(|e| PyIOError::new_err(format!("Failed to open source file: {}", e)))?;

        let mut buffer = Vec::new();
        file.read_to_end(&mut buffer)
            .map_err(|e| PyIOError::new_err(format!("Failed to read source file: {}", e)))?;

        let options = self.get_file_options();

        writer
            .start_file(entry_name, options)
            .map_err(|e| PyIOError::new_err(format!("Failed to start file entry: {}", e)))?;

        writer
            .write_all(&buffer)
            .map_err(|e| PyIOError::new_err(format!("Failed to write file data: {}", e)))?;

        self.writer = Some(writer);
        Ok(())
    }

    /// Add bytes from memory to the ZIP archive
    fn write_bytes(&mut self, _py: Python<'_>, data: &[u8], entry_name: &str) -> PyResult<()> {
        let mut writer = self
            .writer
            .take()
            .ok_or_else(|| PyIOError::new_err("ZipWriter is closed"))?;

        let options = self.get_file_options();

        writer
            .start_file(entry_name, options)
            .map_err(|e| PyIOError::new_err(format!("Failed to start file entry: {}", e)))?;

        writer
            .write_all(data)
            .map_err(|e| PyIOError::new_err(format!("Failed to write data: {}", e)))?;

        self.writer = Some(writer);
        Ok(())
    }

    /// Close the ZIP file
    fn close(&mut self) -> PyResult<()> {
        if let Some(writer) = self.writer.take() {
            writer
                .finish()
                .map_err(|e| PyIOError::new_err(format!("Failed to finish ZIP file: {}", e)))?;
        }
        Ok(())
    }

    /// Context manager support
    fn __enter__(slf: Py<Self>) -> Py<Self> {
        slf
    }

    /// Context manager exit
    #[allow(unused_variables)]
    #[pyo3(signature = (exc_type = None, exc_value = None, traceback = None))]
    fn __exit__(
        &mut self,
        exc_type: Option<PyObject>,
        exc_value: Option<PyObject>,
        traceback: Option<PyObject>,
    ) -> PyResult<bool> {
        self.close()?;
        Ok(false)
    }
}

impl PyZipWriter {
    /// Helper method to get file options with encryption if password is set
    fn get_file_options(&self) -> FileOptions<'static, zip::write::ExtendedFileOptions> {
        let mut options = FileOptions::default().compression_method(CompressionMethod::Deflated);

        if let Some(password) = &self.password {
            // Use the legacy ZipCrypto encryption from the unstable API
            // This is available without enabling a feature flag
            options = options.with_deprecated_encryption(password);
        }

        options
    }
}

/// A Python module implemented in Rust.
#[pymodule]
#[pyo3(name = "_rust")]
fn rustyzip(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyZipWriter>()?;
    Ok(())
}
