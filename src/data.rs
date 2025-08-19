use chrono::NaiveDate;

#[derive(Clone, Debug)]
pub struct Trip {
    pub date: NaiveDate,
    pub provider: String, //NS or GVB
    pub from: String,
    pub to: String,
    pub price: f32,
}

impl Trip {
    pub fn new(date: NaiveDate, provider: String, from: String, to: String, price: f32) -> Trip {
        Trip {
            date,
            provider,
            from,
            to,
            price,
        }
    }
}
