use anyhow::Error;
use http::request::Request;
use http::response::Response;
use yew::format::{Json, Nothing};
use yew::prelude::*;
use log::*;
use crate::fetch::{fetch, RequestExt as _};
use self::login_modal::LoginModal;

mod login_modal;

pub struct App {
    link: ComponentLink<Self>,
    state: State,
}

pub struct State {
    user_name: Option<String>,
    rooms: Vec<api::Room>,
}

impl State {
    fn new() -> Self {
        Self {
            user_name: None,
            rooms: vec![],
        }
    }
}

pub enum Msg {
    SetName(String),
    Rooms(Vec<api::Room>),
    RefreshRooms,
    AddRoom,
    JoinRoom(u64),
    Nop,
}

impl Component for App {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let state = State::new();
        App { link, state }
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        use Msg::*;
        match msg {
            SetName(s) => {
                self.state.user_name = Some(s);
                true
            }
            Rooms(rooms) => {
                self.state.rooms = rooms;
                true
            }
            RefreshRooms => {
                self.rooms();
                false
            }
            AddRoom => {
                self.add_room();
                false
            }
            JoinRoom(id) => {
                self.join_room(id);
                false
            }
            Nop => false,
        }
    }

    fn rendered(&mut self, first_render: bool) {
        if first_render {
            self.rooms();
        }
    }

    fn view(&self) -> Html {
        //let render_rooms = |room: &api::Room| html! { <li> { room.id } { for room.players.iter().map(|x| x.0) } </li> };
        
        let render_room = |room: &api::Room| -> Html {
            let room_id = room.id;
            html! {
                <tr class="clickable-row" onclick=self.link.callback(move |_| Msg::JoinRoom(room_id))>
                    <td class="has-text-centered">{ room.id }</td>
                    <td>{ room.players.iter().map(|x| x.0.to_string()).collect::<Vec<_>>().join(", ") }</td>
                </tr>
            }
        };

        html! {
            <>
                <div id="main-container" class="container">
                    <p>{ format!("Hello {}!", self.state.user_name.as_deref().unwrap_or("anonymous")) } </p>
                    <ul>
                        <table class="table is-hoverable">
                            <thead>
                                <tr>
                                    <th>{ "ID" }</th>
                                    <th>{ "Users" }</th>
                                </tr>
                            </thead>
                            <tbody>
                                { for self.state.rooms.iter().map(render_room) }
                            </tbody>
                        </table>
                    </ul>
                    <div class="mt-2">
                        <button class="button" onclick=self.link.callback(|_| Msg::AddRoom)>{ "add room" }</button>
                    </div>
                </div>
                { 
                    if self.state.user_name.is_none() {
                        html! {
                            <LoginModal callback=self.link.callback(|s| Msg::SetName(s))>
                            </LoginModal>
                        }
                    } else {
                        html! {}
                    }
                }
            </>
        }
    }
}

impl App {
    fn rooms(&self) {
        let request = Request::get("/json/rooms").body(Nothing).unwrap();
        let callback = |response: Response<Json<Result<Vec<api::Room>, Error>>>| {
            let rb = response.into_body();
            if let Json(Ok(data)) = rb {
                Msg::Rooms(data)
            } else {
                Msg::Nop
            }
        };
        fetch(request, &self.link, callback).unwrap();
    }

    fn add_room(&self) {
        let request = Request::post("/json/add_room")
            .json(&api::AddRoom)
            .unwrap();

        let callback = |response: Response<Nothing>| {
            if response.status().is_success() {
                Msg::RefreshRooms
            } else {
                error!("Fail to add a room.");
                Msg::Nop
            }
        };
        fetch(request, &self.link, callback).unwrap();
    }

    fn join_room(&self, id: u64) {
        let data = api::JoinRoom { id };
        let request = Request::post("/json/join_room")
            .json(&data)
            .unwrap();

        let callback = move |response: Response<Nothing>| {
            if response.status().is_success() {
                Msg::RefreshRooms
            } else {
                error!("Fail to join room {}.", id);
                Msg::Nop
            }
        };

        fetch(request, &self.link, callback).unwrap();
    }
}
