use crate::trade_struct::Trade;
use crate::trade_struct::UnwindTrade;

pub struct Core {
    stack: Vec<Trade>,
    unwind_trades: Vec<UnwindTrade>,
    side: i8
}

impl Default for Core {
    fn default() -> Core {
        Core{stack:vec![], unwind_trades:vec![], side: 0}
    }
}

impl Core {

    fn to_stack(&mut self, t: &Trade) {
        self.stack.push(t.clone());
    }

    fn empty_stack(&mut self, t: &Trade) {
        self.to_stack(t);
    
        self.side = match t.qty {
            _ if t.qty > 0.0 => 1,
            _ => -1
        }
    }

    fn less_qty_func(&mut self, t: &Trade) {

        let mut qty_cumsum: f32 = 0.0;
        let mut count: usize = 0;

        for (i, k) in self.stack.iter().enumerate(){
            qty_cumsum += k.qty;

            if t.qty.abs() > qty_cumsum.abs() {
                self.unwind_trades.push(UnwindTrade{qty: k.qty, price: k.price, unwind_price: t.price})
            }

            if t.qty.abs() <= qty_cumsum.abs(){
                count = i;
                break;
            }
        }

        if t.qty.abs() == self.stack[count].qty.abs(){

            self.unwind_trades.push(UnwindTrade{qty: self.stack[count].qty, price: self.stack[count].price, unwind_price: t.price});
            self.stack = self.stack[(count+1)..].to_vec();
        } 

        else if qty_cumsum.abs() - t.qty.abs()  < self.stack[count].qty.abs(){

            let factor = self.stack[count].qty / self.stack[count].qty.abs();
            let balance: f32 = (qty_cumsum.abs() - t.qty.abs()) * factor;
            let unwind_qty: f32 = (self.stack[count].qty.abs() - balance.abs()) * factor;

            self.unwind_trades.push(UnwindTrade{qty: unwind_qty, price: self.stack[count].price, unwind_price: t.price});

            if count == (self.stack.len() - 1) {
                self.stack = Vec::from([Trade{qty: balance, price:self.stack[count].price}]);
            }
            else {
            
                let mut tmp = Vec::from([Trade{qty: balance, price:self.stack[count].price}]);
                tmp.append(&mut self.stack[(count+1)..].to_vec());
                self.stack = tmp;
            }
        } 
    }

    fn same_qty_func(&mut self, t: &Trade) {

        for k in self.stack.iter(){
            self.unwind_trades.push(UnwindTrade{qty: k.qty, price: k.price, unwind_price: t.price})
        }

        self.stack.clear();
        self.side = 0;
    }

    fn more_qty_func(&mut self, t: &Trade) {

        for k in self.stack.iter(){
            self.unwind_trades.push(UnwindTrade{qty: k.qty, price: k.price, unwind_price: t.price})
        }

        let factor = t.qty / t.qty.abs();
        let balance: f32 = (t.qty.abs() - self.count_stack().abs()) * factor;
        self.stack = Vec::from([Trade{qty: balance, price:t.price}]);
        self.side *= -1;

    }

    pub fn new() -> Core {
        Core {
            stack: vec![],
            unwind_trades: vec![],
            side: 0
        }
    }

    fn count_stack(&self) -> f32 {
        self.stack.iter().map(|k| k.qty).sum()
    }

    pub fn run(&mut self, trades: &[Trade]) {

        for t in trades{

            if self.stack.len() == 0 {
                self.empty_stack(t);
            }
    
            else if t.qty.signum() == self.side.signum().into() {
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

