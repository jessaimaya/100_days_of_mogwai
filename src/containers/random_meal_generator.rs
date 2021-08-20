use log::{Level};
use log::info;
use mogwai::prelude::*;

#[derive(Clone)]
pub enum In {
    Click
}

#[derive(Clone)]
pub enum Out {
    DrawClicks(i32)
}

#[derive(Clone)]
pub struct RandomMealGenerator {
    pub num_clicks: i32
}

impl Component for RandomMealGenerator {
    type ModelMsg = In;
    type ViewMsg = Out;
    type DomNode = HtmlElement;

    fn update(&mut self, msg: &In, tx_view: &Transmitter<Out>, _sub: &Subscriber<In>) {
        match msg {
            In::Click => {
                self.num_clicks += 1;
                tx_view.send(&Out::DrawClicks(self.num_clicks));
            }
        }
    }

    fn view(&self, tx: &Transmitter<In>, rx: &Receiver<Out>) -> ViewBuilder<HtmlElement> {
        builder!{
          <button on:click=tx.contra_map(|_| In::Click)>
          {(
              "clicks = 0",
              rx.branch_map(|msg| {
                  match msg {
                      Out::DrawClicks(n) => {
                          format!("clicks = {}", n)
                      }
                  }
              })
          )}
          </button>
      }
    }
}