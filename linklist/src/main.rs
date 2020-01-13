use List::*;
enum List{
    Cons(i64,Box<List>),
    Nil,
}
impl List{
    fn new()-> List{
        Nil
    }
    fn GetLength(&self)-> u32{
        match *self{
            Cons(_,ref next) => 1 + next.GetLength(),
            Nil => 0
        }
    } 
    fn Find(&self,t:i64)-> bool{
        match *self{
            Cons(x,ref next) => if Some(x)==Some(t){
                true
            }
            else{
                next.Find(t)
            },
            Nil => false
        }
    }
    fn Insert(self,x:i64)-> List{
        Cons(x,Box::new(self))
    }
    fn Next(&self)-> List{
        match *self{
            Cons(x,ref next) => next,
            Nil => Nil
        }
    }
}
fn main(){
    let mut l = List::new();
    l = l.Insert(1);
    l = l.Insert(2);
    l = l.Insert(3);
    println!("{}",l.Find(2));
}