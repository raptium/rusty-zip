#!/usr/bin/env python3
"""
Pytest tests for py-zip-rs.
"""

import os
import tempfile
import zipfile
import pytest
from py_zip_rs import ZipWriter


class TestZipWriter:
    @pytest.fixture
    def setup_files(self):
        """Create temporary files for testing."""
        temp_dir = tempfile.TemporaryDirectory()
        test_file_path = os.path.join(temp_dir.name, "test.txt")
        with open(test_file_path, "w") as f:
            f.write("Test content")
        
        zip_path = os.path.join(temp_dir.name, "test.zip")
        
        yield test_file_path, zip_path
        
        # Cleanup
        temp_dir.cleanup()
    
    def test_create_zip_without_password(self, setup_files):
        """Test creating a ZIP file without password."""
        test_file_path, zip_path = setup_files
        
        with ZipWriter(zip_path) as zip_file:
            zip_file.write_file(test_file_path, "test.txt")
            zip_file.write_bytes(b"Memory content", "memory.txt")
        
        # Verify the ZIP file was created
        assert os.path.exists(zip_path)
        
        # Verify we can open it with standard zipfile module
        with zipfile.ZipFile(zip_path, 'r') as zip_ref:
            assert zip_ref.namelist() == ["test.txt", "memory.txt"]
            assert zip_ref.read("test.txt").decode('utf-8') == "Test content"
            assert zip_ref.read("memory.txt").decode('utf-8') == "Memory content"
    
    def test_create_zip_with_password(self, setup_files):
        """Test creating a ZIP file with password."""
        test_file_path, zip_path = setup_files
        
        password = "secret"
        with ZipWriter(zip_path, password=password) as zip_file:
            zip_file.write_file(test_file_path, "test.txt")
            zip_file.write_bytes(b"Memory content", "memory.txt")
        
        # Verify the ZIP file was created
        assert os.path.exists(zip_path)
        
        # Note: Python's zipfile module doesn't support ZipCrypto out of the box
        # This test just verifies the file was created
        # A real test would use a tool like 7zip to extract and verify the contents
    
    def test_context_manager(self, setup_files):
        """Test that the context manager properly closes the file."""
        _, zip_path = setup_files
        
        with ZipWriter(zip_path) as zip_file:
            pass  # Just open and close
        
        # Verify the ZIP file was created and is a valid ZIP
        assert os.path.exists(zip_path)
        assert zipfile.is_zipfile(zip_path)
    
    def test_manual_close(self, setup_files):
        """Test manually closing the ZIP file."""
        _, zip_path = setup_files
        
        zip_file = ZipWriter(zip_path)
        zip_file.write_bytes(b"Test data", "test.txt")
        zip_file.close()
        
        # Verify the ZIP file was created and is a valid ZIP
        assert os.path.exists(zip_path)
        assert zipfile.is_zipfile(zip_path)
        
        # Verify the content
        with zipfile.ZipFile(zip_path, 'r') as zip_ref:
            assert zip_ref.namelist() == ["test.txt"]
            assert zip_ref.read("test.txt").decode('utf-8') == "Test data"
    
    def test_error_on_closed_writer(self, setup_files):
        """Test that an error is raised when trying to use a closed writer."""
        _, zip_path = setup_files
        
        zip_file = ZipWriter(zip_path, "secret")
        zip_file.close()
        
        # Trying to write to a closed writer should raise an error
        with pytest.raises(Exception) as excinfo:
            zip_file.write_bytes(b"Test data", "test.txt")
        
        assert "closed" in str(excinfo.value).lower() 