use plotlars::{Cell, Header, Plot, Rgb, Table, Text};
use polars::prelude::*;

fn main() {
    let dataset = LazyCsvReader::new(PlRefPath::new("data/employee_data.csv"))
        .finish()
        .unwrap()
        .collect()
        .unwrap();

    let header = Header::new()
        .values(vec![
            "Employee Name",
            "Department",
            "Annual Salary ($)",
            "Years of Service",
        ])
        .align("center")
        .font("Arial Black")
        .fill(Rgb(70, 130, 180));

    let cell = Cell::new()
        .align("center")
        .height(25.0)
        .fill(Rgb(240, 248, 255));

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
