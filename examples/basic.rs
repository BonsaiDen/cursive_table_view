// Crate Dependencies ---------------------------------------------------------
// ----------------------------------------------------------------------------
extern crate rand;
extern crate cursive;
extern crate curtable;


// STD Dependencies -----------------------------------------------------------
// ----------------------------------------------------------------------------
use std::cmp::Ordering;


// External Dependencies ------------------------------------------------------
// ----------------------------------------------------------------------------
use rand::Rng;
use cursive::Cursive;
use cursive::traits::*;
use cursive::align::HAlign;
use cursive::views::{Dialog, TextView};


// Modules --------------------------------------------------------------------
// ----------------------------------------------------------------------------
use curtable::{TableView, TableViewItem};

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
enum BasicColumn {
    Name,
    Count,
    Rate
}

impl BasicColumn {
    fn as_str(&self) -> &str {
        match *self {
            BasicColumn::Name => "Name",
            BasicColumn::Count => "Count",
            BasicColumn::Rate => "Rate"
        }
    }
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

    let mut rng = rand::thread_rng();

    let mut siv = Cursive::new();
    let mut table = TableView::<Foo, BasicColumn>::new()
        .column(BasicColumn::Name, "Name", |c| c.width_percent(20))
        .column(BasicColumn::Count, "Count", |c| c.align(HAlign::Center))
        .column(BasicColumn::Rate, "Rate", |c| c.ordering(Ordering::Greater).align(HAlign::Right).width_percent(20));

    let mut items = Vec::new();
    for i in 0..50 {
        items.push(Foo {
            name: format!("Name {}", i),
            count: rng.gen_range(0, 255),
            rate: rng.gen_range(0, 255)
        });
    }

    table.set_items(items);

    table.set_on_sort(|siv: &mut Cursive, column: BasicColumn, order: Ordering| {
        siv.add_layer(
            Dialog::around(TextView::new(format!("{} / {:?}", column.as_str(), order)))
                   .title("Sorted by")
                   .button("Close", |s| s.pop_layer())
        );
    });

    table.set_on_submit(|siv: &mut Cursive, row: usize, index: usize| {

        let value = siv.call_on_id("table", |table: &mut TableView<Foo, BasicColumn>| {
            let value = format!("{:?}", table.item(index).unwrap());
            table.remove_item(index);
            value

        }).unwrap();

        siv.add_layer(
            Dialog::around(TextView::new(value))
                   .title(format!("Removing row # {}", row))
                   .button("Close", |s| s.pop_layer())
        );

    });

    table.set_on_select(|siv: &mut Cursive, row: usize, index: usize| {
        siv.add_layer(
            Dialog::around(TextView::new(format!("{}", index)))
                   .title(format!("Selected row # {}", row))
                   .button("Close", |s| s.pop_layer())
        );
    });

    siv.add_fullscreen_layer(
        Dialog::around(table.with_id("table")).title("Table View").full_screen()
    );

    siv.run();

}

