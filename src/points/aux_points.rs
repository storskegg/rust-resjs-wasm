pub struct AuxPoints {
    open: u32,
    close: u32,
    cartouche_open: u32,
    cartouche_segment: u32,
    cartouche_close: u32,
    oval_open: u32,
    oval_segment: u32,
    oval_close: u32,
    serekh_open: u32,
    serekh_segment: u32,
    serekh_close: u32,
    inb_open: u32,
    inb_segment: u32,
    inb_close: u32,
    rectangle_open: u32,
    rectangle_segment: u32,
    rectangle_close: u32,
    hwt_open_over_open: u32,
    hwt_open_over_segment: u32,
    hwt_open_over_close: u32,
    hwt_open_under_open: u32,
    hwt_open_under_segment: u32,
    hwt_open_under_close: u32,
    hwt_close_over_open: u32,
    hwt_close_over_segment: u32,
    hwt_close_over_close: u32,
    hwt_close_under_open: u32,
    hwt_close_under_segment: u32,
    hwt_close_under_close: u32,
    hlr: u32,
    hrl: u32,
    vlr: u32,
    vrl: u32,
}

impl AuxPoints {
    pub fn new() -> Self {
        AuxPoints {
            open: 35,
            close: 36,

            cartouche_open: 35,
            cartouche_segment: 37,
            cartouche_close: 36,

            oval_open: 35,
            oval_segment: 37,
            oval_close: 41,

            serekh_open: 38,
            serekh_segment: 37,
            serekh_close: 40,

            inb_open: 42,
            inb_segment: 44,
            inb_close: 43,

            rectangle_open: 38,
            rectangle_segment: 37,
            rectangle_close: 39,

            hwt_open_over_open: 45,
            hwt_open_over_segment: 37,
            hwt_open_over_close: 39,

            hwt_open_under_open: 46,
            hwt_open_under_segment: 37,
            hwt_open_under_close: 39,

            hwt_close_over_open: 38,
            hwt_close_over_segment: 37,
            hwt_close_over_close: 47,

            hwt_close_under_open: 38,
            hwt_close_under_segment: 37,
            hwt_close_under_close: 48,

            hlr: 49,
            hrl: 50,
            vlr: 51,
            vrl: 52,
        }
    }

    pub fn open(&self) -> u32 {
        self.open
    }
    pub fn close(&self) -> u32 {
        self.close
    }
    pub fn cartouche_open(&self) -> u32 {
        self.cartouche_open
    }
    pub fn cartouche_segment(&self) -> u32 {
        self.cartouche_segment
    }
    pub fn cartouche_close(&self) -> u32 {
        self.cartouche_close
    }
    pub fn oval_open(&self) -> u32 {
        self.oval_open
    }
    pub fn oval_segment(&self) -> u32 {
        self.oval_segment
    }
    pub fn oval_close(&self) -> u32 {
        self.oval_close
    }
    pub fn serekh_open(&self) -> u32 {
        self.serekh_open
    }
    pub fn serekh_segment(&self) -> u32 {
        self.serekh_segment
    }
    pub fn serekh_close(&self) -> u32 {
        self.serekh_close
    }
    pub fn inb_open(&self) -> u32 {
        self.inb_open
    }
    pub fn inb_segment(&self) -> u32 {
        self.inb_segment
    }
    pub fn inb_close(&self) -> u32 {
        self.inb_close
    }
    pub fn rectangle_open(&self) -> u32 {
        self.rectangle_open
    }
    pub fn rectangle_segment(&self) -> u32 {
        self.rectangle_segment
    }
    pub fn rectangle_close(&self) -> u32 {
        self.rectangle_close
    }
    pub fn hwt_open_over_open(&self) -> u32 {
        self.hwt_open_over_open
    }
    pub fn hwt_open_over_segment(&self) -> u32 {
        self.hwt_open_over_segment
    }
    pub fn hwt_open_over_close(&self) -> u32 {
        self.hwt_open_over_close
    }
    pub fn hwt_open_under_open(&self) -> u32 {
        self.hwt_open_under_open
    }
    pub fn hwt_open_under_segment(&self) -> u32 {
        self.hwt_open_under_segment
    }
    pub fn hwt_open_under_close(&self) -> u32 {
        self.hwt_open_under_close
    }
    pub fn hwt_close_over_open(&self) -> u32 {
        self.hwt_close_over_open
    }
    pub fn hwt_close_over_segment(&self) -> u32 {
        self.hwt_close_over_segment
    }
    pub fn hwt_close_over_close(&self) -> u32 {
        self.hwt_close_over_close
    }
    pub fn hwt_close_under_open(&self) -> u32 {
        self.hwt_close_under_open
    }
    pub fn hwt_close_under_segment(&self) -> u32 {
        self.hwt_close_under_segment
    }
    pub fn hwt_close_under_close(&self) -> u32 {
        self.hwt_close_under_close
    }
    pub fn hlr(&self) -> u32 {
        self.hlr
    }
    pub fn hrl(&self) -> u32 {
        self.hrl
    }
    pub fn vlr(&self) -> u32 {
        self.vlr
    }
    pub fn vrl(&self) -> u32 {
        self.vrl
    }
}
