pub struct Program {
    gl: gl::Gl,
    id: gl::types::GLuint,
    uniforms: std::collections::HashMap<GLint, Uniform>,
    shader_description: Box<ShaderDescription>
    // u_offset_value : f32,
    // u_vp : gl::types::GLint,
    // pub u_vp_value : glm::Mat4,
}

impl Drop for Program {
    fn drop(&mut self) {
        unsafe {
            self.gl.DeleteProgram(self.id);
        }
    }
}


impl Program {
    pub fn set_uniform(&mut self, loc: GLint, value: UniformTypedValue) {
        
        if let Some(v) = self.uniforms.get_mut(&loc) {
            v.value = value;
        } else {
            println!("key {} did not exist", loc);
        };
        //std::collections::HashMap<UniformRole, UniformRole>
    }

    pub fn register_uniform(&mut self, location_name: &String, value: UniformTypedValue, role: UniformRole) -> GLint {
        let loc = self.get_uniform_location(location_name);
        match loc {
            Ok(loc)=> {
                println!("name {} found at loc {}", location_name, loc);
                if !self.uniforms.insert(
                    loc, 
                    Uniform {
                        value,
                        loc,
                        name: location_name.clone(),
                        role 
                    }
                ).is_none() {
                    println!("Error while trying to get location of {}, key {} already exist",location_name, loc)
                }
                loc
            },
            Err(())=>panic!("panic because the behavious when a location is not found is not implemented")
        }
    }
    
    pub fn from_shaders(gl: &gl::Gl, shaders: &[Shader]) -> Result<Program, String> {
        let program_id = unsafe { gl.CreateProgram() };

        for shader in shaders {
            unsafe {
                gl.AttachShader(program_id, shader.id());
            }
        }

        unsafe {
            gl.LinkProgram(program_id);
        }

        let mut success: gl::types::GLint = 1;
        unsafe {
            gl.GetProgramiv(program_id, gl::LINK_STATUS, &mut success);
        }

        if success == 0 {
            let mut len: gl::types::GLint = 0;
            unsafe {
                gl.GetProgramiv(program_id, gl::INFO_LOG_LENGTH, &mut len);
            }

            let error = create_whitespace_cstring_with_len(len as usize);

            unsafe {
                gl.GetProgramInfoLog(
                    program_id,
                    len,
                    std::ptr::null_mut(),
                    error.as_ptr() as *mut gl::types::GLchar,
                );
            }

            return Err(error.to_string_lossy().into_owned());
        }

        for shader in shaders {
            unsafe {
                gl.DetachShader(program_id, shader.id());
            }
        }
       

        Ok(Program { 
            uniforms : std::collections::HashMap::new(),
            gl : gl.clone(),
            id: program_id,
            // u_offset : u_offset_loc,
            // u_offset_value : 0.0,
            // u_vp : u_vp_loc,
            // u_vp_value : glm::translate(&glm::identity(), &glm::vec3(0.5, 0., 0.)) 
        })
    }

    pub fn from_shader_description(gl: &gl::Gl, shader_description: &ShaderDescription) -> Result<Self, String> {
        match (
            shader_description.vertex_shader_file,
            shader_description.fragment_shader_file 
        ) {
            (
                Some(fragment_shader_file), 
                Some(vertex_shader_file)
            ) => {
                match (
                    get_cstr_from_path(&vertex_shader_file),
                    get_cstr_from_path(&fragment_shader_file)
                ) {
                    (
                        Ok(vertex_source),
                        Ok(fragment_source)
                    ) => {
                        match (
                            Shader::from_vert_source(gl, vertex_source.as_c_str()),
                            Shader::from_frag_source(gl, fragment_source.as_c_str())
                        ) {
                            (
                                Ok(vertex_shader),
                                Ok(fragmentShader)
                            ) => {
                                Self::from_shaders(gl, &[vertex_shader, fragmentShader])
                            }
                        }
                        
                    },
                    _ => Err(String::from("at least on shader source asn't found from file path"))
                }
            },
            _ => {
                Err(String::from("raw shaders were missing from description"))
            }
        }
    }

    pub fn get_attribute_location(&self, attribute_name: &String) -> Result<GLint, ()> {
        let mut string = attribute_name.clone();
        string.push('\0');

        match CStr::from_bytes_with_nul(&string.into_bytes()) {
            Ok(cstr) => {
                let loc = unsafe {self.gl.GetAttribLocation(self.id, cstr.as_ptr())};
                if loc >= 0 {
                    return Ok(loc);
                } else {
                    println!("location not found for attribute named {}", attribute_name);
                    Err(())
                }
            }
            _ => {
                println!("uniform named {} is not a formatted as a C string", attribute_name);
                Err(())
            }
        }
    }

    pub fn get_uniform_location(&self, uniform_name: &String) -> Result<GLint, ()>{
        let mut string = uniform_name.clone();
        string.push('\0');

        match CStr::from_bytes_with_nul(&string.into_bytes()) {
            Ok(cstr) => {
                let loc =  unsafe {self.gl.GetUniformLocation(self.id, cstr.as_ptr()) };
                    
                if loc >= 0 {
                    return Ok(loc);
                } else {
                    println!("location not found for uniform named {}", uniform_name);
                    Err(())
                }
            }
            _ => {
                println!("uniform named {} is not a formatted as a C string", uniform_name);
                Err(())
            }
        }

    }

    pub fn id(&self) -> gl::types::GLuint {
        self.id
    }

    pub fn set_used(&self) {
        unsafe {
            // self.gl.Uniform1f(self.u_offset, self.u_offset_value);
            // self.gl.UniformMatrix4fv(self.u_vp, 1, gl::FALSE, self.u_vp_value.as_ptr());
            let gl = &self.gl;
            for uniform in self.uniforms.values() {
                uniform.load_into_program(gl);
                //self.gl.Uniform1f(self.u_offset, self.u_offset_value);
            }
            self.gl.UseProgram(self.id);
        }
    }
}
