use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Clone, Debug)]
pub enum RoomFeature {
    ColumnsSingleAll,
    ColumnsSingleHorizontal,
    ColumnsSingleVertical,
    ColumnsSingleLeft,
    ColumnsSingleRight,
    ColumnsSingleTop,
    ColumnsSingleBottom,
    ColumnsSingleMiddle,
    ColumnsDoubleAll,
    ColumnsDoubleHorizontal,
    ColumnsDoubleVertical,
    ColumnsDoubleLeft,
    ColumnsDoubleRight,
    ColumnsDoubleTop,
    ColumnsDoubleMiddle,
    ColumnsDoubleBottom,
    ColumnsTripleAll,
    ColumnsTripleHorizontal,
    ColumnsTripleVertical,
    ColumnsTripleLeft,
    ColumnsTripleRight,
    ColumnsTripleTop,
    ColumnsTripleMiddle,
    ColumnsTripleBottom,
}
