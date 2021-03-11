use log::info;
use mogwai::prelude::*;
use chrono::prelude::*;
use std::ops::Sub;

#[derive(Clone)]
pub enum In {
    Tick
}

#[derive(Clone)]
pub enum Out {
    Time(DateTime<Utc>)
}

pub struct Clock {
    pub time: DateTime<Utc>,
}

impl Component for Clock {
    type ModelMsg = In;
    type ViewMsg = Out;
    type DomNode = HtmlElement;

    fn update(&mut self, msg: &Self::ModelMsg, tx_view: &Transmitter<Self::ViewMsg>, sub: &Subscriber<Self::ModelMsg>) {
        match msg {
            In::Tick => {
                self.time = Utc::now();
                tx_view.send(&Out::Time(self.time));
            }
        }
    }

    fn view(&self, tx: &Transmitter<Self::ModelMsg>, rx: &Receiver<Self::ViewMsg>) -> ViewBuilder<Self::DomNode> {

        let new_tx = tx.clone();
        mogwai::utils::timeout(1000, move || {
            new_tx.send(&In::Tick);
            true
        });

        let date_to_string = |time: DateTime<Utc>| format!(
            "{day_name_and_month} {day_number} {year}, {time_with_am}",
             day_name_and_month = time.format("%A, %B").to_string(),
             day_number = nth_day(time.day()),
             year = time.year(),
             time_with_am = time.format("%T %P")
        );

        let greet = |time:DateTime<Utc>| {
            let split_afternoon = 12;
            let split_evening = 17;
            let current_hour = time.format("%H").to_string().parse::<i32>().unwrap_or(0);

            match current_hour {
                x if x >= split_afternoon && x <= split_evening => String::from("afternoon"),
                x if x >= split_evening => String::from("evening"),
                _ => String::from("morning"),
            }
        };
        let greet_msg = rx.branch_map(move |msg| match msg {
            Out::Time(t) => greet(*t),
        });
        /// For testing purpose, we can pass a custom timestamp
        /// let fake:DateTime<Utc> = DateTime::from_utc(NaiveDateTime::from_timestamp(1615473738, 0), Utc);
        builder!{
            <div class="clock">
                <p class="greet">"Good "<span class={greet_msg.clone()}>{greet_msg}</span></p>
                <p class="date" >
                    {rx.branch_map(move |msg| match msg {
                        Out::Time(t) => date_to_string(*t),
                    })}
                </p>
            </div>
        }
    }
}

fn nth_day(d: u32) -> String {
    let j = d % 10;
    let k = d % 100;
    if j == 1 && k != 11 {
         return format!("{}st", d.to_string());
    }
    if j == 2 && k != 12 {
        return format!("{}nd", d.to_string());
    }
    if j == 3 && k != 13 {
        return format!("{}rd", d.to_string());
    }
    format!("{}th", d.to_string())
}
