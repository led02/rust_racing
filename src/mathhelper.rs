

pub fn angleToDirectionVector(angle:f64) ->(f64, f64){
    let x = angle.sin();
    let y = angle.cos();

    (x,y)

}