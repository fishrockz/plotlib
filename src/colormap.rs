#[derive(Debug)]
pub struct ColorMap {
    colors: Vec<(f64, u8, u8, u8)>
}

impl ColorMap{
    pub fn new() -> Self {
        let cols: Vec<(f64, u8, u8, u8)> = [(0.0, 0, 0, 0), (1.0, 255, 255, 255)].to_vec();
        ColorMap{colors: cols}
    }

    pub fn get_color(self, v: f64) -> (u8, u8, u8){
        ((v.max(1.0).min(0.0) * 255.0) as u8, 
        (v.max(1.0).min(0.0) * 255.0) as u8, 
        (v.max(1.0).min(0.0) * 255.0) as u8)
    }
    pub fn to_str(self, v: f64) -> String{
        String::from("#FF00FF")
    }
}

