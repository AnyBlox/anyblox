use std::fmt::Display;

pub struct Table {
    headers: Row,
    rows: Vec<Row>,
}

pub struct Row {
    values: Vec<Box<dyn Display>>,
}

pub struct SizeDisplay(pub usize);

pub struct WasmPageDisplay(pub usize);

pub struct OptionDisplay<D>(pub Option<D>);

impl Table {
    pub fn new(headers: Row) -> Self {
        Self { headers, rows: vec![] }
    }

    pub fn add_row(&mut self, row: Row) {
        self.rows.push(row);
    }

    pub fn headers(&self) -> &Row {
        &self.headers
    }

    pub fn rows(&self) -> impl Iterator<Item = &Row> {
        self.rows.iter()
    }
}

impl Row {
    pub fn new() -> Self {
        Self { values: vec![] }
    }

    pub fn add<D: Display + 'static>(&mut self, item: D) {
        self.values.push(Box::new(item))
    }

    pub fn iter(&self) -> impl Iterator<Item = &dyn Display> {
        self.values.iter().map(|x| x.as_ref())
    }
}

impl Default for Row {
    fn default() -> Self {
        Self::new()
    }
}

impl Display for SizeDisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mb = self.0 as f32 / 1_000_000.0;
        write!(f, "{mb: >8.3}MB")
    }
}

impl Display for WasmPageDisplay {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let pages = self.0;
        let bytes = pages * crate::wasm::PAGE_SIZE;
        write!(f, "{pages} pages ({})", SizeDisplay(bytes))
    }
}

impl<D: Display> Display for OptionDisplay<D> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(x) = &self.0 {
            write!(f, "{x}")
        } else {
            Ok(())
        }
    }
}

#[macro_export]
macro_rules! row {
    ($($it:expr),*) => {
        {
            let mut row = $crate::fmt::Row::new();
            $(row.add($it);)*
            row
        }
    };
}
