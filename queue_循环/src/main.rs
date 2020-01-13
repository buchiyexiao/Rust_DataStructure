#[derive(Debug)]
struct Deque<T>{
    qdata: Vec<T>,
    qsize: usize,
    qfront: usize,
    qrear: usize,
}

impl <T> Deque<T>{
    fn new() -> Self{
        Deque{qdata:Vec::new(),qsize:3,qfront:0,qrear:0}
    }
    fn Isempty(&mut self) -> bool{
        if self.qdata.is_empty(){
            return true;
        }
        false
    }
    fn Isfull(&mut self) -> bool{
        if self.qfront==self.qrear{
            return true;
        }
        false
    }
    fn Push(&mut self,item: T){
        if self.Isempty(){
            self.qdata.push(item);
            self.qrear += 1;
            self.qrear %= self.qsize;
            return;
        }
        if self.Isfull(){
            return;
        }
        self.qdata.push(item);
        self.qrear += 1;
        self.qrear %= self.qsize;
    } 
    fn Pop(&mut self) -> T{
        if self.qrear == 0{
            self.qrear = self.qsize;
            return self.qdata.remove(self.qfront);
        }
        self.qrear -= 1;
        self.qrear %= self.qsize;
        self.qdata.remove(self.qfront)
    }
}
fn main(){
    let mut q = Deque::new();
    println!("{:?}",q.Isempty());
    println!("{:?}",q.Isfull());
    q.Push(1);
    q.Push(2);
    q.Push(3);
    println!("{:?}",q);
    q.Push(4);
    println!("{:?}",q);
    q.Pop();
    println!("{:?}",q);
    q.Push(1);
    println!("{:?}",q);
    q.Pop();
    println!("{:?}",q);
}