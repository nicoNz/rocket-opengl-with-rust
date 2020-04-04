pub struct Camera {
    position: glm::Vec3,
    look_at_point: Option<glm::Vec3>,
    view_matrix: glm::Mat4,
    transform_matrix: glm::TMat4::<f32>
}
impl Camera {
    pub fn set_position(&mut self, position: &glm::Vec3) {
        println!("set x");
        self.position = position.clone();
        if let Some( lookat_point) = self.look_at_point {
            self.look_at(&lookat_point);
        }
        //self.transform_matrix = glm::set_column (&self.transform_matrix, 3, &glm::vec4(position.x, position.y, position.z, 1.) );
    }

    pub fn look_at(&mut self, target: &glm::Vec3) {
        self.transform_matrix = glm::look_at(&self.position, &target, &glm::vec3(0., 1., 0.));
       
        //self.transform_matrix= glm::look_at(&self.position,&target, &glm::vec3(0., 1., 0.)) ;
        self.look_at_point = Some(target.clone());
    }
    pub fn from_position_and_look_at(position: &glm::Vec3, target: &glm::Vec3) -> Self {
        let rotation_matrix = glm::look_at(&position, &target,  &glm::vec3(0., 0.1, 0.));
        println!("{}", position);
        println!("{}", target);
        println!("{}", &glm::look_at_rh(position, target, &glm::vec3(0., 1., 0.)));
        Self {
            position : position.clone(),
            look_at_point : Some(target.clone()),
            view_matrix : glm::perspective(16./9., 1.0, 1., 100.),
            // transform_matrix :  glm::translate(
            //     &glm::look_at(position, target, &glm::vec3(0., 1., 0.)),
            //     position
            // )
            transform_matrix :  glm::look_at_rh(position, &glm::vec3(0.0, 0.0, 0.), &glm::vec3(0., 1., 0.)),
         
            
        }
        
    }
    pub fn get_view_projection(&self) -> glm::Mat4 {
        //self.view_matrix * &glm::inverse(&self.transform_matrix)  
          //&self.view_matrix * &glm::inverse(&self.transform_matrix)
        &self.view_matrix * &self.transform_matrix 
    }
}
