/*
设计一个支持 push ，pop ，top 操作，并能在常数时间内检索到最小元素的栈。
实现 MinStack 类:
MinStack() 初始化堆栈对象。
void push(int val) 将元素val推入堆栈。
void pop() 删除堆栈顶部的元素。
int top() 获取堆栈顶部的元素。
int getMin() 获取堆栈中的最小元素。
*/

struct MinStack {
    //tuple: [(val, min_val),...]
    sk: Vec<(i32, i32)>,
    min_val: i32,
}

/**
 * `&self` means the method takes an immutable reference.
 * If you need a mutable reference, change it to `&mut self` instead.
 */
impl MinStack {

    fn new() -> Self {
        Self {
            sk: vec![],
            min_val: i32::MAX,
        }
    }

    fn push(&mut self, val: i32) {
        if self.min_val > val {
            self.min_val = val;
        }
        self.sk.push((val, self.min_val));
    }

    fn pop(&mut self) {
        let _ = self.sk.pop();
        let v = self.sk.last();
        if v.is_some() {
            self.min_val = v.unwrap().1;
        } else {
            self.min_val = i32::MAX;
        }
    }

    fn top(&self) -> i32 {
        let last = self.sk.last();
        last.unwrap().0 
    }

    fn get_min(&self) -> i32 {
        self.min_val
    }
}

pub fn main() {
    let mut obj = MinStack::new();
    obj.push(-2);
    obj.push(0);
    obj.push(-3);
    obj.pop();

    let ret_3: i32 = obj.top();
    println!("ret_3: {}", ret_3);

    let ret_4: i32 = obj.get_min();
    println!("ret_4: {}", ret_4);
}

/*
 * Your MinStack object will be instantiated and called as such:
 * let obj = MinStack::new();
 * obj.push(val);
 * obj.pop();
 * let ret_3: i32 = obj.top();
 * let ret_4: i32 = obj.get_min();
 */
