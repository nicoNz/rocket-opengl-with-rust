
use gl;
use nalgebra_glm::Mat4;

pub struct UniformData {
    location: gl::types::GLuint,
    label: String,
    name: String,
    data_type: UniformType,
    value: UniformValue
}

impl UniformData {
    pub fn set_f32(&mut self, value: f32) -> Result<(), UniformError> {
        match self.data_type {
            UniformType::F32 => {
                self.value = UniformValue::F32(value);
                Ok(())
            },
            _ => Err(UniformError::BadType)
        }        
    }
}


enum UniformType {
    F32,
    MAT4
}

enum UniformValue {
    F32(f32),
    MAT4(Mat4)
}

pub enum UniformError {
    BadType
}