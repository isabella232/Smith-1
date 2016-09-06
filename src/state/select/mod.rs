use super::{Movement, Editable, Saveable, Named, Undoable, CharIter};
use std::io::Result;
use ropey::Rope;

pub type Selection = (usize, usize);

pub trait Selectable {
    fn sel(&self) -> &Option<Selection>;
    fn set_sel(&mut self, selection: Selection);
    fn reset_sel(&mut self);
}

pub struct Select<T>
    where T: Editable
{
    pub content: T,
    pub sel: Option<Selection>,
}

impl<T> Select<T>
    where T: Editable
{
    pub fn new(content: T) -> Select<T> {
        Select {
            content: content,
            sel: None,
        }
    }
}

impl<T> Selectable for Select<T>
    where T: Editable
{
    fn sel(&self) -> &Option<Selection> {
        &self.sel
    }

    fn set_sel(&mut self, selection: Selection) {
        self.sel = Some(selection);
    }

    fn reset_sel(&mut self) {
        self.sel = None;
    }
}

impl<T> Editable for Select<T>
    where T: Editable
{
    delegate!{
        content:
            mut step(mov: Movement) -> (),
            mut move_to(pos: usize) -> (),
            mut move_at(line: usize, col: usize) -> (),
            mut insert(c: char) -> (),
            mut delete() -> Option<char>,
            pos() -> usize,
            line() -> usize,
            col() -> usize,
            line_count() -> usize,
            len() -> usize,
            iter() -> CharIter,
    }
}

impl<T> Saveable for Select<T>
    where T: Editable + Saveable
{
    delegate!{ content: save() -> Result<()> }
}

impl<T> Named for Select<T>
    where T: Editable + Named
{
    delegate!{ content: name() -> &String,
        mut set_name(name: String) -> (),
    }
}

impl<T> Undoable for Select<T>
    where T: Editable + Undoable
{
    delegate!{
        content:
            mut undo() -> (),
            mut redo() -> (),
    }
}
