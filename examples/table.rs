use polars::prelude::*;

use plotlars::{Cell, Header, Plot, Rgb, Table, Text};

fn main() {
    let dataset = df![
        "name" => &["Alice Johnson", "Bob Smith", "Charlie Davis", "Diana Martinez", "Eva Wilson"],
        "department" => &["Engineering", "Marketing", "Engineering", "Sales", "Marketing"],
        "salary" => &[95000, 78000, 102000, 85000, 82000],
        "years" => &[5, 3, 7, 4, 2]
    ]
    .unwrap();

    let header = Header::new()
        .values(vec![
            "Employee Name",
            "Department",
            "Annual Salary ($)",
            "Years of Service",
        ])
        .align("center")
        .font(Text::from("Header").size(14).font("Arial Bold"))
        .fill(Rgb(70, 130, 180));

    let cell = Cell::new()
        .align("center")
        .height(25.0)
        .font(Text::from("Cell").size(12).font("Arial"))
        .fill(vec![Rgb(240, 248, 255), Rgb(255, 255, 255)]);

    Table::builder()
        .data(&dataset)
        .columns(vec!["name", "department", "salary", "years"])
        .header(&header)
        .cell(&cell)
        .plot_title(
            Text::from("Employee Data")
                .font("Arial")
                .size(20)
                .color(Rgb(25, 25, 112)),
        )
        .build()
        .plot();
}
