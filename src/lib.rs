use std::f64::consts::PI;

pub fn add(left: usize, right: usize) -> usize {
    left + right
}

struct TailServerUrl {
    url: String,
    z: u8,
}

impl TailServerUrl {
    pub fn new(url: impl Into<String>, z: u8) -> TailServerUrl {
        TailServerUrl { url: url.into(), z }
    }

    pub fn new_openstreetmap(z: u8) -> TailServerUrl {
        TailServerUrl::new("https://tile.openstreetmap.org/{z}/{x}/{y}.png", z)
    }

    pub fn deg2num(lat_deg: f64, lon_deg: f64, zoom: u8) -> (u64, u64) {
        let n = (1 << zoom) as f64;
        let x = n * ((lon_deg + 180.0) / 360.0);
        let y = (1.0 - lat_deg.to_radians().tan().asinh() / PI) / 2.0 * n;
        let x = unsafe { x.to_int_unchecked() };
        let y = unsafe { y.to_int_unchecked() };
        (x, y)
    }

    pub fn deg(&self, lat_deg: f64, lon_deg: f64) -> String {
        let (x, y) = TailServerUrl::deg2num(lat_deg, lon_deg, self.z);
        self.url
            .replace("{x}", &x.to_string())
            .replace("{y}", &y.to_string())
            .replace("{z}", &self.z.to_string())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        assert_eq!(
            "https://tile.openstreetmap.org/17/67400/43241.png".to_string(),
            TailServerUrl::new_openstreetmap(17).deg(52.090752, 5.121630)
        );
        assert_eq!((526, 337), TailServerUrl::deg2num(52.090752, 5.121630, 10));
    }
}
