use crate::fetch::fetch;
use http::{request::Request, response::Response};
use log::*;
use web_sys::HtmlInputElement as InputElement;
use yew::format::{Json, Nothing};
use yew::prelude::*;

pub struct LoginModal {
    link: ComponentLink<Self>,
    props: Props,
    input_ref: NodeRef,
}

#[derive(Properties, Clone)]
pub struct Props {
    pub callback: Callback<String>
}

pub enum Msg {
    SetName,
    SetNameOk(String),
    Nop,
}

impl Component for LoginModal {
    type Message = Msg;
    type Properties = Props;
    
    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        LoginModal {
            link,
            props,
            input_ref: Default::default(),
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        use Msg::*;
        match msg {
            SetName => {
                let name = self.input_ref.cast::<InputElement>().unwrap().value();
                let data = api::SetUsername { name: name.clone() };
                let request = Request::post("/json/set_username").body(Json(&data)).unwrap();
                let callback = |resp: Response<Nothing>| {
                    if resp.status().is_success() {
                        Msg::SetNameOk(name)
                    } else {
                        error!("Fail to set username");
                        Msg::Nop
                    }
                };
                fetch(request, &self.link, callback).unwrap();
            }
            SetNameOk(name) => {
                self.props.callback.emit(name);
            }
            Nop => ()
        }
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class="modal is-active">
                <div class="modal-background"></div>
                <div class="modal-content">
                    <div class="box">
                        <label class="label">{ "Please set your name" }</label>
                        <div class="field is-grouped">
                            <div class="control is-expanded">
                                <input ref=self.input_ref.clone() class="input" type="text" placeholder="Your name"/>
                            </div>
                            <div class="control">
                                <button class="button is-primary" style="min-width: 5rem;" onclick=self.link.callback(|_| Msg::SetName)>
                                { "Set" }
                                </button>
                            </div>
                        </div>
                    </div>
                </div>
            </div>
        }
    }
}

