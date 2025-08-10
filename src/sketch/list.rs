use std::ops::Range;
use std::slice::Iter;

#[derive(Debug, Clone)]
pub struct List {
    internal_vec: Vec<usize>,
    pub(crate) record_of_operations: Vec<Operation>,
    pub(crate) length: usize,
}

#[derive(Copy, Clone, PartialEq, Debug)]
pub(crate) enum Operation {
    Get(usize),
    Set(usize, usize),
    Swap(usize, usize),
}

impl List {
    pub fn new(vec: Vec<usize>, length: usize) -> Self {
        Self {
            internal_vec: vec.clone(),
            record_of_operations: vec![],
            length,
        }
    }
    fn record(&mut self, operation: Operation) {
        self.record_of_operations.push(operation);
    }
    #[allow(dead_code)]
    pub(crate) fn iter(&self) -> Iter<'_, usize> {
        self.internal_vec.iter()
    }
}

impl ListPart for List {
    fn get(&mut self, i: usize) -> usize {
        self.record(Operation::Get(i));
        self.internal_vec[i]
    }
    fn set(&mut self, i: usize, x: usize) {
        self.record(Operation::Set(i, x));
        self.internal_vec[i] = x;
    }
    fn swap(&mut self, i: usize, j: usize) {
        self.record(Operation::Swap(i, j));
        self.internal_vec.swap(i, j);
    }
    fn slice(&mut self, range: Range<usize>) -> SliceOfList {
        SliceOfList { range, list: self }
    }
    fn len(&self) -> usize {
        self.length
    }
}

pub struct SliceOfList<'a> {
    range: Range<usize>,
    list: &'a mut List,
}

impl SliceOfList<'_> {
    fn sliced(&mut self) -> &mut [usize] {
        &mut self.list.internal_vec[self.range.clone()]
    }
    fn record(&mut self, operation: Operation) {
        let slice_start = self.range.start;
        let operation = match operation {
            Operation::Get(i) => Operation::Get(i + slice_start),
            Operation::Set(i, x) => Operation::Set(i + slice_start, x),
            Operation::Swap(i, j) => Operation::Swap(i + slice_start, j + slice_start),
        };
        self.list.record(operation);
    }
}

pub trait ListPart {
    fn get(&mut self, i: usize) -> usize;
    fn set(&mut self, i: usize, x: usize);
    fn swap(&mut self, i: usize, j: usize);
    fn slice(&mut self, range: Range<usize>) -> SliceOfList;
    fn len(&self) -> usize;
}

impl ListPart for SliceOfList<'_> {
    fn get(&mut self, i: usize) -> usize {
        self.record(Operation::Get(i));
        self.sliced()[i]
    }
    fn set(&mut self, i: usize, x: usize) {
        self.record(Operation::Set(i, x));
        self.sliced()[i] = x;
    }
    fn swap(&mut self, i: usize, j: usize) {
        self.record(Operation::Swap(i, j));
        self.sliced().swap(i, j);
    }
    fn slice(&mut self, new: Range<usize>) -> SliceOfList {
        let new_start = new.start + self.range.start;
        let new_end = new.end + self.range.start;
        assert!(new_end <= self.range.end);
        SliceOfList {
            range: new_start..new_end,
            list: &mut self.list,
        }
    }
    fn len(&self) -> usize {
        self.range.len()
    }
}

