use ndarray::Array2;
use crate::screentone::create_dot::create_dot;
use crate::screentone::r#enum::TypeDot;
pub fn rotate_pixel_coordinates(
    x: f32,
    y: f32,
    center_y: f32,
    center_x: f32,
    cos_theta: f32,
    sin_theta: f32,
) -> (usize, usize) {
    let x_rel = x - center_x;
    let y_rel = y - center_y;

    let rotated_x = (cos_theta * x_rel - sin_theta * y_rel + center_x) as usize;
    let rotated_y = (sin_theta * x_rel + cos_theta * y_rel + center_y) as usize;

    (rotated_x, rotated_y)
}

pub fn compute_cos_sin(theta: f32) -> [f32; 2] {
    let cos_theta = theta.cos();
    let sin_theta = theta.sin();
    [cos_theta, sin_theta]
}
fn angle_mask(mask: &mut Array2<f32>,
              dot_size: usize,
              angle: f32,
              dot_type: Option<TypeDot>){

    let (dot, dot_inv) = create_dot(dot_size,dot_type.unwrap_or(TypeDot::CIRCLE));
    let (h,w)=(mask.shape()[0],mask.shape()[1]);
    let lx_bias = w / 2;
    let ly_bias = h / 2;
    let cos_sin = compute_cos_sin(angle);
    for ly in 0..h {
        let ly2 = ly + ly_bias;
        for lx in 0..w {
            let lx2 = lx + lx_bias;
            let rot = rotate_pixel_coordinates(
                lx2 as f32, ly2 as f32, w as f32, h as f32, cos_sin[0], cos_sin[1],
            );
            mask[[ly,lx]] = if (rot.1 / dot_size + rot.0 / dot_size) % 2 == 1 {
                dot_inv[[rot.0 % dot_size, rot.1 % dot_size]]

            } else {
                dot[[rot.0 % dot_size, rot.1 % dot_size]]
            };
        }
    }
}
fn not_angle_mask(mask: &mut Array2<f32>,
              dot_size: usize,
              dot_type: Option<TypeDot>){

    let (dot, dot_inv) = create_dot(dot_size,dot_type.unwrap_or(TypeDot::CIRCLE));
    let (h,w)=(mask.shape()[0],mask.shape()[1]);
    let lx_bias = dot_size / 2;
    let ly_bias = dot_size / 2;
    for ly in 0..h {
        let ly2 = ly + ly_bias;
        let colum = ly2 / dot_size;
        for lx in 0..w {
            let lx2 = lx + lx_bias;
            mask[[ly,lx]] = if (colum + lx2 / dot_size) % 2 == 1 {
                dot_inv[[lx2 % dot_size, ly2 % dot_size]]
            } else {

                dot[[lx2 % dot_size, ly2 % dot_size]]
            };
        }
    }
}
pub fn create_mask(    mask_size:[usize;2],
                             dot_size: usize,
                             angle: f32,
                             dot_type: Option<TypeDot>) -> Array2<f32> {
    let mut mask = Array2::<f32>::zeros((mask_size[0],mask_size[1]));
    if angle==0.0{
        not_angle_mask(&mut mask, dot_size, dot_type);
    }
    else {
        angle_mask(&mut mask, dot_size, angle.to_radians(), dot_type);
    }
    mask
}