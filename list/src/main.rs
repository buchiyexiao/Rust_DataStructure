use List::*;
//直接使用List 不用声明来自List
enum List{
    Cons(i64,Box<List>),
    Nil,
}

impl List{
    fn new()->List{
        Nil
    }
    fn prepend(self,elem:i64) -> List{
        Cons(elem,Box::new(self))
    }
    fn len(&self) -> u32{
        match *self {
            Cons(_,ref tail) => 1 + tail.len(),
            //self借用 因此不能转移tail的所有权
            //ref 引用tail
            Nil => 0
        }
    }
    fn strintify(&self) -> String{
        match *self{
            Cons(head,ref tail) => {
                format!("{},{}",head,tail.strintify())
            },
            Nil => {
                format!("Nil")
            },
        }
    }
}
fn main(){
    let mut list = List::new();
    list = list.prepend(1);
    list = list.prepend(2);
    list = list.prepend(3);
    println!("length: {}",list.len());
    println!("{}", list.strintify());
}