use std::f64::consts::PI;

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct TailServerUrl {
    url: String,
    z: u8,
}

impl TailServerUrl {
    /// - url:
    /// - z: The zoom parameter is an integer between 0 (zoomed out) and 18 (zoomed in).
    /// 18 is normally the maximum, but some tile servers might go beyond that.
    pub fn new(url: impl Into<String>, z: u8) -> TailServerUrl {
        TailServerUrl { url: url.into(), z }
    }

    /// # OSM 'standard' style
    /// https://tile.openstreetmap.org/{z}/{x}/{y}.png
    ///
    /// zoomlevels: 0-19
    pub fn new_openstreetmap(z: u8) -> TailServerUrl {
        TailServerUrl::new("https://tile.openstreetmap.org/{z}/{x}/{y}.png", z)
    }

    pub fn deg2num(lat: f64, lon: f64, zoom: u8) -> (u64, u64) {
        let n = (1 << zoom) as f64;
        let x = n * ((lon + 180.0) / 360.0);
        let y = (1.0 - lat.to_radians().tan().asinh() / PI) / 2.0 * n;
        let x = unsafe { x.to_int_unchecked() };
        let y = unsafe { y.to_int_unchecked() };
        (x, y)
    }

    pub fn deg(&self, lat_deg: f64, lon_deg: f64) -> String {
        let (x, y) = TailServerUrl::deg2num(lat_deg, lon_deg, self.z);
        self.url(x, y)
    }

    pub fn url(&self, x: u64, y: u64) -> String {
        self.url
            .replace("{x}", &x.to_string())
            .replace("{y}", &y.to_string())
            .replace("{z}", &self.z.to_string())
    }

    /// - lat0: left most Longitude (min lng)
    /// - lon0: bottom most Latitude (min lat)
    /// - lat1: right most Longitude (max lng)
    /// - lon1: = top most Latitude (max lat)
    ///
    /// NOTE: No support for wrapping
    pub fn deg_box(&self, lat0: f64, lon0: f64, lat1: f64, lon1: f64) -> TailServerUrlIter {
        let (x0, y0) = TailServerUrl::deg2num(lat0, lon0, self.z);
        let (x1, y1) = TailServerUrl::deg2num(lat1, lon1, self.z);

        TailServerUrlIter::new(self.clone(), x0, y0, x1, y1)
    }
}

#[derive(Debug, Eq, PartialEq, Clone)]
pub struct TailServerUrlIter {
    tsu: TailServerUrl,
    x: u64,
    y: u64,
    x0: u64,
    y0: u64,
    x1: u64,
    y1: u64,
}

impl TailServerUrlIter {
    fn new(tsu: TailServerUrl, x0: u64, y0: u64, x1: u64, y1: u64) -> TailServerUrlIter {
        TailServerUrlIter {
            tsu,
            x: x0,
            y: y1,
            x0,
            y0,
            x1,
            y1,
        }
    }

    /// The number of x and y tails
    pub fn size(&self) -> (u64, u64) {
        (self.x1 - self.x0 + 1, self.y0 - self.y1 + 1)
    }
}

impl Iterator for TailServerUrlIter {
    type Item = String;

    fn next(&mut self) -> Option<Self::Item> {
        let url = Some(self.tsu.url(self.x, self.y));
        if self.x < self.x1 {
            self.x += 1;
            url
        } else if self.y < self.y0 {
            self.y += 1;
            self.x = self.x0;
            url
        } else if (self.x, self.y) != (u64::MAX, u64::MAX) {
            self.x = u64::MAX;
            self.y = u64::MAX;
            url
        } else {
            None
        }
    }
}
