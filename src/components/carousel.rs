use log::info;
use mogwai::prelude::*;

pub struct Carousel {
    pub current_slide: usize,
    pub slides: Vec<ViewBuilder<HtmlElement>>
}

impl Carousel {
    fn get_controls(&self) -> Vec<ViewBuilder<HtmlElement>> {
        let tpl = |s: (usize, _)| {
            let check: bool = if s.0 == self.current_slide { true } else { false };
            let id = format!("slide-{}", s.0);
            builder!{<input class="carousel-locator" id={id} type="radio" name="carousel-radio" boolean:hidden=true  boolean:checked={check} />}
        };
        self.slides.iter().enumerate().map(|s| tpl(s)).collect::<Vec<_>>()
    }

    fn get_nav(&self) -> ViewBuilder<HtmlElement> {
        let tpl = |s: (usize, _)| {
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
        let mut slides = builder!{<div class="carousel-container"></div>};
        slides.with(self.slides.clone());
        slides
    }

    pub fn get_carousel(&self) -> ViewBuilder<HtmlElement> {
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