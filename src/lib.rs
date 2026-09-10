use pyo3::exceptions::{PyIOError, PyValueError};
use pyo3::prelude::*;
use std::fs::File;
use std::io::{Read, Seek, SeekFrom, Write};
use zip::unstable::write::FileOptionsExt;
use zip::{write::FileOptions, CompressionMethod};

use pyo3::types::PyString;
use pyo3_file::PyFileLikeObject;

enum InnerWriter {
    File(File),
    FileLike(PyFileLikeObject),
}

impl InnerWriter {
    pub fn new(path_or_file_like: Py<PyAny>) -> PyResult<InnerWriter> {
        Python::attach(|py| {
            // is a path
            if let Ok(string_ref) = path_or_file_like.cast_bound::<PyString>(py) {
                let file = File::create(string_ref.to_string_lossy().to_string())
                    .map_err(|e| PyIOError::new_err(e.to_string()))?;
                return Ok(InnerWriter::File(file));
            }

            // is a file-like
            match PyFileLikeObject::with_requirements(path_or_file_like, false, true, true, false) {
                Ok(f) => Ok(InnerWriter::FileLike(f)),
                Err(e) => Err(e),
            }
        })
    }
}

impl Write for InnerWriter {
    fn write(&mut self, buf: &[u8]) -> std::io::Result<usize> {
        match self {
            InnerWriter::File(file) => file.write(buf),
            InnerWriter::FileLike(file_like) => file_like.write(buf),
        }
    }

    fn flush(&mut self) -> std::io::Result<()> {
        match self {
            InnerWriter::File(file) => file.flush(),
            InnerWriter::FileLike(file_like) => file_like.flush(),
        }
    }
}

impl Seek for InnerWriter {
    fn seek(&mut self, pos: SeekFrom) -> std::io::Result<u64> {
        match self {
            InnerWriter::File(file) => file.seek(pos),
            InnerWriter::FileLike(file_like) => file_like.seek(pos),
        }
    }
}

/// A Python wrapper for Rust's zip crate with ZipCrypto support
#[pyclass(name = "ZipWriter")]
struct PyZipWriter {
    writer: Option<zip::ZipWriter<InnerWriter>>,
    password: Option<Vec<u8>>,
}

#[pymethods]
impl PyZipWriter {
    /// Create a new ZIP file with optional password for ZipCrypto encryption
    #[new]
    #[pyo3(signature = (path_or_file_like, password = None))]
    fn new(path_or_file_like: Py<PyAny>, password: Option<&[u8]>) -> PyResult<Self> {
        // An empty ZipCrypto password derives a predictable key, so zip >= 3
        // rejects it. Check it here as well, to fail before creating the file
        // and to raise a clearer error than zip's own wording.
        if password.is_some_and(|p| p.is_empty()) {
            return Err(PyValueError::new_err(
                "ZipCrypto password must not be empty",
            ));
        }

        let inner_writer = InnerWriter::new(path_or_file_like)?;
        Ok(PyZipWriter {
            writer: Some(zip::ZipWriter::new(inner_writer)),
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

        let options = self.get_file_options()?;

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

        let options = self.get_file_options()?;

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
        exc_type: Option<Py<PyAny>>,
        exc_value: Option<Py<PyAny>>,
        traceback: Option<Py<PyAny>>,
    ) -> PyResult<bool> {
        self.close()?;
        Ok(false)
    }
}

impl PyZipWriter {
    /// Helper method to get file options with encryption if password is set
    fn get_file_options(&self) -> PyResult<FileOptions<'static, zip::write::ExtendedFileOptions>> {
        let mut options = FileOptions::default().compression_method(CompressionMethod::Deflated);

        if let Some(password) = &self.password {
            // Use the legacy ZipCrypto encryption from the unstable API. It is
            // available regardless of the `aes-crypto` feature, which is why
            // this crate can build zip without its default features.
            // Since zip 3 this returns a Result and does the empty-password
            // check that the constructor above duplicates for a better error.
            options = options
                .with_deprecated_encryption(password)
                .map_err(|e| PyValueError::new_err(format!("Invalid ZipCrypto password: {e}")))?;
        }

        Ok(options)
    }
}

/// A Python module implemented in Rust.
#[pymodule]
#[pyo3(name = "_rust")]
fn rustyzip(m: &Bound<'_, PyModule>) -> PyResult<()> {
    m.add_class::<PyZipWriter>()?;
    Ok(())
}
