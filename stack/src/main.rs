#[derive(Debug)]
struct Stack<T>
{
    top: Option<Box<StackNode<T>>>,
}
#[derive(Clone,Debug)]
struct StackNode<T>
{
    val: T,
    next: Option<Box<StackNode<T>>>,
}
impl <T> StackNode<T>
{
    fn new(val: T) -> StackNode<T>
    {
        StackNode {val: val,next: None}
    }
}
impl <T> Stack<T>
{
    fn new() -> Stack<T>
    {
        Stack{top: None}
    }
    fn push(&mut self,val: T)
    {
        let mut newnode = StackNode::new(val);
        let next = self.top.take();
        // take将Option<T>中的元素先取走 然后使之为None
        newnode.next = next;
        self.top = Some(Box::new(newnode));
    }
    fn pop(&mut self) -> Option<T>
    {
        let val = self.top.take();
        match val
        {
            None => None,
            Some(mut x) => 
            {
                self.top = x.next.take();
                Some(x.val)
            },
        }
    }
}
fn main()
{
    let mut s = Stack::<i32>::new();
    assert_eq!(s.pop(),None);
    s.push(5);
    s.push(3);
    println!("{:?}",s);
    assert_eq!(s.pop(),Some(3));
    assert_eq!(s.pop(),Some(5));
    assert_eq!(s.pop(),None);
}