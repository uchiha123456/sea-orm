use super::{ColumnTrait, IdenStatic, Iterable};

pub trait PrimaryKeyTrait: IdenStatic + Iterable {
    fn auto_increment() -> bool;
}

pub trait PrimaryKeyToColumn {
    type Column: ColumnTrait;

    fn into_column(self) -> Self::Column;

    fn from_column(col: Self::Column) -> Option<Self>
    where
        Self: Sized;
}
