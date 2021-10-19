use crate::types::Result;

const ITEMS_PER_PAGE: u32 = 25;

pub trait Paginator<T> {
    fn get_page(&self, page: u32) -> Result<Vec<T>>;
}

pub struct BuildkiteIterator<T> {
    current_page: u32,
    current_index: u32,
    list: Vec<T>,
    paginator: Box<dyn Paginator<T>>,
}

impl<T> BuildkiteIterator<T> {
    pub fn new(paginator: Box<dyn Paginator<T>>) -> BuildkiteIterator<T> {
        BuildkiteIterator {
            current_page: 1,
            current_index: 0,
            list: Vec::new(),
            paginator,
        }
    }
}

impl<T: Clone> Iterator for BuildkiteIterator<T> {
    type Item = T;

    fn next(&mut self) -> Option<Self::Item> {
        let idx = (self.current_index % ITEMS_PER_PAGE) as usize;
        self.current_index += 1;

        if idx == 0 {
            match self.paginator.get_page(self.current_page) {
                Ok(l) => self.list = l,
                Err(e) => {
                    println!("Err: {:?}", e);
                    return None;
                }
            }

            // The page is read, increment it.
            self.current_page += 1;
        }

        if self.list.len() > idx {
            Some(self.list[idx].clone())
        } else {
            None
        }
    }
}
