// Crate Dependencies ---------------------------------------------------------
// ----------------------------------------------------------------------------
extern crate rand;
extern crate cursive;
extern crate cursive_table_view;


// STD Dependencies -----------------------------------------------------------
// ----------------------------------------------------------------------------
use std::cmp::Ordering;


// External Dependencies ------------------------------------------------------
// ----------------------------------------------------------------------------
use rand::Rng;
use cursive::Cursive;
use cursive::traits::*;
use cursive::align::HAlign;
use cursive::direction::Orientation;
use cursive::views::{BoxView, Dialog, DummyView, LinearLayout};


// Modules --------------------------------------------------------------------
// ----------------------------------------------------------------------------
use cursive_table_view::{TableView, TableViewItem};

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
enum BasicColumn {
    Name,
    Count,
    Rate
}

#[derive(Clone, Debug)]
struct Foo {
    name: String,
    count: usize,
    rate: usize
}

impl TableViewItem<BasicColumn> for Foo {

    fn to_column(&self, column: BasicColumn) -> String {
        match column {
            BasicColumn::Name => self.name.to_string(),
            BasicColumn::Count => format!("{}", self.count),
            BasicColumn::Rate => format!("{}", self.rate)
        }
    }

    fn cmp(&self, other: &Self, column: BasicColumn) -> Ordering where Self: Sized {
        match column {
            BasicColumn::Name => self.name.cmp(&other.name),
            BasicColumn::Count => self.count.cmp(&other.count),
            BasicColumn::Rate => self.rate.cmp(&other.rate)
        }
    }

}

fn main() {

    let mut siv = Cursive::new();

    let mut layout = LinearLayout::new(Orientation::Horizontal);
    layout.add_child(create_table().min_size((32, 20)));
    layout.add_child(BoxView::with_fixed_size((4, 0), DummyView));
    layout.add_child(create_table().min_size((32, 20)));

    siv.add_layer(
        Dialog::around(layout).title("Table View Demo")
    );

    siv.run();

}

fn create_table() -> TableView<Foo, BasicColumn> {

    let mut items = Vec::new();
    let mut rng = rand::thread_rng();

    for i in 0..50 {
        items.push(Foo {
            name: format!("Name {}", i),
            count: rng.gen_range(0, 255),
            rate: rng.gen_range(0, 255)
        });
    }

    TableView::<Foo, BasicColumn>::new()
        .column(BasicColumn::Name, "Name", |c| c.width_percent(20))
        .column(BasicColumn::Count, "Count", |c| c.align(HAlign::Center))
        .column(BasicColumn::Rate, "Rate", |c| c.ordering(Ordering::Greater).align(HAlign::Right).width_percent(20))
        .items(items)

}

