#[derive(Debug, PartialEq)]
enum SExprValue {
    SString { value: String },
    SInt { value: u8 },
}

#[derive(Debug, PartialEq)]
enum SExpr {
    Atom { name: String },
    Value { value: SExprValue },
    List { element: Vec<SExpr> },
}

// A query would be
// ((:subj :pred :obj)) // this would match every triple in the store
// |    |    |     |
// |----|----|-----|
// |    |    |     |
// ((:parent "parent" :child)) // this would match all parent-child relationships
// |    |    |
// |----|----|
// |    |    |
// or ((:subj :pred :obj) (:obj :pred2 :obj2)) // this would match every path of length 2 in the graph
// |    |    |     |     |     |
// |----|----|-----|-----|-----|
// |    |    |     |     |     |
