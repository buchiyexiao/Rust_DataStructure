use List::*;
//直接使用List 不用声明来自List
enum List{
    Cons(i64,Box<List>),
    Nil,
}

impl List{
    //定义一个空的List链表
    fn new()->List{
        Nil
    }
    //向一个链表中插入一个elem 返回新链表 因此用self即可
    fn prepend(self,elem:i64) -> List{
        Cons(elem,Box::new(self))
    }
    //类似于递归进行长度的计算
    fn len(&self) -> u32{
        match *self {
            Cons(_,ref tail) => 1 + tail.len(),
            //self借用 因此不能转移tail的所有权
            //ref 引用tail
            Nil => 0
        }
    }
    //将一个链表拼接成为一段String
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