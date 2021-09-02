use mogwai::prelude::*;

pub struct Slide {
    pub text: String,
    pub image: String,
}

impl Slide {
    pub fn get_view(&self) -> ViewBuilder<HtmlElement> {
        builder! {
            <figure class="carousel-item">
                <h2 class="login-carousel-slide">{self.text.clone()}</h2>
                <img src={self.image.clone()} alt={self.text.clone()}/>
          </figure>
        }
    }
}
