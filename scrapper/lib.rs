// Import the pyo3 library, which provides the tools to create Python bindings for Rust code.
use pyo3::prelude::*;

// Define an enum for different types of attachments, making it usable in Python with the #[pyclass] attribute.
#[pyclass]
#[derive(Debug, Clone, PartialEq)]
enum AttachmentType {
    Image,
    Video,
    Audio,
    File,
}

// Define a Rust struct to represent an attachment, including its path and type.
// This struct is exposed to Python, allowing its properties to be accessed and modified.
#[pyclass(module = "scrapper", get_all, set_all)]
#[derive(Debug, Clone, PartialEq)]
struct Attachment {
    path: String,
    attachment_type: AttachmentType,
}

// Implement methods for the Attachment struct, including a constructor and a method to return a string representation.
#[pymethods]
impl Attachment {
    #[new]
    fn new(path: String, attachment_type: AttachmentType) -> Self {
        Attachment {
            path,
            attachment_type,
        }
    }
    
    fn __str__(&self) -> PyResult<String> {
        Ok(format!("Attachment(path={}, attachment_type={:?})", self.path, self.attachment_type))
    }
}

// Define a Rust struct to represent an email, including its subject, body, and attachments.
// This struct is also exposed to Python for manipulation.
#[pyclass(module = "scrapper", get_all, set_all)]
#[derive(Debug, Clone)]
struct Email {
    subject: String,
    body: String,
    attachments: Vec<Attachment>,
}

// Implement methods for the Email struct, including a constructor, a method for string representation,
// and a method to simulate sending the email.
#[pymethods]
impl Email {
    #[new]
    fn new(subject: String, body: String, attachments: Vec<Attachment>) -> Self {
        Email {
            subject,
            body,
            attachments,
        }
    }
    
    fn __str__(&self) -> PyResult<String> {
        Ok(format!("Email(subject={}, body={}, attachments={:?})", self.subject, self.body, self.attachments))
    }
    
    fn send(&mut self, to: String) -> PyResult<()> {
        println!("Sending email to: {}", to);
        println!("Subject: {}", self.subject);
        println!("Body: {}", self.body);
        for attachment in &self.attachments {
            println!("Attachment: {:?}", attachment);
        }
        Ok(())
    }
}

// Define a Python module using Rust code, allowing the Attachment, Email, and AttachmentType classes
// to be used in Python scripts.
#[pymodule]
fn scrapper_simple(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_class::<Attachment>()?;
    m.add_class::<Email>()?;
    m.add_class::<AttachmentType>()?;
    Ok(())
}
