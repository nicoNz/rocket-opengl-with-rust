pub fn gen_checker_data() -> (Vec<u8>, Vec<f32>, Vec<f32>, Vec<f32>){
    let mut pos: Vec<f32> = Vec::new();
    pos.push(-5.0);
    pos.push(0.0);
    pos.push(-5.0);

    pos.push(-5.0);
    pos.push(0.0);
    pos.push(5.0);

    pos.push(5.0);
    pos.push(0.0);
    pos.push(-5.0);

    pos.push(5.0);
    pos.push(0.0);
    pos.push(5.0);

    let mut norms: Vec<f32> = Vec::new();
    norms.push(0.0);
    norms.push(1.0);
    norms.push(0.0);

    norms.push(0.0);
    norms.push(1.0);
    norms.push(0.0);

    norms.push(0.0);
    norms.push(1.0);
    norms.push(0.0);

    norms.push(0.0);
    norms.push(1.0);
    norms.push(0.0);


    let mut tex_coords: Vec<f32> = Vec::new();
    tex_coords.push(0.0);
    tex_coords.push(0.0);

    tex_coords.push(0.0);
    tex_coords.push(1.0);

    tex_coords.push(1.0);
    tex_coords.push(0.0);

    tex_coords.push(1.0);
    tex_coords.push(1.0);

    let mut indicies: Vec<u8> = Vec::new();
    indicies.push(0);
    indicies.push(1);
    indicies.push(2);

    indicies.push(3);
    indicies.push(2);
    indicies.push(1);

    (indicies, pos, norms, tex_coords)

}