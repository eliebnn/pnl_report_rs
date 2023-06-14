use crate::trade_struct::Trade;
use crate::trade_struct::UnwindTrade;

    pub struct Core {
        trades: Vec<Trade>,
        stack: Vec<Trade>,
        unwind_trades: Vec<UnwindTrade>,
        side: i8
    }

    impl Default for Core {
        fn default() -> Core {
            Core{trades:vec![], stack:vec![], unwind_trades:vec![], side: 0}
        }
    }

    impl Core {

        // fn empty_stack() {}
        // fn less_qty_func() {}
        // fn same_qty_func() {}
        // fn more_qty_func() {}

        // fn to_stack(&mut self, t: &Trade) {
        //     self.stack.push(t.clone());
        // }

        pub fn new(trades: Vec<Trade>) -> Core {
            Core {
                trades: trades,
                stack: vec![],
                unwind_trades: vec![],
                side: 0
            }
        }

        fn count_stack(&self) -> f32 {
            self.stack.iter().map(|k| k.qty).sum()
        }

        pub fn run(&mut self) {

            for t in &self.trades{

        
                if self.stack.len() == 0 {
        
                    self.stack.push(t.clone());
        
                    self.side = match t.qty {
                        _ if t.qty > 0.0 => 1,
                        _ => -1
                    }
                }
        
                else if t.qty.signum() == self.side.signum().into() {
                    self.stack.push(t.clone());
                    // self.to_stack(t);
                }
        
                else if t.qty.abs() == self.count_stack().abs() {
        
                    for k in self.stack.iter(){
                        self.unwind_trades.push(UnwindTrade{qty: k.qty, price: k.price, unwind_price: t.price})
                    }
        
                    self.stack.clear();
                    self.side = 0;
                    
                }
        
                else if t.qty.abs() < self.count_stack().abs() {
                    
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
        
                else if t.qty.abs() > self.count_stack().abs() {
                    
                    for k in self.stack.iter(){
                        self.unwind_trades.push(UnwindTrade{qty: k.qty, price: k.price, unwind_price: t.price})
                    }
        
                    let factor = t.qty / t.qty.abs();
                    let balance: f32 = (t.qty.abs() - self.count_stack().abs()) * factor;
                    self.stack = Vec::from([Trade{qty: balance, price:t.price}]);
                    self.side *= -1;
        
                } 
        
            }
        }
    }

