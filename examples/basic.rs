// Crate Dependencies ---------------------------------------------------------
// ----------------------------------------------------------------------------
extern crate cursive;
extern crate cursive_table_view;
extern crate rand;

// STD Dependencies -----------------------------------------------------------
// ----------------------------------------------------------------------------
use std::cmp::Ordering;

// External Dependencies ------------------------------------------------------
// ----------------------------------------------------------------------------
use cursive::align::HAlign;
use cursive::traits::*;
use cursive::views::{Dialog, TextView};
use cursive::Cursive;
use rand::Rng;

// Modules --------------------------------------------------------------------
// ----------------------------------------------------------------------------
use cursive_table_view::{TableView, TableViewItem};

#[derive(Copy, Clone, PartialEq, Eq, Hash)]
enum BasicColumn {
    Name,
    Count,
    Rate,
}

impl BasicColumn {
    fn as_str(&self) -> &str {
        match *self {
            BasicColumn::Name => "Name",
            BasicColumn::Count => "Count",
            BasicColumn::Rate => "Rate",
        }
    }
}

#[derive(Clone, Debug)]
struct Foo {
    name: String,
    count: usize,
    rate: usize,
}

impl TableViewItem<BasicColumn> for Foo {
    fn to_column(&self, column: BasicColumn) -> String {
        match column {
            BasicColumn::Name => self.name.to_string(),
            BasicColumn::Count => format!("{}", self.count),
            BasicColumn::Rate => format!("{}", self.rate),
        }
    }

    fn cmp(&self, other: &Self, column: BasicColumn) -> Ordering
    where
        Self: Sized,
    {
        match column {
            BasicColumn::Name => self.name.cmp(&other.name),
            BasicColumn::Count => self.count.cmp(&other.count),
            BasicColumn::Rate => self.rate.cmp(&other.rate),
        }
    }
}

fn main() {
    let mut rng = rand::thread_rng();

    let mut siv = cursive::default();
    let mut table = TableView::<Foo, BasicColumn>::new()
        .column(BasicColumn::Name, "Name", |c| c.width_percent(20))
        .column(BasicColumn::Count, "Count", |c| c.align(HAlign::Center))
        .column(BasicColumn::Rate, "Rate", |c| {
            c.ordering(Ordering::Greater)
                .align(HAlign::Right)
                .width_percent(20)
        });

    let mut items = Vec::new();
    for i in 0..50 {
        items.push(Foo {
            name: format!("Name {}", i),
            count: rng.gen_range(0..=255),
            rate: rng.gen_range(0..=255),
        });
    }

    table.set_items(items);

    table.set_on_sort(|siv: &mut Cursive, column: BasicColumn, order: Ordering| {
        siv.add_layer(
            Dialog::around(TextView::new(format!("{} / {:?}", column.as_str(), order)))
                .title("Sorted by")
                .button("Close", |s| {
                    s.pop_layer();
                }),
        );
    });

    table.set_on_submit(|siv: &mut Cursive, row: Option<usize>, index: Option<usize>| {
        if !index.is_some() {
            return;
        }

        let value = siv
            .call_on_name("table", move |table: &mut TableView<Foo, BasicColumn>| {
                format!("{:?}", table.borrow_item(index.unwrap()).unwrap())
            })
            .unwrap();

        siv.add_layer(
            Dialog::around(TextView::new(value))
                .title(format!("Removing row # {}", row.unwrap()))
                .button("Close", move |s| {
                    s.call_on_name("table", |table: &mut TableView<Foo, BasicColumn>| {
                        table.remove_item(index.unwrap());
                    });
                    s.pop_layer();
                }),
        );
    });

    siv.add_layer(Dialog::around(table.with_name("table").min_size((50, 20))).title("Table View"));

    siv.run();
}
