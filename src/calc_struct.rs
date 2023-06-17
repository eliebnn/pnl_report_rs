use crate::trade_struct::Trade;
use crate::trade_struct::UnwindTrade;

pub trait Core {

    fn get_side(&mut self) -> &mut i8;
    fn get_stack(&mut self) -> &mut Vec<Trade>;
    fn get_unwind_trades(&mut self) -> &mut Vec<UnwindTrade>;

    fn to_stack(&mut self, t: &Trade) {
        (*self.get_stack()).push(t.clone());
    }

    fn empty_stack(&mut self, t: &Trade) {
        self.to_stack(t);
    
        (*self.get_side()) = match t.qty {
            _ if t.qty > 0.0 => 1,
            _ => -1
        }
    }

    fn less_qty_func(&mut self, t: &Trade) {

        let mut qty_cumsum: f32 = 0.0;
        let mut count: usize = 0;

        let mut tmp_vec: Vec<UnwindTrade> = vec![];

        for (i, k) in self.get_stack().iter().enumerate(){
            qty_cumsum += k.qty;

            if t.qty.abs() > qty_cumsum.abs() {
                tmp_vec.push(UnwindTrade{qty: k.qty, price: k.price, unwind_price: t.price})
            }

            if t.qty.abs() <= qty_cumsum.abs(){
                count = i;
                break;
            }
        }

        (*self.get_unwind_trades()).extend(tmp_vec);

        if t.qty.abs() == self.get_stack()[count].qty.abs(){

            let utrade = UnwindTrade{qty: self.get_stack()[count].qty, price: self.get_stack()[count].price, unwind_price: t.price};
            (*self.get_unwind_trades()).push(utrade);

            *self.get_stack() = self.get_stack()[(count+1)..].to_vec();
        } 

        else if qty_cumsum.abs() - t.qty.abs()  < self.get_stack()[count].qty.abs(){

            let factor = self.get_stack()[count].qty / self.get_stack()[count].qty.abs();
            let balance: f32 = (qty_cumsum.abs() - t.qty.abs()) * factor;
            let unwind_qty: f32 = (self.get_stack()[count].qty.abs() - balance.abs()) * factor;

            let utrade = UnwindTrade{qty: unwind_qty, price: self.get_stack()[count].price, unwind_price: t.price}; 

            (*self.get_unwind_trades()).push(utrade);

            if count == (self.get_stack().len() - 1) {
                *self.get_stack() = Vec::from([Trade{qty: balance, price:self.get_stack()[count].price}]);
            }
            else {
            
                let mut tmp = Vec::from([Trade{qty: balance, price:self.get_stack()[count].price}]);
                tmp.append(&mut self.get_stack()[(count+1)..].to_vec());
                *self.get_stack() = tmp;
            }
        } 
    }

    fn same_qty_func(&mut self, t: &Trade) {

        let mut tmp_vec: Vec<UnwindTrade> = vec![];

        for k in self.get_stack().iter(){
            tmp_vec.push(UnwindTrade{qty: k.qty, price: k.price, unwind_price: t.price})
        }

        (*self.get_unwind_trades()).extend(tmp_vec);

        (*self.get_stack()).clear();
        *self.get_side() = 0;
    }

    fn more_qty_func(&mut self, t: &Trade) {

        let mut tmp_vec: Vec<UnwindTrade> = vec![];

        for k in self.get_stack().iter(){
            tmp_vec.push(UnwindTrade{qty: k.qty, price: k.price, unwind_price: t.price})
        }

        (*self.get_unwind_trades()).extend(tmp_vec);

        let factor = t.qty / t.qty.abs();
        let balance: f32 = (t.qty.abs() - self.count_stack().abs()) * factor;
        *self.get_stack() = Vec::from([Trade{qty: balance, price:t.price}]);
        *self.get_side() *= -1;
    }

    fn count_stack(&mut self) -> f32 {
        self.get_stack().iter().map(|k| k.qty).sum()
    }

    fn run(&mut self, trades: &[Trade]) {

        for t in trades{

            if self.get_stack().len() == 0 {
                self.empty_stack(t);
            }
    
            else if t.qty.signum() == self.get_side().signum().into() {
                self.to_stack(t);
            }
    
            else if t.qty.abs() == self.count_stack().abs() {
                self.same_qty_func(t);                   
            }
    
            else if t.qty.abs() < self.count_stack().abs() {
                self.less_qty_func(t);
            }
    
            else if t.qty.abs() > self.count_stack().abs() {
                self.more_qty_func(t);
            } 
        }
    }
}

// --

pub struct FIFO {
    pub stack: Vec<Trade>,
    pub unwind_trades: Vec<UnwindTrade>,
    pub side: i8
}

impl FIFO {

    pub fn new() -> FIFO {
        FIFO {
            stack: vec![],
            unwind_trades: vec![],
            side: 0
        }
    }    
}

impl Core for FIFO {

    fn get_side(&mut self) -> &mut i8 {
        &mut self.side
    }

    fn get_stack(&mut self) -> &mut Vec<Trade>{
        &mut self.stack
    }

    fn get_unwind_trades(&mut self) -> &mut Vec<UnwindTrade>{
        &mut self.unwind_trades
    }
}

// --

pub struct LIFO {
    pub stack: Vec<Trade>,
    pub unwind_trades: Vec<UnwindTrade>,
    pub side: i8
}

impl LIFO {

    pub fn new() -> LIFO {
        LIFO {
            stack: vec![],
            unwind_trades: vec![],
            side: 0
        }
    }    
}

impl Core for LIFO {

    fn get_side(&mut self) -> &mut i8 {
        &mut self.side
    }

    fn get_stack(&mut self) -> &mut Vec<Trade>{
        &mut self.stack
    }

    fn get_unwind_trades(&mut self) -> &mut Vec<UnwindTrade>{
        &mut self.unwind_trades
    }

    fn to_stack(&mut self, t: &Trade) {
        (*self.get_stack()).insert(0, t.clone());
    }
}