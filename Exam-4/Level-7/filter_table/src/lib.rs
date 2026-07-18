#[derive(Clone, Debug, PartialEq)]
pub struct Table {
    pub headers: Vec<String>,
    pub body: Vec<Vec<String>>,
}
impl Table {
    pub fn new() -> Table {
        Table {
            headers: Vec::new(),
            body: Vec::new(),
        }
    }
    pub fn add_row(&mut self, row: &[String]) {
        self.body.push(row.to_vec());
    }
    
    pub fn filter_col<T>(&self, filter: T) -> Option<Self>
    where
        T: Fn(&str) -> bool,
    {
let indices: Vec<usize> =self.headers.iter().enumerate().filter(|(_, h)| filter(h)).map(|(i, _)| i).collect();
        if indices.is_empty() {
            return None;
        }
        let headers = indices.iter().map(|&i| self.headers[i].clone()).collect();
let body = self.body.iter().map(|row| indices.iter().map(|&i| row[i].clone()).collect()).collect();
        Some(Table { headers, body })
    }
    pub fn filter_row<T>(&self, col_name: &str, filter: T) -> Option<Self>
    where
        T: Fn(&str) -> bool,
    {
        let col_index = self.headers.iter().position(|h| h == col_name)?;
let body: Vec<Vec<String>> = self.body.iter().filter(|row| filter(&row[col_index])).cloned().collect();
        Some(Table {
            headers: self.headers.clone(),
            body,
        })
    }
}