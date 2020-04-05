pub struct Camera {
    position: glm::Vec3,
    look_at_point: Option<glm::Vec3>,
    view_matrix: glm::Mat4,
    transform_matrix: glm::TMat4::<f32>
}
impl Camera {
    pub fn set_position_x(&mut self, x: f32) {
        self.set_position(&glm::vec3(
            x,
            self.position.y,
            self.position.z
        ))
    }
    pub fn set_position_y(&mut self, y: f32) {
        self.set_position(&glm::vec3(
            self.position.x,
            y,
            self.position.z
        ))
    }
    pub fn set_position_z(&mut self, z: f32) {
        self.set_position(&glm::vec3(
            self.position.x,
            self.position.y,
            z
        ))
    }
    pub fn set_position(&mut self, position: &glm::Vec3) {

        self.position = position.clone();
        if let Some( ref lookat_point) = self.look_at_point {
            self.transform_matrix = glm::look_at(position, lookat_point, &glm::vec3(0., 1., 0.));
        } else {
            self.transform_matrix = glm::set_column (&self.transform_matrix, 3, &glm::vec4(position.x, position.y, position.z, 1.) );
        }
    }

    pub fn look_at(&mut self, target: &glm::Vec3) {
        self.transform_matrix = glm::look_at(&self.position, &target, &glm::vec3(0., 1., 0.));
        self.look_at_point = Some(target.clone());
    }
    pub fn from_position_and_look_at(position: &glm::Vec3, target: &glm::Vec3) -> Self {

        Self {
            position : position.clone(),
            look_at_point : Some(target.clone()),
            view_matrix : glm::perspective(16./9., 1.0, 1., 100.),
            transform_matrix :  glm::look_at_rh(position, &glm::vec3(0.0, 0.0, 0.), &glm::vec3(0., 1., 0.)),
        }
    }
    pub fn get_view_projection(&self) -> glm::Mat4 {
        //self.view_matrix * &glm::inverse(&self.transform_matrix)  
          //&self.view_matrix * &glm::inverse(&self.transform_matrix)
        &self.view_matrix * &self.transform_matrix 
    }
}
