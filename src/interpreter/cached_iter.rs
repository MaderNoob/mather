use std::collections::binary_heap::Iter;

pub struct CachedIterator<I,T:Iterator<Item=I>>{
    iterator:T,
    previous:Option<I>,
    stepped_back:bool,
}
impl<I,T:Iterator<Item=I>> CachedIterator<I,T>{
    pub fn new(iterator:T)->CachedIterator<I,T>{
        CachedIterator{
            iterator,
            previous:None,
            stepped_back:false,
        }
    }
    pub fn next(&'_ mut self)->&'_ Option<I>{
        if self.stepped_back{
            self.stepped_back=false
        }else{
            self.previous=self.iterator.next()
        }
        &self.previous
    }
    pub fn step_back(&mut self){
        if self.stepped_back{
            panic!("You can only step back once on the CachedIterator")
        }
        self.stepped_back=true
    }
}