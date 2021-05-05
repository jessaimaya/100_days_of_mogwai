use log::info;
use mogwai::prelude::*;

pub struct Slide {
    pub color: String
}

#[derive(Clone)]
pub enum In {
    Next,
    Prev,
}

#[derive(Clone)]
pub enum Out {
    GoTo(Patch<View<HtmlElement>>)
}

pub struct Carousel {
    pub current_slide: usize,
    pub slides: Vec<Slide>
}

impl Carousel {
    fn get_controls(&self) -> Vec<ViewBuilder<HtmlElement>> {
        let tpl = |s: (usize, &Slide)| {
            let check: bool = if s.0 == self.current_slide { true } else { false };
            let id = format!("slide-{}", s.0);
            builder!{<input class="carousel-locator" id={id} type="radio" name="carousel-radio" boolean:hidden=true  boolean:checked={check} />}
        };
        self.slides.iter().enumerate().map(|s| tpl(s)).collect::<Vec<_>>()
    }

    fn get_nav(&self) -> ViewBuilder<HtmlElement> {
        let tpl = |s: (usize, &Slide)| {
            let check: bool = if s.0 == self.current_slide { true } else { false };
            let id = format!("slide-{}", s.0);
            builder!{<label class="nav-item text-hide c-hand" for={id}>{s.0.to_string()}</label>}
        };
        let op = self.slides.iter().enumerate().map(|s| tpl(s)).collect::<Vec<_>>();

        let mut b = builder!{<div class="carousel-nav"></div>};
        b.with(op);
        b
    }

    fn get_slides(&self) -> ViewBuilder<HtmlElement> {
        let tpl =  |s:(usize, &Slide)| {
          builder!{<figure style:background="#333" class="carousel-item">
                <p style:color="#fff">{s.1.color.clone()}</p>
              </figure>}
        };
        let slides_tpl = self.slides.iter().enumerate().map(|s| tpl(s)).collect::<Vec<_>>();
        let mut slides = builder!{<div class="carousel-container"></div>};
        slides.with(slides_tpl);
        slides
    }

    fn get_carousel(&self) -> ViewBuilder<HtmlElement> {
        let mut carousel = builder!{<div class="carousel"></div>};
        let controls = self.get_controls();
        let slides = self.get_slides();
        let nav = self.get_nav();
        carousel.with(controls);
        carousel.with(slides);
        carousel.with(nav);
        carousel
    }
}

impl Component for Carousel {
    type ModelMsg = In;
    type ViewMsg = Out;
    type DomNode = HtmlElement;

    fn update(&mut self, msg: &In, tx_view: &Transmitter<Out>, _sub: &Subscriber<In>) {
        match msg {
            In::Next => {
                let len = self.slides.iter().count();
                let sum = self.current_slide + 1;
                self.current_slide = match  sum > len {
                    true => 0,
                    _ => sum
                } ;
                tx_view.send(&Out::GoTo(Patch::Replace {index: 1, value: View::from(builder!{<p>{self.current_slide.to_string()}</p>})}));
            },
            In::Prev => {
                let len = self.slides.iter().count();
                let min = self.current_slide - 1;
                self.current_slide = match  min > 0 {
                    true => min,
                    _ => len
                } ;
                tx_view.send(&Out::GoTo(Patch::Replace {index: 1, value: View::from(builder!{<p>{self.current_slide.to_string()}</p>})}));
            },
        }
    }

    fn view(&self, tx: &Transmitter<In>, rx: &Receiver<Out>) -> ViewBuilder<HtmlElement> {
        let carousel = self.get_carousel();
        builder!{
            <div>
            <div patch:children=rx.branch_map(|Out::GoTo(patch)| patch.clone())>
            </div>

            {carousel}

          <button on:click=tx.contra_map(|_| In::Next)>
            "Next"
            </button>
            </div>
      }
    }
}