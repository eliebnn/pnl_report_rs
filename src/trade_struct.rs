
#[derive(Clone)]
pub struct Trade {
    pub qty: f32,
    pub price: f32
}

pub struct UnwindTrade {
    pub qty: f32,
    pub price: f32,
    pub unwind_price: f32
}

impl UnwindTrade {
    pub fn get_pnl(&self) -> f32 {
        (self.unwind_price - self.price) * self.qty
    }
}  
