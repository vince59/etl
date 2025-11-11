use std::collections::{HashMap, HashSet};
use std::fmt;

/// Cell value (heterogeneous)
#[derive(Clone, Debug, PartialEq)]
pub enum Value {
    Null,
    Bool(bool),
    Int(i64),
    Float(f64),
    Text(String),
    // Add more variants if needed (Date, Bytes, etc.)
}

impl fmt::Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Value::Null => write!(f, "null"),
            Value::Bool(b) => write!(f, "{}", b),
            Value::Int(i) => write!(f, "{}", i),
            Value::Float(x) => write!(f, "{}", x),
            Value::Text(s) => write!(f, "{}", s),
        }
    }
}

/// Optional column typing
#[derive(Clone, Debug, PartialEq, Eq)]
pub enum ColType {
    Any,
    Bool,
    Int,
    Float,
    Text,
}

/// Column headers: a canonical name plus alias set
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ColHeader {
    /// Preferred canonical column name
    pub canonical: String,
    /// Accepted aliases for identifying this column
    pub aliases: HashSet<String>,
}

/// Column metadata
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ColumnMeta {
    pub header: ColHeader,
    pub kind: ColType,
    // You can extend with description, constraints, etc.
}

/// A table row
#[derive(Clone, Debug, PartialEq)]
pub struct Row {
    /// Cells aligned with the `cols` order
    pub cells: Vec<Value>,
}

/// Indices for row/column addressing
pub type RowIndex = usize;
pub type ColIndex = usize;

/// Main table (row-major storage)
#[derive(Clone, Debug, PartialEq)]
pub struct Table {
    pub cols: Vec<ColumnMeta>,
    pub rows: Vec<Row>,
    pub alias_index: HashMap<String, ColIndex>,
}

/// Logical view over a subset (no data copy)
#[derive(Clone, Debug)]
pub struct TableView<'a> {
    /// Reference to the source table
    pub table: &'a Table,
    /// Visible row indices in desired order
    pub row_idx: Vec<RowIndex>,
    /// Visible column indices in desired order
    pub col_idx: Vec<ColIndex>,
}
#[derive(Debug)]
pub enum TableError {
    /// The number of provided cells does not match the table schema
    ColumnCountMismatch { expected: usize, got: usize },
}

// Implement Display for Table to print it nicely

impl Table {

    pub fn new() -> Self {
        Table {
            cols: Vec::new(),
            rows: Vec::new(),
            alias_index: HashMap::new(),
        }
    }
    pub fn from_headers<I, S>(headers: I) -> Self
    where
        I: IntoIterator<Item = S>,
        S: Into<String>,
    {
        let mut cols: Vec<ColumnMeta> = Vec::new();
        let mut alias_index: HashMap<String, ColIndex> = HashMap::new();

        for (i, h) in headers.into_iter().enumerate() {
            let canonical = h.into();

            // Create column metadata with empty alias set
            let col_meta = ColumnMeta {
                header: ColHeader {
                    canonical: canonical.clone(),
                    aliases: HashSet::new(),
                },
                kind: ColType::Any,
            };

            cols.push(col_meta);

            // Map canonical header to its column index
            alias_index.insert(canonical, i);
        }

        Table {
            cols,
            rows: Vec::new(),
            alias_index,
        }
    }

    /// Return the number of columns in the table.
    pub fn num_cols(&self) -> usize {
        self.cols.len()
    }

    pub fn push_row_from_strings(&mut self, cells: Vec<String>) -> Result<RowIndex, TableError> {
        let expected = self.cols.len();
        let got = cells.len();

        // Enforce schema width
        if expected != got {
            return Err(TableError::ColumnCountMismatch { expected, got });
        }

        // Convert each String to a Value::Text
        let values: Vec<Value> = cells.into_iter().map(Value::Text).collect();

        // Append the row
        self.rows.push(Row { cells: values });

        // Return the index of the inserted row
        Ok(self.rows.len() - 1)
        // Note: if you prefer (), change return type to Result<(), TableError>
    }
}