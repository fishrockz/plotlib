
#[derive(Debug)]
pub struct ColorMap {
    colors: Vec<(f64, u8, u8, u8)>
}

pub trait ColorMapping {
    fn to_str(&self, v: f64) -> String;
}

impl ColorMap{
    pub fn new() -> Self {
        let cols: Vec<(f64, u8, u8, u8)> = [(0.0, 0, 0, 0), (1.0, 255, 255, 255)].to_vec();
        ColorMap{colors: cols}
    }

    pub fn get_color(&self, v: f64) -> (u8, u8, u8){
    ((v.max(0.0).min(1.0) * 255.0) as u8, 
        (v.max(0.0).min(1.0) * 255.0) as u8, 
        (v.max(0.0).min(1.0) * 255.0) as u8)
    }
}

impl ColorMapping for ColorMap {
    fn to_str(&self, v: f64) -> String {
        let uvals = self.get_color(v);

        //println!("{:?}, {:?}", v, uvals);
        let val = hex::encode( vec![uvals.0, uvals.1, uvals.2] ) ;
        String::from("#") + &String::from(val)
    }  
}
