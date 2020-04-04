pub struct Camera {
    position: glm::Vec3,
    look_at_point: Option<glm::Vec3>,
    view_matrix: glm::Mat4,
    transform_matrix: glm::TMat4::<f32>
}
impl Camera {
    pub fn set_position(&mut self, position: &glm::Vec3) {
        
        glm::set_column(&self.transform_matrix, 3, &glm::vec4(position.x, position.y, position.z, 1.) );
        self.position = position.clone();
    }

    pub fn look_at(&mut self, target: &glm::Vec3) {
        let rotation_matrix = glm::look_at(&target, &self.position, &glm::vec3(0., 1., 0.));
        self.transform_matrix = glm::translate(&rotation_matrix, &self.position);
        self.look_at_point = Some(target.clone());
    }
    pub fn from_position_and_look_at(position: &glm::Vec3, target: &glm::Vec3) -> Self {
        let rotation_matrix = glm::look_at(&target, &position, &glm::vec3(0., 1., 0.));

        Self {
            position : position.clone(),
            look_at_point : Some(target.clone()),
            view_matrix : glm::perspective(16./9., 45., 1., 100.),
            transform_matrix : glm::translate(&rotation_matrix, position)
        }
        
    }
}
